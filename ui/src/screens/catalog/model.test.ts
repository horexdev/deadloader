import { describe, expect, it } from 'vitest';
import type { LibraryView, ModView, TaskSnapshot, UpdateReport } from '../../lib/api';
import { card, libraryMod } from '../../mock/fixtures';
import {
  byline,
  clampSearch,
  fileAction,
  galleryPicture,
  installedVersion,
  libraryModOf,
  relativeTime,
  searchable,
  updateAvailable,
} from './model';

function library(...mods: ModView[]): LibraryView {
  return { selected_profile: 'Default', applied_profile: 'Default', pending: false, pending_count: 0, mods, conflicts: [] };
}

describe('the catalog model', () => {
  it('finds the library mod of a GameBanana mod by its source', () => {
    const view = library(libraryMod(0, true), libraryMod(1, true, true));
    expect(libraryModOf(view, 'mod', 700_000)?.id).toBe('gb-700000');
    expect(libraryModOf(view, 'sound', 700_000)).toBeUndefined();
    expect(libraryModOf(view, 'mod', 700_001)).toBeUndefined();
    expect(libraryModOf(null, 'mod', 700_000)).toBeUndefined();
  });

  it('names the installed version of the active file', () => {
    const mod = libraryMod(0, true);
    expect(installedVersion(mod)).toBe('v1.2');
    const unversioned: ModView = { ...mod, files: mod.files.map((f) => ({ ...f, version: null })) };
    expect(installedVersion(unversioned)).toBeNull();
    const prefixed: ModView = { ...mod, files: mod.files.map((f) => ({ ...f, version: 'v3' })) };
    expect(installedVersion(prefixed)).toBe('v3');
  });

  it('says when an update is available for a library mod', () => {
    const report: UpdateReport = {
      checks: [
        {
          mod_id: 'gb-700000',
          status: { kind: 'update_available', version: '2', date_updated: null, replaced: [], kept: [], others: [], posts: [] },
        },
        { mod_id: 'gb-700001', status: { kind: 'up_to_date' } },
      ],
      available: 1,
      stopped: null,
    };
    expect(updateAvailable(report, 'gb-700000')).toBe(true);
    expect(updateAvailable(report, 'gb-700001')).toBe(false);
    expect(updateAvailable(null, 'gb-700000')).toBe(false);
  });

  it('decides what each file of a mod page offers (136, 144)', () => {
    const mod = libraryMod(0, true);
    const [first, second] = mod.files;
    const ids = [first?.remote_file_id ?? 0, second?.remote_file_id ?? 0];
    expect(fileAction({ fileId: ids[0] ?? 0, modId: 700_000, mod, updates: null, tasks: [] })).toEqual({ kind: 'installed' });
    expect(fileAction({ fileId: ids[1] ?? 0, modId: 700_000, mod, updates: null, tasks: [] })).toEqual({
      kind: 'switch',
      file: second?.id,
    });
    expect(fileAction({ fileId: 42, modId: 700_000, mod, updates: null, tasks: [] })).toEqual({ kind: 'install' });
    expect(fileAction({ fileId: 42, modId: 700_000, mod: undefined, updates: null, tasks: [] })).toEqual({ kind: 'install' });
  });

  it('shows a running download of the file with its progress', () => {
    const task: TaskSnapshot = {
      id: 3,
      url: 'https://gamebanana.com/dl/42',
      file_name: 'x.zip',
      md5: null,
      size: 1000,
      initiator: 'user',
      meta: { mod_id: '700000', file_id: '42', section: 'mod' },
      state: { state: 'downloading' },
      bytes: 620,
      total: 1000,
      speed: 1,
    };
    expect(fileAction({ fileId: 42, modId: 700_000, mod: undefined, updates: null, tasks: [task] })).toEqual({
      kind: 'installing',
      percent: 62,
    });
    const failed = { ...task, state: { state: 'failed' as const, message: { id: 'E6', args: {} } } };
    expect(fileAction({ fileId: 42, modId: 700_000, mod: undefined, updates: null, tasks: [failed] })).toEqual({
      kind: 'install',
    });
  });

  it('offers Update for the listed file that replaces an installed one', () => {
    const mod = libraryMod(0, true);
    const report: UpdateReport = {
      checks: [
        {
          mod_id: mod.id,
          status: {
            kind: 'update_available',
            version: '2',
            date_updated: null,
            replaced: [
              {
                remote_file_id: 1,
                archive_name: 'haze.zip',
                files: [mod.files[0]?.id ?? ''],
                candidate: {
                  file: { id: 99, file_name: 'haze_v2.zip', size_bytes: 1, date_added: null, version: '2', description: null },
                  matched_by: 'same',
                  in_library: [],
                },
              },
            ],
            kept: [],
            others: [],
            posts: [],
          },
        },
      ],
      available: 1,
      stopped: null,
    };
    expect(fileAction({ fileId: 99, modId: 700_000, mod, updates: report, tasks: [] })).toEqual({ kind: 'update' });
  });

  it('writes the byline as author · category → hero', () => {
    expect(byline(card(0))).toBe('lumen · Skins → Haze');
    expect(byline({ ...card(0), author: null, subcategory: null })).toBe('Skins');
  });

  it('keeps a search within GameBanana’s 2–100 bytes', () => {
    expect(searchable(' a ')).toBe(false);
    expect(searchable('ab')).toBe(true);
    expect(searchable('м')).toBe(true);
    expect(clampSearch('a'.repeat(120))).toHaveLength(100);
    // 60 Cyrillic letters are 120 bytes: cut at 50 letters, never inside a letter.
    expect(clampSearch('м'.repeat(60))).toBe('м'.repeat(50));
    expect(clampSearch('haze')).toBe('haze');
  });

  it('says how long ago something happened', () => {
    const now = new Date('2026-10-15T12:00:00Z');
    const seconds = (iso: string) => Date.parse(iso) / 1000;
    expect(relativeTime(seconds('2026-10-12T12:00:00Z'), now)).toBe('3 days ago');
    expect(relativeTime(seconds('2026-02-15T12:00:00Z'), now)).toBe('8 months ago');
    expect(relativeTime(seconds('2026-10-15T11:59:30Z'), now)).toBe('now');
  });

  it('shows the gallery from a picture at least as wide as the device pixels it fills', () => {
    const image = { preview: 'p', large: 'l', full: 'f', original: 'o', caption: null };
    // 530 CSS px at 100 %: the preview; at 125–150 %: 800 px; at 175 % and above: the original.
    expect(galleryPicture(image, 530, 1)).toBe('p');
    expect(galleryPicture(image, 530, 1.25)).toBe('l');
    expect(galleryPicture(image, 530, 1.5)).toBe('l');
    expect(galleryPicture(image, 530, 1.75)).toBe('f');
    expect(galleryPicture(image, 530, 2)).toBe('f');
    // A narrower gallery (small window) needs fewer pixels.
    expect(galleryPicture(image, 400, 2)).toBe('l');
    expect(galleryPicture(image, 350, 1.5)).toBe('p');
    // Not measured yet: the gallery's full width; a nonsense scale counts as 1.
    expect(galleryPicture(image, 0, 1.5)).toBe('l');
    expect(galleryPicture(image, 530, 0)).toBe('p');
    expect(galleryPicture(image, 530, Number.NaN)).toBe('p');
  });
});
