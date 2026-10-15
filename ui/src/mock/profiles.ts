// The Profiles screen's preview data and commands for the mock core: four profiles on two game installations
// (board `Profiles`), import errors and an import with missing mods (D9). Invented names, like the rest of the mock.

import type {
  LibraryView,
  MissingDownload,
  MissingStarted,
  ProfileImported,
  ProfileOverview,
  ProfilesView,
} from '../lib/api';

export const D = String.raw`D:\SteamLibrary\steamapps\common\Deadlock`;
export const E = String.raw`E:\SteamLibrary\steamapps\common\Deadlock`;

const HOUR = 3_600_000;
const DAY = 24 * HOUR;

interface Info {
  root: string | null;
  last: number | null;
  /** Enabled mods of a profile that isn't the selected one (the selected one's come from the library). */
  mods: string[];
}

export interface ProfilesMock {
  info: Record<string, Info>;
  /** Which profile each game folder holds. */
  applied: Record<string, string>;
}

/** The profiles the mock starts with; `names` are the core's (`profile_names`). */
export function fresh(names: string[], library: LibraryView): ProfilesMock {
  const now = Date.now();
  const all = library.mods.map((m) => m.name);
  const info: Record<string, Info> = {
    Default: { root: D, last: now - 2 * HOUR, mods: [] },
    Evening: { root: D, last: now - 3 * DAY, mods: all.slice(0, 4) },
    Ranked: { root: D, last: now - 21 * DAY, mods: all.slice(1, 3) },
    Sasha: { root: E, last: now - 26 * HOUR, mods: all.slice(3, 4) },
  };
  for (const name of names) {
    info[name] ??= { root: null, last: null, mods: [] };
  }
  return { info, applied: { [D]: library.applied_profile ?? 'Default', [E]: 'Sasha' } };
}

const same = (a: string, b: string) => a.toLowerCase() === b.toLowerCase();

function without<T>(record: Record<string, T>, key: string): Record<string, T> {
  return Object.fromEntries(Object.entries(record).filter(([k]) => k !== key));
}

export interface Host {
  names: () => string[];
  setNames: (names: string[]) => void;
  library: () => LibraryView;
  data: () => ProfilesMock;
  scenario: () => string | null;
  startDownload: (modId: number, fileId: number, name: string) => number;
}

/** The commands of the Profiles screen, on the mock state `host` gives. */
export function handlers(host: Host): Record<string, (args: Record<string, unknown>) => unknown> {
  const view = (): ProfilesView => {
    const library = host.library();
    const data = host.data();
    const profiles: ProfileOverview[] = host.names().map((name) => {
      const info = data.info[name] ?? { root: null, last: null, mods: [] };
      const mods = name === library.selected_profile ? library.mods.filter((m) => m.enabled).map((m) => m.name) : info.mods;
      const root = info.root ?? D;
      return {
        name,
        game_root: info.root,
        mods: mods.length,
        mod_names: mods,
        applied: data.applied[root] === name,
        last_applied_unix_ms: info.last,
      };
    });
    return { selected: library.selected_profile, installations: [D, E], profiles };
  };
  const exists = (name: string) => host.names().some((n) => same(n, name));
  const taken = (name: string) => ({ id: 'E21', args: { name } });
  return {
    profiles_overview: () => view(),
    set_profile_installation: (args) => {
      const name = String(args.name);
      const info = host.data().info[name];
      if (info) {
        info.root = String(args.root);
      }
      return view();
    },
    rename_profile: (args) => {
      const [old, next] = [String(args.old), String(args.new)];
      if (exists(next) && !same(old, next)) {
        throw taken(next);
      }
      const data = host.data();
      host.setNames(host.names().map((n) => (n === old ? next : n)));
      data.info[next] = data.info[old] ?? { root: null, last: null, mods: [] };
      data.info = without(data.info, old);
      for (const root of Object.keys(data.applied)) {
        if (data.applied[root] === old) {
          data.applied[root] = next;
        }
      }
      const library = host.library();
      if (library.selected_profile === old) {
        library.selected_profile = next;
      }
      if (library.applied_profile === old) {
        library.applied_profile = next;
      }
      return host.names();
    },
    duplicate_profile: (args) => {
      const [name, next] = [String(args.name), String(args.newName)];
      if (exists(next)) {
        throw taken(next);
      }
      const data = host.data();
      const from = data.info[name];
      data.info[next] = { root: from?.root ?? null, last: null, mods: [...(from?.mods ?? [])] };
      host.setNames([...host.names(), next]);
      return host.names();
    },
    delete_profile: (args) => {
      const name = String(args.name);
      const data = host.data();
      host.setNames(host.names().filter((n) => n !== name));
      data.info = without(data.info, name);
      data.applied = Object.fromEntries(Object.entries(data.applied).filter(([, n]) => n !== name));
      const library = host.library();
      if (library.applied_profile === name) {
        library.applied_profile = null;
      }
      if (library.selected_profile === name) {
        library.selected_profile = host.names().find((n) => n === 'Default') ?? host.names()[0] ?? 'Default';
      }
      library.pending = library.selected_profile !== library.applied_profile;
      return host.names();
    },
    // The path the user picked in the save dialog; `profiles-export-cancelled`: the dialog was cancelled.
    export_profile_dialog: (args) =>
      host.scenario() === 'profiles-export-cancelled' ? null : `${String.raw`C:\Users\Player\Documents`}\\${String(args.name)}.json`,
    export_profile: (args) => JSON.stringify({ deadloader_profile: 1, name: String(args.name), mods: [] }),
    import_profile: (args): ProfileImported => {
      const base = String((JSON.parse(String(args.text)) as { name?: string }).name ?? 'Imported');
      let name = base;
      for (let n = 2; exists(name); n += 1) {
        name = `${base} (${n})`;
      }
      host.setNames([...host.names(), name]);
      host.data().info[name] = { root: null, last: null, mods: [] };
      host.library().selected_profile = name;
      return { name, missing: [], listed: 0, file: null };
    },
    import_profile_dialog: (): ProfileImported | null => {
      const scenario = host.scenario();
      if (scenario === 'profiles-import-error') {
        throw { id: 'E17', args: { reason: 'newer', found: '3', supported: '1', file: 'weekend-max.json' } };
      }
      if (scenario === 'profiles-not-a-profile') {
        throw { id: 'E17', args: { reason: 'not_a_profile', file: 'skins-backup.zip' } };
      }
      const missing = scenario === 'profiles-missing-mods';
      let name = missing ? 'Max ranked' : 'Shared';
      for (let n = 2; exists(name); n += 1) {
        name = `${missing ? 'Max ranked' : 'Shared'} (${n})`;
      }
      host.setNames([...host.names(), name]);
      const library = host.library();
      host.data().info[name] = { root: null, last: null, mods: library.mods.slice(0, 5).map((m) => m.name) };
      library.selected_profile = name;
      library.pending = true;
      return {
        name,
        listed: missing ? 8 : 5,
        file: missing ? 'max-ranked.json' : 'shared.json',
        missing: missing
          ? [
              {
                name: 'Numeric Health v2',
                source: { kind: 'gamebanana', section: 'mod', mod_id: 700_090 },
                whole_mod: true,
                files: [{ archive: 'numeric_health_v2.zip', inner_path: 'pak01_dir.vpk', remote_file_id: 1_990_001, version: 'v2' }],
              },
              {
                name: 'Top Bar Plus',
                source: { kind: 'gamebanana', section: 'mod', mod_id: 700_091 },
                whole_mod: true,
                files: [{ archive: 'top_bar_plus.zip', inner_path: 'pak01_dir.vpk', remote_file_id: 1_990_002, version: 'v3' }],
              },
              {
                name: 'crosshair_dot_small.vpk',
                source: { kind: 'local', imported_from: null },
                whole_mod: true,
                files: [{ archive: 'crosshair_dot_small.vpk', inner_path: 'crosshair_dot_small.vpk', remote_file_id: null, version: null }],
              },
            ]
          : [],
      };
    },
    profile_download_missing: (args): MissingStarted => {
      const mods = (args.mods as MissingDownload[]) ?? [];
      return {
        tasks: mods.flatMap((m) => m.file_ids.map((id) => host.startDownload(m.mod_id, id, m.name))),
        failed: [],
      };
    },
  };
}

/** Apply or Play put the selected profile into its game folder. */
export function markApplied(data: ProfilesMock, name: string): void {
  const info = (data.info[name] ??= { root: null, last: null, mods: [] });
  data.applied[info.root ?? D] = name;
  info.last = Date.now();
}
