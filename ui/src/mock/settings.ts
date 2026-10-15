// The mock core's Settings, first-run and maintenance commands (S8, S1, D7, D8), with invented data. Scenarios from
// the address: `wizard`, `wizard-changed`, `wizard-setup-issues`, `wizard-no-write`, `settings-local`,
// `settings-advanced-error` (the boards SettingsScreen.dc.html and Wizard.dc.html).

import type {
  About,
  CacheUsage,
  EditResult,
  InstallationView,
  RollbackPreview,
  SetupCheck,
  Settings,
  WipeSummary,
} from '../lib/api';

/** What the mock core shares with these handlers. */
export interface Shared {
  settings(): Settings;
  setSettings(settings: Settings): void;
  profiles(): string[];
  selectedProfile(): string;
  gameRunning(): boolean;
  scenario(): string | null;
}

interface MaintenanceState {
  cache: CacheUsage;
  installations: InstallationView[];
  rolledBack: boolean;
  backups: number;
  manifestText: string;
}

const STEAM_D = String.raw`D:\SteamLibrary\steamapps\common\Deadlock`;
const STEAM_E = String.raw`E:\SteamLibrary\steamapps\common\Deadlock`;
const DATA = String.raw`D:\Games\deadloader\deadloader-data`;

function freshState(shared: Shared): MaintenanceState {
  const issues = shared.scenario() === 'wizard-setup-issues';
  return {
    cache: { bytes: 64_200_000, thumbnails: 4812, images: 140, pages: 38 },
    installations: issues
      ? []
      : [
          {
            game_root: STEAM_D,
            source: 'steam',
            library: String.raw`D:\SteamLibrary`,
            build_id: 20_314_556,
            profiles: [shared.selectedProfile(), 'Weekend'],
            applied_profile: shared.selectedProfile(),
            selected: true,
            gameinfo: 'ours',
            files: 10,
          },
          {
            game_root: STEAM_E,
            source: 'steam',
            library: String.raw`E:\SteamLibrary`,
            build_id: 20_314_556,
            profiles: ['Ranked'],
            applied_profile: 'Ranked',
            selected: false,
            gameinfo: 'ours',
            files: 6,
          },
        ],
    rolledBack: false,
    backups: 4,
    manifestText: JSON.stringify(
      { schema: 1, mods: [], installations: [{ game_root: STEAM_D, applied_profile: 'Default', applied: [] }] },
      null,
      2,
    ),
  };
}

function settingsText(settings: Settings): string {
  return JSON.stringify({ schema: 1, ...settings }, null, 2);
}

/** The settings file with a missing comma, as the board `SettingsAdvancedError` shows it. */
export function brokenSettingsText(settings: Settings): string {
  return settingsText(settings).replace('"gif_autoplay": false,', '"gif_autoplay": false');
}

/** A JSON syntax error's line and column, like serde's. */
function syntaxError(text: string): { line: number; column: number } | null {
  try {
    JSON.parse(text);
    return null;
  } catch (error) {
    const at = /position (\d+)/.exec(String(error));
    const before = text.slice(0, at ? Number(at[1]) : text.length);
    const lines = before.split('\n');
    return { line: lines.length, column: (lines.at(-1)?.length ?? 0) + 1 };
  }
}

function stamp(): string {
  const d = new Date();
  const two = (n: number) => String(n).padStart(2, '0');
  return `${d.getFullYear()}${two(d.getMonth() + 1)}${two(d.getDate())}-${two(d.getHours())}${two(d.getMinutes())}`;
}

export function settingsHandlers(shared: Shared): Record<string, (args: Record<string, unknown>) => unknown> {
  const state = freshState(shared);
  const scenario = () => shared.scenario();
  const selectedRoot = () => state.installations.find((i) => i.selected)?.game_root ?? null;

  const preview = (): RollbackPreview => {
    const root = selectedRoot();
    if (!root) {
      throw { id: 'E2', args: {} };
    }
    return {
      game_root: root,
      files: state.rolledBack ? 0 : 10,
      bytes: state.rolledBack ? 0 : 236_000_000,
      gameinfo: state.rolledBack ? 'vanilla' : 'ours',
      foreign: [
        { name: 'pak01_dir.vpk', bytes: 38_200_000, from_dmm: true },
        { name: '.dmm.json', bytes: 2_048, from_dmm: true },
      ],
      profiles: [shared.selectedProfile(), 'Weekend'],
      selected_profile: shared.selectedProfile(),
      all_profiles: shared.profiles(),
      library_mods: 11,
      library_bytes: 236_000_000,
      others: [{ game_root: STEAM_E, applied_profile: 'Ranked', files: 6 }],
    };
  };

  return {
    settings_reset: () => {
      const s = shared.settings();
      const fresh: Settings = {
        ...structuredClone(s),
        general: { language: 'system', startup_profile: 'last_selected', check_app_updates: false, zoom_percent: 100 },
      };
      shared.setSettings(fresh);
      state.backups += 1;
      return { backup: `settings.json.bak-${stamp()}`, settings: structuredClone(fresh) };
    },
    settings_file_text: (args) => {
      if (args.file === 'manifest') {
        return state.manifestText;
      }
      return scenario() === 'settings-advanced-error'
        ? brokenSettingsText(shared.settings())
        : settingsText(shared.settings());
    },
    settings_file_save: (args): EditResult => {
      const text = String(args.text);
      const error = syntaxError(text);
      if (error) {
        return { Err: { kind: 'json', ...error } };
      }
      const parsed = JSON.parse(text) as Record<string, unknown>;
      if (args.file === 'manifest') {
        state.manifestText = text;
        return { Ok: { backup: `manifest.json.bak-${stamp()}` } };
      }
      const network = parsed.network as Record<string, unknown> | undefined;
      const retries = Number(network?.retries ?? 5);
      if (retries > 10) {
        return {
          Err: { kind: 'problems', problems: [{ path: 'network.retries', problem: 'out_of_range', found: retries, min: 0, max: 10 }] },
        };
      }
      const settings = { ...parsed };
      delete settings.schema;
      shared.setSettings(settings as Settings);
      state.backups += 1;
      return { Ok: { backup: `settings.json.bak-${stamp()}` } };
    },
    cache_usage: () => ({ ...state.cache }),
    cache_clear: () => {
      const freed = { bytes: state.cache.bytes, files: state.cache.thumbnails + state.cache.images + state.cache.pages, failed: 0 };
      state.cache = { bytes: 0, thumbnails: 0, images: 0, pages: 0 };
      return freed;
    },
    installations: () => structuredClone(state.installations),
    game_folder_dialog: (args) => {
      const root = String.raw`F:\Games\Deadlock`;
      if (!state.installations.some((i) => i.game_root === root)) {
        state.installations.push({
          game_root: root,
          source: 'manual',
          library: null,
          build_id: null,
          profiles: [],
          applied_profile: null,
          selected: false,
          gameinfo: 'vanilla',
          files: 0,
        });
        const s = structuredClone(shared.settings());
        const game = (s.game ?? {}) as { manual_installations?: string[] };
        game.manual_installations = [...(game.manual_installations ?? []), root];
        s.game = game;
        shared.setSettings(s);
      }
      if (args.select === true) {
        for (const i of state.installations) {
          i.selected = i.game_root === root;
        }
      }
      return root;
    },
    installation_remove: (args) => {
      state.installations = state.installations.filter((i) => i.source !== 'manual' || i.game_root !== args.gameRoot);
      const s = structuredClone(shared.settings());
      const game = (s.game ?? {}) as { manual_installations?: string[] };
      game.manual_installations = (game.manual_installations ?? []).filter((m) => m !== args.gameRoot);
      s.game = game;
      shared.setSettings(s);
      return structuredClone(s);
    },
    open_folder: () => null,
    // A stand-in until the Catalog screen brings its own (D1).
    open_link: () => null,
    about: (): About => ({
      version: '0.1.0',
      commit: '3f9c2ab0d7e14c5b9a6f8e2d1c0b9a8f7e6d5c4b',
      webview2: '155.0.4283.45',
      data_folder: DATA,
      data_bytes: 312_600_000 - (state.cache.bytes === 0 ? 64_200_000 : 0),
      exe_sha256: '5e0d1c7a9b3f48e2c61a07d4f9b25e88d13c0a6f7e4b92d5180c3fa6b7e29d41',
    }),
    rollback_preview: () => preview(),
    rollback: (args) => {
      if (shared.gameRunning()) {
        throw { id: 'E13', args: {} };
      }
      const p = preview();
      state.rolledBack = true;
      for (const i of state.installations) {
        if (i.selected) {
          i.gameinfo = 'vanilla';
          i.files = 0;
          i.applied_profile = null;
        }
      }
      return {
        game_root: p.game_root,
        removed: p.files,
        foreign_removed: args.removeForeign === true ? p.foreign.map((f) => f.name) : [],
        gameinfo_restored: true,
        gameinfo: 'vanilla',
      };
    },
    wipe_summary: (): WipeSummary => ({
      library: { mods: 11, files: 14, bytes: 236_000_000 },
      profiles: { names: shared.profiles(), bytes: 24_000 },
      logs: { files: 51, oldest: '2026-09-16', bytes: 12_400_000 },
      cache_bytes: state.cache.bytes,
      settings: { backups: state.backups, bytes: 24_000 },
      other_bytes: 1_200_000,
      total_bytes: 236_000_000 + 24_000 + 12_400_000 + state.cache.bytes + 24_000 + 1_200_000,
    }),
    delete_all_data: () => [],
    setup_check: (): SetupCheck => {
      const issues = scenario() === 'wizard-setup-issues';
      const root = issues ? null : selectedRoot();
      return {
        game_root: root,
        installations: state.installations.length,
        gameinfo: issues ? 'foreign' : root ? 'vanilla' : 'missing',
        foreign: issues
          ? [
              { name: 'pak01_dir.vpk', bytes: 38_200_000, from_dmm: true },
              { name: 'pak02_dir.vpk', bytes: 4_100_000, from_dmm: true },
              { name: '.dmm.json', bytes: 2_048, from_dmm: true },
            ]
          : [],
        exe_folder: issues ? String.raw`C:\Users\Alex\Downloads\deadloader` : String.raw`D:\Games\deadloader`,
        exe_location: issues ? 'downloads' : 'permanent',
        data_volume: issues ? 'C:' : 'D:',
        game_volume: issues ? 'E:' : root ? 'D:' : null,
        same_volume: issues ? false : root ? true : null,
      };
    },
    acknowledge_notice: (args) => {
      const s = structuredClone(shared.settings());
      const catalog = args.catalog === 'catalog';
      s.acknowledgements = {
        ...s.acknowledgements,
        notice: { version: Number(args.version), at: new Date().toISOString().slice(0, 19) + 'Z', language: String(args.language), catalog: args.catalog },
      };
      s.catalog = { ...s.catalog, enabled: catalog };
      s.general = { ...s.general, check_app_updates: catalog && args.checkAppUpdates === true };
      shared.setSettings(s);
      return structuredClone(s);
    },
    choose_catalog: (args) => {
      const s = structuredClone(shared.settings());
      const enabled = args.enabled === true;
      s.catalog = { ...s.catalog, enabled };
      if (!enabled) {
        s.general = { ...s.general, check_app_updates: false };
      }
      const notice = s.acknowledgements?.notice as Record<string, unknown> | null | undefined;
      if (notice) {
        notice.catalog = enabled ? 'catalog' : 'local_mode';
      }
      shared.setSettings(s);
      return structuredClone(s);
    },
  };
}
