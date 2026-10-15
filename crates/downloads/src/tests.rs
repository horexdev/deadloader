//! The queue end to end, against an HTTP server in memory, a fake clock and a temp folder. No real network.

use crate::log::mem::MemLog;
use crate::store::QUEUE_FILE;
use crate::test_support::{
    Call, FakeClock, FakeServer, Fault, Gate, NetRecords, Served, ServerHandle, TempDir, content,
    md5_hex,
};
use crate::{
    CompletionHook, Config, Deps, DownloadError, Downloads, Event, NewTask, TaskId, TaskInfo,
    TaskState, Timing,
};
use deadloader_core::log::FileOp;
use deadloader_core::message::{MessageId, UserMessage};
use deadloader_net::transport::TransportErrorKind;
use deadloader_net::{Allowlist, Client, RetryPolicy};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use std::time::Duration;

const WAIT: Duration = Duration::from_secs(20);

fn url(n: u32) -> String {
    format!("https://gamebanana.com/dl/{n}")
}

#[derive(Default)]
struct Events {
    list: Mutex<Vec<Event>>,
    changed: Condvar,
}

impl Events {
    // Poison-tolerant: a failed assertion in one test thread must not take the queue's threads down with it.
    fn lock(&self) -> std::sync::MutexGuard<'_, Vec<Event>> {
        self.list.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn push(&self, event: &Event) {
        self.lock().push(event.clone());
        self.changed.notify_all();
    }

    fn all(&self) -> Vec<Event> {
        self.lock().clone()
    }

    /// Waits until `done` holds for the events so far.
    fn wait(&self, what: &str, done: impl Fn(&[Event]) -> bool) {
        let mut list = self.lock();
        let deadline = std::time::Instant::now() + WAIT;
        while !done(&list) {
            let left = deadline.saturating_duration_since(std::time::Instant::now());
            if left.is_zero() {
                let seen = format!("{list:#?}");
                drop(list);
                panic!("timed out waiting for {what}: {seen}");
            }
            list = self
                .changed
                .wait_timeout(list, left)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
    }

    fn states(&self, id: TaskId) -> Vec<String> {
        self.all()
            .iter()
            .filter_map(|e| match e {
                Event::State { id: i, state } if *i == id => Some(state.name().to_owned()),
                _ => None,
            })
            .collect()
    }

    fn last_state(&self, id: TaskId) -> Option<TaskState> {
        self.all().iter().rev().find_map(|e| match e {
            Event::State { id: i, state } if *i == id => Some(state.clone()),
            _ => None,
        })
    }

    fn progress_count(&self, id: TaskId) -> usize {
        self.all()
            .iter()
            .filter(|e| matches!(e, Event::Progress { id: i, .. } if *i == id))
            .count()
    }

    fn max_bytes(&self, id: TaskId) -> u64 {
        self.all()
            .iter()
            .filter_map(|e| match e {
                Event::Progress { id: i, bytes, .. } if *i == id => Some(*bytes),
                _ => None,
            })
            .max()
            .unwrap_or(0)
    }
}

fn removed(id: TaskId) -> impl Fn(&[Event]) -> bool {
    move |events| {
        events
            .iter()
            .any(|e| matches!(e, Event::Removed { id: i } if *i == id))
    }
}

fn state_is(id: TaskId, name: &'static str) -> impl Fn(&[Event]) -> bool {
    move |events| {
        events.iter().rev().find_map(|e| match e {
            Event::State { id: i, state } if *i == id => Some(state.name() == name),
            _ => None,
        }) == Some(true)
    }
}

/// Records every hand-off and the archive's content at that moment.
#[derive(Default)]
struct Hook {
    calls: Mutex<Vec<(PathBuf, TaskInfo, Vec<u8>)>>,
    fail_next: Mutex<Option<UserMessage>>,
}

impl CompletionHook for Hook {
    fn downloaded(&self, archive: &Path, task: &TaskInfo) -> Result<(), UserMessage> {
        let bytes = std::fs::read(archive).expect("the archive exists during the hook");
        self.calls
            .lock()
            .expect("lock")
            .push((archive.to_owned(), task.clone(), bytes));
        match self.fail_next.lock().expect("lock").take() {
            Some(message) => Err(message),
            None => Ok(()),
        }
    }
}

impl Hook {
    fn contents(&self) -> Vec<Vec<u8>> {
        self.calls
            .lock()
            .expect("lock")
            .iter()
            .map(|(_, _, b)| b.clone())
            .collect()
    }
}

struct Harness {
    _dir: TempDir,
    server: Arc<FakeServer>,
    clock: FakeClock,
    log: Arc<MemLog>,
    net: NetRecords,
    events: Arc<Events>,
    hook: Arc<Hook>,
    space: Arc<Mutex<Option<u64>>>,
    config: Config,
    downloads: Option<Downloads>,
}

impl Harness {
    fn new(name: &str) -> Self {
        let dir = TempDir::new(name);
        let config = Config {
            dir: dir.path().join("downloads"),
            parallel: 4,
            speed_limit_kib_per_s: None,
            keep_archives: false,
            timing: Timing::default(),
        };
        Self {
            _dir: dir,
            server: FakeServer::new(),
            clock: FakeClock::new(),
            log: Arc::new(MemLog::default()),
            net: NetRecords::default(),
            events: Arc::new(Events::default()),
            hook: Arc::new(Hook::default()),
            space: Arc::new(Mutex::new(None)),
            config,
            downloads: None,
        }
    }

    fn deps(&self) -> Deps {
        let client = Client::new(
            ServerHandle(Arc::clone(&self.server)),
            self.net.clone(),
            self.clock.clone(),
            Allowlist::new(),
        )
        .with_retry_policy(RetryPolicy {
            max_retries: 1,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(1),
        });
        let events = Arc::clone(&self.events);
        let space = Arc::clone(&self.space);
        Deps {
            client: Arc::new(client),
            clock: self.clock.shared(),
            log: self.log.clone(),
            space: Arc::new(move |_: &Path| *space.lock().expect("lock")),
            hook: self.hook.clone(),
            events: Arc::new(move |e: &Event| events.push(e)),
        }
    }

    fn open(&mut self) -> &Downloads {
        let downloads = Downloads::open(self.config.clone(), self.deps()).expect("open");
        self.downloads = Some(downloads);
        self.dl()
    }

    fn dl(&self) -> &Downloads {
        self.downloads.as_ref().expect("open first")
    }

    /// Stops the queue like closing the app does.
    fn close(&mut self) {
        if let Some(downloads) = self.downloads.take() {
            downloads.shutdown();
        }
    }

    fn data(&self) -> &Path {
        &self.config.dir
    }

    fn part(&self, id: TaskId) -> PathBuf {
        self.data().join(format!("{id}.part"))
    }

    /// Waits until the `.part` holds at least `n` bytes: progress events are throttled by the fake clock, which
    /// stands still while nothing sleeps on it.
    fn wait_part(&self, id: TaskId, n: u64) {
        let deadline = std::time::Instant::now() + WAIT;
        while std::fs::metadata(self.part(id)).map_or(0, |m| m.len()) < n {
            assert!(
                std::time::Instant::now() < deadline,
                "timed out waiting for {n} bytes of {id}"
            );
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    /// The queue file once every worker has saved it.
    fn queue_json(&self) -> serde_json::Value {
        if let Some(dl) = &self.downloads {
            dl.shared().wait_idle();
        }
        let text = std::fs::read_to_string(self.data().join(QUEUE_FILE)).expect("queue file");
        serde_json::from_str(&text).expect("queue json")
    }

    /// Serves `bytes` at `url(n)` and queues it with its md5 and size.
    fn add(&self, n: u32, bytes: &[u8]) -> TaskId {
        self.server
            .serve(&url(n), Served::new(bytes.to_vec(), &format!("v{n}")));
        self.dl()
            .add(
                NewTask::new(url(n), format!("file{n}.zip"))
                    .md5(md5_hex(bytes))
                    .size(bytes.len() as u64)
                    .meta("mod", n.to_string()),
            )
            .expect("add")
    }

    fn file_ops(&self) -> Vec<(String, FileOp)> {
        self.log
            .files()
            .into_iter()
            .map(|(msg, e)| (msg, e.op))
            .collect()
    }

    fn calls_to(&self, n: u32) -> Vec<Call> {
        self.server
            .calls()
            .into_iter()
            .filter(|c| c.url == url(n))
            .collect()
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        self.close();
    }
}

fn message_of(state: Option<TaskState>) -> UserMessage {
    match state {
        Some(TaskState::Failed { message } | TaskState::Waiting { message, .. }) => message,
        other => panic!("not failed or waiting: {other:?}"),
    }
}

fn arg<'a>(m: &'a UserMessage, key: &str) -> Option<&'a str> {
    m.args.get(key).map(String::as_str)
}

#[test]
fn a_download_is_verified_handed_off_and_its_archive_deleted() {
    let mut h = Harness::new("happy");
    h.open();
    let bytes = content(300_000, 1);
    let id = h.add(1, &bytes);
    h.events.wait("done", removed(id));

    assert_eq!(
        h.events.states(id),
        ["downloading", "verifying", "installing", "done"]
    );
    assert_eq!(h.hook.contents(), std::slice::from_ref(&bytes));
    let (archive, info, _) = h.hook.calls.lock().expect("lock")[0].clone();
    assert_eq!(archive, h.data().join("1.zip"));
    assert_eq!(info.meta["mod"], "1");
    assert!(!archive.exists(), "keep_archives is off");
    assert!(!h.part(id).exists());
    assert_eq!(h.queue_json()["tasks"], serde_json::json!([]));
    assert!(h.dl().tasks().is_empty());
    let ops = h.file_ops();
    for (msg, op) in [
        ("created a download part", FileOp::Write),
        ("verified a download", FileOp::Rename),
        ("deleted an installed download", FileOp::Delete),
    ] {
        assert!(
            ops.contains(&(msg.to_owned(), op)),
            "{msg} missing from {ops:?}"
        );
    }
    let first = &h.server.calls()[0];
    assert_eq!(first.header("range"), None);
    assert!(
        first
            .header("user-agent")
            .is_some_and(|ua| ua.starts_with("deadloader/"))
    );
    let net = h.net.0.lock().expect("lock").clone();
    assert!(!net.is_empty(), "the request is in the network log");
}

#[test]
fn kept_archives_stay() {
    let mut h = Harness::new("keep");
    h.config.keep_archives = true;
    h.open();
    let id = h.add(1, &content(1000, 2));
    h.events.wait("done", removed(id));
    assert!(h.data().join("1.zip").exists());
}

#[test]
fn a_download_resumes_after_an_interruption_at_any_byte() {
    let len = 200_000;
    for cut in [1, 1000, 65_535, 65_536, 65_537, 131_073, len - 1] {
        let mut h = Harness::new("resume");
        h.open();
        let bytes = content(len, 3);
        h.server.fault(&url(1), Fault::CutAfter(cut));
        let id = h.add(1, &bytes);
        h.events.wait("waiting", state_is(id, "waiting"));
        let message = message_of(h.events.last_state(id));
        assert_eq!(
            (message.id, arg(&message, "reason")),
            (MessageId::E6, Some("connection_lost"))
        );
        assert_eq!(
            std::fs::metadata(h.part(id)).expect("part kept").len(),
            cut as u64,
            "cut {cut}"
        );
        assert_eq!(h.queue_json()["tasks"][0]["checkpoint"], cut);
        assert_eq!(h.queue_json()["tasks"][0]["state"], "waiting");

        h.dl().retry(id).expect("retry");
        h.events.wait("done", removed(id));
        assert_eq!(h.hook.contents(), std::slice::from_ref(&bytes), "cut {cut}");
        let calls = h.calls_to(1);
        assert_eq!(calls.len(), 2);
        let range = format!("bytes={cut}-");
        assert_eq!(calls[1].header("range"), Some(range.as_str()));
        assert_eq!(calls[1].header("if-range"), Some("\"v1\""));
    }
}

#[test]
fn offline_tasks_retry_on_their_own() {
    let mut h = Harness::new("offline");
    h.open();
    h.server
        .fault(&url(1), Fault::Fail(TransportErrorKind::Connect));
    let id = h.add(1, &content(5000, 4));
    h.events.wait("waiting", state_is(id, "waiting"));
    match h.events.last_state(id) {
        Some(TaskState::Waiting {
            message,
            retry_in_s,
        }) => {
            assert_eq!(message.id, MessageId::E6);
            assert_eq!(arg(&message, "reason"), Some("connect"));
            assert!(arg(&message, "log_id").is_some(), "points at the log line");
            assert_eq!(retry_in_s, Some(5));
        }
        other => panic!("{other:?}"),
    }
    h.clock.advance(Duration::from_secs(5));
    h.dl().shared().notify();
    h.events.wait("done", removed(id));
}

#[test]
fn the_network_coming_back_retries_waiting_tasks_now() {
    let mut h = Harness::new("online");
    h.open();
    h.server
        .fault(&url(1), Fault::Fail(TransportErrorKind::Dns));
    let id = h.add(1, &content(5000, 5));
    h.events.wait("waiting", state_is(id, "waiting"));
    h.dl().retry_waiting();
    h.events.wait("done", removed(id));
}

#[test]
fn local_mode_waits_without_a_timer() {
    let mut h = Harness::new("local");
    h.open();
    h.dl().shared().deps.client.set_local_mode(true);
    let id = h.add(1, &content(5000, 6));
    h.events.wait("waiting", state_is(id, "waiting"));
    match h.events.last_state(id) {
        Some(TaskState::Waiting { retry_in_s, .. }) => assert_eq!(retry_in_s, None),
        other => panic!("{other:?}"),
    }
    assert!(h.server.calls().is_empty(), "nothing was sent");
    h.dl().shared().deps.client.set_local_mode(false);
    h.dl().retry_waiting();
    h.events.wait("done", removed(id));
}

#[test]
fn a_server_without_range_support_starts_over() {
    let mut h = Harness::new("norange");
    h.open();
    let bytes = content(150_000, 7);
    h.server.fault(&url(1), Fault::CutAfter(70_000));
    let id = h.add(1, &bytes);
    let mut served = Served::new(bytes.clone(), "v1");
    served.ranges = false;
    h.server.serve(&url(1), served);
    h.events.wait("waiting", state_is(id, "waiting"));
    h.dl().retry(id).expect("retry");
    h.events.wait("done", removed(id));
    assert_eq!(h.hook.contents(), [bytes]);
    assert!(
        h.log
            .app_msgs()
            .contains(&"download restarted from the beginning".to_owned())
    );
    assert!(h.file_ops().contains(&(
        "emptied a download part to start over".to_owned(),
        FileOp::Write
    )));
}

#[test]
fn a_changed_file_is_downloaded_again_from_the_start() {
    // The server honours If-Range: a new version comes back whole (200).
    let mut h = Harness::new("validator");
    h.open();
    let old = content(150_000, 8);
    let new = content(150_000, 9);
    h.server.fault(&url(1), Fault::CutAfter(80_000));
    h.server.serve(&url(1), Served::new(old.clone(), "v1"));
    let id = h
        .dl()
        .add(NewTask::new(url(1), "f.zip").md5(md5_hex(&new)))
        .expect("add");
    h.events.wait("waiting", state_is(id, "waiting"));
    h.server.serve(&url(1), Served::new(new.clone(), "v2"));
    h.dl().retry(id).expect("retry");
    h.events.wait("done", removed(id));
    assert_eq!(h.hook.contents(), [new]);
    let calls = h.calls_to(1);
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[1].header("if-range"), Some("\"v1\""));
}

#[test]
fn a_changed_file_behind_a_server_ignoring_if_range_is_asked_for_again() {
    // The server answers 206 with another ETag: the rest of a different file, useless for us.
    let mut h = Harness::new("validator206");
    h.open();
    let old = content(150_000, 10);
    let new = content(150_000, 11);
    h.server.fault(&url(1), Fault::CutAfter(80_000));
    h.server.serve(&url(1), Served::new(old, "v1"));
    let id = h
        .dl()
        .add(NewTask::new(url(1), "f.zip").md5(md5_hex(&new)))
        .expect("add");
    h.events.wait("waiting", state_is(id, "waiting"));
    let mut served = Served::new(new.clone(), "v2");
    served.if_range = false;
    h.server.serve(&url(1), served);
    h.dl().retry(id).expect("retry");
    h.events.wait("done", removed(id));
    assert_eq!(h.hook.contents(), [new]);
    let calls = h.calls_to(1);
    assert_eq!(calls.len(), 3);
    assert!(calls[1].header("range").is_some());
    assert_eq!(calls[2].header("range"), None, "the whole file again");
}

#[test]
fn an_md5_mismatch_deletes_the_part_and_can_be_retried() {
    let mut h = Harness::new("md5");
    h.open();
    let good = content(100_000, 12);
    let mut bad = good.clone();
    bad[50_000] ^= 0xff;
    h.server.serve(&url(1), Served::new(bad, "v1"));
    let id = h
        .dl()
        .add(
            NewTask::new(url(1), "f.zip")
                .md5(md5_hex(&good))
                .size(good.len() as u64),
        )
        .expect("add");
    h.events.wait("failed", state_is(id, "failed"));
    let message = message_of(h.events.last_state(id));
    assert_eq!(
        (message.id, arg(&message, "reason")),
        (MessageId::E7, Some("checksum"))
    );
    assert_eq!(arg(&message, "expected"), Some(md5_hex(&good).as_str()));
    assert!(!h.part(id).exists(), "the damaged part is gone");
    assert!(h.hook.contents().is_empty());
    assert_eq!(h.queue_json()["tasks"][0]["state"], "failed");
    assert_eq!(h.queue_json()["tasks"][0]["message"]["id"], "E7");

    h.server.serve(&url(1), Served::new(good.clone(), "v2"));
    h.dl().retry(id).expect("retry");
    h.events.wait("done", removed(id));
    assert_eq!(h.hook.contents(), [good]);
    assert_eq!(h.calls_to(1)[1].header("range"), None);
}

#[test]
fn a_size_other_than_the_listed_one_fails_before_downloading() {
    let mut h = Harness::new("size");
    h.open();
    let bytes = content(1000, 13);
    h.server.serve(&url(1), Served::new(bytes, "v1"));
    let id = h
        .dl()
        .add(NewTask::new(url(1), "f.zip").size(999))
        .expect("add");
    h.events.wait("failed", state_is(id, "failed"));
    let message = message_of(h.events.last_state(id));
    assert_eq!(
        (message.id, arg(&message, "reason")),
        (MessageId::E7, Some("size"))
    );
    assert_eq!(h.events.max_bytes(id), 0);
    assert!(!h.part(id).exists());
}

#[test]
fn downloads_start_in_order_and_never_more_than_the_limit() {
    let mut h = Harness::new("parallel");
    h.config.parallel = 2;
    h.open();
    let gate = Gate::new();
    let ids: Vec<TaskId> = (1..=5)
        .map(|n| {
            h.server
                .fault(&url(n), Fault::HoldAt(10_000, Arc::clone(&gate)));
            h.add(n, &content(50_000, n))
        })
        .collect();
    h.wait_part(ids[1], 10_000);
    std::thread::sleep(Duration::from_millis(100));
    let started: Vec<String> = h.server.calls().into_iter().map(|c| c.url).collect();
    assert_eq!(started, [url(1), url(2)]);
    for id in &ids[2..] {
        assert_eq!(h.dl().task(*id).expect("task").state, TaskState::Queued);
    }
    gate.open();
    for id in &ids {
        h.events.wait("all done", removed(*id));
    }
    assert_eq!(h.server.most_open(), 2);
    // Started in queue order (two started together may reach the server in either order).
    let started: Vec<TaskId> = h
        .events
        .all()
        .iter()
        .filter_map(|e| match e {
            Event::State {
                id,
                state: TaskState::Downloading,
            } => Some(*id),
            _ => None,
        })
        .collect();
    assert_eq!(started, ids);
}

#[test]
fn the_speed_limit_holds_across_the_queue() {
    let mut h = Harness::new("speed");
    h.config.speed_limit_kib_per_s = Some(100);
    h.open();
    let bytes = content(1024 * 1024, 14);
    let id = h.add(1, &bytes);
    h.events.wait("done", removed(id));
    let secs = h.clock.elapsed().as_secs_f64();
    // 1 MiB at 100 KiB/s = 10.24 s; within 5 %.
    assert!((secs - 10.24).abs() < 0.52, "{secs}");
    // About ten progress events a second.
    let progress = h.events.progress_count(id);
    assert!((90..=120).contains(&progress), "{progress}");
    let speeds: Vec<u64> = h
        .events
        .all()
        .iter()
        .filter_map(|e| match e {
            Event::Progress { speed, .. } => Some(*speed),
            _ => None,
        })
        .collect();
    let late = speeds[speeds.len() / 2];
    assert!(
        (late as f64 - 102_400.0).abs() < 102_400.0 * 0.15,
        "smoothed speed {late}"
    );
}

#[test]
fn the_speed_limit_changes_while_downloading() {
    let mut h = Harness::new("speed-change");
    h.config.speed_limit_kib_per_s = Some(10);
    h.open();
    let gate = Gate::new();
    h.server
        .fault(&url(1), Fault::HoldAt(20_000, Arc::clone(&gate)));
    let id = h.add(1, &content(220_000, 15));
    h.wait_part(id, 20_000);
    let before = h.clock.elapsed();
    h.dl().set_speed_limit(Some(100));
    gate.open();
    h.events.wait("done", removed(id));
    let secs = (h.clock.elapsed() - before).as_secs_f64();
    // 200 000 bytes at 100 KiB/s ≈ 1.95 s (at 10 KiB/s it would be 19.5 s).
    assert!(secs < 2.5, "{secs}");
}

#[test]
fn cancel_deletes_the_part() {
    let mut h = Harness::new("cancel");
    h.open();
    let gate = Gate::new();
    h.server
        .fault(&url(1), Fault::HoldAt(100_000, Arc::clone(&gate)));
    let id = h.add(1, &content(300_000, 16));
    h.wait_part(id, 100_000);
    assert!(h.part(id).exists());
    h.dl().cancel(id).expect("cancel");
    h.events.wait("removed", removed(id));
    gate.break_off();
    assert!(!h.part(id).exists());
    assert!(h.dl().tasks().is_empty());
    assert_eq!(h.queue_json()["tasks"], serde_json::json!([]));
    assert!(
        h.file_ops()
            .contains(&("deleted a download part".to_owned(), FileOp::Delete))
    );
    assert!(h.hook.contents().is_empty());
}

#[test]
fn cancel_of_a_queued_or_failed_task_cleans_up_at_once() {
    let mut h = Harness::new("cancel-queued");
    h.config.parallel = 1;
    h.open();
    let gate = Gate::new();
    h.server.fault(&url(1), Fault::HoldAt(0, Arc::clone(&gate)));
    let first = h.add(1, &content(1000, 17));
    let second = h.add(2, &content(1000, 18));
    h.dl().cancel(second).expect("cancel queued");
    h.events.wait("removed", removed(second));
    assert!(h.calls_to(2).is_empty());
    gate.open();
    h.events.wait("done", removed(first));
    assert!(matches!(
        h.dl().cancel(first),
        Err(DownloadError::UnknownTask(_))
    ));
}

#[test]
fn pause_keeps_the_part_and_resume_continues_from_it() {
    let mut h = Harness::new("pause");
    h.open();
    let gate = Gate::new();
    let bytes = content(300_000, 19);
    h.server
        .fault(&url(1), Fault::HoldAt(120_000, Arc::clone(&gate)));
    let id = h.add(1, &bytes);
    h.wait_part(id, 120_000);
    h.dl().pause(id).expect("pause");
    h.events.wait("paused", state_is(id, "paused"));
    gate.break_off();
    assert_eq!(std::fs::metadata(h.part(id)).expect("part").len(), 120_000);
    assert_eq!(h.queue_json()["tasks"][0]["state"], "paused");
    assert_eq!(h.queue_json()["tasks"][0]["checkpoint"], 120_000);
    h.dl().resume(id).expect("resume");
    h.events.wait("done", removed(id));
    assert_eq!(h.hook.contents(), [bytes]);
    assert_eq!(h.calls_to(1)[1].header("range"), Some("bytes=120000-"));
}

#[test]
fn queued_tasks_can_be_paused_and_installing_ones_cannot_be_cancelled() {
    let mut h = Harness::new("transitions");
    h.config.parallel = 1;
    h.open();
    let gate = Gate::new();
    h.server.fault(&url(1), Fault::HoldAt(0, Arc::clone(&gate)));
    let first = h.add(1, &content(1000, 20));
    let second = h.add(2, &content(1000, 21));
    h.dl().pause(second).expect("pause queued");
    assert_eq!(h.dl().task(second).expect("task").state, TaskState::Paused);
    assert!(matches!(
        h.dl().retry(second),
        Err(DownloadError::InvalidState { .. })
    ));
    gate.open();
    h.events.wait("done", removed(first));
    std::thread::sleep(Duration::from_millis(50));
    assert!(h.calls_to(2).is_empty(), "a paused task does not start");
    h.dl().resume(second).expect("resume");
    h.events.wait("done", removed(second));
}

#[test]
fn not_enough_free_space_fails_before_any_request() {
    let mut h = Harness::new("space");
    *h.space.lock().expect("lock") = Some(5 * 1024 * 1024);
    h.open();
    let id = h.add(1, &content(10 * 1024 * 1024, 22));
    h.events.wait("failed", state_is(id, "failed"));
    let message = message_of(h.events.last_state(id));
    assert_eq!(message.id, MessageId::E12);
    assert_eq!(arg(&message, "available"), Some("5242880"));
    // 10 MiB + 10 %.
    assert_eq!(arg(&message, "needed"), Some("11534336"));
    assert!(h.server.calls().is_empty());
    *h.space.lock().expect("lock") = None;
    h.dl().retry(id).expect("retry");
    h.events.wait("done", removed(id));
}

#[test]
fn a_full_disk_mid_download_keeps_what_was_written() {
    let mut h = Harness::new("diskfull");
    h.open();
    *h.dl().shared().write_budget.lock().expect("lock") = Some(100_000);
    let bytes = content(300_000, 23);
    let id = h.add(1, &bytes);
    h.events.wait("failed", state_is(id, "failed"));
    let message = message_of(h.events.last_state(id));
    assert_eq!(message.id, MessageId::E12);
    let kept = h.queue_json()["tasks"][0]["checkpoint"]
        .as_u64()
        .expect("checkpoint");
    assert!(kept > 0 && kept <= 100_000, "{kept}");
    *h.dl().shared().write_budget.lock().expect("lock") = None;
    h.dl().retry(id).expect("retry");
    h.events.wait("done", removed(id));
    assert_eq!(h.hook.contents(), [bytes]);
    let range = format!("bytes={kept}-");
    assert_eq!(h.calls_to(1)[1].header("range"), Some(range.as_str()));
}

#[test]
fn a_redirect_off_the_allowlist_is_e16() {
    let mut h = Harness::new("blocked");
    h.open();
    h.server.fault(
        &url(1),
        Fault::Redirect("https://evil.example/x.zip".into()),
    );
    let id = h.add(1, &content(1000, 24));
    h.events.wait("failed", state_is(id, "failed"));
    let message = message_of(h.events.last_state(id));
    assert_eq!(message.id, MessageId::E16);
    assert_eq!(arg(&message, "host"), Some("evil.example"));
    assert_eq!(
        h.server.calls().len(),
        1,
        "the foreign host was never contacted"
    );
}

#[test]
fn a_busy_server_is_e5_and_waits() {
    let mut h = Harness::new("busy");
    h.open();
    h.server.fault(&url(1), Fault::Status(503));
    h.server.fault(&url(1), Fault::Status(503));
    let id = h.add(1, &content(1000, 25));
    h.events.wait("waiting", state_is(id, "waiting"));
    let message = message_of(h.events.last_state(id));
    assert_eq!(
        (message.id, arg(&message, "status")),
        (MessageId::E5, Some("503"))
    );
    h.dl().retry(id).expect("retry");
    h.events.wait("done", removed(id));
}

#[test]
fn a_missing_file_fails_with_its_status() {
    let mut h = Harness::new("missing");
    h.open();
    let id = h.dl().add(NewTask::new(url(9), "gone.zip")).expect("add");
    h.events.wait("failed", state_is(id, "failed"));
    let message = message_of(h.events.last_state(id));
    assert_eq!(
        (message.id, arg(&message, "status")),
        (MessageId::E24, Some("404")),
        "gone from GameBanana, not \"it will resume\" (173)"
    );
}

#[test]
fn a_stalled_connection_times_out_and_waits() {
    let mut h = Harness::new("stall");
    h.config.timing.idle_timeout = Duration::from_millis(200);
    h.open();
    let gate = Gate::new();
    h.server
        .fault(&url(1), Fault::HoldAt(50_000, Arc::clone(&gate)));
    let id = h.add(1, &content(100_000, 26));
    h.events.wait("waiting", state_is(id, "waiting"));
    gate.break_off();
    let message = message_of(h.events.last_state(id));
    assert_eq!(
        (message.id, arg(&message, "reason")),
        (MessageId::E6, Some("timeout"))
    );
    assert_eq!(h.queue_json()["tasks"][0]["checkpoint"], 50_000);
    h.dl().retry(id).expect("retry");
    h.events.wait("done", removed(id));
}

#[test]
fn a_failed_hook_keeps_the_archive_and_retry_only_runs_the_hook() {
    let mut h = Harness::new("hook");
    h.open();
    *h.hook.fail_next.lock().expect("lock") =
        Some(UserMessage::new(MessageId::E8).arg("reason", "damaged"));
    let bytes = content(5000, 27);
    let id = h.add(1, &bytes);
    h.events.wait("failed", state_is(id, "failed"));
    let message = message_of(h.events.last_state(id));
    assert_eq!(message.id, MessageId::E8);
    assert!(h.data().join("1.zip").exists());
    h.dl().retry(id).expect("retry");
    h.events.wait("done", removed(id));
    assert_eq!(h.hook.contents(), [bytes.clone(), bytes]);
    assert_eq!(h.calls_to(1).len(), 1, "not downloaded again");
    assert!(!h.data().join("1.zip").exists());
}

#[test]
fn the_queue_survives_a_restart() {
    let mut h = Harness::new("restart");
    h.config.parallel = 1;
    h.open();
    let gate = Gate::new();
    let one = content(200_000, 28);
    let two = content(3000, 29);
    let three = content(3000, 30);
    h.server
        .fault(&url(1), Fault::HoldAt(90_000, Arc::clone(&gate)));
    let first = h.add(1, &one);
    let second = h.add(2, &two);
    let third = h.add(3, &three);
    h.dl().pause(third).expect("pause");
    h.wait_part(first, 90_000);
    h.close();
    gate.break_off();
    let saved = h.queue_json();
    assert_eq!(saved["tasks"][0]["state"], "queued");
    assert_eq!(saved["tasks"][0]["checkpoint"], 90_000);
    assert_eq!(saved["tasks"][1]["state"], "queued");
    assert_eq!(saved["tasks"][2]["state"], "paused");

    // A new run of the app.
    h.events.list.lock().expect("lock").clear();
    let dl = h.open();
    assert_eq!(
        dl.tasks().iter().map(|t| t.info.id).collect::<Vec<_>>(),
        [first, second, third]
    );
    h.events.wait("first done", removed(first));
    h.events.wait("second done", removed(second));
    assert_eq!(h.dl().task(third).expect("task").state, TaskState::Paused);
    assert_eq!(h.hook.contents(), [one, two]);
    assert_eq!(h.calls_to(1)[1].header("range"), Some("bytes=90000-"));
    let fourth = h.add(4, &content(10, 31));
    assert_eq!(fourth, TaskId(4), "ids are not reused");
}

#[test]
fn a_power_cut_leaves_a_resumable_part() {
    // The `.part` grew past the last checkpoint, but those bytes were never confirmed on disk (after a power cut they
    // may be zeros). They are cut off and fetched again.
    let mut h = Harness::new("power");
    let bytes = content(200_000, 32);
    std::fs::create_dir_all(h.data()).expect("dir");
    let mut part = bytes[..60_000].to_vec();
    part.extend(std::iter::repeat_n(0u8, 40_000));
    std::fs::write(h.data().join("1.part"), &part).expect("part");
    let queue = serde_json::json!({
        "schema": 1,
        "next_id": 2,
        "tasks": [{
            "id": 1,
            "url": url(1),
            "file_name": "f.zip",
            "md5": md5_hex(&bytes),
            "size": bytes.len(),
            "state": "queued",
            "checkpoint": 60_000,
            "validator": { "etag": "\"v1\"" }
        }]
    });
    std::fs::write(h.data().join(QUEUE_FILE), queue.to_string()).expect("queue");
    // A temp file from a save that never finished is ignored.
    std::fs::write(h.data().join(format!("{QUEUE_FILE}.tmp")), "{\"sch").expect("tmp");
    h.server.serve(&url(1), Served::new(bytes.clone(), "v1"));
    h.open();
    h.events.wait("done", removed(TaskId(1)));
    assert_eq!(h.hook.contents(), [bytes]);
    assert_eq!(h.calls_to(1)[0].header("range"), Some("bytes=60000-"));
    assert!(h.file_ops().contains(&(
        "cut a download part back to its last saved position".to_owned(),
        FileOp::Write
    )));
}

#[test]
fn a_complete_part_is_verified_without_a_request() {
    // Closed after the last byte arrived but before the rename.
    let mut h = Harness::new("complete");
    let bytes = content(70_000, 35);
    std::fs::create_dir_all(h.data()).expect("dir");
    std::fs::write(h.data().join("1.part"), &bytes).expect("part");
    let queue = serde_json::json!({
        "schema": 1,
        "next_id": 2,
        "tasks": [{
            "id": 1, "url": url(1), "file_name": "f.7z", "md5": md5_hex(&bytes), "size": bytes.len(),
            "state": "queued", "checkpoint": bytes.len()
        }]
    });
    std::fs::write(h.data().join(QUEUE_FILE), queue.to_string()).expect("queue");
    h.open();
    h.events.wait("done", removed(TaskId(1)));
    assert_eq!(h.hook.contents(), [bytes]);
    assert!(h.server.calls().is_empty());
    assert_eq!(
        h.hook.calls.lock().expect("lock")[0].0,
        h.data().join("1.7z")
    );
}

#[test]
fn orphan_parts_are_swept_on_open() {
    let mut h = Harness::new("orphans");
    std::fs::create_dir_all(h.data()).expect("dir");
    std::fs::write(h.data().join("7.part"), b"left over").expect("part");
    h.open();
    assert!(!h.data().join("7.part").exists());
    let id = h.add(1, &content(10, 33));
    assert_eq!(id, TaskId(8), "ids continue after files on disk");
}

#[test]
fn adding_the_same_url_twice_returns_the_same_task() {
    let mut h = Harness::new("dup");
    h.config.parallel = 1;
    h.open();
    let gate = Gate::new();
    h.server.fault(&url(1), Fault::HoldAt(0, Arc::clone(&gate)));
    let id = h.add(1, &content(10, 34));
    let again = h.dl().add(NewTask::new(url(1), "x.zip")).expect("add");
    assert_eq!(id, again);
    assert_eq!(h.dl().tasks().len(), 1);
    gate.open();
}

#[test]
fn a_malformed_md5_is_refused() {
    let mut h = Harness::new("badmd5");
    h.open();
    assert!(matches!(
        h.dl().add(NewTask::new(url(1), "x.zip").md5("not-hex")),
        Err(DownloadError::InvalidMd5(_))
    ));
    assert!(h.dl().tasks().is_empty());
}
