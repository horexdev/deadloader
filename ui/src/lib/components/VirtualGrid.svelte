<!-- VirtualGrid: the virtualized catalog grid and library list (decision 120). Only the rows in view plus `overscan`
     rows exist in the DOM. Items all have the same size (149, 151), so a column count and a row height per width are
     enough: no measuring. Keyboard: the arrows, Home/End and PageUp/PageDown move focus between items, also to ones
     that were not rendered yet; only the active item is a tab stop (pass `active` on to its tabindex) and its focus
     target is the element marked `data-grid-focus`. The focused item stays rendered while it is scrolled away. -->
<script lang="ts" generics="T">
  import { flushSync, untrack, type Snippet } from 'svelte';
  import { focusableWithin } from './focus';
  import { anchorScrollTop, gridGeometry, moveIndex, scrollTopFor, visibleRange, type GridGeometry } from './virtual';

  interface Props {
    items: T[];
    /** Stable identity of an item, for keyed rendering. */
    key: (item: T, index: number) => string | number;
    /** Row height in px, or a function of the column width. */
    itemHeight: number | ((columnWidth: number) => number);
    /** Grid mode: minimum column width; without it this is a single-column list. */
    minItemWidth?: number;
    /** Gap between rows and columns in px. */
    gap?: number;
    /** Extra rows rendered above and below the viewport. */
    overscan?: number;
    /** Space around the items in px, e.g. so focus rings at the edges are not clipped. */
    padding?: number;
    /** Accessible name of the list. */
    label: string;
    /** The item that holds the tab stop. */
    activeIndex?: number;
    /** Renders one item; `active` tells whether it holds the tab stop. */
    item: Snippet<[T, number, { active: boolean }]>;
    /** The rendered item range changed (start inclusive, end exclusive), e.g. to fetch the next page early. */
    onrangechange?: (start: number, end: number) => void;
  }

  let {
    items,
    key,
    itemHeight,
    minItemWidth,
    gap = 0,
    overscan = 2,
    padding = 0,
    label,
    activeIndex = $bindable(0),
    item,
    onrangechange,
  }: Props = $props();

  let viewport: HTMLElement | undefined = $state();
  let width = $state(0);
  let height = $state(0);
  let scrollTop = $state(0);
  let hasFocus = $state(false);

  const geo = $derived(
    gridGeometry({ width: Math.max(0, width - 2 * padding), count: items.length, minItemWidth, gap, itemHeight }),
  );
  const range = $derived(visibleRange(geo, scrollTop - padding, height, overscan));
  const firstVisible = $derived(
    Math.min(Math.max(0, items.length - 1), Math.floor(Math.max(0, scrollTop - padding) / geo.rowStride) * geo.columns),
  );
  /** The tab stop: the active item while focus is inside or it is in view, otherwise the first visible item. */
  const tabStop = $derived(
    hasFocus || (activeIndex >= range.start && activeIndex < range.end) ? activeIndex : firstVisible,
  );
  const rendered = $derived.by(() => {
    const indices: number[] = [];
    for (let index = range.start; index < range.end; index += 1) {
      indices.push(index);
    }
    if (hasFocus && activeIndex < items.length && (activeIndex < range.start || activeIndex >= range.end)) {
      indices.push(activeIndex);
      indices.sort((a, b) => a - b);
    }
    return indices;
  });

  $effect(() => {
    if (items.length === 0) {
      activeIndex = 0;
    } else if (activeIndex > items.length - 1) {
      activeIndex = items.length - 1;
    }
  });

  $effect(() => {
    onrangechange?.(range.start, range.end);
  });

  $effect(() => {
    const element = viewport;
    if (!element) {
      return;
    }
    let previous: GridGeometry | undefined;
    const measure = () => {
      const nextWidth = element.clientWidth;
      const nextHeight = element.clientHeight;
      if (nextWidth === width && nextHeight === height) {
        return;
      }
      previous = width > 0 ? geo : undefined;
      width = nextWidth;
      height = nextHeight;
      if (previous && previous.columns !== geo.columns) {
        setScrollTop(anchorScrollTop(previous, geo, scrollTop - padding) + padding);
      }
    };
    // Measure once now without making this effect depend on the size it writes.
    untrack(measure);
    if (typeof ResizeObserver === 'undefined') {
      return;
    }
    const observer = new ResizeObserver(measure);
    observer.observe(element);
    return () => observer.disconnect();
  });

  function setScrollTop(top: number) {
    const max = Math.max(0, geo.totalHeight + 2 * padding - height);
    scrollTop = Math.min(Math.max(0, top), max);
    if (viewport) {
      viewport.scrollTop = scrollTop;
    }
  }

  /** Scrolls the least needed to show the item at `index`. */
  export function scrollToIndex(index: number): void {
    setScrollTop(scrollTopFor(geo, index, scrollTop - padding, height - 2 * padding) + padding);
  }

  /** Makes `index` the active item, scrolls it into view and focuses it. */
  export function focusIndex(index: number): void {
    activeIndex = index;
    hasFocus = true;
    scrollToIndex(index);
    flushSync();
    const cell = viewport?.querySelector<HTMLElement>(`[data-index="${index}"]`);
    if (cell) {
      (cell.querySelector<HTMLElement>('[data-grid-focus]') ?? focusableWithin(cell)[0] ?? cell).focus();
    }
  }

  function cellIndex(target: EventTarget | null): number | undefined {
    const cell = target instanceof Element ? target.closest<HTMLElement>('[data-index]') : null;
    return cell && viewport?.contains(cell) ? Number(cell.dataset['index']) : undefined;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.defaultPrevented || event.altKey || event.ctrlKey || event.metaKey) {
      return;
    }
    const from = cellIndex(event.target);
    if (from === undefined) {
      return;
    }
    const pageRows = Math.max(1, Math.floor(height / geo.rowStride));
    const next = moveIndex(geo, from, event.key, pageRows);
    if (next === null) {
      return;
    }
    event.preventDefault();
    focusIndex(next);
  }

  function handleFocusin(event: FocusEvent) {
    hasFocus = true;
    const index = cellIndex(event.target);
    if (index !== undefined) {
      activeIndex = index;
    }
  }

  function handleFocusout(event: FocusEvent) {
    if (!(event.relatedTarget instanceof Node) || !viewport?.contains(event.relatedTarget)) {
      hasFocus = false;
    }
  }
</script>

<!-- The list listens for the navigation keys its items bubble up (one listener instead of one per item). -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  bind:this={viewport}
  class="viewport"
  data-virtual-viewport
  role="list"
  aria-label={label}
  onscroll={() => (scrollTop = viewport?.scrollTop ?? 0)}
  onkeydown={handleKeydown}
  onfocusin={handleFocusin}
  onfocusout={handleFocusout}
>
  <div class="spacer" style:height={`${geo.totalHeight + 2 * padding}px`}>
    {#each rendered as index (key(items[index] as T, index))}
      {@const row = Math.floor(index / geo.columns)}
      {@const column = index % geo.columns}
      <div
        class="cell"
        role="listitem"
        aria-setsize={items.length}
        aria-posinset={index + 1}
        data-index={index}
        style:transform={`translate(${padding + column * (geo.columnWidth + gap)}px, ${padding + row * geo.rowStride}px)`}
        style:width={`${geo.columnWidth}px`}
        style:height={`${geo.rowHeight}px`}
      >
        {@render item(items[index] as T, index, { active: index === tabStop })}
      </div>
    {/each}
  </div>
</div>

<style>
  .viewport {
    position: relative;
    height: 100%;
    overflow-x: hidden;
    overflow-y: auto;
  }

  .spacer {
    position: relative;
  }

  .cell {
    position: absolute;
    top: 0;
    left: 0;
  }
</style>
