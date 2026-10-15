import { flushSync } from 'svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

// The real mock core answers; every call is recorded on the way.
const calls = vi.hoisted(() => [] as [string, Record<string, unknown> | undefined][]);
vi.mock('@tauri-apps/api/core', async () => {
  const core = await import('../../mock/core');
  return {
    ...core,
    invoke: (command: string, args?: Record<string, unknown>) => {
      calls.push([command, args]);
      return core.invoke(command, args);
    },
  };
});

type Core = typeof import('../../mock/core');

// One app, one grid and one mock core for every test: each test starts them afresh.
const core = (await import('@tauri-apps/api/core')) as unknown as Core;
const { app } = await import('../../lib/app.svelte');
const { browser } = await import('./browse.svelte');
const { pathOf, routeOf, router } = await import('../../lib/router.svelte');
const r = await import('../../lib/test/render');
const { default: Screen } = await import('./Screen.svelte');

const size = { width: 1000, height: 700 };

function called(command: string) {
  return calls.filter(([name]) => name === command).map(([, args]) => args);
}

async function settle(): Promise<void> {
  for (let i = 0; i < 20; i += 1) {
    await Promise.resolve();
  }
  flushSync();
}

function button(root: ParentNode, text: string): HTMLElement {
  const found = [...root.querySelectorAll<HTMLElement>('button, a')].find((b) => b.textContent?.trim() === text);
  if (!found) {
    throw new Error(`no button “${text}”`);
  }
  return found;
}

async function open(hash: string, scenario: Parameters<Core['setCatalogScenario']>[0] = null) {
  location.hash = hash;
  router.current = routeOf(hash);
  router.path = pathOf(hash);
  core.resetMock();
  core.setCatalogScenario(scenario);
  browser.reset();
  app.catalogEvent = null;
  await app.start();
  calls.length = 0;
  const rendered = r.render(Screen, {});
  await settle();
  return rendered;
}

beforeEach(() => {
  vi.stubGlobal(
    'ResizeObserver',
    class {
      observe() {}
      unobserve() {}
      disconnect() {}
    },
  );
  vi.spyOn(HTMLElement.prototype, 'clientWidth', 'get').mockImplementation(function (this: HTMLElement) {
    return this.hasAttribute('data-virtual-viewport') ? size.width : 0;
  });
  vi.spyOn(HTMLElement.prototype, 'clientHeight', 'get').mockImplementation(function (this: HTMLElement) {
    return this.hasAttribute('data-virtual-viewport') ? size.height : 0;
  });
});

afterEach(() => {
  r.cleanup();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
  vi.useRealTimers();
  location.hash = '';
});

describe('the catalog (S2)', () => {
  it('shows the newest mods as cards with their state', async () => {
    const { target } = await open('#/catalog');
    expect(target.querySelector('h1')?.textContent).toBe('Catalog');
    const cards = target.querySelectorAll('article.dl-card');
    expect(cards.length).toBeGreaterThan(3);
    const first = cards[0] as HTMLElement;
    expect(first.querySelector('a.dl-card__link')?.getAttribute('href')).toBe('#/catalog/mod/700000');
    expect(first.querySelector('.dl-card__author')?.textContent).toBe('lumen · Skins → Haze');
    // Installed in the library: its version on the thumbnail.
    expect(first.querySelector('.dl-card__badges')?.textContent).toContain('v1.2');
    expect(target.textContent).toContain('NSFW hidden');
  });

  it('marks a favourite with the heart', async () => {
    const { target } = await open('#/catalog');
    const heart = target.querySelector<HTMLButtonElement>('article.dl-card .dl-card__fav');
    expect(heart?.getAttribute('aria-pressed')).toBe('false');
    heart?.click();
    await settle();
    expect(called('set_favourite')).toHaveLength(1);
    expect(target.querySelector('article.dl-card .dl-card__fav')?.getAttribute('aria-pressed')).toBe('true');
  });

  it('explains local mode and offers to turn the catalog on (F13)', async () => {
    location.search = '';
    const { target } = await open('#/catalog');
    const settings = await core.invoke<Record<string, Record<string, unknown>>>('settings');
    await core.invoke('update_settings', { settings: { ...settings, catalog: { ...settings.catalog, enabled: false } } });
    app.network = { kind: 'local_mode' };
    flushSync();
    expect(target.textContent).toContain('The catalog is off (local mode)');
    expect(target.querySelector('input[type="search"]')).toBeNull();
    const before = location.hash;
    r.click(button(target, 'Enable catalog'));
    // F13: the wizard's network step over the window, not a detour through Settings.
    const { notice } = await import('../../lib/notice.svelte');
    expect(notice.current).toEqual({ mode: 'catalog' });
    expect(location.hash).toBe(before);
    notice.close();
  });

  it('shows the API-changed state with the log entry (E4)', async () => {
    const { target } = await open('#/catalog', 'catalog-api-changed');
    const alert = target.querySelector('[role="alert"]');
    expect(alert?.textContent).toContain('The catalog is unavailable.');
    expect(alert?.textContent).toContain('net-20261015-1742-0193');
    expect(target.querySelector<HTMLInputElement>('input[type="search"]')?.disabled).toBe(true);
  });

  it('counts down while GameBanana asks to wait, keeping the saved cards (E5)', async () => {
    const { target } = await open('#/catalog', 'catalog-rate-limited');
    const banner = target.querySelector('.dl-banner--warning');
    expect(banner?.textContent).toContain('GameBanana is busy.');
    expect(banner?.textContent).toMatch(/retries automatically in 1[12] s/);
    expect(target.querySelectorAll('article.dl-card').length).toBeGreaterThan(0);
    r.click(button(target, 'Retry now'));
    await settle();
    expect(called('catalog_retry_now')).toHaveLength(1);
  });

  it('shows saved pages while offline with the time they were saved (N7)', async () => {
    const { target } = await open('#/catalog', 'catalog-offline');
    expect(target.querySelector('.dl-banner--neutral')?.textContent).toContain('Offline — showing the catalog saved');
    expect(target.querySelector<HTMLInputElement>('input[type="search"]')?.disabled).toBe(true);
    expect(target.querySelectorAll('article.dl-card').length).toBeGreaterThan(0);
  });

  it('shows skeleton cards while the first page loads', async () => {
    const { target } = await open('#/catalog', 'catalog-loading');
    expect(target.querySelectorAll('.skeletons .dl-card').length).toBe(12);
    expect(target.querySelector('[role="status"]')?.textContent).toContain('Loading mods from GameBanana…');
  });

  it('says when nothing matches and offers to clear the search', async () => {
    vi.useFakeTimers();
    const { target } = await open('#/catalog', 'catalog-empty');
    const input = target.querySelector<HTMLInputElement>('input[type="search"]') as HTMLInputElement;
    input.value = 'nightshift';
    input.dispatchEvent(new Event('input', { bubbles: true }));
    await vi.advanceTimersByTimeAsync(400);
    await settle();
    expect(target.textContent).toContain('No mods match “nightshift”');
    expect(target.querySelector('.dl-search__meta')?.textContent).toBe('0 results');
    r.click(button(target, 'Clear search'));
    await settle();
    expect(called('catalog_list').at(-1)?.query).toMatchObject({ filter: { kind: 'browse' } });
  });

  it('blurs mods with content ratings when asked, revealing one on click', async () => {
    const { target } = await open('#/catalog');
    const settings = await core.invoke<Record<string, Record<string, unknown>>>('confirm_age');
    app.settings = await core.invoke('update_settings', { settings: { ...settings, catalog: { ...settings.catalog, nsfw: 'blurred' } } });
    await settle();
    const blurred = target.querySelector('.dl-card__thumb--blurred');
    expect(blurred).not.toBeNull();
    expect(blurred?.textContent).toContain('Sensitive content');
    const link = blurred?.closest('article')?.querySelector<HTMLAnchorElement>('a.dl-card__link');
    link?.click();
    flushSync();
    expect(link?.closest('article')?.querySelector('.dl-card__thumb--blurred')).toBeNull();
  });

  it('uses no inline style attributes (the CSP blocks them)', () => {
    const sources = import.meta.glob<string>(['./*.svelte', '../../lib/dialogs/*.svelte'], {
      query: '?raw',
      import: 'default',
      eager: true,
    });
    const offenders = Object.entries(sources)
      .filter(([, source]) => /\sstyle="/.test(source))
      .map(([file]) => file);
    expect(offenders).toEqual([]);
  });
});

describe('the mod page (S3)', () => {
  it('shows the page with the install state of every file', async () => {
    const { target } = await open('#/catalog/mod/700000');
    expect(target.querySelector('h1')?.textContent).toBe('Haze — Midnight Bloom');
    expect(target.textContent).toContain('Installed · v1.2 · Default colours');
    const rows = [...target.querySelectorAll<HTMLElement>('.file')];
    expect(rows[0]?.querySelector('a')?.getAttribute('href')).toBe('#/library/gb-700000');
    expect(rows[1]?.textContent).toContain('Switch to');
    expect(rows[2]?.textContent).toContain('Install');
    expect(rows[0]?.textContent).toContain('Clean');
    expect(rows[3]?.textContent).toContain('Not scanned yet');
  });

  it('switches to a downloaded variant and installs a new file (F2)', async () => {
    const { target } = await open('#/catalog/mod/700000');
    const rows = () => [...target.querySelectorAll<HTMLElement>('.file')];
    r.click(button(rows()[1] as HTMLElement, 'Switch to'));
    await settle();
    expect(called('switch_variant')).toEqual([{ modId: 'gb-700000', file: 'gb-700000-1-1' }]);
    expect(rows()[1]?.textContent).toContain('Installed');
    r.click(button(rows()[2] as HTMLElement, 'Install'));
    await settle();
    expect(called('download_install')).toEqual([
      { section: 'mod', modId: 700_000, fileId: 1_800_002, name: 'Haze — Midnight Bloom', author: 'lumen' },
    ]);
    expect(rows()[2]?.textContent).toContain('Installing');
  });

  it('shows the gallery picture the display scale needs, never a smaller one stretched', async () => {
    const { modPage } = await import('../../mock/fixtures');
    const image = modPage(0).gallery[0];
    const shown = async (scale: number) => {
      vi.stubGlobal('devicePixelRatio', scale);
      const { target } = await open('#/catalog/mod/700000');
      const src = target.querySelector<HTMLImageElement>('.gallery img')?.getAttribute('src');
      r.cleanup();
      return src;
    };
    expect(await shown(1)).toMatch(new RegExp(`/img/${image?.preview}$`));
    expect(await shown(1.5)).toMatch(new RegExp(`/img/${image?.large}$`));
    expect(await shown(2)).toMatch(new RegExp(`/img/${image?.full}$`));
  });

  it('shows a spinner while a gallery picture loads, over the picture shown until then', async () => {
    const { modPage } = await import('../../mock/fixtures');
    const gallery = modPage(0).gallery;
    const { target } = await open('#/catalog/mod/700000');
    const figure = target.querySelector('figure.gallery') as HTMLElement;
    const imgs = () => [...figure.querySelectorAll<HTMLImageElement>('img')];
    const srcOf = (img: HTMLImageElement | undefined) => img?.getAttribute('src')?.split('/img/')[1];
    // First picture: a skeleton and a spinner until it has loaded.
    expect(figure.getAttribute('aria-busy')).toBe('true');
    expect(figure.querySelector('.dl-skel')).not.toBeNull();
    expect(figure.querySelector('.dl-spinner')).not.toBeNull();
    imgs()[0]?.dispatchEvent(new Event('load'));
    flushSync();
    expect(figure.hasAttribute('aria-busy')).toBe(false);
    expect(figure.querySelector('.dl-skel, .dl-spinner')).toBeNull();
    // The next picture: the current one stays in view under the spinner until the next one is in.
    const inView = imgs()[0];
    r.click(target.querySelector('[aria-label="Next image"]') as HTMLElement);
    await settle();
    expect(imgs().map(srcOf)).toEqual([gallery[0]?.preview, gallery[1]?.preview]);
    expect(imgs()[0]).toBe(inView);
    expect(inView?.getAttribute('aria-hidden')).toBe('true');
    expect(figure.querySelector('.dl-spinner')).not.toBeNull();
    expect(figure.querySelector('.dl-skel')).toBeNull();
    imgs()[1]?.dispatchEvent(new Event('load'));
    flushSync();
    expect(imgs().map(srcOf)).toEqual([gallery[1]?.preview]);
    expect(figure.querySelector('.dl-spinner')).toBeNull();
    // "View original" fails: the picture in view stays and the button comes back.
    r.click(button(target, 'View original'));
    await settle();
    expect(figure.querySelector('.dl-spinner')).not.toBeNull();
    imgs()[1]?.dispatchEvent(new Event('error'));
    flushSync();
    expect(imgs().map(srcOf)).toEqual([gallery[1]?.preview]);
    expect(figure.querySelector('.dl-spinner')).toBeNull();
    expect(button(target, 'View original')).toBeTruthy();
  });

  it('keeps the missing-picture placeholder when a gallery picture can’t be loaded', async () => {
    const { target } = await open('#/catalog/mod/700000');
    const figure = target.querySelector('figure.gallery') as HTMLElement;
    figure.querySelector('img')?.dispatchEvent(new Event('error'));
    flushSync();
    expect(figure.querySelector('img, .dl-spinner')).toBeNull();
    expect(figure.querySelector('.missing svg')).not.toBeNull();
  });

  it('opens GameBanana through D1', async () => {
    const { target } = await open('#/catalog/mod/700000');
    const { prompts } = await import('../../lib/prompts.svelte');
    r.click(button(target, 'Open on GameBanana'));
    await settle();
    expect(prompts.link?.url).toBe('https://gamebanana.com/mods/700000');
  });

  it('opens a GameBanana mod linked in the description inside deadloader', async () => {
    const { target } = await open('#/catalog/mod/700000');
    target.querySelector<HTMLElement>('.prose a[data-dl-link="0"]')?.click();
    expect(location.hash).toBe('#/catalog/mod/700002');
  });

  it('asks D2 before a picture from a new host loads', async () => {
    const { target } = await open('#/catalog/mod/700000');
    target.querySelector<HTMLButtonElement>('.dl-image__load[aria-haspopup="dialog"]')?.click();
    await settle();
    const dialog = document.querySelector('[role="dialog"]') as HTMLElement;
    expect(dialog.textContent).toContain('Load an image from a new host?');
    expect(dialog.textContent).toContain('i.imgur.com');
    r.click(button(dialog, 'Load image'));
    await settle();
    expect(called('catalog_grant')).toEqual([{ id: expect.any(String), scope: 'always' }]);
    expect(target.querySelector('.dl-image__figure img')).not.toBeNull();
  });

  it('says when the mod was removed and keeps the installed copy in view', async () => {
    const { target } = await open('#/catalog/mod/700000', 'modpage-removed');
    expect(target.textContent).toContain('This mod was removed from GameBanana');
    expect(target.textContent).toContain('stays in your library');
    r.click(button(target, 'Show in library'));
    expect(location.hash).toBe('#/library/gb-700000');
  });

  it('shows a saved copy while offline, with installing unavailable', async () => {
    const { target } = await open('#/catalog/mod/700000', 'modpage-offline');
    expect(target.querySelector('.dl-banner')?.textContent).toContain('Offline — showing the copy saved on');
    const install = button(target.querySelectorAll('.file')[2] as HTMLElement, 'Install');
    expect(install.getAttribute('aria-disabled')).toBe('true');
  });

  it('says why a page that was never saved can’t be shown offline', async () => {
    const { target } = await open('#/catalog/mod/700003', 'modpage-offline');
    expect(target.querySelector('[role="alert"]')?.textContent).toContain('Offline — this mod page wasn’t saved');
  });

  it('goes back to the grid', async () => {
    const { target } = await open('#/catalog/mod/700000');
    expect(target.querySelector('nav a')?.getAttribute('href')).toBe('#/catalog');
    window.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowLeft', altKey: true }));
    await settle();
    expect(location.hash).toBe('#/catalog');
  });
});
