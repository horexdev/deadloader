// The settings as the Settings screen edits them: the typed shape of `settings.json` (crates/core/src/settings.rs),
// the defaults shown next to changed rows ("Default: … · Restore default"), the core's ranges, and pure helpers.

import { ZOOM_STEPS } from '../../lib/zoom';
import type { Settings } from '../../lib/api';

export type NsfwMode = 'hidden' | 'blurred' | 'shown';
export type Proxy = { mode: 'none' } | { mode: 'system' } | { mode: 'manual'; address: string };

export interface SettingsData {
  general: {
    language: string;
    startup_profile: 'last_selected' | { named: string };
    check_app_updates: boolean;
    zoom_percent: number;
  };
  game: { auto_fix_gameinfo: boolean; manual_installations: string[] };
  catalog: {
    enabled: boolean;
    nsfw: NsfwMode;
    description_images: 'on_click' | 'always';
    always_allow_host_prechecked: boolean;
    allowed_image_hosts: string[];
    gif_autoplay: boolean;
    external_links: 'ask' | 'open_immediately';
    page_ttl_minutes: number;
    prefetch_next_page: boolean;
  };
  library: {
    check_updates_at_start: boolean;
    download_updates_automatically: boolean;
    deletion: 'permanent' | 'recycle_bin';
    old_versions: 'keep_all';
  };
  downloads: {
    parallel: number;
    speed_limit_kib_per_s: number | null;
    keep_archives: boolean;
    max_extracted_mib: number;
    max_extracted_factor: number;
    max_vpk_files: number;
    max_entry_ratio: number;
  };
  network: { parallel_api_requests: number; connect_timeout_s: number; read_timeout_s: number; retries: number; proxy: Proxy };
  cache: { thumbnail_limit_mib: number };
  logs: { retention_days: number; size_limit_mib: number; mask_user_path: boolean };
  advanced: { extra_allowed_domains: string[] };
  ui: { sidebar_collapsed: boolean; last_selected_profile: string | null };
  acknowledgements: {
    notice: { version: number; at: string; language: string; catalog: 'catalog' | 'local_mode' } | null;
    age_confirmed_at: string | null;
  };
}

/** `Settings::default()` with the fields this screen shows. */
export const DEFAULTS = {
  language: 'system',
  startupProfile: 'last_selected',
  checkAppUpdates: false,
  zoom: 100,
  autoFix: true,
  nsfw: 'hidden' as NsfwMode,
  descriptionImages: 'on_click',
  prechecked: true,
  gif: false,
  links: 'ask',
  ttl: 10,
  prefetch: true,
  checkUpdates: true,
  autoDownload: false,
  deletion: 'permanent',
  oldVersions: 'keep_all',
  parallel: 4,
  speed: null as number | null,
  keepArchives: false,
  extractedMib: 2048,
  factor: 20,
  vpkFiles: 256,
  apiParallel: 2,
  connect: 10,
  read: 30,
  retries: 5,
  proxy: 'none',
  thumbnailMib: 100,
  logDays: 30,
  logMib: 100,
  mask: true,
} as const;

/** The core's ranges (`settings::ranges`); the UI checks them before saving, the core again. */
export const RANGES = {
  extractedGb: { min: 1, max: 64 },
  factor: { min: 1, max: 1000 },
  vpkFiles: { min: 1, max: 4096 },
  connect: { min: 1, max: 120 },
  read: { min: 1, max: 600 },
  proxyPort: { min: 1, max: 65_535 },
} as const;

/** The design's choices; a value set by hand outside them is added so the select still shows it. */
export const CHOICES = {
  zoom: ZOOM_STEPS,
  ttl: [2, 5, 10, 30, 60],
  parallel: [1, 2, 3, 4, 5, 6, 8],
  /** MB/s; `null` = no limit. */
  speedMb: [null, 1, 2, 5, 10, 20],
  apiParallel: [1, 2, 3, 4],
  retries: [0, 1, 2, 3, 5, 8],
  thumbnailMib: [50, 100, 250, 500, 1000],
  logDays: [7, 14, 30, 90, 365],
  logMib: [25, 50, 100, 250, 500],
} as const;

export const SECTIONS = [
  'general',
  'game',
  'catalog',
  'library',
  'downloads',
  'network',
  'cache',
  'logs',
  'advanced',
  'about',
] as const;
export type Section = (typeof SECTIONS)[number];

export function asData(settings: Settings): SettingsData {
  return settings as unknown as SettingsData;
}

/** A changed copy: `change` edits a deep copy, the original stays as it is. */
export function changed(settings: Settings, change: (s: SettingsData) => void): Settings {
  // A JSON copy: the settings are JSON, and `app.settings` is a reactive proxy that structuredClone refuses.
  const copy = JSON.parse(JSON.stringify(settings)) as Settings;
  change(asData(copy));
  return copy;
}

/** The list a select shows: the choices, plus the current value when it is not one of them (sorted in). */
export function withCurrent<T extends number | null>(choices: readonly T[], current: T): T[] {
  if (choices.includes(current)) {
    return [...choices];
  }
  const out = [...choices, current];
  return out.sort((a, b) => (a ?? -1) - (b ?? -1));
}

/** KiB/s as stored ↔ MB/s as chosen (1 MB = 1024 KiB here, like the limit itself). */
export function speedMb(kib: number | null): number | null {
  return kib === null ? null : Math.round((kib / 1024) * 100) / 100;
}

export function speedKib(mb: number | null): number | null {
  return mb === null ? null : Math.round(mb * 1024);
}

export type NumberCheck = { ok: true; value: number } | { ok: false };

/** A whole number within `range`, as typed in a field. */
export function wholeNumber(text: string, range: { min: number; max: number }): NumberCheck {
  const trimmed = text.trim();
  if (!/^\d+$/.test(trimmed)) {
    return { ok: false };
  }
  const value = Number(trimmed);
  return value >= range.min && value <= range.max ? { ok: true, value } : { ok: false };
}

/** `http://host:port` from the manual proxy fields; the core checks the host. */
export function proxyAddress(host: string, port: number): string {
  const bare = host.trim().replace(/^https?:\/\//i, '').replace(/\/+$/, '');
  return `http://${bare}:${port}`;
}

/** The host and port of a stored manual proxy. */
export function proxyParts(address: string): { host: string; port: string } {
  const m = /^http:\/\/(.+):(\d+)\/?$/.exec(address);
  return m ? { host: m[1] ?? '', port: m[2] ?? '' } : { host: '', port: '8080' };
}

export type HostProblem = 'empty' | 'not_a_host_only' | 'invalid' | 'built_in' | 'listed';

/** The built-in GameBanana hosts (53). `filecacheNN` stands for every numbered cache host. */
export const BUILT_IN_HOSTS = ['gamebanana.com', 'images.gamebanana.com', 'files.gamebanana.com', 'filecacheNN.gamebanana.com'];

/** Checks a domain typed into Advanced → Additional allowed domains (what `normalise_host` in the core accepts). */
export function checkHost(text: string, listed: readonly string[]): { ok: true; host: string } | { ok: false; problem: HostProblem } {
  const host = text.trim().toLowerCase();
  if (host === '') {
    return { ok: false, problem: 'empty' };
  }
  if (/[:/\s]/.test(host)) {
    return { ok: false, problem: 'not_a_host_only' };
  }
  const label = /^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$/;
  if (host.length > 253 || !host.split('.').every((l) => label.test(l)) || !host.includes('.')) {
    return { ok: false, problem: 'invalid' };
  }
  if (BUILT_IN_HOSTS.includes(host) || /^filecache\d+\.gamebanana\.com$/.test(host)) {
    return { ok: false, problem: 'built_in' };
  }
  if (listed.includes(host)) {
    return { ok: false, problem: 'listed' };
  }
  return { ok: true, host };
}

/** The lines of `text` around a JSON error, for the editor's excerpt: the line before and the one with the error. */
export function excerpt(text: string, line: number): { n: number; text: string; error: boolean }[] {
  const lines = text.split('\n');
  const from = Math.max(1, line - 1);
  const out = [];
  for (let n = from; n <= Math.min(line, lines.length); n++) {
    out.push({ n, text: lines[n - 1] ?? '', error: n === line });
  }
  return out;
}

/** A path with `C:\Users\<name>` hidden the way the logs do it (94). */
export function maskUser(path: string): string {
  return path.replace(/^([A-Za-z]:\\Users\\)[^\\]+/, '$1…');
}

/** A copy of `record` without `key`. */
export function without<K extends string, V>(record: Partial<Record<K, V>>, key: K): Partial<Record<K, V>> {
  return Object.fromEntries(Object.entries(record).filter(([k]) => k !== key)) as Partial<Record<K, V>>;
}
