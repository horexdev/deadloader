//! Listing, search, filters and pages through the cache: fresh, stale and missing pages, the network states (local
//! mode, offline, rate limited, API changed, blocked) and the NSFW filter.

mod support;

use deadloader_catalog::{
    CatalogError, Event, Filter, Initiator, Origin, Query, SearchProblem, Status,
};
use deadloader_core::message::MessageId;
use deadloader_core::settings::NsfwMode;
use deadloader_gamebanana::{Section, Sort, request};
use deadloader_net::{HostSource, Purpose};
use std::time::Duration;
use support::*;

const TTL: Duration = Duration::from_secs(10 * 60);

fn newest(page: u32) -> Query {
    Query::browse(Section::Mod, Sort::Newest, None, page)
}

fn newest_url(page: u32) -> String {
    list_url(Section::Mod, Sort::Newest, page, None)
}

fn online() -> Harness {
    let h = Harness::new(&with_nsfw(online_settings(), NsfwMode::Shown));
    h.on(&newest_url(1), Reply::ok(fixture!("mod_index.json")));
    h
}

#[test]
fn local_mode_makes_zero_requests() {
    // The default settings: the first-run choice has not been made, so the catalog is off.
    let h = Harness::new(&deadloader_core::settings::Settings::default());
    h.on(&newest_url(1), Reply::ok(fixture!("mod_index.json")));

    assert_eq!(h.catalog.status(), Status::LocalMode);
    assert_eq!(
        h.catalog.list(&newest(1), Initiator::User),
        Err(CatalogError::LocalMode)
    );
    assert_eq!(
        h.catalog
            .list(&Query::search(Section::Mod, "wraith", 1), Initiator::User),
        Err(CatalogError::LocalMode)
    );
    assert_eq!(
        h.catalog.categories(Section::Mod, None),
        Err(CatalogError::LocalMode)
    );
    assert_eq!(
        h.catalog
            .details(Section::Mod, 700_001, None, Initiator::User)
            .map(|_| ()),
        Err(CatalogError::LocalMode)
    );
    assert_eq!(
        h.catalog.refresh(&newest(1), Initiator::User).map(|_| ()),
        Err(CatalogError::LocalMode)
    );
    assert!(!h.catalog.prefetch(&newest(2)));
    assert_eq!(h.jobs.run(), 0);

    assert_eq!(h.transport.count(), 0, "{:?}", h.transport.calls());
    assert!(h.net_log.all().is_empty(), "not even a refused request");
}

#[test]
fn a_missing_page_is_fetched_and_then_served_fresh_without_a_request() {
    let h = online();

    let first = h.catalog.list(&newest(1), Initiator::User).expect("page");
    assert_eq!(first.origin, Origin::Network);
    assert_eq!(first.total, 1234);
    assert!(first.has_more);
    assert_eq!(first.page, 1);
    assert_eq!(
        first.items.iter().map(|c| c.id).collect::<Vec<_>>(),
        [700_001, 700_002, 700_003]
    );
    assert_eq!(h.transport.calls(), [newest_url(1)]);
    let record = &h.net_log.all()[0];
    assert_eq!(record.purpose, Purpose::Catalog);
    assert_eq!(record.initiator, Initiator::User);
    assert_eq!(record.host_source, Some(HostSource::BuiltIn));
    assert_eq!(h.catalog.status(), Status::Online);

    h.clock.advance(TTL - Duration::from_secs(1));
    let second = h.catalog.list(&newest(1), Initiator::Auto).expect("page");
    assert_eq!(
        second.origin,
        Origin::Cache {
            fetched_at_unix_ms: h.clock.unix_ms()
                - (TTL - Duration::from_secs(1)).as_millis() as i64,
            stale: false
        }
    );
    assert_eq!(second.items, first.items);
    assert_eq!(h.transport.count(), 1, "fresh pages need no request");
    assert_eq!(h.jobs.len(), 0, "and no refresh");
}

#[test]
fn a_stale_page_is_served_at_once_and_refreshed_in_the_background() {
    let h = online();
    h.catalog.list(&newest(1), Initiator::User).expect("page");
    h.events.take();
    h.clock.advance(TTL);

    let stale = h.catalog.list(&newest(1), Initiator::Auto).expect("page");
    assert!(matches!(stale.origin, Origin::Cache { stale: true, .. }));
    assert_eq!(
        h.transport.count(),
        1,
        "the refresh is not done in the call"
    );

    // Asking again while the refresh is pending does not queue a second one.
    h.catalog.list(&newest(1), Initiator::Auto).expect("page");
    assert_eq!(h.jobs.run(), 1);
    assert_eq!(h.transport.count(), 2);
    assert_eq!(h.net_log.all()[1].initiator, Initiator::Auto);
    assert_eq!(h.events.take(), [Event::PageUpdated(newest(1))]);

    let fresh = h.catalog.list(&newest(1), Initiator::Auto).expect("page");
    assert!(matches!(fresh.origin, Origin::Cache { stale: false, .. }));
}

#[test]
fn offline_serves_the_saved_page_and_reports_since_when() {
    let h = online();
    h.catalog.list(&newest(1), Initiator::User).expect("page");
    h.events.take();
    h.clock.advance(TTL * 2);
    h.transport.set_offline(true);

    let saved = h
        .catalog
        .list(&newest(1), Initiator::Auto)
        .expect("saved page");
    assert!(matches!(saved.origin, Origin::Cache { stale: true, .. }));
    h.jobs.run();
    let since = h.clock.unix_ms();
    assert_eq!(
        h.catalog.status(),
        Status::Offline {
            since_unix_ms: since
        }
    );
    assert_eq!(
        h.events.take(),
        [Event::Status(Status::Offline {
            since_unix_ms: since
        })]
    );

    // A page never saved can't be shown; "since" stays the first failure.
    h.clock.advance(Duration::from_secs(30));
    assert_eq!(
        h.catalog.list(&newest(2), Initiator::User),
        Err(CatalogError::Offline {
            since_unix_ms: since
        })
    );
    assert!(h.events.take().is_empty(), "no new status event");

    // Back online: the next refresh succeeds and the status says so.
    h.transport.set_offline(false);
    h.catalog
        .list(&newest(1), Initiator::Auto)
        .expect("saved page");
    h.jobs.run();
    assert_eq!(h.catalog.status(), Status::Online);
    assert_eq!(
        h.events.take(),
        [Event::Status(Status::Online), Event::PageUpdated(newest(1))]
    );
}

#[test]
fn rate_limiting_backs_off_and_maps_to_e5() {
    let h = online();
    h.on(
        &newest_url(2),
        Reply::status(429).header("Retry-After", "120"),
    );

    let err = h
        .catalog
        .list(&newest(2), Initiator::Auto)
        .expect_err("busy");
    let CatalogError::RateLimited {
        retry_in_s,
        message,
    } = &err
    else {
        panic!("{err:?}");
    };
    assert_eq!(*retry_in_s, 120);
    assert_eq!(message.id, MessageId::E5);
    assert_eq!(message.args["retry_in_s"], "120");
    assert!(message.args.contains_key("log_id"), "{message:?}");
    let until = h.clock.unix_ms() + 120_000;
    assert!(matches!(
        h.catalog.status(),
        Status::RateLimited { until_unix_ms, .. } if until_unix_ms == until
    ));
    assert_eq!(h.transport.count(), 1);

    // While backing off, nothing is requested: saved pages are still served, missing ones are refused.
    h.transport
        .replace(&newest_url(2), Reply::ok(fixture!("mod_index.json")));
    assert!(matches!(
        h.catalog.list(&newest(2), Initiator::Auto),
        Err(CatalogError::RateLimited {
            retry_in_s: 120,
            ..
        })
    ));
    assert!(!h.catalog.prefetch(&newest(2)));
    assert_eq!(h.transport.count(), 1);

    // "Retry now" ends the wait.
    h.catalog.retry_now();
    let page = h.catalog.list(&newest(2), Initiator::User).expect("page");
    assert_eq!(page.origin, Origin::Network);
    assert_eq!(h.catalog.status(), Status::Online);
}

#[test]
fn the_back_off_ends_by_itself() {
    let h = online();
    h.on(&newest_url(2), Reply::status(503));
    h.on(&newest_url(2), Reply::ok(fixture!("mod_index.json")));
    let err = h
        .catalog
        .list(&newest(2), Initiator::Auto)
        .expect_err("busy");
    // Without Retry-After deadloader waits 30 s.
    assert!(
        matches!(err, CatalogError::RateLimited { retry_in_s: 30, .. }),
        "{err:?}"
    );
    h.clock.advance(Duration::from_secs(29));
    assert!(h.catalog.list(&newest(2), Initiator::Auto).is_err());
    assert_eq!(h.transport.count(), 1);
    h.clock.advance(Duration::from_secs(1));
    assert!(h.catalog.list(&newest(2), Initiator::Auto).is_ok());
    assert_eq!(h.transport.count(), 2);
}

#[test]
fn the_status_stops_saying_busy_when_the_wait_is_over() {
    let h = online();
    h.on(&newest_url(2), Reply::status(503));
    h.catalog
        .list(&newest(2), Initiator::Auto)
        .expect_err("busy");
    assert!(matches!(h.catalog.status(), Status::RateLimited { .. }));
    h.events.take();

    h.clock.advance(Duration::from_secs(30));
    assert_eq!(h.catalog.status(), Status::Online);
}

#[test]
fn retry_now_tells_the_ui_that_the_wait_is_over() {
    let h = online();
    h.on(&newest_url(2), Reply::status(503));
    h.catalog
        .list(&newest(2), Initiator::Auto)
        .expect_err("busy");
    h.events.take();

    h.catalog.retry_now();
    assert_eq!(h.catalog.status(), Status::Online);
    assert_eq!(h.events.take(), [Event::Status(Status::Online)]);
    // Nothing more to end.
    h.catalog.retry_now();
    assert!(h.events.take().is_empty());
}

#[test]
fn a_changed_api_maps_to_e4_and_clears_on_success() {
    let h = online();
    h.on(&newest_url(2), Reply::ok(b"<html>maintenance</html>"));
    let err = h
        .catalog
        .list(&newest(2), Initiator::Auto)
        .expect_err("not JSON");
    let CatalogError::ApiChanged { message } = &err else {
        panic!("{err:?}");
    };
    assert_eq!(message.id, MessageId::E4);
    assert!(message.args["detail"].contains("not_json"), "{message:?}");
    assert!(matches!(h.catalog.status(), Status::ApiChanged { .. }));
    assert!(
        h.lines.messages().iter().any(|m| m.contains("API")),
        "{:?}",
        h.lines.messages()
    );

    // A rejected request (GameBanana's own error document) is an API change too.
    h.on(
        &newest_url(3),
        Reply {
            status: 400,
            headers: Vec::new(),
            body: fixture!("error_input.json").to_vec(),
        },
    );
    assert!(matches!(
        h.catalog.list(&newest(3), Initiator::Auto),
        Err(CatalogError::ApiChanged { .. })
    ));

    h.catalog.list(&newest(1), Initiator::Auto).expect("page");
    assert_eq!(h.catalog.status(), Status::Online);
}

#[test]
fn a_redirect_off_the_allowlist_is_blocked_with_e16() {
    let h = online();
    h.on(
        &newest_url(2),
        Reply::status(302).header("Location", "https://evil.example/apiv11"),
    );
    let err = h
        .catalog
        .list(&newest(2), Initiator::Auto)
        .expect_err("blocked");
    let CatalogError::Blocked { message } = &err else {
        panic!("{err:?}");
    };
    assert_eq!(message.id, MessageId::E16);
    assert_eq!(message.args["host"], "evil.example");
    assert_eq!(h.transport.calls(), [newest_url(2)]);
}

#[test]
fn search_validates_the_text_and_uses_the_search_endpoint() {
    let h = online();
    assert_eq!(
        h.catalog
            .list(&Query::search(Section::Mod, " a ", 1), Initiator::User),
        Err(CatalogError::InvalidSearch {
            problem: SearchProblem::TooShort
        })
    );
    assert_eq!(
        h.catalog.list(
            &Query::search(Section::Mod, &"x".repeat(101), 1),
            Initiator::User
        ),
        Err(CatalogError::InvalidSearch {
            problem: SearchProblem::TooLong
        })
    );
    assert_eq!(h.transport.count(), 0);

    let query = deadloader_gamebanana::SearchQuery::new(Section::Mod, "wraith", 1).expect("valid");
    h.on(
        &request::search_url(&query),
        Reply::ok(fixture!("search_results.json")),
    );
    let page = h
        .catalog
        .list(&Query::search(Section::Mod, " wraith ", 1), Initiator::User)
        .expect("results");
    assert!(!page.items.is_empty());
    assert_eq!(h.transport.calls(), [request::search_url(&query)]);

    // The trimmed text is the cache key: the same search with other spacing is served from the cache.
    let again = h
        .catalog
        .list(&Query::search(Section::Mod, "wraith", 1), Initiator::User)
        .expect("results");
    assert!(matches!(again.origin, Origin::Cache { .. }));
}

#[test]
fn category_and_hero_filters_and_sections_are_separate_pages() {
    let h = online();
    let hero = Query::browse(Section::Mod, Sort::MostLiked, Some(33_301), 1);
    let hero_url = list_url(Section::Mod, Sort::MostLiked, 1, Some(33_301));
    let sounds = Query::browse(Section::Sound, Sort::Newest, None, 1);
    let sounds_url = list_url(Section::Sound, Sort::Newest, 1, None);
    h.on(&hero_url, Reply::ok(fixture!("mod_index.json")));
    h.on(&sounds_url, Reply::ok(fixture!("sound_index.json")));

    h.catalog.list(&newest(1), Initiator::User).expect("page");
    h.catalog.list(&hero, Initiator::User).expect("page");
    let sound_page = h.catalog.list(&sounds, Initiator::User).expect("page");
    assert!(sound_page.items.iter().all(|c| c.section == Section::Sound));
    assert_eq!(h.transport.calls(), [newest_url(1), hero_url, sounds_url]);
    assert!(matches!(
        hero.filter,
        Filter::Browse {
            category: Some(33_301),
            ..
        }
    ));
}

#[test]
fn categories_and_heroes_are_cached_like_pages() {
    let h = online();
    let roots = request::categories_url(Section::Mod, None);
    h.on(&roots, Reply::ok(fixture!("categories_root.json")));
    let first = h
        .catalog
        .categories(Section::Mod, None)
        .expect("categories");
    assert_eq!(first.origin, Origin::Network);
    let skins = first
        .items
        .iter()
        .find(|c| c.name == "Skins")
        .expect("skins");
    assert_eq!((skins.id, skins.subcategory_count), (33_295, 44));

    let again = h
        .catalog
        .categories(Section::Mod, None)
        .expect("categories");
    assert!(matches!(again.origin, Origin::Cache { stale: false, .. }));
    assert_eq!(h.transport.count(), 1);

    let heroes = request::categories_url(Section::Mod, Some(33_295));
    h.on(&heroes, Reply::ok(b"[]"));
    assert!(
        h.catalog
            .categories(Section::Mod, Some(33_295))
            .expect("heroes")
            .items
            .is_empty()
    );
    assert_eq!(h.transport.calls(), [roots, heroes]);
}

#[test]
fn prefetch_follows_the_setting() {
    let h = online();
    h.on(&newest_url(2), Reply::ok(fixture!("mod_index.json")));
    assert!(h.catalog.prefetch(&newest(2)));
    assert!(!h.catalog.prefetch(&newest(2)), "already queued");
    assert_eq!(
        h.transport.count(),
        0,
        "prefetching happens in the background"
    );
    h.jobs.run();
    assert_eq!(h.transport.calls(), [newest_url(2)]);
    assert_eq!(h.net_log.all()[0].initiator, Initiator::Auto);
    assert!(!h.catalog.prefetch(&newest(2)), "the page is fresh now");
    let page = h.catalog.list(&newest(2), Initiator::Auto).expect("page");
    assert!(matches!(page.origin, Origin::Cache { stale: false, .. }));

    let mut settings = with_nsfw(online_settings(), NsfwMode::Shown);
    settings.catalog.prefetch_next_page = false;
    h.catalog.apply_settings(&settings);
    assert!(!h.catalog.prefetch(&newest(3)));
    assert_eq!(h.jobs.len(), 0);
}

#[test]
fn refresh_goes_to_the_network_even_for_a_fresh_page() {
    let h = online();
    h.catalog.list(&newest(1), Initiator::User).expect("page");
    let page = h
        .catalog
        .refresh(&newest(1), Initiator::User)
        .expect("page");
    assert_eq!(page.origin, Origin::Network);
    assert_eq!(h.transport.count(), 2);
}

#[test]
fn hidden_nsfw_mods_are_left_out_and_their_thumbnails_never_registered() {
    let h = Harness::new(&online_settings());
    h.on(&newest_url(1), Reply::ok(fixture!("mod_index.json")));
    let page = h.catalog.list(&newest(1), Initiator::User).expect("page");
    assert_eq!(
        page.items.iter().map(|c| c.id).collect::<Vec<_>>(),
        [700_001]
    );
    assert_eq!(page.hidden_nsfw, 2);
    assert!(!page.items[0].nsfw);

    let h = Harness::new(&with_nsfw(online_settings(), NsfwMode::Blurred));
    h.on(&newest_url(1), Reply::ok(fixture!("mod_index.json")));
    let page = h.catalog.list(&newest(1), Initiator::User).expect("page");
    assert_eq!(
        page.items
            .iter()
            .map(|c| (c.id, c.nsfw))
            .collect::<Vec<_>>(),
        [(700_001, false), (700_002, true), (700_003, true)]
    );
    assert_eq!(page.hidden_nsfw, 0);
}

#[test]
fn switching_to_local_mode_stops_requests_and_says_so() {
    let h = online();
    h.catalog.list(&newest(1), Initiator::User).expect("page");
    h.events.take();

    h.catalog
        .apply_settings(&deadloader_core::settings::Settings::default());
    assert_eq!(h.catalog.status(), Status::LocalMode);
    assert_eq!(h.events.take(), [Event::Status(Status::LocalMode)]);
    assert_eq!(
        h.catalog.list(&newest(1), Initiator::User),
        Err(CatalogError::LocalMode)
    );
    assert_eq!(h.transport.count(), 1);

    h.catalog.apply_settings(&online_settings());
    assert_eq!(h.catalog.status(), Status::Online);
    assert_eq!(h.events.take(), [Event::Status(Status::Online)]);
}

#[test]
fn the_page_ttl_comes_from_the_settings() {
    let h = online();
    let mut settings = with_nsfw(online_settings(), NsfwMode::Shown);
    settings.catalog.page_ttl_minutes = 1;
    h.catalog.apply_settings(&settings);
    h.catalog.list(&newest(1), Initiator::User).expect("page");
    h.clock.advance(Duration::from_secs(60));
    let page = h.catalog.list(&newest(1), Initiator::User).expect("page");
    assert!(matches!(page.origin, Origin::Cache { stale: true, .. }));
}
