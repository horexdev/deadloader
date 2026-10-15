import { describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/core', () => import('../mock/core'));

const { zoomMove, zoomStep, watchZoom } = await import('./zoom');
const { app } = await import('./app.svelte');
const { invoke, resetMock } = await import('../mock/core');

const key = (k: string, ctrlKey = true) => new KeyboardEvent('keydown', { key: k, ctrlKey, cancelable: true });
function wheel(deltaY: number, ctrlKey = true): WheelEvent {
  const event = new WheelEvent('wheel', { deltaY, cancelable: true });
  // happy-dom doesn't take modifier keys from a WheelEvent's init.
  Object.defineProperty(event, 'ctrlKey', { value: ctrlKey });
  return event;
}

describe('zoom', () => {
  it('steps through the choices of Settings → General, within 50–200 %', () => {
    expect(zoomStep(100, 'in')).toBe(110);
    expect(zoomStep(100, 'out')).toBe(90);
    expect(zoomStep(150, 'reset')).toBe(100);
    expect(zoomStep(200, 'in')).toBe(200);
    expect(zoomStep(50, 'out')).toBe(50);
    // A value set by hand between two choices moves to the next one.
    expect(zoomStep(105, 'in')).toBe(110);
    expect(zoomStep(105, 'out')).toBe(100);
  });

  it('reads Ctrl + plus, minus, 0 and Ctrl + mouse wheel', () => {
    expect(zoomMove(key('='))).toBe('in');
    expect(zoomMove(key('+'))).toBe('in');
    expect(zoomMove(key('-'))).toBe('out');
    expect(zoomMove(key('0'))).toBe('reset');
    expect(zoomMove(key('=', false))).toBeNull();
    expect(zoomMove(key('a'))).toBeNull();
    expect(zoomMove(wheel(-100))).toBe('in');
    expect(zoomMove(wheel(100))).toBe('out');
    expect(zoomMove(wheel(100, false))).toBeNull();
  });

  it('saves the new zoom, so the core applies it to the window', async () => {
    resetMock();
    await app.start();
    const stop = watchZoom(app);
    const event = key('=');
    window.dispatchEvent(event);
    expect(event.defaultPrevented).toBe(true);
    await vi.waitFor(async () => {
      const saved = await invoke<{ general: { zoom_percent: number } }>('settings');
      expect(saved.general.zoom_percent).toBe(110);
    });
    expect(app.settings?.general?.zoom_percent).toBe(110);
    window.dispatchEvent(key('0'));
    await vi.waitFor(() => expect(app.settings?.general?.zoom_percent).toBe(100));
    stop();
    window.dispatchEvent(key('='));
    await new Promise((resolve) => setTimeout(resolve, 20));
    expect(app.settings?.general?.zoom_percent).toBe(100);
  });
});
