import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { Card, Changed, Favourite, LibraryView, Listing, TaskSnapshot, UiEvent } from '../lib/api';
import { Channel, favouriteOf, invoke, resetMock } from './core';

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

  it('says which command it does not know yet', async () => {
    await expect(invoke('no_such_command')).rejects.toEqual({
      id: 'Internal',
      args: { detail: 'mock: no_such_command is not implemented' },
    });
  });
});
