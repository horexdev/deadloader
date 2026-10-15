// Invented logs for the mock core: a day of browsing the catalog, a download with redirects, a blocked redirect, a
// retried page, a timed-out image, applying profiles and the app's own events, plus a few older days. Shapes follow
// `lib/api.ts` (the core's `activity` module); the filters work like the core's.

import type {
  AppLine,
  FileLine,
  FileOp,
  HostSource,
  LogCounts,
  LogEntry,
  LogKind,
  LogPage,
  LogQuery,
  LogSummary,
  NetworkLine,
  Purpose,
} from '../lib/api';

export interface MockLogs {
  network: LogEntry[];
  files: LogEntry[];
  app: LogEntry[];
  next: number;
}

const GAME = String.raw`D:\SteamLibrary\steamapps\common\Deadlock\game\citadel`;
const DATA = String.raw`D:\Games\deadloader\deadloader-data`;
const RUN = 'mv9fkgpf';

/** RFC 3339 with the local offset and milliseconds, like the core writes. */
export function stamp(date: Date): string {
  const pad = (n: number, width = 2) => String(Math.trunc(Math.abs(n))).padStart(width, '0');
  const offset = -date.getTimezoneOffset();
  const sign = offset < 0 ? '-' : '+';
  return (
    `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}` +
    `T${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}.${pad(date.getMilliseconds(), 3)}` +
    `${sign}${pad(offset / 60)}:${pad(offset % 60)}`
  );
}

function id(n: number): string {
  return `${RUN}-${String(n).padStart(6, '0')}`;
}

interface Builder {
  logs: MockLogs;
  /** Minutes before now of the line being written. */
  at: Date;
}

function nextId(b: Builder): string {
  b.logs.next += 1;
  return id(b.logs.next);
}

function hop(
  b: Builder,
  req: string,
  n: number,
  url: string,
  fields: Partial<NetworkLine> & { purpose: Purpose },
): NetworkLine {
  const blocked = fields.blocked !== undefined;
  const failed = fields.error !== undefined;
  return {
    ts: stamp(b.at),
    id: nextId(b),
    lvl: failed ? 'error' : blocked ? 'warn' : 'info',
    msg: blocked ? 'request blocked' : failed ? 'request failed' : 'response',
    req,
    hop: n,
    method: 'GET',
    url,
    host_source: blocked ? undefined : 'builtin',
    initiator: 'auto',
    status: blocked || failed ? undefined : 200,
    bytes: blocked || failed ? undefined : 20_000,
    ms: blocked ? 0 : 95,
    ...fields,
  };
}

function request(b: Builder, lines: NetworkLine[]): void {
  b.logs.network.unshift({ kind: 'network', req: lines[0]?.req ?? '', lines });
}

function fileOp(b: Builder, op: FileOp, fields: Partial<FileLine>): void {
  const failed = fields.result === 'failed';
  b.logs.files.unshift({
    kind: 'files',
    record: {
      ts: stamp(b.at),
      id: nextId(b),
      lvl: failed ? 'error' : 'info',
      msg: failed ? 'file operation failed' : 'file operation',
      op,
      result: 'ok',
      ...fields,
    },
  });
}

function event(b: Builder, lvl: AppLine['lvl'], msg: string, fields?: Record<string, unknown>): void {
  b.logs.app.unshift({ kind: 'app', record: { ts: stamp(b.at), id: nextId(b), lvl, msg, fields } });
}

function later(b: Builder, ms: number): void {
  b.at = new Date(b.at.getTime() + ms);
}

const INDEX = '/apiv11/Mod/Index?_nPerpage=15&_aFilters[Generic_Game]=20948&_sSort=Generic_Newest&_nPage=';
const UPDATE = '?_csvProperties=_sName,_sVersion,_tsDateUpdated,_aFiles';

/** A catalog page and its thumbnails. */
function browse(b: Builder, page: number, by: 'auto' | 'user'): void {
  const req = nextId(b);
  request(b, [hop(b, req, 0, `https://gamebanana.com${INDEX}${page}`, { purpose: 'catalog', initiator: by, bytes: 41_000, ms: 310 })]);
  for (let i = 0; i < 15; i += 1) {
    later(b, 40);
    const thumb = nextId(b);
    const name = ((page * 15 + i) * 2654435761 >>> 0).toString(16).padStart(8, '0');
    request(b, [hop(b, thumb, 0, `https://images.gamebanana.com/img/ss/mods/220-90_${name}.jpg`, { purpose: 'thumbnail', bytes: 17_000 + i * 600, ms: 80 + i })]);
  }
}

/** Applying a profile: gameinfo.gi checked, files linked under temporary names, renamed into place, one removed. */
function apply(b: Builder, txn: number, mods: string[], removed: number | null): void {
  mods.forEach((mod, i) => {
    const slot = String(i + 3).padStart(2, '0');
    fileOp(b, 'hardlink', {
      src: String.raw`${DATA}\library\${mod}\1\0.vpk`,
      dst: String.raw`${GAME}\addons\pak${slot}_dir.vpk.${txn}.tmp`,
      bytes: 4_000_000 + i * 900_000,
      txn,
    });
    later(b, 30);
    fileOp(b, 'rename', {
      src: String.raw`${GAME}\addons\pak${slot}_dir.vpk.${txn}.tmp`,
      dst: String.raw`${GAME}\addons\pak${slot}_dir.vpk`,
      txn,
    });
    later(b, 30);
  });
  if (removed !== null) {
    const slot = String(removed).padStart(2, '0');
    fileOp(b, 'delete', { src: String.raw`${GAME}\addons\pak${slot}_dir.vpk`, bytes: 23_000_000, txn });
  }
}

/** A day of logs ending `now`; `local`: the catalog is off, so nothing went over the network. */
export function mockLogs(now: Date, scenario: string | null): MockLogs {
  const logs: MockLogs = { network: [], files: [], app: [], next: 0 };
  if (scenario === 'activity-empty') {
    return logs;
  }
  const local = scenario === 'activity-local';
  // The day starts 9 hours ago, or just after midnight when that would be yesterday.
  const midnight = new Date(now);
  midnight.setHours(0, 1, 0, 0);
  const b: Builder = { logs, at: new Date(Math.max(now.getTime() - 9 * 3600_000, midnight.getTime())) };

  // Two days ago: a short session.
  const old: Builder = { logs, at: new Date(now.getTime() - 50 * 3600_000) };
  event(old, 'info', 'deadloader started', { version: '0.1.0', data_folder: DATA });
  if (!local) {
    browse(old, 1, 'user');
  }
  apply(old, 1_791_800_000_000_000, ['haze-midnight-bloom'], null);
  // The older entries go below today's.
  const older = { network: logs.network, files: logs.files, app: logs.app };
  logs.network = [];
  logs.files = [];
  logs.app = [];

  event(b, 'info', 'deadloader started', { version: '0.1.0', data_folder: DATA });
  later(b, 300);
  event(b, 'info', 'webview2 runtime', { version: '155.0.4283.45' });
  event(b, 'info', 'webview2 environment variables', { variables: {} });
  event(b, 'info', 'page loaded', { ms_since_start: 412 });
  later(b, 900);
  fileOp(b, 'backup', {
    src: String.raw`${GAME}\gameinfo.gi`,
    dst: String.raw`${DATA}\backups\gameinfo\1792000000.gi`,
    bytes: 2150,
    txn: 1_792_000_000_000_000,
  });
  fileOp(b, 'rename', {
    src: String.raw`${GAME}\gameinfo.gi.tmp`,
    dst: String.raw`${GAME}\gameinfo.gi`,
    bytes: 2190,
    txn: 1_792_000_000_000_000,
  });
  event(b, 'info', 'deadlock was updated: our line in gameinfo.gi was added again');
  if (!local) {
    for (const mod of [548112, 652300, 541277, 700004, 700011]) {
      later(b, 160);
      const req = nextId(b);
      request(b, [hop(b, req, 0, `https://gamebanana.com/apiv11/Mod/${mod}${UPDATE}`, { purpose: 'update-check', bytes: 3100, ms: 170 })]);
    }
    event(b, 'info', 'mod update check at start', { mods: 5, updates_available: 1, stopped: false });
  }

  later(b, 3 * 3600_000);
  event(b, 'info', 'deadlock is running');
  later(b, 3600_000);
  event(b, 'info', 'deadlock is not running');
  apply(b, 1_792_011_000_000_000, ['toon-ivy', 'formal-wraith'], 7);
  fileOp(b, 'delete', {
    src: String.raw`${GAME}\addons\pak09_dir.vpk`,
    bytes: 42_291_000,
    result: 'failed',
    error: 'The process cannot access the file because it is being used by another process. (os error 32)',
    txn: 1_792_011_000_000_000,
  });
  event(b, 'warn', 'a file in the game folder could not be deleted', { file: String.raw`${GAME}\addons\pak09_dir.vpk` });

  if (!local) {
    later(b, 1800_000);
    for (let page = 1; page <= 12; page += 1) {
      later(b, 9000);
      browse(b, page, page === 1 ? 'user' : 'auto');
    }
    // A page that was busy, then worked on the second try.
    later(b, 4000);
    const busy = nextId(b);
    const first = hop(b, busy, 0, `https://gamebanana.com${INDEX}13`, { purpose: 'catalog', status: 429, bytes: 200, ms: 95, msg: 'response; retrying', lvl: 'info' });
    later(b, 12_000);
    request(b, [first, hop(b, busy, 0, `https://gamebanana.com${INDEX}13`, { purpose: 'catalog', attempt: 2, bytes: 40_900, ms: 301 })]);
    // An image that didn't answer.
    later(b, 3000);
    const slow = nextId(b);
    request(b, [hop(b, slow, 0, 'https://images.gamebanana.com/img/ss/mods/220-90_6709a1c2e7f43.jpg', { purpose: 'thumbnail', error: 'timed out after 10 s', ms: 10_000 })]);
    // A mod page, its gallery and two description images.
    later(b, 20_000);
    const page = nextId(b);
    request(b, [hop(b, page, 0, 'https://gamebanana.com/apiv11/Mod/652300/ProfilePage', { purpose: 'catalog', initiator: 'user', bytes: 63_200, ms: 288 })]);
    later(b, 300);
    const imgur = nextId(b);
    request(b, [hop(b, imgur, 0, 'https://i.imgur.com/Qm3xT7a.png', { purpose: 'description-image', initiator: 'user', host_source: 'user-image', bytes: 412_000, ms: 640 })]);
    later(b, 900);
    const discord = nextId(b);
    request(b, [hop(b, discord, 0, 'https://cdn.discordapp.com/attachments/1290412253741/1294410077821/preview.png', { purpose: 'description-image', initiator: 'user', host_source: 'user', bytes: 286_000, ms: 520 })]);
    // A download through two redirects.
    later(b, 30_000);
    const download = nextId(b);
    const dl = (n: number, url: string, extra: Partial<NetworkLine>) =>
      hop(b, download, n, url, { purpose: 'download', initiator: 'user', ...extra });
    const hops = [dl(0, 'https://gamebanana.com/dl/1187654', { status: 302, bytes: 400, ms: 204, msg: 'redirect' })];
    later(b, 120);
    hops.push(dl(1, 'https://files.gamebanana.com/mods/haze_bubblegum_f5f81.rar', { status: 302, bytes: 300, ms: 118, msg: 'redirect' }));
    later(b, 3500);
    hops.push(dl(2, 'https://filecache07.gamebanana.com/mods/haze_bubblegum_f5f81.rar', { bytes: 8_381_000, ms: 3490 }));
    request(b, hops);
    later(b, 4000);
    fileOp(b, 'write', { dst: String.raw`${DATA}\downloads\1.rar`, bytes: 8_381_000 });
    later(b, 800);
    fileOp(b, 'copy', { src: String.raw`${DATA}\downloads\1.rar`, dst: String.raw`${DATA}\library\gb-652300\1\0.vpk`, bytes: 8_180_000, sha256: 'ab'.repeat(32) });
    fileOp(b, 'delete', { src: String.raw`${DATA}\downloads\1.rar`, bytes: 8_381_000 });
    apply(b, 1_792_020_000_000_000, ['gb-652300'], null);
    event(b, 'info', 'download installed', { mod_id: 'gb-652300', file: 'haze_bubblegum_f5f81.rar' });
  }

  if (!local) {
    // An update whose last redirect went to a host that isn't allowed.
    later(b, scenario === 'activity-blocked' ? 600_000 : 60_000);
    const blocked = scenario === 'activity-blocked' ? 'r-blocked' : nextId(b);
    const up = (n: number, url: string, extra: Partial<NetworkLine>) =>
      hop(b, blocked, n, url, { purpose: 'download', initiator: 'user', ...extra });
    const lines = [up(0, 'https://gamebanana.com/dl/1203311', { status: 302, bytes: 400, ms: 196, msg: 'redirect' })];
    later(b, 150);
    lines.push(up(1, 'https://files.gamebanana.com/mods/yamato_redesign_v2_0c41d.zip', { status: 302, bytes: 300, ms: 122, msg: 'redirect' }));
    lines.push(up(2, 'https://dl.fastmirror.example/f/yamato_redesign_v2_0c41d.zip', { blocked: 'host-not-listed' }));
    request(b, lines);
    event(b, 'error', 'download failed', { message: { id: 'E16', args: { host: 'dl.fastmirror.example' } } });
  }
  if (scenario !== 'activity-blocked' && !local) {
    for (let page = 13; page <= 15; page += 1) {
      later(b, 7000);
      browse(b, page, 'auto');
    }
  }
  logs.network.push(...older.network);
  logs.files.push(...older.files);
  logs.app.push(...older.app);
  return logs;
}

// --- the commands -----------------------------------------------------------------------------------------------

function lines(entry: LogEntry): (NetworkLine | FileLine | AppLine)[] {
  return entry.kind === 'network' ? entry.lines : [entry.record];
}

function newest(entry: LogEntry): string {
  return lines(entry).at(-1)?.ts ?? '';
}

function hostOf(url: string): string {
  return (url.split('://')[1] ?? '').split(/[/?#]/)[0]?.split(':')[0]?.toLowerCase() ?? '';
}

function failedLine(line: NetworkLine): boolean {
  return line.blocked !== undefined || line.error !== undefined || (line.status ?? 0) >= 400;
}

function inRange(entry: LogEntry, range: LogQuery['range'], now: Date): boolean {
  const when = new Date(newest(entry));
  const days = { hour: 0, today: 1, '7d': 7, '30d': 30 }[range];
  if (range === 'hour') {
    return now.getTime() - when.getTime() <= 3600_000;
  }
  const start = new Date(now);
  start.setHours(0, 0, 0, 0);
  start.setDate(start.getDate() - (days - 1));
  return when >= start;
}

function matches(entry: LogEntry, q: LogQuery, now: Date): boolean {
  if (entry.kind !== q.kind || !inRange(entry, q.range, now)) {
    return false;
  }
  if (entry.kind === 'network') {
    const failed = failedLine(entry.lines.at(-1) as NetworkLine);
    if (q.purpose && entry.lines[0]?.purpose !== q.purpose) return false;
    if (q.host && !entry.lines.some((l) => hostOf(l.url) === q.host?.toLowerCase())) return false;
    if (q.result === 'ok' && failed) return false;
    if (q.result === 'error' && !failed) return false;
  } else if (entry.kind === 'files') {
    if (q.op && entry.record.op !== q.op) return false;
    if (q.result === 'ok' && entry.record.result !== 'ok') return false;
    if (q.result === 'error' && entry.record.result === 'ok') return false;
  } else if (q.level && entry.record.lvl !== q.level) {
    return false;
  }
  const needle = (q.text ?? '').trim().toLowerCase();
  return needle === '' || JSON.stringify(lines(entry)).toLowerCase().replaceAll('\\\\', '\\').includes(needle);
}

export function page(logs: MockLogs, q: LogQuery, now: Date): LogPage {
  const matching = logs[q.kind].filter((e) => matches(e, q, now));
  const start = q.cursor?.offset ?? 0;
  const limit = Math.min(Math.max(q.limit ?? 100, 1), 500);
  const entries = structuredClone(matching.slice(start, start + limit));
  const more = start + limit < matching.length;
  return { entries, cursor: more ? { file: `${q.kind}-mock.jsonl`, offset: start + limit, skip: [] } : null };
}

export function entry(logs: MockLogs, wanted: string): LogEntry | null {
  for (const kind of ['network', 'files', 'app'] as LogKind[]) {
    const found = logs[kind].find(
      (e) => (e.kind === 'network' && e.req === wanted) || lines(e).some((l) => l.id === wanted),
    );
    if (found) {
      return structuredClone(found);
    }
  }
  return null;
}

export function counts(logs: MockLogs, range: LogQuery['range'], now: Date): LogCounts {
  const out: LogCounts = {
    network: { requests: 0, failed: 0, purposes: {}, hosts: {} },
    files: { operations: 0, failed: 0, ops: {} },
    app: { events: 0, errors: 0, levels: {} },
  };
  const add = <K extends string>(map: Partial<Record<K, number>>, key: K) => {
    map[key] = (map[key] ?? 0) + 1;
  };
  for (const e of logs.network) {
    if (e.kind !== 'network' || !inRange(e, range, now)) continue;
    out.network.requests += 1;
    if (failedLine(e.lines.at(-1) as NetworkLine)) out.network.failed += 1;
    const purpose = e.lines[0]?.purpose;
    if (purpose) add(out.network.purposes, purpose);
    for (const l of e.lines) add(out.network.hosts, hostOf(l.url));
  }
  for (const e of logs.files) {
    if (e.kind !== 'files' || !inRange(e, range, now)) continue;
    out.files.operations += 1;
    if (e.record.result !== 'ok') out.files.failed += 1;
    add(out.files.ops, e.record.op);
  }
  for (const e of logs.app) {
    if (e.kind !== 'app' || !inRange(e, range, now)) continue;
    out.app.events += 1;
    if (e.record.lvl === 'error') out.app.errors += 1;
    add(out.app.levels, e.record.lvl);
  }
  return out;
}

export function summary(logs: MockLogs, now: Date): LogSummary {
  const contacted = new Map<string, { requests: number; source: HostSource | null }>();
  const blocked = new Map<string, { requests: number; last: string }>();
  for (const e of logs.network) {
    if (e.kind !== 'network' || !inRange(e, 'today', now)) continue;
    for (const l of e.lines) {
      const host = hostOf(l.url);
      if (l.blocked !== undefined) {
        const b = blocked.get(host) ?? { requests: 0, last: e.req };
        blocked.set(host, { requests: b.requests + 1, last: b.last });
      } else {
        const c = contacted.get(host) ?? { requests: 0, source: l.host_source ?? null };
        contacted.set(host, { requests: c.requests + 1, source: c.source });
      }
    }
  }
  const rank: Record<string, number> = { builtin: 0, github: 1, user: 2, 'user-image': 3 };
  const game: LogSummary['game'] = { changes: 0, ops: {}, gameinfo: 0, failed: 0, batches: 0, folders: [] };
  const txns = new Set<number>();
  for (const e of logs.files) {
    if (e.kind !== 'files' || !inRange(e, 'today', now)) continue;
    const r = e.record;
    const changed = r.op === 'delete' ? r.src : (r.dst ?? r.src);
    if (!changed?.toLowerCase().includes('\\game\\citadel\\')) continue;
    if (r.result !== 'ok') {
      game.failed += 1;
      continue;
    }
    game.changes += 1;
    if (changed.toLowerCase().endsWith('gameinfo.gi')) {
      game.gameinfo += 1;
    } else {
      game.ops[r.op] = (game.ops[r.op] ?? 0) + 1;
    }
    if (r.txn !== undefined) txns.add(r.txn);
    const folder = `${GAME}\\addons`;
    if (!game.folders.includes(folder)) game.folders.push(folder);
  }
  game.batches = txns.size;
  return {
    contacted: [...contacted]
      .map(([host, c]) => ({ host, ...c }))
      .sort((a, b) => (rank[a.source ?? ''] ?? 4) - (rank[b.source ?? ''] ?? 4) || b.requests - a.requests),
    blocked: [...blocked].map(([host, b]) => ({ host, ...b })),
    game,
    empty: logs.network.length === 0 && logs.files.length === 0,
  };
}

/** A new thumbnail request, as if the catalog had just loaded one (for the live view). */
export function addRequest(logs: MockLogs, now: Date): void {
  const b: Builder = { logs, at: now };
  const req = nextId(b);
  const name = (logs.next * 2654435761 >>> 0).toString(16).padStart(8, '0');
  request(b, [hop(b, req, 0, `https://images.gamebanana.com/img/ss/mods/220-90_${name}.jpg`, { purpose: 'thumbnail', bytes: 19_000, ms: 90 })]);
}
