import { describe, expect, it } from 'vitest';
import { limitRows } from './limits';

const MiB = 1024 * 1024;
const NBSP = ' ';

/** An archive of 412 MB whose list says 3.8 GB (board DownloadsD12), under the default limits. */
const measured = {
  limit: String(2048 * MiB),
  entry: 'mina/pak01_dir.vpk',
  unpacked: String(3892 * MiB),
  files: '41',
  archive_size: String(412 * MiB),
  size_limit: String(2048 * MiB),
  factor_limit: '20',
  files_limit: '256',
  ratio_limit: '100',
};

describe('the limits table of E9 in D12', () => {
  it('marks the size as over the limit and the rest as within it', () => {
    expect(limitRows({ reason: 'too_large', ...measured })).toEqual([
      { check: 'Unpacked size', value: `3.80${NBSP}GB`, limit: `2.00${NBSP}GB`, result: 'over' },
      { check: 'Compression ratio', value: '9.4×', limit: '20×', result: 'within' },
      { check: 'Mod files', value: '41', limit: '256', result: 'within' },
    ]);
  });

  it('marks the ratio when 20 × the archive’s size was the smaller cap', () => {
    const rows = limitRows({
      ...measured,
      reason: 'too_large',
      archive_size: String(10 * MiB),
      limit: String(200 * MiB),
      unpacked: String(300 * MiB),
    });
    expect(rows[0]).toEqual({ check: 'Unpacked size', value: `300${NBSP}MB`, limit: `2.00${NBSP}GB`, result: 'within' });
    expect(rows[1]).toEqual({ check: 'Compression ratio', value: '30×', limit: '20×', result: 'over' });
  });

  it('names the file compressed too well', () => {
    const rows = limitRows({ ...measured, reason: 'ratio', limit: '100', entry: 'flat.vpk', unpacked: String(90 * MiB) });
    expect(rows[0]?.result).toBe('within');
    expect(rows.at(-1)).toEqual({
      check: 'Compression of one file',
      value: 'flat.vpk: 100× or more',
      limit: '100×',
      result: 'over',
    });
  });

  it('counts the mod files of a pack', () => {
    const rows = limitRows({ ...measured, reason: 'too_many_files', limit: '256', found: '300', files: '300' });
    expect(rows[2]).toEqual({ check: 'Mod files', value: '300', limit: '256', result: 'over' });
  });

  it('says only “more than the limit” when the archive’s list can’t be read or claims less', () => {
    for (const unpacked of [undefined, String(10 * MiB)]) {
      const rows = limitRows({ ...measured, reason: 'too_large', unpacked, files: undefined });
      expect(rows[0]).toEqual({
        check: 'Unpacked size',
        value: `more than 2.00${NBSP}GB`,
        limit: `2.00${NBSP}GB`,
        result: 'over',
      });
      expect(rows[1]).toEqual({ check: 'Compression ratio', value: '—', limit: '20×', result: null });
      expect(rows[2]).toEqual({ check: 'Mod files', value: '—', limit: '256', result: 'within' });
    }
  });

  it('adds the memory a 7z needs when that stopped it', () => {
    const rows = limitRows({ ...measured, reason: 'memory', limit: String(256 * MiB), needed: String(1024 * MiB) });
    expect(rows.at(-1)).toEqual({ check: 'Memory to unpack', value: `1.00${NBSP}GB`, limit: `256${NBSP}MB`, result: 'over' });
  });

  it('has no table for a message from an older core without the limits', () => {
    expect(limitRows({ reason: 'too_large', limit: String(2048 * MiB) })).toEqual([]);
  });
});
