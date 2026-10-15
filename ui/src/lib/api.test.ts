import { beforeEach, describe, expect, it, vi } from 'vitest';

const invoke = vi.fn();

/** Tauri's channel: the test plays the core and calls `onmessage`. */
class FakeChannel<T> {
  onmessage: (message: T) => void = () => {};
}

vi.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: unknown[]) => invoke(...args),
  Channel: FakeChannel,
}));

const { api, isUserMessage, subscribe } = await import('./api');

describe('api', () => {
  beforeEach(() => {
    invoke.mockReset();
  });

  it('calls each command by its Rust name with camelCase arguments', async () => {
    invoke.mockResolvedValue({});
    await api.library();
    await api.setEnabled('local-1', false);
    await api.moveMod('local-1', 2);
    await api.switchVariant('local-1', 'local-1-1-0');
    await api.choose('local-1', ['local-1-1-1']);
    await api.selectProfile('Evening');
    await api.createProfile('Evening');
    await api.profileNames();
    await api.importPaths(['D:\\mods\\haze.zip']);
    await api.apply();
    await api.removalPlan('local-1');
    await api.remove('local-1', ['local-1-1-0']);
    await api.renameProfile('A', 'B');
    await api.duplicateProfile('B', 'C');
    await api.deleteProfile('C');
    await api.exportProfile('B');
    await api.importProfile('{}');
    await api.settings();
    await api.updateSettings({ general: { zoom_percent: 110 } });
    const text = { title: 'Add mods', filter: 'Mod files' };
    await api.addFromDisk(text);
    await api.exportProfileDialog('B', text);
    await api.importProfileDialog(text);
    expect(invoke.mock.calls).toEqual([
      ['library', {}],
      ['set_enabled', { modId: 'local-1', enabled: false }],
      ['move_mod', { modId: 'local-1', index: 2 }],
      ['switch_variant', { modId: 'local-1', file: 'local-1-1-0' }],
      ['choose', { modId: 'local-1', files: ['local-1-1-1'] }],
      ['select_profile', { name: 'Evening' }],
      ['create_profile', { name: 'Evening' }],
      ['profile_names', {}],
      ['import_paths', { paths: ['D:\\mods\\haze.zip'] }],
      ['apply', {}],
      ['removal_plan', { modId: 'local-1', files: null }],
      ['remove', { modId: 'local-1', files: ['local-1-1-0'] }],
      ['rename_profile', { old: 'A', new: 'B' }],
      ['duplicate_profile', { name: 'B', newName: 'C' }],
      ['delete_profile', { name: 'C' }],
      ['export_profile', { name: 'B' }],
      ['import_profile', { text: '{}' }],
      ['settings', {}],
      ['update_settings', { settings: { general: { zoom_percent: 110 } } }],
      ['add_from_disk', { text }],
      ['export_profile_dialog', { name: 'B', text }],
      ['import_profile_dialog', { text }],
    ]);
  });

  it('passes the core message of a failure through unchanged', async () => {
    const message = { id: 'E13', args: {} };
    invoke.mockRejectedValue(message);
    await expect(api.apply()).rejects.toEqual(message);
  });

  it('turns an unexpected failure into an internal message', async () => {
    invoke.mockRejectedValue(new Error('ipc broke'));
    await expect(api.library()).rejects.toEqual({ id: 'Internal', args: { detail: 'Error: ipc broke' } });
  });

  it('subscribes once with a channel and hands every push to the handler', async () => {
    invoke.mockResolvedValue(null);
    const seen: unknown[] = [];
    await subscribe((event) => seen.push(event));
    expect(invoke).toHaveBeenCalledTimes(1);
    const [command, args] = invoke.mock.calls[0] as [string, { events: FakeChannel<unknown> }];
    expect(command).toBe('subscribe');
    expect(args.events).toBeInstanceOf(FakeChannel);
    args.events.onmessage({ event: 'game', data: { running: true } });
    args.events.onmessage({ event: 'drag_hover', data: { over: false } });
    expect(seen).toEqual([
      { event: 'game', data: { running: true } },
      { event: 'drag_hover', data: { over: false } },
    ]);
  });

  it('recognises core messages', () => {
    expect(isUserMessage({ id: 'E2', args: {} })).toBe(true);
    expect(isUserMessage({ id: 'E2' })).toBe(false);
    expect(isUserMessage('E2')).toBe(false);
    expect(isUserMessage(null)).toBe(false);
  });
});
