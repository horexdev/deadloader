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

const { api, catalog, downloads, imageUrl, isUserMessage, subscribe, updates } = await import('./api');

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

  it('calls the catalog commands by their Rust names', async () => {
    invoke.mockResolvedValue({});
    const query = { section: 'mod' as const, filter: { kind: 'search' as const, text: 'haze' }, page: 2 };
    const id = '0123456789abcdef0123456789abcdef';
    await catalog.list(query, true);
    await catalog.refresh(query);
    await catalog.prefetch(query);
    await catalog.categories('mod', 4321);
    await catalog.details('mod', 655209, 1792060000);
    await catalog.status();
    await catalog.retryNow();
    await catalog.activate(id);
    await catalog.consentPrompt(id);
    await catalog.grant(id, 'always');
    expect(invoke.mock.calls).toEqual([
      ['catalog_list', { query, byUser: true }],
      ['catalog_refresh', { query }],
      ['catalog_prefetch', { query }],
      ['catalog_categories', { section: 'mod', parent: 4321 }],
      ['catalog_details', { section: 'mod', id: 655209, listedUpdated: 1792060000 }],
      ['catalog_status', {}],
      ['catalog_retry_now', {}],
      ['catalog_activate', { id }],
      ['catalog_consent_prompt', { id }],
      ['catalog_grant', { id, scope: 'always' }],
    ]);
  });

  it('passes the catalog states through instead of calling them internal errors', async () => {
    const query = { section: 'mod' as const, filter: { kind: 'browse' as const, sort: 'newest' as const, category: null }, page: 1 };
    invoke.mockRejectedValue({ kind: 'local_mode' });
    await expect(catalog.list(query, false)).rejects.toEqual({ kind: 'local_mode' });
    invoke.mockRejectedValue({ id: 'E1', args: {} });
    await expect(catalog.status()).rejects.toEqual({ id: 'E1', args: {} });
    invoke.mockRejectedValue('broken');
    await expect(catalog.status()).rejects.toEqual({ id: 'Internal', args: { detail: 'broken' } });
  });

  it('calls the download commands by their Rust names', async () => {
    invoke.mockResolvedValue({});
    await downloads.install('mod', 655209, 1480331, 'Haze V2', 'Nyx');
    await downloads.list();
    await downloads.control(17, 'pause');
    await downloads.control(17, 'cancel');
    expect(invoke.mock.calls).toEqual([
      ['download_install', { section: 'mod', modId: 655209, fileId: 1480331, name: 'Haze V2', author: 'Nyx' }],
      ['downloads', {}],
      ['download_control', { id: 17, action: 'pause' }],
      ['download_control', { id: 17, action: 'cancel' }],
    ]);
  });

  it('checks for mod updates through the catalog failure path', async () => {
    invoke.mockResolvedValue({ checks: [], available: 0, stopped: null });
    await updates.check();
    await updates.dismiss('gb-655209', [7, 8]);
    expect(invoke.mock.calls).toEqual([
      ['updates_check', {}],
      ['updates_dismiss', { modId: 'gb-655209', fileIds: [7, 8] }],
    ]);
    invoke.mockRejectedValue({ id: 'E1', args: {} });
    await expect(updates.check()).rejects.toEqual({ id: 'E1', args: {} });
  });

  it('builds picture addresses on the image protocol only', () => {
    expect(imageUrl('0123456789abcdef0123456789abcdef')).toBe(
      'https://dl.localhost/img/0123456789abcdef0123456789abcdef',
    );
  });

  it('recognises core messages', () => {
    expect(isUserMessage({ id: 'E2', args: {} })).toBe(true);
    expect(isUserMessage({ id: 'E2' })).toBe(false);
    expect(isUserMessage('E2')).toBe(false);
    expect(isUserMessage(null)).toBe(false);
  });
});
