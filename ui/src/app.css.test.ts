import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

// Read as text: the test setup doesn't process this stylesheet.
const css = readFileSync(join(process.cwd(), 'src', 'app.css'), 'utf8');

describe('the window frame', () => {
  it('never scrolls as a whole: only the areas inside the shell scroll', () => {
    const root = /html,\s*body,\s*#app\s*\{([^}]*)\}/.exec(css)?.[1] ?? '';
    expect(root).toMatch(/height:\s*100%/);
    expect(root).toMatch(/overflow:\s*hidden/);
  });
});
