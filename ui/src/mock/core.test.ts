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
  LogCounts,
  LogPage,
  LogSummary,
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

  it('pages through invented logs, filters them and sums them up', async () => {
    const first = await invoke<LogPage>('log_page', { query: { kind: 'network', range: 'today', limit: 50 } });
    expect(first.entries).toHaveLength(50);
    const second = await invoke<LogPage>('log_page', {
      query: { kind: 'network', range: 'today', limit: 50, cursor: first.cursor },
    });
    expect(second.entries[0]).not.toEqual(first.entries[0]);
    const blocked = await invoke<LogPage>('log_page', { query: { kind: 'network', range: 'today', result: 'error', text: 'FASTMIRROR' } });
    expect(blocked.entries).toHaveLength(1);
    const counts = await invoke<LogCounts>('log_counts', { range: '7d' });
    expect(counts.files.operations).toBeGreaterThan(10);
    const summary = await invoke<LogSummary>('log_summary');
    expect(summary.game.changes).toBeGreaterThan(0);
    expect(summary.empty).toBe(false);
  });

  it('says which command it does not know yet', async () => {
    await expect(invoke('no_such_command')).rejects.toEqual({
      id: 'Internal',
      args: { detail: 'mock: no_such_command is not implemented' },
    });
  });

  describe('the Downloads scenarios', () => {
    afterEach(() => {
      history.replaceState(null, '', '/');
    });

    async function scenario(name: string): Promise<{ tasks: TaskSnapshot[]; seen: UiEvent[] }> {
      history.replaceState(null, '', `/?scenario=${name}`);
      resetMock();
      const seen: UiEvent[] = [];
      const channel = new Channel<UiEvent>();
      channel.onmessage = (e) => seen.push(e);
      await invoke('subscribe', { events: channel });
      return { tasks: await invoke<TaskSnapshot[]>('downloads'), seen };
    }

    it('has a task in every stage, a choice waiting and a short history', async () => {
      const { tasks, seen } = await scenario('downloads');
      const states = new Set(tasks.map((t) => t.state.state));
      expect([...states].sort()).toEqual(['downloading', 'failed', 'installing', 'paused', 'queued', 'verifying']);
      const waiting = seen.filter((e) => e.event === 'installed' && e.data.installed.changed === null);
      expect(waiting).toHaveLength(1);
      expect(seen.filter((e) => e.event === 'download' && e.data.event === 'removed').length).toBeGreaterThan(2);
    });

    it('pauses downloads for the network offline, and for space when the disk is full', async () => {
      const offline = await scenario('downloads-offline');
      expect(offline.tasks.some((t) => t.state.state === 'downloading')).toBe(false);
      expect(await invoke('catalog_status')).toMatchObject({ kind: 'offline' });
      const full = await scenario('downloads-disk-full');
      const waiting = full.tasks.filter((t) => t.state.state === 'waiting');
      expect(waiting.length).toBeGreaterThan(0);
      expect(waiting.every((t) => t.state.state === 'waiting' && t.state.message.id === 'E12')).toBe(true);
    });

    it('cancels a task, and turns a picked variant on at the top', async () => {
      const { tasks } = await scenario('downloads');
      const first = tasks[0] as TaskSnapshot;
      await invoke('download_control', { id: first.id, action: 'cancel' });
      expect((await invoke<TaskSnapshot[]>('downloads')).map((t) => t.id)).not.toContain(first.id);
      const changed = await invoke<Changed>('choose', { modId: 'gb-700042', files: ['gb-700042-1-3'] });
      expect(changed.library.mods[0]?.id).toBe('gb-700042');
      expect(changed.library.mods[0]?.variant_groups[0]?.active).toBe('gb-700042-1-3');
    });
  });
});

