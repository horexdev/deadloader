// The core for `pnpm dev:mock`: `@tauri-apps/api/core` is aliased here (vite.config.ts), so screens run in a plain
// browser with invented data and can be compared with the design boards. Never part of the app build.
//
// A command this mock doesn't know rejects with an Internal message naming it; add it here when a screen needs it.

import type {
  Card,
  CatalogStatus,
  Changed,
  Favourite,
  LibraryView,
  Listing,
  ModPage,
  Query,
  Settings,
  LogQuery,
  TaskSnapshot,
  UiEvent,
} from '../lib/api';
import {
  HERO_CATEGORIES,
  MOD_ROOTS,
  SOUND_ROOTS,
  card,
  libraryMod,
  modPage,
  settings as defaultSettings,
  soundCard,
  soundPage,
} from './fixtures';
import * as downloadsPreview from './downloads';
import * as profilesPreview from './profiles';
import * as activity from './activity';
import { settingsHandlers } from './settings';
import { LibraryMock, libraryConflicts, libraryMods } from './library';

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
  favourites: Favourite[];
  /** Pushed right after `subscribe`, like the events the core keeps while nobody listens. */
  onSubscribe: UiEvent[];
  /** Overrides the catalog status the settings imply (e.g. offline). */
  catalogStatus: CatalogStatus | null;
  /** The Profiles screen's data: game folders, last applied, mods of the other profiles. */
  profileData: profilesPreview.ProfilesMock;
  logs: activity.MockLogs;
  /** The game folder Steam knows; null: Deadlock wasn't found. */
  gameRoot: string | null;
  /** The Library's part (mock/library.ts). */
  libraryMock: LibraryMock;
}

let state: State;

function fresh(): State {
  const s: Omit<State, 'libraryMock'> = {
    library: {
      selected_profile: 'Default',
      applied_profile: 'Default',
      pending: false,
      pending_count: 0,
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
    favourites: [favouriteOf(card(9)), favouriteOf(card(3))],
    onSubscribe: [],
    catalogStatus: null,
    profileData: { info: {}, applied: {} },
    logs: activity.mockLogs(new Date(), scenario()),
    gameRoot: String.raw`D:\SteamLibrary\steamapps\common\Deadlock`,
  };
  // The Library's boards have their own invented library (mock/library.ts); the other screens keep the shared one.
  if (onLibrary()) {
    s.library.mods = libraryMods();
    s.library.conflicts = libraryConflicts();
  }
  return withLibrary(s);
}

function onLibrary(): boolean {
  const hash = typeof location === 'undefined' ? '' : location.hash;
  return hash.startsWith('#/library') || (scenario() ?? '').startsWith('library');
}

/** The library mock reads and changes the shared state through this host. */
function withLibrary(s: Omit<State, 'libraryMock'>): State {
  const full = s as State;
  full.libraryMock = new LibraryMock({
    get library() {
      return full.library;
    },
    set library(value) {
      full.library = value;
    },
    get gameRunning() {
      return full.gameRunning;
    },
    set gameRunning(value) {
      full.gameRunning = value;
    },
    get gameRoot() {
      return full.gameRoot;
    },
    set gameRoot(value) {
      full.gameRoot = value;
    },
    catalogEnabled: () => full.settings.catalog?.enabled === true,
    push: (event) => full.listener?.onmessage(event),
  });
  return full;
}

/** What the heart saves for a card (the core keeps it in favourites.json). */
export function favouriteOf(c: Card): Favourite {
  return {
    section: c.section,
    id: c.id,
    name: c.name,
    author: c.author,
    root_category: c.root_category,
    subcategory: c.subcategory,
    nsfw: c.nsfw,
    date_updated: c.date_updated,
  };
}

function scenario(): string | null {
  return typeof location === 'undefined' ? null : new URLSearchParams(location.search).get('scenario');
}

/**
 * Preview scenarios from the address, e.g. `http://localhost:5199/?scenario=switched#/library`:
 * `switched` (another profile selected than applied), `running` (the game runs), `local` (local mode),
 * `activity-empty` (no logs yet), `activity-local` (logs of local mode), `activity-blocked` (the newest request was
 * blocked; open `#/activity/r-blocked` for its details).
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
    case 'activity-local':
    case 'library-local':
      s.settings.catalog = { ...s.settings.catalog, enabled: false };
      break;
    // Downloads (S6): the board `Downloads` and its states; `downloads-empty` is the first start.
    case 'downloads':
    case 'downloads-choice':
    case 'downloads-offline':
    case 'downloads-disk-full': {
      const choice = downloadsPreview.variantMod();
      s.library.mods.push(choice);
      const queue = downloadsPreview.queue();
      s.tasks =
        scenario === 'downloads-offline'
          ? downloadsPreview.offline(queue)
          : scenario === 'downloads-disk-full'
            ? downloadsPreview.diskFull(queue)
            : scenario === 'downloads-choice'
              ? queue.slice(0, 2)
              : queue;
      s.onSubscribe = downloadsPreview.finishedEvents(choice, {
        library: structuredClone(s.library),
        applied: true,
        waiting_for_game: false,
      });
      if (scenario === 'downloads-offline') {
        s.catalogStatus = { kind: 'offline', since_unix_ms: Date.now() - 12 * 60_000 };
      }
      break;
    }
  }
  applyNoticeScenario(s, scenario);
  if (scenario?.startsWith('library-')) {
    s.libraryMock.scenario(scenario);
  }
  return s;
}

/**
 * The first-run notice: acknowledged in every scenario except the wizard's own, so other screens preview without it.
 * `wizard`, `wizard-setup-issues`: a first start (local mode until the choice). `settings-local`: local mode.
 */
function applyNoticeScenario(s: State, scenario: string | null): void {
  const firstStart = scenario === 'wizard' || scenario === 'wizard-setup-issues';
  if (firstStart) {
    s.settings.catalog = { ...s.settings.catalog, enabled: false };
    return;
  }
  if (scenario === 'settings-local') {
    s.settings.catalog = { ...s.settings.catalog, enabled: false };
  }
  s.settings.acknowledgements = {
    ...s.settings.acknowledgements,
    notice: {
      version: 1,
      at: '2026-10-14T16:42:00Z',
      language: 'en',
      catalog: s.settings.catalog?.enabled === true ? 'catalog' : 'local_mode',
    },
  };
}

/** Settings, first run and maintenance (src/mock/settings.ts), made again with each reset. */
let maintenanceHandlers: Record<string, (args: Args) => unknown> = {};

function currentScenario(): string | null {
  return typeof location === 'undefined' ? null : new URLSearchParams(location.search).get('scenario');
}

/** Back to the first-start state (tests). */
export function resetMock(): void {
  state = applyScenario(fresh());
  maintenanceHandlers = settingsHandlers({
    settings: () => state.settings,
    setSettings: (settings) => {
      state.settings = structuredClone(settings);
    },
    profiles: () => [...state.profiles],
    selectedProfile: () => state.library.selected_profile,
    gameRunning: () => state.gameRunning,
    scenario: currentScenario,
  });
  catalogScenario = scenarioFromAddress();
  activated.clear();
  // Profiles (S5): a fourth profile on a second installation (153).
  state.profiles = [...state.profiles, 'Sasha'];
  state.profileData = profilesPreview.fresh(state.profiles, state.library);
}

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

// ---- the catalog (S2, S3): pages, categories, mod pages, description pictures and the catalog's scenarios

/**
 * The catalog's preview states (`?scenario=…`): `catalog-offline` (saved pages only), `catalog-api-changed` (E4),
 * `catalog-rate-limited` (E5 with a countdown, saved pages stay), `catalog-empty` (nothing matches),
 * `catalog-loading` (the first page never arrives), `modpage-removed` (gone from GameBanana) and `modpage-offline`
 * (a saved copy of the page while offline). Tests switch them with `setCatalogScenario`.
 */
export type CatalogScenario =
  | 'catalog-offline'
  | 'catalog-api-changed'
  | 'catalog-rate-limited'
  | 'catalog-empty'
  | 'catalog-loading'
  | 'modpage-removed'
  | 'modpage-offline';

const CATALOG_SCENARIOS: readonly string[] = [
  'catalog-offline',
  'catalog-api-changed',
  'catalog-rate-limited',
  'catalog-empty',
  'catalog-loading',
  'modpage-removed',
  'modpage-offline',
];

let catalogScenario: CatalogScenario | null = null;
/** When the rate-limited scenario started waiting. */
let limitedAt = Date.now();
const activated = new Set<string>();

function scenarioFromAddress(): CatalogScenario | null {
  const name = typeof location === 'undefined' ? null : new URLSearchParams(location.search).get('scenario');
  return name && CATALOG_SCENARIOS.includes(name) ? (name as CatalogScenario) : null;
}

/** Tests: the catalog's state for the following calls (null: online). */
export function setCatalogScenario(scenario: CatalogScenario | null): void {
  catalogScenario = scenario;
  limitedAt = Date.now();
}

const MODS = Array.from({ length: 120 }, (_, i) => card(i));
const SOUNDS = Array.from({ length: 40 }, (_, i) => soundCard(i));
const SAVED_AT = Date.now() - 47 * 60_000;
const LOG_ID = 'net-20261015-1742-0193';

function catalogStatus(): CatalogStatus {
  if (state.catalogStatus) {
    return state.catalogStatus;
  }
  if (!state.settings.catalog?.enabled) {
    return { kind: 'local_mode' };
  }
  switch (catalogScenario) {
    case 'catalog-offline':
    case 'modpage-offline':
      return { kind: 'offline', since_unix_ms: SAVED_AT + 20 * 60_000 };
    case 'catalog-api-changed':
      return { kind: 'api_changed', message: { id: 'E4', args: { log_id: LOG_ID } } };
    case 'catalog-rate-limited': {
      const until = limitedAt + 12_000;
      return Date.now() < until
        ? { kind: 'rate_limited', until_unix_ms: until, message: { id: 'E5', args: { status: '429' } } }
        : { kind: 'online' };
    }
    default:
      return { kind: 'online' };
  }
}

const SORTS: Record<string, (a: Card, b: Card) => number> = {
  newest: (a, b) => (b.date_added ?? 0) - (a.date_added ?? 0),
  latest_updated: (a, b) => (b.date_updated ?? b.date_added ?? 0) - (a.date_updated ?? a.date_added ?? 0),
  most_downloaded: (a, b) => b.views / 3 - a.views / 3,
  most_liked: (a, b) => b.likes - a.likes,
  most_viewed: (a, b) => b.views - a.views,
  most_commented: (a, b) => (b.id % 17) - (a.id % 17),
  oldest: (a, b) => (a.date_added ?? 0) - (b.date_added ?? 0),
};

function listing(query: Query): Listing {
  const filter = query.filter;
  const all = query.section === 'sound' ? SOUNDS : MODS;
  let matching: Card[];
  if (filter.kind === 'search') {
    const text = filter.text.trim();
    const bytes = new TextEncoder().encode(text).length;
    if (bytes < 2 || bytes > 100) {
      throw { kind: 'invalid_search', problem: bytes < 2 ? 'too_short' : 'too_long' };
    }
    matching = all.filter((c) => c.name.toLowerCase().includes(text.toLowerCase()));
  } else {
    const category = filter.category;
    matching = all
      .filter((c) => category === null || c.root_category?.id === category || c.subcategory?.id === category)
      .sort(SORTS[filter.sort] ?? SORTS['newest']);
  }
  if (catalogScenario === 'catalog-empty') {
    matching = [];
  }
  const perPage = filter.kind === 'search' ? 15 : 50;
  const nsfw = state.settings.catalog?.nsfw ?? 'hidden';
  const start = (Math.max(query.page, 1) - 1) * perPage;
  const page = matching.slice(start, start + perPage);
  const items = page.filter((c) => !c.nsfw || nsfw !== 'hidden');
  const offline = catalogScenario === 'catalog-offline' || catalogScenario === 'catalog-rate-limited';
  return {
    items: structuredClone(items),
    hidden_nsfw: page.length - items.length,
    total: matching.length,
    page: query.page,
    has_more: start + perPage < matching.length,
    origin: offline ? { kind: 'cache', fetched_at_unix_ms: SAVED_AT, stale: true } : { kind: 'network' },
  };
}

function catalogList(args: Record<string, unknown>): Listing | Promise<never> {
  if (!state.settings.catalog?.enabled) {
    throw { kind: 'local_mode' };
  }
  const query = args.query as Query;
  switch (catalogScenario) {
    case 'catalog-loading':
      return new Promise<never>(() => {});
    case 'catalog-api-changed':
      throw { kind: 'api_changed', message: { id: 'E4', args: { log_id: LOG_ID } } };
    case 'catalog-offline':
      // Only the first two pages of the default listing were saved.
      if (query.filter.kind === 'search' || query.page > 2) {
        throw { kind: 'offline', since_unix_ms: SAVED_AT + 20 * 60_000 };
      }
      break;
    case 'catalog-rate-limited':
      if (query.page > 2 && catalogStatus().kind === 'rate_limited') {
        throw { kind: 'rate_limited', retry_in_s: 12, message: { id: 'E5', args: { status: '429' } } };
      }
      break;
    default:
      break;
  }
  return listing(query);
}

function categories(args: Record<string, unknown>) {
  const parent = args.parent === null || args.parent === undefined ? null : Number(args.parent);
  const all = args.section === 'sound' ? SOUNDS : MODS;
  const count = (id: number) => all.filter((c) => c.root_category?.id === id || c.subcategory?.id === id).length;
  const items =
    parent === null
      ? (args.section === 'sound' ? SOUND_ROOTS : MOD_ROOTS).map((c) => ({
          ...c,
          item_count: count(c.id),
          subcategory_count: c.id === 1 ? HERO_CATEGORIES.length : 0,
        }))
      : parent === 1
        ? HERO_CATEGORIES.map((c) => ({ ...c, item_count: count(c.id), subcategory_count: 0 }))
        : [];
  return { items, origin: { kind: 'network' } };
}

function details(args: Record<string, unknown>): ModPage {
  const id = Number(args.id);
  if (catalogScenario === 'modpage-removed') {
    throw { kind: 'not_found', other_game: null };
  }
  if (catalogScenario === 'catalog-api-changed') {
    throw { kind: 'api_changed', message: { id: 'E4', args: { log_id: LOG_ID } } };
  }
  const page = args.section === 'sound' ? soundPage(id - 800_000) : modPage(id - 700_000);
  if (catalogScenario === 'modpage-offline' || catalogScenario === 'catalog-offline') {
    if (id % 4 === 3) {
      throw { kind: 'offline', since_unix_ms: SAVED_AT };
    }
    return { ...page, origin: { kind: 'cache', fetched_at_unix_ms: SAVED_AT - 3 * 86_400_000, stale: true } };
  }
  if (!state.settings.catalog?.enabled) {
    throw { kind: 'local_mode' };
  }
  // Pictures the user loaded before show at once.
  const images = page.description.images.map((image) => ({
    ...image,
    access: activated.has(image.id) || (state.settings.catalog?.allowed_image_hosts as string[] | undefined)?.includes(image.host) ? 'load' : image.access,
  }));
  // While NSFW is hidden the core withholds such a mod's pictures (27).
  const nsfwHidden = page.nsfw && (state.settings.catalog?.nsfw ?? 'hidden') === 'hidden';
  return { ...page, nsfw_hidden: nsfwHidden, description: { ...page.description, images } as ModPage['description'] };
}

function imageHost(id: string): string | undefined {
  for (const i of [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]) {
    const found = modPage(i).description.images.find((image) => image.id === id);
    if (found) {
      return found.host;
    }
  }
  return undefined;
}

const catalogHandlers: Record<string, (args: Record<string, unknown>) => unknown> = {
  catalog_status: () => catalogStatus(),
  catalog_list: catalogList,
  catalog_refresh: catalogList,
  catalog_prefetch: () => catalogScenario === null,
  catalog_categories: (args) => {
    if (catalogScenario === 'catalog-api-changed') {
      throw { kind: 'api_changed', message: { id: 'E4', args: { log_id: LOG_ID } } };
    }
    return categories(args);
  },
  catalog_details: details,
  catalog_retry_now: () => {
    limitedAt = 0;
    push({ event: 'catalog', data: { event: 'status', data: catalogStatus() } });
    return null;
  },
  catalog_activate: (args) => {
    activated.add(String(args.id));
    return null;
  },
  catalog_consent_prompt: (args) => ({
    host: imageHost(String(args.id)) ?? 'i.imgur.com',
    always_allow_prechecked: state.settings.catalog?.always_allow_host_prechecked !== false,
  }),
  catalog_grant: (args) => {
    const id = String(args.id);
    activated.add(id);
    const host = imageHost(id);
    if (args.scope === 'always' && host) {
      const hosts = (state.settings.catalog?.allowed_image_hosts as string[] | undefined) ?? [];
      state.settings.catalog = { ...state.settings.catalog, allowed_image_hosts: [...new Set([...hosts, host])] };
    }
    return { activated: [id] };
  },
};

// Everything the mock state needs is declared above.
resetMock();

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
    for (const event of state.onSubscribe) {
      push(structuredClone(event));
    }
    state.libraryMock.subscribed();
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
    profilesPreview.markApplied(state.profileData, state.library.selected_profile);
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
  ...catalogHandlers,
  downloads: () => structuredClone(state.tasks),
  download_install: (args) => startDownload(Number(args.modId), Number(args.fileId), String(args.name)),
  download_control: (args) => {
    const task = state.tasks.find((t) => t.id === args.id);
    if (task && args.action === 'pause') {
      task.state = { state: 'paused' };
    } else if (task && args.action === 'resume') {
      task.state = { state: 'downloading' };
    } else if (task && args.action === 'retry') {
      task.state = { state: 'queued' };
    } else if (task && args.action === 'cancel') {
      state.tasks = state.tasks.filter((t) => t !== task);
      push({ event: 'download', data: { event: 'removed', id: task.id } });
      return null;
    }
    if (task) {
      push({ event: 'download', data: { event: 'state', id: task.id, state: task.state } });
    }
    return null;
  },
  updates_check: () => ({ checks: [], available: 0, stopped: null }),
  game_status: () => ({
    running: state.gameRunning,
    root: state.gameRoot,
    installations: state.gameRoot ? 2 : 0,
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
    profilesPreview.markApplied(state.profileData, state.library.selected_profile);
    state.gameRunning = true;
    push({ event: 'game', data: { running: true } });
    return { library: structuredClone(state.library), changed: true, gameinfo_foreign: false };
  },
  choose: (args) => {
    const m = mod(args.modId);
    const files = (args.files as string[]) ?? [];
    for (const group of m.variant_groups) {
      const pick = files.find((f) => group.files.includes(f));
      if (pick) {
        group.active = pick;
      }
    }
    m.enabled = true;
    state.library.mods = [m, ...state.library.mods.filter((x) => x !== m)];
    return changed();
  },
  add_from_disk: () => {
    const imported = libraryMod(state.library.mods.length + 20, true, true);
    state.library.mods.unshift(imported);
    return [{ Ok: { mod_id: imported.id, new_files: imported.files.map((f) => f.id), choices: [], skipped: [] } }];
  },
  ...profilesPreview.handlers({
    names: () => state.profiles,
    setNames: (names) => {
      state.profiles = names;
    },
    library: () => state.library,
    data: () => state.profileData,
    scenario: () => (typeof location === 'undefined' ? null : new URLSearchParams(location.search).get('scenario')),
    startDownload,
  }),
  window_minimize: () => null,
  window_toggle_maximize: () => null,
  window_close: () => null,
  window_start_drag: () => null,
  // Like the core, only https pages are handed to the browser.
  open_link: (args) => {
    if (!/^https:\/\/\S+$/i.test(String(args.url))) {
      throw { id: 'Internal', args: { detail: `mock: only https pages are opened, not ${String(args.url)}` } };
    }
    return null;
  },
  favourites: () => structuredClone(state.favourites),
  set_favourite: (args) => {
    const favourite = args.favourite as Favourite;
    const at = state.favourites.findIndex((f) => f.section === favourite.section && f.id === favourite.id);
    if (args.on && at >= 0) {
      state.favourites[at] = structuredClone(favourite);
    } else if (args.on) {
      state.favourites.unshift(structuredClone(favourite));
    } else if (at >= 0) {
      state.favourites.splice(at, 1);
    }
    return structuredClone(state.favourites);
  },
  confirm_age: () => {
    state.settings.acknowledgements = {
      ...state.settings.acknowledgements,
      age_confirmed_at: new Date().toISOString().replace(/\.\d+Z$/, 'Z'),
    };
    return structuredClone(state.settings);
  },
  log_page: (args) => activity.page(state.logs, args.query as LogQuery, new Date()),
  log_entry: (args) => activity.entry(state.logs, String(args.id)),
  log_counts: (args) => activity.counts(state.logs, args.range as LogQuery['range'], new Date()),
  log_summary: () => activity.summary(state.logs, new Date()),
  open_logs_folder: () => String.raw`D:\Games\deadloader\deadloader-data\logs`,
};

/** Tauri's `invoke`, answered from the mock state after a short, realistic delay. */
export async function invoke<T>(command: string, args: Args = {}): Promise<T> {
  const handler = state.libraryMock.handlers()[command] ?? handlers[command] ?? maintenanceHandlers[command];
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
    if (!running) {
      state.libraryMock.gameClosed();
    }
  },
  /** A new network request in the log, pushed like the core does (Activity's live view). */
  addLogRequest(): void {
    activity.addRequest(state.logs, new Date());
    push({ event: 'log', data: { kinds: ['network'] } });
  },
};

// The browser console can drive the preview: `__mock.setGameRunning(true)`.
Object.assign(globalThis, { __mock: devtools });
