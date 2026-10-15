import Check from '@lucide/svelte/icons/check';
import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
import { flushSync, type ComponentProps } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import { all, click, fire, one, render } from '../test/render';
import CatalogCard from './CatalogCard.svelte';

type CardProps = ComponentProps<typeof CatalogCard>;

const base: CardProps = {
  href: '#/mod/652300',
  title: 'Faun Celeste (QUALITY UPDATE)',
  byline: 'basilisken · Skins → Celeste',
  thumb: 'thumbs/00.jpg',
  likes: '221',
  likesLabel: '221 likes',
  date: '3 days ago',
  dateTitle: '11 Oct 2026, 14:20',
};

function setup(extra: Partial<CardProps> = {}) {
  const props: CardProps = $state({ ...base, ...extra });
  const rendered = render(CatalogCard, props);
  return { ...rendered, props, card: one(rendered.target, 'article.dl-card') };
}

describe('CatalogCard', () => {
  it('shows the thumbnail, the title as the card link, the byline and the meta line', () => {
    const { card } = setup();
    const img = one<HTMLImageElement>(card, '.dl-card__thumb img');
    expect(img.getAttribute('src')).toBe('thumbs/00.jpg');
    expect(img.alt).toBe('');
    expect(img.getAttribute('loading')).toBe('lazy');
    const link = one<HTMLAnchorElement>(card, 'h3.dl-card__title a.dl-card__link');
    expect(link.getAttribute('href')).toBe('#/mod/652300');
    expect(link.textContent).toBe('Faun Celeste (QUALITY UPDATE)');
    expect(link.hasAttribute('data-grid-focus')).toBe(true);
    expect(one(card, '.dl-card__author').textContent).toBe('basilisken · Skins → Celeste');
    const meta = one(card, '.dl-card__meta');
    expect(meta.textContent).toContain('221 likes');
    expect(one(meta, '[title="11 Oct 2026, 14:20"]').textContent).toBe('3 days ago');
  });

  it('falls back to the missing-image box when there is no thumbnail or it fails', () => {
    const none = setup({ thumb: null });
    expect(one(none.card, '.dl-card__thumb-missing svg')).toBeTruthy();
    const failing = setup();
    fire(one(failing.card, '.dl-card__thumb img'), 'error', { bubbles: false });
    expect(failing.card.querySelector('.dl-card__thumb img')).toBeNull();
    expect(one(failing.card, '.dl-card__thumb-missing')).toBeTruthy();
  });

  it('tells the page when its thumbnail failed, so the page can be fetched again', () => {
    const onthumberror = vi.fn();
    const { card } = setup({ onthumberror });
    fire(one(card, '.dl-card__thumb img'), 'error', { bubbles: false });
    expect(onthumberror).toHaveBeenCalledTimes(1);
  });

  it('shows state badges on the thumbnail', () => {
    const { card } = setup({
      badges: [
        { icon: Check, label: 'v2', variant: 'accent' },
        { icon: CircleArrowUp, label: 'Update', variant: 'info' },
      ],
    });
    const badges = all(card, '.dl-card__badges .dl-badge');
    expect(badges.map((b) => b.textContent)).toEqual(['v2', 'Update']);
    expect(badges[1]?.classList).toContain('dl-badge--info');
  });

  it('toggles the favourite as a pressed button', () => {
    const onfavourite = vi.fn();
    const { card, props } = setup({ favourite: false, favouriteLabel: 'Add to favourites', onfavourite });
    const fav = one(card, 'button.dl-card__fav');
    expect(fav.getAttribute('aria-pressed')).toBe('false');
    expect(fav.getAttribute('aria-label')).toBe('Add to favourites');
    click(fav);
    expect(onfavourite).toHaveBeenCalledOnce();
    props.favourite = true;
    flushSync();
    expect(fav.getAttribute('aria-pressed')).toBe('true');
  });

  it('offers quick install as a small solid primary icon button, with busy and retry states', () => {
    const oninstall = vi.fn();
    const { card, props } = setup({ quickInstall: { label: 'Install' }, oninstall });
    const button = one(card, '.dl-card__meta button.dl-card__quick');
    expect(button.classList).toContain('dl-btn--primary');
    expect(button.classList).toContain('dl-btn--icon');
    expect(button.getAttribute('aria-label')).toBe('Install');
    click(button);
    expect(oninstall).toHaveBeenCalledOnce();
    props.quickInstall = { label: 'Installing, 62 %', busy: true };
    props.progressText = 'Installing · 62 %';
    flushSync();
    expect(one(card, '.dl-card__quick').getAttribute('aria-busy')).toBe('true');
    expect(one(card, '.dl-card__meta').textContent).toContain('Installing · 62 %');
    props.quickInstall = { label: 'Retry', error: true };
    props.error = true;
    flushSync();
    expect(one(card, '.dl-card__quick').dataset['state']).toBe('error');
    expect(card.classList).toContain('is-error');
  });

  it('is dimmed and not openable while offline and not saved', () => {
    const { card } = setup({ offline: 'Offline' });
    expect(card.getAttribute('aria-disabled')).toBe('true');
    const link = one<HTMLAnchorElement>(card, '.dl-card__link');
    expect(link.getAttribute('aria-disabled')).toBe('true');
    const event = new MouseEvent('click', { bubbles: true, cancelable: true });
    link.dispatchEvent(event);
    expect(event.defaultPrevented).toBe(true);
    expect(one(card, '.dl-card__meta').textContent).toContain('Offline');
  });

  it('blurs sensitive thumbnails with the generic label and reveals on click (Enter clicks the link)', () => {
    const onreveal = vi.fn();
    const { card } = setup({
      nsfw: { blurredThumb: 'blur/06_blur.png', label: 'Sensitive content', hint: 'Click to show' },
      onreveal,
      favourite: false,
      favouriteLabel: 'Add to favourites',
    });
    expect(one(card, '.dl-card__thumb').classList).toContain('dl-card__thumb--blurred');
    expect(one<HTMLImageElement>(card, '.dl-card__thumb img').getAttribute('src')).toBe('blur/06_blur.png');
    expect(one(card, '.dl-card__nsfw span').textContent.trim()).toBe('Sensitive content');
    expect(one(card, '.dl-card__nsfw small').textContent).toBe('Click to show');
    expect(card.querySelector('.dl-card__fav')).toBeNull();
    const link = one(card, '.dl-card__link');
    const event = new MouseEvent('click', { bubbles: true, cancelable: true });
    link.dispatchEvent(event);
    expect(event.defaultPrevented).toBe(true);
    expect(onreveal).toHaveBeenCalledOnce();
  });

  it('calls onopen for the link so the page can route without reloading', () => {
    const onopen = vi.fn((event: MouseEvent) => event.preventDefault());
    const { card } = setup({ onopen });
    click(one(card, '.dl-card__link'));
    expect(onopen).toHaveBeenCalledOnce();
  });

  it('takes its controls out of the tab order when it does not hold the grid tab stop', () => {
    const { card } = setup({ tabindex: -1, favourite: false, favouriteLabel: 'Fav', quickInstall: { label: 'Install' } });
    const tabbable = [one(card, '.dl-card__link'), one(card, '.dl-card__fav'), one(card, '.dl-card__quick')];
    expect(tabbable.map((el) => el.tabIndex)).toEqual([-1, -1, -1]);
  });
});
