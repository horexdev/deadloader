import { describe, expect, it } from 'vitest';
import type { FileLine } from '../../lib/api';
import { changeText } from './labels';

function line(over: Partial<FileLine>): FileLine {
  return { ts: '2026-10-15T09:00:00.000+02:00', id: 'f-1', lvl: 'info', msg: 'file operation', op: 'rename', result: 'ok', ...over };
}

describe('the change a file operation was part of (186)', () => {
  it('names each kind of change in words', () => {
    expect(changeText(line({ txn: 1, change: { kind: 'apply', profile: 'Evening' } }))).toBe('Profile “Evening” applied');
    expect(changeText(line({ txn: 1, change: { kind: 'rollback' } }))).toBe('Rollback');
    expect(changeText(line({ txn: 1, change: { kind: 'profile_removed', profile: 'Ranked' } }))).toBe(
      'Profile “Ranked” deleted, its mods left the game',
    );
    expect(changeText(line({ txn: 1, change: { kind: 'take_foreign', file: 'pak03_dir.vpk' } }))).toBe(
      'pak03_dir.vpk taken under management',
    );
    expect(changeText(line({ txn: 1, change: { kind: 'gameinfo_fixed' } }))).toBe('Our lines added to gameinfo.gi');
    expect(changeText(line({ txn: 1, change: { kind: 'recovered' } }))).toBe('Interrupted change finished at start');
  });

  it('falls back to the change number for lines written before changes were described, and to nothing', () => {
    expect(changeText(line({ txn: 1792011000000000 }))).toBe('1792011000000000');
    expect(changeText(line({}))).toBeNull();
  });
});
