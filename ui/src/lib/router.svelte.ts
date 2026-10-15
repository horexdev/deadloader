// The area on screen, kept in the address (`#/library`) so a reload or the back button lands where the user was.

export const ROUTES = ['catalog', 'library', 'profiles', 'downloads', 'activity', 'settings'] as const;
export type Route = (typeof ROUTES)[number];

/** The area an address names; anything else is the catalog, where the app starts. */
export function routeOf(hash: string): Route {
  const name = hash.replace(/^#\/?/, '').split('/')[0] ?? '';
  return (ROUTES as readonly string[]).includes(name) ? (name as Route) : 'catalog';
}

/** What follows the area in the address (`#/library/<mod id>` opens that mod), or null. */
export function paramOf(hash: string): string | null {
  const rest = hash.replace(/^#\/?/, '').split('/').slice(1).join('/');
  return rest === '' ? null : decodeURIComponent(rest);
}

export class Router {
  current: Route = $state(routeOf(typeof location === 'undefined' ? '' : location.hash));
  param: string | null = $state(paramOf(typeof location === 'undefined' ? '' : location.hash));

  constructor() {
    if (typeof window !== 'undefined') {
      window.addEventListener('hashchange', () => {
        this.current = routeOf(location.hash);
        this.param = paramOf(location.hash);
      });
    }
  }

  /** Another area, optionally with a parameter, e.g. `go('library', modId)` opens that mod's details. */
  go(route: Route, param?: string): void {
    location.hash = param === undefined ? `#/${route}` : `#/${route}/${encodeURIComponent(param)}`;
    this.current = route;
    this.param = param ?? null;
  }
}

export const router = new Router();
