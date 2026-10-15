import { describe, expect, it } from 'vitest';
import { firstLoad, overBudget, type Chunks } from './budget';

function chunk(code: string, isEntry = false, imports: string[] = [], dynamicImports: string[] = []) {
  return { type: 'chunk' as const, code, isEntry, imports, dynamicImports };
}

describe('the first-load budget', () => {
  it('counts the entry and every chunk it imports statically, each once, but not the ones it loads later', () => {
    const bundle: Chunks = {
      'index.js': chunk('x'.repeat(100), true, ['shared.js', 'shell.js'], ['catalog.js']),
      'shell.js': chunk('x'.repeat(40), false, ['shared.js']),
      'shared.js': chunk('x'.repeat(10)),
      'catalog.js': chunk('x'.repeat(500), false, ['shared.js']),
      'style.css': { type: 'asset' as const },
    };
    expect(firstLoad(bundle)).toEqual({ bytes: 150, files: ['index.js', 'shared.js', 'shell.js'] });
  });

  it('counts bytes, not characters', () => {
    expect(firstLoad({ 'index.js': chunk('é', true) }).bytes).toBe(2);
  });

  it('says what is over the budget and by how much', () => {
    const bundle: Chunks = { 'index.js': chunk('x'.repeat(2048), true) };
    expect(overBudget(bundle, 4096)).toBeNull();
    expect(overBudget(bundle, 1024)).toBe(
      'the first load is 2.0 kB (index.js), over its budget of 1.0 kB: load the new code when it is needed (import())',
    );
  });
});
