import { describe, expect, it } from 'vitest';
import type { FileLine, LogEntry, NetworkLine, LogSummary } from '../../lib/api';
import {
  DEFAULT_FILTERS,
  anyFilter,
  copyText,
  entryId,
  entryProblem,
  flatten,
  gameParts,
  hopCounts,
  hostPath,
  mergeHead,
  networkStatus,
  queryFor,
  sizeOf,
  durationOf,
  tabOf,
} from './model';

function line(fields: Partial<NetworkLine>): NetworkLine {
  return {
    ts: '2026-10-15T14:29:13.482+03:00',
    id: 'l-1',
    lvl: 'info',
    msg: 'response',
    req: 'r-1',
    hop: 0,
    method: 'GET',
    url: 'https://gamebanana.com/dl/1187654',
    host_source: 'builtin',
    initiator: 'user',
    purpose: 'download',
    status: 200,
    bytes: 1000,
    ms: 100,
    ...fields,
  };
}

function request(...lines: NetworkLine[]): LogEntry {
  return { kind: 'network', req: lines[0]?.req ?? 'r-1', lines };
}

function file(fields: Partial<FileLine>): LogEntry {
  return {
    kind: 'files',
    record: { ts: '2026-10-15T14:29:22.207+03:00', id: 'f-1', lvl: 'info', msg: 'file operation', op: 'hardlink', result: 'ok', ...fields },
  };
}

const redirected = request(
  line({ id: 'l-1', status: 302, bytes: 400, ms: 204 }),
  line({ id: 'l-2', hop: 1, url: 'https://files.gamebanana.com/mods/x.rar', status: 302, bytes: 300, ms: 118 }),
  line({ id: 'l-3', hop: 2, url: 'https://filecache07.gamebanana.com/mods/x.rar', bytes: 8_381_000, ms: 3490 }),
);
const blocked = request(
  line({ id: 'b-1', req: 'r-2', status: 302 }),
  line({ id: 'b-2', req: 'r-2', hop: 1, url: 'https://dl.fastmirror.example/f/x.zip', status: undefined, bytes: undefined, ms: 0, blocked: 'host-not-listed', host_source: undefined }),
);

describe('the Activity model', () => {
  it('builds the query of the tab, leaving out the other tabs’ filters', () => {
    const filters = { ...DEFAULT_FILTERS, purpose: 'download' as const, op: 'delete' as const, result: 'error' as const, host: 'gamebanana.com' };
    expect(queryFor('network', filters, ' dl ', null)).toEqual({
      kind: 'network',
      range: 'today',
      purpose: 'download',
      host: 'gamebanana.com',
      op: null,
      level: null,
      result: 'error',
      text: 'dl',
      cursor: null,
      limit: 100,
    });
    expect(queryFor('app', filters, '', null)).toMatchObject({ kind: 'app', purpose: null, host: null, op: null, result: 'all' });
    expect(queryFor('files', filters, '', null)).toMatchObject({ op: 'delete', purpose: null, result: 'error' });
  });

  it('tells whether a filter of the tab is set', () => {
    expect(anyFilter('network', DEFAULT_FILTERS)).toBe(false);
    expect(anyFilter('network', { ...DEFAULT_FILTERS, op: 'copy' })).toBe(false);
    expect(anyFilter('files', { ...DEFAULT_FILTERS, op: 'copy' })).toBe(true);
    expect(anyFilter('app', { ...DEFAULT_FILTERS, range: 'hour' })).toBe(true);
  });

  it('names entries and their tabs by id', () => {
    expect(entryId(redirected)).toBe('r-1');
    expect(entryId(file({}))).toBe('f-1');
    expect(tabOf(file({}))).toBe('files');
  });

  it('splits an address into host and the rest', () => {
    expect(hostPath('https://gamebanana.com/apiv11/Mod/1?x=1')).toEqual({ host: 'gamebanana.com', path: '/apiv11/Mod/1?x=1' });
    expect(hostPath('https://i.imgur.com')).toEqual({ host: 'i.imgur.com', path: '' });
  });

  it('gives a request the status of its last hop and adds up sizes and durations', () => {
    expect(networkStatus(redirected)).toEqual({ kind: 'ok', code: 200 });
    expect(networkStatus(blocked)).toEqual({ kind: 'blocked', code: null });
    expect(networkStatus(request(line({ error: 'timed out', status: undefined })))).toEqual({ kind: 'failed', code: null });
    expect(networkStatus(request(line({ status: 404 })))).toEqual({ kind: 'error', code: 404 });
    expect(sizeOf(redirected)).toBe(8_381_700);
    expect(durationOf(redirected)).toBe(3812);
    expect(sizeOf(request(line({ bytes: undefined })))).toBeNull();
  });

  it('turns a failed entry into the message that explains it', () => {
    expect(entryProblem(blocked)).toEqual({ id: 'E16', args: { reason: 'not_listed', host: 'dl.fastmirror.example' } });
    expect(entryProblem(request(line({ purpose: 'catalog', status: 429 })))).toMatchObject({ id: 'E5', args: { status: '429' } });
    expect(entryProblem(request(line({ purpose: 'download', status: 404 })))).toMatchObject({ id: 'E24' });
    expect(entryProblem(request(line({ purpose: 'catalog', status: 403 })))).toMatchObject({ id: 'E25' });
    expect(entryProblem(request(line({ purpose: 'thumbnail', status: undefined, error: 'timed out' })))).toMatchObject({
      id: 'E19',
      args: { reason: 'no_response' },
    });
    expect(entryProblem(request(line({ status: undefined, error: 'dns failed' })))).toMatchObject({ id: 'E6', args: { detail: 'dns failed' } });
    expect(entryProblem(file({ result: 'failed', error: 'Access is denied. (os error 5)' }))).toMatchObject({
      id: 'E18',
      args: { reason: 'access_denied' },
    });
    expect(entryProblem(file({ result: 'failed', error: 'being used by another process. (os error 32)' }))).toMatchObject({
      args: { reason: 'in_use' },
    });
    expect(entryProblem(redirected)).toBeNull();
  });

  it('tells redirects from retries', () => {
    expect(hopCounts(redirected)).toEqual({ redirects: 2, retries: 0 });
    expect(hopCounts(request(line({ status: 429 }), line({ attempt: 2 })))).toEqual({ redirects: 0, retries: 1 });
  });

  it('flattens rows and their open hops for the virtual list', () => {
    const rows = flatten([redirected, blocked], new Set(['r-2']));
    expect(rows.map((r) => (r.type === 'hop' ? `${r.entry.kind}:${r.index}` : r.type))).toEqual([
      'entry',
      'entry',
      'network:0',
      'network:1',
    ]);
    // A request without redirects has nothing to open.
    expect(flatten([request(line({}))], new Set(['r-1'])).length).toBe(1);
  });

  it('puts new entries on top and keeps the older ones once', () => {
    const a = file({ id: 'a' });
    const b = file({ id: 'b' });
    const c = file({ id: 'c' });
    const updated = request(line({ req: 'r-1' }), line({ req: 'r-1', hop: 1 }));
    expect(mergeHead([b, a], [c, b]).entries.map(entryId)).toEqual(['c', 'b', 'a']);
    expect(mergeHead([b, a], [c, b]).added).toBe(1);
    // A request that got another hop is replaced, not counted as new.
    const merged = mergeHead([redirected], [updated]);
    expect(merged.added).toBe(0);
    expect(merged.entries).toEqual([updated]);
  });

  it('copies the entry as the log lines it came from', () => {
    expect(copyText(blocked).split('\n')).toHaveLength(2);
    expect(JSON.parse(copyText(blocked).split('\n')[1] ?? '')).toMatchObject({ id: 'b-2', blocked: 'host-not-listed' });
  });

  it('lists the kinds of change made in the game folder', () => {
    const game: LogSummary['game'] = {
      changes: 9,
      ops: { hardlink: 4, delete: 1, rename: 3 },
      gameinfo: 1,
      failed: 1,
      batches: 3,
      folders: [],
    };
    expect(gameParts(game)).toEqual([
      ['hardlink', 4],
      ['copy', 0],
      ['delete', 1],
      ['rename', 3],
      ['write', 0],
      ['backup', 0],
      ['restore', 0],
    ].filter(([, n]) => n !== 0));
  });
});
