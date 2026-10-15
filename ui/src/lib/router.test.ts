import { describe, expect, it } from 'vitest';
import { ROUTES, routeOf } from './router.svelte';

describe('routes', () => {
  it('read the area from the address', () => {
    for (const route of ROUTES) {
      expect(routeOf(`#/${route}`)).toBe(route);
    }
    expect(routeOf('#/library/gb-655209')).toBe('library');
  });

  it('start in the catalog for anything else', () => {
    expect(routeOf('')).toBe('catalog');
    expect(routeOf('#/')).toBe('catalog');
    expect(routeOf('#/nowhere')).toBe('catalog');
  });
});
