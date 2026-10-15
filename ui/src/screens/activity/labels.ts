// The words and numbers of the Activity screen, in the current language.

import type { FileLine, FileOp, HostSource, LogEntry, LogLevel, NetworkLine, Purpose } from '../../lib/api';
import { formatClock, formatInstant, formatNumber, formatSize, t } from '../../i18n';
import { hopStatus, networkStatus } from './model';

export const purposeLabel = (p: Purpose): string => t(`activity-p-${p}`);
export const opLabel = (op: FileOp): string => t(`activity-op-${op}`);
export const levelLabel = (lvl: LogLevel): string => t(`activity-lv-${lvl}`);
export const sourceLabel = (s: HostSource | undefined): string => t(`activity-src-${s ?? 'blocked'}`);
export const sourceLong = (s: HostSource | undefined | null): string => t(`activity-src-long-${s ?? 'blocked'}`);
export const initiatorLabel = (i: 'auto' | 'user'): string => t(`activity-by-${i}`);

/** The change a file operation was part of, in words ("Profile “Evening” applied"); its number on lines written before
 * changes were described (186); null when it was part of none. */
export function changeText(line: FileLine): string | null {
  const change = line.change;
  if (change) {
    const key = `activity-change-${change.kind.replace(/_/g, '-')}`;
    switch (change.kind) {
      case 'apply':
      case 'profile_removed':
        return t(key, { profile: change.profile });
      case 'take_foreign':
        return t(key, { file: change.file });
      default:
        return t(key);
    }
  }
  return line.txn === undefined ? null : String(line.txn);
}

/** "14:32:09". */
export function clock(ts: string): string {
  const date = new Date(ts);
  return Number.isNaN(date.getTime()) ? ts : formatClock(date);
}

/** "Thu, Oct 15, 2026, 14:32:09.601". */
export function instant(ts: string): string {
  const date = new Date(ts);
  return Number.isNaN(date.getTime()) ? ts : formatInstant(date);
}

export function sizeText(bytes: number | null | undefined): string {
  return bytes === null || bytes === undefined ? '—' : formatSize(bytes);
}

/** "92 ms", "3.8 s". */
export function durationText(ms: number): string {
  if (ms >= 1000) {
    return t('activity-seconds', { s: formatNumber(Math.round(ms / 100) / 10) });
  }
  return t('activity-ms', { ms: formatNumber(ms) });
}

/** The status column: the HTTP code, or what happened instead. */
export function statusText(entry: LogEntry): string {
  const status = networkStatus(entry);
  return statusWord(status.kind, status.code);
}

export function hopStatusText(line: NetworkLine, long = false): string {
  const status = hopStatus(line);
  if (status.kind === 'blocked' && long) {
    return t('activity-st-blocked-long');
  }
  return statusWord(status.kind, status.code);
}

function statusWord(kind: string, code: number | null): string {
  if (kind === 'blocked') {
    return t('activity-st-blocked');
  }
  if (code === null) {
    return t('activity-st-failed');
  }
  return String(code);
}

/** The tone of a status: error colour for anything but success. */
export function statusTone(kind: string): '' | 'err' {
  return kind === 'ok' ? '' : 'err';
}
