import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import type { CacheUsage, EditResult, RolledBack, Settings, SetupCheck } from '../lib/api';
import { invoke, resetMock } from './core';

function scenario(name: string | null): void {
  history.replaceState(null, '', name ? `/?scenario=${name}` : '/');
  resetMock();
}

describe('the mock core: settings, first run and maintenance', () => {
  beforeEach(() => scenario(null));
  afterEach(() => scenario(null));

  it('acknowledges the notice everywhere but in the wizard scenarios', async () => {
    const settings = await invoke<Settings>('settings');
    expect(settings.acknowledgements?.notice).toMatchObject({ version: 1 });
    scenario('wizard');
    const first = await invoke<Settings>('settings');
    expect(first.acknowledgements?.notice).toBeNull();
    expect(first.catalog?.enabled).toBe(false);
  });

  it('records the notice and keeps the update check off in local mode', async () => {
    scenario('wizard');
    const saved = await invoke<Settings>('acknowledge_notice', {
      version: 1,
      language: 'de',
      catalog: 'local_mode',
      checkAppUpdates: true,
    });
    expect(saved.general?.check_app_updates).toBe(false);
    expect(saved.acknowledgements?.notice).toMatchObject({ version: 1, language: 'de', catalog: 'local_mode' });
    expect((await invoke<Settings>('settings')).catalog?.enabled).toBe(false);
  });

  it('rolls the selected folder back once, and refuses while the game runs', async () => {
    const before = await invoke<{ files: number }>('rollback_preview');
    expect(before.files).toBe(10);
    const done = await invoke<RolledBack>('rollback', { removeForeign: true });
    expect(done.foreign_removed).toHaveLength(2);
    expect((await invoke<{ files: number }>('rollback_preview')).files).toBe(0);
    scenario('running');
    await expect(invoke('rollback', { removeForeign: false })).rejects.toEqual({ id: 'E13', args: {} });
  });

  it('checks the Advanced text like the core: a syntax error or a value out of range saves nothing', async () => {
    scenario('settings-advanced-error');
    const text = await invoke<string>('settings_file_text', { file: 'settings' });
    const broken = await invoke<EditResult>('settings_file_save', { file: 'settings', text });
    expect(broken).toMatchObject({ Err: { kind: 'json' } });
    const fixed = text.replace('"gif_autoplay": false', '"gif_autoplay": false,');
    const range = await invoke<EditResult>('settings_file_save', {
      file: 'settings',
      text: fixed.replace('"retries": 5', '"retries": 50'),
    });
    expect(range).toMatchObject({ Err: { kind: 'problems' } });
    const ok = await invoke<EditResult>('settings_file_save', { file: 'settings', text: fixed.replace('"retries": 5', '"retries": 2') });
    expect(ok).toMatchObject({ Ok: {} });
    expect((await invoke<Settings>('settings')).network?.retries).toBe(2);
  });

  it('empties the cache', async () => {
    expect((await invoke<CacheUsage>('cache_usage')).bytes).toBeGreaterThan(0);
    await invoke('cache_clear');
    expect(await invoke<CacheUsage>('cache_usage')).toEqual({ bytes: 0, thumbnails: 0, images: 0, pages: 0 });
  });

  it('shows the setup issues of the board WizardSetupIssues', async () => {
    scenario('wizard-setup-issues');
    const check = await invoke<SetupCheck>('setup_check');
    expect(check).toMatchObject({ game_root: null, exe_location: 'downloads', same_volume: false, gameinfo: 'foreign' });
    expect(check.foreign).toHaveLength(3);
    const root = await invoke<string>('game_folder_dialog', { text: { title: '', filter: '' }, select: true });
    expect((await invoke<SetupCheck>('setup_check')).installations).toBe(1);
    expect(root).toContain('Deadlock');
  });
});
