// The applying banner's numbers (board LibraryApplying): how far the bar is, the step in progress, the files linked or
// copied so far, and the time left when the core could estimate it from the copy rate.

import type { Applying } from '../../lib/api';
import { t } from '../../i18n';

/** 0–1: by bytes once files are copied (the copies take the time), else by steps (links are instant). */
export function progressValue(p: Applying): number {
  if (p.bytes_copied > 0 && p.bytes_total > 0) {
    return Math.min(1, p.bytes_done / p.bytes_total);
  }
  return p.total > 0 ? Math.min(1, p.done / p.total) : 0;
}

/** "Step 4 of 10": the step running now. */
export function progressStep(p: Applying): string {
  return t('library-applying-step', { done: Math.min(p.done + 1, p.total), total: p.total });
}

/** "9 files linked, 1 copied"; empty before the first file is placed. */
export function progressFiles(p: Applying): string {
  if (p.linked > 0 && p.copied > 0) {
    return t('library-applying-both', { linked: p.linked, copied: p.copied });
  }
  if (p.copied > 0) {
    return t('library-applying-copied', { count: p.copied });
  }
  return p.linked > 0 ? t('library-applying-linked', { count: p.linked }) : '';
}

/** "About 5 s left", rounded up; empty without an estimate or under a second. */
export function progressLeft(p: Applying): string {
  if (p.left_ms === null || p.left_ms < 1000) {
    return '';
  }
  const total = Math.ceil(p.left_ms / 1000);
  return total < 60
    ? t('library-applying-left-seconds', { seconds: total })
    : t('library-applying-left-minutes', { minutes: Math.floor(total / 60), seconds: total % 60 });
}
