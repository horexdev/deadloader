// What the catalog and the mod page derive from the core's data: the library mod of a GameBanana mod, what each file
// of a mod page offers (136, 144), card bylines, search limits and relative dates. Pure functions, no DOM.

import type { Card, GalleryImage, ImageId, LibraryView, ModView, Section, TaskSnapshot, UpdateReport } from '../../lib/api';
import { language } from '../../i18n';

/** The library mod made from a GameBanana mod (downloaded or linked, one per GameBanana mod, 144). */
export function libraryModOf(library: LibraryView | null, section: Section, id: number): ModView | undefined {
  return library?.mods.find(
    (mod) => mod.source.kind === 'gamebanana' && mod.source.section === section && mod.source.mod_id === id,
  );
}

/** Whether a library file is in use: the active variant of its group, or an enabled add-on. */
export function isActive(mod: ModView, fileId: string): boolean {
  return (
    mod.variant_groups.some((group) => group.active === fileId) ||
    mod.add_ons.some(([id, enabled]) => id === fileId && enabled)
  );
}

/** "v1.2" for the active file's version; null when it has none. */
export function installedVersion(mod: ModView): string | null {
  const active = mod.files.find((file) => isActive(mod, file.id)) ?? mod.files[0];
  const version = active?.version?.trim();
  if (!version) {
    return null;
  }
  return /^v/i.test(version) ? version : `v${version}`;
}

export function updateAvailable(report: UpdateReport | null, modId: string): boolean {
  return report?.checks.some((check) => check.mod_id === modId && check.status.kind === 'update_available') ?? false;
}

export type FileAction =
  | { kind: 'install' }
  | { kind: 'installing'; percent: number | null }
  | { kind: 'installed' }
  | { kind: 'switch'; file: string }
  | { kind: 'update' };

export interface FileQuery {
  /** GameBanana's file id. */
  fileId: number;
  /** GameBanana's mod id. */
  modId: number;
  mod: ModView | undefined;
  updates: UpdateReport | null;
  tasks: TaskSnapshot[];
}

/** Install / Installing / Installed / Switch to / Update for one listed file (S3). */
export function fileAction({ fileId, modId, mod, updates, tasks }: FileQuery): FileAction {
  const task = tasks.find(
    (t) =>
      t.meta['mod_id'] === String(modId) &&
      t.meta['file_id'] === String(fileId) &&
      t.state.state !== 'failed' &&
      t.state.state !== 'done',
  );
  if (task) {
    const percent = task.total ? Math.min(100, Math.floor((task.bytes / task.total) * 100)) : null;
    return { kind: 'installing', percent };
  }
  if (mod) {
    const status = updates?.checks.find((check) => check.mod_id === mod.id)?.status;
    if (status?.kind === 'update_available' && status.replaced.some((r) => r.candidate?.file.id === fileId)) {
      return { kind: 'update' };
    }
    const files = mod.files.filter((file) => file.remote_file_id === fileId);
    if (files.some((file) => isActive(mod, file.id))) {
      return { kind: 'installed' };
    }
    const variant = files.find((file) => mod.variant_groups.some((group) => group.files.includes(file.id)));
    if (variant) {
      return { kind: 'switch', file: variant.id };
    }
    if (files.length > 0) {
      return { kind: 'installed' };
    }
  }
  return { kind: 'install' };
}

/** "author · Skins → Haze" (design: CatalogCard). */
export function byline(card: Pick<Card, 'author' | 'root_category' | 'subcategory'>): string {
  const category = [card.root_category?.name, card.subcategory?.name].filter(Boolean).join(' → ');
  return [card.author, category].filter(Boolean).join(' · ');
}

/** GameBanana's search takes 2–100 bytes of UTF-8 after trimming. */
export const SEARCH_MIN_BYTES = 2;
export const SEARCH_MAX_BYTES = 100;

const encoder = new TextEncoder();

export function searchable(text: string): boolean {
  const bytes = encoder.encode(text.trim()).length;
  return bytes >= SEARCH_MIN_BYTES && bytes <= SEARCH_MAX_BYTES;
}

/** Cuts a typed search to 100 bytes, never inside a character. */
export function clampSearch(text: string): string {
  let out = '';
  let bytes = 0;
  for (const char of text) {
    const size = encoder.encode(char).length;
    if (bytes + size > SEARCH_MAX_BYTES) {
      break;
    }
    out += char;
    bytes += size;
  }
  return out;
}

const UNITS: [Intl.RelativeTimeFormatUnit, number][] = [
  ['year', 365 * 86_400],
  ['month', 30 * 86_400],
  ['week', 7 * 86_400],
  ['day', 86_400],
  ['hour', 3_600],
  ['minute', 60],
];

/** "3 days ago" in the UI language; the exact date goes into a tooltip. */
export function relativeTime(unixSeconds: number, now: Date = new Date()): string {
  const diff = unixSeconds - now.getTime() / 1000;
  const format = new Intl.RelativeTimeFormat(language(), { numeric: 'auto' });
  for (const [unit, seconds] of UNITS) {
    if (Math.abs(diff) >= seconds) {
      return format.format(Math.round(diff / seconds), unit);
    }
  }
  return format.format(0, 'second');
}

/** The exact moment for a tooltip: "11 October 2026, 14:20". */
export function exactTime(unixSeconds: number): string {
  return new Intl.DateTimeFormat(language(), { dateStyle: 'long', timeStyle: 'short' }).format(unixSeconds * 1000);
}

/** The gallery's widest size in CSS pixels (the design's 530 px column). */
export const GALLERY_WIDTH = 530;

/**
 * The gallery picture to show at `width` CSS pixels and `displayScale` (Windows scaling times the zoom): the smallest
 * one at least that many device pixels wide (530, 800, then the original), so it is never stretched.
 */
export function galleryPicture(image: GalleryImage, width: number, displayScale: number): ImageId {
  const scale = Number.isFinite(displayScale) && displayScale > 0 ? displayScale : 1;
  const needed = (width > 0 ? width : GALLERY_WIDTH) * scale;
  if (needed <= 530) {
    return image.preview;
  }
  return needed <= 800 ? image.large : image.full;
}
