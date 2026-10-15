// The words of the Downloads screen (S6): stage names, progress lines and the session history, in the UI language.

import type { FinishedDownload } from '../../lib/app.svelte';
import { formatNumber, formatSize, t } from '../../i18n';
import { secondsLeft, splitDuration, type Row, type Stage } from './tasks';

const STAGE_KEYS: Record<Stage, string> = {
  choice: 'downloads-stage-choice',
  downloading: 'downloads-stage-downloading',
  verifying: 'downloads-stage-verifying',
  extracting: 'downloads-stage-extracting',
  offline: 'downloads-stage-offline',
  disk: 'downloads-stage-disk',
  retrying: 'downloads-stage-retrying',
  failed: 'downloads-stage-failed',
  paused: 'downloads-stage-paused',
  queued: 'downloads-stage-queued',
  done: 'downloads-done-installed',
};

export function stageWord(stage: Stage): string {
  return t(STAGE_KEYS[stage]);
}

/** "47 s left", "1 min 5 s left", "1 h 2 min left". */
export function leftText(seconds: number): string {
  const d = splitDuration(seconds);
  if (d.hours > 0) {
    return t('downloads-left-hours', { hours: d.hours, minutes: d.minutes });
  }
  if (d.minutes > 0) {
    return t('downloads-left-minutes', { minutes: d.minutes, seconds: d.seconds });
  }
  return t('downloads-left-seconds', { seconds: d.seconds });
}

export interface Meta {
  /** After the file name: amounts and percentage, or the size. */
  left: string;
  /** On the right: speed and time left, or what happens next. */
  right: string;
  /** The bar's fill, 0–1. */
  value: number;
  /** Whether the row has a progress bar. */
  track: boolean;
  /** The bar moves without a known end (verifying, extracting). */
  indeterminate: boolean;
}

function sizeOf(bytes: number | null): string {
  return bytes === null ? '' : formatSize(bytes);
}

/**
 * The two lines under a row's name. `position` is the place among queued tasks (0 = next), `parallel` the download
 * slots from the settings, `offline` whether the network is gone.
 */
export function metaOf(row: Row, position: number | null, parallel: number, offline = false): Meta {
  if (row.kind === 'choice') {
    const count = row.choice.choices.reduce((n, group) => n + group.length, 0);
    const left = [sizeOf(row.choice.size), t('downloads-choice-note', { count })].filter(Boolean).join(' · ');
    return { left, right: '', value: 0, track: false, indeterminate: false };
  }
  const task = row.task;
  const total = task.total ?? task.size;
  const value = total ? Math.min(1, task.bytes / total) : 0;
  const amounts =
    total && task.bytes > 0
      ? t('downloads-progress', {
          done: formatSize(task.bytes),
          total: formatSize(total),
          percent: formatNumber(Math.floor(value * 100)),
        })
      : sizeOf(total);
  const base = { left: amounts, value, track: true, indeterminate: false };
  const state = task.state;
  switch (row.stage) {
    case 'downloading': {
      const seconds = secondsLeft(task);
      const speed = formatSize(task.speed);
      return {
        ...base,
        right: seconds === null ? t('downloads-per-second', { size: speed }) : t('downloads-rate', { speed, left: leftText(seconds) }),
      };
    }
    case 'verifying':
      return { ...base, left: sizeOf(total), right: t('downloads-verifying-note'), indeterminate: true };
    case 'extracting':
      return { ...base, left: sizeOf(total), right: t('downloads-extracting-note'), indeterminate: true };
    case 'offline':
      return { ...base, right: t('downloads-offline-note') };
    case 'disk': {
      const drive = state.state === 'waiting' || state.state === 'failed' ? state.message.args.drive : undefined;
      return { ...base, right: drive ? t('downloads-disk-note', { drive }) : t('downloads-disk-note-plain') };
    }
    case 'retrying': {
      const wait = state.state === 'waiting' ? state.retry_in_s : null;
      return { ...base, right: wait === null ? '' : t('downloads-retrying-note', { seconds: wait }) };
    }
    case 'paused':
      return { ...base, right: task.bytes > 0 ? t('downloads-paused-note') : t('downloads-held-note') };
    case 'queued':
      return {
        ...base,
        left: sizeOf(total),
        right: offline
          ? t('downloads-queued-offline')
          : position === 0
            ? t('downloads-queued-next', { count: parallel })
            : t('downloads-queued-ahead', { count: position ?? 0 }),
      };
    case 'failed': {
      const downloaded = total !== null && task.bytes >= total;
      return {
        ...base,
        left: [sizeOf(total), downloaded ? t('downloads-downloaded') : ''].filter(Boolean).join(' · '),
        right: '',
        track: false,
      };
    }
    case 'choice':
    case 'done':
      return { ...base, right: '' };
  }
}

/** What became of a finished download. */
export function historyWord(entry: Pick<FinishedDownload, 'outcome' | 'variant' | 'version'>): string {
  switch (entry.outcome) {
    case 'installed':
      return entry.variant ? t('downloads-done-variant', { variant: entry.variant }) : t('downloads-done-installed');
    case 'updated':
      return entry.version ? t('downloads-done-updated', { version: entry.version }) : t('downloads-done-updated-plain');
    case 'cancelled':
      return t('downloads-done-cancelled');
    case 'removed':
      return t('downloads-done-removed');
    case 'kept':
      return t('downloads-done-kept');
  }
}

/** "just now", "6 min ago", "3 h ago". */
export function agoText(at: number, now: number): string {
  const minutes = Math.floor((now - at) / 60_000);
  if (minutes < 1) {
    return t('downloads-ago-now');
  }
  if (minutes < 60) {
    return t('downloads-ago-minutes', { count: minutes });
  }
  return t('downloads-ago-hours', { count: Math.floor(minutes / 60) });
}
