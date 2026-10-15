import { flushSync, tick } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import wizardCss from './wizard.css?raw';

// The wizard talks to the mock core, the same one `pnpm dev:mock` uses.
vi.mock('@tauri-apps/api/core', () => import('../mock/core'));

const { default: Wizard } = await import('./Wizard.svelte');
const { default: NoWrite } = await import('./NoWrite.svelte');
const { app } = await import('../lib/app.svelte');
const { shell } = await import('../lib/api');
const { invoke, resetMock } = await import('../mock/core');
const { NOTICE } = await import('../lib/notice.svelte');
const { render, click, all, one } = await import('../lib/test/render');
const { setLanguage } = await import('../i18n');

async function settle(): Promise<void> {
  for (let i = 0; i < 6; i++) {
    await tick();
    await Promise.resolve();
  }
  flushSync();
}

function button(text: string): HTMLButtonElement {
  const found = all<HTMLButtonElement>(document.body, 'button').find((b) => b.textContent?.trim() === text);
  if (!found) {
    throw new Error(`no button "${text}"`);
  }
  return found;
}

async function start(scenario: string): Promise<void> {
  history.replaceState(null, '', `/?scenario=${scenario}`);
  resetMock();
  app.toasts = [];
  app.settings = await invoke('settings');
  app.library = await invoke('library');
}

const title = () => one(document.body, '#wz-title').textContent;

/** The declarations of a top-level rule of a style sheet, e.g. `display:flex;min-width:0`. */
function rule(css: string, selector: string): string {
  const start = css.split('\n').find((line) => line.startsWith(`${selector}{`));
  return start?.slice(selector.length + 1, start.indexOf('}')) ?? '';
}

const text = () => document.body.textContent ?? '';

/** Walks a first start to the step titled `last`, choosing `choice` on the network step. */
async function walkTo(last: string, choice: 'catalog' | 'local' = 'catalog'): Promise<void> {
  click(button('Next'));
  click(one(document.body, '.wz-consent input'));
  click(button('Next'));
  click(one(document.body, `input[value="${choice}"]`));
  while (title() !== last) {
    click(button('Next'));
    await settle();
  }
}

describe('the first-run wizard', () => {
  it('needs the acknowledgement and a choice, then saves the notice and opens the library in local mode', async () => {
    await start('wizard');
    const ondone = vi.fn();
    render(Wizard, { app, mode: { kind: 'first' }, ondone });
    expect(title()).toBe('Welcome to deadloader');
    click(button('Next'));
    expect(title()).toBe('Mods are at your own risk');
    // Next explains why it waits, and says it again under the box when pressed.
    expect(text()).toContain('Tick the box first.');
    click(button('Next'));
    await settle();
    expect(text()).toContain('Tick this box to continue.');
    expect(title()).toBe('Mods are at your own risk');
    click(one(document.body, '.wz-consent input'));
    click(button('Next'));
    expect(title()).toBe('Network and GameBanana');
    click(button('Next'));
    await settle();
    expect(text()).toContain('Choose one of the two options.');
    click(one(document.body, 'input[value="local"]'));
    click(button('Next'));
    expect(text()).toContain('You chose local mode, so deadloader sends nothing at all.');
    click(button('Next'));
    // 157: no update check in local mode.
    expect(one<HTMLInputElement>(document.body, 'input[role="switch"]').disabled).toBe(true);
    click(button('Next'));
    await settle();
    expect(title()).toBe('Setup');
    expect(text()).toContain('Deadlock found');
    expect(text()).toContain('Opens your library next (local mode).');
    click(button('Finish setup'));
    await settle();
    expect(ondone).toHaveBeenCalledWith('library');
    const notice = (app.settings?.acknowledgements as { notice: { version: number; catalog: string } }).notice;
    expect(notice).toMatchObject({ version: NOTICE.version, catalog: 'local_mode' });
  });

  it('turns the catalog and the update check on when both are chosen, then opens the catalog', async () => {
    await start('wizard');
    const ondone = vi.fn();
    render(Wizard, { app, mode: { kind: 'first' }, ondone });
    await walkTo('Updates of deadloader');
    const updates = one<HTMLInputElement>(document.body, 'input[role="switch"]');
    expect(updates.disabled).toBe(false);
    expect(one(document.body, '#wz-upd-state').textContent).toBe('Off (default)');
    click(updates);
    expect(one(document.body, '#wz-upd-state').textContent).toBe('On');
    click(button('Next'));
    await settle();
    expect(text()).toContain('Opens the catalog next.');
    click(button('Finish setup'));
    await settle();
    expect(ondone).toHaveBeenCalledWith('catalog');
    expect(app.settings?.catalog?.enabled).toBe(true);
    expect(app.settings?.general?.check_app_updates).toBe(true);
  });

  it('lets finished steps be revisited but not later ones', async () => {
    await start('wizard');
    render(Wizard, { app, mode: { kind: 'first' }, ondone: vi.fn() });
    const steps = all<HTMLButtonElement>(document.body, '.wz-step');
    click(steps[3] as HTMLButtonElement);
    expect(title()).toBe('Welcome to deadloader');
    click(button('Next'));
    click(steps[0] as HTMLButtonElement);
    expect(title()).toBe('Welcome to deadloader');
  });

  it('switches its language on every step and keeps the choice', async () => {
    await start('wizard');
    render(Wizard, { app, mode: { kind: 'first' }, ondone: vi.fn() });
    click(one(document.body, '.wz-lang .dl-select__trigger'));
    const deutsch = all(document.body, '.wz-lang [role="option"]').find((o) => o.textContent?.trim() === 'Deutsch');
    click(deutsch as HTMLElement);
    try {
      // The German texts load on demand.
      await vi.waitFor(() => {
        flushSync();
        expect(title()).toBe('Willkommen bei deadloader');
      });
      expect(app.settings?.general?.language).toBe('de');
    } finally {
      await setLanguage('en');
    }
  });

  it('shows what the setup step found and what to do about each (setup with issues)', async () => {
    await start('wizard-setup-issues');
    render(Wizard, { app, mode: { kind: 'first' }, ondone: vi.fn() });
    await walkTo('Setup');
    await settle();
    expect(text()).toContain('Deadlock wasn’t found');
    click(button('Help'));
    expect(text()).toContain(String.raw`steamapps\common\Deadlock`);
    expect(text()).toContain('deadloader is in your Downloads folder');
    click(button('Open folder'));
    await settle();
    expect(text()).toContain('Opened in Explorer');
    expect(text()).toContain('Mods will be copied, not linked');
    expect(text()).toContain('deadloader is on C: and the game is on E:');
    expect(text()).toContain('3 files from another source');
    click(button('Show files'));
    expect(all(document.body, '.wz-row__more li').map((li) => li.firstElementChild?.textContent)).toEqual([
      'pak01_dir.vpk',
      'pak02_dir.vpk',
      '.dmm.json',
    ]);
    expect(text()).toContain('Changed by another program');
    click(button('What is this?'));
    expect(text()).toContain('verifying the game files in Steam');
    click(button('Browse…'));
    await settle();
    expect(text()).toContain('Deadlock found');
    expect(text()).toContain('Folder chosen');
  });

  it('shows only the changed step of a newer notice, with what changed', async () => {
    await start('wizard-changed');
    const ondone = vi.fn();
    const previous = { version: 1, at: '2026-09-02T10:00:00Z', language: 'en', catalog: 'catalog' as const };
    render(Wizard, { app, mode: { kind: 'changed', changes: NOTICE.changes, previous }, ondone });
    expect(title()).toBe('Network and GameBanana');
    expect(text()).toContain('files.gamebanana.com redirects downloads there');
    expect(text()).toContain('Your previous choice');
    // The host the change added is marked in the list.
    const marked = all(document.body, '.wz-hosts li').filter((li) => li.querySelector('.dl-badge'));
    expect(marked.map((li) => [li.querySelector('.dl-mono')?.textContent, li.querySelector('.dl-badge')?.textContent])).toEqual([
      ['filecacheNN.gamebanana.com', 'New'],
    ]);
    click(one(document.body, 'input[value="catalog"]'));
    click(button('Continue'));
    await settle();
    expect(ondone).toHaveBeenCalledWith(null);
    const notice = (app.settings?.acknowledgements as { notice: { version: number } }).notice;
    expect(notice.version).toBe(2);
  });

  it('turns the catalog on from Settings only after its network step (F13)', async () => {
    await start('settings-local');
    const ondone = vi.fn();
    render(Wizard, { app, mode: { kind: 'catalog' }, ondone });
    expect(all(document.body, '.wz-step')).toHaveLength(6);
    click(one(document.body, 'input[value="catalog"]'));
    click(button('Continue'));
    await settle();
    expect(ondone).toHaveBeenCalledWith('catalog');
    expect(app.settings?.catalog?.enabled).toBe(true);
  });

  it('leaves the catalog off when its network step is cancelled or answered with local mode', async () => {
    await start('settings-local');
    const cancelled = vi.fn();
    render(Wizard, { app, mode: { kind: 'catalog' }, ondone: cancelled });
    click(button('Cancel'));
    expect(cancelled).toHaveBeenCalledWith(null);
    document.body.replaceChildren();
    const declined = vi.fn();
    render(Wizard, { app, mode: { kind: 'catalog' }, ondone: declined });
    click(one(document.body, 'input[value="local"]'));
    click(button('Continue'));
    await settle();
    expect(declined).toHaveBeenCalledWith(null);
    expect(app.settings?.catalog?.enabled).toBe(false);
  });

  it('can be read again without changing anything', async () => {
    await start('settings-local');
    const ondone = vi.fn();
    render(Wizard, { app, mode: { kind: 'review' }, ondone });
    click(all<HTMLButtonElement>(document.body, '.wz-step')[2] as HTMLButtonElement);
    expect(title()).toBe('Network and GameBanana');
    expect(one<HTMLInputElement>(document.body, 'input[value="local"]').checked).toBe(true);
    expect(one<HTMLInputElement>(document.body, 'input[value="catalog"]').disabled).toBe(true);
  });
});

describe('E1, no write permission', () => {
  const props = {
    folder: String.raw`C:\Program Files\deadloader`,
    detail: String.raw`Access is denied. · C:\Program Files\deadloader\deadloader-data`,
  };

  it('says where deadloader is and what to do', () => {
    resetMock();
    const { target } = render(NoWrite, props);
    expect(target.textContent).toContain('deadloader can’t save its data.');
    expect(target.textContent).toContain('Not saved to the log');
    expect(one(target, '.dl-path__tip').textContent).toBe(String.raw`C:\Program Files\deadloader`);
  });

  it('keeps its column centred: a long folder path is cut, never widening the column (E1 board)', () => {
    resetMock();
    const folder = String.raw`C:\Users\Alex\AppData\Local\Temp\a-very-long-folder-name-0123456789abcdef0123456789abcdef\deadloader`;
    const { target } = render(NoWrite, { ...props, folder });
    // The column sits alone in the stage, which centres it (`place-items`), at most 620 px wide.
    const column = one(target, '.wz-stage > .wz-nw');
    expect(rule(wizardCss, '.wz-nw')).toMatch(/max-width:620px/);
    expect(rule(wizardCss, '.wz-stage')).toMatch(/place-items:safe center/);
    expect(column.children).not.toHaveLength(0);
    // The board's path box: a flex row that may shrink, so the path's minimum width is not the whole path's.
    const path = one(column, '.wz-well .dl-path');
    expect(path.parentElement?.className).toBe('wz-nw-path');
    expect(rule(wizardCss, '.wz-nw-path')).toBe('display:flex;min-width:0');
  });

  it('copies the details, opens the folder and exits', async () => {
    history.replaceState(null, '', '/?scenario=wizard-no-write');
    resetMock();
    const writeText = vi.fn(() => Promise.resolve());
    Object.defineProperty(navigator, 'clipboard', { value: { writeText }, configurable: true });
    const close = vi.spyOn(shell, 'close');
    const { target } = render(NoWrite, props);
    click(one(target, 'button[aria-label="Copy details"]'));
    await settle();
    expect(writeText).toHaveBeenCalledWith(props.detail);
    expect(target.textContent).toContain('Copied');
    click(button('Open folder'));
    await settle();
    expect(target.textContent).toContain('Opened in Explorer');
    click(button('Exit'));
    expect(close).toHaveBeenCalled();
    close.mockRestore();
  });
});
