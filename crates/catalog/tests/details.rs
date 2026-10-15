//! Mod pages: fetched once, kept as cards by update date, sanitised descriptions, NSFW, and the file list for
//! downloads.

mod support;

use deadloader_catalog::{Access, CatalogError, Event, Initiator, Origin};
use deadloader_core::settings::NsfwMode;
use deadloader_gamebanana::{Section, request};
use std::time::Duration;
use support::*;

const MOD: u64 = 700_001;
/// `_tsDateUpdated` of the recorded mod page.
const UPDATED: i64 = 1_761_000_100;

fn page_url() -> String {
    request::details_url(Section::Mod, MOD)
}

fn harness(text: &str, rated: bool, nsfw: NsfwMode) -> Harness {
    let settings = if nsfw == NsfwMode::Hidden {
        online_settings()
    } else {
        with_nsfw(online_settings(), nsfw)
    };
    let h = Harness::new(&settings);
    h.on(&page_url(), Reply::ok(&profile_with(text, rated)));
    h
}

#[test]
fn the_description_is_sanitised_and_its_links_listed() {
    let h = harness(
        r#"Hi <script>alert(1)</script><b>bold</b> <a href="https://example.org/notes">notes</a>"#,
        false,
        NsfwMode::Hidden,
    );
    let page = h
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    assert_eq!(page.origin, Origin::Network);
    assert_eq!(page.name, "Copper Coat for Wraith");
    assert!(!page.description.html.contains("script"));
    assert!(!page.description.html.contains("href"));
    assert!(page.description.html.contains("<b>bold</b>"));
    assert_eq!(page.description.links.len(), 1);
    assert_eq!(page.description.links[0].url, "https://example.org/notes");
    assert_eq!(page.description.links[0].host, "example.org");
    assert_eq!(h.transport.calls(), [page_url()]);
    assert_eq!(h.net_log.all()[0].initiator, Initiator::User);
}

#[test]
fn plain_http_links_in_a_description_are_upgraded_to_https() {
    let h = harness(
        r#"<a href="http://example.org/old?a=1">old</a> <a href="HTTP://Example.com/x">x</a> <a href="https://example.net/">new</a>"#,
        false,
        NsfwMode::Hidden,
    );
    let page = h
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    let urls: Vec<&str> = page
        .description
        .links
        .iter()
        .map(|l| l.url.as_str())
        .collect();
    assert_eq!(
        urls,
        [
            "https://example.org/old?a=1",
            "https://example.com/x",
            "https://example.net/"
        ]
    );
}

#[test]
fn files_are_listed_without_download_urls_and_kept_for_downloads() {
    let h = harness("text", false, NsfwMode::Hidden);
    let page = h
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    assert_eq!(
        page.files.iter().map(|f| f.id).collect::<Vec<_>>(),
        [1_800_002, 1_800_003, 1_800_004, 1_800_005]
    );
    let json = serde_json::to_string(&page).expect("serialises for the UI");
    assert!(!json.contains("gamebanana.com/dl/"), "{json}");

    let file = h
        .catalog
        .remote_file(Section::Mod, MOD, 1_800_003)
        .expect("from the saved page");
    assert_eq!(file.file_name, "copper_coat_gloves_only.zip");
    assert!(file.download_url.starts_with("https://"));
    assert!(h.catalog.remote_file(Section::Mod, MOD, 1).is_none());
    assert!(h.catalog.remote_file(Section::Mod, 1, 1_800_003).is_none());
    assert_eq!(h.transport.count(), 1);
}

#[test]
fn a_download_shows_the_first_picture_of_its_saved_page_at_card_size() {
    let h = harness("text", false, NsfwMode::Hidden);
    assert_eq!(
        h.catalog.download_thumbnail(Section::Mod, MOD),
        None,
        "not saved yet: nothing is requested for it"
    );
    assert_eq!(h.transport.count(), 0);
    h.catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    let id = h
        .catalog
        .download_thumbnail(Section::Mod, MOD)
        .expect("from the saved page");
    assert_eq!(
        h.catalog.download_thumbnail(Section::Mod, MOD),
        Some(id),
        "the same picture every time, also after a restart"
    );
    assert_eq!(h.catalog.download_thumbnail(Section::Mod, 1), None);
    assert_eq!(h.transport.count(), 1, "no request of its own");
    h.on(THUMB_220, Reply::image("image/jpeg", JPEG));
    h.catalog.image(&id).expect("the 220 px picture");
    assert_eq!(h.net_log.all()[1].initiator, Initiator::Auto);
}

#[test]
fn a_download_of_an_nsfw_mod_shows_its_picture_only_when_nsfw_is_shown() {
    for (mode, shown) in [
        (NsfwMode::Hidden, false),
        (NsfwMode::Blurred, false),
        (NsfwMode::Shown, true),
    ] {
        let h = harness("text", true, mode);
        h.catalog
            .details(Section::Mod, MOD, None, Initiator::User)
            .expect("page");
        assert_eq!(
            h.catalog.download_thumbnail(Section::Mod, MOD).is_some(),
            shown,
            "{mode:?}"
        );
    }
}

#[test]
fn a_saved_page_is_current_while_the_listing_shows_no_newer_update() {
    let h = harness("text", false, NsfwMode::Hidden);
    h.catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    // Long after the page TTL, the card is still current for the update date the grid shows.
    h.clock.advance(Duration::from_secs(24 * 3600));
    let page = h
        .catalog
        .details(Section::Mod, MOD, Some(UPDATED), Initiator::User)
        .expect("page");
    assert!(matches!(page.origin, Origin::Cache { stale: false, .. }));
    assert_eq!(h.transport.count(), 1);
    assert_eq!(h.jobs.len(), 0);

    // The listing shows a newer update: served at once, refreshed in the background.
    let page = h
        .catalog
        .details(Section::Mod, MOD, Some(UPDATED + 1), Initiator::User)
        .expect("page");
    assert!(matches!(page.origin, Origin::Cache { stale: true, .. }));
    assert_eq!(h.jobs.run(), 1);
    assert_eq!(h.transport.count(), 2);
    assert!(h.events.take().contains(&Event::DetailsUpdated {
        section: Section::Mod,
        id: MOD
    }));

    // Without a listing date the page TTL decides.
    let page = h
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    assert!(matches!(page.origin, Origin::Cache { stale: false, .. }));
    h.clock.advance(Duration::from_secs(600));
    let page = h
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    assert!(matches!(page.origin, Origin::Cache { stale: true, .. }));
}

#[test]
fn a_removed_mod_is_not_found() {
    let h = Harness::new(&online_settings());
    h.on(
        &page_url(),
        Reply {
            status: 404,
            headers: Vec::new(),
            body: fixture!("error_not_found.json").to_vec(),
        },
    );
    assert_eq!(
        h.catalog
            .details(Section::Mod, MOD, None, Initiator::User)
            .map(|_| ()),
        Err(CatalogError::NotFound { other_game: None })
    );
    assert_eq!(h.catalog.status(), deadloader_catalog::Status::Online);
}

#[test]
fn local_mode_shows_a_saved_page_but_requests_nothing() {
    let h = harness("text", false, NsfwMode::Hidden);
    h.catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    h.clock.advance(Duration::from_secs(3600));
    h.catalog
        .apply_settings(&deadloader_core::settings::Settings::default());
    let page = h
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("saved page");
    assert!(matches!(page.origin, Origin::Cache { stale: true, .. }));
    assert_eq!(h.jobs.len(), 0, "no refresh in local mode");
    let thumb = page.gallery[0].preview;
    assert_eq!(
        h.catalog.image(&thumb),
        Err(deadloader_catalog::ImageError::LocalMode)
    );
    assert_eq!(h.transport.count(), 1);
}

#[test]
fn nsfw_pages_are_flagged_and_hidden_ones_show_no_pictures() {
    let text = r#"<img src="https://images.gamebanana.com/img/ss/mods/x.jpg">"#;
    let shown = harness(text, true, NsfwMode::Shown);
    let page = shown
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    assert!(page.nsfw);
    assert!(!page.nsfw_hidden);
    assert!(!page.gallery.is_empty());
    assert!(!page.content_ratings.is_empty());

    let hidden = harness(text, true, NsfwMode::Hidden);
    let page = hidden
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    assert!(page.nsfw && page.nsfw_hidden);
    assert!(page.gallery.is_empty());
    // Description images are never loaded by themselves on a hidden page.
    assert_eq!(page.description.images[0].access, Access::Click);
}

#[test]
fn the_gallery_never_stretches_a_smaller_variant() {
    let h = harness("text", false, NsfwMode::Hidden);
    let page = h
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    // The first picture has a 530 px variant but no 800 px one: larger displays get the original.
    let first = &page.gallery[0];
    assert_ne!(first.preview, first.large);
    assert_eq!(first.large, first.full);
    // The second has only 100 px: every display gets the original, never 100 px stretched to the gallery.
    let second = &page.gallery[1];
    assert_eq!(second.preview, second.large);
    assert_eq!(second.large, second.full);
    let original = "https://images.gamebanana.com/img/ss/mods/aaaa0002.jpg";
    h.on(original, Reply::image("image/jpeg", JPEG));
    h.catalog.image(&second.preview).expect("preview");
    assert_eq!(
        h.transport.calls().last().map(String::as_str),
        Some(original)
    );
    assert_eq!(
        h.net_log.all().last().map(|r| r.initiator),
        Some(Initiator::Auto),
        "shown with the page, not on a click"
    );
    // "View original" afterwards is the same file: served from the cache.
    h.catalog.image(&second.original).expect("original");
    assert_eq!(h.transport.calls().len(), 2);
}

#[test]
fn the_gallery_has_a_preview_and_the_original_of_each_image() {
    let h = harness("text", false, NsfwMode::Hidden);
    let page = h
        .catalog
        .details(Section::Mod, MOD, None, Initiator::User)
        .expect("page");
    let first = &page.gallery[0];
    assert_ne!(first.preview, first.original);
    h.on(
        "https://images.gamebanana.com/img/ss/mods/530-90_aaaa0001.jpg",
        Reply::image("image/jpeg", JPEG),
    );
    h.on(
        "https://images.gamebanana.com/img/ss/mods/aaaa0001.jpg",
        Reply::image("image/jpeg", JPEG),
    );
    h.catalog.image(&first.preview).expect("preview");
    h.catalog.image(&first.original).expect("original");
    let records = h.net_log.all();
    assert_eq!(records[1].initiator, Initiator::Auto);
    assert_eq!(
        records[2].initiator,
        Initiator::User,
        "the original is on demand"
    );
}
