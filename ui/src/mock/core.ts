// The core for `pnpm dev:mock`: `@tauri-apps/api/core` is aliased here (vite.config.ts), so screens run in a plain
// browser with invented data and can be compared with the design boards. Never part of the app build.
//
// A command this mock doesn't know rejects with an Internal message naming it; add it here when a screen needs it.

import type {
  Card,
  Changed,
  LibraryView,
  Listing,
  Query,
  Settings,
  TaskSnapshot,
  UiEvent,
} from '../lib/api';
import { card, libraryMod, modPage, settings as defaultSettings } from './fixtures';

/** Tauri's `Channel`: the UI sets `onmessage`; the mock calls it. */
export class Channel<T> {
  onmessage: (message: T) => void = () => {};
}

interface State {
  library: LibraryView;
  profiles: string[];
  settings: Settings;
  tasks: TaskSnapshot[];
  nextTask: number;
  listener: Channel<UiEvent> | null;
  gameRunning: boolean;
}

let state: State;

function fresh(): State {
  return {
    library: {
      selected_profile: 'Default',
      applied_profile: 'Default',
      pending: false,
      mods: [libraryMod(0, true), libraryMod(1, true), libraryMod(2, false), libraryMod(3, true, true), libraryMod(4, true)],
      conflicts: [
        {
          winner_mod: 'gb-700000',
          winner_file: 'gb-700000-1-0',
          loser_mod: 'gb-700004',
          loser_file: 'gb-700004-1-0',
          paths: ['models/heroes/haze/haze.vmdl_c'],
        },
      ],
    },
    profiles: ['Default', 'Evening', 'Ranked'],
    settings: defaultSettings(),
    tasks: [],
    nextTask: 1,
    listener: null,
    gameRunning: false,
  };
}

/**
 * Preview scenarios from the address, e.g. `http://localhost:5199/?scenario=switched#/library`:
 * `switched` (another profile selected than applied), `running` (the game runs), `local` (local mode).
 * Screens add their own when they need a state.
 */
function applyScenario(s: State): State {
  const scenario = typeof location === 'undefined' ? null : new URLSearchParams(location.search).get('scenario');
  switch (scenario) {
    case 'switched':
      s.library.selected_profile = 'Evening';
      s.library.pending = true;
      break;
    case 'running':
      s.gameRunning = true;
      break;
    case 'local':
      s.settings.catalog = { ...s.settings.catalog, enabled: false };
      break;
  }
  return s;
}

/** Back to the first-start state (tests). */
export function resetMock(): void {
  state = applyScenario(fresh());
}
resetMock();

function push(event: UiEvent): void {
  state.listener?.onmessage(event);
}

function changed(): Changed {
  return { library: structuredClone(state.library), applied: !state.gameRunning, waiting_for_game: state.gameRunning };
}

function mod(id: unknown) {
  const found = state.library.mods.find((m) => m.id === id);
  if (!found) {
    throw { id: 'Internal', args: { detail: `mock: no mod ${String(id)}` } };
  }
  return found;
}

const LISTED = Array.from({ length: 120 }, (_, i) => card(i));

function listing(query: Query): Listing {
  const perPage = query.filter.kind === 'search' ? 15 : 50;
  const matching =
    query.filter.kind === 'search'
      ? LISTED.filter((c) => c.name.toLowerCase().includes((query.filter as { text: string }).text.toLowerCase()))
      : LISTED;
  const visible = matching.filter((c: Card) => !c.nsfw || state.settings.catalog?.nsfw !== 'hidden');
  const start = (Math.max(query.page, 1) - 1) * perPage;
  return {
    items: visible.slice(start, start + perPage),
    hidden_nsfw: matching.slice(start, start + perPage).length - visible.slice(start, start + perPage).length,
    total: visible.length,
    page: query.page,
    has_more: start + perPage < visible.length,
    origin: { kind: 'network' },
  };
}

/** A download that takes a few seconds, reported like the real queue. */
function startDownload(modId: number, fileId: number, name: string): number {
  const id = state.nextTask++;
  const total = 4_800_000;
  const task: TaskSnapshot = {
    id,
    url: `https://gamebanana.com/dl/${fileId}`,
    file_name: `${name.toLowerCase().replace(/\W+/g, '_')}.zip`,
    md5: null,
    size: total,
    initiator: 'user',
    meta: { section: 'mod', mod_id: String(modId), file_id: String(fileId), mod_name: name },
    state: { state: 'queued' },
    bytes: 0,
    total,
    speed: 0,
  };
  state.tasks.push(task);
  push({ event: 'download', data: { event: 'added', task: structuredClone(task) } });
  const tick = setInterval(() => {
    if (task.state.state === 'paused') {
      return;
    }
    if (task.state.state === 'queued') {
      task.state = { state: 'downloading' };
      push({ event: 'download', data: { event: 'state', id, state: task.state } });
    }
    task.bytes = Math.min(total, task.bytes + 600_000);
    task.speed = 6_000_000;
    push({ event: 'download', data: { event: 'progress', id, bytes: task.bytes, total, speed: task.speed } });
    if (task.bytes >= total) {
      clearInterval(tick);
      task.state = { state: 'installing' };
      push({ event: 'download', data: { event: 'state', id, state: task.state } });
      const installed = libraryMod(state.library.mods.length + 10, true);
      installed.name = name;
      state.library.mods.unshift(installed);
      state.tasks = state.tasks.filter((t) => t.id !== id);
      push({ event: 'download', data: { event: 'removed', id } });
      push({
        event: 'installed',
        data: {
          task: id,
          installed: {
            outcome: { mod_id: installed.id, new_files: installed.files.map((f) => f.id), choices: [], skipped: [] },
            changed: changed(),
          },
        },
      });
    }
  }, 500);
  return id;
}

type Args = Record<string, unknown>;

const handlers: Record<string, (args: Args) => unknown> = {
  subscribe: (args) => {
    state.listener = args.events as Channel<UiEvent>;
    push({ event: 'game', data: { running: state.gameRunning } });
    return null;
  },
  system_locale: () => ({ languages: [...navigator.languages], region: null }),
  library: () => structuredClone(state.library),
  set_enabled: (args) => {
    mod(args.modId).enabled = Boolean(args.enabled);
    return changed();
  },
  move_mod: (args) => {
    const m = mod(args.modId);
    state.library.mods = state.library.mods.filter((x) => x !== m);
    state.library.mods.splice(Number(args.index), 0, m);
    return changed();
  },
  switch_variant: (args) => {
    const m = mod(args.modId);
    for (const group of m.variant_groups) {
      if (group.files.includes(String(args.file))) {
        group.active = String(args.file);
      }
    }
    return changed();
  },
  apply: () => {
    state.library.pending = false;
    state.library.applied_profile = state.library.selected_profile;
    return { library: structuredClone(state.library), changed: true, gameinfo_foreign: false };
  },
  profile_names: () => [...state.profiles],
  select_profile: (args) => {
    state.library.selected_profile = String(args.name);
    state.library.pending = state.library.selected_profile !== state.library.applied_profile;
    return structuredClone(state.library);
  },
  create_profile: (args) => {
    state.profiles.push(String(args.name));
    return [...state.profiles];
  },
  settings: () => structuredClone(state.settings),
  update_settings: (args) => {
    state.settings = structuredClone(args.settings as Settings);
    return structuredClone(state.settings);
  },
  catalog_status: () => (state.settings.catalog?.enabled ? { kind: 'online' } : { kind: 'local_mode' }),
  catalog_list: (args) => {
    if (!state.settings.catalog?.enabled) {
      throw { kind: 'local_mode' };
    }
    return listing(args.query as Query);
  },
  catalog_prefetch: () => true,
  catalog_details: (args) => modPage(Number(args.id) - 700_000),
  catalog_retry_now: () => null,
  downloads: () => structuredClone(state.tasks),
  download_install: (args) => startDownload(Number(args.modId), Number(args.fileId), String(args.name)),
  download_control: (args) => {
    const task = state.tasks.find((t) => t.id === args.id);
    if (task && args.action === 'pause') {
      task.state = { state: 'paused' };
    } else if (task && args.action === 'resume') {
      task.state = { state: 'downloading' };
    }
    if (task) {
      push({ event: 'download', data: { event: 'state', id: task.id, state: task.state } });
    }
    return null;
  },
  updates_check: () => ({ checks: [], available: 0, stopped: null }),
  game_status: () => ({
    running: state.gameRunning,
    root: String.raw`D:\SteamLibrary\steamapps\common\Deadlock`,
    installations: 2,
  }),
  apply_preview: () => {
    const switched = state.library.selected_profile !== state.library.applied_profile;
    return switched
      ? { turned_off: state.library.mods.filter((m) => m.enabled).map((m) => m.name).slice(0, 3), turned_on: [] }
      : { turned_off: [], turned_on: [] };
  },
  play: () => {
    if (state.gameRunning) {
      throw { id: 'E13', args: {} };
    }
    state.library.pending = false;
    state.library.applied_profile = state.library.selected_profile;
    state.gameRunning = true;
    push({ event: 'game', data: { running: true } });
    return { library: structuredClone(state.library), changed: true, gameinfo_foreign: false };
  },
  window_minimize: () => null,
  window_toggle_maximize: () => null,
  window_close: () => null,
  window_start_drag: () => null,
};

/** Tauri's `invoke`, answered from the mock state after a short, realistic delay. */
export async function invoke<T>(command: string, args: Args = {}): Promise<T> {
  const handler = handlers[command];
  if (!handler) {
    throw { id: 'Internal', args: { detail: `mock: ${command} is not implemented` } };
  }
  await Promise.resolve();
  return handler(args) as T;
}

/**
 * For the console while previewing: `__mock.setGameRunning(true)`, or any command on the instance the app uses,
 * e.g. `await __mock.invoke('select_profile', { name: 'Evening' })` (importing the module by hand may load a copy).
 */
export const devtools = {
  invoke,
  setGameRunning(running: boolean): void {
    state.gameRunning = running;
    push({ event: 'game', data: { running } });
  },
};

// The browser console can drive the preview: `__mock.setGameRunning(true)`.
Object.assign(globalThis, { __mock: devtools });
