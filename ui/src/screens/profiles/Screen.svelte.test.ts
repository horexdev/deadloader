import { flushSync } from 'svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

// The screen talks to the mock core, the same one `pnpm dev:mock` uses; calls are recorded.
const calls: { command: string; args: Record<string, unknown> }[] = [];
vi.mock('@tauri-apps/api/core', async () => {
  const mock = await import('../../mock/core');
  return {
    ...mock,
    invoke: (command: string, args: Record<string, unknown> = {}) => {
      calls.push({ command, args });
      return mock.invoke(command, args);
    },
  };
});

const { AppState } = await import('../../lib/app.svelte');
const { resetMock } = await import('../../mock/core');
const { default: Screen } = await import('./Screen.svelte');
const { all, click, fire, keydown, one, render } = await import('../../lib/test/render');

const plain = (text: string | null | undefined) => (text ?? '').replace(/\s+/g, ' ').trim();

async function start(scenario: string | null = null) {
  history.replaceState(null, '', scenario ? `/?scenario=${scenario}` : '/');
  resetMock();
  const app = new AppState();
  await app.start();
  const { target } = render(Screen, { app });
  await vi.waitFor(() => expect(target.querySelectorAll('.pf-row').length).toBeGreaterThan(0));
  calls.length = 0;
  return { app, target };
}

function row(target: HTMLElement, name: string): HTMLElement {
  const found = all(target, '.pf-row').find((r) => r.querySelector('.pf-name')?.textContent === name);
  if (!found) {
    throw new Error(`no row ${name}`);
  }
  return found;
}

function menuItem(target: HTMLElement, name: string, item: string): HTMLElement {
  click(one(row(target, name), 'button[aria-haspopup="menu"]'));
  const found = all(row(target, name), '[role="menuitem"]').find((b) => b.textContent?.trim() === item);
  if (!found) {
    throw new Error(`no menu item ${item}`);
  }
  return found;
}

const commands = () => calls.map((c) => c.command);

beforeEach(() => {
  calls.length = 0;
});

describe('the Profiles screen (S5)', () => {
  it('lists the profiles with their markers, mods, installation and last apply', async () => {
    const { target } = await start();
    expect(all(target, '.summary span').map((s) => s.textContent)).toEqual([
      '4 profiles',
      '·',
      'In the game now: “Default” on D:, “Sasha” on E:',
    ]);
    const def = row(target, 'Default');
    expect(def.classList).toContain('is-sel');
    expect(all(def, '.pf-mark').map((m) => plain(m.textContent))).toEqual(['Selected', 'Applied']);
    const meta = plain(one(def, '.pf-meta').textContent);
    expect(meta.startsWith('4 mods enabled')).toBe(true);
    expect(meta).toContain('Deadlock');
    expect(meta.endsWith('Applied 2 hours ago')).toBe(true);
    expect(all(row(target, 'Sasha'), '.pf-mark').map((m) => plain(m.textContent))).toEqual(['Applied']);
    expect(plain(one(row(target, 'Evening'), '.pf-meta').textContent)).toContain('Last applied 3 days ago');
    expect(row(target, 'Default').querySelector('button.select')).toBeNull();
  });

  it('selects a profile without applying it, then applies it from the banner', async () => {
    const { app, target } = await start();
    click(one(row(target, 'Evening'), 'button.select'));
    await vi.waitFor(() => expect(target.querySelector('.pending')).not.toBeNull());
    expect(commands()).toContain('select_profile');
    expect(commands()).not.toContain('apply');
    const banner = one(target, '.pending');
    expect(plain(banner.textContent)).toContain('Profile “Evening” isn’t in the game yet.');
    expect(plain(banner.textContent)).toContain('still has “Default”');
    const apply = all(banner, 'button').find((b) => b.textContent?.includes('Apply “Evening”'));
    click(apply as HTMLElement);
    await vi.waitFor(() => expect(commands()).toContain('apply'));
    await vi.waitFor(() => expect(app.toasts.at(-1)?.title).toMatch(/^Profile “Evening” applied/));
    expect(app.library?.applied_profile).toBe('Evening');
  });

  it('renames in place, refusing a name that is taken', async () => {
    const { target } = await start();
    click(menuItem(target, 'Ranked', 'Rename'));
    const input = one<HTMLInputElement>(target, 'input[data-rename]');
    input.value = 'evening';
    fire(input, 'input');
    keydown(input, 'Enter');
    expect(plain(one(target, '.dl-field__error').textContent)).toBe('A profile named “evening” already exists. Choose another name.');
    expect(commands()).not.toContain('rename_profile');
    input.value = 'Practice';
    fire(input, 'input');
    keydown(input, 'Enter');
    await vi.waitFor(() => expect(calls.find((c) => c.command === 'rename_profile')?.args).toEqual({ old: 'Ranked', new: 'Practice' }));
    await vi.waitFor(() => expect(() => row(target, 'Practice')).not.toThrow());
  });

  it('duplicates under a free name and offers to rename the copy', async () => {
    const { target } = await start();
    click(menuItem(target, 'Evening', 'Duplicate'));
    await vi.waitFor(() =>
      expect(calls.find((c) => c.command === 'duplicate_profile')?.args).toEqual({ name: 'Evening', newName: 'Evening (copy)' }),
    );
    await vi.waitFor(() => expect(one<HTMLInputElement>(target, 'input[data-rename]').value).toBe('Evening (copy)'));
  });

  it('deletes a profile that isn’t applied at once, with Undo', async () => {
    const { app, target } = await start();
    click(menuItem(target, 'Ranked', 'Delete'));
    await vi.waitFor(() => expect(commands()).toContain('delete_profile'));
    expect(commands()).toContain('export_profile');
    const toast = app.toasts.at(-1);
    expect(toast?.title).toBe('Profile “Ranked” deleted.');
    expect(toast?.action?.label).toBe('Undo');
  });

  it('asks before deleting the applied profile and says what leaves the game', async () => {
    const { target } = await start();
    click(menuItem(target, 'Default', 'Delete…'));
    const dialog = one(document.body, '.dl-dialog');
    expect(one(dialog, '.dl-dialog__title').textContent).toBe('Delete profile “Default”?');
    expect(plain(dialog.textContent)).toContain('deadloader will remove its 4 mods from the game folder:');
    expect(plain(one(dialog, '.dl-url').textContent)).toMatch(/Deadlock\\game\\citadel\\addons$/);
    expect(commands()).not.toContain('delete_profile');
    click(one(dialog, '.dl-btn--danger'));
    await vi.waitFor(() => expect(commands()).toContain('delete_profile'));
  });

  it('creates a profile as a copy of the selected one, in its installation', async () => {
    const { target } = await start();
    click(all(target, '.header button').find((b) => b.textContent?.includes('New profile')) as HTMLElement);
    const dialog = one(document.body, '.dl-dialog');
    const name = one<HTMLInputElement>(dialog, 'input[type="text"]');
    name.value = 'Practice';
    fire(name, 'input');
    click(all<HTMLInputElement>(dialog, 'input[type="radio"]')[1] as HTMLElement);
    click(one(dialog, '.dl-dialog__foot .dl-btn--primary'));
    await vi.waitFor(() =>
      expect(calls.find((c) => c.command === 'duplicate_profile')?.args).toEqual({ name: 'Default', newName: 'Practice' }),
    );
    await vi.waitFor(() => expect(commands()).toContain('set_profile_installation'));
  });

  it('moves a profile to another game installation', async () => {
    const { target } = await start();
    click(menuItem(target, 'Evening', 'Change game installation…'));
    const dialog = one(document.body, '.dl-dialog');
    expect(one(dialog, '.dl-dialog__title').textContent).toBe('Game installation for “Evening”');
    const options = all<HTMLInputElement>(dialog, 'input[type="radio"]');
    expect(options).toHaveLength(2);
    click(options[1] as HTMLElement);
    click(one(dialog, '.dl-dialog__foot .dl-btn--primary'));
    await vi.waitFor(() => expect(calls.find((c) => c.command === 'set_profile_installation')?.args.name).toBe('Evening'));
    expect(String(calls.find((c) => c.command === 'set_profile_installation')?.args.root)).toMatch(/^E:/);
  });

  it('explains an import error in D12, with another file to choose', async () => {
    const { app, target } = await start('profiles-import-error');
    click(all(target, '.header button').find((b) => b.textContent?.includes('Import')) as HTMLElement);
    await vi.waitFor(() => expect(app.error?.message.id).toBe('E17'));
    expect(app.error?.actions.map((a) => a.label)).toEqual(['Check for deadloader update', 'Choose another file']);
  });

  it('lists missing mods in D9 and downloads the ones GameBanana has', async () => {
    const { app, target } = await start('profiles-missing-mods');
    click(all(target, '.header button').find((b) => b.textContent?.includes('Import')) as HTMLElement);
    await vi.waitFor(() => expect(document.body.querySelector('.dl-dialog')).not.toBeNull());
    const dialog = one(document.body, '.dl-dialog');
    expect(one(dialog, '.dl-dialog__title').textContent).toBe('3 mods from “Max ranked” aren’t in your library');
    expect(all(dialog, '.pf-miss')).toHaveLength(3);
    expect(plain(dialog.textContent)).toContain('lists 8 mods. 5 are already in your library; these 3 aren’t:');
    const download = one(dialog, '.dl-dialog__foot .dl-btn--primary');
    expect(plain(download.textContent)).toBe('Download missing (2)');
    click(download);
    await vi.waitFor(() => expect(commands()).toContain('profile_download_missing'));
    const mods = calls.find((c) => c.command === 'profile_download_missing')?.args.mods as unknown[];
    expect(mods).toHaveLength(2);
    await vi.waitFor(() => expect(app.toasts.at(-1)?.title).toBe('Profile “Max ranked” imported — 2 mods are downloading.'));
  });

  it('cancelling D9 undoes the import', async () => {
    const { target } = await start('profiles-missing-mods');
    click(all(target, '.header button').find((b) => b.textContent?.includes('Import')) as HTMLElement);
    await vi.waitFor(() => expect(document.body.querySelector('.dl-dialog')).not.toBeNull());
    const cancel = all(document.body, '.dl-dialog__foot button').find((b) => b.textContent?.trim() === 'Cancel');
    click(cancel as HTMLElement);
    await vi.waitFor(() => expect(calls.find((c) => c.command === 'delete_profile')?.args).toEqual({ name: 'Max ranked' }));
    await vi.waitFor(() => expect(calls.find((c) => c.command === 'select_profile')?.args).toEqual({ name: 'Default' }));
    flushSync();
  });
});
