use super::*;
use crate::log::{
    AppEvent, FileEvent, FileOp, HostSource, Initiator, Level, Masker, NetworkEvent, Outcome,
    Purpose,
};
use crate::testing::MemLogDir;
use serde_json::{Map, json};
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

const TODAY: &str = "2026-10-15";
const YESTERDAY: &str = "2026-10-14";

fn day(y: i32, m: u8, d: u8) -> Date {
    Date::from_calendar_date(y, Month::try_from(m).expect("month"), d).expect("date")
}

/// 2026-10-15 at `h:mi` local time, UTC+3.
fn at(h: u8, mi: u8) -> OffsetDateTime {
    OffsetDateTime::new_in_offset(
        day(2026, 10, 15),
        Time::from_hms(h, mi, 0).expect("time"),
        UtcOffset::from_hms(3, 0, 0).expect("offset"),
    )
}

fn ts(date: &str, hm: &str) -> String {
    format!("{date}T{hm}:00.000+03:00")
}

fn alice() -> Masker {
    Masker::new(r"C:\Users\Alice")
}

fn nobody() -> Masker {
    Masker::none()
}

fn reader<'a>(dir: &'a MemLogDir, masker: &'a Masker) -> Reader<'a, MemLogDir> {
    Reader {
        dir,
        now: at(15, 0),
        masker,
    }
}

fn line<T: serde::Serialize>(
    date: &str,
    hm: &str,
    id: &str,
    lvl: Level,
    msg: &str,
    body: T,
) -> String {
    let record = Record {
        ts: ts(date, hm),
        id: LogId(id.to_owned()),
        lvl,
        msg: msg.to_owned(),
        body,
    };
    format!("{}\n", serde_json::to_string(&record).expect("json"))
}

fn app(dir: &MemLogDir, date: &str, hm: &str, id: &str, lvl: Level, msg: &str) {
    dir.push(
        &format!("app-{date}.jsonl"),
        &line(date, hm, id, lvl, msg, AppEvent { fields: Map::new() }),
    );
}

fn hop(req: &str, hop: u32, url: &str) -> NetworkEvent {
    NetworkEvent {
        req: LogId(req.to_owned()),
        hop,
        attempt: 1,
        method: "GET".into(),
        url: url.into(),
        host_source: Some(HostSource::Builtin),
        initiator: Initiator::Auto,
        purpose: Purpose::Thumbnail,
        status: Some(200),
        bytes: Some(1000),
        ms: 90,
        range: None,
        error: None,
        blocked: None,
    }
}

fn net(dir: &MemLogDir, date: &str, hm: &str, id: &str, msg: &str, event: NetworkEvent) {
    let lvl = if event.error.is_some() {
        Level::Error
    } else if event.blocked.is_some() {
        Level::Warn
    } else {
        Level::Info
    };
    dir.push(
        &format!("network-{date}.jsonl"),
        &line(date, hm, id, lvl, msg, event),
    );
}

fn file(dir: &MemLogDir, date: &str, hm: &str, id: &str, event: FileEvent) {
    let lvl = if event.result == Outcome::Ok {
        Level::Info
    } else {
        Level::Error
    };
    dir.push(
        &format!("files-{date}.jsonl"),
        &line(date, hm, id, lvl, "file operation", event),
    );
}

fn query(kind: Kind) -> Query {
    Query {
        kind,
        range: Range::Today,
        purpose: None,
        op: None,
        level: None,
        result: Results::All,
        host: None,
        text: String::new(),
        cursor: None,
        limit: 100,
    }
}

/// The ids of a page's entries: line ids, or request ids for the network log.
fn ids(page: &Page) -> Vec<String> {
    page.entries
        .iter()
        .map(|e| match e {
            Entry::Network { req, .. } => req.as_str().to_owned(),
            Entry::Files { record } => record.id.as_str().to_owned(),
            Entry::App { record } => record.id.as_str().to_owned(),
        })
        .collect()
}

fn hops(entry: &Entry) -> Vec<(u32, u32)> {
    match entry {
        Entry::Network { lines, .. } => {
            lines.iter().map(|l| (l.body.hop, l.body.attempt)).collect()
        }
        _ => panic!("not a request: {entry:?}"),
    }
}

/// Everything, page by page, following the cursors.
fn all_pages(r: &Reader<'_, MemLogDir>, mut q: Query) -> Vec<String> {
    let mut out = Vec::new();
    for _ in 0..100 {
        let page = r.page(&q).expect("page");
        out.extend(ids(&page));
        match page.cursor {
            Some(cursor) => q.cursor = Some(cursor),
            None => return out,
        }
    }
    panic!("the pages never ended: {out:?}");
}

// --- pages ---------------------------------------------------------------------------------------------------------

#[test]
fn app_events_come_newest_first_a_page_at_a_time() {
    let dir = MemLogDir::default();
    for (n, hm) in ["09:00", "09:01", "09:02", "09:03", "09:04"]
        .iter()
        .enumerate()
    {
        app(&dir, TODAY, hm, &format!("a-{n}"), Level::Info, "started");
    }
    let masker = nobody();
    let r = reader(&dir, &masker);
    let mut q = query(Kind::App);
    q.limit = 2;
    let first = r.page(&q).expect("page");
    assert_eq!(ids(&first), ["a-4", "a-3"]);
    assert!(first.cursor.is_some());
    assert_eq!(all_pages(&r, q), ["a-4", "a-3", "a-2", "a-1", "a-0"]);
}

#[test]
fn a_page_reads_only_the_end_of_a_big_log() {
    let dir = MemLogDir::default();
    for n in 0..20_000 {
        app(
            &dir,
            TODAY,
            "09:00",
            &format!("a-{n:05}"),
            Level::Info,
            "a line of an ordinary size",
        );
    }
    let total = dir
        .file_len(&Kind::App.file_name(day(2026, 10, 15)))
        .expect("len");
    let masker = nobody();
    let mut q = query(Kind::App);
    q.limit = 10;
    let page = reader(&dir, &masker).page(&q).expect("page");
    assert_eq!(ids(&page).first().map(String::as_str), Some("a-19999"));
    assert!(
        dir.read_so_far() < total / 10,
        "read {} of {total}",
        dir.read_so_far()
    );
}

#[test]
fn the_range_picks_the_days() {
    let dir = MemLogDir::default();
    app(&dir, TODAY, "09:00", "today", Level::Info, "x");
    app(&dir, YESTERDAY, "09:00", "yesterday", Level::Info, "x");
    app(
        &dir,
        "2026-10-09",
        "09:00",
        "six-days-ago",
        Level::Info,
        "x",
    );
    app(&dir, "2026-10-08", "09:00", "a-week-ago", Level::Info, "x");
    app(&dir, "2026-09-16", "09:00", "29-days-ago", Level::Info, "x");
    app(&dir, "2026-09-15", "09:00", "30-days-ago", Level::Info, "x");
    let masker = nobody();
    let r = reader(&dir, &masker);
    let mut q = query(Kind::App);
    assert_eq!(all_pages(&r, q.clone()), ["today"]);
    q.range = Range::Week;
    assert_eq!(
        all_pages(&r, q.clone()),
        ["today", "yesterday", "six-days-ago"]
    );
    q.range = Range::Month;
    assert_eq!(
        all_pages(&r, q),
        [
            "today",
            "yesterday",
            "six-days-ago",
            "a-week-ago",
            "29-days-ago"
        ]
    );
}

#[test]
fn the_last_hour_reaches_into_yesterday_after_midnight() {
    let dir = MemLogDir::default();
    app(&dir, TODAY, "00:20", "today", Level::Info, "x");
    app(&dir, YESTERDAY, "23:00", "too-old", Level::Info, "x");
    app(&dir, YESTERDAY, "23:50", "late", Level::Info, "x");
    let masker = nobody();
    let r = Reader {
        dir: &dir,
        now: at(0, 30),
        masker: &masker,
    };
    let mut q = query(Kind::App);
    q.range = Range::Hour;
    assert_eq!(all_pages(&r, q), ["today", "late"]);

    let r = reader(&dir, &masker);
    app(&dir, TODAY, "14:30", "recent", Level::Info, "x");
    let mut q = query(Kind::App);
    q.range = Range::Hour;
    assert_eq!(all_pages(&r, q), ["recent"]);
}

#[test]
fn damaged_and_blank_lines_are_skipped() {
    let dir = MemLogDir::default();
    app(&dir, TODAY, "09:00", "a", Level::Info, "x");
    dir.push("app-2026-10-15.jsonl", "garbage\n\n\r\n");
    app(&dir, TODAY, "09:01", "b", Level::Info, "x");
    dir.push("app-2026-10-15.jsonl", "{\"ts\":\"2026-");
    let masker = nobody();
    assert_eq!(
        all_pages(&reader(&dir, &masker), query(Kind::App)),
        ["b", "a"]
    );
}

#[test]
fn lines_longer_than_a_read_chunk_are_read_whole() {
    let dir = MemLogDir::default();
    let long = "x".repeat(200_000);
    app(&dir, TODAY, "09:00", "a", Level::Info, "short");
    app(&dir, TODAY, "09:01", "b", Level::Info, &long);
    app(&dir, TODAY, "09:02", "c", Level::Info, "short");
    let masker = nobody();
    let page = reader(&dir, &masker).page(&query(Kind::App)).expect("page");
    assert_eq!(ids(&page), ["c", "b", "a"]);
    let Some(Entry::App { record }) = page.entries.get(1) else {
        panic!("{page:?}");
    };
    assert_eq!(record.msg.len(), 200_000);
}

#[test]
fn the_page_size_is_bounded() {
    let dir = MemLogDir::default();
    for n in 0..700 {
        app(&dir, TODAY, "09:00", &format!("a-{n:03}"), Level::Info, "x");
    }
    let masker = nobody();
    let mut q = query(Kind::App);
    q.limit = 10_000;
    assert_eq!(
        reader(&dir, &masker).page(&q).expect("page").entries.len(),
        MAX_LIMIT
    );
    q.limit = 0;
    assert_eq!(
        reader(&dir, &masker).page(&q).expect("page").entries.len(),
        1
    );
}

#[test]
fn a_cursor_can_only_name_a_log_of_the_asked_kind_and_range() {
    let dir = MemLogDir::default();
    app(&dir, TODAY, "09:00", "a", Level::Info, "x");
    let masker = nobody();
    let r = reader(&dir, &masker);
    for file in [
        r"..\settings.json",
        "network-2026-10-15.jsonl",
        "app-2026-01-01.jsonl",
    ] {
        let mut q = query(Kind::App);
        q.cursor = Some(Cursor {
            file: file.to_owned(),
            offset: 10,
            skip: Vec::new(),
        });
        let err = r.page(&q).expect_err("refused");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput, "{file}");
    }
}

#[test]
fn no_logs_yet_is_an_empty_page() {
    let dir = MemLogDir::default();
    let masker = nobody();
    let page = reader(&dir, &masker)
        .page(&query(Kind::Network))
        .expect("page");
    assert!(page.entries.is_empty());
    assert!(page.cursor.is_none());
}

// --- requests and their hops ------------------------------------------------------------------------------------

#[test]
fn the_hops_of_a_request_make_one_entry_oldest_first() {
    let dir = MemLogDir::default();
    net(
        &dir,
        TODAY,
        "14:29",
        "n-2",
        "redirect",
        NetworkEvent {
            status: Some(302),
            ..hop("r-1", 0, "https://gamebanana.com/dl/1")
        },
    );
    net(
        &dir,
        TODAY,
        "14:29",
        "n-3",
        "thumb",
        hop("r-9", 0, "https://images.gamebanana.com/a.jpg"),
    );
    net(
        &dir,
        TODAY,
        "14:29",
        "n-4",
        "redirect",
        NetworkEvent {
            status: Some(302),
            ..hop("r-1", 1, "https://files.gamebanana.com/mods/x.zip")
        },
    );
    net(
        &dir,
        TODAY,
        "14:29",
        "n-5",
        "response",
        hop("r-1", 2, "https://filecache07.gamebanana.com/mods/x.zip"),
    );
    let masker = nobody();
    let page = reader(&dir, &masker)
        .page(&query(Kind::Network))
        .expect("page");
    assert_eq!(ids(&page), ["r-1", "r-9"]);
    assert_eq!(hops(&page.entries[0]), [(0, 1), (1, 1), (2, 1)]);
    assert_eq!(hops(&page.entries[1]), [(0, 1)]);
}

#[test]
fn retries_belong_to_their_request() {
    let dir = MemLogDir::default();
    net(
        &dir,
        TODAY,
        "14:24",
        "n-1",
        RETRYING_MSG,
        NetworkEvent {
            status: Some(429),
            ..hop("r-1", 0, "https://gamebanana.com/apiv11/Mod/Index")
        },
    );
    net(
        &dir,
        TODAY,
        "14:25",
        "n-2",
        "response",
        NetworkEvent {
            attempt: 2,
            ..hop("r-1", 0, "https://gamebanana.com/apiv11/Mod/Index")
        },
    );
    let masker = nobody();
    let page = reader(&dir, &masker)
        .page(&query(Kind::Network))
        .expect("page");
    assert_eq!(ids(&page), ["r-1"]);
    assert_eq!(hops(&page.entries[0]), [(0, 1), (0, 2)]);
}

#[test]
fn a_request_cut_by_a_page_boundary_is_not_shown_twice() {
    let dir = MemLogDir::default();
    // r-1's hops are spread around other requests.
    net(
        &dir,
        TODAY,
        "14:00",
        "n-01",
        "redirect",
        NetworkEvent {
            status: Some(302),
            ..hop("r-1", 0, "https://gamebanana.com/dl/1")
        },
    );
    for n in 2..6 {
        net(
            &dir,
            TODAY,
            "14:00",
            &format!("n-0{n}"),
            "response",
            hop(&format!("r-{n}"), 0, "https://images.gamebanana.com/a.jpg"),
        );
    }
    net(
        &dir,
        TODAY,
        "14:00",
        "n-06",
        "response",
        hop("r-1", 1, "https://files.gamebanana.com/x.zip"),
    );
    for n in 7..9 {
        net(
            &dir,
            TODAY,
            "14:00",
            &format!("n-0{n}"),
            "response",
            hop(&format!("r-{n}"), 0, "https://images.gamebanana.com/a.jpg"),
        );
    }
    let masker = nobody();
    let r = reader(&dir, &masker);
    let mut q = query(Kind::Network);
    for limit in 1..6 {
        q.limit = limit;
        assert_eq!(
            all_pages(&r, q.clone()),
            ["r-8", "r-7", "r-1", "r-5", "r-4", "r-3", "r-2"],
            "pages of {limit}"
        );
    }
    q.limit = 3;
    let first = r.page(&q).expect("page");
    assert_eq!(hops(&first.entries[2]), [(0, 1), (1, 1)]);
}

#[test]
fn a_request_whose_start_is_gone_is_still_shown() {
    let dir = MemLogDir::default();
    // Only the last hop survived (e.g. the start was in a file removed by rotation).
    net(
        &dir,
        TODAY,
        "14:00",
        "n-1",
        "response",
        hop("r-1", 2, "https://files.gamebanana.com/x.zip"),
    );
    let masker = nobody();
    assert_eq!(
        all_pages(&reader(&dir, &masker), query(Kind::Network)),
        ["r-1"]
    );
}

// --- filters -----------------------------------------------------------------------------------------------------

/// A morning of requests: a thumbnail, a blocked download redirect, a failed image, a 404 and a retried page.
fn morning(dir: &MemLogDir) {
    net(
        dir,
        TODAY,
        "09:00",
        "n-01",
        "response",
        hop("thumb", 0, "https://images.gamebanana.com/img/a.jpg"),
    );
    let download = |h: u32, url: &str| NetworkEvent {
        purpose: Purpose::Download,
        initiator: Initiator::User,
        status: Some(302),
        ..hop("blocked", h, url)
    };
    net(
        dir,
        TODAY,
        "09:01",
        "n-02",
        "redirect",
        download(0, "https://gamebanana.com/dl/7"),
    );
    net(
        dir,
        TODAY,
        "09:01",
        "n-03",
        "request blocked",
        NetworkEvent {
            host_source: None,
            status: None,
            bytes: None,
            blocked: Some("host-not-listed".into()),
            ..download(1, "https://dl.fastmirror.example/f/x.zip")
        },
    );
    net(
        dir,
        TODAY,
        "09:02",
        "n-04",
        "request failed",
        NetworkEvent {
            purpose: Purpose::DescriptionImage,
            host_source: Some(HostSource::UserImage),
            status: None,
            error: Some("timed out".into()),
            ..hop("timeout", 0, "https://i.imgur.com/Qm3xT7a.png")
        },
    );
    net(
        dir,
        TODAY,
        "09:03",
        "n-05",
        "response",
        NetworkEvent {
            purpose: Purpose::Catalog,
            status: Some(404),
            ..hop(
                "missing",
                0,
                "https://gamebanana.com/apiv11/Mod/1/ProfilePage",
            )
        },
    );
    net(
        dir,
        TODAY,
        "09:04",
        "n-06",
        RETRYING_MSG,
        NetworkEvent {
            purpose: Purpose::Catalog,
            status: Some(429),
            ..hop("retried", 0, "https://gamebanana.com/apiv11/Mod/Index")
        },
    );
    net(
        dir,
        TODAY,
        "09:04",
        "n-07",
        "response",
        NetworkEvent {
            purpose: Purpose::Catalog,
            attempt: 2,
            ..hop("retried", 0, "https://gamebanana.com/apiv11/Mod/Index")
        },
    );
}

#[test]
fn requests_filter_by_purpose_host_and_result() {
    let dir = MemLogDir::default();
    morning(&dir);
    let masker = nobody();
    let r = reader(&dir, &masker);
    let all = query(Kind::Network);
    assert_eq!(
        all_pages(&r, all.clone()),
        ["retried", "missing", "timeout", "blocked", "thumb"]
    );
    let mut q = all.clone();
    q.purpose = Some(Purpose::Catalog);
    assert_eq!(all_pages(&r, q), ["retried", "missing"]);
    let mut q = all.clone();
    // Any hop's host counts, in any case.
    q.host = Some("DL.fastmirror.example".into());
    assert_eq!(all_pages(&r, q), ["blocked"]);
    let mut q = all.clone();
    q.result = Results::Error;
    assert_eq!(all_pages(&r, q), ["missing", "timeout", "blocked"]);
    let mut q = all;
    q.result = Results::Ok;
    assert_eq!(all_pages(&r, q), ["retried", "thumb"]);
}

#[test]
fn file_operations_filter_by_operation_and_result() {
    let dir = MemLogDir::default();
    file(
        &dir,
        TODAY,
        "09:00",
        "f-1",
        FileEvent {
            dst: Some(r"D:\Deadlock\game\citadel\addons\pak01_dir.vpk".into()),
            ..FileEvent::new(FileOp::Hardlink)
        },
    );
    file(
        &dir,
        TODAY,
        "09:01",
        "f-2",
        FileEvent {
            src: Some(r"D:\Deadlock\game\citadel\addons\pak07_dir.vpk".into()),
            result: Outcome::Failed,
            error: Some("Access is denied. (os error 5)".into()),
            ..FileEvent::new(FileOp::Delete)
        },
    );
    file(
        &dir,
        TODAY,
        "09:02",
        "f-3",
        FileEvent {
            src: Some(r"D:\Deadlock\game\citadel\addons\pak07_dir.vpk".into()),
            ..FileEvent::new(FileOp::Delete)
        },
    );
    let masker = nobody();
    let r = reader(&dir, &masker);
    let mut q = query(Kind::Files);
    q.op = Some(FileOp::Delete);
    assert_eq!(all_pages(&r, q.clone()), ["f-3", "f-2"]);
    q.result = Results::Error;
    assert_eq!(all_pages(&r, q.clone()), ["f-2"]);
    q.op = None;
    q.result = Results::Ok;
    assert_eq!(all_pages(&r, q), ["f-3", "f-1"]);
}

#[test]
fn app_events_filter_by_level() {
    let dir = MemLogDir::default();
    app(&dir, TODAY, "09:00", "a-1", Level::Info, "started");
    app(
        &dir,
        TODAY,
        "09:01",
        "a-2",
        Level::Warn,
        "settings corrected",
    );
    app(&dir, TODAY, "09:02", "a-3", Level::Error, "catalog failed");
    let masker = nobody();
    let mut q = query(Kind::App);
    q.level = Some(Level::Warn);
    assert_eq!(all_pages(&reader(&dir, &masker), q), ["a-2"]);
}

#[test]
fn search_finds_text_in_any_field_ignoring_case() {
    let dir = MemLogDir::default();
    morning(&dir);
    let masker = nobody();
    let r = reader(&dir, &masker);
    let mut q = query(Kind::Network);
    q.text = "  FASTMIRROR ".into();
    assert_eq!(all_pages(&r, q.clone()), ["blocked"]);
    q.text = "timed out".into();
    assert_eq!(all_pages(&r, q.clone()), ["timeout"]);
    q.text = "404".into();
    assert_eq!(all_pages(&r, q.clone()), ["missing"]);
    // Ids are searchable too: an error message names its entry.
    q.text = "n-03".into();
    assert_eq!(all_pages(&r, q), ["blocked"]);

    file(
        &dir,
        TODAY,
        "09:00",
        "f-1",
        FileEvent {
            src: Some(r"D:\Games\deadloader\deadloader-data\library\haze\1\0.vpk".into()),
            dst: Some(r"D:\Deadlock\game\citadel\addons\pak01_dir.vpk".into()),
            ..FileEvent::new(FileOp::Hardlink)
        },
    );
    let mut q = query(Kind::Files);
    q.text = r"citadel\addons\PAK01".into();
    assert_eq!(all_pages(&r, q), ["f-1"]);
}

// --- the user name ------------------------------------------------------------------------------------------------

#[test]
fn the_user_name_is_hidden_in_what_is_read_while_masking_is_on() {
    let dir = MemLogDir::default();
    // Written while masking was switched off.
    app(
        &dir,
        TODAY,
        "09:00",
        "a-1",
        Level::Info,
        r"started from C:\Users\Alice\Desktop",
    );
    let masker = alice();
    let r = reader(&dir, &masker);
    let page = r.page(&query(Kind::App)).expect("page");
    let Some(Entry::App { record }) = page.entries.first() else {
        panic!("{page:?}");
    };
    assert_eq!(record.msg, r"started from %USERPROFILE%\Desktop");
    // Nor can it be found by searching for it.
    let mut q = query(Kind::App);
    q.text = "alice".into();
    assert!(r.page(&q).expect("page").entries.is_empty());

    let mut off = alice();
    off.set_enabled(false);
    let page = reader(&dir, &off).page(&query(Kind::App)).expect("page");
    let Some(Entry::App { record }) = page.entries.first() else {
        panic!("{page:?}");
    };
    assert_eq!(record.msg, r"started from C:\Users\Alice\Desktop");
}

// --- one entry ----------------------------------------------------------------------------------------------------

#[test]
fn one_entry_is_found_by_its_id() {
    let dir = MemLogDir::default();
    app(
        &dir,
        YESTERDAY,
        "09:00",
        "mv9fkgpf-000001",
        Level::Info,
        "older",
    );
    app(
        &dir,
        TODAY,
        "09:00",
        "mv9fkgpf-000002",
        Level::Error,
        "the one",
    );
    file(
        &dir,
        TODAY,
        "09:01",
        "mv9fkgpf-000003",
        FileEvent::new(FileOp::Write),
    );
    let masker = nobody();
    let r = reader(&dir, &masker);
    let Some(Entry::App { record }) = r.entry("mv9fkgpf-000002").expect("read") else {
        panic!("not found");
    };
    assert_eq!(record.msg, "the one");
    assert!(matches!(
        r.entry("mv9fkgpf-000003").expect("read"),
        Some(Entry::Files { .. })
    ));
    assert!(matches!(
        r.entry("mv9fkgpf-000001").expect("read"),
        Some(Entry::App { .. })
    ));
    assert_eq!(r.entry("mv9fkgpf-000999").expect("read"), None);
    // Only ids are looked up, nothing that could match other text.
    assert_eq!(r.entry("the one").expect("read"), None);
}

#[test]
fn a_network_line_or_its_request_id_finds_the_whole_request() {
    let dir = MemLogDir::default();
    morning(&dir);
    let masker = nobody();
    let r = reader(&dir, &masker);
    for id in ["blocked", "n-02", "n-03"] {
        let entry = r.entry(id).expect("read").expect("found");
        assert_eq!(hops(&entry), [(0, 1), (1, 1)], "{id}");
    }
}

// --- counts and summary -------------------------------------------------------------------------------------------

#[test]
fn counts_tell_each_tab_its_totals_and_errors() {
    let dir = MemLogDir::default();
    morning(&dir);
    net(
        &dir,
        YESTERDAY,
        "09:00",
        "y-1",
        "response",
        hop("old", 0, "https://gamebanana.com/apiv11/Mod/Index"),
    );
    file(
        &dir,
        TODAY,
        "09:00",
        "f-1",
        FileEvent::new(FileOp::Hardlink),
    );
    file(
        &dir,
        TODAY,
        "09:01",
        "f-2",
        FileEvent {
            result: Outcome::Failed,
            ..FileEvent::new(FileOp::Delete)
        },
    );
    app(&dir, TODAY, "09:00", "a-1", Level::Info, "x");
    app(&dir, TODAY, "09:01", "a-2", Level::Error, "x");
    let mut tallies = Tallies::default();
    let counts = tallies
        .counts(&dir, at(15, 0), Range::Today)
        .expect("counts");
    assert_eq!(counts.network.requests, 5);
    assert_eq!(counts.network.failed, 3);
    assert_eq!(counts.network.purposes.get(&Purpose::Catalog), Some(&2));
    assert_eq!(counts.network.purposes.get(&Purpose::Download), Some(&1));
    assert_eq!(counts.network.hosts.get("gamebanana.com"), Some(&4));
    assert_eq!(counts.network.hosts.get("dl.fastmirror.example"), Some(&1));
    assert_eq!(counts.files.operations, 2);
    assert_eq!(counts.files.failed, 1);
    assert_eq!(counts.files.ops.get(&FileOp::Delete), Some(&1));
    assert_eq!(counts.app.events, 2);
    assert_eq!(counts.app.errors, 1);
    assert_eq!(counts.app.levels.get(&Level::Error), Some(&1));

    let week = tallies
        .counts(&dir, at(15, 0), Range::Week)
        .expect("counts");
    assert_eq!(week.network.requests, 6);
}

#[test]
fn counts_read_only_what_was_added_since_the_last_time() {
    let dir = MemLogDir::default();
    for n in 0..1000 {
        app(&dir, TODAY, "09:00", &format!("a-{n}"), Level::Info, "x");
    }
    let mut tallies = Tallies::default();
    assert_eq!(
        tallies
            .counts(&dir, at(15, 0), Range::Today)
            .expect("counts")
            .app
            .events,
        1000
    );
    let before = dir.read_so_far();
    app(&dir, TODAY, "09:00", "a-new", Level::Warn, "x");
    let counts = tallies
        .counts(&dir, at(15, 0), Range::Today)
        .expect("counts");
    assert_eq!(counts.app.events, 1001);
    assert_eq!(counts.app.levels.get(&Level::Warn), Some(&1));
    assert!(
        dir.read_so_far() - before < 1000,
        "read {}",
        dir.read_so_far() - before
    );
}

#[test]
fn counts_of_the_last_hour_go_by_time() {
    let dir = MemLogDir::default();
    app(&dir, TODAY, "13:00", "a-1", Level::Info, "x");
    app(&dir, TODAY, "14:30", "a-2", Level::Info, "x");
    let mut tallies = Tallies::default();
    assert_eq!(
        tallies
            .counts(&dir, at(15, 0), Range::Hour)
            .expect("counts")
            .app
            .events,
        1
    );
}

#[test]
fn the_summary_names_the_hosts_contacted_today_and_what_was_blocked() {
    let dir = MemLogDir::default();
    morning(&dir);
    net(
        &dir,
        YESTERDAY,
        "09:00",
        "y-1",
        "response",
        hop("old", 0, "https://example.org/x"),
    );
    let mut tallies = Tallies::default();
    let summary = tallies
        .summary(&dir, at(15, 0), &nobody())
        .expect("summary");
    let hosts: Vec<(&str, u64, Option<HostSource>)> = summary
        .contacted
        .iter()
        .map(|h| (h.host.as_str(), h.requests, h.source))
        .collect();
    // Built-in hosts first, the most used first among them.
    assert_eq!(
        hosts,
        [
            ("gamebanana.com", 4, Some(HostSource::Builtin)),
            ("images.gamebanana.com", 1, Some(HostSource::Builtin)),
            ("i.imgur.com", 1, Some(HostSource::UserImage)),
        ]
    );
    assert_eq!(summary.blocked.len(), 1);
    assert_eq!(summary.blocked[0].host, "dl.fastmirror.example");
    assert_eq!(summary.blocked[0].requests, 1);
    assert_eq!(summary.blocked[0].last.as_str(), "blocked");
    assert!(!summary.empty);
}

#[test]
fn the_summary_counts_todays_changes_in_the_game_folder() {
    let dir = MemLogDir::default();
    let addons = r"D:\SteamLibrary\steamapps\common\Deadlock\game\citadel\addons";
    let lib = r"D:\Games\deadloader\deadloader-data\library\haze\1\0.vpk";
    let in_change = |txn: u64, event: FileEvent| FileEvent {
        txn: Some(txn),
        ..event
    };
    file(
        &dir,
        TODAY,
        "09:00",
        "f-1",
        in_change(
            1,
            FileEvent {
                src: Some(
                    r"D:\SteamLibrary\steamapps\common\Deadlock\game\citadel\gameinfo.gi".into(),
                ),
                dst: Some(r"D:\Games\deadloader\deadloader-data\backups\gameinfo\1.gi".into()),
                ..FileEvent::new(FileOp::Backup)
            },
        ),
    );
    file(
        &dir,
        TODAY,
        "09:00",
        "f-2",
        in_change(
            1,
            FileEvent {
                src: Some(
                    r"D:\SteamLibrary\steamapps\common\Deadlock\game\citadel\gameinfo.gi.tmp"
                        .into(),
                ),
                dst: Some(
                    r"D:\SteamLibrary\steamapps\common\Deadlock\game\citadel\gameinfo.gi".into(),
                ),
                ..FileEvent::new(FileOp::Rename)
            },
        ),
    );
    file(
        &dir,
        TODAY,
        "09:00",
        "f-3",
        in_change(
            1,
            FileEvent {
                src: Some(lib.into()),
                dst: Some(format!(r"{addons}\pak01_dir.vpk")),
                ..FileEvent::new(FileOp::Hardlink)
            },
        ),
    );
    file(
        &dir,
        TODAY,
        "12:00",
        "f-4",
        in_change(
            2,
            FileEvent {
                src: Some(format!(r"{addons}\pak02_dir.vpk")),
                ..FileEvent::new(FileOp::Delete)
            },
        ),
    );
    file(
        &dir,
        TODAY,
        "12:00",
        "f-5",
        in_change(
            2,
            FileEvent {
                src: Some(format!(r"{addons}\pak03_dir.vpk")),
                result: Outcome::Failed,
                ..FileEvent::new(FileOp::Delete)
            },
        ),
    );
    // Not in the game folder: imports and the image cache.
    file(
        &dir,
        TODAY,
        "12:01",
        "f-6",
        FileEvent {
            dst: Some(lib.into()),
            ..FileEvent::new(FileOp::Copy)
        },
    );
    let mut tallies = Tallies::default();
    let game = tallies
        .summary(&dir, at(15, 0), &nobody())
        .expect("summary")
        .game;
    assert_eq!(game.changes, 3);
    assert_eq!(game.gameinfo, 1);
    assert_eq!(game.ops.get(&FileOp::Hardlink), Some(&1));
    assert_eq!(game.ops.get(&FileOp::Delete), Some(&1));
    assert_eq!(game.ops.get(&FileOp::Rename), None);
    assert_eq!(game.failed, 1);
    assert_eq!(game.batches, 2);
    assert_eq!(game.folders, [addons]);
}

#[test]
fn the_summary_hides_the_user_name_in_folders() {
    let dir = MemLogDir::default();
    file(
        &dir,
        TODAY,
        "09:00",
        "f-1",
        FileEvent {
            dst: Some(r"C:\Users\Alice\Games\Deadlock\game\citadel\addons\pak01_dir.vpk".into()),
            ..FileEvent::new(FileOp::Hardlink)
        },
    );
    let mut tallies = Tallies::default();
    let game = tallies
        .summary(&dir, at(15, 0), &alice())
        .expect("summary")
        .game;
    assert_eq!(
        game.folders,
        [r"%USERPROFILE%\Games\Deadlock\game\citadel\addons"]
    );
}

#[test]
fn a_fresh_install_has_an_empty_summary() {
    let dir = MemLogDir::default();
    // The app log always has the start lines; they don't count.
    app(
        &dir,
        TODAY,
        "09:00",
        "a-1",
        Level::Info,
        "deadloader started",
    );
    let mut tallies = Tallies::default();
    let summary = tallies
        .summary(&dir, at(15, 0), &nobody())
        .expect("summary");
    assert!(summary.empty);
    assert!(summary.contacted.is_empty() && summary.blocked.is_empty());
    assert_eq!(summary.game.changes, 0);

    net(
        &dir,
        "2026-10-01",
        "09:00",
        "n-1",
        "response",
        hop("r-1", 0, "https://gamebanana.com/x"),
    );
    assert!(
        !tallies
            .summary(&dir, at(15, 0), &nobody())
            .expect("summary")
            .empty
    );
}

// --- the shapes the UI reads --------------------------------------------------------------------------------------

#[test]
fn queries_and_pages_have_the_shapes_the_ui_uses() {
    let q: Query = serde_json::from_value(json!({
        "kind": "network", "range": "7d", "purpose": "download", "result": "error", "host": "gamebanana.com",
        "text": "dl", "limit": 50
    }))
    .expect("query");
    assert_eq!(
        (q.kind, q.range, q.result, q.limit),
        (Kind::Network, Range::Week, Results::Error, 50)
    );
    let q: Query =
        serde_json::from_value(json!({ "kind": "app", "range": "hour", "level": "warn" }))
            .expect("query");
    assert_eq!(
        (q.range, q.level, q.limit),
        (Range::Hour, Some(Level::Warn), DEFAULT_LIMIT)
    );
    let q: Query =
        serde_json::from_value(json!({ "kind": "files", "range": "30d", "op": "hardlink" }))
            .expect("query");
    assert_eq!((q.range, q.op), (Range::Month, Some(FileOp::Hardlink)));

    let dir = MemLogDir::default();
    app(&dir, TODAY, "09:00", "a-1", Level::Info, "x");
    app(&dir, TODAY, "09:01", "a-2", Level::Info, "x");
    let masker = nobody();
    let mut q = query(Kind::App);
    q.limit = 1;
    let page = serde_json::to_value(reader(&dir, &masker).page(&q).expect("page")).expect("json");
    assert_eq!(page["entries"][0]["kind"], "app");
    assert_eq!(page["entries"][0]["record"]["id"], "a-2");
    assert_eq!(page["cursor"]["file"], "app-2026-10-15.jsonl");
    // The UI hands the cursor back as it got it.
    let cursor: Cursor = serde_json::from_value(page["cursor"].clone()).expect("cursor");
    q.cursor = Some(cursor);
    assert_eq!(ids(&reader(&dir, &masker).page(&q).expect("page")), ["a-1"]);
}

#[test]
fn a_filtered_out_request_cut_by_a_page_boundary_does_not_come_back_in_part() {
    let dir = MemLogDir::default();
    let page = "https://gamebanana.com/apiv11/Mod/Index";
    // r-1 failed once, then worked: not an error. Its first try alone would look like one.
    net(
        &dir,
        TODAY,
        "14:00",
        "n-1",
        RETRYING_MSG,
        NetworkEvent {
            status: Some(429),
            ..hop("r-1", 0, page)
        },
    );
    net(
        &dir,
        TODAY,
        "14:00",
        "n-2",
        "response",
        NetworkEvent {
            status: Some(404),
            ..hop("r-x", 0, page)
        },
    );
    net(
        &dir,
        TODAY,
        "14:00",
        "n-3",
        "response",
        NetworkEvent {
            status: Some(404),
            ..hop("r-z", 0, page)
        },
    );
    net(
        &dir,
        TODAY,
        "14:01",
        "n-4",
        "response",
        NetworkEvent {
            attempt: 2,
            ..hop("r-1", 0, page)
        },
    );
    let masker = nobody();
    let mut q = query(Kind::Network);
    q.result = Results::Error;
    q.limit = 1;
    assert_eq!(all_pages(&reader(&dir, &masker), q), ["r-z", "r-x"]);
}
