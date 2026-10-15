import { beforeEach, describe, expect, it, vi } from 'vitest';

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

const core = await import('../../mock/core');
const { app } = await import('../../lib/app.svelte');
const { CatalogBrowser } = await import('./browse.svelte');

let browser: InstanceType<typeof CatalogBrowser>;

function called(command: string) {
  return calls.filter(([name]) => name === command).map(([, args]) => args);
}

beforeEach(async () => {
  core.resetMock();
  calls.length = 0;
  app.library = await core.invoke('library');
  app.settings = await core.invoke('settings');
  app.network = { kind: 'online' };
  browser = new CatalogBrowser();
});

describe('browsing the catalog', () => {
  it('loads the first page of the newest mods', async () => {
    await browser.load();
    expect(browser.items.length).toBeGreaterThan(40);
    expect(browser.hasMore).toBe(true);
    expect(browser.error).toBeNull();
    expect(called('catalog_list')[0]).toEqual({
      query: { section: 'mod', filter: { kind: 'browse', sort: 'newest', category: null }, page: 1 },
      byUser: true,
    });
  });

  it('prefetches the next page at 70 % and lists it at the end (28)', async () => {
    await browser.load();
    const count = browser.items.length;
    browser.rangeChanged(0, Math.floor(count * 0.5));
    expect(called('catalog_prefetch')).toEqual([]);
    browser.rangeChanged(0, Math.ceil(count * 0.7));
    browser.rangeChanged(0, Math.ceil(count * 0.75));
    expect(called('catalog_prefetch')).toHaveLength(1);
    expect((called('catalog_prefetch')[0]?.query as { page: number }).page).toBe(2);
    await browser.rangeChanged(0, count);
    expect(browser.items.length).toBeGreaterThan(count);
  });

  it('searches with text of 2–100 bytes, and browses otherwise', async () => {
    await browser.search('Haze');
    expect(browser.searching).toBe(true);
    expect(browser.items.every((c) => c.name.includes('Haze'))).toBe(true);
    expect(called('catalog_list').at(-1)?.query).toMatchObject({ filter: { kind: 'search', text: 'Haze' } });
    await browser.search('H');
    expect(browser.searching).toBe(false);
    expect(called('catalog_list').at(-1)?.query).toMatchObject({ filter: { kind: 'browse' } });
  });

  it('offers the heroes when Skins is chosen and filters by one', async () => {
    await browser.loadCategories();
    const skins = browser.roots.find((c) => c.name === 'Skins');
    await browser.setCategory(skins?.id ?? null);
    expect(browser.heroes.length).toBeGreaterThan(5);
    const haze = browser.heroes.find((c) => c.name === 'Haze');
    await browser.setHero(haze?.id ?? null);
    expect(called('catalog_list').at(-1)?.query).toMatchObject({ filter: { category: haze?.id } });
    expect(browser.items.every((c) => c.subcategory?.name === 'Haze')).toBe(true);
    await browser.setCategory(null);
    expect(browser.heroes).toEqual([]);
    expect(browser.hero).toBeNull();
  });

  it('keeps the category while searching by filtering the results', async () => {
    await browser.loadCategories();
    await browser.setCategory(2);
    await browser.search('Shop');
    expect(browser.items.length).toBeGreaterThan(0);
    expect(browser.items.every((c) => c.root_category?.id === 2)).toBe(true);
  });

  it('hides installed mods when asked', async () => {
    await browser.load();
    expect(browser.items.some((c) => c.id === 700_000)).toBe(true);
    browser.setHideInstalled(true);
    expect(browser.items.some((c) => c.id === 700_000)).toBe(false);
    expect(browser.activeFilters).toBe(1);
  });

  it('lists the favourites, newest first, and toggles one', async () => {
    await browser.load();
    await browser.setFavouritesOnly(true);
    expect(browser.items.map((c) => c.id)).toEqual([700_009, 700_003]);
    const first = browser.items[0];
    if (!first) {
      throw new Error('a favourite');
    }
    expect(browser.isFavourite(first)).toBe(true);
    await browser.toggleFavourite(first);
    expect(browser.items.map((c) => c.id)).toEqual([700_003]);
    await browser.setFavouritesOnly(false);
    const other = browser.items.find((c) => c.id === 700_010);
    if (!other) {
      throw new Error('a card');
    }
    await browser.toggleFavourite(other);
    expect(browser.isFavourite(other)).toBe(true);
  });

  it('switches to sounds', async () => {
    await browser.setSection('sound');
    expect(browser.items.every((c) => c.section === 'sound')).toBe(true);
  });

  it('reports why the catalog can’t be shown', async () => {
    core.setCatalogScenario('catalog-api-changed');
    await browser.load();
    expect(browser.error).toMatchObject({ kind: 'api_changed' });
    expect(browser.items).toEqual([]);
    const settings = await core.invoke<Record<string, Record<string, unknown>>>('settings');
    await core.invoke('update_settings', { settings: { ...settings, catalog: { ...settings.catalog, enabled: false } } });
    core.setCatalogScenario(null);
    await browser.load();
    expect(browser.error).toEqual({ kind: 'local_mode' });
  });

  it('keeps saved pages while offline and stops at the last saved one', async () => {
    core.setCatalogScenario('catalog-offline');
    await browser.load();
    expect(browser.origin).toMatchObject({ kind: 'cache' });
    await browser.rangeChanged(0, browser.items.length);
    const two = browser.items.length;
    await browser.rangeChanged(0, two);
    expect(browser.items.length).toBe(two);
    expect(browser.moreError).toMatchObject({ kind: 'offline' });
    expect(browser.error).toBeNull();
  });

  it('fetches a page again once when a thumbnail is gone (the core forgot its picture id)', async () => {
    await browser.load();
    const card = browser.items[3];
    if (!card) {
      throw new Error('a card');
    }
    calls.length = 0;
    await browser.thumbFailed(card);
    await browser.thumbFailed(card);
    expect(called('catalog_list')).toEqual([
      { query: { section: 'mod', filter: { kind: 'browse', sort: 'newest', category: null }, page: 1 }, byUser: false },
    ]);
  });

  it('takes a page refreshed in the background', async () => {
    await browser.load();
    calls.length = 0;
    await browser.pageUpdated({ section: 'mod', filter: { kind: 'browse', sort: 'newest', category: null }, page: 1 });
    await browser.pageUpdated({ section: 'mod', filter: { kind: 'browse', sort: 'oldest', category: null }, page: 1 });
    expect(called('catalog_list')).toHaveLength(1);
  });

  it('ignores an answer that arrives after the user changed the query', async () => {
    const first = browser.load();
    const second = browser.setSort('oldest');
    await Promise.all([first, second]);
    expect(browser.sort).toBe('oldest');
    const sorted = [...browser.items].sort((a, b) => (a.date_added ?? 0) - (b.date_added ?? 0));
    expect(browser.items.map((c) => c.id)).toEqual(sorted.map((c) => c.id));
  });

  it('clears the filters and the search', async () => {
    await browser.setCategory(2);
    browser.setHideInstalled(true);
    await browser.search('Shop');
    await browser.clearFilters();
    expect(browser.category).toBeNull();
    expect(browser.hideInstalled).toBe(false);
    expect(browser.text).toBe('Shop');
    await browser.clearSearch();
    expect(browser.text).toBe('');
    expect(browser.searching).toBe(false);
  });
});
