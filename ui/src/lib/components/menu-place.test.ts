import { describe, expect, it } from 'vitest';
import { placeMenu } from './menu-place';

const viewport = { width: 1280, height: 800 };
const trigger = (top: number, left = 100, width = 180, height = 32) => ({ top, left, width, height, bottom: top + height, right: left + width });

describe('where a menu opens', () => {
  it('opens below its trigger, 4 px apart, when it fits there', () => {
    expect(placeMenu(trigger(100), 200, viewport)).toEqual({ above: false, top: 136, left: 100, minWidth: 180, maxHeight: 320 });
  });

  it('opens above when it does not fit below and there is more room above', () => {
    const place = placeMenu(trigger(700), 300, viewport);
    expect(place.above).toBe(true);
    // Its bottom edge sits 4 px above the trigger; its height is capped to the room above.
    expect(place.bottom).toBe(800 - 700 + 4);
    expect(place.maxHeight).toBe(320);
  });

  it('stays below when neither side has room for all of it but below has more, and fits the room', () => {
    const place = placeMenu(trigger(300, 100, 180, 32), 900, { width: 1280, height: 800 });
    expect(place.above).toBe(false);
    expect(place.maxHeight).toBe(320);
    const tight = placeMenu(trigger(60), 300, { width: 1280, height: 300 });
    expect(tight.above).toBe(false);
    expect(tight.maxHeight).toBe(300 - 92 - 4 - 8);
  });

  it('keeps the menu inside the window horizontally', () => {
    const place = placeMenu(trigger(100, 1200, 70), 200, viewport, 200);
    expect(place.left).toBe(1280 - 8 - 200);
  });
});
