import { describe, expect, it } from 'vitest';
import { ROUTES, pathOf, routeOf } from './router.svelte';

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

describe('sub-routes', () => {
  it('read the parts after the area, e.g. a mod page in the catalog', () => {
    expect(pathOf('#/catalog/mod/655209')).toEqual(['mod', '655209']);
    expect(routeOf('#/catalog/mod/655209')).toBe('catalog');
    expect(pathOf('#/catalog')).toEqual([]);
    expect(pathOf('#/library/gb-655209')).toEqual(['gb-655209']);
    expect(pathOf('')).toEqual([]);
  });

  it('decode the parts', () => {
    expect(pathOf('#/library/local%2D1')).toEqual(['local-1']);
  });
});
