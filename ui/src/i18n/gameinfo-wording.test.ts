import { describe, expect, it } from 'vitest';

// Since 0.1.1 deadloader adds up to three lines to gameinfo.gi (the addons path, `Mod`, `Write`), so no text may
// speak of its one line. English is the source every translation follows.
const english = import.meta.glob<string>('./locales/en/*.ftl', { query: '?raw', import: 'default', eager: true });

const ONE_LINE = /\b(one line|its (own )?line|our line|deadloader’s line|the line deadloader)\b/i;

describe('texts about gameinfo.gi', () => {
  it('never speak of a single line deadloader adds', () => {
    const offenders = Object.entries(english).flatMap(([file, source]) =>
      source
        .split('\n')
        .filter((line) => ONE_LINE.test(line))
        .map((line) => `${file}: ${line.trim()}`),
    );
    expect(offenders).toEqual([]);
  });
});
