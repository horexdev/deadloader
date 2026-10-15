import { describe, expect, it } from 'vitest';
import tokens from '../theme/tokens.css?raw';
import fonts from './fonts.css?raw';

describe('fonts per script (decision 106: system CJK fonts, none embedded)', () => {
  it.each([
    ['ja', '--font-sans-ja'],
    ['ko', '--font-sans-ko'],
    ['zh', '--font-sans-zh'],
  ])('%s uses the %s stack of the design tokens', (lang, token) => {
    expect(tokens).toContain(`${token}:`);
    const rule = new RegExp(`:root:lang\\(${lang}\\)\\s*\\{\\s*--font-sans:\\s*var\\(${token}\\);\\s*\\}`);
    expect(fonts).toMatch(rule);
  });

  it('embeds no font files', () => {
    expect(fonts).not.toMatch(/@font-face|url\(/);
  });
});
