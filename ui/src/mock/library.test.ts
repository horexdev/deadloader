import { beforeEach, describe, expect, it } from 'vitest';
import type { Changed, ForeignReport, GameinfoStatus, LibraryView, Pending } from '../lib/api';
import { devtools, invoke, resetMock } from './core';
import { LibraryMock, libraryMods } from './library';

beforeEach(() => {
  // The Library's own mock data is seeded on its route.
  history.replaceState(null, '', '/#/library');
  resetMock();
});

function host(running = false) {
  const library: LibraryView = {
    selected_profile: 'Evening',
    applied_profile: 'Evening',
    pending: false,
    pending_count: 0,
    mods: libraryMods(),
    conflicts: [],
  };
  return { library, gameRunning: running, gameRoot: 'D:\\Deadlock', catalogEnabled: () => true, push: () => undefined };
}

describe('the library mock', () => {
  it('applies edits at once while the game is closed', async () => {
    const changed = await invoke<Changed>('set_enabled', { modId: 'gb-710001', enabled: false });
    expect(changed.applied).toBe(true);
    expect(changed.library.pending_count).toBe(0);
  });

  it('lists edits made while the game runs, takes them back one by one, and applies the rest when it closes', async () => {
    devtools.setGameRunning(true);
    const changed = await invoke<Changed>('set_enabled', { modId: 'gb-710001', enabled: false });
    expect(changed.waiting_for_game).toBe(true);
    await invoke('move_mod', { modId: 'gb-710011', index: 0 });
    let pending = await invoke<Pending>('pending');
    expect(pending.changes.map((c) => c.kind).sort()).toEqual(['disable', 'order']);
    await invoke('pending_revert', { modId: 'gb-710001', aspect: 'enabled' });
    pending = await invoke<Pending>('pending');
    expect(pending.changes.map((c) => c.kind)).toEqual(['order']);

    devtools.setGameRunning(false);
    pending = await invoke<Pending>('pending');
    expect(pending.changes).toEqual([]);
    expect(pending.applied_after_game?.changes.length).toBe(1);
    const undone = await invoke<Changed>('pending_undo_applied');
    expect(undone.library.mods[0]?.id).toBe('gb-710001');
  });

  it('sets up the running board with four pending changes', () => {
    const mock = new LibraryMock(host());
    mock.scenario('library-running');
    expect(mock.changes().map((c) => c.kind).sort()).toEqual(['disable', 'enable', 'order', 'variant']);
  });

  it('sets up the applied board: the game closed after the UI subscribed, and the four changes went in with Undo', () => {
    const pushed: string[] = [];
    const mock = new LibraryMock({ ...host(), push: (event) => pushed.push(event.event) });
    mock.scenario('library-applied');
    expect(mock.changes()).toHaveLength(4);
    mock.subscribed();
    expect(pushed).toContain('applied_after_game');
    expect(mock.changes()).toEqual([]);
    expect(mock.auto?.changes).toHaveLength(4);
  });

  it('takes a foreign file under management and fixes gameinfo after Steam checked the files', async () => {
    resetMock();
    const mock = new LibraryMock(host());
    mock.scenario('library-foreign');
    const handlers = mock.handlers();
    const before = handlers.foreign_files?.({}) as ForeignReport;
    expect(before.items.map((i) => i.name)).toContain('pak01_dir.vpk');
    handlers.foreign_take?.({ name: 'pak01_dir.vpk', modName: 'Haze skin' });
    const after = handlers.foreign_files?.({}) as ForeignReport;
    expect(after.items.map((i) => i.name)).not.toContain('pak01_dir.vpk');

    mock.scenario('library-gameinfo-foreign');
    expect((handlers.gameinfo_status?.({ fix: true }) as GameinfoStatus).state).toBe('foreign');
    handlers.verify_game_files?.({});
    expect((handlers.gameinfo_status?.({ fix: true }) as GameinfoStatus).state).toBe('ours');
  });
});
