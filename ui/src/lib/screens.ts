// Screens load the first time they are shown, so the start parses only the shell and the first screen (decision 120:
// first frame ≤ 800 ms). A screen shown before comes back at once, without a blank frame.
import type { Component } from 'svelte';
import type { Route } from './router.svelte';

type Loaders = Record<string, () => Promise<{ default: Component }>>;

/** `loaders` is `import.meta.glob('./screens/*\/Screen.svelte')` from `src/`. */
export function screenLoader(loaders: Loaders) {
  const loaded = new Map<Route, Component>();
  const loading = new Map<Route, Promise<Component>>();
  return {
    /** The screen of `route`: the component when it was loaded before, a promise of it, or undefined if there is none. */
    get(route: Route): Component | Promise<Component> | undefined {
      const ready = loaded.get(route);
      if (ready) {
        return ready;
      }
      const load = loaders[`./screens/${route}/Screen.svelte`];
      if (!load) {
        return undefined;
      }
      let pending = loading.get(route);
      if (!pending) {
        pending = load()
          .then((module) => {
            loaded.set(route, module.default);
            return module.default;
          })
          .finally(() => loading.delete(route));
        loading.set(route, pending);
      }
      return pending;
    },
  };
}
