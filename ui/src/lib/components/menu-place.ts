// Where a dropdown menu opens (Select): below its trigger when it fits, above when there is more room there, its height
// capped to the room, and kept inside the window horizontally. The menu is fixed to the window, so it never makes the
// page or a panel scroll.

export interface MenuPlace {
  above: boolean;
  /** Distance from the window's top (below) or bottom (above) edge, in CSS px. */
  top?: number;
  bottom?: number;
  left: number;
  minWidth: number;
  maxHeight: number;
}

interface Rect {
  top: number;
  bottom: number;
  left: number;
  width: number;
}

/** The design's gap between trigger and menu. */
const GAP = 4;
/** Room kept free at the window's edges. */
const MARGIN = 8;
/** The design's tallest menu (`.dl-menu` max-height). */
const MAX_HEIGHT = 320;

/** `height` is the menu's natural height; `width` its rendered width, when known. */
export function placeMenu(
  trigger: Rect,
  height: number,
  viewport: { width: number; height: number },
  width = trigger.width,
): MenuPlace {
  const roomBelow = Math.max(0, viewport.height - trigger.bottom - GAP - MARGIN);
  const roomAbove = Math.max(0, trigger.top - GAP - MARGIN);
  const wanted = Math.min(height, MAX_HEIGHT);
  const above = wanted > roomBelow && roomAbove > roomBelow;
  const room = above ? roomAbove : roomBelow;
  const left = Math.max(MARGIN, Math.min(trigger.left, viewport.width - MARGIN - Math.max(width, trigger.width)));
  const maxHeight = Math.min(MAX_HEIGHT, room);
  return above
    ? { above, bottom: viewport.height - trigger.top + GAP, left, minWidth: trigger.width, maxHeight }
    : { above, top: trigger.bottom + GAP, left, minWidth: trigger.width, maxHeight };
}
