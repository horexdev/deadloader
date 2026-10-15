import { flushSync } from 'svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

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
  it('starts with the library, the settings, the game and the network', () => {
    expect(app.library?.selected_profile).toBe('Default');
    expect(app.settings).not.toBeNull();
    expect(app.game).toEqual({ running: false, root: expect.stringContaining('Deadlock'), installations: 2 });
    expect(app.network).toEqual({ kind: 'online' });
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
});
