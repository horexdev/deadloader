// The area on screen, kept in the address (`#/library`) so a reload or the back button lands where the user was.

export const ROUTES = ['catalog', 'library', 'profiles', 'downloads', 'activity', 'settings'] as const;
export type Route = (typeof ROUTES)[number];

/** The area an address names; anything else is the catalog, where the app starts. */
export function routeOf(hash: string): Route {
  const name = hash.replace(/^#\/?/, '').split('/')[0] ?? '';
  return (ROUTES as readonly string[]).includes(name) ? (name as Route) : 'catalog';
}

export class Router {
  current: Route = $state(routeOf(typeof location === 'undefined' ? '' : location.hash));

  constructor() {
    if (typeof window !== 'undefined') {
      window.addEventListener('hashchange', () => {
        this.current = routeOf(location.hash);
      });
    }
  }

  go(route: Route): void {
    location.hash = `#/${route}`;
    this.current = route;
  }
}

export const router = new Router();
