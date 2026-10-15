// The tooltip attachment: `<button {@attach tooltip({ text })}>`. Shows after 500 ms of hover and at once on keyboard
// focus, 8 px above the anchor (below when there is no room), kept inside the window; Esc, blur and pointer leave
// hide it. While shown, the anchor is described by it (aria-describedby).
import { flushSync, mount, unmount } from 'svelte';
import type { Attachment } from 'svelte/attachments';
import Tooltip from './Tooltip.svelte';

export interface TooltipOptions {
  text: string;
  sub?: string;
  /** Hover delay in ms. */
  delay?: number;
}

const GAP = 8;
const EDGE = 8;
let counter = 0;

/** Tooltip anchored to the element it is attached to; renders nothing while `text` is empty. */
export function tooltip(options: TooltipOptions): Attachment<HTMLElement> {
  return (anchor) => {
    const id = `dl-tooltip-${(counter += 1)}`;
    let instance: ReturnType<typeof mount> | undefined;
    let timer: ReturnType<typeof setTimeout> | undefined;

    function describe(on: boolean) {
      const ids = (anchor.getAttribute('aria-describedby') ?? '').split(/\s+/).filter((x) => x !== '' && x !== id);
      if (on) {
        ids.push(id);
      }
      if (ids.length > 0) {
        anchor.setAttribute('aria-describedby', ids.join(' '));
      } else {
        anchor.removeAttribute('aria-describedby');
      }
    }

    function place(tip: HTMLElement) {
      const target = anchor.getBoundingClientRect();
      const size = tip.getBoundingClientRect();
      let top = target.top - size.height - GAP;
      if (top < EDGE) {
        top = target.bottom + GAP;
      }
      const centred = target.left + target.width / 2 - size.width / 2;
      const left = Math.min(Math.max(EDGE, centred), Math.max(EDGE, window.innerWidth - size.width - EDGE));
      tip.style.top = `${Math.round(top)}px`;
      tip.style.left = `${Math.round(left)}px`;
    }

    function show() {
      clearTimeout(timer);
      if (instance || options.text === '') {
        return;
      }
      instance = mount(Tooltip, {
        target: document.body,
        props: { id, text: options.text, sub: options.sub, floating: true },
      });
      flushSync();
      const tip = document.getElementById(id);
      if (tip) {
        place(tip);
      }
      describe(true);
      document.addEventListener('keydown', onKeydown, true);
    }

    function hide() {
      clearTimeout(timer);
      if (!instance) {
        return;
      }
      void unmount(instance);
      instance = undefined;
      describe(false);
      document.removeEventListener('keydown', onKeydown, true);
    }

    function onKeydown(event: KeyboardEvent) {
      if (event.key === 'Escape') {
        hide();
      }
    }

    function onPointerEnter() {
      clearTimeout(timer);
      timer = setTimeout(show, options.delay ?? 500);
    }

    function onFocus() {
      let keyboard = true;
      try {
        keyboard = anchor.matches(':focus-visible');
      } catch {
        // Older engines: treat every focus as keyboard focus.
      }
      if (keyboard) {
        show();
      }
    }

    anchor.addEventListener('pointerenter', onPointerEnter);
    anchor.addEventListener('pointerleave', hide);
    anchor.addEventListener('focus', onFocus);
    anchor.addEventListener('blur', hide);

    return () => {
      hide();
      anchor.removeEventListener('pointerenter', onPointerEnter);
      anchor.removeEventListener('pointerleave', hide);
      anchor.removeEventListener('focus', onFocus);
      anchor.removeEventListener('blur', hide);
    };
  };
}

/** Splits a path at its last separator (\ or /) for middle truncation; a trailing separator stays with the tail. */
export function splitPath(path: string): { head: string; tail: string } {
  const body = path.replace(/[\\/]+$/, '');
  const cut = Math.max(body.lastIndexOf('\\'), body.lastIndexOf('/'));
  return cut < 0 ? { head: '', tail: path } : { head: path.slice(0, cut + 1), tail: path.slice(cut + 1) };
}
