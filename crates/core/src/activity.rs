//! Reading the logs for the Activity screen (S7): pages of entries newest first, one entry in full, the totals of each
//! tab and today's summary.
//!
//! The logs can reach the size cap (100 MB by default), so nothing here reads a whole file at once. A page is read
//! backwards from the end of the newest day's file, a part at a time, and stops when it is full; the next page goes on
//! from the [`Cursor`] it returned. The totals are kept per file in [`Tallies`] and only what was appended since the
//! last time is read again: log files only ever grow, and only today's file does.
//!
//! The network log has one line per hop and attempt (ADR 0009); the screen shows one row per request, so the lines
//! of one request (`req`) become one [`Entry::Network`]. A request's first line is hop 0, attempt 1; reading
//! backwards, a request is complete once that line is reached. Requests run side by side, so a request may begin
//! before the point where a page ends: its older lines are then skipped on the next page (the cursor names them).
//!
//! What is returned is masked again with the user-path masker when masking is on (decision 94), so lines written while
//! it was off don't show the Windows user name either.

use crate::log::{
    AppEvent, FileEvent, FileOp, HostSource, Kind, Level, LogDir, LogId, Masker, NetworkEvent,
    Outcome, Purpose, RETRYING_MSG, Record, log_files,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::io;
use time::{Date, Duration, Month, OffsetDateTime, Time, UtcOffset};

/// Entries per page unless the UI asks for another number.
pub const DEFAULT_LIMIT: usize = 100;
/// The most entries one page returns.
pub const MAX_LIMIT: usize = 500;
/// Bytes read from the file at a time.
const CHUNK: u64 = 64 * 1024;
/// A page stops after reading this much and returns a cursor, so a search that matches little never blocks for long.
const SCAN_BUDGET: u64 = 32 << 20;
/// A request whose first line hasn't turned up within this much older log is shown with the lines found: its start
/// was lost (rotation, a damaged line).
const COMPLETION_WINDOW: u64 = 4 << 20;
/// Requests a cursor remembers to skip.
const SKIP_MAX: usize = 512;

/// The time range of the list and of the totals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Range {
    #[serde(rename = "hour")]
    Hour,
    #[default]
    #[serde(rename = "today")]
    Today,
    #[serde(rename = "7d")]
    Week,
    #[serde(rename = "30d")]
    Month,
}

impl Range {
    /// The days whose files the range covers, newest first.
    fn dates(self, now: OffsetDateTime) -> Vec<Date> {
        let today = now.date();
        let back = |days: i64| {
            (0..days)
                .filter_map(|d| today.checked_sub(Duration::days(d)))
                .collect()
        };
        match self {
            Range::Hour => {
                let start = (now - Duration::HOUR).date();
                if start == today {
                    vec![today]
                } else {
                    vec![today, start]
                }
            }
            Range::Today => vec![today],
            Range::Week => back(7),
            Range::Month => back(30),
        }
    }

    /// The oldest moment the range includes, in Unix milliseconds; `None` when whole days are meant.
    fn since_ms(self, now: OffsetDateTime) -> Option<i128> {
        match self {
            Range::Hour => Some(unix_ms(now - Duration::HOUR)),
            Range::Today | Range::Week | Range::Month => None,
        }
    }
}

/// Which outcomes the list shows (network and files tabs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Results {
    #[default]
    All,
    Ok,
    Error,
}

/// What the list shows. Filters that don't belong to `kind` are ignored.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Query {
    pub kind: Kind,
    #[serde(default)]
    pub range: Range,
    /// Network.
    #[serde(default)]
    pub purpose: Option<Purpose>,
    /// Files.
    #[serde(default)]
    pub op: Option<FileOp>,
    /// App.
    #[serde(default)]
    pub level: Option<Level>,
    /// Network and files.
    #[serde(default)]
    pub result: Results,
    /// Network: requests with a hop to this host.
    #[serde(default)]
    pub host: Option<String>,
    /// Found in any field, ignoring case.
    #[serde(default)]
    pub text: String,
    /// Where the previous page ended; `None` for the newest entries.
    #[serde(default)]
    pub cursor: Option<Cursor>,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    DEFAULT_LIMIT
}

/// Where a page ended: the entries before `offset` in `file` (and in older files of the range) come next. The UI
/// hands it back as it got it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    pub file: String,
    pub offset: u64,
    /// Requests already shown whose older lines come after the cursor.
    #[serde(default)]
    pub skip: Vec<String>,
}

/// One row of the list, with its full record.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Entry {
    /// One request: every hop and retry, oldest first.
    Network {
        req: LogId,
        lines: Vec<Record<NetworkEvent>>,
    },
    Files {
        record: Record<FileEvent>,
    },
    App {
        record: Record<AppEvent>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Page {
    pub entries: Vec<Entry>,
    /// `None` when there is nothing older in the range.
    pub cursor: Option<Cursor>,
}

/// Reads the logs in `dir` as of `now`, masking with `masker`.
#[derive(Debug)]
pub struct Reader<'a, D> {
    pub dir: &'a D,
    pub now: OffsetDateTime,
    pub masker: &'a Masker,
}

impl<D: LogDir> Reader<'_, D> {
    /// A page of entries, newest first. `InvalidInput` when the cursor doesn't belong to the query.
    pub fn page(&self, query: &Query) -> io::Result<Page> {
        match query.kind {
            Kind::Network => self.scan::<NetworkEvent>(query),
            Kind::Files => self.scan::<FileEvent>(query),
            Kind::App => self.scan::<AppEvent>(query),
        }
    }

    /// The entry with this line id, or the request with this request id; for a network line, its whole request.
    pub fn entry(&self, id: &str) -> io::Result<Option<Entry>> {
        if id.is_empty()
            || !id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        {
            return Ok(None);
        }
        // Lines of a run are in the files of the day it started and later; a day earlier covers a clock change.
        let from = run_date(id).and_then(|d| d.previous_day());
        let mut files = log_files(self.dir)?;
        files.retain(|f| from.is_none_or(|from| f.date >= from));
        files.sort_by_key(|f| std::cmp::Reverse(f.date));
        let wanted = |text: &str, key: &str| text.contains(&format!("\"{key}\":\"{id}\""));
        for file in &files {
            let mut found: Option<Entry> = None;
            let mut req: Option<String> = None;
            forward(self.dir, &file.name, 0, |bytes| {
                let text = String::from_utf8_lossy(bytes);
                if !(wanted(&text, "id") || (file.kind == Kind::Network && wanted(&text, "req"))) {
                    return true;
                }
                match file.kind {
                    Kind::Network => {
                        req = self
                            .parse::<NetworkEvent>(bytes)
                            .map(|record| record.body.req.as_str().to_owned());
                    }
                    Kind::Files => found = self.parse(bytes).map(|record| Entry::Files { record }),
                    Kind::App => found = self.parse(bytes).map(|record| Entry::App { record }),
                }
                found.is_none() && req.is_none()
            })?;
            if let Some(req) = req {
                // Every line of the request, also the ones before the line asked for.
                let mut lines: Vec<Record<NetworkEvent>> = Vec::new();
                let needle = format!("\"req\":\"{req}\"");
                forward(self.dir, &file.name, 0, |bytes| {
                    if String::from_utf8_lossy(bytes).contains(&needle)
                        && let Some(record) = self.parse::<NetworkEvent>(bytes)
                    {
                        lines.push(record);
                    }
                    true
                })?;
                return Ok(Some(Entry::Network {
                    req: LogId(req),
                    lines,
                }));
            }
            if found.is_some() {
                return Ok(found);
            }
        }
        Ok(None)
    }

    /// One line, masked, as a record; `None` for a damaged line.
    fn parse<T: DeserializeOwned>(&self, bytes: &[u8]) -> Option<Record<T>> {
        let text = std::str::from_utf8(bytes).ok()?;
        serde_json::from_str(&self.masker.mask(text)).ok()
    }

    fn scan<T: Line>(&self, query: &Query) -> io::Result<Page> {
        let limit = query.limit.clamp(1, MAX_LIMIT);
        let names: Vec<String> = query
            .range
            .dates(self.now)
            .into_iter()
            .map(|date| T::KIND.file_name(date))
            .collect();
        let since = query.range.since_ms(self.now);
        let needle = query.text.trim().to_lowercase();
        let (first, mut end, mut skip) = match &query.cursor {
            Some(cursor) => {
                let index = names
                    .iter()
                    .position(|n| *n == cursor.file)
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "the cursor belongs to another list",
                        )
                    })?;
                (
                    index,
                    Some(cursor.offset),
                    cursor.skip.iter().cloned().collect::<HashSet<_>>(),
                )
            }
            None => (0, None, HashSet::new()),
        };

        let mut groups: VecDeque<Group<T>> = VecDeque::new();
        // Key → sequence number of its group while it waits for its first line.
        let mut open: HashMap<String, u64> = HashMap::new();
        let mut popped: u64 = 0;
        let mut finished: Vec<(String, Pos)> = Vec::new();
        let mut entries = Vec::new();
        let mut scanned: u64 = 0;
        let mut closed = false;

        for (index, name) in names.iter().enumerate().skip(first) {
            let end_here = match end.take() {
                Some(offset) => offset,
                None => match self.dir.file_len(name) {
                    Ok(len) => len,
                    Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                    Err(e) => return Err(e),
                },
            };
            let mut back = Backward::new(self.dir, name, end_here);
            loop {
                // A page that resumes at a line starts after its end; one that resumes after it, at its start.
                let line_end = back.position();
                let Some((start, bytes)) = back.prev()? else {
                    break;
                };
                scanned += bytes.len() as u64 + 1;
                let here = Pos {
                    file: index,
                    offset: start,
                };
                if let Some(record) = self.parse::<T>(&bytes) {
                    let key = T::key(&record).to_owned();
                    let starts = T::starts(&record);
                    if skip.contains(&key) {
                        if starts {
                            skip.remove(&key);
                        }
                    } else if let Some(&seq) = open.get(&key) {
                        if let Some(group) =
                            groups.get_mut(usize::try_from(seq - popped).unwrap_or(usize::MAX))
                        {
                            group.lines.push(record);
                            group.oldest = here;
                            if starts {
                                group.complete = true;
                                open.remove(&key);
                            }
                        }
                    } else if !closed {
                        if since.is_some_and(|since| {
                            parse_ts_ms(&record.ts).is_some_and(|ms| ms < since)
                        }) {
                            closed = true;
                        } else {
                            let seq = popped + groups.len() as u64;
                            if !starts {
                                open.insert(key.clone(), seq);
                            }
                            groups.push_back(Group {
                                key,
                                lines: vec![record],
                                created: Pos {
                                    file: index,
                                    offset: line_end,
                                },
                                oldest: here,
                                created_at: scanned,
                                complete: starts,
                            });
                        }
                    }
                }
                // A request whose start doesn't turn up is shown with what was found.
                while let Some(front) = groups.front_mut() {
                    if front.complete || scanned - front.created_at <= COMPLETION_WINDOW {
                        break;
                    }
                    front.complete = true;
                    open.remove(&front.key);
                }
                if emit(
                    &mut groups,
                    &mut open,
                    &mut popped,
                    &mut finished,
                    &mut entries,
                    query,
                    &needle,
                    limit,
                ) {
                    return Ok(Page {
                        entries,
                        cursor: Some(cursor_at(&names, &groups, here, &finished, skip)),
                    });
                }
                if closed && groups.is_empty() {
                    return Ok(Page {
                        entries,
                        cursor: None,
                    });
                }
                if scanned > SCAN_BUDGET {
                    let position = Pos {
                        file: index,
                        offset: back.position(),
                    };
                    return Ok(Page {
                        entries,
                        cursor: Some(cursor_at(&names, &groups, position, &finished, skip)),
                    });
                }
            }
        }
        // Nothing older: what is still open is shown as it is.
        for group in &mut groups {
            group.complete = true;
        }
        open.clear();
        if emit(
            &mut groups,
            &mut open,
            &mut popped,
            &mut finished,
            &mut entries,
            query,
            &needle,
            limit,
        ) && !groups.is_empty()
        {
            let last = Pos {
                file: names.len().saturating_sub(1),
                offset: 0,
            };
            return Ok(Page {
                entries,
                cursor: Some(cursor_at(&names, &groups, last, &finished, skip)),
            });
        }
        Ok(Page {
            entries,
            cursor: None,
        })
    }
}

/// A place in the range's files: a lower `file` is a newer day; within a file a higher offset is newer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    file: usize,
    offset: u64,
}

impl Pos {
    fn older_than(self, other: Pos) -> bool {
        self.file > other.file || (self.file == other.file && self.offset < other.offset)
    }
}

/// The lines of one entry found so far, newest first.
#[derive(Debug)]
struct Group<T> {
    key: String,
    lines: Vec<Record<T>>,
    /// Where its newest line is: the next page starts there when this group isn't shown yet.
    created: Pos,
    oldest: Pos,
    /// How much had been read when it was created.
    created_at: u64,
    complete: bool,
}

/// Moves complete groups at the front into the page; true once the page is full.
#[allow(clippy::too_many_arguments)]
fn emit<T: Line>(
    groups: &mut VecDeque<Group<T>>,
    open: &mut HashMap<String, u64>,
    popped: &mut u64,
    finished: &mut Vec<(String, Pos)>,
    entries: &mut Vec<Entry>,
    query: &Query,
    needle: &str,
    limit: usize,
) -> bool {
    if entries.len() >= limit {
        return true;
    }
    while groups.front().is_some_and(|g| g.complete) {
        let Some(mut group) = groups.pop_front() else {
            break;
        };
        *popped += 1;
        open.remove(&group.key);
        finished.push((group.key.clone(), group.oldest));
        group.lines.reverse();
        if T::matches(&group.lines, query)
            && (needle.is_empty() || text_matches(&group.lines, needle))
        {
            entries.push(T::entry(group.key, group.lines));
            if entries.len() >= limit {
                return true;
            }
        }
    }
    false
}

/// The cursor for a page that stops at `position`: at the first group not shown yet, else at `position`.
fn cursor_at<T>(
    names: &[String],
    groups: &VecDeque<Group<T>>,
    position: Pos,
    finished: &[(String, Pos)],
    carried: HashSet<String>,
) -> Cursor {
    let at = groups.front().map_or(position, |g| g.created);
    let mut skip: Vec<String> = carried.into_iter().collect();
    // Shown requests with lines the next page will read again.
    skip.extend(
        finished
            .iter()
            .filter(|(_, oldest)| oldest.older_than(at))
            .map(|(key, _)| key.clone()),
    );
    skip.sort();
    skip.dedup();
    skip.truncate(SKIP_MAX);
    Cursor {
        file: names.get(at.file).cloned().unwrap_or_default(),
        offset: at.offset,
        skip,
    }
}

/// What the page reader needs to know about the lines of each log.
trait Line: DeserializeOwned + Serialize + Sized {
    const KIND: Kind;
    /// Lines with the same key form one entry.
    fn key(record: &Record<Self>) -> &str;
    /// The first line of its entry.
    fn starts(record: &Record<Self>) -> bool;
    /// The query's filters for this log (not the range or the text); `lines` oldest first.
    fn matches(lines: &[Record<Self>], query: &Query) -> bool;
    fn entry(key: String, lines: Vec<Record<Self>>) -> Entry;
}

impl Line for NetworkEvent {
    const KIND: Kind = Kind::Network;

    fn key(record: &Record<Self>) -> &str {
        record.body.req.as_str()
    }

    fn starts(record: &Record<Self>) -> bool {
        record.body.hop == 0 && record.body.attempt <= 1
    }

    fn matches(lines: &[Record<Self>], query: &Query) -> bool {
        let Some(first) = lines.first() else {
            return false;
        };
        let failed = lines.last().is_some_and(|last| fails(&last.body));
        query.purpose.is_none_or(|p| first.body.purpose == p)
            && query.host.as_deref().is_none_or(|host| {
                let host = host.trim();
                lines
                    .iter()
                    .any(|l| host_of(&l.body.url).eq_ignore_ascii_case(host))
            })
            && match query.result {
                Results::All => true,
                Results::Ok => !failed,
                Results::Error => failed,
            }
    }

    fn entry(key: String, lines: Vec<Record<Self>>) -> Entry {
        Entry::Network {
            req: LogId(key),
            lines,
        }
    }
}

impl Line for FileEvent {
    const KIND: Kind = Kind::Files;

    fn key(record: &Record<Self>) -> &str {
        record.id.as_str()
    }

    fn starts(_: &Record<Self>) -> bool {
        true
    }

    fn matches(lines: &[Record<Self>], query: &Query) -> bool {
        lines.iter().all(|l| {
            query.op.is_none_or(|op| l.body.op == op)
                && match query.result {
                    Results::All => true,
                    Results::Ok => l.body.result == Outcome::Ok,
                    Results::Error => l.body.result == Outcome::Failed,
                }
        })
    }

    fn entry(_: String, mut lines: Vec<Record<Self>>) -> Entry {
        let record = lines.swap_remove(0);
        Entry::Files { record }
    }
}

impl Line for AppEvent {
    const KIND: Kind = Kind::App;

    fn key(record: &Record<Self>) -> &str {
        record.id.as_str()
    }

    fn starts(_: &Record<Self>) -> bool {
        true
    }

    fn matches(lines: &[Record<Self>], query: &Query) -> bool {
        lines
            .iter()
            .all(|l| query.level.is_none_or(|lvl| l.lvl == lvl))
    }

    fn entry(_: String, mut lines: Vec<Record<Self>>) -> Entry {
        let record = lines.swap_remove(0);
        Entry::App { record }
    }
}

/// Whether a network line is a failure: refused, not answered, or answered with an error status.
fn fails(event: &NetworkEvent) -> bool {
    event.blocked.is_some() || event.error.is_some() || event.status.is_some_and(|s| s >= 400)
}

/// Whether any text or number of the records contains `needle` (already lowercase).
fn text_matches<T: Serialize>(lines: &[Record<T>], needle: &str) -> bool {
    fn walk(value: &serde_json::Value, needle: &str) -> bool {
        match value {
            serde_json::Value::String(s) => s.to_lowercase().contains(needle),
            serde_json::Value::Number(n) => n.to_string().contains(needle),
            serde_json::Value::Array(items) => items.iter().any(|v| walk(v, needle)),
            serde_json::Value::Object(map) => map.values().any(|v| walk(v, needle)),
            serde_json::Value::Null | serde_json::Value::Bool(_) => false,
        }
    }
    lines
        .iter()
        .any(|l| serde_json::to_value(l).is_ok_and(|v| walk(&v, needle)))
}

/// The host of an `https://host[:port]/…` address, lowercase; empty when there is none.
pub fn host_of(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
    let host = authority
        .rsplit_once('@')
        .map_or(authority, |(_, host)| host);
    let host = if host.starts_with('[') {
        host.split_once(']')
            .map_or(host, |(h, _)| h)
            .trim_start_matches('[')
    } else {
        host.split(':').next().unwrap_or_default()
    };
    host.to_ascii_lowercase()
}

/// Reads one file's lines backwards from an offset, a chunk at a time.
struct Backward<'a, D> {
    dir: &'a D,
    name: &'a str,
    /// The unread bytes are `[base, base + buf.len())`.
    base: u64,
    buf: Vec<u8>,
}

impl<'a, D: LogDir> Backward<'a, D> {
    fn new(dir: &'a D, name: &'a str, end: u64) -> Self {
        Self {
            dir,
            name,
            base: end,
            buf: Vec::new(),
        }
    }

    /// Where the unread part ends: every line before it is still to come.
    fn position(&self) -> u64 {
        self.base + self.buf.len() as u64
    }

    /// The previous non-blank line and where it starts, without its line break.
    fn prev(&mut self) -> io::Result<Option<(u64, Vec<u8>)>> {
        loop {
            if self.buf.is_empty() && self.base == 0 {
                return Ok(None);
            }
            let mut line_end = self.buf.len();
            if self.buf.last() == Some(&b'\n') {
                line_end -= 1;
            }
            let found = self.buf[..line_end].iter().rposition(|&b| b == b'\n');
            let from = match found {
                Some(newline) => newline + 1,
                None if self.base == 0 => 0,
                None => {
                    self.load()?;
                    continue;
                }
            };
            let mut line = self.buf[from..line_end].to_vec();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            let start = self.base + from as u64;
            self.buf.truncate(from);
            if line.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            return Ok(Some((start, line)));
        }
    }

    /// Prepends the chunk before the unread part.
    fn load(&mut self) -> io::Result<()> {
        let from = self.base.saturating_sub(CHUNK);
        let mut more = self.dir.read_range(self.name, from, self.base - from)?;
        if more.len() as u64 != self.base - from {
            // The file shrank under us (deleted by rotation): what is left can't be placed, so stop here.
            self.base = 0;
            self.buf.clear();
            return Ok(());
        }
        more.extend_from_slice(&self.buf);
        self.buf = more;
        self.base = from;
        Ok(())
    }
}

/// Calls `each` with every complete line of `name` from `from` on, until it returns false. Returns where the last
/// complete line ends: a line still being written (or torn by a crash) is left for later.
fn forward(
    dir: &impl LogDir,
    name: &str,
    from: u64,
    mut each: impl FnMut(&[u8]) -> bool,
) -> io::Result<u64> {
    const FORWARD_CHUNK: u64 = 1 << 20;
    let len = match dir.file_len(name) {
        Ok(len) => len,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(from),
        Err(e) => return Err(e),
    };
    let mut done = from;
    let mut carry: Vec<u8> = Vec::new();
    let mut at = from;
    while at < len {
        let chunk = dir.read_range(name, at, FORWARD_CHUNK.min(len - at))?;
        if chunk.is_empty() {
            break;
        }
        at += chunk.len() as u64;
        carry.extend_from_slice(&chunk);
        let mut start = 0;
        while let Some(newline) = carry[start..].iter().position(|&b| b == b'\n') {
            let mut line = &carry[start..start + newline];
            if line.last() == Some(&b'\r') {
                line = &line[..line.len() - 1];
            }
            start += newline + 1;
            done += newline as u64 + 1;
            if !line.iter().all(u8::is_ascii_whitespace) && !each(line) {
                return Ok(done);
            }
        }
        carry.drain(..start);
    }
    Ok(done)
}

// --- totals and the summary ---------------------------------------------------------------------------------------

/// The totals of the network tab.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct NetworkCounts {
    pub requests: u64,
    /// Requests whose last word was a failure.
    pub failed: u64,
    pub purposes: BTreeMap<Purpose, u64>,
    /// Lines (hops and tries) per host, refused ones included: the host filter's numbers.
    pub hosts: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct FilesCounts {
    pub operations: u64,
    pub failed: u64,
    pub ops: BTreeMap<FileOp, u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct AppCounts {
    pub events: u64,
    pub errors: u64,
    pub levels: BTreeMap<Level, u64>,
}

/// Each tab's totals over a range, independent of the other filters.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Counts {
    pub network: NetworkCounts,
    pub files: FilesCounts,
    pub app: AppCounts,
}

/// A host deadloader connected to today.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HostCount {
    pub host: String,
    pub requests: u64,
    /// Why it is allowed (the latest line's).
    pub source: Option<HostSource>,
}

/// A host requests were refused for, never contacted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BlockedHost {
    pub host: String,
    pub requests: u64,
    /// The latest such request, for "Show entry".
    pub last: LogId,
}

/// What deadloader changed today in game folders (`…\game\citadel\…`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct GameChanges {
    /// Successful operations.
    pub changes: u64,
    /// By operation, `gameinfo.gi` left out.
    pub ops: BTreeMap<FileOp, u64>,
    /// Changes of `gameinfo.gi`.
    pub gameinfo: u64,
    pub failed: u64,
    /// Changes (journal transactions) the operations belonged to.
    pub batches: u64,
    /// The addons folders, masked when masking is on.
    pub folders: Vec<String>,
}

/// Today at a glance.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Summary {
    /// Built-in hosts first, then GitHub, user-added and image hosts; the busiest first within each.
    pub contacted: Vec<HostCount>,
    pub blocked: Vec<BlockedHost>,
    pub game: GameChanges,
    /// Nothing in the network or files log at all: a fresh install (the app log always has the start).
    pub empty: bool,
}

/// What one file adds to the totals.
#[derive(Debug, Clone, Default)]
struct Tally {
    network: NetworkCounts,
    contacted: BTreeMap<String, (u64, Option<HostSource>)>,
    blocked: BTreeMap<String, (u64, String)>,
    files: FilesCounts,
    game: GameTally,
    app: AppCounts,
}

#[derive(Debug, Clone, Default)]
struct GameTally {
    changes: u64,
    ops: BTreeMap<FileOp, u64>,
    gameinfo: u64,
    failed: u64,
    txns: BTreeSet<u64>,
    folders: BTreeSet<String>,
}

/// The fields of a line the totals need, from any of the three logs.
#[derive(Debug, Deserialize)]
struct Light {
    ts: String,
    lvl: Level,
    msg: String,
    #[serde(default)]
    req: Option<String>,
    #[serde(default)]
    hop: Option<u32>,
    #[serde(default)]
    attempt: Option<u32>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    host_source: Option<HostSource>,
    #[serde(default)]
    purpose: Option<Purpose>,
    #[serde(default)]
    status: Option<u16>,
    #[serde(default)]
    error: Option<serde::de::IgnoredAny>,
    #[serde(default)]
    blocked: Option<serde::de::IgnoredAny>,
    #[serde(default)]
    op: Option<FileOp>,
    #[serde(default)]
    src: Option<String>,
    #[serde(default)]
    dst: Option<String>,
    #[serde(default)]
    txn: Option<u64>,
    #[serde(default)]
    result: Option<Outcome>,
}

impl Tally {
    fn add(&mut self, kind: Kind, line: &Light) {
        match kind {
            Kind::Network => self.add_network(line),
            Kind::Files => self.add_file(line),
            Kind::App => {
                self.app.events += 1;
                if line.lvl == Level::Error {
                    self.app.errors += 1;
                }
                *self.app.levels.entry(line.lvl).or_default() += 1;
            }
        }
    }

    fn add_network(&mut self, line: &Light) {
        let host = line.url.as_deref().map(host_of).unwrap_or_default();
        if line.hop == Some(0) && line.attempt.unwrap_or(1) <= 1 {
            self.network.requests += 1;
            if let Some(purpose) = line.purpose {
                *self.network.purposes.entry(purpose).or_default() += 1;
            }
        }
        let failed =
            line.blocked.is_some() || line.error.is_some() || line.status.is_some_and(|s| s >= 400);
        if failed && line.msg != RETRYING_MSG {
            self.network.failed += 1;
        }
        if host.is_empty() {
            return;
        }
        *self.network.hosts.entry(host.clone()).or_default() += 1;
        if line.blocked.is_some() {
            let entry = self.blocked.entry(host).or_insert((0, String::new()));
            entry.0 += 1;
            entry.1 = line.req.clone().unwrap_or_default();
        } else {
            let entry = self.contacted.entry(host).or_insert((0, None));
            entry.0 += 1;
            if line.host_source.is_some() {
                entry.1 = line.host_source;
            }
        }
    }

    fn add_file(&mut self, line: &Light) {
        let Some(op) = line.op else {
            return;
        };
        let ok = line.result != Some(Outcome::Failed);
        self.files.operations += 1;
        if !ok {
            self.files.failed += 1;
        }
        *self.files.ops.entry(op).or_default() += 1;
        // The file that changed: what was deleted, otherwise what was written.
        let changed = match op {
            FileOp::Delete => line.src.as_deref(),
            _ => line.dst.as_deref().or(line.src.as_deref()),
        };
        let Some((folder, inner)) = changed.and_then(game_folder) else {
            return;
        };
        if !ok {
            self.game.failed += 1;
            return;
        }
        self.game.changes += 1;
        if inner.eq_ignore_ascii_case("gameinfo.gi") {
            self.game.gameinfo += 1;
        } else {
            *self.game.ops.entry(op).or_default() += 1;
        }
        if let Some(txn) = line.txn {
            self.game.txns.insert(txn);
        }
        self.game.folders.insert(folder);
    }

    fn merge(&mut self, other: &Tally) {
        let net = &mut self.network;
        net.requests += other.network.requests;
        net.failed += other.network.failed;
        add_all(&mut net.purposes, &other.network.purposes);
        add_all(&mut net.hosts, &other.network.hosts);
        self.files.operations += other.files.operations;
        self.files.failed += other.files.failed;
        add_all(&mut self.files.ops, &other.files.ops);
        self.app.events += other.app.events;
        self.app.errors += other.app.errors;
        add_all(&mut self.app.levels, &other.app.levels);
    }
}

fn add_all<K: Ord + Clone>(into: &mut BTreeMap<K, u64>, from: &BTreeMap<K, u64>) {
    for (key, n) in from {
        *into.entry(key.clone()).or_default() += n;
    }
}

/// For a path inside a game's `game\citadel` folder: the game's addons folder, and the rest of the path after
/// `citadel\` (e.g. `gameinfo.gi`, `addons\pak01_dir.vpk`).
fn game_folder(path: &str) -> Option<(String, &str)> {
    const MARK: &str = "game\\citadel\\";
    let normal = path.replace('/', "\\");
    let lower = normal.to_ascii_lowercase();
    let at = lower.find(&format!("\\{MARK}"))? + 1;
    let inner = path.get(at + MARK.len()..)?;
    let folder = format!("{}{MARK}addons", &normal[..at]);
    Some((folder, inner))
}

/// The totals per file, kept between calls: files only grow, so only their new lines are read again.
#[derive(Debug, Default)]
pub struct Tallies {
    files: HashMap<String, (u64, Tally)>,
}

impl Tallies {
    /// Each tab's totals over `range`.
    pub fn counts(
        &mut self,
        dir: &impl LogDir,
        now: OffsetDateTime,
        range: Range,
    ) -> io::Result<Counts> {
        let mut total = Tally::default();
        for date in range.dates(now) {
            for kind in Kind::ALL {
                let name = kind.file_name(date);
                match range.since_ms(now) {
                    // The last hour isn't a whole file: counted afresh, never kept.
                    Some(since) => {
                        let mut tally = Tally::default();
                        forward(dir, &name, 0, |bytes| {
                            if let Some(line) = light(bytes)
                                && parse_ts_ms(&line.ts).is_some_and(|ms| ms >= since)
                            {
                                tally.add(kind, &line);
                            }
                            true
                        })?;
                        total.merge(&tally);
                    }
                    None => total.merge(self.tally(dir, kind, &name)?),
                }
            }
        }
        Ok(Counts {
            network: total.network,
            files: total.files,
            app: total.app,
        })
    }

    /// Today at a glance; folders masked with `masker`.
    pub fn summary(
        &mut self,
        dir: &impl LogDir,
        now: OffsetDateTime,
        masker: &Masker,
    ) -> io::Result<Summary> {
        let today = now.date();
        let net = self
            .tally(dir, Kind::Network, &Kind::Network.file_name(today))?
            .clone();
        let files = self
            .tally(dir, Kind::Files, &Kind::Files.file_name(today))?
            .clone();
        let rank = |source: Option<HostSource>| match source {
            Some(HostSource::Builtin) => 0,
            Some(HostSource::Github) => 1,
            Some(HostSource::User) => 2,
            Some(HostSource::UserImage) => 3,
            None => 4,
        };
        let mut contacted: Vec<HostCount> = net
            .contacted
            .into_iter()
            .map(|(host, (requests, source))| HostCount {
                host,
                requests,
                source,
            })
            .collect();
        contacted.sort_by(|a, b| {
            (rank(a.source), std::cmp::Reverse(a.requests), &a.host).cmp(&(
                rank(b.source),
                std::cmp::Reverse(b.requests),
                &b.host,
            ))
        });
        let mut blocked: Vec<BlockedHost> = net
            .blocked
            .into_iter()
            .map(|(host, (requests, last))| BlockedHost {
                host,
                requests,
                last: LogId(last),
            })
            .collect();
        blocked.sort_by(|a, b| {
            b.requests
                .cmp(&a.requests)
                .then_with(|| a.host.cmp(&b.host))
        });
        let game = files.game;
        let empty = !log_files(dir)?
            .iter()
            .any(|f| f.kind != Kind::App && f.bytes > 0);
        Ok(Summary {
            contacted,
            blocked,
            game: GameChanges {
                changes: game.changes,
                ops: game.ops,
                gameinfo: game.gameinfo,
                failed: game.failed,
                batches: game.txns.len() as u64,
                folders: game
                    .folders
                    .iter()
                    .map(|f| masker.mask(f).into_owned())
                    .collect(),
            },
            empty,
        })
    }

    /// The tally of one file, brought up to date.
    fn tally(&mut self, dir: &impl LogDir, kind: Kind, name: &str) -> io::Result<&Tally> {
        let (done, tally) = self.files.entry(name.to_owned()).or_default();
        let len = match dir.file_len(name) {
            Ok(len) => len,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                *done = 0;
                *tally = Tally::default();
                return Ok(tally);
            }
            Err(e) => return Err(e),
        };
        if len < *done {
            // Deleted and written anew.
            *done = 0;
            *tally = Tally::default();
        }
        if len > *done {
            *done = forward(dir, name, *done, |bytes| {
                if let Some(line) = light(bytes) {
                    tally.add(kind, &line);
                }
                true
            })?;
        }
        Ok(tally)
    }
}

fn light(bytes: &[u8]) -> Option<Light> {
    serde_json::from_slice(bytes).ok()
}

// --- times --------------------------------------------------------------------------------------------------------

fn unix_ms(t: OffsetDateTime) -> i128 {
    t.unix_timestamp_nanos() / 1_000_000
}

/// `2026-10-15T14:03:27.123+03:00` (the logs' own format) in Unix milliseconds.
fn parse_ts_ms(ts: &str) -> Option<i128> {
    let b = ts.as_bytes();
    if b.len() != 29 || b[10] != b'T' || b[19] != b'.' {
        return None;
    }
    let num = |from: usize, to: usize| -> Option<u32> {
        let digits = ts.get(from..to)?;
        digits
            .bytes()
            .all(|c| c.is_ascii_digit())
            .then(|| digits.parse().ok())?
    };
    let date = Date::from_calendar_date(
        i32::try_from(num(0, 4)?).ok()?,
        Month::try_from(u8::try_from(num(5, 7)?).ok()?).ok()?,
        u8::try_from(num(8, 10)?).ok()?,
    )
    .ok()?;
    let time = Time::from_hms_milli(
        u8::try_from(num(11, 13)?).ok()?,
        u8::try_from(num(14, 16)?).ok()?,
        u8::try_from(num(17, 19)?).ok()?,
        u16::try_from(num(20, 23)?).ok()?,
    )
    .ok()?;
    let sign: i8 = match b[23] {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };
    let offset = UtcOffset::from_hms(
        sign * i8::try_from(num(24, 26)?).ok()?,
        sign * i8::try_from(num(27, 29)?).ok()?,
        0,
    )
    .ok()?;
    Some(unix_ms(OffsetDateTime::new_in_offset(date, time, offset)))
}

/// The UTC day a log id's run started on (its prefix is the start time in milliseconds, base 36).
fn run_date(id: &str) -> Option<Date> {
    let run = id.split_once('-')?.0;
    let ms = u128::from_str_radix(run, 36).ok()?;
    let nanos = i128::try_from(ms.checked_mul(1_000_000)?).ok()?;
    OffsetDateTime::from_unix_timestamp_nanos(nanos)
        .ok()
        .map(OffsetDateTime::date)
}

#[cfg(test)]
#[path = "activity_tests.rs"]
mod tests;
