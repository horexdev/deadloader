// The library list's virtualization (decision 120): rows are `--row-h` high; an expanded block (a mod with its
// variants or add-ons) is taller and its height is measured once rendered. Pure functions, no DOM.

export interface Layout {
  /** Top of each item in px. */
  tops: number[];
  total: number;
}

/** Item `i` is `heights[i]` high (or `fallback` while unmeasured), with `gap` between items. */
export function layout(count: number, heights: ReadonlyMap<number, number>, fallback: number, gap: number): Layout {
  const tops: number[] = [];
  let y = 0;
  for (let i = 0; i < count; i += 1) {
    tops.push(y);
    y += (heights.get(i) ?? fallback) + gap;
  }
  return { tops, total: count > 0 ? y - gap : 0 };
}

/** The items intersecting the viewport, plus `overscan` on each side (end exclusive). */
export function visible(at: Layout, scrollTop: number, height: number, overscan: number): { start: number; end: number } {
  const count = at.tops.length;
  if (count === 0 || height <= 0) {
    return { start: 0, end: Math.min(count, overscan) };
  }
  const first = Math.max(0, indexAt(at, scrollTop));
  let last = first;
  while (last < count - 1 && (at.tops[last + 1] ?? Infinity) < scrollTop + height) {
    last += 1;
  }
  return { start: Math.max(0, first - overscan), end: Math.min(count, last + 1 + overscan) };
}

/** The item at height `y` (the last one whose top is at or above it). */
export function indexAt(at: Layout, y: number): number {
  let lo = 0;
  let hi = at.tops.length - 1;
  if (hi < 0) {
    return -1;
  }
  while (lo < hi) {
    const mid = Math.ceil((lo + hi) / 2);
    if ((at.tops[mid] ?? 0) <= y) {
      lo = mid;
    } else {
      hi = mid - 1;
    }
  }
  return lo;
}

/**
 * Where a row dragged by the pointer lands: the index it takes in the list without it, from the pointer's height
 * (the middle of each other row decides above or below).
 */
export function dropIndex(at: Layout, heights: ReadonlyMap<number, number>, fallback: number, from: number, y: number): number {
  let index = 0;
  for (let i = 0; i < at.tops.length; i += 1) {
    if (i === from) {
      continue;
    }
    const middle = (at.tops[i] ?? 0) + (heights.get(i) ?? fallback) / 2;
    if (y > middle) {
      index += 1;
    }
  }
  return index;
}

/** The scroll position that shows all of item `i`. */
export function scrollFor(at: Layout, heights: ReadonlyMap<number, number>, fallback: number, i: number, scrollTop: number, height: number): number {
  const top = at.tops[i] ?? 0;
  const bottom = top + (heights.get(i) ?? fallback);
  if (top < scrollTop) {
    return top;
  }
  if (bottom > scrollTop + height) {
    return Math.max(0, bottom - height);
  }
  return scrollTop;
}
