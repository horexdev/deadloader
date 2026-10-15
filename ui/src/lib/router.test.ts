import { describe, expect, it } from 'vitest';
import { ROUTES, Router, paramOf, routeOf } from './router.svelte';

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

describe('route parameters', () => {
  it('read what follows the area, decoded', () => {
    expect(paramOf('#/library/gb-655209')).toBe('gb-655209');
    expect(paramOf('#/library/local%201')).toBe('local 1');
    expect(paramOf('#/library')).toBeNull();
    expect(paramOf('#/library/')).toBeNull();
    expect(paramOf('')).toBeNull();
  });

  it('are written into the address by go', () => {
    const router = new Router();
    router.go('library', 'gb-1');
    expect(location.hash).toBe('#/library/gb-1');
    expect(router.current).toBe('library');
    expect(router.param).toBe('gb-1');
    router.go('library');
    expect(location.hash).toBe('#/library');
    expect(router.param).toBeNull();
  });
});
