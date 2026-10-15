// What the Activity screen derives from the log entries the core returns: queries, row values, the message that
// explains a failure, the rows of the virtual list and merging live updates. Pure functions; texts come from i18n.

import type {
  FileOp,
  LogCursor,
  LogEntry,
  LogKind,
  LogLevel,
  LogQuery,
  LogRange,
  LogResults,
  LogSummary,
  NetworkLine,
  Purpose,
  UserMessage,
} from '../../lib/api';

export type Tab = LogKind;

export interface Filters {
  range: LogRange;
  purpose: Purpose | 'all';
  host: string | 'all';
  op: FileOp | 'all';
  level: LogLevel | 'all';
  result: LogResults;
}

export const DEFAULT_FILTERS: Filters = {
  range: 'today',
  purpose: 'all',
  host: 'all',
  op: 'all',
  level: 'all',
  result: 'all',
};

export const PAGE_SIZE = 100;
export const RANGES: readonly LogRange[] = ['hour', 'today', '7d', '30d'];
export const PURPOSES: readonly Purpose[] = ['catalog', 'thumbnail', 'description-image', 'download', 'update-check'];
export const OPS: readonly FileOp[] = ['hardlink', 'copy', 'rename', 'delete', 'write', 'backup', 'restore'];
export const LEVELS: readonly LogLevel[] = ['info', 'warn', 'error'];

const all = <T extends string>(value: T | 'all'): T | null => (value === 'all' ? null : value);

/** The core's query for a tab: only that tab's filters. */
export function queryFor(tab: Tab, filters: Filters, text: string, cursor: LogCursor | null, limit = PAGE_SIZE): LogQuery {
  return {
    kind: tab,
    range: filters.range,
    purpose: tab === 'network' ? all(filters.purpose) : null,
    host: tab === 'network' ? all(filters.host) : null,
    op: tab === 'files' ? all(filters.op) : null,
    level: tab === 'app' ? all(filters.level) : null,
    result: tab === 'app' ? 'all' : filters.result,
    text: text.trim(),
    cursor,
    limit,
  };
}

/** Whether a filter of this tab differs from the default ("Clear filters"). */
export function anyFilter(tab: Tab, f: Filters): boolean {
  return (
    f.range !== 'today' ||
    (tab === 'network' && (f.purpose !== 'all' || f.host !== 'all' || f.result !== 'all')) ||
    (tab === 'files' && (f.op !== 'all' || f.result !== 'all')) ||
    (tab === 'app' && f.level !== 'all')
  );
}

/** A row's id: the request id, or the line id. */
export function entryId(entry: LogEntry): string {
  return entry.kind === 'network' ? entry.req : entry.record.id;
}

export function tabOf(entry: LogEntry): Tab {
  return entry.kind;
}

/** When the entry began (a request: its first hop). */
export function startTs(entry: LogEntry): string {
  return entry.kind === 'network' ? (entry.lines[0]?.ts ?? '') : entry.record.ts;
}

/** Every line id of the entry. */
export function lineIds(entry: LogEntry): string[] {
  return entry.kind === 'network' ? entry.lines.map((l) => l.id) : [entry.record.id];
}

export function hostPath(url: string): { host: string; path: string } {
  const rest = url.includes('://') ? url.slice(url.indexOf('://') + 3) : url;
  const cut = rest.search(/[/?#]/);
  return cut < 0 ? { host: rest, path: '' } : { host: rest.slice(0, cut), path: rest.slice(cut) };
}

export type StatusKind = 'ok' | 'blocked' | 'failed' | 'error';

function lineStatus(line: NetworkLine | undefined): { kind: StatusKind; code: number | null } {
  if (!line) {
    return { kind: 'failed', code: null };
  }
  if (line.blocked !== undefined) {
    return { kind: 'blocked', code: null };
  }
  if (line.error !== undefined) {
    return { kind: 'failed', code: null };
  }
  const code = line.status ?? null;
  return { kind: code !== null && code >= 400 ? 'error' : 'ok', code };
}

/** A request's outcome: its last hop's. */
export function networkStatus(entry: LogEntry): { kind: StatusKind; code: number | null } {
  return entry.kind === 'network' ? lineStatus(entry.lines.at(-1)) : { kind: 'ok', code: null };
}

export function hopStatus(line: NetworkLine): { kind: StatusKind; code: number | null } {
  return lineStatus(line);
}

/** Bytes over all hops, `null` when no hop announced a size. */
export function sizeOf(entry: LogEntry): number | null {
  if (entry.kind !== 'network') {
    return entry.kind === 'files' ? (entry.record.bytes ?? null) : null;
  }
  const known = entry.lines.filter((l) => l.bytes !== undefined);
  return known.length === 0 ? null : known.reduce((sum, l) => sum + (l.bytes ?? 0), 0);
}

export function durationOf(entry: LogEntry): number {
  return entry.kind === 'network' ? entry.lines.reduce((sum, l) => sum + l.ms, 0) : 0;
}

/** Whether the entry is an error (the tabs' error counts, the row's colour). */
export function failed(entry: LogEntry): boolean {
  switch (entry.kind) {
    case 'network':
      return networkStatus(entry).kind !== 'ok';
    case 'files':
      return entry.record.result !== 'ok';
    case 'app':
      return entry.record.lvl === 'error';
  }
}

const BLOCK_REASONS: Record<string, string> = {
  'host-not-listed': 'not_listed',
  'host-wrong-purpose': 'not_listed',
  'not-https': 'not_https',
  'invalid-url': 'bad_redirect',
  'too-many-redirects': 'too_many_redirects',
};

/** The message (spec 04-messages) that explains why the entry failed; `null` when it didn't. */
export function entryProblem(entry: LogEntry): UserMessage | null {
  if (entry.kind === 'files') {
    const r = entry.record;
    if (r.result === 'ok') {
      return null;
    }
    const error = r.error ?? '';
    const reason = /os error 5\)/.test(error) ? 'access_denied' : /os error (32|33)\)/.test(error) ? 'in_use' : 'other';
    return { id: 'E18', args: { reason, detail: error } };
  }
  if (entry.kind !== 'network') {
    return null;
  }
  const last = entry.lines.at(-1);
  const first = entry.lines[0];
  if (!last || !first) {
    return null;
  }
  const host = hostPath(last.url).host;
  if (last.blocked !== undefined) {
    if (last.blocked === 'local-mode') {
      return { id: 'E6', args: { reason: 'local_mode' } };
    }
    return { id: 'E16', args: { reason: BLOCK_REASONS[last.blocked] ?? 'not_listed', host } };
  }
  if (last.error !== undefined) {
    const image = first.purpose === 'thumbnail' || first.purpose === 'description-image';
    return image
      ? { id: 'E19', args: { reason: 'no_response', detail: last.error } }
      : { id: 'E6', args: { reason: /timed? ?out/i.test(last.error) ? 'timeout' : 'other', detail: last.error } };
  }
  const status = last.status ?? 0;
  if (status === 429 || status >= 500) {
    return { id: 'E5', args: { status: String(status), host } };
  }
  if (status >= 400) {
    const gone = first.purpose === 'download' && [403, 404, 410].includes(status);
    return { id: gone ? 'E24' : 'E25', args: { status: String(status) } };
  }
  return null;
}

/** A row of the virtual list: an entry, or one hop of an open request. */
export type Row = { type: 'entry'; entry: LogEntry } | { type: 'hop'; entry: LogEntry & { kind: 'network' }; index: number };

/** Redirects a request followed (hops after the first) and retries of its hops. */
export function hopCounts(entry: LogEntry): { redirects: number; retries: number } {
  if (entry.kind !== 'network') {
    return { redirects: 0, retries: 0 };
  }
  const hops = new Set(entry.lines.map((l) => l.hop)).size;
  return { redirects: Math.max(0, hops - 1), retries: entry.lines.length - hops };
}

/** Whether the request has more than one line to show. */
export function hasHops(entry: LogEntry): entry is LogEntry & { kind: 'network' } {
  return entry.kind === 'network' && entry.lines.length > 1;
}

export function flatten(entries: readonly LogEntry[], open: ReadonlySet<string>): Row[] {
  const rows: Row[] = [];
  for (const entry of entries) {
    rows.push({ type: 'entry', entry });
    if (hasHops(entry) && open.has(entry.req)) {
      entry.lines.forEach((_, index) => rows.push({ type: 'hop', entry, index }));
    }
  }
  return rows;
}

export function rowKey(row: Row): string {
  return row.type === 'entry' ? entryId(row.entry) : `${row.entry.req}#${row.index}`;
}

/**
 * The newest page on top of what is shown. `added` counts entries not shown before; a request that got another hop is
 * replaced. Without any overlap the head stands alone (`joined` false): what lies between is unknown.
 */
export function mergeHead(
  current: readonly LogEntry[],
  head: readonly LogEntry[],
): { entries: LogEntry[]; added: number; joined: boolean } {
  const headIds = new Set(head.map(entryId));
  const shown = new Set(current.map(entryId));
  const added = head.filter((e) => !shown.has(entryId(e))).length;
  const joined = current.length === 0 || head.length === 0 || added < head.length;
  const rest = joined ? current.filter((e) => !headIds.has(entryId(e))) : [];
  return { entries: [...head, ...rest], added, joined };
}

/** `more` after `current`, without entries already shown. */
export function appendNew(current: readonly LogEntry[], more: readonly LogEntry[]): LogEntry[] {
  const shown = new Set(current.map(entryId));
  return [...current, ...more.filter((e) => !shown.has(entryId(e)))];
}

/** How many of `head` aren't shown yet. */
export function countNew(current: readonly LogEntry[], head: readonly LogEntry[]): number {
  const shown = new Set(current.map(entryId));
  return head.filter((e) => !shown.has(entryId(e))).length;
}

/** The entry as the log lines it came from, one JSON object per line. */
export function copyText(entry: LogEntry): string {
  const lines = entry.kind === 'network' ? entry.lines : [entry.record];
  return lines.map((l) => JSON.stringify(l)).join('\n');
}

/** The kinds of change in the game folder today, in the design's order (linked, copied, removed, renamed…), without the ones that didn't happen. */
export function gameParts(game: LogSummary['game']): [FileOp, number][] {
  const order: FileOp[] = ['hardlink', 'copy', 'delete', 'rename', 'write', 'backup', 'restore'];
  return order.map((op): [FileOp, number] => [op, game.ops[op] ?? 0]).filter(([, n]) => n > 0);
}

/** `D:\…\addons\pak01_dir.vpk` → head `D:\…\addons\`, tail `pak01_dir.vpk`. */
export function splitLast(path: string): { head: string; tail: string } {
  const at = Math.max(path.lastIndexOf('\\'), path.lastIndexOf('/'));
  return { head: path.slice(0, at + 1), tail: path.slice(at + 1) };
}

/** Now, as a log time (for "Paused at"). */
export function nowStamp(): string {
  return new Date().toISOString();
}
