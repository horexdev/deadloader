import { FluentResource } from '@fluent/bundle';
import type { Message } from '@fluent/bundle';
import { describe, expect, it } from 'vitest';
import messageRs from '../../../crates/core/src/message.rs?raw';
import { ERROR_IDS, type ErrorId, NOTICE_IDS, TEXTS, textId } from './catalog';
import { LANGS, type Lang } from './languages';

// The runtime AST of @fluent/bundle (its `ast.d.ts`), walked to compare translations structurally.
type Pattern = NonNullable<Message['value']>;
type Expression = Exclude<Extract<Pattern, unknown[]>[number], string>;
type Select = Extract<Expression, { type: 'select' }>;

const sources = import.meta.glob<string>('./locales/*.ftl', { query: '?raw', import: 'default', eager: true });
const crates = import.meta.glob<string>('../../../crates/*/src/**/*.rs', {
  query: '?raw',
  import: 'default',
  eager: true,
});

/**
 * (message id, reason) pairs a Rust file sends: `MessageId::E7).arg("reason", "size")`, and helpers such as
 * `let e6 = |reason: &str| UserMessage::new(MessageId::E6).arg("reason", reason)` called as `e6("dns")` or
 * `e6(match kind { … => "tls", … })`.
 */
function reasonsSent(rust: string): [string, string][] {
  const pairs: [string, string][] = [];
  for (const m of rust.matchAll(/MessageId::(E\d+)\)\s*\.arg\(\s*"reason",\s*"(\w+)"/g)) {
    pairs.push([m[1] ?? '', m[2] ?? '']);
  }
  for (const helper of rust.matchAll(/let (\w+) = \|reason: &str\|\s*\{?\s*UserMessage::new\(MessageId::(E\d+)\)/g)) {
    const [, name = '', id = ''] = helper;
    for (const call of rust.matchAll(new RegExp(String.raw`\b${name}\(\s*"(\w+)"`, 'g'))) {
      pairs.push([id, call[1] ?? '']);
    }
    for (const call of rust.matchAll(new RegExp(String.raw`\b${name}\(match [^{]*\{([^}]*)\}`, 'g'))) {
      for (const arm of (call[1] ?? '').matchAll(/=>\s*"(\w+)"/g)) {
        pairs.push([id, arm[1] ?? '']);
      }
    }
  }
  return pairs;
}

function source(lang: Lang): string {
  const text = sources[`./locales/${lang}.ftl`];
  if (text === undefined) {
    throw new Error(`locales/${lang}.ftl is missing`);
  }
  return text;
}

function messages(lang: Lang): Map<string, Message> {
  const body = new FluentResource(source(lang)).body;
  return new Map(body.filter((entry) => !entry.id.startsWith('-')).map((entry) => [entry.id, entry as Message]));
}

const english = messages('en');
const others = LANGS.filter((lang) => lang !== 'en');

/** The value and every attribute of a message, by name (`""` is the value). */
function patterns(message: Message): Map<string, Pattern> {
  const all = new Map<string, Pattern>(Object.entries(message.attributes));
  if (message.value !== null) {
    all.set('', message.value);
  }
  return all;
}

function expressions(pattern: Pattern): Expression[] {
  if (typeof pattern === 'string') {
    return [];
  }
  const found: Expression[] = [];
  const visit = (e: Expression): void => {
    found.push(e);
    if (e.type === 'select') {
      visit(e.selector);
      e.variants.forEach((v) => expressions(v.value).forEach(visit));
    }
  };
  pattern.forEach((element) => {
    if (typeof element !== 'string') {
      visit(element);
    }
  });
  return found;
}

function variables(pattern: Pattern): Set<string> {
  return new Set(expressions(pattern).flatMap((e) => (e.type === 'var' ? [e.name] : [])));
}

function selects(pattern: Pattern): Select[] {
  return expressions(pattern).filter((e): e is Select => e.type === 'select');
}

function selectorName(select: Select): string {
  return select.selector.type === 'var' ? select.selector.name : `<${select.selector.type}>`;
}

function keys(select: Select): string[] {
  return select.variants.map((v) => String(v.key.value));
}

/** Plural categories integers can take in a language, and all of its categories (decimals included). */
function pluralCategories(lang: Lang): { integers: Set<string>; all: Set<string> } {
  const rules = new Intl.PluralRules(lang);
  const integers = new Set<string>();
  for (let n = 0; n <= 1_000_000; n = n < 1100 ? n + 1 : n * 10) {
    integers.add(rules.select(n));
  }
  return { integers, all: new Set(rules.resolvedOptions().pluralCategories) };
}

describe('locale files', () => {
  it('exist for exactly the sixteen languages', () => {
    expect(Object.keys(sources).sort()).toEqual(LANGS.map((lang) => `./locales/${lang}.ftl`).sort());
  });

  it.each(LANGS)('%s parses completely: every entry in the file is understood', (lang) => {
    // Fluent skips what it can't parse; an entry that went missing this way shows up here.
    const declared = [...source(lang).matchAll(/^(-?[a-zA-Z][\w-]*)\s*=/gm)].map((m) => m[1]);
    expect(new FluentResource(source(lang)).body.map((entry) => entry.id)).toEqual(declared);
  });
});

describe('the English texts', () => {
  it('cover every text the catalog describes, and nothing else', () => {
    expect([...english.keys()].sort()).toEqual(Object.keys(TEXTS).sort());
  });

  it('have a message for every MessageId of core::message', () => {
    const body = /pub enum MessageId \{([^}]*)\}/.exec(messageRs)?.[1];
    expect(body, 'MessageId not found in message.rs').toBeDefined();
    const variants = (body ?? '')
      .split('\n')
      .map((line) => line.trim())
      .filter((line) => line !== '' && !line.startsWith('//'))
      .map((line) => line.replace(/,$/, ''));
    expect(variants).toEqual([...ERROR_IDS]);
    for (const id of ERROR_IDS) {
      const message = english.get(textId(id));
      expect(message?.value, id).toBeTruthy();
      expect(Object.keys(message?.attributes ?? {}).sort(), id).toEqual(['todo', 'why']);
    }
  });

  it('have a message for every notification of the spec', () => {
    for (const id of NOTICE_IDS) {
      expect(english.get(textId(id))?.value, id).toBeTruthy();
    }
  });

  it('know every reason the core sends', () => {
    const sent = new Set(
      [...messageRs.matchAll(/"reason",\s*"(\w+)"|unpack\(\s*"(\w+)"/g)].map((m) => m[1] ?? m[2] ?? ''),
    );
    expect(sent.size).toBeGreaterThan(10);
    const known = new Set(ERROR_IDS.flatMap((id) => TEXTS[textId(id)]?.selectors?.reason?.values ?? []));
    for (const value of sent) {
      expect(known, value).toContain(value);
    }
  });

  it('have a variant for every reason any crate sends with its message', () => {
    const sent = [...Object.entries(crates)].flatMap(([file, text]) =>
      reasonsSent(text).map(([id, reason]) => ({ file, id, reason })),
    );
    // The downloads crate sends E6, E7 and E16 with reasons; if this drops, the scan no longer sees them.
    expect(sent.filter((s) => s.file.includes('/downloads/')).length).toBeGreaterThan(10);
    for (const { file, id, reason } of sent) {
      const values = TEXTS[textId(id as ErrorId)]?.selectors?.reason?.values ?? [];
      expect(values, `${id} [${reason}] from ${file}`).toContain(reason);
    }
  });

  it.each(Object.keys(TEXTS))('%s uses only its declared variables', (id) => {
    const spec = TEXTS[id] ?? {};
    const declared = new Set([...Object.keys(spec.args ?? {}), ...Object.keys(spec.selectors ?? {})]);
    const message = english.get(id);
    expect(message).toBeDefined();
    for (const [, pattern] of patterns(message as Message)) {
      for (const name of variables(pattern)) {
        expect(declared, `$${name}`).toContain(name);
      }
    }
  });

  const selecting = Object.keys(TEXTS).filter((id) => TEXTS[id]?.selectors);
  it.each(selecting)('%s has a variant for every selector value', (id) => {
    const message = english.get(id) as Message;
    const all = [...patterns(message).values()].flatMap(selects);
    for (const [name, selector] of Object.entries(TEXTS[id]?.selectors ?? {})) {
      const using = all.filter((s) => selectorName(s) === name);
      expect(using.length, `$${name} is never selected on`).toBeGreaterThan(0);
      // At least one text spells out every case; the others may share a default.
      expect(
        using.some((s) => selector.values.every((v) => keys(s).includes(v))),
        `$${name}: ${selector.values.join(', ')}`,
      ).toBe(true);
      for (const s of using) {
        for (const key of keys(s)) {
          expect(selector.values, `$${name} [${key}]`).toContain(key);
        }
      }
    }
  });
});

describe.each(others)('%s', (lang) => {
  const translated = messages(lang);

  it('has every key of English and no others', () => {
    expect([...translated.keys()].sort()).toEqual([...english.keys()].sort());
  });

  it('has the same attributes and variables in every message', () => {
    for (const [id, source] of english) {
      const target = translated.get(id);
      expect(target, id).toBeDefined();
      const want = patterns(source);
      const got = patterns(target as Message);
      expect([...got.keys()].sort(), id).toEqual([...want.keys()].sort());
      for (const [attr, pattern] of want) {
        const name = attr === '' ? id : `${id}.${attr}`;
        expect([...variables(got.get(attr) as Pattern)].sort(), name).toEqual([...variables(pattern)].sort());
      }
    }
  });

  it('chooses between the same cases as English', () => {
    for (const [id, source] of english) {
      const selectorNames = new Set(Object.keys(TEXTS[id]?.selectors ?? {}));
      for (const [attr, pattern] of patterns(source)) {
        const want = selects(pattern).filter((s) => selectorNames.has(selectorName(s)));
        const got = selects(patterns(translated.get(id) as Message).get(attr) as Pattern).filter((s) =>
          selectorNames.has(selectorName(s)),
        );
        const name = attr === '' ? id : `${id}.${attr}`;
        expect(
          got.map((s) => [selectorName(s), keys(s).sort()]),
          name,
        ).toEqual(want.map((s) => [selectorName(s), keys(s).sort()]));
      }
    }
  });

  it('has every plural form the language needs', () => {
    const { integers, all } = pluralCategories(lang);
    for (const [id, message] of translated) {
      const spec = TEXTS[id] ?? {};
      for (const [attr, pattern] of patterns(message)) {
        for (const select of selects(pattern)) {
          const name = selectorName(select);
          if (spec.selectors?.[name]) {
            continue;
          }
          const where = `${attr === '' ? id : `${id}.${attr}`} $${name}`;
          expect(spec.args?.[name], `${where} is not a number`).toBe('count');
          const words = select.variants.filter((v) => v.key.type === 'str').map((v) => String(v.key.value));
          for (const key of words) {
            expect(all, `${where} [${key}]`).toContain(key);
          }
          for (const category of integers) {
            expect(words, `${where} needs [${category}]`).toContain(category);
          }
        }
      }
    }
  });
});

describe('English plural forms', () => {
  it('are complete as well', () => {
    const { integers } = pluralCategories('en');
    for (const [id, message] of english) {
      for (const [, pattern] of patterns(message)) {
        for (const select of selects(pattern)) {
          if (TEXTS[id]?.args?.[selectorName(select)] === 'count') {
            for (const category of integers) {
              expect(keys(select), id).toContain(category);
            }
          }
        }
      }
    }
  });
});
