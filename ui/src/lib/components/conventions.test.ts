import { describe, expect, it } from 'vitest';
import * as library from './index';

const sources = import.meta.glob<string>('./*.svelte', { query: '?raw', import: 'default', eager: true });

describe('component conventions', () => {
  it('use no static style attributes: the CSP (default-src self) blocks inline style attributes in markup', () => {
    const offenders = Object.entries(sources)
      .filter(([, source]) => /\sstyle="/.test(source))
      .map(([file]) => file);
    expect(offenders).toEqual([]);
  });

  it('are all exported from the index', () => {
    const internal = new Set(['./FieldError.svelte', './Spinner.svelte']);
    const exported = new Set(Object.keys(library));
    const missing = Object.keys(sources)
      .filter((file) => !internal.has(file))
      .map((file) => file.replace('./', '').replace('.svelte', ''))
      .filter((name) => !exported.has(name));
    expect(missing).toEqual([]);
  });
});
