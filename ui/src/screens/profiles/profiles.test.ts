import { describe, expect, it } from 'vitest';
import type { Missing, ProfileOverview, ProfilesView } from '../../lib/api';
import {
  capitalize,
  copyName,
  downloadsFor,
  driveOf,
  folderHolder,
  inGame,
  nameProblem,
  nextSelected,
  rootOf,
  since,
} from './profiles';

const D = String.raw`D:\SteamLibrary\steamapps\common\Deadlock`;
const E = String.raw`E:\SteamLibrary\steamapps\common\Deadlock`;

function profile(name: string, extra: Partial<ProfileOverview> = {}): ProfileOverview {
  return { name, game_root: D, mods: 0, mod_names: [], applied: false, last_applied_unix_ms: null, ...extra };
}

const view: ProfilesView = {
  selected: 'Weekend',
  installations: [D, E],
  profiles: [
    profile('Evening', { applied: true, mods: 9 }),
    profile('Weekend'),
    profile('Sasha', { game_root: E, applied: true }),
    profile('Fresh', { game_root: null }),
  ],
};

describe('profile names', () => {
  it('must not be empty or taken, compared without case', () => {
    const names = ['Evening', 'Weekend'];
    expect(nameProblem('  ', names)).toBe('empty');
    expect(nameProblem('weekend', names)).toBe('taken');
    expect(nameProblem('Weekend', names, 'Weekend')).toBeNull();
    expect(nameProblem('Practice', names)).toBeNull();
  });

  it('finds a free name for a copy', () => {
    const word = (name: string, n: number) => (n === 1 ? `${name} (copy)` : `${name} (copy ${n})`);
    expect(copyName('Evening', ['Evening'], word)).toBe('Evening (copy)');
    expect(copyName('Evening', ['Evening', 'evening (COPY)'], word)).toBe('Evening (copy 2)');
  });
});

describe('installations', () => {
  it('names the drive of a game folder', () => {
    expect(driveOf(D)).toBe('D:');
    expect(driveOf('/games/deadlock')).toBe('');
  });

  it('uses the first installation for a profile without its own', () => {
    expect(rootOf(profile('Fresh', { game_root: null }), [D, E])).toBe(D);
    expect(rootOf(profile('Sasha', { game_root: E }), [D, E])).toBe(E);
    expect(rootOf(profile('Fresh', { game_root: null }), [])).toBeNull();
  });

  it('knows which profile is in the selected profile’s game folder', () => {
    expect(folderHolder(view)?.name).toBe('Evening');
    expect(folderHolder({ ...view, selected: 'Sasha' })).toBeNull();
  });

  it('lists what is in the game on each drive', () => {
    expect(inGame(view)).toEqual([
      { name: 'Evening', drive: 'D:' },
      { name: 'Sasha', drive: 'E:' },
    ]);
  });

  it('picks the next selected profile like the core: Default, else the first other one', () => {
    expect(nextSelected(['Evening', 'Weekend'], 'Evening')).toBe('Weekend');
    expect(nextSelected(['Default', 'Evening'], 'Evening')).toBe('Default');
  });
});

describe('times', () => {
  const now = Date.UTC(2026, 9, 15, 14, 32);
  it('says just now, then hours, yesterday, days and weeks', () => {
    expect(since(now - 20_000, now, 'en')).toBeNull();
    expect(since(now - 2 * 3_600_000, now, 'en')).toBe('2 hours ago');
    expect(since(now - 26 * 3_600_000, now, 'en')).toBe('yesterday');
    expect(since(now - 3 * 86_400_000, now, 'en')).toBe('3 days ago');
    expect(since(now - 21 * 86_400_000, now, 'en')).toBe('3 weeks ago');
    expect(since(now - 3 * 86_400_000, now, 'de')).toBe('vor 3 Tagen');
  });

  it('starts a sentence with a capital', () => {
    expect(capitalize('vor 3 Tagen angewendet')).toBe('Vor 3 Tagen angewendet');
    expect(capitalize('')).toBe('');
  });
});

describe('missing mods (D9)', () => {
  const missing: Missing[] = [
    {
      name: 'Numeric Health',
      source: { kind: 'gamebanana', section: 'mod', mod_id: 12 },
      whole_mod: true,
      files: [{ archive: 'numeric.zip', inner_path: 'a.vpk', remote_file_id: 101, version: 'v2' }],
    },
    {
      name: 'crosshair_dot_small.vpk',
      source: { kind: 'local', imported_from: null },
      whole_mod: true,
      files: [{ archive: 'crosshair.vpk', inner_path: 'crosshair.vpk', remote_file_id: null, version: null }],
    },
    {
      name: 'No ids',
      source: { kind: 'gamebanana', section: 'sound', mod_id: 13 },
      whole_mod: true,
      files: [{ archive: 'x.zip', inner_path: 'x.vpk', remote_file_id: null, version: null }],
    },
  ];

  it('downloads only GameBanana files with a known id', () => {
    expect(downloadsFor(missing)).toEqual([{ section: 'mod', mod_id: 12, name: 'Numeric Health', file_ids: [101] }]);
  });
});
