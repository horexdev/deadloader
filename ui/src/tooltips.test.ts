import { describe, expect, it } from 'vitest';

// Every tooltip is the design system's (`tooltip` attachment): a native `title` shows Windows' own tooltip instead.
const sources = import.meta.glob<string>(['./**/*.svelte'], { query: '?raw', import: 'default', eager: true });
// Code that builds DOM itself (the mod description) sets titles with DOM calls instead of markup.
const scripts = import.meta.glob<string>(['./**/*.ts', '!./**/*.test.ts'], {
  query: '?raw',
  import: 'default',
  eager: true,
});

/** A native title set from code: `element.title = …` or `setAttribute('title', …)` (`document.title` is the window's). */
function scriptTitles(source: string): string[] {
  return [...source.matchAll(/(?<!document)\.title\s*=(?!=)|setAttribute\(\s*['"]title['"]/g)].map(
    (match) => `line ${source.slice(0, match.index).split('\n').length}`,
  );
}

/** `title=` on a plain HTML element in a component's markup (components like Dialog or Banner take `title` as a
 *  heading, which is fine). */
function nativeTitles(source: string): string[] {
  const markup = source.replace(/<(script|style)[^>]*>[\s\S]*?<\/\1>/g, (block) => block.replace(/[^\n]/g, ' '));
  const found: string[] = [];
  for (const match of markup.matchAll(/(?<![\w:-])title=/g)) {
    const start = markup.lastIndexOf('<', match.index);
    const tag = /^<([A-Za-z][\w.:-]*)/.exec(markup.slice(start))?.[1] ?? '?';
    if (/^[a-z]/.test(tag)) {
      found.push(`<${tag}> line ${markup.slice(0, match.index).split('\n').length}`);
    }
  }
  return found;
}

describe('tooltips', () => {
  it('are never native title attributes', () => {
    const offenders = Object.entries(sources)
      .map(([file, source]) => nativeTitles(source).map((where) => `${file}: ${where}`))
      .flat();
    expect(offenders).toEqual([]);
  });

  it('are never native titles set from code', () => {
    expect(scriptTitles("a.title = url;\nb.setAttribute('title', x);\ndocument.title = 'deadloader';")).toEqual([
      'line 1',
      'line 2',
    ]);
    const offenders = Object.entries(scripts)
      .map(([file, source]) => scriptTitles(source).map((where) => `${file}: ${where}`))
      .flat();
    expect(offenders).toEqual([]);
  });

  it('are what Button and Toggle make of their title, not a native one', () => {
    for (const file of ['./lib/components/Button.svelte', './lib/components/Toggle.svelte']) {
      const source = sources[file] ?? '';
      expect(source, file).toContain('tooltip(');
      expect(source, file).not.toMatch(/\{title\}/);
    }
  });
});
