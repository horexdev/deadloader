//! The intent journal: changes to the game folder as crash-safe transactions (ADR 0005).
//!
//! A transaction is written as a `begin` record listing every step, a `done` record after each step and a `commit`
//! record; each record is durable before the step it announces runs. Steps are small and reversible (place a file
//! under a temporary name, rename without replacing); deletions are only allowed at the end and run after the commit.
//! After a crash, [`recover`] rolls the open transaction forward when every remaining step is still possible —
//! checked against the real files, since a step may have run without its `done` record — and back otherwise.

use crate::fs::FileOps;
use crate::place::{PlaceError, place};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Step {
    /// Hard-link or copy a library file to a new (temporary) name.
    Place {
        source: PathBuf,
        dest: PathBuf,
        #[serde(with = "crate::hex")]
        sha256: [u8; 32],
    },
    /// Rename without replacing.
    Rename { from: PathBuf, to: PathBuf },
    /// Remove a file that earlier steps moved out of the way. Irreversible, so only after the commit.
    Delete { path: PathBuf },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "rec", rename_all = "snake_case")]
enum Record {
    Begin {
        tx: u64,
        steps: Vec<Step>,
        /// What the change achieves (e.g. the new slot layout), handed back by recovery so the caller can record it.
        #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
        meta: serde_json::Value,
    },
    Done {
        tx: u64,
        step: usize,
    },
    Commit {
        tx: u64,
    },
}

/// Durable, line-oriented storage of journal records.
pub trait JournalStore {
    /// Appends one line and makes it durable before returning.
    fn append(&self, line: &str) -> io::Result<()>;
    fn read_all(&self) -> io::Result<String>;
    fn clear(&self) -> io::Result<()>;
}

/// `deadloader-data\journal.jsonl`.
#[derive(Debug, Clone)]
pub struct FileJournal {
    path: PathBuf,
}

impl FileJournal {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl JournalStore for FileJournal {
    fn append(&self, line: &str) -> io::Result<()> {
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        f.write_all(line.as_bytes())?;
        f.write_all(b"\n")?;
        f.sync_data()
    }

    fn read_all(&self) -> io::Result<String> {
        match std::fs::read_to_string(&self.path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(String::new()),
            other => other,
        }
    }

    fn clear(&self) -> io::Result<()> {
        std::fs::File::create(&self.path)?.sync_all()
    }
}

#[derive(Debug)]
pub enum TxError {
    /// Deletions before other steps.
    Invalid,
    /// The journal could not be written; what ran has been undone where possible.
    Journal(io::Error),
    /// A step failed; earlier steps have been undone where possible.
    Step { index: usize, error: PlaceError },
    /// Committed, but leftovers could not be removed yet; the next start retries.
    Cleanup(io::Error),
}

impl std::fmt::Display for TxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => f.write_str("invalid plan: deletions must come last"),
            Self::Journal(e) => write!(f, "could not write the intent journal: {e}"),
            Self::Step { index, error } => write!(f, "step {} failed: {error}", index + 1),
            Self::Cleanup(e) => write!(
                f,
                "changes applied, but leftover files could not be removed: {e}"
            ),
        }
    }
}

impl std::error::Error for TxError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recovery {
    /// No interrupted transaction.
    Clean,
    /// It had been committed; leftovers were removed.
    Completed,
    RolledForward,
    RolledBack,
}

/// Runs `steps` as one transaction.
pub fn run(
    fs: &impl FileOps,
    store: &impl JournalStore,
    tx: u64,
    steps: &[Step],
) -> Result<(), TxError> {
    run_with(fs, store, tx, steps, serde_json::Value::Null, || Ok(()))
}

/// Runs `steps` as one transaction, storing `meta` with it and calling `on_commit` once the commit record is durable
/// and before the journal is cleared — the place to record the result (e.g. save the manifest). If `on_commit` fails,
/// the journal stays and [`recover_with_meta`] hands `meta` back on the next start.
pub fn run_with(
    fs: &impl FileOps,
    store: &impl JournalStore,
    tx: u64,
    steps: &[Step],
    meta: serde_json::Value,
    on_commit: impl FnOnce() -> io::Result<()>,
) -> Result<(), TxError> {
    let split = deletions_start(steps).ok_or(TxError::Invalid)?;
    append(
        store,
        &Record::Begin {
            tx,
            steps: steps.to_vec(),
            meta,
        },
    )
    .map_err(TxError::Journal)?;
    for (index, step) in steps[..split].iter().enumerate() {
        if let Err(error) = execute(fs, step) {
            abort(fs, store, &steps[..=index]);
            return Err(TxError::Step { index, error });
        }
        if let Err(e) = append(store, &Record::Done { tx, step: index }) {
            abort(fs, store, &steps[..=index]);
            return Err(TxError::Journal(e));
        }
    }
    if let Err(e) = append(store, &Record::Commit { tx }) {
        abort(fs, store, &steps[..split]);
        return Err(TxError::Journal(e));
    }
    on_commit().map_err(TxError::Cleanup)?;
    cleanup(fs, &steps[split..]).map_err(TxError::Cleanup)?;
    store.clear().map_err(TxError::Cleanup)
}

/// Finishes or undoes a transaction interrupted by a crash. Run before anything else touches the game folder.
pub fn recover(fs: &impl FileOps, store: &impl JournalStore) -> io::Result<Recovery> {
    recover_with_meta(fs, store).map(|(outcome, _)| outcome)
}

/// Like [`recover`], also returning the transaction's `meta` when its changes are now in place (completed or rolled
/// forward), so the caller can record the result it may have missed.
pub fn recover_with_meta(
    fs: &impl FileOps,
    store: &impl JournalStore,
) -> io::Result<(Recovery, Option<serde_json::Value>)> {
    let text = store.read_all()?;
    let Some(open) = last_transaction(&text)? else {
        if !text.trim().is_empty() {
            store.clear()?;
        }
        return Ok((Recovery::Clean, None));
    };
    let split = deletions_start(&open.steps)
        .ok_or_else(|| io::Error::other("journal holds an invalid plan"))?;

    if open.committed {
        cleanup(fs, &open.steps[split..])?;
        store.clear()?;
        return Ok((Recovery::Completed, meta(open.meta)));
    }

    let remaining: Vec<usize> = (0..split).filter(|i| !open.done.contains(i)).collect();
    if can_roll_forward(fs, &open.steps, &remaining)?
        && roll_forward(fs, &open.steps, &remaining).is_ok()
    {
        append(store, &Record::Commit { tx: open.tx })?;
        cleanup(fs, &open.steps[split..])?;
        store.clear()?;
        return Ok((Recovery::RolledForward, meta(open.meta)));
    }

    if split > 0 {
        // The step after the last `done` may have run without its record; undo checks the real files either way.
        let reached = open.done.iter().max().map_or(0, |&i| i + 1).min(split - 1);
        for step in open.steps[..=reached].iter().rev() {
            undo(fs, step)?;
        }
    }
    store.clear()?;
    Ok((Recovery::RolledBack, None))
}

fn meta(value: serde_json::Value) -> Option<serde_json::Value> {
    (!value.is_null()).then_some(value)
}

struct Open {
    tx: u64,
    steps: Vec<Step>,
    meta: serde_json::Value,
    done: BTreeSet<usize>,
    committed: bool,
}

fn last_transaction(text: &str) -> io::Result<Option<Open>> {
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    let mut open: Option<Open> = None;
    for (n, line) in lines.iter().enumerate() {
        let record = match serde_json::from_str::<Record>(line) {
            Ok(r) => r,
            // A crash while writing leaves a damaged last line; anything else is real damage.
            Err(_) if n + 1 == lines.len() => break,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        };
        match record {
            Record::Begin { tx, steps, meta } => {
                open = Some(Open {
                    tx,
                    steps,
                    meta,
                    done: BTreeSet::new(),
                    committed: false,
                })
            }
            Record::Done { tx, step } => {
                if let Some(o) = open.as_mut().filter(|o| o.tx == tx) {
                    o.done.insert(step);
                }
            }
            Record::Commit { tx } => {
                if let Some(o) = open.as_mut().filter(|o| o.tx == tx) {
                    o.committed = true;
                }
            }
        }
    }
    Ok(open)
}

fn deletions_start(steps: &[Step]) -> Option<usize> {
    let split = steps
        .iter()
        .position(|s| matches!(s, Step::Delete { .. }))
        .unwrap_or(steps.len());
    steps[split..]
        .iter()
        .all(|s| matches!(s, Step::Delete { .. }))
        .then_some(split)
}

fn append(store: &impl JournalStore, record: &Record) -> io::Result<()> {
    store.append(&serde_json::to_string(record).map_err(io::Error::other)?)
}

fn execute(fs: &impl FileOps, step: &Step) -> Result<(), PlaceError> {
    match step {
        Step::Place {
            source,
            dest,
            sha256,
        } => place(fs, source, dest, sha256).map(|_| ()),
        Step::Rename { from, to } => fs.rename(from, to).map_err(PlaceError::Io),
        Step::Delete { path } => remove_if_present(fs, path).map_err(PlaceError::Io),
    }
}

fn undo(fs: &impl FileOps, step: &Step) -> io::Result<()> {
    match step {
        Step::Place { dest, .. } => remove_if_present(fs, dest),
        Step::Rename { from, to } => {
            if fs.exists(to)? && !fs.exists(from)? {
                fs.rename(to, from)?;
            }
            Ok(())
        }
        Step::Delete { .. } => Ok(()),
    }
}

/// Best-effort undo right after a failure in this process; whatever stays is left to [`recover`].
fn abort(fs: &impl FileOps, store: &impl JournalStore, ran: &[Step]) {
    if ran.iter().rev().all(|step| undo(fs, step).is_ok()) {
        let _ = store.clear();
    }
}

fn cleanup(fs: &impl FileOps, deletions: &[Step]) -> io::Result<()> {
    for step in deletions {
        if let Step::Delete { path } = step {
            remove_if_present(fs, path)?;
        }
    }
    Ok(())
}

fn remove_if_present(fs: &impl FileOps, path: &Path) -> io::Result<()> {
    match fs.remove_file(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

/// Replays the remaining steps on a model of which files exist, starting from the real files.
fn can_roll_forward(fs: &impl FileOps, steps: &[Step], remaining: &[usize]) -> io::Result<bool> {
    let mut model = Model {
        fs,
        known: HashMap::new(),
    };
    for &i in remaining {
        match &steps[i] {
            Step::Place { source, dest, .. } => {
                if !model.exists(source)? {
                    return Ok(false);
                }
                model.set(dest, true);
            }
            Step::Rename { from, to } => match (model.exists(from)?, model.exists(to)?) {
                (true, false) => {
                    model.set(from, false);
                    model.set(to, true);
                }
                (false, true) => {} // already happened before the crash
                _ => return Ok(false),
            },
            Step::Delete { .. } => {}
        }
    }
    Ok(true)
}

/// Which files exist, as the real filesystem overlaid with the effects of steps replayed so far.
struct Model<'a, F> {
    fs: &'a F,
    known: HashMap<String, bool>,
}

impl<F: FileOps> Model<'_, F> {
    fn key(path: &Path) -> String {
        path.to_string_lossy().to_ascii_lowercase()
    }

    fn exists(&self, path: &Path) -> io::Result<bool> {
        match self.known.get(&Self::key(path)) {
            Some(&e) => Ok(e),
            None => self.fs.exists(path),
        }
    }

    fn set(&mut self, path: &Path, exists: bool) {
        self.known.insert(Self::key(path), exists);
    }
}

fn roll_forward(fs: &impl FileOps, steps: &[Step], remaining: &[usize]) -> Result<(), PlaceError> {
    for &i in remaining {
        match &steps[i] {
            Step::Place { dest, .. } => {
                // A crash may have left a partial copy under the temporary name.
                remove_if_present(fs, dest).map_err(PlaceError::Io)?;
                execute(fs, &steps[i])?;
            }
            Step::Rename { from, to } => {
                if fs.exists(from).map_err(PlaceError::Io)?
                    && !fs.exists(to).map_err(PlaceError::Io)?
                {
                    fs.rename(from, to).map_err(PlaceError::Io)?;
                }
            }
            Step::Delete { .. } => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::fake::{MemFs, Power};
    use crate::fs::sha256;
    use std::cell::RefCell;
    use std::path::PathBuf;

    /// Journal lines in memory; appends fail when the shared power is cut.
    #[derive(Debug, Default)]
    struct MemStore {
        lines: RefCell<Vec<String>>,
        power: Power,
    }

    impl JournalStore for MemStore {
        fn append(&self, line: &str) -> io::Result<()> {
            self.power.spend()?;
            self.lines.borrow_mut().push(line.to_owned());
            Ok(())
        }

        fn read_all(&self) -> io::Result<String> {
            Ok(self
                .lines
                .borrow()
                .iter()
                .map(|l| format!("{l}\n"))
                .collect())
        }

        fn clear(&self) -> io::Result<()> {
            self.power.spend()?;
            self.lines.borrow_mut().clear();
            Ok(())
        }
    }

    fn p(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    /// Library on D:, addons on D:. Before: pak01 = A, pak03 = C. After: pak01 = B (new), pak02 = A, C removed.
    fn scenario() -> (MemFs, MemStore, Vec<Step>) {
        let fs = MemFs::default()
            .volume("D:", true)
            .with(r"D:\lib\b.vpk", b"mod B")
            .with(r"D:\addons\pak01_dir.vpk", b"mod A")
            .with(r"D:\addons\pak03_dir.vpk", b"mod C");
        let store = MemStore {
            power: fs.power(),
            ..MemStore::default()
        };
        let steps = vec![
            Step::Rename {
                from: p(r"D:\addons\pak01_dir.vpk"),
                to: p(r"D:\addons\~dl-1.tmp"),
            },
            Step::Place {
                source: p(r"D:\lib\b.vpk"),
                dest: p(r"D:\addons\~dl-2.tmp"),
                sha256: sha256(b"mod B"),
            },
            Step::Rename {
                from: p(r"D:\addons\pak03_dir.vpk"),
                to: p(r"D:\addons\~dl-3.trash"),
            },
            Step::Rename {
                from: p(r"D:\addons\~dl-1.tmp"),
                to: p(r"D:\addons\pak02_dir.vpk"),
            },
            Step::Rename {
                from: p(r"D:\addons\~dl-2.tmp"),
                to: p(r"D:\addons\pak01_dir.vpk"),
            },
            Step::Delete {
                path: p(r"D:\addons\~dl-3.trash"),
            },
        ];
        (fs, store, steps)
    }

    fn after_state() -> BTreeMapState {
        let fs = MemFs::default()
            .with(r"D:\lib\b.vpk", b"mod B")
            .with(r"D:\addons\pak01_dir.vpk", b"mod B")
            .with(r"D:\addons\pak02_dir.vpk", b"mod A");
        fs.snapshot()
    }

    type BTreeMapState = std::collections::BTreeMap<String, Vec<u8>>;

    #[test]
    fn a_transaction_applies_every_step_and_clears_the_journal() {
        let (fs, store, steps) = scenario();
        run(&fs, &store, 1, &steps).expect("applied");
        assert_eq!(fs.snapshot(), after_state());
        assert!(store.read_all().expect("read").is_empty());
    }

    #[test]
    fn every_record_is_written_before_its_step_runs() {
        let (fs, store, steps) = scenario();
        // Cut the power right after the Begin record: nothing may have changed on disk.
        fs.power().cut_after(1);
        let before = fs.snapshot();
        assert!(run(&fs, &store, 1, &steps).is_err());
        assert_eq!(fs.snapshot(), before);
        assert_eq!(store.lines.borrow().len(), 1, "only the Begin record");
    }

    #[test]
    fn a_failing_step_is_rolled_back_at_once() {
        let (fs, store, mut steps) = scenario();
        // The fourth step targets a name that already exists.
        steps[3] = Step::Rename {
            from: p(r"D:\addons\~dl-1.tmp"),
            to: p(r"D:\lib\b.vpk"),
        };
        let before = fs.snapshot();
        let err = run(&fs, &store, 1, &steps).expect_err("step 3 fails");
        assert!(matches!(err, TxError::Step { index: 3, .. }), "{err:?}");
        assert_eq!(fs.snapshot(), before, "rolled back");
        assert!(
            store.read_all().expect("read").is_empty(),
            "nothing left to recover"
        );
    }

    #[test]
    fn a_crash_at_any_point_recovers_to_before_or_after() {
        let (fs, _, _) = scenario();
        let before = fs.snapshot();
        let after = after_state();
        let mut outcomes = Vec::new();
        // Enough budget to cover every journal write and filesystem operation of the transaction.
        for cut in 0..40 {
            let (fs, store, steps) = scenario();
            fs.power().cut_after(cut);
            let completed = run(&fs, &store, 7, &steps).is_ok();
            fs.power().restore();
            let recovery = recover(&fs, &store).expect("recovery works with power back");
            let state = fs.snapshot();
            assert!(
                state == before || state == after,
                "cut after {cut}: {state:?} after {recovery:?}"
            );
            assert!(
                store.read_all().expect("read").is_empty(),
                "cut after {cut}: journal cleared"
            );
            if completed {
                assert_eq!(recovery, Recovery::Clean);
            }
            outcomes.push(recovery);
        }
        // With every file intact, going forward is always possible, so a crash never forces a rollback here;
        // `recovery_rolls_back_when_going_forward_is_impossible` covers that path.
        assert!(!outcomes.contains(&Recovery::RolledBack));
        assert!(outcomes.contains(&Recovery::RolledForward));
        assert!(outcomes.contains(&Recovery::Completed));
        assert!(outcomes.contains(&Recovery::Clean));
    }

    #[test]
    fn recovery_rolls_back_when_going_forward_is_impossible() {
        let (fs, store, steps) = scenario();
        fs.power().cut_after(4); // Begin, step 0, Done 0, Place
        assert!(run(&fs, &store, 1, &steps).is_err());
        fs.power().restore();
        // Someone deletes the library file before the next start: the Place can't be redone.
        fs.remove_file(Path::new(r"D:\addons\~dl-2.tmp"))
            .expect("temp");
        let lib = fs.read(r"D:\lib\b.vpk").expect("lib");
        fs.remove_file(Path::new(r"D:\lib\b.vpk")).expect("lib");
        assert_eq!(
            recover(&fs, &store).expect("recovery"),
            Recovery::RolledBack
        );
        let mut expected = scenario().0.snapshot();
        expected.remove(r"d:\lib\b.vpk");
        assert_eq!(fs.snapshot(), expected);
        assert_eq!(lib, b"mod B");
    }

    #[test]
    fn an_empty_journal_needs_no_recovery() {
        let (fs, store, _) = scenario();
        assert_eq!(recover(&fs, &store).expect("recovery"), Recovery::Clean);
    }

    #[test]
    fn a_damaged_last_line_is_ignored() {
        let (fs, store, steps) = scenario();
        fs.power().cut_after(2); // Begin + first rename
        assert!(run(&fs, &store, 1, &steps).is_err());
        fs.power().restore();
        store.lines.borrow_mut().push("{\"rec\":\"do".to_owned());
        let r = recover(&fs, &store).expect("recovery");
        assert!(matches!(r, Recovery::RolledForward | Recovery::RolledBack));
        assert!(store.read_all().expect("read").is_empty());
    }

    #[test]
    fn deletions_must_come_after_every_other_step() {
        let (fs, store, mut steps) = scenario();
        steps.swap(0, 5);
        assert!(matches!(run(&fs, &store, 1, &steps), Err(TxError::Invalid)));
    }

    #[test]
    fn records_are_readable_json_lines() {
        let (fs, store, steps) = scenario();
        fs.power().cut_after(1);
        let _ = run(&fs, &store, 42, &steps);
        let line = store.lines.borrow()[0].clone();
        assert!(
            line.starts_with(r#"{"rec":"begin","tx":42,"steps":[{"op":"rename""#),
            "{line}"
        );
        let hash = crate::fs::sha256(b"mod B")
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        assert!(
            line.contains(&format!("\"sha256\":\"{hash}\"")),
            "hash as hex: {line}"
        );
        assert!(!line.contains('\n'));
    }

    #[test]
    fn the_commit_hook_runs_after_the_commit_record_and_before_clearing() {
        let (fs, store, steps) = scenario();
        let seen = RefCell::new(Vec::new());
        run_with(&fs, &store, 1, &steps, serde_json::json!({"k": 1}), || {
            seen.borrow_mut()
                .extend(store.lines.borrow().iter().cloned());
            Ok(())
        })
        .expect("applied");
        let lines = seen.borrow();
        assert!(
            lines
                .last()
                .expect("records")
                .contains("\"rec\":\"commit\""),
            "{lines:?}"
        );
        assert!(
            store.read_all().expect("read").is_empty(),
            "cleared after the hook"
        );
    }

    #[test]
    fn a_failing_commit_hook_leaves_the_result_for_recovery() {
        let (fs, store, steps) = scenario();
        let meta = serde_json::json!({"layout": [[1, "b"], [2, "a"]]});
        let err = run_with(&fs, &store, 1, &steps, meta.clone(), || {
            Err(io::Error::other("disk full"))
        })
        .expect_err("hook failed");
        assert!(matches!(err, TxError::Cleanup(_)), "{err:?}");
        // The game-visible files are in place; the trashed file waits for the cleanup that recovery finishes.
        assert_eq!(
            fs.read(r"D:\addons\pak01_dir.vpk").as_deref(),
            Some(&b"mod B"[..])
        );
        assert_eq!(
            fs.read(r"D:\addons\pak02_dir.vpk").as_deref(),
            Some(&b"mod A"[..])
        );
        let r = recover_with_meta(&fs, &store).expect("recovery");
        assert_eq!(r, (Recovery::Completed, Some(meta)));
        assert_eq!(fs.snapshot(), after_state());
        assert!(store.read_all().expect("read").is_empty());
    }

    #[test]
    fn recovery_hands_back_the_result_when_it_goes_forward() {
        let meta = serde_json::json!({"layout": "after"});
        for cut in 0..40 {
            let (fs, store, steps) = scenario();
            fs.power().cut_after(cut);
            let completed = run_with(&fs, &store, 7, &steps, meta.clone(), || Ok(())).is_ok();
            fs.power().restore();
            let (outcome, got) = recover_with_meta(&fs, &store).expect("recovery");
            match outcome {
                Recovery::RolledForward | Recovery::Completed => {
                    assert_eq!(got.as_ref(), Some(&meta), "cut {cut}");
                    assert_eq!(fs.snapshot(), after_state(), "cut {cut}");
                }
                Recovery::RolledBack => assert_eq!(got, None, "cut {cut}"),
                Recovery::Clean => assert!(completed || got.is_none(), "cut {cut}"),
            }
        }
    }

    #[test]
    fn the_file_journal_appends_reads_and_clears() {
        let dir = std::env::temp_dir().join(format!("deadloader-journal-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let journal = FileJournal::new(dir.join("journal.jsonl"));
        journal.append("one").expect("append");
        journal.append("two").expect("append");
        assert_eq!(journal.read_all().expect("read"), "one\ntwo\n");
        journal.clear().expect("clear");
        assert_eq!(journal.read_all().expect("read"), "");
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    #[test]
    fn a_missing_journal_file_reads_as_empty() {
        let journal =
            FileJournal::new(std::env::temp_dir().join("deadloader-no-such-journal.jsonl"));
        assert_eq!(journal.read_all().expect("read"), "");
    }
}
