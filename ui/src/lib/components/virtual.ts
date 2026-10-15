// Geometry of the virtualized grid and list (decision 120: virtualization is mandatory; 149/151: every item in the
// grid has the same size, so one row height per width is all the virtualizer needs). Pure functions, no DOM.

export interface GeometryInput {
  /** Content width in px. */
  width: number;
  count: number;
  /** Grid: minimum column width; columns stretch to share the leftover. Omit for a single-column list. */
  minItemWidth?: number | undefined;
  /** Gap between rows and between columns, in px. */
  gap: number;
  /** Item height in px, or a function of the column width (catalog cards: 16:9 thumbnail plus the info block). */
  itemHeight: number | ((columnWidth: number) => number);
}

export interface GridGeometry {
  count: number;
  columns: number;
  columnWidth: number;
  rowHeight: number;
  /** Row height plus the gap. */
  rowStride: number;
  rows: number;
  totalHeight: number;
}

export interface VisibleRange {
  firstRow: number;
  /** Exclusive. */
  lastRow: number;
  /** First item index. */
  start: number;
  /** Exclusive. */
  end: number;
}

export function gridGeometry({ width, count, minItemWidth, gap, itemHeight }: GeometryInput): GridGeometry {
  const columns =
    minItemWidth === undefined ? 1 : Math.max(1, Math.floor((width + gap) / (minItemWidth + gap)));
  const columnWidth = Math.max(0, (width - gap * (columns - 1)) / columns);
  const rowHeight = typeof itemHeight === 'function' ? itemHeight(columnWidth) : itemHeight;
  const rows = Math.ceil(Math.max(0, count) / columns);
  return {
    count: Math.max(0, count),
    columns,
    columnWidth,
    rowHeight,
    rowStride: rowHeight + gap,
    rows,
    totalHeight: rows > 0 ? rows * rowHeight + (rows - 1) * gap : 0,
  };
}

/**
 * Rows that intersect the viewport, plus `overscan` rows on each side; `scrollTop` is clamped to the scrollable
 * range. Empty until the viewport has a height.
 */
export function visibleRange(geo: GridGeometry, scrollTop: number, viewportHeight: number, overscan: number) {
  if (viewportHeight <= 0 || geo.rows === 0 || geo.rowStride <= 0) {
    return { firstRow: 0, lastRow: 0, start: 0, end: 0 } satisfies VisibleRange;
  }
  const top = Math.min(Math.max(0, scrollTop), Math.max(0, geo.totalHeight - viewportHeight));
  const firstRow = Math.max(0, Math.floor(top / geo.rowStride) - overscan);
  const lastRow = Math.min(geo.rows, Math.ceil((top + viewportHeight) / geo.rowStride) + overscan);
  return {
    firstRow,
    lastRow,
    start: firstRow * geo.columns,
    end: Math.min(geo.count, lastRow * geo.columns),
  } satisfies VisibleRange;
}

/** The smallest change of `scrollTop` that shows the whole row of `index`. */
export function scrollTopFor(geo: GridGeometry, index: number, scrollTop: number, viewportHeight: number): number {
  const top = Math.floor(index / geo.columns) * geo.rowStride;
  const bottom = top + geo.rowHeight;
  if (top < scrollTop) {
    return top;
  }
  if (bottom > scrollTop + viewportHeight) {
    return Math.max(0, bottom - viewportHeight);
  }
  return scrollTop;
}

/**
 * The item a navigation key moves to, or null when the key is not for the grid. In a single-column list the left and
 * right arrows stay with the items (e.g. controls inside a row).
 */
export function moveIndex(geo: GridGeometry, index: number, key: string, pageRows: number): number | null {
  const last = geo.count - 1;
  if (last < 0) {
    return null;
  }
  const clamp = (value: number) => Math.min(last, Math.max(0, value));
  const lastRow = Math.floor(last / geo.columns);
  switch (key) {
    case 'ArrowRight':
      return geo.columns > 1 ? clamp(index + 1) : null;
    case 'ArrowLeft':
      return geo.columns > 1 ? clamp(index - 1) : null;
    case 'ArrowDown':
      if (index + geo.columns <= last) {
        return index + geo.columns;
      }
      return Math.floor(index / geo.columns) < lastRow ? last : index;
    case 'ArrowUp':
      return index - geo.columns >= 0 ? index - geo.columns : index;
    case 'PageDown':
      return clamp(index + Math.max(1, pageRows) * geo.columns);
    case 'PageUp':
      return clamp(index - Math.max(1, pageRows) * geo.columns);
    case 'Home':
      return 0;
    case 'End':
      return last;
    default:
      return null;
  }
}

/** The scroll position that keeps the first visible item at the top after the geometry changed (e.g. on resize). */
export function anchorScrollTop(before: GridGeometry, after: GridGeometry, scrollTop: number): number {
  if (before.rowStride <= 0) {
    return 0;
  }
  const firstItem = Math.floor(Math.max(0, scrollTop) / before.rowStride) * before.columns;
  return Math.floor(firstItem / after.columns) * after.rowStride;
}
