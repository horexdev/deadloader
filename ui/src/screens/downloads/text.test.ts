import { describe, expect, it } from 'vitest';
import type { TaskSnapshot, TaskState } from '../../lib/api';
import { agoText, historyWord, leftText, metaOf, stageWord } from './text';

const MB = 1_000_000;

/** Sizes keep their unit on the line with a no-break space. */
const plain = (text: string) => text.replaceAll(String.fromCharCode(0xa0), ' ');

function task(state: TaskState, extra: Partial<TaskSnapshot> = {}): TaskSnapshot {
  return {
    id: 1,
    url: 'https://gamebanana.com/dl/1',
    file_name: 'seven.7z',
    md5: null,
    size: 214.6 * MB,
    initiator: 'user',
    meta: { mod_name: 'Seven' },
    state,
    bytes: 0,
    total: 214.6 * MB,
    speed: 0,
    ...extra,
  };
}

describe('the words of a download', () => {
  it('names each stage as the design does', () => {
    expect(stageWord('downloading')).toBe('Downloading');
    expect(stageWord('extracting')).toBe('Extracting');
    expect(stageWord('choice')).toBe('Waiting for your choice');
    expect(stageWord('offline')).toBe('Paused — offline');
    expect(stageWord('disk')).toBe('Paused — disk full');
  });

  it('says how far a download is and how long it still takes', () => {
    const t = task({ state: 'downloading' }, { bytes: 66.4 * MB, speed: 3.1 * MB });
    const meta = metaOf({ kind: 'task', task: t, stage: 'downloading' }, null, 4);
    expect(plain(meta.left)).toBe('63.3 MB of 204 MB · 30 %');
    expect(plain(meta.right)).toBe('2.95 MB/s · 48 s left');
    expect(meta.value).toBeCloseTo(0.309, 3);
  });

  it('writes longer waits in minutes and hours', () => {
    expect(leftText(65)).toBe('1 min 5 s left');
    expect(leftText(3725)).toBe('1 h 2 min left');
  });

  it('explains queued, paused and waiting downloads', () => {
    const queued = { kind: 'task' as const, task: task({ state: 'queued' }), stage: 'queued' as const };
    expect(metaOf(queued, 0, 4).right).toBe('Next — starts when one of the 4 download slots is free');
    expect(metaOf(queued, 2, 4).right).toBe('2 ahead');
    expect(metaOf({ ...queued, stage: 'queued' }, 0, 4, true).right).toBe('Starts when you’re back online');
    const paused = { kind: 'task' as const, task: task({ state: 'paused' }, { bytes: 10 * MB }), stage: 'paused' as const };
    expect(metaOf(paused, null, 4).right).toBe('Continues from where it stopped');
    const held = { kind: 'task' as const, task: task({ state: 'paused' }), stage: 'paused' as const };
    expect(metaOf(held, null, 4).right).toBe('Held — won’t start until you resume it');
    const disk = {
      kind: 'task' as const,
      task: task({ state: 'waiting', message: { id: 'E12', args: { drive: 'D:' } }, retry_in_s: null }),
      stage: 'disk' as const,
    };
    expect(metaOf(disk, null, 4).right).toBe('Waits for free space on D:');
    const retrying = {
      kind: 'task' as const,
      task: task({ state: 'waiting', message: { id: 'E5', args: {} }, retry_in_s: 30 }),
      stage: 'retrying' as const,
    };
    expect(metaOf(retrying, null, 4).right).toBe('Tries again in 30 s');
  });

  it('counts the variants of a choice', () => {
    const meta = metaOf(
      { kind: 'choice', choice: { key: 'c', modId: 'm', choices: [['a', 'b', 'c', 'd']], name: 'Pocket', file: 'p.zip', size: 46.2 * MB }, stage: 'choice' },
      null,
      4,
    );
    expect(plain(meta.left)).toBe('44.0 MB · 4 variants inside');
    expect(meta.track).toBe(false);
  });
});

describe('the session history', () => {
  it('says what became of each download', () => {
    const base = { key: 'k', name: 'Seven', file: 'seven.zip', at: 0 };
    expect(historyWord({ ...base, outcome: 'installed' })).toBe('Installed and enabled');
    expect(historyWord({ ...base, outcome: 'installed', variant: 'Casual' })).toBe('Installed · Casual');
    expect(historyWord({ ...base, outcome: 'updated', version: 'v2' })).toBe('Updated to v2');
    expect(historyWord({ ...base, outcome: 'updated' })).toBe('Updated');
    expect(historyWord({ ...base, outcome: 'removed' })).toBe('Removed — archive deleted');
  });

  it('says how long ago', () => {
    const now = 1_792_000_000_000;
    expect(agoText(now - 20_000, now)).toBe('just now');
    expect(agoText(now - 6 * 60_000, now)).toBe('6 min ago');
    expect(agoText(now - 3 * 3_600_000, now)).toBe('3 h ago');
  });
});
