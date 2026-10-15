import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { Changed, LibraryView, Listing, TaskSnapshot, UiEvent } from '../lib/api';
import { Channel, invoke, resetMock } from './core';

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

