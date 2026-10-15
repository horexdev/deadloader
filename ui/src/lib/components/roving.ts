// Keyboard movement over a row of items (tabs, segments, options) that may contain disabled ones.

export interface Steppable {
  disabled?: boolean | undefined;
}

/**
 * Index of the next enabled item from `from` in direction `delta` (±1), or -1 when none is enabled. With `from` = -1
 * the search starts from the edge the direction points away from. Without `wrap` it stops at the last enabled item.
 */
export function stepEnabled(items: readonly Steppable[], from: number, delta: 1 | -1, wrap = true): number {
  const count = items.length;
  if (count === 0) {
    return -1;
  }
  let index = from < 0 ? (delta > 0 ? -1 : count) : from;
  for (let tries = 0; tries < count; tries += 1) {
    index += delta;
    if (index < 0 || index >= count) {
      if (!wrap) {
        return items[from]?.disabled ? -1 : from;
      }
      index = (index + count) % count;
    }
    if (!items[index]?.disabled) {
      return index;
    }
  }
  return -1;
}

/** Index of the first or last enabled item, or -1. */
export function edgeEnabled(items: readonly Steppable[], edge: 'first' | 'last'): number {
  return edge === 'first' ? stepEnabled(items, -1, 1, false) : stepEnabled(items, -1, -1, false);
}

/**
 * Type-ahead: the item whose label starts with `typed` (case-insensitive). The active item wins while it still
 * matches, otherwise the search continues after it and wraps. Returns -1 when nothing matches.
 */
export function typeahead(
  items: readonly (Steppable & { label: string })[],
  active: number,
  typed: string,
): number {
  const needle = typed.toLocaleLowerCase();
  const matches = (index: number) => {
    const item = items[index];
    return item !== undefined && !item.disabled && item.label.toLocaleLowerCase().startsWith(needle);
  };
  if (needle.length > 1 && active >= 0 && matches(active)) {
    return active;
  }
  for (let offset = 1; offset <= items.length; offset += 1) {
    const index = (Math.max(active, -1) + offset + items.length) % items.length;
    if (matches(index)) {
      return index;
    }
  }
  return -1;
}
