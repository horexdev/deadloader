import { describe, expect, it } from 'vitest';
import { anchorScrollTop, gridGeometry, moveIndex, scrollTopFor, visibleRange } from './virtual';

const card = (columnWidth: number) => Math.round((columnWidth * 9) / 16) + 112;

describe('gridGeometry', () => {
  it('fits as many minimum-width columns as the width allows and shares the leftover', () => {
    // 1280 px content: floor((1280 + 16) / 236) = 5 columns of (1280 - 4 × 16) / 5 = 243.2 px.
    const geo = gridGeometry({ width: 1280, count: 101, minItemWidth: 220, gap: 16, itemHeight: card });
    expect(geo.columns).toBe(5);
    expect(geo.columnWidth).toBeCloseTo(243.2);
    expect(geo.rowHeight).toBe(Math.round((243.2 * 9) / 16) + 112);
    expect(geo.rowStride).toBe(geo.rowHeight + 16);
    expect(geo.rows).toBe(21);
    expect(geo.totalHeight).toBe(21 * geo.rowHeight + 20 * 16);
  });

  it('keeps one column when the width is below the minimum and handles an empty list', () => {
    const narrow = gridGeometry({ width: 100, count: 3, minItemWidth: 220, gap: 16, itemHeight: card });
    expect(narrow.columns).toBe(1);
    expect(narrow.columnWidth).toBe(100);
    const empty = gridGeometry({ width: 1280, count: 0, minItemWidth: 220, gap: 16, itemHeight: card });
    expect(empty.rows).toBe(0);
    expect(empty.totalHeight).toBe(0);
  });

  it('is a single-column list without a minimum width', () => {
    const list = gridGeometry({ width: 900, count: 12, gap: 4, itemHeight: 56 });
    expect(list.columns).toBe(1);
    expect(list.columnWidth).toBe(900);
    expect(list.rowStride).toBe(60);
    expect(list.totalHeight).toBe(12 * 56 + 11 * 4);
  });
});

describe('visibleRange', () => {
  const list = gridGeometry({ width: 900, count: 10_000, gap: 4, itemHeight: 56 });

  it('covers the viewport plus the overscan rows', () => {
    // 600 px viewport at 60 px per row: rows 0..10, plus 2 overscan rows after.
    expect(visibleRange(list, 0, 600, 2)).toEqual({ firstRow: 0, lastRow: 12, start: 0, end: 12 });
    // Scrolled to row 100: two rows before, ten visible (one partly), two after.
    expect(visibleRange(list, 6000, 600, 2)).toEqual({ firstRow: 98, lastRow: 112, start: 98, end: 112 });
  });

  it('stops at the end of the list', () => {
    const end = visibleRange(list, list.totalHeight - 600, 600, 2);
    expect(end.end).toBe(10_000);
    expect(end.lastRow).toBe(10_000);
  });

  it('clamps a scroll position past the end', () => {
    expect(visibleRange(list, 10_000_000, 600, 2).end).toBe(10_000);
  });

  it('turns rows into item indices for grids', () => {
    const grid = gridGeometry({ width: 1280, count: 23, minItemWidth: 220, gap: 16, itemHeight: card });
    const range = visibleRange(grid, 0, 400, 1);
    expect(range.start).toBe(0);
    expect(range.end).toBe(Math.min(23, range.lastRow * 5));
  });

  it('renders nothing before the size is known', () => {
    expect(visibleRange(list, 0, 0, 2)).toEqual({ firstRow: 0, lastRow: 0, start: 0, end: 0 });
  });
});

describe('scrollTopFor', () => {
  const list = gridGeometry({ width: 900, count: 100, gap: 4, itemHeight: 56 });

  it('scrolls the least needed to show the item', () => {
    expect(scrollTopFor(list, 50, 0, 600)).toBe(50 * 60 + 56 - 600);
    expect(scrollTopFor(list, 2, 3000, 600)).toBe(120);
    expect(scrollTopFor(list, 52, 3000, 600)).toBe(3000);
  });
});

describe('moveIndex', () => {
  const grid = gridGeometry({ width: 1280, count: 23, minItemWidth: 220, gap: 16, itemHeight: card });
  const list = gridGeometry({ width: 900, count: 23, gap: 4, itemHeight: 56 });

  it('moves by one item sideways and by a row up and down in grids', () => {
    expect(moveIndex(grid, 7, 'ArrowRight', 3)).toBe(8);
    expect(moveIndex(grid, 7, 'ArrowLeft', 3)).toBe(6);
    expect(moveIndex(grid, 7, 'ArrowDown', 3)).toBe(12);
    expect(moveIndex(grid, 7, 'ArrowUp', 3)).toBe(2);
  });

  it('stops at the edges and lands on the last item from a partial last row', () => {
    expect(moveIndex(grid, 0, 'ArrowLeft', 3)).toBe(0);
    expect(moveIndex(grid, 22, 'ArrowRight', 3)).toBe(22);
    expect(moveIndex(grid, 2, 'ArrowUp', 3)).toBe(2);
    expect(moveIndex(grid, 19, 'ArrowDown', 3)).toBe(22);
  });

  it('jumps to the ends and by pages', () => {
    expect(moveIndex(grid, 7, 'Home', 3)).toBe(0);
    expect(moveIndex(grid, 7, 'End', 3)).toBe(22);
    expect(moveIndex(list, 2, 'PageDown', 10)).toBe(12);
    expect(moveIndex(list, 2, 'PageUp', 10)).toBe(0);
    expect(moveIndex(list, 20, 'PageDown', 10)).toBe(22);
  });

  it('leaves the left and right arrows to the items of a list, and ignores other keys', () => {
    expect(moveIndex(list, 3, 'ArrowRight', 10)).toBeNull();
    expect(moveIndex(list, 3, 'ArrowLeft', 10)).toBeNull();
    expect(moveIndex(list, 3, 'a', 10)).toBeNull();
  });
});

describe('anchorScrollTop', () => {
  it('keeps the first visible item at the top when the column count changes', () => {
    const wide = gridGeometry({ width: 1280, count: 200, minItemWidth: 220, gap: 16, itemHeight: card });
    const narrow = gridGeometry({ width: 600, count: 200, minItemWidth: 220, gap: 16, itemHeight: card });
    // Row 10 of the 5-column grid starts with item 50, which is in row 25 of the 2-column grid.
    expect(anchorScrollTop(wide, narrow, 10 * wide.rowStride)).toBe(25 * narrow.rowStride);
  });
});
