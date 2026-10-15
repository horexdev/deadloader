// The area on screen, kept in the address (`#/library`) so a reload or the back button lands where the user was.

export const ROUTES = ['catalog', 'library', 'profiles', 'downloads', 'activity', 'settings'] as const;
export type Route = (typeof ROUTES)[number];

/** The area an address names; anything else is the catalog, where the app starts. */
export function routeOf(hash: string): Route {
  const name = hash.replace(/^#\/?/, '').split('/')[0] ?? '';
  return (ROUTES as readonly string[]).includes(name) ? (name as Route) : 'catalog';
}

/** The parts of the address after the area, decoded: `#/catalog/mod/655209` → `['mod', '655209']`. */
export function pathOf(hash: string): string[] {
  const parts = hash.replace(/^#\/?/, '').split('/').slice(1).filter((part) => part !== '');
  return parts.map((part) => {
    try {
      return decodeURIComponent(part);
    } catch {
      return part;
    }
  });
}

export class Router {
  current: Route = $state(routeOf(typeof location === 'undefined' ? '' : location.hash));
  /** What follows the area in the address, e.g. the mod page's section and id in the catalog. */
  path: string[] = $state(pathOf(typeof location === 'undefined' ? '' : location.hash));

  constructor() {
    if (typeof window !== 'undefined') {
      window.addEventListener('hashchange', () => {
        this.current = routeOf(location.hash);
        this.path = pathOf(location.hash);
      });
    }
  }

  /** A place inside an area, e.g. `open('catalog', 'mod', '655209')` or `open('library', 'gb-655209')`. */
  open(route: Route, ...path: string[]): void {
    location.hash = `#/${[route, ...path.map(encodeURIComponent)].join('/')}`;
    this.current = route;
    this.path = path;
  }

  go(route: Route): void {
    location.hash = `#/${route}`;
    this.current = route;
    this.path = [];
  }
}

export const router = new Router();
