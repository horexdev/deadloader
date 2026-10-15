import { flushSync, mount, unmount, type Component } from 'svelte';

type Mounted = ReturnType<typeof mount>;

const mounted = new Set<Mounted>();

export interface Rendered<Exports> {
  /** The element the component was mounted into. */
  target: HTMLElement;
  /** The component's exports (functions declared with `export function`). */
  instance: Exports;
}

// Svelte's own `mount` constrains props this way; `unknown` would reject component prop interfaces.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyProps = Record<string, any>;

/**
 * Mounts a component into a fresh element attached to the document and flushes the first render. Pass a `$state`
 * object as props (from a `.svelte.test.ts` file) to change props later; call `flushSync()` after changing them.
 */
export function render<Props extends AnyProps, Exports extends AnyProps>(
  component: Component<Props, Exports, string>,
  props: NoInfer<Props>,
): Rendered<Exports> {
  const target = document.createElement('div');
  document.body.append(target);
  const instance = mount(component, { target, props });
  mounted.add(instance);
  flushSync();
  return { target, instance };
}

/** Unmounts one component that `render` mounted, before the test ends. */
export function destroy(instance: object): void {
  void unmount(instance as Mounted);
  mounted.delete(instance as Mounted);
  flushSync();
}

/** Unmounts everything `render` mounted and empties the document. Runs after every test (see setup.ts). */
export function cleanup(): void {
  for (const instance of mounted) {
    void unmount(instance);
  }
  mounted.clear();
  flushSync();
  document.body.replaceChildren();
}

/** Dispatches a keydown on `element` (bubbling, cancelable) and flushes. Returns false when it was prevented. */
export function keydown(element: Element, key: string, init: KeyboardEventInit = {}): boolean {
  const accepted = element.dispatchEvent(new KeyboardEvent('keydown', { key, bubbles: true, cancelable: true, ...init }));
  flushSync();
  return accepted;
}

/** Clicks `element` and flushes. */
export function click(element: Element): void {
  (element as HTMLElement).click();
  flushSync();
}

/** Dispatches a bubbling event of `type` on `element` and flushes. */
export function fire(element: Element | Window, type: string, init: EventInit = {}): void {
  element.dispatchEvent(new Event(type, { bubbles: true, cancelable: true, ...init }));
  flushSync();
}

/** Returns the single element matching `selector` inside `root`, failing the test when there is none. */
export function one<E extends Element = HTMLElement>(root: ParentNode, selector: string): E {
  const found = root.querySelector<E>(selector);
  if (!found) {
    throw new Error(`no element matches ${selector}`);
  }
  return found;
}

/** Returns all elements matching `selector` inside `root`. */
export function all<E extends Element = HTMLElement>(root: ParentNode, selector: string): E[] {
  return [...root.querySelectorAll<E>(selector)];
}
