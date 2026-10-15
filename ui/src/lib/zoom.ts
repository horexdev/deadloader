// The window's zoom: Ctrl + mouse wheel, Ctrl + plus / minus and Ctrl + 0 step through the choices of Settings →
// General and save the new value; the core applies the saved zoom to the window (WebView2's own zoom keys are off, so
// the zoom is always the saved one, within 50–200 %).

import { api } from './api';
import type { AppState } from './app.svelte';

/** The design's choices (Settings → General). */
export const ZOOM_STEPS = [50, 67, 75, 80, 90, 100, 110, 125, 150, 175, 200] as const;
const RESET = 100;
/** A trackpad pinch sends many small wheel events: one step at most this often. */
const WHEEL_PAUSE_MS = 150;

export type ZoomMove = 'in' | 'out' | 'reset';

export function zoomStep(current: number, move: ZoomMove): number {
  if (move === 'reset') {
    return RESET;
  }
  if (move === 'in') {
    return ZOOM_STEPS.find((z) => z > current) ?? ZOOM_STEPS[ZOOM_STEPS.length - 1] ?? current;
  }
  return [...ZOOM_STEPS].reverse().find((z) => z < current) ?? ZOOM_STEPS[0] ?? current;
}

/** What a key or wheel event asks of the zoom, or null. */
export function zoomMove(event: KeyboardEvent | WheelEvent): ZoomMove | null {
  if (!event.ctrlKey || event.altKey) {
    return null;
  }
  if (event instanceof WheelEvent) {
    return event.deltaY < 0 ? 'in' : event.deltaY > 0 ? 'out' : null;
  }
  switch (event.key) {
    case '+':
    case '=':
      return 'in';
    case '-':
      return 'out';
    case '0':
      return 'reset';
    default:
      return null;
  }
}

/** Listens on the window until the returned function is called. */
export function watchZoom(app: AppState): () => void {
  let lastWheel = 0;
  const handle = (event: KeyboardEvent | WheelEvent): void => {
    const move = zoomMove(event);
    const settings = app.settings;
    if (!move || !settings?.general) {
      return;
    }
    event.preventDefault();
    if (event instanceof WheelEvent) {
      const now = Date.now();
      if (now - lastWheel < WHEEL_PAUSE_MS) {
        return;
      }
      lastWheel = now;
    }
    const current = Number(settings.general.zoom_percent);
    const next = zoomStep(current, move);
    if (next === current) {
      return;
    }
    // A JSON copy: the settings are JSON, and `app.settings` is a reactive proxy that structuredClone refuses.
    const changed = JSON.parse(JSON.stringify(settings)) as typeof settings;
    if (changed.general) {
      changed.general.zoom_percent = next;
    }
    app.settings = changed;
    api
      .updateSettings(changed)
      .then((saved) => {
        app.settings = saved;
      })
      .catch((error: unknown) => {
        app.settings = settings;
        app.fail(error);
      });
  };
  const onKey = (event: KeyboardEvent): void => handle(event);
  const onWheel = (event: WheelEvent): void => handle(event);
  window.addEventListener('keydown', onKey);
  // Not passive: Ctrl + wheel must not also scroll or zoom the page.
  window.addEventListener('wheel', onWheel, { passive: false });
  return () => {
    window.removeEventListener('keydown', onKey);
    window.removeEventListener('wheel', onWheel);
  };
}
