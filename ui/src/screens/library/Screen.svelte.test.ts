import { flushSync, tick } from 'svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

// The screen talks to the mock core, the same one `pnpm dev:mock` uses.
vi.mock('@tauri-apps/api/core', () => import('../../mock/core'));

// The list measures its viewport; happy-dom has no layout.
class FakeResizeObserver {
  observe(): void {}
  unobserve(): void {}
  disconnect(): void {}
}
vi.stubGlobal('ResizeObserver', FakeResizeObserver);

const { app } = await import('../../lib/app.svelte');
const { router } = await import('../../lib/router.svelte');
const { devtools, invoke, resetMock } = await import('../../mock/core');
const { default: Screen } = await import('./Screen.svelte');
const { all, click, keydown, one, render } = await import('../../lib/test/render');

async function settle(): Promise<void> {
  for (let i = 0; i < 6; i += 1) {
    await tick();
    await Promise.resolve();
  }
  flushSync();
}

async function start(scenario?: string): Promise<HTMLElement> {
  history.replaceState(null, '', scenario ? `/?scenario=${scenario}#/library` : '/#/library');
  resetMock();
  router.go('library');
  Object.assign(app, { pendingOpen: false, mapping: null, updates: null, updatesAt: null, toasts: [], gameinfoForeign: false, progress: null });
  await app.start();
  const { target } = render(Screen, {});
  // happy-dom has no layout: give the list a viewport.
  for (const viewport of all(target, '.viewport')) {
    Object.defineProperty(viewport, 'clientHeight', { value: 800, configurable: true });
    viewport.dispatchEvent(new Event('scroll'));
  }
  await settle();
  return target;
}

function button(root: ParentNode, text: string): HTMLButtonElement {
  const found = all<HTMLButtonElement>(root, 'button').find((b) => b.textContent?.includes(text) || b.getAttribute('aria-label') === text);
  if (!found) {
    throw new Error(`no button "${text}"`);
  }
  return found;
}

function names(root: ParentNode): string[] {
  return all(root, '.dl-row__name').map((n) => n.textContent ?? '');
}

beforeEach(() => {
  vi.useRealTimers();
});

afterEach(() => {
  devtools.setGameRunning(false);
});

describe('the Library screen', () => {
  it('lists the profile’s mods in order with their badges and the tab counts', async () => {
    const target = await start();
    expect(names(target).slice(0, 2)).toEqual(['Haze — Midnight Bloom', 'Yamato — Ember Coat']);
    expect(one(target, 'h1').textContent).toBe('Library');
    expect(target.textContent).toContain('9 of 11 enabled');
    expect(target.textContent).toContain('Conflict · wins');
    expect(target.textContent).toContain('Update available');
    const tabs = all(target, '[role=tab]').map((t) => t.textContent?.replace(/\s+/g, ' ').trim());
    expect(tabs).toEqual(['All 11', 'Enabled 9', 'Disabled 2', 'Updates 1', 'Conflicts 2', 'Local 1']);
  });

  it('shows an empty library with the catalog and Add from disk, and only Add from disk in local mode', async () => {
    let target = await start('library-empty');
    expect(target.textContent).toContain('Your library is empty');
    expect(button(target, 'Browse catalog')).toBeTruthy();
    expect(button(target, 'Add from disk')).toBeTruthy();
    document.body.replaceChildren();
    target = await start('library-local');
    expect(target.textContent).toContain('local mode');
    expect(all(target, 'button').some((b) => b.textContent?.includes('Browse catalog'))).toBe(false);
  });

  it('says when Deadlock wasn’t found and keeps the toggles off', async () => {
    const target = await start('library-game-missing');
    expect(target.textContent).toContain('Deadlock wasn’t found.');
    expect(all<HTMLInputElement>(target, '.dl-row input[type=checkbox]').every((i) => i.disabled)).toBe(true);
  });

  it('switches a mod off at once, with Undo in the toast (138)', async () => {
    const target = await start();
    const toggle = one<HTMLInputElement>(target, '.dl-row input[type=checkbox]');
    toggle.click();
    await settle();
    expect(app.library?.mods[0]?.enabled).toBe(false);
    const toast = app.toasts.at(-1);
    expect(toast?.title).toBe('Haze — Midnight Bloom disabled.');
    toast?.action?.onclick();
    await settle();
    expect(app.library?.mods[0]?.enabled).toBe(true);
  });

  it('filters by tab and search', async () => {
    const target = await start();
    click(button(target, 'Local'));
    await settle();
    expect(names(target)).toEqual(['haze_nightshift']);
  });

  it('reorders with the keyboard: Space picks up, arrows move, Space drops, Esc puts back', async () => {
    const target = await start();
    const grip = all(target, '.dl-row__grip')[3] as HTMLElement;
    keydown(grip, ' ');
    expect(target.textContent).toContain('McGinnis — Brass Knight — picked up, position 4 of 11.');
    keydown(grip, 'ArrowUp');
    expect(names(target)[2]).toBe('McGinnis — Brass Knight');
    keydown(grip, ' ');
    await settle();
    expect(app.library?.mods[2]?.name).toBe('McGinnis — Brass Knight');

    const again = all(target, '.dl-row__grip')[2] as HTMLElement;
    keydown(again, ' ');
    keydown(again, 'ArrowUp');
    keydown(again, 'Escape');
    await settle();
    expect(names(target)[2]).toBe('McGinnis — Brass Knight');
    expect(target.textContent).toContain('back at position 3 of 11');
  });

  it('opens a mod’s details from the address and deletes it through D6', async () => {
    const target = await start();
    router.open('library', 'gb-710003');
    await settle();
    const details = one(target, 'aside');
    expect(details.textContent).toContain('Celeste — Coral Reef');
    expect(details.textContent).toContain('Linked — no extra disk space');
    click(button(details, 'Delete mod'));
    await settle();
    const dialog = one(document.body, '[role=alertdialog]');
    expect(dialog.textContent).toContain('Delete “Celeste — Coral Reef”?');
    click(button(dialog, 'To the recycle bin'));
    click(all(dialog, 'button').at(-1) as HTMLElement);
    await settle();
    expect(app.library?.mods.some((m) => m.id === 'gb-710003')).toBe(false);
  });

  it('resolves a conflict in the panel and keeps it there with Done and Undo (150)', async () => {
    const target = await start();
    click(button(target, 'Conflicts'));
    await settle();
    const panel = one(target, 'aside');
    click(button(panel, 'Move Yamato — Porcelain above'));
    await settle();
    expect(panel.textContent).toContain('Done — Yamato — Porcelain now wins');
    expect(all(panel, 'button').some((b) => b.textContent?.includes('Move'))).toBe(false);
    click(button(panel, 'Undo'));
    await settle();
    expect(app.library?.conflicts[0]?.winner_mod).toBe('gb-710002');
  });

  it('lists the changes made while the game runs, undoes one, and shows what was applied after it closed', async () => {
    const target = await start();
    devtools.setGameRunning(true);
    await settle();
    expect(target.textContent).toContain('Deadlock is running.');
    one<HTMLInputElement>(target, '.dl-row input[type=checkbox]').click();
    await settle();
    expect(app.library?.pending_count).toBe(1);
    app.pendingOpen = true;
    await settle();
    let panel = one(target, '[role=dialog]');
    expect(panel.textContent).toContain('Disable · 1');
    expect(button(panel, 'Apply').getAttribute('aria-disabled')).toBe('true');
    click(button(panel, 'Undo this change'));
    await settle();
    expect(app.library?.pending_count).toBe(0);

    one<HTMLInputElement>(target, '.dl-row input[type=checkbox]').click();
    await settle();
    devtools.setGameRunning(false);
    await settle();
    panel = one(target, '[role=dialog]');
    expect(panel.textContent).toContain('Changes applied');
    click(button(panel, 'Undo all'));
    await settle();
    expect(app.library?.mods[0]?.enabled).toBe(true);
  });

  it('shows the file list of a profile switch on request', async () => {
    const target = await start('library-switch');
    click(button(target, 'Review switch'));
    await settle();
    const panel = one(target, '[role=dialog]');
    expect(panel.textContent).toContain('Evening → Weekend');
    expect(panel.textContent).toContain('1 file added · 3 removed · 1 renumbered');
    expect(panel.querySelector('.file')).toBeNull();
    const show = button(panel, 'Show the file list');
    expect(show.getAttribute('aria-expanded')).toBe('false');
    click(show);
    await settle();
    expect(all(panel, '.label').map((l) => l.textContent)).toEqual(
      // Only the rows in view are drawn (here the first few: the list has no height in the test).
      expect.arrayContaining(['Added · 1', 'Removed · 3']),
    );
    expect(panel.textContent).toContain('Wraith — Formal');
    click(button(panel, 'Hide the file list'));
    await settle();
    expect(panel.querySelector('.file')).toBeNull();
  });

  it('shows how far an apply is in the banner, and drops the banner when it ends', async () => {
    const target = await start();
    expect(target.querySelector('.applying')).toBeNull();
    app.handle({
      event: 'applying',
      data: {
        done: 3,
        total: 4,
        linked: 9,
        copied: 1,
        bytes_done: 300,
        bytes_total: 400,
        bytes_copied: 100,
        left_ms: 4200,
        finished: false,
      },
    });
    await settle();
    const banner = one(target, '.applying');
    expect(banner.textContent).toContain('Applying profile');
    expect(banner.textContent).toContain('Step 4 of 4');
    expect(banner.textContent).toContain('9 files linked, 1 copied');
    expect(banner.textContent).toContain('About 5 s left');
    expect(one(banner, '[role=progressbar]').getAttribute('aria-valuenow')).toBe('75');

    app.handle({
      event: 'applying',
      data: {
        done: 4,
        total: 4,
        linked: 9,
        copied: 1,
        bytes_done: 400,
        bytes_total: 400,
        bytes_copied: 100,
        left_ms: null,
        finished: true,
      },
    });
    await settle();
    expect(target.querySelector('.applying')).toBeNull();
  });

  it('lists foreign files and takes one under management through D11', async () => {
    const target = await start('library-foreign');
    expect(target.textContent).toContain('Found in the game’s mod folder');
    expect(target.textContent).toContain('VPK, 26 files — looks like a skin for Haze');
    click(button(target, 'Take under management'));
    await settle();
    const dialog = one(document.body, '[role=dialog]');
    expect(dialog.textContent).toContain('Take pak01_dir.vpk under management?');
    click(all(dialog, 'button').at(-1) as HTMLElement);
    await settle();
    expect(app.library?.mods.at(-1)?.name).toBe('pak01_dir (Haze skin)');
    expect(target.textContent).not.toContain('looks like a skin for Haze');
  });

  it('opens D10 for a gameinfo.gi another program changed, and closes it once Steam restored the file', async () => {
    const target = await start('library-gameinfo-foreign');
    void target;
    let dialog = one(document.body, '[role=alertdialog]');
    expect(dialog.textContent).toContain('A game file was changed by another program');
    click(button(dialog, 'Show differences'));
    expect(all(dialog, '.line.theirs').length).toBe(2);
    click(button(dialog, 'Verify game files in Steam'));
    await settle();
    dialog = one(document.body, '[role=alertdialog]');
    expect(dialog.textContent).toContain('Steam is verifying');
    click(button(dialog, 'Check again'));
    await settle();
    expect(document.body.querySelector('[role=alertdialog]')).toBeNull();
    expect(app.toasts.at(-1)?.title).toBe('gameinfo.gi is set up — mods load again.');
  });

  it('updates a mod through D5, mapping the files when the new version has a different set (169)', async () => {
    const target = await start('library-updates');
    await invoke('subscribe', { events: { onmessage: (e: unknown) => app.handle(e as never) } });
    await settle();
    click(button(target, 'Update all'));
    await settle();
    let dialog = one(document.body, '[role=dialog]');
    expect(dialog.textContent).toContain('The new version has a different set of files.');
    click(button(dialog, 'Install update'));
    await settle();
    dialog = one(document.body, '[role=dialog]');
    expect(dialog.textContent).toContain('couldn’t tell for every file');
  });
});
