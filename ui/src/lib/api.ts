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
  | { event: 'notice'; data: Notice };

/**
 * Listens to the core's pushes. Call once after the app mounts; events the core sent before are delivered first.
 */
export async function subscribe(onEvent: (event: UiEvent) => void): Promise<void> {
  const events = new Channel<UiEvent>();
  events.onmessage = onEvent;
  await call<null>('subscribe', { events });
}

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
