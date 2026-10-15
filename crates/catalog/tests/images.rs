//! Thumbnails and description images: opaque ids, the cache, the network purposes, and the consent flows of
//! decision 122.

mod support;

use deadloader_catalog::{
    Access, ConsentError, ConsentPrompt, ImageError, ImageId, Initiator, Query, Scope,
};
use deadloader_core::cache::ImageType;
use deadloader_core::message::MessageId;
use deadloader_core::settings::{DescriptionImages, NsfwMode};
use deadloader_gamebanana::{Section, Sort, request};
use deadloader_net::{HostSource, Purpose};
use std::sync::atomic::Ordering;
use support::*;

fn listed(h: &Harness) -> deadloader_catalog::Listing {
    h.on(
        &list_url(Section::Mod, Sort::Newest, 1, None),
        Reply::ok(fixture!("mod_index.json")),
    );
    h.catalog
        .list(
            &Query::browse(Section::Mod, Sort::Newest, None, 1),
            Initiator::User,
        )
        .expect("page")
}

#[test]
fn thumbnails_are_fetched_once_and_then_served_from_the_cache() {
    let h = Harness::new(&online_settings());
    let page = listed(&h);
    let thumb = page.items[0].thumbnail.expect("thumbnail");
    assert_ne!(thumb.small, thumb.large);
    h.on(THUMB_220, Reply::image("image/jpeg", JPEG));

    let image = h.catalog.image(&thumb.small).expect("thumbnail");
    assert_eq!(image.bytes, JPEG);
    assert_eq!(image.content_type, ImageType::Jpeg);
    let record = &h.net_log.all()[1];
    assert_eq!(record.url, THUMB_220);
    assert_eq!(record.purpose, Purpose::Thumbnail);
    assert_eq!(record.initiator, Initiator::Auto);

    let again = h.catalog.image(&thumb.small).expect("cached");
    assert_eq!(again, image);
    assert_eq!(h.transport.count(), 2, "page + one thumbnail");
    assert!(
        h.fs.file_names().iter().any(|n| n.contains("\\thumbs\\")),
        "{:?}",
        h.fs.file_names()
    );
}

#[test]
fn the_large_thumbnail_is_the_530_px_variant() {
    let h = Harness::new(&online_settings());
    let thumb = listed(&h).items[0].thumbnail.expect("thumbnail");
    h.on(THUMB_530, Reply::image("image/jpeg", JPEG));
    h.catalog.image(&thumb.large).expect("thumbnail");
    assert_eq!(h.transport.calls()[1], THUMB_530);
}

#[test]
fn ids_are_opaque_and_only_issued_ids_resolve() {
    let h = Harness::new(&online_settings());
    let thumb = listed(&h).items[0].thumbnail.expect("thumbnail");
    let text = thumb.small.to_string();
    assert_eq!(text.len(), 32);
    assert!(
        text.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    );
    assert!(!text.contains("gamebanana"));
    assert_eq!(ImageId::parse(&text), Some(thumb.small));
    for bad in [
        "",
        "xyz",
        &text.to_uppercase(),
        &text[..31],
        &format!("{text}0"),
    ] {
        assert_eq!(ImageId::parse(bad), None, "{bad:?}");
    }

    let unknown = ImageId::parse("0123456789abcdef0123456789abcdef").expect("well formed");
    assert_eq!(h.catalog.image(&unknown), Err(ImageError::UnknownId));
    assert_eq!(h.transport.count(), 1);
}

#[test]
fn what_is_not_an_image_is_refused_and_not_cached() {
    let h = Harness::new(&online_settings());
    let thumb = listed(&h).items[0].thumbnail.expect("thumbnail");
    h.on(
        THUMB_220,
        Reply::image("image/jpeg", b"<html>not found</html>"),
    );
    assert_eq!(h.catalog.image(&thumb.small), Err(ImageError::NotAnImage));
    h.on(THUMB_530, Reply::status(404));
    assert_eq!(
        h.catalog.image(&thumb.large),
        Err(ImageError::Upstream { status: 404 })
    );
    assert!(
        !h.fs.file_names().iter().any(|n| n.contains("\\thumbs\\")),
        "{:?}",
        h.fs.file_names()
    );
}

#[test]
fn a_missing_content_type_is_taken_from_the_bytes_and_a_wrong_one_refused() {
    let h = Harness::new(&online_settings());
    let thumb = listed(&h).items[0].thumbnail.expect("thumbnail");
    h.on(THUMB_220, Reply::ok(JPEG));
    assert_eq!(
        h.catalog.image(&thumb.small).expect("sniffed").content_type,
        ImageType::Jpeg
    );
    h.on(THUMB_530, Reply::image("text/html", JPEG));
    assert_eq!(h.catalog.image(&thumb.large), Err(ImageError::NotAnImage));
}

#[test]
fn a_generic_binary_content_type_is_taken_from_the_bytes() {
    // Storage-backed hosts often label every file this way.
    for declared in ["application/octet-stream", "binary/octet-stream"] {
        let h = Harness::new(&online_settings());
        let thumb = listed(&h).items[0].thumbnail.expect("thumbnail");
        h.on(THUMB_220, Reply::image(declared, JPEG));
        let blob = h.catalog.image(&thumb.small).expect(declared);
        assert_eq!(blob.content_type, ImageType::Jpeg, "{declared}");
    }
}

#[test]
fn while_offline_thumbnails_come_only_from_the_cache() {
    let h = Harness::new(&online_settings());
    let thumb = listed(&h).items[0].thumbnail.expect("thumbnail");
    h.on(THUMB_220, Reply::image("image/jpeg", JPEG));
    h.catalog.image(&thumb.small).expect("thumbnail");

    // A failed catalog request puts the catalog offline.
    h.transport.set_offline(true);
    let _ = h.catalog.list(
        &Query::browse(Section::Mod, Sort::Newest, None, 2),
        Initiator::Auto,
    );
    let calls = h.transport.count();
    assert!(h.catalog.image(&thumb.small).is_ok(), "cached");
    assert_eq!(h.catalog.image(&thumb.large), Err(ImageError::Offline));
    assert_eq!(h.transport.count(), calls, "no attempt while offline");
}

fn mod_page(h: &Harness) -> deadloader_catalog::ModPage {
    h.catalog
        .details(Section::Mod, 700_001, None, Initiator::User)
        .expect("page")
}

const IMGUR_A: &str = "https://i.imgur.com/a.png";
const IMGUR_B: &str = "https://i.imgur.com/b.png";
const GB_IMG: &str = "https://images.gamebanana.com/img/ss/mods/desc.png";
const EMOTE: &str = "https://cdn.7tv.app/emote/1/2x.webp";

fn consent_harness(images: DescriptionImages) -> Harness {
    let text = format!(
        r#"<img src="{IMGUR_A}" alt="front"><img src="{GB_IMG}"><img src="//cdn.7tv.app/emote/1/2x.webp"><img src="http://i.imgur.com/b.png">"#
    );
    let h = Harness::new(&with_images(online_settings(), images));
    h.on(
        &request::details_url(Section::Mod, 700_001),
        Reply::ok(&profile_with(&text, false)),
    );
    for url in [IMGUR_A, IMGUR_B, GB_IMG] {
        h.on(url, Reply::image("image/png", PNG));
    }
    h.on(EMOTE, Reply::image("image/webp", b"RIFF\x10\0\0\0WEBPVP8 "));
    h
}

#[test]
fn description_images_wait_for_a_click_and_foreign_hosts_for_consent() {
    let h = consent_harness(DescriptionImages::OnClick);
    let page = mod_page(&h);
    let images = &page.description.images;
    let summary: Vec<_> = images
        .iter()
        .map(|i| (i.host.as_str(), i.alt.as_deref(), i.access))
        .collect();
    assert_eq!(
        summary,
        [
            ("i.imgur.com", Some("front"), Access::Consent),
            ("images.gamebanana.com", None, Access::Click),
            ("cdn.7tv.app", None, Access::Consent),
            ("i.imgur.com", None, Access::Consent),
        ]
    );
    // Hosts waiting for consent, each with its images.
    let pending: Vec<_> = page
        .description
        .consent
        .iter()
        .map(|p| (p.host.as_str(), p.images.clone()))
        .collect();
    assert_eq!(
        pending,
        [
            ("cdn.7tv.app", vec![images[2].id]),
            ("i.imgur.com", vec![images[0].id, images[3].id]),
        ]
    );

    // Nothing is loaded before the click, whatever the host.
    for image in images {
        assert_eq!(h.catalog.image(&image.id), Err(ImageError::NotActivated));
    }
    assert_eq!(h.transport.count(), 1, "only the mod page");

    // An allowed host loads on click, as the user's request.
    h.catalog.activate(&images[1].id).expect("allowed host");
    h.catalog.image(&images[1].id).expect("loaded");
    let record = h.net_log.all().pop().expect("record");
    assert_eq!(record.url, GB_IMG);
    assert_eq!(record.purpose, Purpose::DescriptionImage);
    assert_eq!(record.initiator, Initiator::User);
    assert!(
        h.fs.file_names().iter().any(|n| n.contains("\\images\\")),
        "description images live in their own area"
    );

    // A foreign host needs the dialog first.
    assert_eq!(
        h.catalog.activate(&images[0].id),
        Err(ConsentError::NeedsConsent {
            host: "i.imgur.com".to_owned()
        })
    );
    assert_eq!(
        h.catalog.consent_prompt(&images[0].id),
        Ok(ConsentPrompt {
            host: "i.imgur.com".to_owned(),
            always_allow_prechecked: true,
        })
    );
}

#[test]
fn consent_for_one_image_loads_only_that_image() {
    let h = consent_harness(DescriptionImages::OnClick);
    let page = mod_page(&h);
    let images = &page.description.images;

    let granted = h
        .catalog
        .grant(&images[0].id, Scope::Once)
        .expect("granted");
    assert_eq!(granted.activated, [images[0].id]);
    h.catalog.image(&images[0].id).expect("loaded");
    let record = h.net_log.all().pop().expect("record");
    assert_eq!(record.url, IMGUR_A);
    assert_eq!(record.host_source, Some(HostSource::UserImage));
    assert_eq!(record.initiator, Initiator::User);

    // The host is not remembered: the other imgur image still asks, nothing was saved.
    assert_eq!(
        h.catalog.image(&images[3].id),
        Err(ImageError::NotActivated)
    );
    assert!(matches!(
        h.catalog.activate(&images[3].id),
        Err(ConsentError::NeedsConsent { .. })
    ));
    assert!(lock(&h.saved.hosts).is_empty());
    assert_eq!(h.net.allowlist().image_hosts().count(), 0);
    assert!(
        h.lines
            .all()
            .iter()
            .any(|(_, m, f)| m.contains("once") && f["host"] == "i.imgur.com")
    );
}

#[test]
fn always_allowing_a_host_saves_it_and_frees_its_other_images() {
    let h = consent_harness(DescriptionImages::OnClick);
    let page = mod_page(&h);
    let images = &page.description.images;

    let granted = h
        .catalog
        .grant(&images[0].id, Scope::Always)
        .expect("granted");
    assert_eq!(
        granted.activated,
        [images[0].id],
        "on click: only the clicked one loads"
    );
    assert_eq!(*lock(&h.saved.hosts), ["i.imgur.com"]);
    assert_eq!(
        h.net
            .allowlist()
            .image_hosts()
            .map(|h| h.as_str().to_owned())
            .collect::<Vec<_>>(),
        ["i.imgur.com"]
    );
    h.catalog.image(&images[0].id).expect("loaded");
    assert_eq!(
        h.net_log.all().pop().expect("record").host_source,
        Some(HostSource::UserImage)
    );

    // The second imgur image now loads on a click without the dialog.
    h.catalog.activate(&images[3].id).expect("allowed now");
    h.catalog.image(&images[3].id).expect("loaded");
    assert_eq!(
        h.net_log.all().pop().expect("record").url,
        IMGUR_B,
        "http was upgraded to https"
    );

    // Viewed again, the page shows the host as allowed.
    let again = mod_page(&h);
    assert_eq!(
        again.description.images[3].access,
        Access::Load,
        "loaded before"
    );
    assert!(
        again
            .description
            .consent
            .iter()
            .all(|p| p.host != "i.imgur.com")
    );
}

#[test]
fn with_always_load_allowed_hosts_load_by_themselves() {
    let mut settings = with_images(online_settings(), DescriptionImages::Always);
    settings.catalog.allowed_image_hosts = vec!["cdn.7tv.app".to_owned()];
    settings.catalog.always_allow_host_prechecked = false;
    let h = consent_harness(DescriptionImages::Always);
    h.catalog.apply_settings(&settings);
    let page = mod_page(&h);
    let images = &page.description.images;
    assert_eq!(
        images.iter().map(|i| i.access).collect::<Vec<_>>(),
        [Access::Consent, Access::Load, Access::Load, Access::Consent]
    );
    h.catalog.image(&images[2].id).expect("loads by itself");
    let record = h.net_log.all().pop().expect("record");
    assert_eq!(record.initiator, Initiator::Auto);
    assert_eq!(record.host_source, Some(HostSource::UserImage));
    assert_eq!(
        h.catalog.consent_prompt(&images[0].id).expect("prompt"),
        ConsentPrompt {
            host: "i.imgur.com".to_owned(),
            always_allow_prechecked: false,
        }
    );

    // Always allowing imgur here loads every imgur image of the page.
    let granted = h
        .catalog
        .grant(&images[0].id, Scope::Always)
        .expect("granted");
    assert_eq!(granted.activated, [images[0].id, images[3].id]);
}

#[test]
fn hosts_removed_in_settings_need_consent_again() {
    let mut settings = online_settings();
    settings.catalog.allowed_image_hosts = vec!["i.imgur.com".to_owned()];
    let h = consent_harness(DescriptionImages::OnClick);
    h.catalog.apply_settings(&settings);
    assert_eq!(h.net.allowlist().image_hosts().count(), 1);
    let page = mod_page(&h);
    assert_eq!(page.description.images[0].access, Access::Click);

    h.catalog.apply_settings(&online_settings());
    assert_eq!(h.net.allowlist().image_hosts().count(), 0);
    let page = mod_page(&h);
    assert_eq!(page.description.images[0].access, Access::Consent);
    assert!(matches!(
        h.catalog.activate(&page.description.images[0].id),
        Err(ConsentError::NeedsConsent { .. })
    ));
}

#[test]
fn a_host_that_cannot_be_saved_is_not_allowed() {
    let h = consent_harness(DescriptionImages::OnClick);
    let page = mod_page(&h);
    h.saved.fail.store(true, Ordering::SeqCst);
    let err = h
        .catalog
        .grant(&page.description.images[0].id, Scope::Always)
        .expect_err("not saved");
    let ConsentError::Save { message } = &err else {
        panic!("{err:?}");
    };
    assert_eq!(message.id, MessageId::E1);
    assert_eq!(h.net.allowlist().image_hosts().count(), 0);
    assert_eq!(
        h.catalog.image(&page.description.images[0].id),
        Err(ImageError::NotActivated)
    );
}

#[test]
fn consent_applies_to_description_images_only() {
    let h = consent_harness(DescriptionImages::OnClick);
    let page = mod_page(&h);
    let gallery = page.gallery[0].preview;
    assert_eq!(
        h.catalog.grant(&gallery, Scope::Always),
        Err(ConsentError::NotADescriptionImage)
    );
    let unknown = ImageId::parse("0123456789abcdef0123456789abcdef").expect("id");
    assert_eq!(h.catalog.activate(&unknown), Err(ConsentError::UnknownId));
}

#[test]
fn a_redirect_from_a_consented_host_elsewhere_is_blocked() {
    let h = consent_harness(DescriptionImages::OnClick);
    h.transport.replace(
        IMGUR_A,
        Reply::status(302).header("Location", "https://imgur.com/removed.png"),
    );
    let page = mod_page(&h);
    let id = page.description.images[0].id;
    h.catalog.grant(&id, Scope::Once).expect("granted");
    assert_eq!(h.catalog.image(&id), Err(ImageError::Blocked));
}

#[test]
fn hidden_nsfw_mods_get_no_thumbnail_id() {
    // The recorded index with a picture for the NSFW mod 700002 too.
    let mut doc: serde_json::Value =
        serde_json::from_slice(fixture!("mod_index.json")).expect("fixture");
    let records = doc["_aRecords"].as_array_mut().expect("records");
    let mut media = records[0]["_aPreviewMedia"].clone();
    let image = &mut media["_aImages"][0];
    image["_sFile"] = "nsfw.jpg".into();
    image["_sFile220"] = "220-90_nsfw.jpg".into();
    image["_sFile530"] = "530-90_nsfw.jpg".into();
    records[1]["_aPreviewMedia"] = media;
    let index = serde_json::to_vec(&doc).expect("json");
    let list = |h: &Harness| {
        h.on(
            &list_url(Section::Mod, Sort::Newest, 1, None),
            Reply::ok(&index),
        );
        h.catalog
            .list(
                &Query::browse(Section::Mod, Sort::Newest, None, 1),
                Initiator::User,
            )
            .expect("page")
    };

    let shown = Harness::new(&with_nsfw(online_settings(), NsfwMode::Shown));
    let nsfw = list(&shown)
        .items
        .into_iter()
        .find(|c| c.id == 700_002)
        .expect("listed");
    let thumb = nsfw.thumbnail.expect("has a picture");
    assert!(nsfw.nsfw);

    let hidden = Harness::new(&online_settings());
    let page = list(&hidden);
    assert!(page.items.iter().all(|c| c.id != 700_002));
    // Ids are derived from the address, so the hidden catalog would resolve this one had it issued it.
    assert_eq!(
        hidden.catalog.image(&thumb.small),
        Err(ImageError::UnknownId)
    );
    assert_eq!(
        hidden.catalog.image(&thumb.large),
        Err(ImageError::UnknownId)
    );
    assert_eq!(hidden.transport.count(), 1);
}

#[test]
fn a_busy_gamebanana_holds_thumbnails_back_too() {
    let h = Harness::new(&online_settings());
    let thumb = listed(&h).items[0].thumbnail.expect("thumbnail");
    h.on(THUMB_220, Reply::status(429).header("Retry-After", "60"));
    assert_eq!(h.catalog.image(&thumb.small), Err(ImageError::RateLimited));
    assert!(matches!(
        h.catalog.status(),
        deadloader_catalog::Status::RateLimited { .. }
    ));
    let calls = h.transport.count();
    h.on(THUMB_530, Reply::image("image/jpeg", JPEG));
    assert_eq!(h.catalog.image(&thumb.large), Err(ImageError::RateLimited));
    assert_eq!(h.transport.count(), calls, "nothing asked while waiting");
    h.catalog.retry_now();
    assert!(h.catalog.image(&thumb.large).is_ok());
}

#[test]
fn a_busy_foreign_image_host_does_not_stop_the_catalog() {
    let h = consent_harness(DescriptionImages::OnClick);
    h.transport.replace(IMGUR_A, Reply::status(503));
    let page = mod_page(&h);
    let id = page.description.images[0].id;
    h.catalog.grant(&id, Scope::Once).expect("granted");
    assert_eq!(
        h.catalog.image(&id),
        Err(ImageError::Upstream { status: 503 })
    );
    assert_eq!(h.catalog.status(), deadloader_catalog::Status::Online);
}
