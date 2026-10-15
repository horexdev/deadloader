import { describe, expect, it } from 'vitest';
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

// Read as text: the test setup doesn't process this stylesheet.
const css = readFileSync(join(process.cwd(), 'src', 'app.css'), 'utf8');

describe('the window frame', () => {
  it('never scrolls as a whole: only the areas inside the shell scroll', () => {
    const root = /html,\s*body,\s*#app\s*\{([^}]*)\}/.exec(css)?.[1] ?? '';
    expect(root).toMatch(/height:\s*100%/);
    expect(root).toMatch(/overflow:\s*hidden/);
  });

  it('has no minimum size of its own: the window keeps it, and zoom makes the page smaller than 960×600', () => {
    // At 110 % zoom a 960 px window is 873 CSS px wide: a CSS minimum of 960 px would stick out of it, cut off.
    const src = join(process.cwd(), 'src');
    const offenders = readdirSync(src, { recursive: true, encoding: 'utf8' })
      .filter((file) => /\.(css|svelte)$/.test(file) && !file.startsWith('theme'))
      .filter((file) => /var\(--window-min-[wh]\)/.test(readFileSync(join(src, file), 'utf8')));
    expect(offenders).toEqual([]);
  });
});

describe('the product name in texts', () => {
  it('is set apart in the accent text colour, nothing more (decision 193)', () => {
    const rule = /\.dl-brand\s*\{([^}]*)\}/.exec(css)?.[1] ?? '';
    expect(rule).toMatch(/color:\s*var\(--accent-text\)/);
    expect(rule).not.toMatch(/font-weight|background|gradient/);
  });
});
