//! Mapping GameBanana's JSON into deadloader's types.
//!
//! The body is read into a JSON value first, so an error document (`_sErrorCode`) is recognised whatever the endpoint
//! and whatever the HTTP status. Then private `Raw*` structs — the only place GameBanana's field names appear — are
//! filled by serde and converted. Optional fields fall back to defaults; a missing required field or a wrong type is
//! an [`Error::ApiChanged`]. GameBanana is a PHP site and sends an empty object as `[]`, so nested objects accept
//! that too.

use crate::error::{ApiChange, ApiChangeReason, Error, FetchError};
use crate::request::GAME_ID;
use crate::types::{
    AlternateSource, Antivirus, CatalogItem, CatalogPage, Category, CategoryRef, ContentRating,
    Credit, CreditGroup, Image, ImageSize, ImageVariant, ModDetails, RemoteFile, Section,
    Visibility,
};
use serde::Deserialize;
use serde::de::{self, DeserializeOwned, Deserializer};
use serde_json::Value;
use std::collections::BTreeMap;

const NO_SUCH_RECORD: &str = "NO_SUCH_RECORD";

/// A `section/Index` page.
pub fn parse_list(section: Section, body: &[u8]) -> Result<CatalogPage, Error> {
    parse_page(list_endpoint(section), body)
}

/// A `Util/Search/Results` page.
pub fn parse_search(body: &[u8]) -> Result<CatalogPage, Error> {
    parse_page(SEARCH_ENDPOINT, body)
}

/// A `section/Categories` listing.
pub fn parse_categories(section: Section, body: &[u8]) -> Result<Vec<Category>, Error> {
    let endpoint = categories_endpoint(section);
    let raw: Vec<RawCategory> = decode(endpoint, body)?;
    Ok(raw
        .into_iter()
        .map(|c| Category {
            id: c.id,
            name: c.name,
            item_count: c.item_count,
            subcategory_count: c.category_count,
        })
        .collect())
}

/// A `section/{id}/ProfilePage` document.
pub fn parse_details(section: Section, body: &[u8]) -> Result<ModDetails, Error> {
    let raw: RawProfile = decode(details_endpoint(section), body)?;
    if let Some(game_id) = raw.game.and_then(|g| g.id)
        && game_id != GAME_ID
    {
        return Err(Error::WrongGame { game_id });
    }
    let content_ratings: Vec<ContentRating> = raw
        .content_ratings
        .unwrap_or_default()
        .into_iter()
        .map(|(code, label)| content_rating(code, label))
        .collect();
    let initial_visibility = visibility(
        raw.initial_visibility.as_deref(),
        !content_ratings.is_empty(),
    );
    let category_path = [raw.super_category, raw.category]
        .into_iter()
        .flatten()
        .map(category_ref)
        .collect();
    let (gallery, audio_preview_url) = preview(raw.preview_media);
    Ok(ModDetails {
        id: raw.id,
        section,
        name: raw.name,
        version: non_empty(raw.version),
        author: raw.submitter.map(|s| s.name),
        credits: raw
            .credits
            .into_iter()
            .map(|g| CreditGroup {
                name: g.name,
                authors: g
                    .authors
                    .into_iter()
                    .map(|a| Credit {
                        name: a.name,
                        role: non_empty(a.role),
                    })
                    .collect(),
            })
            .collect(),
        category_path,
        gallery,
        audio_preview_url,
        short_description: non_empty(raw.description),
        description_html: raw.text.unwrap_or_default(),
        licence_html: non_empty(raw.license),
        likes: raw.like_count,
        views: raw.view_count,
        downloads: raw.download_count,
        date_added: raw.date_added,
        date_modified: raw.date_modified,
        date_updated: raw.date_updated,
        initial_visibility,
        content_ratings,
        obsolete: raw.is_obsolete,
        embedded_media: raw.embedded_media,
        alternate_sources: raw
            .alternate_sources
            .into_iter()
            .map(|s| AlternateSource {
                url: s.url,
                description: non_empty(s.description),
            })
            .collect(),
        files: raw.files.into_iter().map(remote_file).collect(),
        archived_files: raw.archived_files.into_iter().map(remote_file).collect(),
    })
}

/// Turns a failed fetch into this module's error. A non-success status whose body is GameBanana's error JSON is an
/// answer, not a transport failure: a missing record or a changed API.
pub fn fetch_error(endpoint: &'static str, error: FetchError) -> Error {
    if let FetchError::Status { body, .. } = &error
        && let Ok(value) = serde_json::from_slice::<Value>(body)
        && let Some(answer) = error_answer(endpoint, &value)
    {
        return answer;
    }
    Error::Fetch(error)
}

pub(crate) const SEARCH_ENDPOINT: &str = "Util/Search/Results";

pub(crate) fn list_endpoint(section: Section) -> &'static str {
    match section {
        Section::Mod => "Mod/Index",
        Section::Sound => "Sound/Index",
    }
}

pub(crate) fn categories_endpoint(section: Section) -> &'static str {
    match section {
        Section::Mod => "Mod/Categories",
        Section::Sound => "Sound/Categories",
    }
}

pub(crate) fn details_endpoint(section: Section) -> &'static str {
    match section {
        Section::Mod => "Mod/ProfilePage",
        Section::Sound => "Sound/ProfilePage",
    }
}

fn api_changed(endpoint: &'static str, reason: ApiChangeReason) -> Error {
    Error::ApiChanged(ApiChange { endpoint, reason })
}

/// Recognises GameBanana's error document: `{"_sErrorCode": "...", "_aErrorData": {"param": {"_sErrorCode": ...}}}`.
fn error_answer(endpoint: &'static str, value: &Value) -> Option<Error> {
    let code = value.get("_sErrorCode")?.as_str()?;
    if code == NO_SUCH_RECORD {
        return Some(Error::NotFound);
    }
    let details = value
        .get("_aErrorData")
        .and_then(Value::as_object)
        .map(|data| {
            data.iter()
                .filter_map(|(param, entry)| {
                    let inner = entry.get("_sErrorCode")?.as_str()?;
                    Some(format!("{param}={inner}"))
                })
                .collect()
        })
        .unwrap_or_default();
    Some(api_changed(
        endpoint,
        ApiChangeReason::ErrorCode {
            code: code.to_owned(),
            details,
        },
    ))
}

fn decode<T: DeserializeOwned>(endpoint: &'static str, body: &[u8]) -> Result<T, Error> {
    let value: Value = serde_json::from_slice(body)
        .map_err(|_| api_changed(endpoint, ApiChangeReason::NotJson))?;
    if let Some(answer) = error_answer(endpoint, &value) {
        return Err(answer);
    }
    T::deserialize(value).map_err(|e| api_changed(endpoint, shape_reason(&e)))
}

fn shape_reason(error: &serde_json::Error) -> ApiChangeReason {
    let message = error.to_string();
    if let Some(rest) = message.strip_prefix("missing field `")
        && let Some((field, _)) = rest.split_once('`')
    {
        return ApiChangeReason::MissingField(field.to_owned());
    }
    ApiChangeReason::UnexpectedShape(message)
}

fn parse_page(endpoint: &'static str, body: &[u8]) -> Result<CatalogPage, Error> {
    let raw: RawPage = decode(endpoint, body)?;
    let items = raw.records.into_iter().filter_map(catalog_item).collect();
    Ok(CatalogPage {
        items,
        total: raw.metadata.record_count,
        is_complete: raw.metadata.is_complete,
    })
}

/// `None` for records of models the catalog does not show (search and feeds may mix in tools and the like).
fn catalog_item(raw: RawRecord) -> Option<CatalogItem> {
    let section = match raw.model.as_str() {
        "Mod" => Section::Mod,
        "Sound" => Section::Sound,
        _ => return None,
    };
    let (images, audio_preview_url) = preview(raw.preview_media);
    Some(CatalogItem {
        id: raw.id,
        section,
        name: raw.name,
        author: raw.submitter.map(|s| s.name),
        root_category: raw.root_category.map(category_ref),
        subcategory: raw.sub_category.map(category_ref),
        likes: raw.like_count,
        views: raw.view_count,
        date_added: raw.date_added,
        date_modified: raw.date_modified,
        date_updated: raw.date_updated,
        thumbnail: images.into_iter().next(),
        audio_preview_url,
        has_content_ratings: raw.has_content_ratings,
        initial_visibility: visibility(raw.initial_visibility.as_deref(), raw.has_content_ratings),
        obsolete: raw.is_obsolete,
        featured: raw.was_featured,
        has_files: raw.has_files,
    })
}

/// Unknown values hide the submission: when GameBanana invents a new level, failing safe beats showing it.
/// A missing value is derived from whether the submission has content ratings.
fn visibility(value: Option<&str>, rated: bool) -> Visibility {
    match value {
        Some("show") => Visibility::Show,
        Some("warn") => Visibility::Warn,
        Some("hide") => Visibility::Hide,
        Some(_) => Visibility::Hide,
        None if rated => Visibility::Hide,
        None => Visibility::Show,
    }
}

fn content_rating(code: String, label: String) -> ContentRating {
    match code.as_str() {
        "st" => ContentRating::SexualThemes,
        "sc" => ContentRating::SexualContent,
        "sa" => ContentRating::SkimpyAttire,
        "lp" => ContentRating::LewdAnglesAndPoses,
        "nu" => ContentRating::FullNudity,
        _ => ContentRating::Other { code, label },
    }
}

fn category_ref(raw: RawCategoryLink) -> CategoryRef {
    let id = raw
        .id
        .or_else(|| raw.profile_url.as_deref().and_then(id_from_category_url));
    CategoryRef { id, name: raw.name }
}

/// List records name their categories only by profile URL: `https://gamebanana.com/mods/cats/33295`.
fn id_from_category_url(url: &str) -> Option<u64> {
    let (_, rest) = url.rsplit_once("/cats/")?;
    rest.split(['/', '?', '#']).next()?.parse().ok()
}

fn preview(media: Option<RawPreviewMedia>) -> (Vec<Image>, Option<String>) {
    let Some(media) = media else {
        return (Vec::new(), None);
    };
    let images = media.images.into_iter().filter_map(image).collect();
    let audio = media.metadata.and_then(|m| non_empty(m.audio_url));
    (images, audio)
}

/// Entries without a base URL or a file cannot be shown and are skipped rather than failing the whole page.
fn image(raw: RawImage) -> Option<Image> {
    let base = non_empty(raw.base_url)?;
    let base = base.trim_end_matches('/');
    let original = non_empty(raw.file)?;
    let url = |file: &str| format!("{base}/{file}");
    let variants = [
        (ImageSize::W100, raw.file_100, raw.width_100, raw.height_100),
        (ImageSize::W220, raw.file_220, raw.width_220, raw.height_220),
        (ImageSize::W530, raw.file_530, raw.width_530, raw.height_530),
        (ImageSize::W800, raw.file_800, raw.width_800, raw.height_800),
    ]
    .into_iter()
    .filter_map(|(size, file, width, height)| {
        let file = non_empty(file)?;
        Some(ImageVariant {
            size,
            url: url(&file),
            width,
            height,
        })
    })
    .collect();
    Some(Image {
        original_url: url(&original),
        caption: non_empty(raw.caption),
        variants,
    })
}

fn remote_file(raw: RawFile) -> RemoteFile {
    let antivirus = antivirus(non_empty(raw.av_state), non_empty(raw.av_result));
    RemoteFile {
        id: raw.id,
        file_name: raw.file,
        size_bytes: raw.file_size,
        date_added: raw.date_added,
        download_count: raw.download_count,
        description: non_empty(raw.description),
        version: non_empty(raw.version),
        md5: non_empty(raw.md5),
        antivirus,
        download_url: raw.download_url,
    }
}

fn antivirus(state: Option<String>, result: Option<String>) -> Antivirus {
    match (state.as_deref(), result) {
        (None | Some("done"), Some(result)) if result == "clean" => Antivirus::Clean,
        (None | Some("done"), Some(result)) => Antivirus::Flagged(result),
        (None | Some("done"), None) => Antivirus::NotReported,
        (Some(state), _) => Antivirus::Unfinished(state.to_owned()),
    }
}

fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|v| !v.trim().is_empty())
}

/// An object, or `None` for `null` and the `[]` PHP sends for an empty object.
fn lenient<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    match Value::deserialize(deserializer)? {
        Value::Null => Ok(None),
        Value::Array(items) if items.is_empty() => Ok(None),
        other => T::deserialize(other).map(Some).map_err(de::Error::custom),
    }
}

#[derive(Deserialize)]
struct RawPage {
    #[serde(rename = "_aMetadata")]
    metadata: RawMetadata,
    #[serde(rename = "_aRecords")]
    records: Vec<RawRecord>,
}

#[derive(Deserialize)]
struct RawMetadata {
    #[serde(rename = "_nRecordCount")]
    record_count: u64,
    #[serde(rename = "_bIsComplete")]
    is_complete: bool,
}

#[derive(Deserialize)]
struct RawRecord {
    #[serde(rename = "_idRow")]
    id: u64,
    #[serde(rename = "_sModelName")]
    model: String,
    #[serde(rename = "_sName")]
    name: String,
    #[serde(rename = "_aSubmitter", default, deserialize_with = "lenient")]
    submitter: Option<RawMember>,
    #[serde(rename = "_aRootCategory", default, deserialize_with = "lenient")]
    root_category: Option<RawCategoryLink>,
    #[serde(rename = "_aSubCategory", default, deserialize_with = "lenient")]
    sub_category: Option<RawCategoryLink>,
    #[serde(rename = "_nLikeCount", default)]
    like_count: u64,
    #[serde(rename = "_nViewCount", default)]
    view_count: u64,
    #[serde(rename = "_tsDateAdded", default)]
    date_added: Option<i64>,
    #[serde(rename = "_tsDateModified", default)]
    date_modified: Option<i64>,
    #[serde(rename = "_tsDateUpdated", default)]
    date_updated: Option<i64>,
    #[serde(rename = "_aPreviewMedia", default, deserialize_with = "lenient")]
    preview_media: Option<RawPreviewMedia>,
    #[serde(rename = "_bHasContentRatings", default)]
    has_content_ratings: bool,
    #[serde(rename = "_sInitialVisibility", default)]
    initial_visibility: Option<String>,
    #[serde(rename = "_bIsObsolete", default)]
    is_obsolete: bool,
    #[serde(rename = "_bWasFeatured", default)]
    was_featured: bool,
    #[serde(rename = "_bHasFiles", default)]
    has_files: bool,
}

#[derive(Deserialize)]
struct RawMember {
    #[serde(rename = "_sName")]
    name: String,
}

#[derive(Deserialize)]
struct RawCategoryLink {
    #[serde(rename = "_idRow", default)]
    id: Option<u64>,
    #[serde(rename = "_sName")]
    name: String,
    #[serde(rename = "_sProfileUrl", default)]
    profile_url: Option<String>,
}

#[derive(Deserialize)]
struct RawCategory {
    #[serde(rename = "_idRow")]
    id: u64,
    #[serde(rename = "_sName")]
    name: String,
    #[serde(rename = "_nItemCount", default)]
    item_count: u64,
    #[serde(rename = "_nCategoryCount", default)]
    category_count: u64,
}

#[derive(Deserialize)]
struct RawPreviewMedia {
    #[serde(rename = "_aImages", default)]
    images: Vec<RawImage>,
    #[serde(rename = "_aMetadata", default, deserialize_with = "lenient")]
    metadata: Option<RawMediaMetadata>,
}

#[derive(Deserialize)]
struct RawMediaMetadata {
    #[serde(rename = "_sAudioUrl", default)]
    audio_url: Option<String>,
}

#[derive(Deserialize)]
struct RawImage {
    #[serde(rename = "_sBaseUrl", default)]
    base_url: Option<String>,
    #[serde(rename = "_sFile", default)]
    file: Option<String>,
    #[serde(rename = "_sCaption", default)]
    caption: Option<String>,
    #[serde(rename = "_sFile100", default)]
    file_100: Option<String>,
    #[serde(rename = "_wFile100", default)]
    width_100: Option<u32>,
    #[serde(rename = "_hFile100", default)]
    height_100: Option<u32>,
    #[serde(rename = "_sFile220", default)]
    file_220: Option<String>,
    #[serde(rename = "_wFile220", default)]
    width_220: Option<u32>,
    #[serde(rename = "_hFile220", default)]
    height_220: Option<u32>,
    #[serde(rename = "_sFile530", default)]
    file_530: Option<String>,
    #[serde(rename = "_wFile530", default)]
    width_530: Option<u32>,
    #[serde(rename = "_hFile530", default)]
    height_530: Option<u32>,
    #[serde(rename = "_sFile800", default)]
    file_800: Option<String>,
    #[serde(rename = "_wFile800", default)]
    width_800: Option<u32>,
    #[serde(rename = "_hFile800", default)]
    height_800: Option<u32>,
}

#[derive(Deserialize)]
struct RawProfile {
    #[serde(rename = "_idRow")]
    id: u64,
    #[serde(rename = "_sName")]
    name: String,
    #[serde(rename = "_sVersion", default)]
    version: Option<String>,
    #[serde(rename = "_aSubmitter", default, deserialize_with = "lenient")]
    submitter: Option<RawMember>,
    #[serde(rename = "_aCredits", default)]
    credits: Vec<RawCreditGroup>,
    #[serde(rename = "_aCategory", default, deserialize_with = "lenient")]
    category: Option<RawCategoryLink>,
    #[serde(rename = "_aSuperCategory", default, deserialize_with = "lenient")]
    super_category: Option<RawCategoryLink>,
    #[serde(rename = "_aPreviewMedia", default, deserialize_with = "lenient")]
    preview_media: Option<RawPreviewMedia>,
    #[serde(rename = "_sDescription", default)]
    description: Option<String>,
    #[serde(rename = "_sText", default)]
    text: Option<String>,
    #[serde(rename = "_sLicense", default)]
    license: Option<String>,
    #[serde(rename = "_nLikeCount", default)]
    like_count: u64,
    #[serde(rename = "_nViewCount", default)]
    view_count: u64,
    #[serde(rename = "_nDownloadCount", default)]
    download_count: u64,
    #[serde(rename = "_tsDateAdded", default)]
    date_added: Option<i64>,
    #[serde(rename = "_tsDateModified", default)]
    date_modified: Option<i64>,
    #[serde(rename = "_tsDateUpdated", default)]
    date_updated: Option<i64>,
    #[serde(rename = "_sInitialVisibility", default)]
    initial_visibility: Option<String>,
    #[serde(rename = "_aContentRatings", default, deserialize_with = "lenient")]
    content_ratings: Option<BTreeMap<String, String>>,
    #[serde(rename = "_bIsObsolete", default)]
    is_obsolete: bool,
    #[serde(rename = "_aEmbeddedMedia", default)]
    embedded_media: Vec<String>,
    #[serde(rename = "_aAlternateFileSources", default)]
    alternate_sources: Vec<RawAlternateSource>,
    #[serde(rename = "_aFiles", default)]
    files: Vec<RawFile>,
    #[serde(rename = "_aArchivedFiles", default)]
    archived_files: Vec<RawFile>,
    #[serde(rename = "_aGame", default, deserialize_with = "lenient")]
    game: Option<RawGame>,
}

#[derive(Deserialize)]
struct RawGame {
    #[serde(rename = "_idRow", default)]
    id: Option<u64>,
}

#[derive(Deserialize)]
struct RawCreditGroup {
    #[serde(rename = "_sGroupName", default)]
    name: String,
    #[serde(rename = "_aAuthors", default)]
    authors: Vec<RawCreditAuthor>,
}

#[derive(Deserialize)]
struct RawCreditAuthor {
    #[serde(rename = "_sName")]
    name: String,
    #[serde(rename = "_sRole", default)]
    role: Option<String>,
}

#[derive(Deserialize)]
struct RawAlternateSource {
    url: String,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Deserialize)]
struct RawFile {
    #[serde(rename = "_idRow")]
    id: u64,
    #[serde(rename = "_sFile")]
    file: String,
    #[serde(rename = "_nFilesize")]
    file_size: u64,
    #[serde(rename = "_tsDateAdded", default)]
    date_added: Option<i64>,
    #[serde(rename = "_nDownloadCount", default)]
    download_count: u64,
    #[serde(rename = "_sDescription", default)]
    description: Option<String>,
    #[serde(rename = "_sVersion", default)]
    version: Option<String>,
    #[serde(rename = "_sMd5Checksum", default)]
    md5: Option<String>,
    #[serde(rename = "_sAvState", default)]
    av_state: Option<String>,
    #[serde(rename = "_sAvResult", default)]
    av_result: Option<String>,
    #[serde(rename = "_sDownloadUrl")]
    download_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! fixture {
        ($name:literal) => {
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/",
                $name
            ))
        };
    }

    const IMG: &str = "https://images.gamebanana.com/img/ss/mods";

    fn changed(error: Error) -> ApiChange {
        match error {
            Error::ApiChanged(change) => change,
            other => panic!("expected ApiChanged, got {other:?}"),
        }
    }

    #[test]
    fn mod_index_page() {
        let page = parse_list(Section::Mod, fixture!("mod_index.json")).expect("parses");
        assert_eq!(page.total, 1234);
        assert!(!page.is_complete);
        assert_eq!(page.items.len(), 3);

        let full = &page.items[0];
        assert_eq!(full.id, 700_001);
        assert_eq!(full.section, Section::Mod);
        assert_eq!(full.name, "Copper Coat for Wraith");
        assert_eq!(full.author.as_deref(), Some("tinker_fox"));
        assert_eq!(
            full.root_category,
            Some(CategoryRef {
                id: Some(33_295),
                name: "Skins".into()
            })
        );
        assert_eq!(
            full.subcategory,
            Some(CategoryRef {
                id: Some(33_320),
                name: "Wraith".into()
            })
        );
        assert_eq!((full.likes, full.views), (42, 3100));
        assert_eq!(full.date_added, Some(1_760_000_000));
        assert_eq!(full.date_modified, Some(1_761_000_000));
        assert_eq!(full.date_updated, Some(1_761_000_100));
        assert!(!full.has_content_ratings);
        assert_eq!(full.initial_visibility, Visibility::Show);
        assert!(!full.obsolete);
        assert!(full.featured);
        assert!(full.has_files);
        assert_eq!(full.audio_preview_url, None);

        let thumb = full.thumbnail.as_ref().expect("first image");
        assert_eq!(thumb.original_url, format!("{IMG}/aaaa0001.jpg"));
        assert_eq!(
            thumb.variants,
            vec![
                ImageVariant {
                    size: ImageSize::W100,
                    url: format!("{IMG}/100-90_aaaa0001.jpg"),
                    width: Some(100),
                    height: Some(56),
                },
                ImageVariant {
                    size: ImageSize::W220,
                    url: format!("{IMG}/220-90_aaaa0001.jpg"),
                    width: Some(220),
                    height: Some(124),
                },
                ImageVariant {
                    size: ImageSize::W530,
                    url: format!("{IMG}/530-90_aaaa0001.jpg"),
                    width: Some(530),
                    height: Some(298),
                },
            ]
        );
    }

    #[test]
    fn list_tolerates_missing_optional_fields() {
        let page = parse_list(Section::Mod, fixture!("mod_index.json")).expect("parses");

        let rated = &page.items[1];
        assert!(rated.has_content_ratings);
        assert_eq!(rated.initial_visibility, Visibility::Hide);
        assert!(rated.obsolete);
        assert_eq!(rated.thumbnail, None, "PHP's [] for an empty object");
        assert_eq!(rated.subcategory, None);
        assert_eq!(rated.date_updated, None);

        let sparse = &page.items[2];
        assert_eq!(sparse.author, None);
        assert_eq!(sparse.root_category, None);
        assert_eq!((sparse.likes, sparse.views), (0, 0));
        assert_eq!(sparse.date_added, None);
        assert_eq!(sparse.initial_visibility, Visibility::Warn);
        assert!(!sparse.has_files);
        let thumb = sparse.thumbnail.as_ref().expect("image");
        // A trailing slash on the base URL does not double up.
        assert_eq!(thumb.original_url, format!("{IMG}/aaaa0003.png"));
        assert_eq!(
            thumb.variant(ImageSize::W800).map(|v| v.url.as_str()),
            Some(format!("{IMG}/800-90_aaaa0003.png").as_str())
        );
        assert_eq!(thumb.variant(ImageSize::W220), None);
    }

    #[test]
    fn sound_index_page() {
        let page = parse_list(Section::Sound, fixture!("sound_index.json")).expect("parses");
        assert_eq!(page.total, 2);
        assert!(page.is_complete);
        let first = &page.items[0];
        assert_eq!(first.section, Section::Sound);
        assert_eq!(first.thumbnail, None);
        assert_eq!(
            first.audio_preview_url.as_deref(),
            Some("https://files.gamebanana.com/preview/sounds/bell_chime.mp3")
        );
        assert_eq!(
            first.root_category,
            Some(CategoryRef {
                id: Some(6270),
                name: "Abilities".into()
            })
        );
        assert_eq!(first.likes, 0);
        let second = &page.items[1];
        assert_eq!(second.audio_preview_url, None);
        assert_eq!(second.likes, 3);
        assert_eq!(second.initial_visibility, Visibility::Show);
    }

    #[test]
    fn search_page_skips_models_the_catalog_does_not_show() {
        let page = parse_search(fixture!("search_results.json")).expect("parses");
        assert_eq!(page.total, 31);
        assert!(!page.is_complete);
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].name, "Haze in Teal");
        assert_eq!(
            page.items[0].subcategory.as_ref().and_then(|c| c.id),
            Some(33_310)
        );
    }

    #[test]
    fn categories() {
        let cats =
            parse_categories(Section::Mod, fixture!("categories_root.json")).expect("parses");
        assert_eq!(
            cats,
            vec![
                Category {
                    id: 47_611,
                    name: "Animations".into(),
                    item_count: 19,
                    subcategory_count: 0
                },
                Category {
                    id: 31_713,
                    name: "HUD".into(),
                    item_count: 638,
                    subcategory_count: 4
                },
                Category {
                    id: 33_295,
                    name: "Skins".into(),
                    item_count: 3175,
                    subcategory_count: 44
                },
                Category {
                    id: 31_710,
                    name: "Other/Misc".into(),
                    item_count: 0,
                    subcategory_count: 0
                },
            ]
        );
        assert_eq!(
            parse_categories(Section::Mod, b"[]").expect("parses"),
            vec![]
        );
    }

    #[test]
    fn mod_profile() {
        let d = parse_details(Section::Mod, fixture!("mod_profile.json")).expect("parses");
        assert_eq!(d.id, 700_001);
        assert_eq!(d.section, Section::Mod);
        assert_eq!(d.name, "Copper Coat for Wraith");
        assert_eq!(d.version.as_deref(), Some("1.2"));
        assert_eq!(d.author.as_deref(), Some("tinker_fox"));
        assert_eq!(
            d.credits,
            vec![
                CreditGroup {
                    name: "Key Authors".into(),
                    authors: vec![Credit {
                        name: "tinker_fox".into(),
                        role: Some("Modeller".into())
                    }]
                },
                CreditGroup {
                    name: "Special Thanks".into(),
                    authors: vec![Credit {
                        name: "a friend without an account".into(),
                        role: None
                    }]
                },
            ]
        );
        assert_eq!(
            d.category_path,
            vec![
                CategoryRef {
                    id: Some(33_295),
                    name: "Skins".into()
                },
                CategoryRef {
                    id: Some(33_320),
                    name: "Wraith".into()
                },
            ]
        );
        assert_eq!(d.gallery.len(), 2, "the entry without a file is skipped");
        assert_eq!(d.gallery[1].caption.as_deref(), Some("back view"));
        assert_eq!(
            d.gallery[1]
                .variants
                .iter()
                .map(|v| v.size)
                .collect::<Vec<_>>(),
            vec![ImageSize::W100]
        );
        assert_eq!(d.audio_preview_url, None);
        assert_eq!(
            d.short_description.as_deref(),
            Some("A copper-toned coat with brass buttons.")
        );
        assert_eq!(
            d.description_html,
            "Swaps the coat for a <b>copper</b> one.<br><a href=\"https://example.org/notes\">notes</a>"
        );
        assert_eq!(
            d.licence_html.as_deref(),
            Some("<p>Ask before reusing the textures.</p>")
        );
        assert_eq!((d.likes, d.views, d.downloads), (42, 3100, 5120));
        assert_eq!(d.date_added, Some(1_760_000_000));
        assert_eq!(d.date_modified, Some(1_761_000_000));
        assert_eq!(d.date_updated, Some(1_761_000_100));
        assert_eq!(d.initial_visibility, Visibility::Warn);
        assert_eq!(
            d.content_ratings,
            vec![
                ContentRating::SkimpyAttire,
                ContentRating::SexualThemes,
                ContentRating::Other {
                    code: "xx".into(),
                    label: "Something New".into()
                },
            ]
        );
        assert!(!d.obsolete);
        assert_eq!(
            d.embedded_media,
            vec!["https://www.youtube.com/watch?v=abcdefghijk".to_owned()]
        );
        assert_eq!(
            d.alternate_sources,
            vec![
                AlternateSource {
                    url: "https://example.org/copper-coat".into(),
                    description: None
                },
                AlternateSource {
                    url: "https://example.org/copper-coat/mirror".into(),
                    description: Some("Project page".into())
                },
            ]
        );
    }

    #[test]
    fn mod_profile_files() {
        let d = parse_details(Section::Mod, fixture!("mod_profile.json")).expect("parses");
        assert_eq!(
            d.files[0],
            RemoteFile {
                id: 1_800_002,
                file_name: "copper_coat.7z".into(),
                size_bytes: 12_345_678,
                date_added: Some(1_761_000_000),
                download_count: 4320,
                description: None,
                version: Some("1.2".into()),
                md5: Some("fedcba9876543210fedcba9876543210".into()),
                antivirus: Antivirus::Clean,
                download_url: "https://gamebanana.com/dl/1800002".into(),
            }
        );
        let pending = &d.files[1];
        assert_eq!(pending.md5, None, "empty checksum");
        assert_eq!(pending.description, None, "blank description");
        assert_eq!(pending.date_added, None);
        assert_eq!(pending.antivirus, Antivirus::Unfinished("pending".into()));
        assert_eq!(
            d.files[2].antivirus,
            Antivirus::Flagged("suspicious".into())
        );
        assert_eq!(d.files[3].antivirus, Antivirus::NotReported);
        assert_eq!(d.files[3].download_count, 0);

        assert_eq!(d.archived_files.len(), 1);
        let old = &d.archived_files[0];
        assert_eq!(old.id, 1_800_001);
        assert_eq!(old.description.as_deref(), Some("First release"));
        assert_eq!(old.antivirus, Antivirus::Clean);
    }

    #[test]
    fn sound_profile() {
        let d = parse_details(Section::Sound, fixture!("sound_profile.json")).expect("parses");
        assert_eq!(d.section, Section::Sound);
        assert_eq!(d.version, None);
        assert_eq!(d.short_description, None);
        assert_eq!(d.description_html, "");
        assert_eq!(d.licence_html, None);
        assert_eq!(
            d.category_path,
            vec![CategoryRef {
                id: Some(6270),
                name: "Abilities".into()
            }]
        );
        assert!(d.gallery.is_empty());
        assert_eq!(
            d.audio_preview_url.as_deref(),
            Some("https://files.gamebanana.com/preview/sounds/bell_chime.mp3")
        );
        assert!(d.content_ratings.is_empty());
        assert_eq!(d.initial_visibility, Visibility::Show);
        assert!(d.archived_files.is_empty());
        assert!(d.alternate_sources.is_empty());
        assert_eq!(d.files.len(), 1);
        assert_eq!(d.credits[0].authors[0].role.as_deref(), Some("Creator"));
    }

    #[test]
    fn profile_of_another_game_is_refused() {
        let body = br#"{"_idRow": 1, "_sName": "Elsewhere", "_aGame": {"_idRow": 297}}"#;
        assert_eq!(
            parse_details(Section::Mod, body),
            Err(Error::WrongGame { game_id: 297 })
        );
    }

    #[test]
    fn minimal_profile_parses() {
        let d = parse_details(Section::Mod, br#"{"_idRow": 5, "_sName": "Bare"}"#).expect("parses");
        assert_eq!(d.name, "Bare");
        assert!(d.files.is_empty());
        assert!(d.credits.is_empty());
        assert!(d.category_path.is_empty());
        assert_eq!(d.initial_visibility, Visibility::Show);
    }

    #[test]
    fn ratings_without_a_visibility_hide_the_submission() {
        let body =
            br#"{"_idRow": 5, "_sName": "Rated", "_aContentRatings": {"nu": "Full Nudity"}}"#;
        let d = parse_details(Section::Mod, body).expect("parses");
        assert_eq!(d.content_ratings, vec![ContentRating::FullNudity]);
        assert_eq!(d.initial_visibility, Visibility::Hide);
        let empty = br#"{"_idRow": 5, "_sName": "Unrated", "_aContentRatings": []}"#;
        assert!(
            parse_details(Section::Mod, empty)
                .expect("parses")
                .content_ratings
                .is_empty()
        );
    }

    #[test]
    fn unknown_visibility_fails_safe() {
        let body = br#"{"_aMetadata": {"_nRecordCount": 1, "_bIsComplete": true}, "_aRecords": [
            {"_idRow": 1, "_sModelName": "Mod", "_sName": "x", "_sInitialVisibility": "blur"}]}"#;
        let page = parse_list(Section::Mod, body).expect("parses");
        assert_eq!(page.items[0].initial_visibility, Visibility::Hide);
    }

    #[test]
    fn known_content_rating_codes() {
        let body = br#"{"_idRow": 5, "_sName": "r", "_aContentRatings":
            {"st": "a", "sc": "b", "sa": "c", "lp": "d", "nu": "e"}}"#;
        let mut ratings = parse_details(Section::Mod, body)
            .expect("parses")
            .content_ratings;
        ratings.sort_by_key(|r| format!("{r:?}"));
        assert_eq!(
            ratings,
            vec![
                ContentRating::FullNudity,
                ContentRating::LewdAnglesAndPoses,
                ContentRating::SexualContent,
                ContentRating::SexualThemes,
                ContentRating::SkimpyAttire,
            ]
        );
    }

    #[test]
    fn error_document_is_an_api_change() {
        let change =
            changed(parse_list(Section::Mod, fixture!("error_input.json")).expect_err("error"));
        assert_eq!(change.endpoint, "Mod/Index");
        assert_eq!(
            change.reason,
            ApiChangeReason::ErrorCode {
                code: "INPUT_ERRORS".into(),
                details: vec!["_sSort=UNKNOWN_SORT".into()],
            }
        );
        let change = changed(
            parse_categories(Section::Sound, fixture!("error_input.json")).expect_err("error"),
        );
        assert_eq!(change.endpoint, "Sound/Categories");
        let change = changed(parse_search(fixture!("error_input.json")).expect_err("error"));
        assert_eq!(change.endpoint, "Util/Search/Results");
    }

    #[test]
    fn missing_record_is_not_found() {
        assert_eq!(
            parse_details(Section::Mod, fixture!("error_not_found.json")),
            Err(Error::NotFound)
        );
    }

    #[test]
    fn missing_required_field_is_an_api_change() {
        let no_id = br#"{"_aMetadata": {"_nRecordCount": 1, "_bIsComplete": true},
            "_aRecords": [{"_sModelName": "Mod", "_sName": "x"}]}"#;
        let change = changed(parse_list(Section::Mod, no_id).expect_err("error"));
        assert_eq!(
            change.reason,
            ApiChangeReason::MissingField("_idRow".into())
        );

        let no_meta = br#"{"_aRecords": []}"#;
        let change = changed(parse_list(Section::Mod, no_meta).expect_err("error"));
        assert_eq!(
            change.reason,
            ApiChangeReason::MissingField("_aMetadata".into())
        );

        let file_without_url = br#"{"_idRow": 5, "_sName": "x",
            "_aFiles": [{"_idRow": 1, "_sFile": "a.zip", "_nFilesize": 1}]}"#;
        let change = changed(parse_details(Section::Mod, file_without_url).expect_err("error"));
        assert_eq!(change.endpoint, "Mod/ProfilePage");
        assert_eq!(
            change.reason,
            ApiChangeReason::MissingField("_sDownloadUrl".into())
        );

        let nested = br#"{"_idRow": 5, "_sName": "x", "_aCategory": {"_idRow": 3}}"#;
        let change = changed(parse_details(Section::Mod, nested).expect_err("error"));
        assert_eq!(
            change.reason,
            ApiChangeReason::MissingField("_sName".into())
        );
    }

    #[test]
    fn wrong_type_is_an_api_change() {
        let cases: [&[u8]; 4] = [
            br#"{"_idRow": "5", "_sName": "x"}"#,
            br#"{"_idRow": 5, "_sName": "x", "_nLikeCount": "many"}"#,
            br#"{"_idRow": 5, "_sName": "x", "_aContentRatings": ["nu"]}"#,
            br#"{"_idRow": 5, "_sName": "x", "_aFiles": {"_idRow": 1}}"#,
        ];
        for body in cases {
            let change = changed(parse_details(Section::Mod, body).expect_err("error"));
            assert!(
                matches!(change.reason, ApiChangeReason::UnexpectedShape(_)),
                "{change:?}"
            );
        }
        let change =
            changed(parse_categories(Section::Mod, br#"{"_idRow": 1}"#).expect_err("error"));
        assert!(matches!(change.reason, ApiChangeReason::UnexpectedShape(_)));
    }

    #[test]
    fn non_json_is_an_api_change() {
        for body in [&b"<html>maintenance</html>"[..], b"", b"{\"_idRow\": "] {
            let change = changed(parse_details(Section::Mod, body).expect_err("error"));
            assert_eq!(change.reason, ApiChangeReason::NotJson);
        }
    }

    #[test]
    fn http_error_with_error_document_is_an_answer() {
        let not_found = FetchError::Status {
            status: 404,
            body: fixture!("error_not_found.json").to_vec(),
        };
        assert_eq!(fetch_error("Mod/ProfilePage", not_found), Error::NotFound);

        let bad_input = FetchError::Status {
            status: 400,
            body: fixture!("error_input.json").to_vec(),
        };
        let change = changed(fetch_error("Mod/Index", bad_input));
        assert_eq!(change.endpoint, "Mod/Index");
    }

    #[test]
    fn http_error_without_error_document_stays_a_fetch_error() {
        for error in [
            FetchError::Status {
                status: 503,
                body: b"<html>busy</html>".to_vec(),
            },
            FetchError::Status {
                status: 500,
                body: br#"{"message": "oops"}"#.to_vec(),
            },
            FetchError::Transport("timed out".into()),
        ] {
            assert_eq!(fetch_error("Mod/Index", error.clone()), Error::Fetch(error));
        }
    }

    #[test]
    fn category_id_from_profile_url() {
        assert_eq!(
            id_from_category_url("https://gamebanana.com/mods/cats/33295"),
            Some(33_295)
        );
        assert_eq!(
            id_from_category_url("https://gamebanana.com/sounds/cats/6270/"),
            Some(6270)
        );
        assert_eq!(
            id_from_category_url("https://gamebanana.com/mods/cats/x"),
            None
        );
        assert_eq!(
            id_from_category_url("https://gamebanana.com/mods/33295"),
            None
        );
    }
}
