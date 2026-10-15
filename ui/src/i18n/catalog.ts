/**
 * Which texts exist and which values they take (spec `04-messages`).
 *
 * Each Fluent message is described here: the variables its texts may use and how a raw value becomes one, and the
 * selectors the UI derives from the raw values (`$reason`, …) so a translation never sees a missing variable.
 */

/** `core::message::MessageId`, kept in step with the Rust enum by a test. */
export const ERROR_IDS = [
  'E1',
  'E2',
  'E3',
  'E4',
  'E5',
  'E6',
  'E7',
  'E8',
  'E9',
  'E10',
  'E11',
  'E12',
  'E13',
  'E14',
  'E15',
  'E16',
  'E17',
  'E18',
  'E19',
  'E20',
  'E21',
  'E22',
  'E23',
  'Internal',
] as const;

/** Notifications that are not errors (N1–N12). They come from the UI itself, not from the core. */
export const NOTICE_IDS = ['N1', 'N2', 'N3', 'N4', 'N5', 'N6', 'N7', 'N8', 'N9', 'N10', 'N11', 'N12'] as const;

export type ErrorId = (typeof ERROR_IDS)[number];
export type NoticeId = (typeof NOTICE_IDS)[number];
export type MessageId = ErrorId | NoticeId;

/** A value as it arrives: the core sends strings; the UI may pass numbers and dates for notifications. */
export type RawValue = string | number | Date;
export type RawArgs = Readonly<Record<string, RawValue | undefined>>;

/**
 * - `text`: shown as is;
 * - `count`: a number, so Fluent picks the plural form;
 * - `size`: bytes, shown in Explorer units;
 * - `list`: lines joined into a sentence list;
 * - `moment`: a point in time, shown as a time today and as a date otherwise.
 */
export type ArgKind = 'text' | 'count' | 'size' | 'list' | 'moment';

export interface Selector {
  /** Every value `pick` can return; each needs its own variant in the English text. */
  readonly values: readonly string[];
  readonly pick: (args: RawArgs, now: Date) => string;
}

export interface TextSpec {
  readonly args?: Readonly<Record<string, ArgKind | ((args: RawArgs) => ArgKind)>>;
  readonly selectors?: Readonly<Record<string, Selector>>;
}

function present(args: RawArgs, ...names: string[]): boolean {
  return names.every((name) => args[name] !== undefined && args[name] !== '');
}

/** `args.reason` as the core sends it; anything else (or nothing) selects the last value. */
function reason(...values: string[]): Selector {
  const fallback = values[values.length - 1] ?? 'other';
  return {
    values,
    pick: (args) => {
      const value = args.reason;
      return typeof value === 'string' && values.includes(value) ? value : fallback;
    },
  };
}

export function sameDay(a: Date, b: Date): boolean {
  return a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate();
}

export function toDate(value: RawValue): Date {
  return value instanceof Date ? value : new Date(value);
}

const count = 'count';
const size = 'size';
const text = 'text';

/** Every Fluent message id with its variables. The ids of spec messages are their lower-case spec ids. */
export const TEXTS: Readonly<Record<string, TextSpec>> = {
  // Notifications.
  n1: {},
  n2: { args: { name: text } },
  n3: { args: { count } },
  n4: { args: { count } },
  n5: { selectors: { reason: reason('completed', 'undone') } },
  n6: { args: { count } },
  n7: {
    args: { saved: 'moment' },
    selectors: {
      when: {
        values: ['today', 'earlier'],
        pick: (args, now) => (args.saved !== undefined && sameDay(toDate(args.saved), now) ? 'today' : 'earlier'),
      },
    },
  },
  n8: { args: { name: text, count } },
  n9: { args: { version: text } },
  n10: {},
  n11: { args: { size } },
  n12: { args: { count } },

  // Errors.
  e1: { selectors: { reason: reason('access_denied', 'in_use', 'other') } },
  e2: {},
  e3: {},
  e4: {},
  e5: {
    args: { seconds: count },
    selectors: {
      reason: reason('rate_limited', 'server', 'other'),
      wait: { values: ['seconds', 'later'], pick: (args) => (present(args, 'seconds') ? 'seconds' : 'later') },
    },
  },
  e6: { selectors: { reason: reason('connection', 'timeout', 'server', 'other') } },
  e7: {},
  e8: {
    args: { entry: text },
    selectors: { reason: reason('not_an_archive', 'damaged', 'encrypted', 'unsupported', 'not_a_vpk', 'other') },
  },
  e9: {
    args: {
      limit: (args) => (args.reason === 'too_large' || args.reason === 'memory' ? size : count),
      found: count,
      needed: size,
      entry: text,
    },
    selectors: { reason: reason('too_large', 'too_many_files', 'ratio', 'memory', 'other') },
  },
  e10: {
    args: { contents: 'list' },
    selectors: {
      listing: { values: ['files', 'empty'], pick: (args) => (present(args, 'contents') ? 'files' : 'empty') },
    },
  },
  e11: {},
  e12: {
    args: { needed: size, available: size },
    selectors: {
      reason: { values: ['space', 'full'], pick: (args) => (present(args, 'needed', 'available') ? 'space' : 'full') },
    },
  },
  e13: {},
  e14: {},
  e15: {},
  e16: { args: { host: text } },
  e17: { args: { file: text }, selectors: { reason: reason('newer', 'unreadable', 'not_a_profile') } },
  e18: { selectors: { reason: reason('access_denied', 'in_use', 'other') } },
  e19: { selectors: { reason: reason('no_response', 'not_an_image', 'too_large', 'other') } },
  e20: { args: { wanted: count, free: count } },
  e21: { args: { name: text } },
  e22: { args: { name: text } },
  e23: {
    args: { file: text, line: count, column: count },
    selectors: {
      reason: {
        values: ['damaged_at', 'damaged', 'newer', 'invalid'],
        pick: (args) => {
          if (args.reason === 'damaged') {
            return present(args, 'line', 'column') ? 'damaged_at' : 'damaged';
          }
          return args.reason === 'newer' ? 'newer' : 'invalid';
        },
      },
      kind: {
        values: ['settings', 'manifest', 'other'],
        pick: (args) => {
          const file = String(args.file ?? '');
          if (file.endsWith('settings.json')) {
            return 'settings';
          }
          return file.endsWith('manifest.json') ? 'manifest' : 'other';
        },
      },
    },
  },
  internal: {},

  // Buttons named in the spec texts.
  'action-undo': {},
  'action-review': {},
  'action-retry': {},
  'action-retry-now': {},
  'action-browse': {},
  'action-verify-in-steam': {},
  'action-show-differences': {},
  'action-details': {},
  'action-repair': {},
  'action-copy-link': {},
  'action-open-folder': {},
  'action-open-release-page': {},
  'action-enable-catalog': {},
  'action-learn-more': {},
  'action-view-log': {},

  /** The log entry a message links to (decision 98). */
  'log-entry': { args: { id: text } },
  /** The end of a shortened list: "… and 3 more". */
  'list-more': { args: { count } },

  // Sizes, in the units Windows Explorer shows in each language.
  'size-bytes': { args: { count } },
  'size-kb': { args: { value: text } },
  'size-mb': { args: { value: text } },
  'size-gb': { args: { value: text } },
  'size-tb': { args: { value: text } },
};

/** The Fluent message id of a spec message: `E12` → `e12`, `Internal` → `internal`. */
export function textId(id: MessageId): string {
  return id.toLowerCase();
}
