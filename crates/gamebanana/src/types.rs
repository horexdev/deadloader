//! deadloader's own view of GameBanana content. Nothing here mirrors GameBanana's field names, so the rest of the
//! app keeps compiling unchanged when the API does. Timestamps are Unix seconds, as GameBanana sends them.

/// The GameBanana sections the catalog shows (decision 131).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Section {
    Mod,
    Sound,
}

/// One page of a catalog listing or a search.
#[derive(Debug, Clone, PartialEq)]
pub struct CatalogPage {
    pub items: Vec<CatalogItem>,
    /// Records matching the query across all pages, as GameBanana counts them.
    pub total: u64,
    /// No further pages exist.
    pub is_complete: bool,
}

/// What a catalog card needs. Content ratings are only known as "has some" here; the specific ones come with
/// [`ModDetails`], so a hidden mod's thumbnail never has to be fetched to decide (decision 27).
#[derive(Debug, Clone, PartialEq)]
pub struct CatalogItem {
    pub id: u64,
    pub section: Section,
    pub name: String,
    pub author: Option<String>,
    pub root_category: Option<CategoryRef>,
    /// For Skins this is the hero.
    pub subcategory: Option<CategoryRef>,
    pub likes: u64,
    pub views: u64,
    pub date_added: Option<i64>,
    pub date_modified: Option<i64>,
    pub date_updated: Option<i64>,
    /// The first preview image; GameBanana only reliably resizes the first one.
    pub thumbnail: Option<Image>,
    /// Sounds carry an audio preview instead of images.
    pub audio_preview_url: Option<String>,
    pub has_content_ratings: bool,
    pub initial_visibility: Visibility,
    pub obsolete: bool,
    pub featured: bool,
    pub has_files: bool,
}

/// How GameBanana suggests showing a submission before the viewer opts in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibility {
    Show,
    /// Show behind a warning (blurred).
    Warn,
    Hide,
}

/// A category as other records point to it. The id can be missing when GameBanana only gives a name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategoryRef {
    pub id: Option<u64>,
    pub name: String,
}

/// An entry of a category listing (root categories, or heroes under Skins).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub item_count: u64,
    pub subcategory_count: u64,
}

/// A preview image with the resized variants GameBanana has for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    /// The uploaded file, full size. Large; fetched only on demand.
    pub original_url: String,
    pub caption: Option<String>,
    /// Ordered by width, smallest first.
    pub variants: Vec<ImageVariant>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageVariant {
    pub size: ImageSize,
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// GameBanana's resize steps, named by their width in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ImageSize {
    W100,
    W220,
    W530,
    W800,
}

impl Image {
    pub fn variant(&self, size: ImageSize) -> Option<&ImageVariant> {
        self.variants.iter().find(|v| v.size == size)
    }

    /// The URL a catalog card of `card_width_px` CSS pixels should load at `display_scale` (decision 152): the 220 px
    /// variant while it is not upscaled, otherwise 530 px. A missing variant is replaced by the nearest larger one,
    /// then the nearest smaller one; the original is the last resort.
    pub fn thumbnail_url(&self, card_width_px: u32, display_scale: f64) -> &str {
        use ImageSize::{W100, W220, W530, W800};
        let scale = if display_scale.is_finite() && display_scale > 0.0 {
            display_scale
        } else {
            1.0
        };
        let order: [ImageSize; 4] = if f64::from(card_width_px) * scale <= 220.0 {
            [W220, W530, W800, W100]
        } else {
            [W530, W800, W220, W100]
        };
        order
            .iter()
            .find_map(|&size| self.variant(size))
            .map_or(self.original_url.as_str(), |v| v.url.as_str())
    }
}

/// Everything the mod page shows. Used for sounds too.
#[derive(Debug, Clone, PartialEq)]
pub struct ModDetails {
    pub id: u64,
    pub section: Section,
    pub name: String,
    pub version: Option<String>,
    pub author: Option<String>,
    pub credits: Vec<CreditGroup>,
    /// Root first: e.g. Skins → Celeste.
    pub category_path: Vec<CategoryRef>,
    pub gallery: Vec<Image>,
    pub audio_preview_url: Option<String>,
    pub short_description: Option<String>,
    /// Untrusted HTML exactly as received; sanitising is the renderer's job.
    pub description_html: String,
    /// Untrusted HTML exactly as received.
    pub licence_html: Option<String>,
    pub likes: u64,
    pub views: u64,
    pub downloads: u64,
    pub date_added: Option<i64>,
    pub date_modified: Option<i64>,
    pub date_updated: Option<i64>,
    pub initial_visibility: Visibility,
    pub content_ratings: Vec<ContentRating>,
    pub obsolete: bool,
    /// Video links (YouTube) the author attached.
    pub embedded_media: Vec<String>,
    /// Links to other places (a repository, a chat); shown as external links, never downloaded from (decision 33).
    pub alternate_sources: Vec<AlternateSource>,
    pub files: Vec<RemoteFile>,
    /// Superseded files, kept by GameBanana for older versions.
    pub archived_files: Vec<RemoteFile>,
    pub availability: Availability,
}

/// Whether GameBanana still shows a submission to everyone. A submission that is gone entirely is
/// [`crate::Error::NotFound`] instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Availability {
    Public,
    /// Hidden by GameBanana's moderators, usually until the author fixes something.
    Withheld,
    /// Deleted into GameBanana's trash by the author or a moderator.
    Trashed,
    /// Hidden by the author.
    Private,
}

/// The cheap answer an update check needs: a submission's current files and state, without the description,
/// gallery and credits of [`ModDetails`].
#[derive(Debug, Clone, PartialEq)]
pub struct ModFileList {
    pub id: u64,
    pub section: Section,
    pub name: String,
    pub version: Option<String>,
    /// Moves with every update post, but also for other edits, so it is not an update signal by itself.
    pub date_updated: Option<i64>,
    pub availability: Availability,
    pub files: Vec<RemoteFile>,
}

/// One page of a submission's update posts, newest first.
#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePage {
    /// Trashed and private posts are left out.
    pub posts: Vec<UpdatePost>,
    /// Posts across all pages, as GameBanana counts them.
    pub total: u64,
    pub is_complete: bool,
}

/// An author's update post: what changed and which files came with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatePost {
    pub id: u64,
    pub title: String,
    /// Free text chosen by the author (`v5.3`, `2.0`, …); not comparable in general.
    pub version: Option<String>,
    pub date_added: Option<i64>,
    pub changelog: Vec<ChangeEntry>,
    /// Ids of the files uploaded with this post. Files are re-uploaded under new ids, so older posts usually list
    /// ids that no longer exist.
    pub file_ids: Vec<u64>,
    /// Untrusted HTML exactly as received; empty when the post has no text.
    pub text_html: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeEntry {
    /// The author's label, e.g. `Bugfix` or `Addition`; free text.
    pub category: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditGroup {
    pub name: String,
    pub authors: Vec<Credit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credit {
    pub name: String,
    pub role: Option<String>,
}

/// A specific content rating. Unknown ones keep GameBanana's code and English label so nothing is silently lost.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentRating {
    SexualThemes,
    SexualContent,
    SkimpyAttire,
    LewdAnglesAndPoses,
    FullNudity,
    Other { code: String, label: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternateSource {
    pub url: String,
    pub description: Option<String>,
}

/// A downloadable file of a submission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteFile {
    pub id: u64,
    pub file_name: String,
    pub size_bytes: u64,
    pub date_added: Option<i64>,
    pub download_count: u64,
    pub description: Option<String>,
    pub version: Option<String>,
    /// Hex MD5 as GameBanana reports it; verified after download (decision 46).
    pub md5: Option<String>,
    pub antivirus: Antivirus,
    pub download_url: String,
}

/// GameBanana's own antivirus scan of a file, shown next to it (decision 131).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Antivirus {
    /// The scan finished and found nothing.
    Clean,
    /// The scan finished with another verdict, kept verbatim.
    Flagged(String),
    /// The scan has not finished; GameBanana's state is kept verbatim.
    Unfinished(String),
    /// GameBanana sent no scan information.
    NotReported,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn variant(size: ImageSize, name: &str) -> ImageVariant {
        ImageVariant {
            size,
            url: format!("https://images.example/{name}"),
            width: None,
            height: None,
        }
    }

    fn image(sizes: &[ImageSize]) -> Image {
        Image {
            original_url: "https://images.example/original".to_owned(),
            caption: None,
            variants: sizes
                .iter()
                .map(|&s| variant(s, &format!("{s:?}")))
                .collect(),
        }
    }

    fn pick(sizes: &[ImageSize], width: u32, scale: f64) -> String {
        image(sizes)
            .thumbnail_url(width, scale)
            .trim_start_matches("https://images.example/")
            .to_owned()
    }

    #[test]
    fn thumbnail_choice_table() {
        use ImageSize::{W100, W220, W530, W800};
        let all = [W100, W220, W530, W800];
        let cases: &[(&[ImageSize], u32, f64, &str)] = &[
            // 220 px while the card is not upscaled…
            (&all, 220, 1.0, "W220"),
            (&all, 176, 1.25, "W220"),
            (&all, 146, 1.5, "W220"),
            (&all, 110, 2.0, "W220"),
            // …and 530 px as soon as it would be.
            (&all, 221, 1.0, "W530"),
            (&all, 180, 1.25, "W530"),
            (&all, 298, 1.5, "W530"),
            (&all, 600, 1.0, "W530"),
            // Missing variants: nearest larger first, then smaller.
            (&[W100, W530], 200, 1.0, "W530"),
            (&[W100, W800], 200, 1.0, "W800"),
            (&[W100], 200, 1.0, "W100"),
            (&[W100, W220, W800], 300, 1.0, "W800"),
            (&[W100, W220], 300, 1.0, "W220"),
            (&[W100], 300, 1.0, "W100"),
            // Nothing resized: the original.
            (&[], 200, 1.0, "original"),
            (&[], 300, 1.0, "original"),
        ];
        for &(sizes, width, scale, expected) in cases {
            assert_eq!(
                pick(sizes, width, scale),
                expected,
                "sizes {sizes:?}, width {width}, scale {scale}"
            );
        }
    }

    #[test]
    fn nonsense_scale_counts_as_one() {
        use ImageSize::{W220, W530};
        for scale in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert_eq!(pick(&[W220, W530], 200, scale), "W220", "scale {scale}");
            assert_eq!(pick(&[W220, W530], 300, scale), "W530", "scale {scale}");
        }
    }
}
