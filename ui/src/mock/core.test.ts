import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type {
  Card,
  Categories,
  CatalogStatus,
  Changed,
  ConsentPrompt,
  Favourite,
  Granted,
  LibraryView,
  Listing,
  ModPage,
  TaskSnapshot,
  UiEvent,
} from '../lib/api';
import { Channel, favouriteOf, invoke, resetMock, setCatalogScenario } from './core';

beforeEach(() => {
  resetMock();
});

afterEach(() => {
  vi.useRealTimers();
});

describe('the mock core (pnpm dev:mock)', () => {
  it('answers the library with a few mods in the selected profile', async () => {
    const view = await invoke<LibraryView>('library');
    expect(view.selected_profile).toBe('Default');
    expect(view.mods.length).toBeGreaterThan(3);
  });

  it('keeps state between calls', async () => {
    const view = await invoke<LibraryView>('library');
    const id = view.mods[0]?.id;
    const changed = await invoke<Changed>('set_enabled', { modId: id, enabled: false });
    expect(changed.library.mods.find((m) => m.id === id)?.enabled).toBe(false);
    expect((await invoke<LibraryView>('library')).mods.find((m) => m.id === id)?.enabled).toBe(false);
  });

  it('pushes events to the subscribed channel, a download included', async () => {
    vi.useFakeTimers();
    const seen: UiEvent[] = [];
    const channel = new Channel<UiEvent>();
    channel.onmessage = (e) => seen.push(e);
    await invoke('subscribe', { events: channel });
    expect(seen[0]).toEqual({ event: 'game', data: { running: false } });

    const listing = await invoke<Listing>('catalog_list', {
      query: { section: 'mod', filter: { kind: 'browse', sort: 'newest', category: null }, page: 1 },
      byUser: true,
    });
    const card = listing.items[0];
    if (!card) {
      throw new Error('the mock lists cards');
    }
    await invoke('download_install', { section: 'mod', modId: card.id, fileId: 1, name: card.name, author: null });
    await vi.advanceTimersByTimeAsync(10_000);
    const kinds = seen.map((e) => (e.event === 'download' ? `download:${e.data.event}` : e.event));
    expect(kinds).toContain('download:added');
    expect(kinds).toContain('download:progress');
    expect(kinds).toContain('installed');
    expect(await invoke<TaskSnapshot[]>('downloads')).toEqual([]);
  });

  it('keeps favourites newest first and opens only https links', async () => {
    const before = await invoke<Favourite[]>('favourites');
    expect(before.length).toBeGreaterThan(0);
    const listing = await invoke<Listing>('catalog_list', {
      query: { section: 'mod', filter: { kind: 'browse', sort: 'newest', category: null }, page: 1 },
      byUser: true,
    });
    const fresh = favouriteOf(listing.items[0] as Card);
    const after = await invoke<Favourite[]>('set_favourite', { favourite: fresh, on: true });
    expect(after[0]).toEqual(fresh);
    expect(await invoke<Favourite[]>('set_favourite', { favourite: fresh, on: false })).toEqual(before);
    await expect(invoke('open_link', { url: 'https://gamebanana.com/' })).resolves.toBeNull();
    await expect(invoke('open_link', { url: 'http://example.org/' })).rejects.toMatchObject({ id: 'Internal' });
  });

  it('lists categories, heroes under Skins, and pages filtered by them', async () => {
    const roots = await invoke<Categories>('catalog_categories', { section: 'mod', parent: null });
    expect(roots.items.map((c) => c.name)).toContain('Skins');
    const heroes = await invoke<Categories>('catalog_categories', { section: 'mod', parent: 1 });
    const haze = heroes.items.find((c) => c.name === 'Haze');
    const page = await invoke<Listing>('catalog_list', {
      query: { section: 'mod', filter: { kind: 'browse', sort: 'most_liked', category: haze?.id ?? 0 }, page: 1 },
      byUser: true,
    });
    expect(page.items.length).toBeGreaterThan(0);
    expect(page.items.every((c) => c.subcategory?.name === 'Haze')).toBe(true);
    const sounds = await invoke<Listing>('catalog_list', {
      query: { section: 'sound', filter: { kind: 'browse', sort: 'newest', category: null }, page: 1 },
      byUser: true,
    });
    expect(sounds.items.every((c) => c.section === 'sound')).toBe(true);
  });

  it('plays the catalog scenarios', async () => {
    const query = { section: 'mod', filter: { kind: 'browse', sort: 'newest', category: null }, page: 1 };
    setCatalogScenario('catalog-api-changed');
    await expect(invoke('catalog_list', { query, byUser: true })).rejects.toMatchObject({ kind: 'api_changed' });
    expect(await invoke<CatalogStatus>('catalog_status')).toMatchObject({ kind: 'api_changed' });
    setCatalogScenario('catalog-offline');
    const saved = await invoke<Listing>('catalog_list', { query, byUser: true });
    expect(saved.origin.kind).toBe('cache');
    await expect(invoke('catalog_list', { query: { ...query, page: 3 }, byUser: true })).rejects.toMatchObject({
      kind: 'offline',
    });
    setCatalogScenario('modpage-removed');
    await expect(invoke('catalog_details', { section: 'mod', id: 700_001, listedUpdated: null })).rejects.toEqual({
      kind: 'not_found',
      other_game: null,
    });
    setCatalogScenario(null);
    await expect(
      invoke('catalog_list', { query: { section: 'mod', filter: { kind: 'search', text: 'a' }, page: 1 }, byUser: true }),
    ).rejects.toEqual({ kind: 'invalid_search', problem: 'too_short' });
  });

  it('loads a description picture after consent, and remembers an allowed host', async () => {
    const page = await invoke<ModPage>('catalog_details', { section: 'mod', id: 700_000, listedUpdated: null });
    const image = page.description.images.find((i) => i.access === 'consent');
    expect(image).toBeDefined();
    const prompt = await invoke<ConsentPrompt>('catalog_consent_prompt', { id: image?.id });
    expect(prompt).toEqual({ host: 'i.imgur.com', always_allow_prechecked: true });
    expect(await invoke<Granted>('catalog_grant', { id: image?.id, scope: 'always' })).toEqual({ activated: [image?.id] });
    const again = await invoke<ModPage>('catalog_details', { section: 'mod', id: 700_000, listedUpdated: null });
    expect(again.description.images.find((i) => i.id === image?.id)?.access).toBe('load');
  });

  it('says which command it does not know yet', async () => {
    await expect(invoke('no_such_command')).rejects.toEqual({
      id: 'Internal',
      args: { detail: 'mock: no_such_command is not implemented' },
    });
  });
});
