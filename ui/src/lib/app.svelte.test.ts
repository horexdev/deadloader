import { flushSync } from 'svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { LibraryView, TaskSnapshot } from './api';

// The app state talks to the mock core, the same one `pnpm dev:mock` uses.
vi.mock('@tauri-apps/api/core', () => import('../mock/core'));

const { AppState } = await import('./app.svelte');
const { devtools, invoke, resetMock } = await import('../mock/core');

let app: InstanceType<typeof AppState>;

beforeEach(async () => {
  resetMock();
  app = new AppState();
  await app.start();
});

afterEach(() => {
  vi.useRealTimers();
});

describe('the app state', () => {
  it('tells who listens which logs got new lines, until they stop', () => {
    const heard: string[][] = [];
    const stop = app.onLog((kinds) => heard.push(kinds));
    app.handle({ event: 'log', data: { kinds: ['network', 'files'] } });
    stop();
    app.handle({ event: 'log', data: { kinds: ['app'] } });
    expect(heard).toEqual([['network', 'files']]);
  });

  it('starts with the library, the settings, the game and the network', () => {
    expect(app.library?.selected_profile).toBe('Default');
    expect(app.settings).not.toBeNull();
    expect(app.game).toEqual({ running: false, root: expect.stringContaining('Deadlock'), installations: 2 });
    expect(app.network).toEqual({ kind: 'online' });
  });

  it('keeps the latest catalog change and update report for the screens', () => {
    const query = { section: 'mod' as const, filter: { kind: 'browse' as const, sort: 'newest' as const, category: null }, page: 2 };
    app.handle({ event: 'catalog', data: { event: 'page_updated', data: query } });
    expect(app.catalogEvent).toEqual({ event: 'page_updated', data: query });
    app.handle({ event: 'catalog', data: { event: 'status', data: { kind: 'offline', since_unix_ms: 5 } } });
    expect(app.catalogEvent?.event).toBe('page_updated');
    const report = { checks: [{ mod_id: 'gb-700003', status: { kind: 'up_to_date' as const } }], available: 0, stopped: null };
    app.handle({ event: 'updates', data: report });
    expect(app.updates).toEqual(report);
  });

  it('follows the game starting and stopping', () => {
    devtools.setGameRunning(true);
    flushSync();
    expect(app.game?.running).toBe(true);
    devtools.setGameRunning(false);
    expect(app.game?.running).toBe(false);
  });

  it('counts running downloads and takes the installed mod into the library', async () => {
    vi.useFakeTimers();
    const before = app.library?.mods.length ?? 0;
    await invoke('download_install', { section: 'mod', modId: 700_001, fileId: 1, name: 'Seven — Ember Coat', author: null });
    expect(app.downloads).toBe(1);
    await vi.advanceTimersByTimeAsync(10_000);
    expect(app.downloads).toBe(0);
    expect(app.library?.mods.length).toBe(before + 1);
  });

  it('asks before Play takes mods out of the game for a profile switch (D13)', async () => {
    await invoke('select_profile', { name: 'Evening' });
    app.library = await invoke('library');
    await app.requestPlay();
    expect(app.confirm?.turned_off.length).toBeGreaterThan(0);
    expect(app.game?.running).toBe(false);

    await app.confirmPlay();
    expect(app.confirm).toBeNull();
    expect(app.library?.applied_profile).toBe('Evening');
    expect(app.game?.running).toBe(true);
  });

  it('plays at once when nothing leaves the game', async () => {
    await app.requestPlay();
    expect(app.confirm).toBeNull();
    expect(app.game?.running).toBe(true);
  });

  it('shows a failure as a toast', async () => {
    devtools.setGameRunning(true);
    await app.play();
    expect(app.toasts.at(-1)?.kind).toBe('error');
  });

  it('opens D12 for an error with its actions, and closes it', () => {
    const retry = vi.fn();
    app.showError({ id: 'E9', args: { reason: 'too_large', limit: '2147483648' } }, [
      { label: 'Retry', place: 'primary', run: retry },
    ]);
    expect(app.error?.message.id).toBe('E9');
    expect(app.error?.actions.map((a) => a.label)).toEqual(['Retry']);
    app.closeError();
    expect(app.error).toBeNull();
  });

  it('opens D12 for anything thrown, as an internal error', () => {
    app.showError(new Error('boom'));
    expect(app.error?.message).toEqual({ id: 'Internal', args: { detail: 'Error: boom' } });
    expect(app.error?.actions).toEqual([]);
  });

  describe('downloads finished this session', () => {
    const snapshot = (id: number, state: TaskSnapshot['state']): TaskSnapshot => ({
      id,
      url: `https://gamebanana.com/dl/${id}`,
      file_name: `cafe_${id}.zip`,
      md5: null,
      size: 46_200_000,
      initiator: 'user',
      meta: { section: 'mod', mod_id: '700011', file_id: String(1_800_000 + id), mod_name: 'Cafe (4 Variants)' },
      state,
      bytes: 0,
      total: 46_200_000,
      speed: 0,
    });
    const added = (id: number, state: TaskSnapshot['state'] = { state: 'downloading' }) =>
      app.handle({ event: 'download', data: { event: 'added', task: snapshot(id, state) } });
    const removed = (id: number) => app.handle({ event: 'download', data: { event: 'removed', id } });
    const installed = (id: number, choices: string[][]) =>
      app.handle({
        event: 'installed',
        data: {
          task: id,
          installed: {
            outcome: { mod_id: 'gb-700000', new_files: choices.flat(), choices, skipped: [] },
            changed:
              choices.length === 0 ? { library: app.library as LibraryView, applied: true, waiting_for_game: false } : null,
          },
        },
      });

    it('remembers an installed download once, not as cancelled when it leaves the queue', () => {
      added(41);
      installed(41, []);
      app.handle({ event: 'download', data: { event: 'state', id: 41, state: { state: 'done' } } });
      removed(41);
      expect(app.history.map((h) => [h.name, h.file, h.outcome])).toEqual([['Cafe (4 Variants)', 'cafe_41.zip', 'installed']]);
      expect(app.history[0]?.again).toEqual({ section: 'mod', modId: 700011, fileId: 1800041, author: null });
    });

    it('keeps a download with variants waiting for the choice (D4)', () => {
      added(42);
      installed(42, [['f-a', 'f-b', 'f-c']]);
      removed(42);
      expect(app.history).toEqual([]);
      expect(app.choices).toEqual([
        {
          key: 'task-42',
          modId: 'gb-700000',
          choices: [['f-a', 'f-b', 'f-c']],
          name: 'Cafe (4 Variants)',
          file: 'cafe_42.zip',
          size: 46_200_000,
          source: { section: 'mod', modId: 700011, fileId: 1800042 },
        },
      ]);
    });

    it('remembers cancelled downloads, and failed ones that were removed', () => {
      added(43);
      removed(43);
      added(44, { state: 'failed', message: { id: 'E9', args: {} } });
      removed(44);
      expect(app.history.map((h) => [h.file, h.outcome])).toEqual([
        ['cafe_44.zip', 'removed'],
        ['cafe_43.zip', 'cancelled'],
      ]);
    });

    it('remembers an update that finished as updated', () => {
      app.handle({
        event: 'download',
        data: { event: 'added', task: { ...snapshot(45, { state: 'installing' }), meta: { ...snapshot(45, { state: 'queued' }).meta, replaces: '1', version: '2.0' } } },
      });
      app.handle({ event: 'download', data: { event: 'state', id: 45, state: { state: 'done' } } });
      removed(45);
      expect(app.history.map((h) => [h.outcome, h.version])).toEqual([['updated', '2.0']]);
    });

    it('turns the picked variant on and remembers it', async () => {
      added(46);
      installed(46, [['gb-700000-1-0', 'gb-700000-1-1']]);
      removed(46);
      expect(await app.chooseVariant('task-46', ['gb-700000-1-1'], 'blue')).toBe(true);
      expect(app.choices).toEqual([]);
      expect(app.history.map((h) => [h.outcome, h.variant])).toEqual([['installed', 'blue']]);
    });

    it('puts a choice aside: the mod stays in the library, not turned on', () => {
      added(47);
      installed(47, [['a', 'b']]);
      app.putChoiceAside('task-47');
      expect(app.choices).toEqual([]);
      expect(app.history.map((h) => h.outcome)).toEqual(['kept']);
      app.clearHistory();
      expect(app.history).toEqual([]);
    });
  });
});

