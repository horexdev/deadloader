// The UI's only way to the core: typed wrappers around the app's Tauri commands (crates/app/src/commands.rs).
// Field names mirror the Rust types' serde output (snake_case); command arguments are camelCase, as Tauri expects.

import { invoke } from '@tauri-apps/api/core';

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

/** Rust `Result<ImportOutcome, UserMessage>` as serde writes it. */
export type ImportResult = { Ok: ImportOutcome } | { Err: UserMessage };

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
};
