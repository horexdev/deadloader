import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

// The screen's state talks to the mock core, the same one `pnpm dev:mock` uses.
vi.mock('@tauri-apps/api/core', () => import('../../mock/core'));

const { ActivityView } = await import('./view.svelte');
const { devtools, resetMock } = await import('../../mock/core');
const { entryId } = await import('./model');

let failures: unknown[];

/** The id of the n-th entry shown (failing when there is none). */
function idAt(v: InstanceType<typeof ActivityView>, n: number): string {
  const entry = v.entries[n];
  if (!entry) {
    throw new Error(`no entry ${n}`);
  }
  return entryId(entry);
}

function view() {
  failures = [];
  return new ActivityView({ fail: (e) => failures.push(e) });
}

beforeEach(() => {
  resetMock();
});

afterEach(() => {
  expect(failures).toEqual([]);
});

describe('the Activity screen state', () => {
  it('opens on the network tab with today’s requests, totals and summary', async () => {
    const v = view();
    await v.open();
    expect(v.tab).toBe('network');
    expect(v.entries.length).toBe(100);
    expect(v.entries.every((e) => e.kind === 'network')).toBe(true);
    expect(v.cursor).not.toBeNull();
    expect(v.counts?.network.requests).toBeGreaterThan(100);
    expect(v.summary?.contacted.map((h) => h.host)).toEqual(
      expect.arrayContaining(['gamebanana.com', 'images.gamebanana.com', 'i.imgur.com']),
    );
    expect(v.summary?.blocked[0]?.host).toBe('dl.fastmirror.example');
  });

  it('loads the next page once, however often it is asked', async () => {
    const v = view();
    await v.open();
    await Promise.all([v.more(), v.more()]);
    expect(v.entries.length).toBe(200);
    const ids = v.entries.map(entryId);
    expect(ids.filter((id, i) => ids.indexOf(id) === i)).toHaveLength(200);
  });

  it('stops loading pages at the most entries it keeps (the memory budget)', async () => {
    failures = [];
    const v = new ActivityView({ fail: (e) => failures.push(e), maxLoaded: 150 });
    await v.open();
    await v.more();
    await v.more();
    expect(v.entries.length).toBe(200);
    expect(v.atLimit).toBe(true);
  });

  it('filters and searches, and clears both', async () => {
    const v = view();
    await v.open();
    await v.setFilter('result', 'error');
    expect(v.entries.length).toBeGreaterThan(0);
    expect(v.entries.length).toBeLessThan(10);
    await v.search('fastmirror');
    expect(v.entries.length).toBe(1);
    await v.clearAll();
    expect(v.entries.length).toBe(100);
    expect(v.text).toBe('');
    expect(v.filters.result).toBe('all');
  });

  it('switches tabs and closes the details', async () => {
    const v = view();
    await v.open();
    v.select(idAt(v, 0));
    await v.setTab('files');
    expect(v.selected).toBeNull();
    expect(v.entries.every((e) => e.kind === 'files')).toBe(true);
  });

  it('opens on one entry named in the address, on its tab', async () => {
    const v = view();
    await v.open('mv9fkgpf-000001');
    expect(v.tab).toBe('app');
    expect(v.selected).toBe('mv9fkgpf-000001');
    expect(v.selectedEntry?.kind).toBe('app');
  });

  it('shows new entries at once while following, and offers them while paused', async () => {
    const v = view();
    await v.open();
    const first = idAt(v, 0);
    devtools.addLogRequest();
    await v.onLog(['network']);
    expect(idAt(v, 1)).toBe(first);

    v.setFollow(false);
    devtools.addLogRequest();
    devtools.addLogRequest();
    await v.onLog(['network']);
    expect(v.pending).toBe(2);
    expect(v.pausedAt).not.toBeNull();
    const top = idAt(v, 0);
    expect(top).not.toBe(first);
    v.showNew();
    expect(v.pending).toBe(0);
    expect(idAt(v, 2)).toBe(top);
  });

  it('ignores news of other logs except for the totals', async () => {
    const v = view();
    await v.open();
    const before = v.entries;
    await v.onLog(['app']);
    expect(v.entries).toBe(before);
  });

  it('opens and closes a request’s hops', async () => {
    const v = view();
    await v.open();
    await v.search('fastmirror');
    const req = idAt(v, 0);
    v.toggleHops(req);
    expect(v.rows.length).toBe(4);
    v.toggleHops(req);
    expect(v.rows.length).toBe(1);
  });
});
