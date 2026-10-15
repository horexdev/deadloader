import { describe, expect, it, vi } from 'vitest';
import type { Description } from '../../lib/api';
import { enhanceDescription, type DescriptionTexts } from './description';

const texts: DescriptionTexts = {
  opensBrowser: 'opens in your browser',
  opensHere: 'Open in deadloader',
  imageFrom: 'Image from',
  clickToLoad: 'click to load',
  imageFailed: 'couldn’t be loaded',
  imageAlt: 'Image from the description',
  youtube: 'YouTube video',
  spoiler: 'Spoiler — click to show',
  spoilerHide: 'Hide spoiler',
};

const description: Description = {
  html:
    '<p>See <a data-dl-link="0">this mod</a> and <a data-dl-link="2">the site</a>.</p>' +
    '<span class="dl-image" data-dl-image="0"></span>' +
    '<span class="dl-image" data-dl-image="1"></span>' +
    '<span class="dl-image" data-dl-image="2"></span>' +
    '<span class="dl-youtube" data-dl-link="1"></span>' +
    '<div class="dl-spoiler"><p>Secret</p></div>',
  links: [
    { url: 'https://gamebanana.com/mods/700002', host: 'gamebanana.com', internal_mod_id: 700_002 },
    { url: 'https://www.youtube.com/watch?v=x', host: 'www.youtube.com', internal_mod_id: null },
    { url: 'https://example.org/a', host: 'example.org', internal_mod_id: null },
  ],
  images: [
    { id: 'a'.repeat(32), host: 'i.imgur.com', alt: 'At night', access: 'consent' },
    { id: 'b'.repeat(32), host: 'images.gamebanana.com', alt: null, access: 'click' },
    { id: 'c'.repeat(32), host: 'images.gamebanana.com', alt: null, access: 'load' },
  ],
  consent: [{ host: 'i.imgur.com', images: ['a'.repeat(32)] }],
  truncated: false,
};

function setup() {
  const root = document.createElement('div');
  root.innerHTML = description.html;
  document.body.append(root);
  const handlers = {
    onlink: vi.fn(),
    onmod: vi.fn(),
    onimage: vi.fn(),
    imageUrl: (id: string) => `https://dl.localhost/img/${id}`,
  };
  const controller = enhanceDescription(root, description, texts, handlers);
  return { root, handlers, controller };
}

describe('a mod description (S3)', () => {
  it('opens external links through D1 and GameBanana mod links inside deadloader', () => {
    const { root, handlers } = setup();
    const links = [...root.querySelectorAll<HTMLElement>('a[data-dl-link]')];
    expect(links.map((a) => a.getAttribute('href'))).toEqual([null, null]);
    expect(links.every((a) => a.tabIndex === 0 && a.getAttribute('role') === 'link')).toBe(true);
    expect(links[1]?.querySelector('svg')?.getAttribute('aria-label')).toBe('opens in your browser');
    links[0]?.click();
    expect(handlers.onmod).toHaveBeenCalledWith(700_002);
    links[1]?.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));
    expect(handlers.onlink).toHaveBeenCalledWith('https://example.org/a');
  });

  it('shows pictures as placeholders naming their host until clicked (29, 122)', () => {
    const { root, handlers } = setup();
    const placeholders = [...root.querySelectorAll<HTMLButtonElement>('button.dl-image__load')];
    expect(placeholders).toHaveLength(2);
    expect(placeholders[0]?.textContent).toBe('Image from i.imgur.com — click to load');
    expect(placeholders[0]?.getAttribute('aria-haspopup')).toBe('dialog');
    expect(placeholders[1]?.hasAttribute('aria-haspopup')).toBe(false);
    // An already allowed picture shows at once.
    const shown = root.querySelector<HTMLImageElement>('[data-dl-image="2"] img');
    expect(shown?.src).toBe(`https://dl.localhost/img/${'c'.repeat(32)}`);
    placeholders[0]?.click();
    expect(handlers.onimage).toHaveBeenCalledWith(description.images[0]);
  });

  it('puts a picture in place of its placeholder once it may load', () => {
    const { root, controller } = setup();
    controller.show('a'.repeat(32));
    const img = root.querySelector<HTMLImageElement>('[data-dl-image="0"] img');
    expect(img?.src).toBe(`https://dl.localhost/img/${'a'.repeat(32)}`);
    expect(img?.alt).toBe('At night');
    expect(root.querySelector('[data-dl-image="0"] figcaption')?.textContent).toBe('i.imgur.com');
    img?.dispatchEvent(new Event('error'));
    expect(root.querySelector('[data-dl-image="0"]')?.textContent).toContain('couldn’t be loaded');
  });

  it('turns a YouTube embed into a card that opens the video in the browser', () => {
    const { root, handlers } = setup();
    const card = root.querySelector<HTMLButtonElement>('.dl-youtube button');
    expect(card?.textContent).toContain('YouTube video');
    expect(card?.textContent).toContain('www.youtube.com');
    card?.click();
    expect(handlers.onlink).toHaveBeenCalledWith('https://www.youtube.com/watch?v=x');
  });

  it('keeps spoilers closed until asked', () => {
    const { root } = setup();
    const toggle = root.querySelector<HTMLButtonElement>('.dl-spoiler > button');
    const body = root.querySelector<HTMLElement>('.dl-spoiler > .dl-spoiler__body');
    expect(toggle?.getAttribute('aria-expanded')).toBe('false');
    expect(body?.hidden).toBe(true);
    toggle?.click();
    expect(toggle?.getAttribute('aria-expanded')).toBe('true');
    expect(body?.hidden).toBe(false);
    expect(body?.textContent).toBe('Secret');
  });
});
