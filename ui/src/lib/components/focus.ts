// Focus helpers for modal layers (Dialog, the Sidebar overlay).

const FOCUSABLE = [
  'a[href]',
  'button:not([disabled])',
  'input:not([disabled]):not([type="hidden"])',
  'select:not([disabled])',
  'textarea:not([disabled])',
  '[tabindex]:not([tabindex="-1"])',
].join(',');

/** Elements inside `root` that Tab can reach, in document order. */
export function focusableWithin(root: ParentNode): HTMLElement[] {
  return [...root.querySelectorAll<HTMLElement>(FOCUSABLE)].filter(
    (element) => element.tabIndex >= 0 && !element.closest('[hidden], [inert]'),
  );
}

/**
 * Keeps Tab and Shift+Tab inside `root`: from the last element Tab wraps to the first, and back. Call it from the
 * layer's keydown handler; it does nothing for other keys.
 */
export function trapTab(event: KeyboardEvent, root: HTMLElement): void {
  if (event.key !== 'Tab') {
    return;
  }
  const items = focusableWithin(root);
  const first = items[0];
  const last = items.at(-1);
  if (!first || !last) {
    event.preventDefault();
    root.focus();
    return;
  }
  const active = document.activeElement;
  const outside = !(active instanceof Node) || !root.contains(active) || active === root;
  if (event.shiftKey && (active === first || outside)) {
    event.preventDefault();
    last.focus();
  } else if (!event.shiftKey && (active === last || outside)) {
    event.preventDefault();
    first.focus();
  }
}

/** The element to focus first: one marked `data-autofocus`, else the first focusable one, else `root` itself. */
export function initialFocus(root: HTMLElement): HTMLElement {
  return root.querySelector<HTMLElement>('[data-autofocus]') ?? focusableWithin(root)[0] ?? root;
}
