//! What an update check asks GameBanana (decision 164): a mod's current files or why it is gone, and its update posts.

mod support;

use deadloader_catalog::{CatalogError, Initiator};
use deadloader_core::updates::{RemoteMod, RemovedReason};
use deadloader_gamebanana::{Section, request};
use support::*;

/// The mod recorded in `mod_files.json` and `mod_updates.json` (Manhattan Cafe).
const MOD: u64 = 638_388;

fn files_url() -> String {
    request::file_list_url(Section::Mod, MOD)
}

#[test]
fn a_listed_mod_comes_with_its_files() {
    let h = Harness::new(&online_settings());
    h.on(&files_url(), Reply::ok(fixture!("mod_files.json")));
    let RemoteMod::Listed(listed) = h
        .catalog
        .remote_mod(Section::Mod, MOD, Initiator::Auto)
        .expect("answered")
    else {
        panic!("listed");
    };
    assert_eq!(listed.version.as_deref(), Some("v5.3"));
    assert!(!listed.files.is_empty());
    assert!(
        listed
            .files
            .iter()
            .all(|f| f.id > 0 && !f.file_name.is_empty())
    );
    assert!(listed.posts.is_empty(), "posts are fetched separately");
}

#[test]
fn a_deleted_mod_is_removed_not_an_error() {
    let h = Harness::new(&online_settings());
    h.on(
        &files_url(),
        Reply::status(404).body(fixture!("error_not_found.json")),
    );
    assert_eq!(
        h.catalog.remote_mod(Section::Mod, MOD, Initiator::Auto),
        Ok(RemoteMod::Removed(RemovedReason::Deleted))
    );
}

#[test]
fn a_withheld_mod_or_one_without_files_is_removed() {
    let h = Harness::new(&online_settings());
    let text = std::str::from_utf8(fixture!("mod_files.json")).expect("utf-8");
    let withheld = text.replace(r#""_bIsWithheld": false"#, r#""_bIsWithheld": true"#);
    h.on(&files_url(), Reply::ok(withheld.as_bytes()));
    assert_eq!(
        h.catalog.remote_mod(Section::Mod, MOD, Initiator::Auto),
        Ok(RemoteMod::Removed(RemovedReason::Withheld))
    );

    let mut value: serde_json::Value = serde_json::from_str(text).expect("json");
    value["_aFiles"] = serde_json::json!([]);
    h.transport
        .replace(&files_url(), Reply::ok(value.to_string().as_bytes()));
    assert_eq!(
        h.catalog.remote_mod(Section::Mod, MOD, Initiator::Auto),
        Ok(RemoteMod::Removed(RemovedReason::NoFiles))
    );
}

#[test]
fn local_mode_asks_nothing() {
    let h = Harness::new(&deadloader_core::settings::Settings::default());
    assert_eq!(
        h.catalog.remote_mod(Section::Mod, MOD, Initiator::User),
        Err(CatalogError::LocalMode)
    );
    assert_eq!(
        h.catalog.update_posts(Section::Mod, MOD, Initiator::User),
        Err(CatalogError::LocalMode)
    );
    assert_eq!(h.transport.count(), 0);
}

#[test]
fn update_posts_are_the_first_page() {
    let h = Harness::new(&online_settings());
    h.on(
        &request::updates_url(Section::Mod, MOD, 1),
        Reply::ok(fixture!("mod_updates.json")),
    );
    let posts = h
        .catalog
        .update_posts(Section::Mod, MOD, Initiator::User)
        .expect("posts");
    assert_eq!(posts.len(), 9);
    assert_eq!(posts[0].version.as_deref(), Some("v5.3"));
}

#[test]
fn a_busy_gamebanana_holds_update_checks_back_too() {
    let h = Harness::new(&online_settings());
    h.on(&files_url(), Reply::status(429).header("Retry-After", "60"));
    assert!(matches!(
        h.catalog.remote_mod(Section::Mod, MOD, Initiator::Auto),
        Err(CatalogError::RateLimited { .. })
    ));
    assert!(matches!(
        h.catalog.remote_mod(Section::Mod, MOD, Initiator::Auto),
        Err(CatalogError::RateLimited { .. })
    ));
    assert_eq!(h.transport.count(), 1, "nothing asked while waiting");
}
