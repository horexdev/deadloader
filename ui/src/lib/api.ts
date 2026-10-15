// The UI's only way to the core: typed wrappers around the app's Tauri commands (crates/app/src/commands.rs).
// Field names mirror the Rust types' serde output (snake_case); command arguments are camelCase, as Tauri expects.

import { Channel, invoke } from '@tauri-apps/api/core';

/** An error or notice from the core: a message id from spec 04-messages and named values for its text. */
export interface UserMessage {
  id: string;
  args: Record<string, string>;
}

export type Source =
  | { kind: 'gamebanana'; section: 'mod' | 'sound'; mod_id: number }
  | { kind: 'local'; imported_from: string | null };

export interface FileView {
  id: string;
  archive_name: string;
  inner_path: string;
  version: string | null;
  len: number;
  damaged: boolean;
  /** A newer version of this file is in the library (170): hint "Switch to" when this old one is active. */
  newer_in_library: string | null;
}

export interface VariantGroupView {
  files: string[];
  active: string | null;
}

export interface ModView {
  id: string;
  name: string;
  author: string | null;
  source: Source;
  enabled: boolean;
  files: FileView[];
  variant_groups: VariantGroupView[];
  /** [file id, enabled] for files that overlap nothing else in the mod. */
  add_ons: [string, boolean][];
  bytes: number;
}

export interface ConflictView {
  winner_mod: string;
  winner_file: string;
  loser_mod: string;
  loser_file: string;
  paths: string[];
}

export interface LibraryView {
  selected_profile: string;
  applied_profile: string | null;
  pending: boolean;
  mods: ModView[];
  conflicts: ConflictView[];
}

export interface Changed {
  library: LibraryView;
  applied: boolean;
  waiting_for_game: boolean;
}

export interface ApplyResult {
  library: LibraryView;
  changed: boolean;
  gameinfo_foreign: boolean;
}

export interface ImportOutcome {
  mod_id: string;
  new_files: string[];
  choices: string[][];
  skipped: string[];
}

export interface RemovalPlan {
  mod_id: string;
  files: string[];
  bytes: number;
  removes_mod: boolean;
  profiles: string[];
  applied_in: string[];
}

export interface Removed {
  files: string[];
  leftovers: string[];
}

export interface MissingFile {
  archive: string;
  inner_path: string;
  remote_file_id: number | null;
  version: string | null;
}

export interface Missing {
  name: string;
  source: Source;
  whole_mod: boolean;
  files: MissingFile[];
}

export interface ProfileImported {
  name: string;
  missing: Missing[];
}

/** settings.json as the core writes it; sections are typed where screens use them. */
export type Settings = Record<string, Record<string, unknown>>;

/** Windows' preferred UI languages (best first) and regional format, as the core reads them. */
export interface SystemLocale {
  languages: string[];
  region: string | null;
}

/** The translated texts of a system file dialog. */
export interface DialogText {
  title: string;
  filter: string;
}

/** Rust `Result<ImportOutcome, UserMessage>` as serde writes it. */
export type ImportResult = { Ok: ImportOutcome } | { Err: UserMessage };

/** What the start found (crates/app/src/events.rs `Notice`). */
export type Notice =
  | { kind: 'settings_corrected'; fields: string[] }
  | { kind: 'settings_unreadable'; message: UserMessage }
  | { kind: 'recovered'; undone: boolean }
  | { kind: 'swept'; files: number }
  | { kind: 'broken_profiles'; names: string[] }
  | { kind: 'damaged_files'; files: string[] };

/** A push from the core (crates/app/src/events.rs `UiEvent`). */
export type UiEvent =
  | { event: 'game'; data: { running: boolean } }
  | { event: 'applied_after_game'; data: { changes: number; library: LibraryView } }
  | { event: 'apply_after_game_failed'; data: { message: UserMessage } }
  | { event: 'drag_hover'; data: { over: boolean } }
  | { event: 'dropped'; data: { outcomes: ImportResult[] } }
  | { event: 'drop_failed'; data: { message: UserMessage } }
  | { event: 'notice'; data: Notice }
  | { event: 'catalog'; data: CatalogEvent }
  | { event: 'download'; data: DownloadEvent }
  | { event: 'installed'; data: { task: TaskId; installed: Installed } }
  | { event: 'updates'; data: UpdateReport }
  | { event: 'update_step'; data: { task: TaskId | null; step: UpdateStep } }
  | { event: 'log'; data: { kinds: LogKind[] } };

/**
 * Listens to the core's pushes. Call once after the app mounts; events the core sent before are delivered first.
 */
export async function subscribe(onEvent: (event: UiEvent) => void): Promise<void> {
  const events = new Channel<UiEvent>();
  events.onmessage = onEvent;
  await call<null>('subscribe', { events });
}

// ---- the catalog (crates/catalog/src/view.rs, error.rs; crates/app/src/catalog.rs)

export type Section = 'mod' | 'sound';
export type Sort =
  | 'newest'
  | 'latest_updated'
  | 'most_downloaded'
  | 'most_liked'
  | 'most_viewed'
  | 'most_commented'
  | 'oldest';

export type Filter = { kind: 'browse'; sort: Sort; category: number | null } | { kind: 'search'; text: string };

export interface Query {
  section: Section;
  filter: Filter;
  /** 1-based. */
  page: number;
}

/** An opaque picture id; show it with `imageUrl`. */
export type ImageId = string;

export type Origin = { kind: 'network' } | { kind: 'cache'; fetched_at_unix_ms: number; stale: boolean };

export interface CategoryRef {
  id: number;
  name: string;
}

export interface Thumbnail {
  /** 220 px; use while card width × display scale ≤ 220 (decision 152). */
  small: ImageId;
  /** 530 px. */
  large: ImageId;
}

export interface Card {
  id: number;
  section: Section;
  name: string;
  author: string | null;
  root_category: CategoryRef | null;
  subcategory: CategoryRef | null;
  likes: number;
  views: number;
  date_added: number | null;
  date_updated: number | null;
  thumbnail: Thumbnail | null;
  has_audio_preview: boolean;
  nsfw: boolean;
  obsolete: boolean;
  featured: boolean;
  has_files: boolean;
}

export interface Listing {
  items: Card[];
  hidden_nsfw: number;
  total: number;
  page: number;
  has_more: boolean;
  origin: Origin;
}

export interface Categories {
  items: Record<string, unknown>[];
  origin: Origin;
}

export interface GalleryImage {
  preview: ImageId;
  original: ImageId;
  caption: string | null;
}

export interface CatalogFile {
  id: number;
  file_name: string;
  size_bytes: number;
  date_added: number | null;
  download_count: number;
  description: string | null;
  version: string | null;
  md5: string | null;
  antivirus: unknown;
}

export type Access = 'load' | 'click' | 'consent';

export interface DescriptionImage {
  id: ImageId;
  host: string;
  alt: string | null;
  access: Access;
}

export interface Description {
  /** Sanitised HTML with `data-dl-link` / `data-dl-image` placeholders. */
  html: string;
  links: { url: string; host: string; internal_mod_id: number | null }[];
  images: DescriptionImage[];
  consent: { host: string; images: ImageId[] }[];
  truncated: boolean;
}

export interface ModPage {
  id: number;
  section: Section;
  name: string;
  version: string | null;
  author: string | null;
  credits: unknown[];
  category_path: CategoryRef[];
  gallery: GalleryImage[];
  has_audio_preview: boolean;
  short_description: string | null;
  description: Description;
  licence: Description | null;
  likes: number;
  views: number;
  downloads: number;
  date_added: number | null;
  date_modified: number | null;
  date_updated: number | null;
  nsfw: boolean;
  nsfw_hidden: boolean;
  content_ratings: unknown[];
  obsolete: boolean;
  embedded_media: string[];
  alternate_sources: unknown[];
  files: CatalogFile[];
  archived_files: CatalogFile[];
  origin: Origin;
}

/** The global network indicator. */
export type CatalogStatus =
  | { kind: 'local_mode' }
  | { kind: 'online' }
  | { kind: 'offline'; since_unix_ms: number }
  | { kind: 'rate_limited'; until_unix_ms: number; message: UserMessage }
  | { kind: 'api_changed'; message: UserMessage };

/** Why a listing, categories or a mod page can't be shown; the states without a message are shown in place. */
export type CatalogError =
  | { kind: 'local_mode' }
  | { kind: 'offline'; since_unix_ms: number }
  | { kind: 'rate_limited'; retry_in_s: number; message: UserMessage }
  | { kind: 'api_changed'; message: UserMessage }
  | { kind: 'blocked'; message: UserMessage }
  | { kind: 'refused'; message: UserMessage }
  | { kind: 'not_found'; other_game: number | null }
  | { kind: 'invalid_search'; problem: 'too_short' | 'too_long' }
  | { kind: 'internal'; message: UserMessage };

export type ConsentError =
  | { kind: 'unknown_id' }
  | { kind: 'not_a_description_image' }
  | { kind: 'needs_consent'; host: string }
  | { kind: 'save'; message: UserMessage };

export interface ConsentPrompt {
  host: string;
  always_allow_prechecked: boolean;
}

export type Scope = 'once' | 'always';

export interface Granted {
  activated: ImageId[];
}

export type CatalogEvent =
  | { event: 'status'; data: CatalogStatus }
  | { event: 'page_updated'; data: Query }
  | { event: 'categories_updated'; data: { section: Section; parent: number | null } }
  | { event: 'details_updated'; data: { section: Section; id: number } };

/** Where a catalog picture is loaded from: the core's image protocol, never the internet (ADR 0001). */
export function imageUrl(id: ImageId): string {
  // `.env.mock` points the browser preview at placeholder pictures; the app always uses the image protocol.
  return `${import.meta.env.VITE_IMAGE_ORIGIN ?? 'https://dl.localhost'}/img/${id}`;
}

function isCatalogAnswer(value: unknown): boolean {
  return typeof value === 'object' && value !== null && typeof (value as { kind?: unknown }).kind === 'string';
}

/** Like `call`, but the catalog's own answers (`{ kind, … }`) pass through as they are. */
async function callCatalog<T>(command: string, args: Record<string, unknown> = {}): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error: unknown) {
    if (isUserMessage(error) || isCatalogAnswer(error)) {
      throw error;
    }
    const internal: UserMessage = { id: 'Internal', args: { detail: String(error) } };
    throw internal;
  }
}

/** Failures are a `CatalogError` / `ConsentError` (`kind`) or a `UserMessage` (`id`) when the catalog is unavailable. */
export const catalog = {
  list: (query: Query, byUser: boolean) => callCatalog<Listing>('catalog_list', { query, byUser }),
  refresh: (query: Query) => callCatalog<Listing>('catalog_refresh', { query }),
  prefetch: (query: Query) => callCatalog<boolean>('catalog_prefetch', { query }),
  categories: (section: Section, parent: number | null) =>
    callCatalog<Categories>('catalog_categories', { section, parent }),
  details: (section: Section, id: number, listedUpdated: number | null) =>
    callCatalog<ModPage>('catalog_details', { section, id, listedUpdated }),
  status: () => callCatalog<CatalogStatus>('catalog_status'),
  retryNow: () => callCatalog<null>('catalog_retry_now'),
  activate: (id: ImageId) => callCatalog<null>('catalog_activate', { id }),
  consentPrompt: (id: ImageId) => callCatalog<ConsentPrompt>('catalog_consent_prompt', { id }),
  grant: (id: ImageId, scope: Scope) => callCatalog<Granted>('catalog_grant', { id, scope }),
};

// ---- downloads (crates/downloads/src/task.rs; crates/app/src/downloads.rs)

export type TaskId = number;

export type TaskState =
  | { state: 'queued' }
  | { state: 'downloading' }
  | { state: 'verifying' }
  | { state: 'installing' }
  | { state: 'paused' }
  | { state: 'waiting'; message: UserMessage; retry_in_s: number | null }
  | { state: 'failed'; message: UserMessage }
  | { state: 'done' };

/** A task's fixed data and where it stands (Rust flattens `TaskInfo` into it). */
export interface TaskSnapshot {
  id: TaskId;
  url: string;
  file_name: string;
  md5: string | null;
  size: number | null;
  initiator: 'user' | 'auto';
  /** mod_id, file_id, mod_name, author, version, section. */
  meta: Record<string, string>;
  state: TaskState;
  bytes: number;
  total: number | null;
  /** Bytes per second, smoothed. */
  speed: number;
}

export type DownloadEvent =
  | { event: 'added'; task: TaskSnapshot }
  | { event: 'state'; id: TaskId; state: TaskState }
  | { event: 'progress'; id: TaskId; bytes: number; total: number | null; speed: number }
  | { event: 'removed'; id: TaskId };

/** A finished download in the library; without `changed` the user picks variants first (D4, then `api.choose`). */
export interface Installed {
  outcome: ImportOutcome;
  changed: Changed | null;
}

export type DownloadAction = 'pause' | 'resume' | 'retry' | 'cancel';

export const downloads = {
  /** Install (F2) one file of the mod page on screen; `name` and `author` are what the page shows. */
  install: (section: Section, modId: number, fileId: number, name: string, author: string | null) =>
    call<TaskId>('download_install', { section, modId, fileId, name, author }),
  list: () => call<TaskSnapshot[]>('downloads'),
  control: (id: TaskId, action: DownloadAction) => call<null>('download_control', { id, action }),
};

// ---- mod updates (crates/core/src/updates.rs; crates/app/src/updates.rs)

export interface RemoteModFile {
  id: number;
  file_name: string;
  size_bytes: number;
  date_added: number | null;
  version: string | null;
  description: string | null;
}

export interface UpdatePost {
  id: number;
  title: string;
  version: string | null;
  date_added: number | null;
  changelog: { category: string | null; text: string }[];
  file_ids: number[];
  /** Untrusted HTML: never render it as is. */
  text_html: string;
}

export type RemovedReason = 'deleted' | 'trashed' | 'withheld' | 'private' | 'no_files';

export interface Replaced {
  remote_file_id: number;
  archive_name: string;
  files: string[];
  candidate: { file: RemoteModFile; matched_by: 'same' | 'without_suffix'; in_library: string[] } | null;
}

export type UpdateStatus =
  | { kind: 'local' }
  | { kind: 'not_checked' }
  | { kind: 'untracked' }
  | { kind: 'up_to_date' }
  | { kind: 'new_files'; files: RemoteModFile[] }
  | {
      kind: 'update_available';
      version: string | null;
      date_updated: number | null;
      replaced: Replaced[];
      kept: number[];
      others: RemoteModFile[];
      posts: UpdatePost[];
    }
  | { kind: 'removed'; reason: RemovedReason };

export interface UpdateReport {
  checks: { mod_id: string; status: UpdateStatus }[];
  /** N3. */
  available: number;
  /** Why the check stopped early; the mods not asked are `not_checked`. */
  stopped: CatalogError | null;
}

export const updates = {
  /** "Check for updates" (38). */
  check: () => callCatalog<UpdateReport>('updates_check'),
  /** "Hide" on "new files available" (168). */
  dismiss: (modId: string, fileIds: number[]) => call<null>('updates_dismiss', { modId, fileIds }),
  /** "Update" in D5: per replaced GameBanana file, the listed file that replaces it, or `null` to keep the old one. */
  install: (modId: string, picks: UpdatePick[]) => call<UpdateStarted>('update_install', { modId, picks }),
  /** The user's mapping after a `needs_mapping` step; `new: null` keeps an old file as it is. */
  map: (modId: string, mappings: { old: string; new: string | null }[], version: string | null) =>
    call<Changed>('update_map', { modId, mappings, version }),
};

export interface UpdatePick {
  replaced: number;
  file: number | null;
}

/** Where an old library file goes (D5). */
export type Proposed =
  | { kind: 'pair'; new: string }
  | { kind: 'overlap'; new: string; shared_paths: number; identical: number }
  | { kind: 'tie'; candidates: string[]; shared_paths: number; identical: number }
  | { kind: 'no_overlap' };

export type UpdateStep =
  | { kind: 'applied'; mod_id: string; changed: Changed }
  | {
      kind: 'needs_mapping';
      mod_id: string;
      replaced: number;
      new_files: string[];
      proposals: { old: string; proposed: Proposed }[];
      version: string | null;
    };

export interface UpdateStarted {
  tasks: TaskId[];
  steps: UpdateStep[];
}

// ---- linking a mod imported from disk to its GameBanana page (F10, 171; crates/core/src/session.rs)

export interface LinkedArchive {
  archive_name: string;
  files: string[];
  /** The GameBanana file it is; `null` when nothing matched: offer "Link" by hand. */
  remote_file_id: number | null;
  matched_by: 'md5' | 'name' | null;
}

export interface LinkReport {
  mod_id: string;
  archives: LinkedArchive[];
}

/** Our own problems with the address (shown next to the field), a catalog answer, or a message. */
export type LinkFailure =
  | { kind: 'not_a_page' }
  | { kind: 'removed'; reason: RemovedReason }
  | CatalogError
  | UserMessage;

export const links = {
  /** The GameBanana page address pasted for a mod imported from disk. */
  link: (modId: string, url: string) => callCatalog<LinkReport>('mod_link', { modId, url }),
  /** "Link" by hand for an archive nothing matched; `null` unlinks. */
  linkFiles: (modId: string, files: string[], remoteFileId: number | null) =>
    call<null>('mod_link_files', { modId, files, remoteFileId }),
  rename: (modId: string, name: string) => call<LibraryView>('mod_rename', { modId, name }),
};

// ---- the window frame and the sidebar (crates/app/src/commands/shell.rs)

export interface ApplyPreview {
  turned_off: string[];
  turned_on: string[];
}

/** The sidebar's game status: the folder the selected profile uses, else the first one Steam knows. */
export interface GameStatus {
  running: boolean;
  root: string | null;
  installations: number;
}

export const shell = {
  /** Play (F4): applies the selected profile, then Steam starts Deadlock. Ask with D13 first (`applyPreview`). */
  play: () => call<ApplyResult>('play'),
  /** D13: the mods Apply or Play would take out of the game and bring into it, by name. */
  applyPreview: () => call<ApplyPreview>('apply_preview'),
  gameStatus: () => call<GameStatus>('game_status'),
  minimize: () => call<null>('window_minimize'),
  toggleMaximize: () => call<null>('window_toggle_maximize'),
  close: () => call<null>('window_close'),
  /** On a press on the title bar's empty area. */
  startDrag: () => call<null>('window_start_drag'),
};

export function isUserMessage(value: unknown): value is UserMessage {
  return (
    typeof value === 'object' &&
    value !== null &&
    typeof (value as UserMessage).id === 'string' &&
    typeof (value as UserMessage).args === 'object' &&
    (value as UserMessage).args !== null
  );
}

async function call<T>(command: string, args: Record<string, unknown> = {}): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error: unknown) {
    if (isUserMessage(error)) {
      throw error;
    }
    const internal: UserMessage = { id: 'Internal', args: { detail: String(error) } };
    throw internal;
  }
}

export const api = {
  systemLocale: () => call<SystemLocale>('system_locale'),
  library: () => call<LibraryView>('library'),
  setEnabled: (modId: string, enabled: boolean) => call<Changed>('set_enabled', { modId, enabled }),
  moveMod: (modId: string, index: number) => call<Changed>('move_mod', { modId, index }),
  switchVariant: (modId: string, file: string) => call<Changed>('switch_variant', { modId, file }),
  choose: (modId: string, files: string[]) => call<Changed>('choose', { modId, files }),
  selectProfile: (name: string) => call<LibraryView>('select_profile', { name }),
  createProfile: (name: string) => call<string[]>('create_profile', { name }),
  profileNames: () => call<string[]>('profile_names'),
  importPaths: (paths: string[]) => call<ImportResult[]>('import_paths', { paths }),
  apply: () => call<ApplyResult>('apply'),
  removalPlan: (modId: string, files?: string[]) =>
    call<RemovalPlan>('removal_plan', { modId, files: files ?? null }),
  remove: (modId: string, files?: string[]) => call<Removed>('remove', { modId, files: files ?? null }),
  renameProfile: (old: string, newName: string) => call<string[]>('rename_profile', { old, new: newName }),
  duplicateProfile: (name: string, newName: string) => call<string[]>('duplicate_profile', { name, newName }),
  deleteProfile: (name: string) => call<string[]>('delete_profile', { name }),
  exportProfile: (name: string) => call<string>('export_profile', { name }),
  importProfile: (text: string) => call<ProfileImported>('import_profile', { text }),
  settings: () => call<Settings>('settings'),
  updateSettings: (settings: Settings) => call<Settings>('update_settings', { settings }),
  addFromDisk: (text: DialogText) => call<ImportResult[]>('add_from_disk', { text }),
  exportProfileDialog: (name: string, text: DialogText) => call<boolean>('export_profile_dialog', { name, text }),
  importProfileDialog: (text: DialogText) => call<ProfileImported | null>('import_profile_dialog', { text }),
};

// ---- Activity: the logs (crates/core/src/log.rs, activity.rs; crates/app/src/commands/activity.rs)

export type LogKind = 'network' | 'files' | 'app';
export type LogLevel = 'debug' | 'info' | 'warn' | 'error';
export type LogRange = 'hour' | 'today' | '7d' | '30d';
export type LogResults = 'all' | 'ok' | 'error';
export type HostSource = 'builtin' | 'github' | 'user' | 'user-image';
export type Purpose = 'catalog' | 'thumbnail' | 'description-image' | 'download' | 'update-check';
export type FileOp = 'hardlink' | 'copy' | 'rename' | 'delete' | 'write' | 'backup' | 'restore';

/** The fields every log line starts with; `ts` is RFC 3339 with the local offset and milliseconds. */
export interface LogLine {
  ts: string;
  id: string;
  lvl: LogLevel;
  msg: string;
}

/** One hop or try of a request (`network-*.jsonl`). */
export interface NetworkLine extends LogLine {
  req: string;
  hop: number;
  /** Written only for retries; 1 when missing. */
  attempt?: number;
  method: string;
  url: string;
  host_source?: HostSource;
  initiator: 'auto' | 'user';
  purpose: Purpose;
  status?: number;
  /** The announced `Content-Length`. */
  bytes?: number;
  ms: number;
  range?: string;
  error?: string;
  /** Refused before any byte left the machine: `host-not-listed`, `local-mode`, `not-https`… */
  blocked?: string;
}

export interface FileLine extends LogLine {
  op: FileOp;
  src?: string;
  dst?: string;
  bytes?: number;
  sha256?: string;
  /** The change (journal transaction) it belonged to. */
  txn?: number;
  result: 'ok' | 'failed';
  error?: string;
}

export interface AppLine extends LogLine {
  fields?: Record<string, unknown>;
}

/** A row of the list with its full record: a request with every hop and try (oldest first), or one line. */
export type LogEntry =
  | { kind: 'network'; req: string; lines: NetworkLine[] }
  | { kind: 'files'; record: FileLine }
  | { kind: 'app'; record: AppLine };

/** Where a page ended; hand it back as it came. */
export interface LogCursor {
  file: string;
  offset: number;
  skip: string[];
}

export interface LogQuery {
  kind: LogKind;
  range: LogRange;
  purpose?: Purpose | null;
  op?: FileOp | null;
  level?: LogLevel | null;
  result?: LogResults;
  host?: string | null;
  text?: string;
  cursor?: LogCursor | null;
  limit?: number;
}

export interface LogPage {
  entries: LogEntry[];
  /** `null`: nothing older in the range. */
  cursor: LogCursor | null;
}

export interface LogCounts {
  network: { requests: number; failed: number; purposes: Partial<Record<Purpose, number>>; hosts: Record<string, number> };
  files: { operations: number; failed: number; ops: Partial<Record<FileOp, number>> };
  app: { events: number; errors: number; levels: Partial<Record<LogLevel, number>> };
}

export interface LogSummary {
  contacted: { host: string; requests: number; source: HostSource | null }[];
  blocked: { host: string; requests: number; last: string }[];
  game: {
    changes: number;
    ops: Partial<Record<FileOp, number>>;
    gameinfo: number;
    failed: number;
    batches: number;
    folders: string[];
  };
  /** Nothing in the network or files log at all: a fresh install. */
  empty: boolean;
}

export const activity = {
  page: (query: LogQuery) => call<LogPage>('log_page', { query }),
  /** One entry by line id or request id; a network line gives its whole request. */
  entry: (id: string) => call<LogEntry | null>('log_entry', { id }),
  counts: (range: LogRange) => call<LogCounts>('log_counts', { range }),
  summary: () => call<LogSummary>('log_summary'),
  /** Opens deadloader's logs folder in File Explorer; resolves to the folder as it may be shown. */
  openFolder: () => call<string>('open_logs_folder'),
};
