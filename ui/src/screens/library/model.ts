// What the Library screen derives from the core's library view: filters and counts, the files a mod uses, the words
// for its row (version, variant, add-ons), its badges, and moving rows. Pure functions; the screen adds the texts.

import type { ConflictView, FileView, ModView, UpdateReport, UpdateStatus } from '../../lib/api';

export const FILTERS = ['all', 'enabled', 'disabled', 'updates', 'conflicts', 'local'] as const;
export type FilterId = (typeof FILTERS)[number];

/** What the screen knows besides the library: the last update check. */
export interface Context {
  conflicts: ConflictView[];
  updates: UpdateReport | null;
}

export function isLocal(mod: ModView): boolean {
  return mod.source.kind === 'local';
}

export function updateOf(id: string, updates: UpdateReport | null): UpdateStatus | null {
  return updates?.checks.find((c) => c.mod_id === id)?.status ?? null;
}

export function hasUpdate(id: string, updates: UpdateReport | null): boolean {
  return updateOf(id, updates)?.kind === 'update_available';
}

/** "wins" when the mod wins any of its conflicts with an enabled mod, "loses" when it only loses. */
export function conflictRole(id: string, conflicts: ConflictView[]): 'wins' | 'loses' | null {
  if (conflicts.some((c) => c.winner_mod === id)) {
    return 'wins';
  }
  return conflicts.some((c) => c.loser_mod === id) ? 'loses' : null;
}

export function matches(mod: ModView, filter: FilterId, query: string, ctx: Context): boolean {
  const text = query.trim().toLowerCase();
  if (text && !`${mod.name} ${mod.author ?? ''}`.toLowerCase().includes(text)) {
    return false;
  }
  switch (filter) {
    case 'all':
      return true;
    case 'enabled':
      return mod.enabled;
    case 'disabled':
      return !mod.enabled;
    case 'updates':
      return hasUpdate(mod.id, ctx.updates);
    case 'conflicts':
      return conflictRole(mod.id, ctx.conflicts) !== null;
    case 'local':
      return isLocal(mod);
  }
}

export function counts(mods: ModView[], ctx: Context): Record<FilterId, number> {
  const out = Object.fromEntries(FILTERS.map((f) => [f, 0])) as Record<FilterId, number>;
  for (const mod of mods) {
    for (const filter of FILTERS) {
      if (matches(mod, filter, '', ctx)) {
        out[filter] += 1;
      }
    }
  }
  return out;
}

/** The file a mod shows as its own: the active variant of each group, else the first file of the group. */
function chosen(mod: ModView): string[] {
  return mod.variant_groups.map((g) => g.active ?? g.files[0] ?? '').filter(Boolean);
}

/**
 * The main files and the add-ons: with variants, the chosen variants are the main files and every standalone file is
 * an add-on; without, the first standalone file is the main one (LibraryRow.md "Main files").
 */
export function parts(mod: ModView): { main: string[]; addOns: [string, boolean][] } {
  const variants = chosen(mod);
  if (variants.length > 0) {
    return { main: variants, addOns: mod.add_ons };
  }
  const [first, ...rest] = mod.add_ons;
  return { main: first ? [first[0]] : [], addOns: rest };
}

export function file(mod: ModView, id: string): FileView | undefined {
  return mod.files.find((f) => f.id === id);
}

/** The groups with more than one file: the mod's variants. */
export function variantGroups(mod: ModView) {
  return mod.variant_groups.filter((g) => g.files.length > 1);
}

/** "variants" or "addons" when the row can be expanded (23, 144), else null. */
export function partsKind(mod: ModView): 'variants' | 'addons' | null {
  if (variantGroups(mod).length > 0) {
    return 'variants';
  }
  return parts(mod).addOns.length > 0 ? 'addons' : null;
}

function stem(name: string): string {
  const base = name.split(/[\\/]/).pop() ?? name;
  return base.replace(/\.[^.]+$/, '');
}

/**
 * A file's name for people: its archive without the extension and GameBanana's `_xxxxx` suffix; when several files
 * of the mod come from the same archive, the folder or name inside it.
 */
export function fileLabel(mod: ModView, id: string): string {
  const f = file(mod, id);
  if (!f) {
    return id;
  }
  const shared = mod.files.filter((o) => o.archive_name === f.archive_name).length > 1;
  if (shared) {
    const inner = f.inner_path.split('/').filter(Boolean);
    const folder = inner.length > 1 ? inner[inner.length - 2] : undefined;
    return folder ?? stem(f.inner_path);
  }
  return stem(f.archive_name).replace(/_[0-9a-f]{5}$/i, '');
}

/** "v1" from the version GameBanana lists ("1", "v2", "2.0"), or null. */
export function versionText(version: string | null | undefined): string | null {
  const v = version?.trim();
  if (!v) {
    return null;
  }
  return /^v/i.test(v) ? v : `v${v}`;
}

/** The version of the mod's main file. */
export function modVersion(mod: ModView): string | null {
  const main = parts(mod).main[0];
  return versionText(main ? file(mod, main)?.version : null);
}

/** The active variant's name, when the mod has variants. */
export function variantName(mod: ModView): string | null {
  const group = variantGroups(mod)[0];
  const active = group?.active ?? group?.files[0];
  return active ? fileLabel(mod, active) : null;
}

/** Files missing from the library or no longer readable (E14). */
export function damaged(mod: ModView): boolean {
  return mod.files.some((f) => f.damaged);
}

/** The order of `ids` with the item at `from` moved to `to`. */
export function moved<T>(ids: readonly T[], from: number, to: number): T[] {
  const out = [...ids];
  const [item] = out.splice(from, 1);
  if (item !== undefined) {
    out.splice(Math.max(0, Math.min(to, out.length)), 0, item);
  }
  return out;
}

export type BadgeKind = 'pending' | 'damaged' | 'update' | 'wins' | 'loses' | 'variants' | 'local' | 'copied';

/** The row's badges, most important first; the row shows two (LibraryRow.md). */
export function badges(mod: ModView, ctx: Context): BadgeKind[] {
  const out: BadgeKind[] = [];
  if (mod.pending) {
    out.push('pending');
  }
  if (damaged(mod)) {
    out.push('damaged');
  }
  if (hasUpdate(mod.id, ctx.updates)) {
    out.push('update');
  }
  // Conflicts are between enabled mods (133, 154).
  const role = mod.enabled ? conflictRole(mod.id, ctx.conflicts) : null;
  if (role) {
    out.push(role);
  }
  if (variantGroups(mod).length > 0) {
    out.push('variants');
  }
  if (isLocal(mod)) {
    out.push('local');
  }
  if (mod.in_game?.copied) {
    out.push('copied');
  }
  return out;
}

/** The conflicting pairs of mods, each once with all their paths (C1 groups them by folder). */
export interface Pair {
  winner: string;
  loser: string;
  paths: string[];
}

export function pairs(conflicts: ConflictView[]): Pair[] {
  const out: Pair[] = [];
  for (const c of conflicts) {
    const found = out.find((p) => p.winner === c.winner_mod && p.loser === c.loser_mod);
    if (found) {
      found.paths.push(...c.paths.filter((p) => !found.paths.includes(p)));
    } else {
      out.push({ winner: c.winner_mod, loser: c.loser_mod, paths: [...c.paths] });
    }
  }
  return out;
}

/** Paths grouped by their folder ("models/heroes/yamato/"), the folders in order of first appearance. */
export function byFolder(paths: string[]): { folder: string; files: string[] }[] {
  const out: { folder: string; files: string[] }[] = [];
  for (const path of paths) {
    const cut = path.lastIndexOf('/');
    const folder = cut >= 0 ? path.slice(0, cut + 1) : '';
    const name = path.slice(cut + 1);
    const group = out.find((g) => g.folder === folder);
    if (group) {
      group.files.push(name);
    } else {
      out.push({ folder, files: [name] });
    }
  }
  return out;
}

/**
 * The index in the profile's whole order (`all`) for `id` after it was moved within the shown list (`shown`, a
 * filtered part of `all` in the new order): right above the shown item that now follows it, else right below the
 * one before it.
 */
export function targetIndex(all: readonly string[], shown: readonly string[], id: string): number {
  const rest = all.filter((x) => x !== id);
  const at = shown.indexOf(id);
  const next = shown[at + 1];
  if (next !== undefined && rest.includes(next)) {
    return rest.indexOf(next);
  }
  const before = shown[at - 1];
  return before !== undefined && rest.includes(before) ? rest.indexOf(before) + 1 : 0;
}

/** A hero folder name as people write it ("lady_geist" → "Lady Geist"); the game's internal names stay as they are. */
export function heroName(hero: string): string {
  return hero
    .split('_')
    .filter(Boolean)
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(' ');
}
