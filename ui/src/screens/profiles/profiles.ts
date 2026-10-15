// The Profiles screen's (S5) logic: names, game installations (153), times, and what D9 can download.

import type { Missing, MissingDownload, ProfileOverview, ProfilesView } from '../../lib/api';

const same = (a: string, b: string) => a.toLocaleLowerCase() === b.toLocaleLowerCase();

/** Names are compared without case, like file names (E21). `except` is the profile being renamed. */
export function nameProblem(name: string, existing: readonly string[], except?: string): 'empty' | 'taken' | null {
  const trimmed = name.trim();
  if (trimmed === '') {
    return 'empty';
  }
  return existing.some((n) => n !== except && same(n, trimmed)) ? 'taken' : null;
}

/** "Evening (copy)", then "Evening (copy 2)"… — the first one no profile uses. */
export function copyName(base: string, existing: readonly string[], word: (name: string, n: number) => string): string {
  for (let n = 1; ; n += 1) {
    const name = word(base, n);
    if (!existing.some((x) => same(x, name))) {
      return name;
    }
  }
}

export function driveOf(root: string | null | undefined): string {
  return root?.match(/^[A-Za-z]:/)?.[0] ?? '';
}

/** The game folder a profile goes into: its own, else the first installation (where Apply would put it). */
export function rootOf(profile: ProfileOverview, installations: readonly string[]): string | null {
  return profile.game_root ?? installations[0] ?? null;
}

export function selectedOf(view: ProfilesView): ProfileOverview | undefined {
  return view.profiles.find((p) => p.name === view.selected);
}

/** The profile whose files are in the selected profile's game folder, when that isn't the selected one. */
export function folderHolder(view: ProfilesView): ProfileOverview | null {
  const selected = selectedOf(view);
  if (!selected || selected.applied) {
    return null;
  }
  const root = rootOf(selected, view.installations);
  return view.profiles.find((p) => p.applied && p !== selected && rootOf(p, view.installations) === root) ?? null;
}

/** "In the game now": each applied profile and its drive. */
export function inGame(view: ProfilesView): { name: string; drive: string }[] {
  return view.profiles
    .filter((p) => p.applied)
    .map((p) => ({ name: p.name, drive: driveOf(rootOf(p, view.installations)) }));
}

/** Who is selected after `deleted` goes: Default, else the first other profile (as the core does). */
export function nextSelected(names: readonly string[], deleted: string): string | null {
  const rest = names.filter((n) => !same(n, deleted));
  return rest.find((n) => n === 'Default') ?? rest[0] ?? null;
}

const UNITS: [Intl.RelativeTimeFormatUnit, number][] = [
  ['minute', 60_000],
  ['hour', 3_600_000],
  ['day', 86_400_000],
  ['week', 7 * 86_400_000],
  ['month', 30 * 86_400_000],
  ['year', 365 * 86_400_000],
];

/** "2 hours ago", "yesterday", "3 weeks ago" in `lang`; `null` for under a minute ("just now"). */
export function since(moment: number, now: number, lang: string): string | null {
  const elapsed = now - moment;
  if (elapsed < 60_000) {
    return null;
  }
  let unit: Intl.RelativeTimeFormatUnit = 'minute';
  let size = 60_000;
  for (const [u, ms] of UNITS) {
    if (elapsed >= ms) {
      unit = u;
      size = ms;
    }
  }
  if (unit === 'day' && elapsed < 2 * 86_400_000) {
    return new Intl.RelativeTimeFormat(lang, { numeric: 'auto' }).format(-1, 'day');
  }
  return new Intl.RelativeTimeFormat(lang, { numeric: 'auto' }).format(-Math.floor(elapsed / size), unit);
}

/** A sentence may start with a relative time ("vor 3 Tagen angewendet"). */
export function capitalize(text: string): string {
  return text.charAt(0).toLocaleUpperCase() + text.slice(1);
}

/** What "Download missing" can fetch: GameBanana mods whose files have an id. */
export function downloadsFor(missing: readonly Missing[]): MissingDownload[] {
  const out: MissingDownload[] = [];
  for (const m of missing) {
    if (m.source.kind !== 'gamebanana') {
      continue;
    }
    const ids = m.files.map((f) => f.remote_file_id).filter((id): id is number => id !== null);
    if (ids.length > 0) {
      out.push({ section: m.source.section, mod_id: m.source.mod_id, name: m.name, file_ids: ids });
    }
  }
  return out;
}
