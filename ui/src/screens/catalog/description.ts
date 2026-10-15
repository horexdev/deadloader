// Brings a sanitised mod description to life (S3, R07, decisions 29, 31, 122). The core's HTML holds no addresses:
// links, pictures and YouTube embeds are placeholders with indexes into the description's lists. Here they become
// controls: links open through D1 (or inside deadloader for GameBanana mods), pictures load only when clicked (D2 for
// a new host), YouTube embeds become cards that open the video in the browser, and spoilers stay closed until asked.
// Everything is built with DOM calls and text nodes, never from strings of HTML.

import type { Description, DescriptionImage, ImageId } from '../../lib/api';

export interface DescriptionTexts {
  opensBrowser: string;
  opensHere: string;
  imageFrom: string;
  clickToLoad: string;
  imageFailed: string;
  imageAlt: string;
  youtube: string;
  spoiler: string;
  spoilerHide: string;
}

export interface DescriptionHandlers {
  /** An external link: D1 decides. */
  onlink: (url: string) => void;
  /** A GameBanana mod link: its page inside deadloader. */
  onmod: (id: number) => void;
  /** A picture placeholder was clicked; call `show` once it may load. */
  onimage: (image: DescriptionImage) => void;
  imageUrl: (id: ImageId) => string;
}

export interface DescriptionController {
  /** Shows the picture `id` in place of its placeholder. */
  show: (id: ImageId) => void;
}

const SVG = 'http://www.w3.org/2000/svg';

/** A Lucide icon drawn from its path data (the same markup the icon components render). */
function icon(paths: string[], label?: string): SVGSVGElement {
  const svg = document.createElementNS(SVG, 'svg');
  svg.setAttribute('class', 'dl-icon');
  svg.setAttribute('viewBox', '0 0 24 24');
  svg.setAttribute('fill', 'none');
  svg.setAttribute('stroke', 'currentColor');
  svg.setAttribute('stroke-linecap', 'round');
  svg.setAttribute('stroke-linejoin', 'round');
  if (label) {
    svg.setAttribute('role', 'img');
    svg.setAttribute('aria-label', label);
  } else {
    svg.setAttribute('aria-hidden', 'true');
  }
  for (const d of paths) {
    const path = document.createElementNS(SVG, 'path');
    path.setAttribute('d', d);
    svg.append(path);
  }
  return svg;
}

const EXTERNAL = ['M15 3h6v6', 'M10 14 21 3', 'M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6'];
const IMAGE = [
  'M5 3h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z',
  'M9 7a2 2 0 1 0 0 4a2 2 0 1 0 0-4',
  'm21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21',
];
const PLAY = ['M22 12c0 5.523-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2s10 4.477 10 10z', 'm10 8 6 4-6 4V8z'];
const EYE_OFF = [
  'M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49',
  'M14.084 14.158a3 3 0 0 1-4.242-4.242',
  'M17.479 17.499a10.75 10.75 0 0 1-15.417-5.151 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.446-5.143',
  'm2 2 20 20',
];

function index(element: Element, attribute: string): number {
  return Number(element.getAttribute(attribute));
}

function activate(element: HTMLElement, action: () => void): void {
  element.addEventListener('click', (event) => {
    event.preventDefault();
    action();
  });
  element.addEventListener('keydown', (event) => {
    if (event.key === 'Enter') {
      event.preventDefault();
      action();
    }
  });
}

export function enhanceDescription(
  root: HTMLElement,
  description: Description,
  texts: DescriptionTexts,
  handlers: DescriptionHandlers,
): DescriptionController {
  for (const link of root.querySelectorAll<HTMLElement>('a[data-dl-link]')) {
    const target = description.links[index(link, 'data-dl-link')];
    if (!target) {
      continue;
    }
    link.classList.add('dl-link');
    link.tabIndex = 0;
    link.setAttribute('role', 'link');
    if (target.internal_mod_id !== null) {
      link.title = texts.opensHere;
      activate(link, () => handlers.onmod(target.internal_mod_id as number));
    } else {
      link.title = target.url;
      link.append(icon(EXTERNAL, texts.opensBrowser));
      activate(link, () => handlers.onlink(target.url));
    }
  }

  const placeholders = new Map<ImageId, HTMLElement>();
  const showImage = (holder: HTMLElement, image: DescriptionImage) => {
    const figure = document.createElement('figure');
    figure.className = 'dl-image__figure';
    const img = document.createElement('img');
    img.src = handlers.imageUrl(image.id);
    img.alt = image.alt ?? texts.imageAlt;
    img.loading = 'lazy';
    const caption = document.createElement('figcaption');
    const host = document.createElement('span');
    host.className = 'dl-mono';
    host.textContent = image.host;
    caption.append(host);
    img.addEventListener('error', () => {
      holder.replaceChildren(placeholder(image, texts.imageFailed));
    });
    figure.append(img, caption);
    holder.replaceChildren(figure);
  };
  const placeholder = (image: DescriptionImage, state: string): HTMLButtonElement => {
    const button = document.createElement('button');
    button.type = 'button';
    button.className = 'dl-image__load';
    if (image.access === 'consent') {
      button.setAttribute('aria-haspopup', 'dialog');
    }
    const label = document.createElement('span');
    const host = document.createElement('b');
    host.className = 'dl-mono';
    host.textContent = image.host;
    label.append(`${texts.imageFrom} `, host, ` — ${state}`);
    button.append(icon(IMAGE), label);
    button.addEventListener('click', () => handlers.onimage(image));
    return button;
  };
  for (const holder of root.querySelectorAll<HTMLElement>('[data-dl-image]')) {
    const image = description.images[index(holder, 'data-dl-image')];
    if (!image) {
      continue;
    }
    placeholders.set(image.id, holder);
    if (image.access === 'load') {
      showImage(holder, image);
    } else {
      holder.replaceChildren(placeholder(image, texts.clickToLoad));
    }
  }

  for (const holder of root.querySelectorAll<HTMLElement>('.dl-youtube[data-dl-link]')) {
    const target = description.links[index(holder, 'data-dl-link')];
    if (!target) {
      continue;
    }
    const card = document.createElement('button');
    card.type = 'button';
    card.className = 'dl-youtube__card';
    const text = document.createElement('span');
    text.className = 'dl-youtube__text';
    const title = document.createElement('strong');
    title.textContent = texts.youtube;
    const host = document.createElement('span');
    host.className = 'dl-mono';
    host.textContent = target.host;
    text.append(title, host);
    card.append(icon(PLAY), text, icon(EXTERNAL, texts.opensBrowser));
    card.title = target.url;
    card.addEventListener('click', () => handlers.onlink(target.url));
    holder.replaceChildren(card);
  }

  for (const spoiler of root.querySelectorAll<HTMLElement>('.dl-spoiler')) {
    const body = document.createElement('div');
    body.className = 'dl-spoiler__body';
    body.append(...spoiler.childNodes);
    body.hidden = true;
    const toggle = document.createElement('button');
    toggle.type = 'button';
    toggle.className = 'dl-spoiler__toggle';
    toggle.setAttribute('aria-expanded', 'false');
    const label = document.createElement('span');
    label.textContent = texts.spoiler;
    toggle.append(icon(EYE_OFF), label);
    toggle.addEventListener('click', () => {
      const open = body.hidden;
      body.hidden = !open;
      toggle.setAttribute('aria-expanded', open ? 'true' : 'false');
      label.textContent = open ? texts.spoilerHide : texts.spoiler;
    });
    spoiler.replaceChildren(toggle, body);
  }

  return {
    show(id) {
      const holder = placeholders.get(id);
      const image = description.images.find((i) => i.id === id);
      if (holder && image) {
        showImage(holder, image);
      }
    },
  };
}
