import { describe, expect, it } from 'vitest';

// Every tooltip is the design system's (`tooltip` attachment): a native `title` shows Windows' own tooltip instead.
const sources = import.meta.glob<string>(['./**/*.svelte'], { query: '?raw', import: 'default', eager: true });

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

  it('are what Button and Toggle make of their title, not a native one', () => {
    for (const file of ['./lib/components/Button.svelte', './lib/components/Toggle.svelte']) {
      const source = sources[file] ?? '';
      expect(source, file).toContain('tooltip(');
      expect(source, file).not.toMatch(/\{title\}/);
    }
  });
});
