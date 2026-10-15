import { flushSync, tick } from 'svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

// The screen talks to the mock core, the same one `pnpm dev:mock` uses.
vi.mock('@tauri-apps/api/core', () => import('../../mock/core'));

const { default: Screen } = await import('./Screen.svelte');
const { app } = await import('../../lib/app.svelte');
const { invoke, resetMock } = await import('../../mock/core');
const { render, click, all, one } = await import('../../lib/test/render');

/** Lets the mock core answer and the screen render what came back. */
async function settle(): Promise<void> {
  for (let i = 0; i < 6; i++) {
    await tick();
    await Promise.resolve();
  }
  flushSync();
}

function button(root: ParentNode, text: string): HTMLButtonElement {
  const found = all<HTMLButtonElement>(root, 'button').find((b) => b.textContent?.trim() === text);
  if (!found) {
    throw new Error(`no button "${text}"`);
  }
  return found;
}

async function open(section: string, scenario?: string): Promise<HTMLElement> {
  history.replaceState(null, '', `${scenario ? `/?scenario=${scenario}` : '/'}#/settings/${section}`);
  resetMock();
  app.toasts = [];
  app.settings = await invoke('settings');
  app.library = await invoke('library');
  const { target } = render(Screen, {});
  await settle();
  return target;
}

describe('the Settings screen', () => {
  beforeEach(() => {
    history.replaceState(null, '', '/');
  });

  it('lists the sections and shows the one in the address', async () => {
    const target = await open('network', 'settings-local');
    expect(all(target, '.st-nav .dl-nav-item')).toHaveLength(11);
    expect(one(target, '.st-head h2').textContent).toBe('Network');
    expect(target.textContent).toContain('Local mode is on.');
    click(button(target, 'Cache'));
    await settle();
    expect(one(target, '.st-head h2').textContent).toBe('Cache');
    expect(location.hash).toBe('#/settings/cache');
  });

  it('saves a switch through the core and offers the default back', async () => {
    const target = await open('catalog');
    const prefetch = all<HTMLInputElement>(target, 'input[role="switch"]').find(
      (i) => i.getAttribute('aria-label') === 'Load the next page in advance',
    );
    expect(prefetch?.checked).toBe(true);
    prefetch?.click();
    await settle();
    expect((await invoke<{ catalog: { prefetch_next_page: boolean } }>('settings')).catalog.prefetch_next_page).toBe(false);
    expect(target.textContent).toContain('Default: On');
    click(button(target, 'Restore default'));
    await settle();
    expect((await invoke<{ catalog: { prefetch_next_page: boolean } }>('settings')).catalog.prefetch_next_page).toBe(true);
  });

  it('asks for the network step before the catalog turns on (F13)', async () => {
    const target = await open('catalog', 'settings-local');
    const toggle = all<HTMLInputElement>(target, 'input[role="switch"]')[0];
    toggle?.click();
    await settle();
    expect(target.textContent).toContain('One step first.');
    // The switch stays off until the user agrees in the wizard.
    expect(all<HTMLInputElement>(target, 'input[role="switch"]')[0]?.checked).toBe(false);
    expect((await invoke<{ catalog: { enabled: boolean } }>('settings')).catalog.enabled).toBe(false);
  });

  it('rolls the game folder back through D7 and says what happened', async () => {
    const target = await open('game');
    click(button(target, 'Roll back…'));
    await settle();
    const dialog = one(document.body, '[role="alertdialog"]');
    expect(dialog.textContent).toContain('10 mod files it added in');
    expect(dialog.textContent).toContain('Also remove 2 files that deadloader didn’t create');
    click(button(dialog, 'Roll back'));
    await settle();
    expect(document.body.querySelector('[role="alertdialog"]')).toBeNull();
    expect(target.textContent).toContain('Game folder on D: rolled back.');
  });

  it('unlocks deleting all data only after the word is typed (D8)', async () => {
    const target = await open('game');
    click(button(target, 'Delete all data…'));
    await settle();
    const dialog = one(document.body, '[role="alertdialog"]');
    const confirm = button(dialog, 'Roll back, delete and close');
    expect(confirm.disabled).toBe(true);
    const input = one<HTMLInputElement>(dialog, '#d8-word');
    input.value = 'delete';
    input.dispatchEvent(new Event('input', { bubbles: true }));
    flushSync();
    expect(confirm.disabled).toBe(false);
    expect(dialog.textContent).toContain('Confirmed. This can’t be undone.');
  });

  it('checks an added domain before saving it', async () => {
    const target = await open('advanced');
    const input = one<HTMLInputElement>(target, '#st-domain');
    input.value = 'https://files.example.com/x';
    input.dispatchEvent(new Event('input', { bubbles: true }));
    click(button(target, 'Add'));
    await settle();
    expect(target.textContent).toContain('Enter the host name only');
    input.value = 'files.example.com';
    input.dispatchEvent(new Event('input', { bubbles: true }));
    click(button(target, 'Add'));
    await settle();
    expect((await invoke<{ advanced: { extra_allowed_domains: string[] } }>('settings')).advanced.extra_allowed_domains).toEqual([
      'files.example.com',
    ]);
  });

  it('shows where the Advanced text is wrong and saves nothing', async () => {
    const target = await open('advanced', 'settings-advanced-error');
    click(button(target, 'Check and save'));
    await settle();
    expect(target.textContent).toContain('This isn’t valid JSON, so it wasn’t saved.');
    expect(all(target, '.st-excerpt .is-err')).toHaveLength(1);
  });

  it('resets all settings after asking, with Undo in the toast', async () => {
    const target = await open('general');
    click(button(target, 'Reset all settings…'));
    await settle();
    click(button(one(document.body, '[role="alertdialog"]'), 'Reset settings'));
    await settle();
    const toast = app.toasts.at(-1);
    expect(toast?.title).toBe('All settings reset to their defaults.');
    expect(toast?.action?.label).toBe('Undo');
  });
});
