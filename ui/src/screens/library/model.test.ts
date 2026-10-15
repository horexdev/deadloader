import { describe, expect, it } from 'vitest';
import type { ModView, UpdateReport } from '../../lib/api';
import {
  badges,
  byFolder,
  conflictRole,
  counts,
  fileLabel,
  heroName,
  matches,
  modVersion,
  moved,
  pairs,
  parts,
  partsKind,
  targetIndex,
  variantName,
  versionText,
  type Context,
} from './model';

function mod(id: string, over: Partial<ModView> = {}): ModView {
  return {
    id,
    name: `Mod ${id}`,
    author: 'nyx',
    source: { kind: 'gamebanana', section: 'mod', mod_id: 1 },
    enabled: true,
    files: [
      {
        id: `${id}-1`,
        archive_name: `${id}_ab12f.zip`,
        inner_path: 'pak01_dir.vpk',
        version: '1',
        len: 10,
        damaged: false,
        newer_in_library: null,
      },
    ],
    variant_groups: [],
    add_ons: [[`${id}-1`, true]],
    bytes: 10,
    pending: false,
    in_game: null,
    ...over,
  };
}

const faun = mod('faun', {
  files: ['pink', 'red'].map((c) => ({
    id: `faun-${c}`,
    archive_name: `faunceleste_${c}_5027e.rar`,
    inner_path: 'pak01_dir.vpk',
    version: 'v1',
    len: 5,
    damaged: false,
    newer_in_library: null,
  })),
  variant_groups: [{ files: ['faun-pink', 'faun-red'], active: 'faun-red' }],
  add_ons: [],
});

const report: UpdateReport = {
  checks: [{ mod_id: 'b', status: { kind: 'update_available', version: '2', date_updated: null, replaced: [], kept: [], others: [], posts: [] } }],
  available: 1,
  stopped: null,
};

const ctx: Context = {
  conflicts: [{ winner_mod: 'a', winner_file: 'a-1', loser_mod: 'b', loser_file: 'b-1', paths: ['models/x/y.vmdl_c'] }],
  updates: report,
};

describe('library model', () => {
  it('counts and filters the tabs and the search', () => {
    const local = mod('l', { source: { kind: 'local', imported_from: null }, enabled: false, author: null });
    const mods = [mod('a'), mod('b'), local];
    expect(counts(mods, ctx)).toEqual({ all: 3, enabled: 2, disabled: 1, updates: 1, conflicts: 2, local: 1 });
    expect(matches(mods[0] as ModView, 'all', 'MOD A', ctx)).toBe(true);
    expect(matches(mods[0] as ModView, 'all', 'nyx', ctx)).toBe(true);
    expect(matches(local, 'all', 'nyx', ctx)).toBe(false);
  });

  it('says who wins a conflict', () => {
    expect(conflictRole('a', ctx.conflicts)).toBe('wins');
    expect(conflictRole('b', ctx.conflicts)).toBe('loses');
    expect(conflictRole('c', ctx.conflicts)).toBeNull();
  });

  it('names files after their archive, or their folder inside a shared one', () => {
    expect(fileLabel(faun, 'faun-red')).toBe('faunceleste_red');
    const shared = mod('m', {
      files: ['pink', 'blue'].map((c) => ({
        id: `m-${c}`,
        archive_name: 'haze.zip',
        inner_path: `${c}/pak01_dir.vpk`,
        version: null,
        len: 1,
        damaged: false,
        newer_in_library: null,
      })),
    });
    expect(fileLabel(shared, 'm-blue')).toBe('blue');
    expect(fileLabel(mod('a'), 'a-1')).toBe('a');
  });

  it('writes versions as v1 and knows variants and add-ons', () => {
    expect(versionText('1')).toBe('v1');
    expect(versionText('V2')).toBe('V2');
    expect(versionText(null)).toBeNull();
    expect(modVersion(faun)).toBe('v1');
    expect(variantName(faun)).toBe('faunceleste_red');
    expect(partsKind(faun)).toBe('variants');
    expect(partsKind(mod('a'))).toBeNull();
    const withAddOn = mod('m', {
      add_ons: [
        ['m-1', true],
        ['m-2', true],
      ],
    });
    expect(parts(withAddOn)).toEqual({ main: ['m-1'], addOns: [['m-2', true]] });
    expect(partsKind(withAddOn)).toBe('addons');
  });

  it('lists badges by importance; conflicts only for enabled mods', () => {
    const b = mod('b', { pending: true, in_game: { copied: true, bytes: 10 } });
    expect(badges(b, ctx)).toEqual(['pending', 'update', 'loses', 'copied']);
    expect(badges(mod('a', { enabled: false }), ctx)).toEqual([]);
    expect(badges(faun, ctx)).toEqual(['variants']);
  });

  it('moves an item within bounds', () => {
    expect(moved(['a', 'b', 'c'], 2, 0)).toEqual(['c', 'a', 'b']);
    expect(moved(['a', 'b', 'c'], 0, 9)).toEqual(['b', 'c', 'a']);
  });

  it('groups conflicts by pair and paths by folder', () => {
    const merged = pairs([
      ...ctx.conflicts,
      { winner_mod: 'a', winner_file: 'a-2', loser_mod: 'b', loser_file: 'b-1', paths: ['models/x/z.vmdl_c'] },
    ]);
    expect(merged).toEqual([{ winner: 'a', loser: 'b', paths: ['models/x/y.vmdl_c', 'models/x/z.vmdl_c'] }]);
    expect(byFolder(['models/x/y.vmdl_c', 'materials/x/a.vmat_c', 'models/x/z.vmdl_c', 'top.txt'])).toEqual([
      { folder: 'models/x/', files: ['y.vmdl_c', 'z.vmdl_c'] },
      { folder: 'materials/x/', files: ['a.vmat_c'] },
      { folder: '', files: ['top.txt'] },
    ]);
  });
});

describe('moving within a filtered list', () => {
  it('lands next to the shown neighbour in the whole order', () => {
    const all = ['a', 'x', 'b', 'y', 'c'];
    // Only a, b, c are shown; c moved up between a and b.
    expect(targetIndex(all, ['a', 'c', 'b'], 'c')).toBe(2);
    // a moved to the bottom of the shown ones.
    expect(targetIndex(all, ['b', 'c', 'a'], 'a')).toBe(4);
    expect(targetIndex(all, ['c', 'a', 'b'], 'c')).toBe(0);
  });
});

describe('foreign files', () => {
  it('write hero folders as names', () => {
    expect(heroName('haze')).toBe('Haze');
    expect(heroName('lady_geist')).toBe('Lady Geist');
  });
});
