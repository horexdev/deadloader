import { describe, expect, it } from 'vitest';
import type { TaskSnapshot, TaskState } from '../../lib/api';
import type { PendingChoice } from '../../lib/app.svelte';
import {
  actionsOf,
  diskProblem,
  modHref,
  queuePosition,
  rowsOf,
  secondsLeft,
  splitDuration,
  stageOf,
  totals,
} from './tasks';

function task(id: number, state: TaskState, extra: Partial<TaskSnapshot> = {}): TaskSnapshot {
  return {
    id,
    url: `https://gamebanana.com/dl/${id}`,
    file_name: `file_${id}.zip`,
    md5: null,
    size: 1000,
    initiator: 'user',
    meta: { section: 'mod', mod_id: '655209', file_id: String(id), mod_name: `Mod ${id}` },
    state,
    bytes: 0,
    total: 1000,
    speed: 0,
    ...extra,
  };
}

const offlineMessage = { id: 'E6', args: { reason: 'connect' } };

describe('the stage a download is shown in', () => {
  it('follows the queue state', () => {
    expect(stageOf(task(1, { state: 'downloading' }), false)).toBe('downloading');
    expect(stageOf(task(1, { state: 'verifying' }), false)).toBe('verifying');
    expect(stageOf(task(1, { state: 'installing' }), false)).toBe('extracting');
    expect(stageOf(task(1, { state: 'paused' }), false)).toBe('paused');
    expect(stageOf(task(1, { state: 'queued' }), false)).toBe('queued');
    expect(stageOf(task(1, { state: 'failed', message: { id: 'E9', args: {} } }), false)).toBe('failed');
  });

  it('tells waiting for the network, for disk space and for a retry apart', () => {
    const waiting = (message: { id: string; args: Record<string, string> }) =>
      task(1, { state: 'waiting', message, retry_in_s: 30 });
    expect(stageOf(waiting(offlineMessage), true)).toBe('offline');
    expect(stageOf(waiting(offlineMessage), false)).toBe('retrying');
    expect(stageOf(waiting({ id: 'E12', args: { reason: 'full' } }), false)).toBe('disk');
    expect(stageOf(waiting({ id: 'E12', args: { reason: 'full' } }), true)).toBe('disk');
    expect(stageOf(waiting({ id: 'E5', args: {} }), false)).toBe('retrying');
  });
});

describe('the order of the queue', () => {
  it('puts choices first, then running, failed, paused by the user, and queued; keeping queue order inside', () => {
    const choice: PendingChoice = { key: 'c1', modId: 'gb-1', choices: [['a', 'b']], name: 'Cafe', file: 'cafe.zip', size: 10 };
    const rows = rowsOf(
      [
        task(1, { state: 'queued' }),
        task(2, { state: 'paused' }),
        task(3, { state: 'failed', message: { id: 'E11', args: {} } }),
        task(4, { state: 'downloading' }),
        task(5, { state: 'waiting', message: offlineMessage, retry_in_s: null }),
        task(6, { state: 'queued' }),
      ],
      [choice],
      true,
    );
    expect(rows.map((r) => (r.kind === 'choice' ? r.choice.key : r.task.id))).toEqual(['c1', 4, 5, 3, 2, 1, 6]);
    expect(rows.map((r) => r.stage)).toEqual(['choice', 'downloading', 'offline', 'failed', 'paused', 'queued', 'queued']);
  });

  it('numbers the queued ones', () => {
    const rows = rowsOf([task(1, { state: 'queued' }), task(2, { state: 'downloading' }), task(3, { state: 'queued' })], [], false);
    expect(rows.map((r) => queuePosition(rows, r))).toEqual([null, 0, 1]);
  });
});

describe('totals and time left', () => {
  it('counts active and queued downloads and adds up the speed of those downloading', () => {
    expect(
      totals([
        task(1, { state: 'downloading' }, { speed: 3_000_000 }),
        task(2, { state: 'downloading' }, { speed: 1_000_000 }),
        task(3, { state: 'verifying' }, { speed: 9 }),
        task(4, { state: 'installing' }),
        task(5, { state: 'queued' }),
        task(6, { state: 'paused' }),
      ]),
    ).toEqual({ active: 4, queued: 1, speed: 4_000_000 });
  });

  it('knows the time left only while bytes arrive', () => {
    expect(secondsLeft(task(1, { state: 'downloading' }, { bytes: 400, total: 1000, speed: 100 }))).toBe(6);
    expect(secondsLeft(task(1, { state: 'downloading' }, { bytes: 400, total: 1000, speed: 0 }))).toBeNull();
    expect(secondsLeft(task(1, { state: 'downloading' }, { bytes: 400, total: null, speed: 10 }))).toBeNull();
    expect(secondsLeft(task(1, { state: 'paused' }, { bytes: 400, total: 1000, speed: 100 }))).toBeNull();
  });

  it('splits a duration into hours, minutes and seconds, never below one second', () => {
    expect(splitDuration(0.2)).toEqual({ hours: 0, minutes: 0, seconds: 1 });
    expect(splitDuration(65)).toEqual({ hours: 0, minutes: 1, seconds: 5 });
    expect(splitDuration(3725)).toEqual({ hours: 1, minutes: 2, seconds: 5 });
  });
});

describe('what a row offers', () => {
  it('pauses running and queued downloads, resumes paused ones, removes failed ones', () => {
    expect(actionsOf('downloading')).toEqual({ pause: true, resume: false, cancel: true, remove: false, choose: false });
    expect(actionsOf('queued')).toEqual({ pause: true, resume: false, cancel: true, remove: false, choose: false });
    expect(actionsOf('paused')).toEqual({ pause: false, resume: true, cancel: true, remove: false, choose: false });
    expect(actionsOf('failed')).toEqual({ pause: false, resume: false, cancel: false, remove: true, choose: false });
    expect(actionsOf('choice')).toEqual({ pause: false, resume: false, cancel: true, remove: false, choose: true });
  });

  it('cannot stop a download while it is checked or unpacked', () => {
    expect(actionsOf('verifying').cancel).toBe(false);
    expect(actionsOf('extracting').cancel).toBe(false);
  });

  it('pauses downloads that wait for the network or a retry, and lets them be cancelled', () => {
    expect(actionsOf('offline')).toEqual({ pause: true, resume: false, cancel: true, remove: false, choose: false });
    expect(actionsOf('retrying').pause).toBe(true);
    expect(actionsOf('disk').cancel).toBe(true);
  });
});

describe('links and problems', () => {
  it('links a download to its mod page in the catalog', () => {
    expect(modHref(task(1, { state: 'queued' }))).toBe('#/catalog/mod/655209');
    expect(modHref(task(1, { state: 'queued' }, { meta: { section: 'sound', mod_id: '12' } }))).toBe('#/catalog/sound/12');
    expect(modHref(task(1, { state: 'queued' }, { meta: {} }))).toBeNull();
  });

  it('finds the first download stopped by a full disk', () => {
    const full = { id: 'E12', args: { reason: 'space', needed: '247463936', available: '100663296' } };
    expect(diskProblem([task(1, { state: 'downloading' })])).toBeNull();
    expect(
      diskProblem([
        task(1, { state: 'failed', message: { id: 'E9', args: {} } }),
        task(2, { state: 'waiting', message: full, retry_in_s: null }),
      ]),
    ).toEqual(full);
  });
});
