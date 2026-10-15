import { beforeEach, describe, expect, it, vi } from 'vitest';

const invoke = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({ invoke: (...args: unknown[]) => invoke(...args) }));

const { api, isUserMessage } = await import('./api');

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

  it('recognises core messages', () => {
    expect(isUserMessage({ id: 'E2', args: {} })).toBe(true);
    expect(isUserMessage({ id: 'E2' })).toBe(false);
    expect(isUserMessage('E2')).toBe(false);
    expect(isUserMessage(null)).toBe(false);
  });
});
