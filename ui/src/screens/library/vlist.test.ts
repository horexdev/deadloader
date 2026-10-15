import { describe, expect, it } from 'vitest';
import { dropIndex, indexAt, layout, scrollFor, visible } from './vlist';

const none = new Map<number, number>();

describe('library list virtualization', () => {
  it('stacks rows with gaps and taller measured blocks', () => {
    const at = layout(4, new Map([[1, 150]]), 56, 4);
    expect(at.tops).toEqual([0, 60, 214, 274]);
    expect(at.total).toBe(330);
    expect(layout(0, none, 56, 4)).toEqual({ tops: [], total: 0 });
  });

  it('renders only what intersects the viewport, plus overscan', () => {
    const at = layout(10_000, none, 56, 4);
    expect(visible(at, 6000, 600, 2)).toEqual({ start: 98, end: 112 });
    expect(visible(at, 0, 0, 2)).toEqual({ start: 0, end: 2 });
  });

  it('finds the item at a height', () => {
    const at = layout(3, none, 56, 4);
    expect(indexAt(at, 0)).toBe(0);
    expect(indexAt(at, 59)).toBe(0);
    expect(indexAt(at, 60)).toBe(1);
    expect(indexAt(at, 9999)).toBe(2);
  });

  it('drops a dragged row where the pointer passes the middle of the others', () => {
    const at = layout(4, none, 56, 4);
    // Row 3 dragged up to above row 1 (row 1's middle is at 88).
    expect(dropIndex(at, none, 56, 3, 80)).toBe(1);
    expect(dropIndex(at, none, 56, 3, 0)).toBe(0);
    expect(dropIndex(at, none, 56, 0, 500)).toBe(3);
  });

  it('scrolls just enough to show a row', () => {
    const at = layout(20, none, 56, 4);
    expect(scrollFor(at, none, 56, 0, 300, 200)).toBe(0);
    expect(scrollFor(at, none, 56, 10, 0, 200)).toBe(456);
    expect(scrollFor(at, none, 56, 2, 100, 200)).toBe(100);
  });
});
