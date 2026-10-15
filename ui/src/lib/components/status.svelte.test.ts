// Display-only components: Badge, Count, Banner, EmptyState, Skeleton, ProgressBar.
import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
import LibraryBig from '@lucide/svelte/icons/library-big';
import WifiOff from '@lucide/svelte/icons/wifi-off';
import { createRawSnippet, flushSync } from 'svelte';
import { describe, expect, it } from 'vitest';
import { all, one, render } from '../test/render';
import Badge from './Badge.svelte';
import Banner from './Banner.svelte';
import CardSkeleton from './CardSkeleton.svelte';
import Count from './Count.svelte';
import EmptyState from './EmptyState.svelte';
import ProgressBar from './ProgressBar.svelte';
import RowSkeleton from './RowSkeleton.svelte';
import Skeleton from './Skeleton.svelte';

describe('Badge', () => {
  it('shows an icon and a word, neutral by default', () => {
    const { target } = render(Badge, { icon: CircleArrowUp, label: 'Linked' });
    const badge = one(target, 'span.dl-badge');
    expect(badge.className).toBe('dl-badge');
    expect(badge.querySelector('svg.dl-icon')).not.toBeNull();
    expect(badge.textContent.trim()).toBe('Linked');
  });

  it.each(['accent', 'info', 'warning', 'danger', 'success', 'outline'] as const)('maps the %s variant', (variant) => {
    const { target } = render(Badge, { icon: CircleArrowUp, label: 'x', variant });
    expect(one(target, '.dl-badge').classList).toContain(`dl-badge--${variant}`);
  });
});

describe('Count', () => {
  it('renders the accent pill, or the muted one', () => {
    const accent = render(Count, { value: '3' });
    const muted = render(Count, { value: '1 204', muted: true });
    expect(one(accent.target, '.dl-count').className).toBe('dl-count');
    expect(one(muted.target, '.dl-count').classList).toContain('dl-count--muted');
    expect(one(muted.target, '.dl-count').textContent).toBe('1 204');
  });
});

describe('Banner', () => {
  it('states what and why with the info look and a polite status role by default', () => {
    const { target } = render(Banner, { title: 'Local mode.', text: 'No requests are made.' });
    const banner = one(target, '.dl-banner');
    expect(banner.className).toBe('dl-banner');
    expect(banner.getAttribute('role')).toBe('status');
    expect(one(banner, '.dl-banner__text strong').textContent).toBe('Local mode.');
    expect(one(banner, '.dl-banner__why').textContent).toBe('No requests are made.');
    expect(one(banner, '.dl-banner__text').textContent).toBe('Local mode. No requests are made.');
    expect(one(banner, 'svg.dl-banner__icon').getAttribute('aria-hidden')).toBe('true');
  });

  it('uses the alert role for errors', () => {
    const { target } = render(Banner, { kind: 'error', title: 'The catalog is unavailable.' });
    const banner = one(target, '.dl-banner');
    expect(banner.classList).toContain('dl-banner--error');
    expect(banner.getAttribute('role')).toBe('alert');
  });

  it.each(['neutral', 'warning', 'success'] as const)('maps the %s kind', (kind) => {
    const { target } = render(Banner, { kind, title: 'x' });
    expect(one(target, '.dl-banner').classList).toContain(`dl-banner--${kind}`);
  });

  it('accepts a specific icon and actions', () => {
    const actions = createRawSnippet(() => ({ render: () => '<button>Retry now</button>' }));
    const { target } = render(Banner, { kind: 'neutral', icon: WifiOff, title: 'Offline.', actions });
    expect(one(target, '.dl-banner__icon').classList).toContain('lucide-wifi-off');
    expect(one(target, '.dl-banner__actions button').textContent).toBe('Retry now');
  });
});

describe('EmptyState', () => {
  it('shows the glyph, title, text and actions', () => {
    const actions = createRawSnippet(() => ({ render: () => '<button>Browse catalog</button>' }));
    const { target } = render(EmptyState, {
      icon: LibraryBig,
      title: 'Your library is empty',
      text: 'Install mods from the catalog.',
      actions,
    });
    const root = one(target, '.dl-empty');
    expect(root.className).toBe('dl-empty');
    expect(one(root, '.dl-empty__glyph svg').getAttribute('aria-hidden')).toBe('true');
    expect(one(root, 'h3.dl-empty__title').textContent).toBe('Your library is empty');
    expect(one(root, 'p.dl-empty__text').textContent).toBe('Install mods from the catalog.');
    expect(one(root, '.dl-empty__actions button').textContent).toBe('Browse catalog');
  });

  it('supports the compact and error looks and another heading level', () => {
    const { target } = render(EmptyState, { icon: LibraryBig, title: 'Removed', compact: true, error: true, level: 2 });
    const root = one(target, '.dl-empty');
    expect(root.classList).toContain('dl-empty--compact');
    expect(root.classList).toContain('dl-empty--error');
    expect(root.querySelector('h2.dl-empty__title')).not.toBeNull();
    expect(root.querySelector('.dl-empty__text')).toBeNull();
  });
});

describe('Skeleton', () => {
  it('renders a static block of the requested kind and size', () => {
    const { target } = render(Skeleton, { kind: 'title', width: '92%' });
    const block = one(target, '.dl-skel');
    expect(block.classList).toContain('dl-skel--title');
    expect(block.style.width).toBe('92%');
    expect(block.getAttribute('aria-hidden')).toBe('true');
  });

  it('builds a catalog card placeholder with the card geometry, hidden from assistive technology', () => {
    const { target } = render(CardSkeleton, { index: 1 });
    const card = one(target, '.dl-card');
    expect(card.getAttribute('aria-hidden')).toBe('true');
    expect(card.querySelector('.dl-skel--thumb')).not.toBeNull();
    // The second pattern has a one-line title: the second title bar is kept for height but hidden.
    const titles = all(card, '.dl-skel--title');
    expect(titles).toHaveLength(2);
    expect(titles[1]?.classList).toContain('dl-skel--hidden');
  });

  it('builds a library row placeholder', () => {
    const { target } = render(RowSkeleton, { index: 0 });
    const row = one(target, '.dl-row');
    expect(row.getAttribute('aria-hidden')).toBe('true');
    expect(all(row, '.dl-skel')).toHaveLength(4);
  });
});

describe('ProgressBar', () => {
  it('exposes the value to assistive technology and scales the fill', () => {
    const props = $state({ value: 0.62, label: 'faunceleste_bubblegum_f5f81.rar' });
    const { target } = render(ProgressBar, props);
    const bar = one(target, '.dl-progress');
    expect(bar.getAttribute('role')).toBe('progressbar');
    expect(bar.getAttribute('aria-valuemin')).toBe('0');
    expect(bar.getAttribute('aria-valuemax')).toBe('100');
    expect(bar.getAttribute('aria-valuenow')).toBe('62');
    expect(bar.getAttribute('aria-label')).toBe('faunceleste_bubblegum_f5f81.rar');
    expect(one(bar, '.dl-progress__fill').style.getPropertyValue('--value')).toBe('0.62');
    props.value = 1.4;
    flushSync();
    expect(bar.getAttribute('aria-valuenow')).toBe('100');
    expect(one(bar, '.dl-progress__fill').style.getPropertyValue('--value')).toBe('1');
  });

  it('shows the meta lines it is given', () => {
    const { target } = render(ProgressBar, {
      value: 0.5,
      label: 'file.rar',
      name: 'file.rar',
      stage: 'Downloading',
      detail: '5.2 of 8.4 MB · 62 %',
      rate: '2.4 MB/s · 2 s left',
      valueText: '62 %',
    });
    const metas = all(target, '.dl-progress__meta');
    expect(metas).toHaveLength(2);
    expect(one(metas[0] as HTMLElement, 'strong.dl-mono').textContent).toBe('file.rar');
    expect(metas[0]?.textContent).toContain('Downloading');
    expect(metas[1]?.textContent).toContain('2.4 MB/s');
    expect(one(target, '.dl-progress').getAttribute('aria-valuetext')).toBe('62 %');
  });

  it('has no value while indeterminate', () => {
    const { target } = render(ProgressBar, { value: 0, label: 'Verifying', state: 'indeterminate' });
    const bar = one(target, '.dl-progress');
    expect(bar.classList).toContain('dl-progress--indeterminate');
    expect(bar.hasAttribute('aria-valuenow')).toBe(false);
  });

  it.each(['paused', 'error', 'done'] as const)('maps the %s state', (state) => {
    const { target } = render(ProgressBar, { value: 0.3, label: 'x', state });
    expect(one(target, '.dl-progress').classList).toContain(`dl-progress--${state}`);
  });

  it('has a thin variant without meta lines', () => {
    const { target } = render(ProgressBar, { value: 0.3, label: 'x', thin: true, stage: 'Copying' });
    expect(one(target, '.dl-progress').classList).toContain('dl-progress--thin');
    expect(target.querySelector('.dl-progress__meta')).toBeNull();
  });
});
