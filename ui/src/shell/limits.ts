// The limits table of E9 in D12 (board DownloadsD12): the limits of Settings → Downloads, what this archive measured
// and whether it went over. Values come from the core's message; what isn't known shows as "—".

import type { RawArgs } from '../i18n';
import { formatNumber, formatSize, t } from '../i18n';

export interface LimitRow {
  check: string;
  /** What this archive measured. */
  value: string;
  limit: string;
  /** `null`: not known (unpacking stopped before this check saw every file). */
  result: 'over' | 'within' | null;
}

const UNKNOWN = '—';

function number(args: RawArgs, key: string): number | null {
  const raw = args[key];
  if (raw === undefined || raw === null || raw === '') {
    return null;
  }
  const n = Number(raw);
  return Number.isFinite(n) ? n : null;
}

function times(n: number): string {
  return t('errors-limit-times', { limit: formatNumber(Math.round(n * 10) / 10) });
}

function mark(over: boolean, known: boolean): LimitRow['result'] {
  return over ? 'over' : known ? 'within' : null;
}

/** The rows for an E9's arguments; none when the message doesn't carry the limits (an older core). */
export function limitRows(args: RawArgs): LimitRow[] {
  const sizeLimit = number(args, 'size_limit');
  const factorLimit = number(args, 'factor_limit');
  const filesLimit = number(args, 'files_limit');
  const ratioLimit = number(args, 'ratio_limit');
  if (sizeLimit === null || factorLimit === null || filesLimit === null || ratioLimit === null) {
    return [];
  }
  const reason = String(args.reason ?? '');
  const limit = number(args, 'limit');
  const unpacked = number(args, 'unpacked');
  const archive = number(args, 'archive_size');
  const files = number(args, 'files') ?? number(args, 'found');
  const moreThan = (value: string) => t('errors-limit-more-than', { limit: value });

  // The total may not pass the smaller of the size and factor × the archive: which one stopped it.
  const binding = reason === 'too_large' && limit !== null ? (limit >= sizeLimit ? 'size' : 'factor') : null;
  // The archive's own list: not believed when it claims less than the limit that was reached on real bytes.
  const believed = unpacked !== null && (reason !== 'too_large' || limit === null || unpacked > limit) ? unpacked : null;
  const ratio = believed !== null && archive !== null && archive > 0 ? believed / archive : null;

  const sizeOver = binding === 'size' || (believed !== null && believed > sizeLimit);
  const factorOver = binding === 'factor' || (ratio !== null && ratio > factorLimit);
  const rows: LimitRow[] = [
    {
      check: t('errors-limit-size'),
      value: believed !== null ? formatSize(believed) : binding === 'size' ? moreThan(formatSize(sizeLimit)) : UNKNOWN,
      limit: formatSize(sizeLimit),
      result: mark(sizeOver, believed !== null),
    },
    {
      check: t('errors-limit-ratio'),
      value: ratio !== null ? times(ratio) : binding === 'factor' ? moreThan(times(factorLimit)) : UNKNOWN,
      limit: times(factorLimit),
      result: mark(factorOver, ratio !== null),
    },
    {
      check: t('errors-limit-files'),
      value: files === null ? UNKNOWN : formatNumber(files),
      limit: formatNumber(filesLimit),
      // The count is checked before anything is unpacked, so any other stop means it was within.
      result: reason === 'too_many_files' ? 'over' : 'within',
    },
  ];
  if (reason === 'ratio') {
    rows.push({
      check: t('errors-limit-entry-ratio'),
      value: t('errors-limit-entry-over', { entry: String(args.entry ?? ''), limit: formatNumber(ratioLimit) }),
      limit: times(ratioLimit),
      result: 'over',
    });
  }
  const needed = number(args, 'needed');
  if (reason === 'memory' && needed !== null && limit !== null) {
    rows.push({ check: t('errors-limit-memory'), value: formatSize(needed), limit: formatSize(limit), result: 'over' });
  }
  return rows;
}
