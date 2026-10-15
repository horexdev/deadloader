import { beforeEach, describe, expect, it, vi } from 'vitest';

const invoke = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({ invoke: (...args: unknown[]) => invoke(...args) }));

const { maintenance } = await import('./api');

describe('the settings and maintenance commands', () => {
  beforeEach(() => {
    invoke.mockReset();
    invoke.mockResolvedValue({});
  });

  it('call the core by their Rust names with camelCase arguments', async () => {
    const text = { title: 'Choose', filter: '' };
    await maintenance.reset();
    await maintenance.fileText('manifest');
    await maintenance.saveFile('settings', '{}');
    await maintenance.cacheUsage();
    await maintenance.clearCache();
    await maintenance.installations();
    await maintenance.gameFolderDialog(text, true);
    await maintenance.removeInstallation('F:\\Deadlock');
    await maintenance.openFolder({ place: 'game', root: 'D:\\Deadlock' });
    await maintenance.about();
    await maintenance.rollbackPreview();
    await maintenance.rollback(true);
    await maintenance.wipeSummary();
    await maintenance.deleteAllData(false);
    await maintenance.setupCheck();
    await maintenance.acknowledgeNotice(1, 'de', 'local_mode', false);
    await maintenance.chooseCatalog(true);
    expect(invoke.mock.calls).toEqual([
      ['settings_reset', {}],
      ['settings_file_text', { file: 'manifest' }],
      ['settings_file_save', { file: 'settings', text: '{}' }],
      ['cache_usage', {}],
      ['cache_clear', {}],
      ['installations', {}],
      ['game_folder_dialog', { text, select: true }],
      ['installation_remove', { gameRoot: 'F:\\Deadlock' }],
      ['open_folder', { place: { place: 'game', root: 'D:\\Deadlock' } }],
      ['about', {}],
      ['rollback_preview', {}],
      ['rollback', { removeForeign: true }],
      ['wipe_summary', {}],
      ['delete_all_data', { rollback: false }],
      ['setup_check', {}],
      ['acknowledge_notice', { version: 1, language: 'de', catalog: 'local_mode', checkAppUpdates: false }],
      ['choose_catalog', { enabled: true }],
    ]);
  });

  it('pass core messages through as they are', async () => {
    invoke.mockRejectedValueOnce({ id: 'E13', args: {} });
    await expect(maintenance.rollback(false)).rejects.toEqual({ id: 'E13', args: {} });
  });
});
