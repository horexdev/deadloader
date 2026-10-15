import { createRawSnippet, flushSync } from 'svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { fire, keydown, one, render } from '../test/render';
import PathTooltip from './PathTooltip.svelte';
import Tooltip from './Tooltip.svelte';
import { splitPath, tooltip } from './tooltip';

describe('Tooltip', () => {
  it('renders the label with an optional second line', () => {
    const { target } = render(Tooltip, { id: 'tt1', text: 'Install needs a connection', sub: 'Go online to install.' });
    const tip = one(target, '.dl-tooltip');
    expect(tip.getAttribute('role')).toBe('tooltip');
    expect(tip.id).toBe('tt1');
    expect(tip.classList).toContain('dl-tooltip--multiline');
    expect(one(tip, '.dl-tooltip__sub').textContent).toBe('Go online to install.');
  });
});

describe('tooltip attachment', () => {
  let anchor: HTMLButtonElement;
  let detach: ReturnType<ReturnType<typeof tooltip>>;

  beforeEach(() => {
    vi.useFakeTimers();
    anchor = document.createElement('button');
    anchor.setAttribute('aria-describedby', 'other');
    document.body.append(anchor);
    anchor.getBoundingClientRect = () => new DOMRect(100, 200, 40, 32);
  });

  afterEach(() => {
    if (typeof detach === 'function') detach();
    vi.useRealTimers();
  });

  const tip = () => document.querySelector<HTMLElement>('body > .dl-tooltip');

  it('appears after 500 ms of hover and leaves with the pointer', () => {
    detach = tooltip({ text: 'Library' })(anchor);
    fire(anchor, 'pointerenter', { bubbles: false });
    vi.advanceTimersByTime(499);
    expect(tip()).toBeNull();
    vi.advanceTimersByTime(1);
    flushSync();
    expect(tip()?.textContent).toBe('Library');
    expect(anchor.getAttribute('aria-describedby')).toBe(`other ${tip()?.id}`);
    fire(anchor, 'pointerleave', { bubbles: false });
    expect(tip()).toBeNull();
    expect(anchor.getAttribute('aria-describedby')).toBe('other');
  });

  it('appears at once on keyboard focus and hides on Esc and blur', () => {
    detach = tooltip({ text: 'Library' })(anchor);
    anchor.focus();
    flushSync();
    expect(tip()).not.toBeNull();
    keydown(anchor, 'Escape');
    expect(tip()).toBeNull();
    anchor.blur();
    anchor.focus();
    flushSync();
    expect(tip()).not.toBeNull();
    anchor.blur();
    flushSync();
    expect(tip()).toBeNull();
  });

  it('sits 8 px above the anchor, centred, and flips below when there is no room', () => {
    // The tooltip measures 60 × 24; the anchor keeps its own rectangle.
    const measure = vi.spyOn(HTMLElement.prototype, 'getBoundingClientRect').mockReturnValue(new DOMRect(0, 0, 60, 24));
    try {
      detach = tooltip({ text: 'Library' })(anchor);
      anchor.focus();
      flushSync();
      expect(tip()?.style.top).toBe('168px');
      expect(tip()?.style.left).toBe('90px');
      anchor.blur();
      anchor.getBoundingClientRect = () => new DOMRect(100, 10, 40, 32);
      anchor.focus();
      flushSync();
      expect(tip()?.style.top).toBe('50px');
    } finally {
      measure.mockRestore();
    }
  });

  it('does nothing without text', () => {
    detach = tooltip({ text: '' })(anchor);
    anchor.focus();
    flushSync();
    expect(tip()).toBeNull();
  });

  it('removes the tooltip when detached', () => {
    const cleanup = tooltip({ text: 'Library' })(anchor);
    anchor.focus();
    flushSync();
    expect(tip()).not.toBeNull();
    if (typeof cleanup === 'function') cleanup();
    expect(tip()).toBeNull();
  });
});

describe('splitPath', () => {
  it('splits at the last separator, keeping it in the head', () => {
    expect(splitPath('D:\\SteamLibrary\\steamapps\\common\\Deadlock')).toEqual({
      head: 'D:\\SteamLibrary\\steamapps\\common\\',
      tail: 'Deadlock',
    });
    expect(splitPath('/home/me/mods/haze.vpk')).toEqual({ head: '/home/me/mods/', tail: 'haze.vpk' });
  });

  it('keeps a trailing separator with the last segment', () => {
    expect(splitPath('C:\\Users\\…\\')).toEqual({ head: 'C:\\Users\\', tail: '…\\' });
  });

  it('keeps a path without separators whole', () => {
    expect(splitPath('Deadlock')).toEqual({ head: '', tail: 'Deadlock' });
  });
});

describe('PathTooltip', () => {
  const path = 'D:\\SteamLibrary\\steamapps\\common\\Deadlock';

  it('shows the folder part and the last segment, focusable, described by the full path', () => {
    const { target } = render(PathTooltip, { path });
    const text = one(target, '.dl-path .dl-path__text');
    expect(text.tabIndex).toBe(0);
    expect(one(text, '.dl-path__head').textContent).toBe('D:\\SteamLibrary\\steamapps\\common\\');
    expect(one(text, '.dl-path__tail').textContent).toBe('Deadlock');
    const tip = one(target, '.dl-path__tip');
    expect(tip.getAttribute('role')).toBe('tooltip');
    expect(tip.textContent).toBe(path);
    expect(text.getAttribute('aria-describedby')).toBe(tip.id);
  });

  it('hides the tooltip on Esc until the path is left', () => {
    const { target } = render(PathTooltip, { path });
    const text = one(target, '.dl-path__text');
    const tip = one(target, '.dl-path__tip');
    keydown(text, 'Escape');
    expect(tip.style.display).toBe('none');
    fire(text, 'focusout');
    expect(tip.style.display).toBe('');
  });

  it('shows an error message under the path', () => {
    const { target } = render(PathTooltip, { path, error: 'This folder no longer exists.' });
    expect(one(target, '.dl-path').classList).toContain('is-error');
    const error = one(target, '.dl-field__error');
    expect(error.textContent.trim()).toBe('This folder no longer exists.');
    expect(one(target, '.dl-path__text').getAttribute('aria-describedby')).toBe(
      `${one(target, '.dl-path__tip').id} ${error.id}`,
    );
  });

  it('renders actions next to the path, not inside the tooltip', () => {
    const actions = createRawSnippet(() => ({ render: () => '<button>Copy</button>' }));
    const { target } = render(PathTooltip, { path, actions });
    expect(one(target, '.dl-path__tip').querySelector('button')).toBeNull();
    expect(target.querySelector('button')?.textContent).toBe('Copy');
  });
});
