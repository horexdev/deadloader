import { beforeEach, describe, expect, it, vi } from 'vitest';

const invoke = vi.fn();

vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => invoke(...args),
  Channel: class {
    onmessage = (): void => undefined;
  },
}));

const { library } = await import('./api');

describe('library api', () => {
  beforeEach(() => {
    invoke.mockReset();
  });

  it('calls the pending, foreign and gameinfo commands by their Rust names', async () => {
    invoke.mockResolvedValue({});
    await library.pending();
    await library.revert('gb-1', 'order');
    await library.discard();
    await library.undoApplied();
    await library.foreign();
    await library.leaveForeign('replays');
    await library.takeForeign('pak01_dir.vpk', 'Haze skin');
    await library.gameinfo(true);
    await library.verifyGameFiles();
    await library.show({ kind: 'foreign', name: 'pak01_dir.vpk' }, false);
    await library.show({ kind: 'mod_files', mod_id: 'gb-1' }, false);
    expect(invoke.mock.calls).toEqual([
      ['pending', {}],
      ['pending_revert', { modId: 'gb-1', aspect: 'order' }],
      ['pending_discard', {}],
      ['pending_undo_applied', {}],
      ['foreign_files', {}],
      ['foreign_leave', { name: 'replays' }],
      ['foreign_take', { name: 'pak01_dir.vpk', modName: 'Haze skin' }],
      ['gameinfo_status', { fix: true }],
      ['verify_game_files', {}],
      ['show_place', { place: { kind: 'foreign', name: 'pak01_dir.vpk' }, asFile: false }],
      ['show_place', { place: { kind: 'mod_files', mod_id: 'gb-1' }, asFile: false }],
    ]);
  });

  it('passes core messages through', async () => {
    invoke.mockRejectedValue({ id: 'E13', args: {} });
    await expect(library.takeForeign('pak01_dir.vpk', '')).rejects.toEqual({ id: 'E13', args: {} });
  });
});
