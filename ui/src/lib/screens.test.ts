import { describe, expect, it, vi } from 'vitest';
import type { Component } from 'svelte';
import { screenLoader } from './screens';

const Library = (() => null) as unknown as Component;
const Catalog = (() => null) as unknown as Component;

describe('the screen loader', () => {
  it('loads a screen only when it is first shown, and hands it over at once after that', async () => {
    const library = vi.fn(async () => ({ default: Library }));
    const catalog = vi.fn(async () => ({ default: Catalog }));
    const screens = screenLoader({
      './screens/library/Screen.svelte': library,
      './screens/catalog/Screen.svelte': catalog,
    });

    const first = screens.get('library');
    expect(first).toBeInstanceOf(Promise);
    expect(await first).toBe(Library);
    expect(screens.get('library')).toBe(Library);
    expect(library).toHaveBeenCalledTimes(1);
    expect(catalog).not.toHaveBeenCalled();
  });

  it('asks once while a screen is still loading', async () => {
    const library = vi.fn(async () => ({ default: Library }));
    const screens = screenLoader({ './screens/library/Screen.svelte': library });
    const [a, b] = [screens.get('library'), screens.get('library')];
    expect(await a).toBe(await b);
    expect(library).toHaveBeenCalledTimes(1);
  });

  it('has nothing for a route without a screen', () => {
    expect(screenLoader({}).get('profiles')).toBeUndefined();
  });

  it('tries again after a failed load', async () => {
    const library = vi
      .fn<() => Promise<{ default: Component }>>()
      .mockRejectedValueOnce(new Error('gone'))
      .mockResolvedValueOnce({ default: Library });
    const screens = screenLoader({ './screens/library/Screen.svelte': library });
    await expect(screens.get('library')).rejects.toThrow('gone');
    expect(await screens.get('library')).toBe(Library);
  });
});
