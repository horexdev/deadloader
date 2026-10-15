import { describe, expect, it } from 'vitest';

// Decision 193: wherever a text can name deadloader it is shown through `Text`, which sets the name apart.
const sources = import.meta.glob<string>(['./**/*.svelte'], { query: '?raw', import: 'default', eager: true });
const english = import.meta.glob<string>('./i18n/locales/en/*.ftl', { query: '?raw', import: 'default', eager: true });
const locales = import.meta.glob<string>('./i18n/locales/*/*.ftl', { query: '?raw', import: 'default', eager: true });

const NAME = /(?<![\w\\/.-])deadloader(?![\w-]|\.\w)/;

/** Message ids whose English text (value or an attribute) names the product. */
function namingIds(): Set<string> {
  const ids = new Set<string>();
  for (const source of Object.values(english)) {
    let current = '';
    for (const line of source.split('\n')) {
      current = /^([a-z][\w-]*)\s*=/.exec(line)?.[1] ?? current;
      if (current !== '' && !line.trimStart().startsWith('#') && NAME.test(line)) {
        ids.add(current);
      }
    }
  }
  return ids;
}

/** `{t('id' …)}` naming the product as text in a component's markup (not an attribute or prop value). */
function plainNamingTexts(source: string, ids: Set<string>): string[] {
  const markup = source.replace(/<(script|style)[^>]*>[\s\S]*?<\/\1>/g, (block) => block.replace(/[^\n]/g, ' '));
  const found: string[] = [];
  for (const match of markup.matchAll(/\bt\(\s*['"]([a-z][\w-]*)/g)) {
    if (!ids.has(match[1] ?? '')) {
      continue;
    }
    let depth = 0;
    let open = match.index;
    while (open > 0) {
      open -= 1;
      const c = markup[open];
      if (c === '}') {
        depth += 1;
      } else if (c === '{' && markup[open - 1] !== '$') {
        if (depth === 0) {
          break;
        }
        depth -= 1;
      } else if (c === '{') {
        // A template literal's `${…}`: the text is part of a longer expression.
        depth -= depth > 0 ? 1 : 0;
      }
    }
    // An attribute or prop value, or an object or argument inside an expression (`{#each [{ … }]}`, `tooltip({ … })`).
    if (!/[=([,:?]\s*$/.test(markup.slice(0, open))) {
      found.push(`${match[1] ?? ''} line ${markup.slice(0, match.index).split('\n').length}`);
    }
  }
  return found;
}

describe('the product name', () => {
  it('is shown through Text wherever a text names it', () => {
    const ids = namingIds();
    expect(ids.size).toBeGreaterThan(100);
    const offenders = Object.entries(sources)
      .map(([file, source]) => plainNamingTexts(source, ids).map((where) => `${file}: ${where}`))
      .flat();
    expect(offenders).toEqual([]);
  });

  it('is set apart in the text props of the design components', () => {
    for (const name of ['Banner', 'Dialog', 'EmptyState', 'Group', 'Row']) {
      const file = Object.keys(sources).find((path) => path.endsWith(`/${name}.svelte`)) ?? name;
      expect(sources[file] ?? '', file).toMatch(/<Text value=/);
    }
  });

  it('is spelled the same in every language', () => {
    const misspelt = Object.entries(locales).flatMap(([file, source]) =>
      [...source.matchAll(/deadloader/gi)].filter((m) => m[0] !== 'deadloader').map((m) => `${file}: ${m[0]}`),
    );
    expect(misspelt).toEqual([]);
  });
});
