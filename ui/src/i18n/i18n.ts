import { FluentBundle, FluentResource, type FluentVariable } from '@fluent/bundle';
import {
  type ArgKind,
  ERROR_IDS,
  type MessageId,
  type RawArgs,
  type RawValue,
  TEXTS,
  sameDay,
  textId,
  toDate,
} from './catalog';
import { language, showLanguage } from './current.svelte';
import { FALLBACK, type Lang, resolveLanguage } from './languages';
import { sizeParts } from './size';
// A language is a folder of files, `messages.ftl` (spec 04-messages) plus one file per area of the UI, so screens can
// add their texts without touching each other's files. English is the fallback for anything missing, so it is part of
// the main bundle; the others load on demand.
const english = Object.values(
  import.meta.glob<string>('./locales/en/*.ftl', { query: '?raw', import: 'default', eager: true }),
);
const loaders = import.meta.glob<string>(['./locales/*/*.ftl', '!./locales/en/*.ftl'], {
  query: '?raw',
  import: 'default',
});

function createBundle(lang: Lang, sources: readonly string[]): FluentBundle {
  // No bidi isolation marks: none of our languages is written right to left (decision 106).
  const bundle = new FluentBundle(lang, { useIsolating: false });
  for (const source of sources) {
    bundle.addResource(new FluentResource(source));
  }
  return bundle;
}

const bundles = new Map<Lang, FluentBundle>([[FALLBACK, createBundle(FALLBACK, english)]]);
const fallback = bundles.get(FALLBACK) as FluentBundle;

let request = 0;

export { language };

async function load(lang: Lang): Promise<FluentBundle> {
  const loaded = bundles.get(lang);
  if (loaded) {
    return loaded;
  }
  const files = Object.entries(loaders)
    .filter(([path]) => path.startsWith(`./locales/${lang}/`))
    .map(([, loader]) => loader());
  if (files.length === 0) {
    return fallback;
  }
  const bundle = createBundle(lang, await Promise.all(files));
  bundles.set(lang, bundle);
  return bundle;
}

/**
 * Applies the `general.language` setting: `"system"` picks the first of `preferred` we have (English if none), a
 * language code is used as is. Resolves to the language now shown.
 */
export async function setLanguage(setting: string, preferred: readonly string[] = navigator.languages): Promise<Lang> {
  const lang = resolveLanguage(setting, preferred);
  const mine = ++request;
  await load(lang);
  if (mine === request) {
    showLanguage(lang);
    if (typeof document !== 'undefined') {
      // Lets CSS pick the font stack of the script and the right CJK glyph shapes.
      document.documentElement.lang = lang;
    }
  }
  return language();
}

function formatIn(bundle: FluentBundle, key: string, attr: string | undefined, args?: Record<string, FluentVariable>) {
  const message = bundle.getMessage(key);
  const pattern = attr === undefined ? message?.value : message?.attributes[attr];
  if (pattern === undefined || pattern === null) {
    return null;
  }
  return bundle.formatPattern(pattern, args, []);
}

/** The text `id` (`"e12"`, or `"e12.why"` for an attribute) in the current language, falling back to English. */
export function t(id: string, args?: Record<string, FluentVariable>): string {
  const [key = '', attr] = id.split('.', 2);
  const bundle = bundles.get(language()) ?? fallback;
  return formatIn(bundle, key, attr, args) ?? formatIn(fallback, key, attr, args) ?? id;
}

/** Windows' regional format (178): the clock (12 or 24 hours) and how numbers are written. Words stay in the UI language. */
let region: string | undefined;
let regionHourCycle: Intl.DateTimeFormatOptions['hourCycle'];

/** Sets the regional format the core reported (`system_locale`); `undefined` or a tag that isn't one: the language's own. */
export function setRegion(tag: string | undefined): void {
  region = undefined;
  regionHourCycle = undefined;
  if (tag === undefined) {
    return;
  }
  try {
    const canonical = Intl.getCanonicalLocales(tag)[0];
    if (canonical === undefined) {
      return;
    }
    region = canonical;
    regionHourCycle = new Intl.DateTimeFormat(canonical, { hour: 'numeric' }).resolvedOptions().hourCycle;
  } catch {
    region = undefined;
    regionHourCycle = undefined;
  }
}

const numberFormats = new Map<string, Intl.NumberFormat>();

function numberFormat(digits?: number): Intl.NumberFormat {
  const locale = region ?? language();
  const key = `${locale}|${digits ?? ''}`;
  let format = numberFormats.get(key);
  if (!format) {
    format = new Intl.NumberFormat(
      locale,
      digits === undefined ? undefined : { minimumFractionDigits: digits, maximumFractionDigits: digits },
    );
    numberFormats.set(key, format);
  }
  return format;
}

export function formatNumber(value: number): string {
  return numberFormat().format(value);
}

/** Bytes in the units Explorer shows in the current language ("1,30 KB", "1,30 Ko", "1,30 КБ"). */
export function formatSize(bytes: number): string {
  const { unit, value, digits } = sizeParts(bytes);
  const text =
    unit === 'bytes'
      ? t('size-bytes', { count: value })
      : t(`size-${unit}`, { value: numberFormat(digits).format(value) });
  // The unit stays on the line of its number.
  return text.replaceAll(' ', ' ');
}

export function formatTime(moment: Date | number): string {
  return new Intl.DateTimeFormat(language(), {
    hour: 'numeric',
    minute: '2-digit',
    hourCycle: regionHourCycle,
  }).format(moment);
}

/**
 * Day and month, plus the year when it is not the current one. The month is written out: abbreviations end with a
 * dot in many of our languages ("12. Okt."), which reads badly at the end of a sentence.
 */
export function formatDate(moment: Date | number, now: Date = new Date()): string {
  const date = new Date(moment);
  const year = date.getFullYear() === now.getFullYear() ? undefined : 'numeric';
  return new Intl.DateTimeFormat(language(), { day: 'numeric', month: 'long', year }).format(date);
}

const LIST_LIMIT = 5;

/** "a, b and c"; longer lists end with "and N more". */
export function formatList(items: readonly string[]): string {
  const shown =
    items.length > LIST_LIMIT
      ? [...items.slice(0, LIST_LIMIT), t('list-more', { count: items.length - LIST_LIMIT })]
      : [...items];
  return new Intl.ListFormat(language(), { type: 'conjunction' }).format(shown);
}

function convert(kind: ArgKind, value: RawValue, now: Date): FluentVariable {
  switch (kind) {
    case 'count': {
      const n = Number(value);
      return Number.isFinite(n) ? n : String(value);
    }
    case 'size':
      return formatSize(Number(value));
    case 'list':
      return formatList(
        String(value)
          .split('\n')
          .filter((line) => line !== ''),
      );
    case 'moment': {
      const date = toDate(value);
      return sameDay(date, now) ? formatTime(date) : formatDate(date, now);
    }
    case 'text':
      return value instanceof Date ? formatDate(value, now) : String(value);
  }
}

/** Turns raw values into what the texts of `key` expect, adding the selectors derived from them. */
function prepare(key: string, args: RawArgs, now: Date): Record<string, FluentVariable> {
  const spec = TEXTS[key] ?? {};
  const out: Record<string, FluentVariable> = {};
  for (const [name, kindOf] of Object.entries(spec.args ?? {})) {
    const value = args[name];
    if (value !== undefined) {
      out[name] = convert(typeof kindOf === 'function' ? kindOf(args) : kindOf, value, now);
    }
  }
  for (const [name, selector] of Object.entries(spec.selectors ?? {})) {
    out[name] = selector.pick(args, now);
  }
  return out;
}

/** A message from the core (`core::message::UserMessage`) or a notification of the UI. */
export interface MessageInput {
  readonly id: MessageId | string;
  readonly args?: RawArgs;
}

export interface MessageText {
  readonly what: string;
  /** Always set for errors; set for notifications that explain more. */
  readonly why: string | null;
  /** Always set for errors. */
  readonly todo: string | null;
  /** Technical details for the Details view, untranslated. */
  readonly detail: string | null;
}

function isKnown(id: string): id is MessageId {
  return TEXTS[textId(id as MessageId)] !== undefined && /^(E\d+|N\d+|Internal)$/.test(id);
}

/** "What happened / why / what to do" for a message (decision 98), in the current language. */
export function describe(message: MessageInput, now: Date = new Date()): MessageText {
  const args = message.args ?? {};
  // A newer core may send an id this UI doesn't know yet; it is still our problem, never the user's.
  const id: MessageId = isKnown(message.id) ? message.id : 'Internal';
  const key = textId(id);
  const values = prepare(key, args, now);
  const part = (attr: string): string | null => {
    const bundle = bundles.get(language()) ?? fallback;
    return formatIn(bundle, key, attr, values) ?? formatIn(fallback, key, attr, values);
  };
  const isError = (ERROR_IDS as readonly string[]).includes(id);
  const detail = args.detail;
  return {
    what: t(key, values),
    why: part('why') ?? (isError ? '' : null),
    todo: part('todo') ?? (isError ? '' : null),
    detail: detail === undefined ? null : String(detail),
  };
}
