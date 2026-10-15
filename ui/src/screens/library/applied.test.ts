import { describe, expect, it } from 'vitest';
import type { Change, ModView, PendingChange } from '../../lib/api';
import { appliedSummary, changePhrase } from './applied';

function mod(id: string, name: string, files: { id: string; archive: string; version?: string }[] = []): ModView {
  return {
    id,
    name,
    author: null,
    source: { kind: 'gamebanana', section: 'mod', mod_id: 1 },
    enabled: true,
    files: files.map((f) => ({
      id: f.id,
      remote_file_id: null,
      archive_name: f.archive,
      inner_path: 'pak01_dir.vpk',
      version: f.version ?? null,
      len: 1,
      damaged: false,
      newer_in_library: null,
    })),
    variant_groups: [],
    add_ons: [],
    bytes: 1,
    pending: false,
    in_game: null,
  };
}

const MODS = [
  mod('wraith', 'Formal Wraith'),
  mod('gordon', 'Gordon Freeman'),
  mod('remodel', 'Yamato remodel'),
  mod('redesign', 'Yamato redesign'),
  mod('faun', 'Faun Celeste', [
    { id: 'faun-b', archive: 'faunceleste_bubblegum_f5f81.rar' },
    { id: 'faun-c', archive: 'Crimson_5027e.rar' },
  ]),
  mod('haze', 'Haze', [
    { id: 'haze-1', archive: 'haze.zip', version: '1' },
    { id: 'haze-2', archive: 'haze_v2.zip', version: '2' },
  ]),
];

function change(mod_id: string, c: Change): PendingChange {
  return { mod_id, name: MODS.find((m) => m.id === mod_id)?.name ?? mod_id, ...c };
}

describe('the changes applied after the game closed (N12)', () => {
  it('names each change in a few words, as the board does', () => {
    expect(changePhrase(change('wraith', { kind: 'enable' }), MODS)).toBe('Formal Wraith on');
    expect(changePhrase(change('gordon', { kind: 'disable' }), MODS)).toBe('Gordon Freeman off');
    expect(changePhrase(change('remodel', { kind: 'order', above: 'redesign', below: null }), MODS)).toBe(
      'Yamato remodel moved above Yamato redesign',
    );
    expect(changePhrase(change('remodel', { kind: 'order', above: null, below: 'redesign' }), MODS)).toBe(
      'Yamato remodel moved below Yamato redesign',
    );
    expect(changePhrase(change('faun', { kind: 'variant', from: ['faun-b'], to: ['faun-c'] }), MODS)).toBe(
      'Faun Celeste switched to Crimson',
    );
    expect(changePhrase(change('haze', { kind: 'version', from: 'haze-1', to: 'haze-2' }), MODS)).toBe(
      'Haze switched to v2',
    );
  });

  it('says only that the files changed when an add-on went out and nothing came in', () => {
    expect(changePhrase(change('faun', { kind: 'variant', from: ['faun-b'], to: [] }), MODS)).toBe(
      'Faun Celeste: files changed',
    );
  });

  it('uses the name the core sent for a mod no longer in the library', () => {
    const gone: PendingChange = { mod_id: 'gone', name: 'Old skin', kind: 'disable' };
    expect(changePhrase(gone, MODS)).toBe('Old skin off');
  });

  it('lists a few changes and counts the rest', () => {
    const items = [
      change('wraith', { kind: 'enable' }),
      change('gordon', { kind: 'disable' }),
      change('remodel', { kind: 'order', above: 'redesign', below: null }),
      change('faun', { kind: 'variant', from: ['faun-b'], to: ['faun-c'] }),
    ];
    expect(appliedSummary(items, MODS)).toBe(
      'Formal Wraith on, Gordon Freeman off, Yamato remodel moved above Yamato redesign, and Faun Celeste switched to Crimson',
    );
    const many = [...items, ...['a', 'b', 'c'].map((id) => change(id, { kind: 'enable' }))];
    expect(appliedSummary(many, MODS)).toMatch(/, and 2 more$/);
    expect(appliedSummary([], MODS)).toBe('');
  });
});
