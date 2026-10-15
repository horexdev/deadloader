import { flushSync } from 'svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import GridHarness from '../test/GridHarness.svelte';
import { all, fire, keydown, one, render } from '../test/render';
import { CARD_MIN_W, GRID_GAP, LIST_GAP, ROW_H, catalogCardHeight } from './geometry';

// happy-dom has no layout: give the viewport a size and drive ResizeObserver by hand.
const size = { width: 1280, height: 800 };
const observers: ResizeObserverCallback[] = [];

beforeEach(() => {
  size.width = 1280;
  size.height = 800;
  observers.length = 0;
  vi.stubGlobal(
    'ResizeObserver',
    class {
      constructor(callback: ResizeObserverCallback) {
        observers.push(callback);
      }
      observe() {}
      disconnect() {}
    },
  );
  vi.spyOn(HTMLElement.prototype, 'clientWidth', 'get').mockImplementation(function (this: HTMLElement) {
    return this.hasAttribute('data-virtual-viewport') ? size.width : 0;
  });
  vi.spyOn(HTMLElement.prototype, 'clientHeight', 'get').mockImplementation(function (this: HTMLElement) {
    return this.hasAttribute('data-virtual-viewport') ? size.height : 0;
  });
});

afterEach(() => {
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

function resize(width: number, height: number) {
  size.width = width;
  size.height = height;
  for (const callback of observers) {
    callback([], {} as ResizeObserver);
  }
  flushSync();
}

function scrollTo(viewport: HTMLElement, top: number) {
  viewport.scrollTop = top;
  fire(viewport, 'scroll', { bubbles: false });
}

const catalog = { itemHeight: catalogCardHeight, minItemWidth: CARD_MIN_W, gap: GRID_GAP, overscan: 2 };
const library = { itemHeight: ROW_H, gap: LIST_GAP, overscan: 3 };

const cells = (root: ParentNode) => all(root, '[role="listitem"]');
const indices = (root: ParentNode) => cells(root).map((cell) => Number(cell.dataset['index']));

describe('VirtualGrid', () => {
  it('is a labelled list whose items know their position in the whole set', () => {
    const { target } = render(GridHarness, { count: 10_000, ...library });
    const list = one(target, '[role="list"]');
    expect(list.getAttribute('aria-label')).toBe('Mods');
    const first = cells(target)[0] as HTMLElement;
    expect(first.getAttribute('aria-setsize')).toBe('10000');
    expect(first.getAttribute('aria-posinset')).toBe('1');
  });

  it('keeps the DOM bounded for 10 000 catalog cards wherever it is scrolled', () => {
    const { target } = render(GridHarness, { count: 10_000, ...catalog });
    const viewport = one(target, '[data-virtual-viewport]');
    // 1280 px: 5 columns of 243.2 px, rows of 249 + 16 px; 800 px shows 4 rows, plus 2 overscan rows after.
    const stride = catalogCardHeight(243.2) + GRID_GAP;
    const bound = (Math.ceil(800 / stride) + 1 + 2 * 2) * 5;
    for (const top of [0, 50_000, 250_000, 10_000_000]) {
      scrollTo(viewport, top);
      const count = cells(target).length;
      expect(count).toBeGreaterThan(0);
      expect(count).toBeLessThanOrEqual(bound);
      expect(target.querySelectorAll('*').length).toBeLessThan(bound * 4 + 10);
    }
    expect(indices(target).at(-1)).toBe(9_999);
  });

  it('sizes the scroll area for every item and positions rendered ones by row and column', () => {
    const { target } = render(GridHarness, { count: 23, ...catalog });
    const spacer = one(target, '[data-virtual-viewport] > div');
    const rowHeight = catalogCardHeight(243.2);
    expect(spacer.style.height).toBe(`${5 * rowHeight + 4 * GRID_GAP}px`);
    const seventh = one(target, '[data-index="7"]');
    expect(seventh.style.transform).toBe(`translate(${2 * (243.2 + GRID_GAP)}px, ${rowHeight + GRID_GAP}px)`);
    expect(seventh.style.width).toBe('243.2px');
    expect(seventh.style.height).toBe(`${rowHeight}px`);
  });

  it('reflows the columns when the width changes, keeping the first visible item in view', () => {
    const { target } = render(GridHarness, { count: 1000, ...catalog });
    const viewport = one(target, '[data-virtual-viewport]');
    const wideStride = catalogCardHeight(243.2) + GRID_GAP;
    scrollTo(viewport, 20 * wideStride);
    expect(indices(target)).toContain(100);
    // 600 px: 2 columns of 292 px.
    resize(600, 800);
    const narrowStride = catalogCardHeight(292) + GRID_GAP;
    expect(viewport.scrollTop).toBe(50 * narrowStride);
    expect(one(target, '[data-index="100"]').style.width).toBe('292px');
  });

  it('moves focus with the arrow keys into items that were not rendered yet', () => {
    const props = $state({ count: 10_000, ...catalog, activeIndex: 0 });
    const { target } = render(GridHarness, props);
    const viewport = one(target, '[data-virtual-viewport]');
    const first = one(target, '[data-index="0"] .card');
    first.focus();
    keydown(first, 'ArrowRight');
    expect(document.activeElement).toBe(one(target, '[data-index="1"] .card'));
    expect(props.activeIndex).toBe(1);
    keydown(document.activeElement as HTMLElement, 'ArrowDown');
    expect(document.activeElement).toBe(one(target, '[data-index="6"] .card'));
    keydown(document.activeElement as HTMLElement, 'End');
    expect(document.activeElement).toBe(one(target, '[data-index="9999"] .card'));
    expect(viewport.scrollTop).toBeGreaterThan(0);
    expect(cells(target).length).toBeLessThan(60);
    keydown(document.activeElement as HTMLElement, 'Home');
    expect(document.activeElement).toBe(one(target, '[data-index="0"] .card'));
    expect(viewport.scrollTop).toBe(0);
  });

  it('pages through a list with PageDown and PageUp', () => {
    const { target } = render(GridHarness, { count: 10_000, ...library });
    const first = one(target, '[data-index="0"] .card');
    first.focus();
    // 800 px at 60 px per row: a page is 13 whole rows.
    keydown(first, 'PageDown');
    expect(document.activeElement).toBe(one(target, '[data-index="13"] .card'));
    keydown(document.activeElement as HTMLElement, 'PageUp');
    expect(document.activeElement).toBe(one(target, '[data-index="0"] .card'));
  });

  it('gives only the active item a tab stop, and the first visible one while focus is elsewhere', () => {
    const props = $state({ count: 10_000, ...library, activeIndex: 0 });
    const { target } = render(GridHarness, props);
    const viewport = one(target, '[data-virtual-viewport]');
    const stops = () => all(target, '[tabindex="0"]').map((el) => el.closest<HTMLElement>('[data-index]')?.dataset['index']);
    expect(new Set(stops())).toEqual(new Set(['0']));
    scrollTo(viewport, 600 * 60);
    expect(new Set(stops())).toEqual(new Set(['600']));
  });

  it('keeps the focused item rendered while it is scrolled out of view', () => {
    const { target } = render(GridHarness, { count: 10_000, ...library });
    const viewport = one(target, '[data-virtual-viewport]');
    const first = one(target, '[data-index="0"] .card');
    first.focus();
    fire(first, 'focusin');
    scrollTo(viewport, 5000 * 60);
    expect(document.activeElement).toBe(first);
    expect(first.isConnected).toBe(true);
    expect(indices(target)[0]).toBe(0);
  });

  it('leaves keys that an item handled itself alone', () => {
    const { target } = render(GridHarness, { count: 100, ...library });
    const first = one(target, '[data-index="0"] .card');
    first.addEventListener('keydown', (event) => event.preventDefault());
    first.focus();
    keydown(first, 'ArrowDown');
    expect(document.activeElement).toBe(first);
  });

  it('reports the rendered range for prefetching', () => {
    const onrangechange = vi.fn();
    const { target } = render(GridHarness, { count: 10_000, ...library, onrangechange });
    expect(onrangechange).toHaveBeenLastCalledWith(0, 17);
    scrollTo(one(target, '[data-virtual-viewport]'), 6000);
    expect(onrangechange).toHaveBeenLastCalledWith(97, 117);
  });

  it('scrolls an item into view on request', () => {
    const { target, instance } = render(GridHarness, { count: 10_000, ...library });
    instance.scrollToIndex(5000);
    flushSync();
    expect(indices(target)).toContain(5000);
  });

  it('clamps the active item when the list shrinks', () => {
    const props = $state({ count: 100, ...library, activeIndex: 90 });
    render(GridHarness, props);
    props.count = 10;
    flushSync();
    expect(props.activeIndex).toBe(9);
  });

  it('adds padding around the items', () => {
    const { target } = render(GridHarness, { count: 10, ...library, padding: 4 });
    const spacer = one(target, '[data-virtual-viewport] > div');
    expect(spacer.style.height).toBe(`${10 * ROW_H + 9 * LIST_GAP + 8}px`);
    expect(one(target, '[data-index="1"]').style.transform).toBe(`translate(4px, ${4 + ROW_H + LIST_GAP}px)`);
    expect(one(target, '[data-index="1"]').style.width).toBe(`${1280 - 8}px`);
  });
});
