// What the Downloads screen (S6) shows about each task: its stage, its place in the queue, the totals and what can be
// done with it. Pure functions over the queue the app state keeps current.

import type { TaskSnapshot, UserMessage } from '../../lib/api';
import type { PendingChoice } from '../../lib/app.svelte';

/**
 * How a task is shown. `offline`, `disk` and `retrying` are the queue's `waiting` state (a temporary problem): the
 * network is gone, the disk is full, or the server asked for patience. `choice` is a finished download waiting for
 * the user to pick a variant (D4).
 */
export type Stage =
  | 'choice'
  | 'downloading'
  | 'verifying'
  | 'extracting'
  | 'offline'
  | 'disk'
  | 'retrying'
  | 'failed'
  | 'paused'
  | 'queued'
  | 'done';

export type Row =
  | { kind: 'task'; task: TaskSnapshot; stage: Stage }
  | { kind: 'choice'; choice: PendingChoice; stage: 'choice' };

export function stageOf(task: TaskSnapshot, offline: boolean): Stage {
  const state = task.state;
  switch (state.state) {
    case 'downloading':
      return 'downloading';
    case 'verifying':
      return 'verifying';
    case 'installing':
      return 'extracting';
    case 'paused':
      return 'paused';
    case 'queued':
      return 'queued';
    case 'failed':
      return 'failed';
    case 'done':
      return 'done';
    case 'waiting':
      if (state.message.id === 'E12') {
        return 'disk';
      }
      return offline ? 'offline' : 'retrying';
  }
}

/** The design's order: what needs you first, then what runs (or waits for the system), errors, your pauses, the queue. */
const RANK: Record<Stage, number> = {
  choice: 0,
  downloading: 1,
  verifying: 1,
  extracting: 1,
  offline: 1,
  disk: 1,
  retrying: 1,
  failed: 2,
  paused: 3,
  queued: 4,
  done: 5,
};

/** The queue as shown: choices and tasks in the design's order, queue order kept within a rank. */
export function rowsOf(tasks: readonly TaskSnapshot[], choices: readonly PendingChoice[], offline: boolean): Row[] {
  const rows: Row[] = [
    ...choices.map((choice) => ({ kind: 'choice' as const, choice, stage: 'choice' as const })),
    ...tasks.map((task) => ({ kind: 'task' as const, task, stage: stageOf(task, offline) })),
  ];
  return rows
    .map((row, index) => ({ row, index }))
    .sort((a, b) => RANK[a.row.stage] - RANK[b.row.stage] || a.index - b.index)
    .map(({ row }) => row);
}

/** 0 for the next queued task, then how many are ahead; `null` for a task that isn't queued. */
export function queuePosition(rows: readonly Row[], row: Row): number | null {
  if (row.stage !== 'queued') {
    return null;
  }
  return rows.filter((r) => r.stage === 'queued').indexOf(row);
}

export interface Totals {
  active: number;
  queued: number;
  /** Bytes per second of everything downloading. */
  speed: number;
}

export function totals(tasks: readonly TaskSnapshot[]): Totals {
  const active = tasks.filter((t) => ['downloading', 'verifying', 'installing'].includes(t.state.state)).length;
  const queued = tasks.filter((t) => t.state.state === 'queued').length;
  const speed = tasks.filter((t) => t.state.state === 'downloading').reduce((sum, t) => sum + t.speed, 0);
  return { active, queued, speed };
}

/** Seconds until a download is done at its current speed; `null` when that isn't known. */
export function secondsLeft(task: TaskSnapshot): number | null {
  if (task.state.state !== 'downloading' || task.total === null || task.speed <= 0) {
    return null;
  }
  return Math.max(0, task.total - task.bytes) / task.speed;
}

export function splitDuration(seconds: number): { hours: number; minutes: number; seconds: number } {
  const whole = Math.max(1, Math.round(seconds));
  return { hours: Math.floor(whole / 3600), minutes: Math.floor((whole % 3600) / 60), seconds: whole % 60 };
}

export interface RowActions {
  pause: boolean;
  resume: boolean;
  /** Cancel a download (or put aside a choice). */
  cancel: boolean;
  /** Remove a failed task and its files. */
  remove: boolean;
  /** Open D4. */
  choose: boolean;
}

export function actionsOf(stage: Stage): RowActions {
  const none: RowActions = { pause: false, resume: false, cancel: false, remove: false, choose: false };
  switch (stage) {
    case 'downloading':
    case 'queued':
    case 'offline':
    case 'disk':
    case 'retrying':
      return { ...none, pause: true, cancel: true };
    case 'paused':
      return { ...none, resume: true, cancel: true };
    case 'failed':
      return { ...none, remove: true };
    case 'choice':
      return { ...none, cancel: true, choose: true };
    // A check or an install stops where it is safe, and its files go; it can't be paused there.
    case 'verifying':
    case 'extracting':
      return { ...none, cancel: true };
    case 'done':
      return none;
  }
}

/** The task's mod page in the catalog (the catalog screen owns `#/catalog/<section>/<id>`). */
export function modHref(task: TaskSnapshot): string | null {
  const section = task.meta.section;
  const id = task.meta.mod_id;
  return section && id ? `#/catalog/${section}/${id}` : null;
}

/** The message of the first task stopped by a full disk (E12), for the banner. */
export function diskProblem(tasks: readonly TaskSnapshot[]): UserMessage | null {
  for (const task of tasks) {
    const state = task.state;
    if ((state.state === 'waiting' || state.state === 'failed') && state.message.id === 'E12') {
      return state.message;
    }
  }
  return null;
}
