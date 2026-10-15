import { describe, expect, it, vi } from 'vitest';

const invoke = vi.fn().mockResolvedValue(null);
vi.mock('@tauri-apps/api/core', () => ({ invoke: (...args: unknown[]) => invoke(...args) }));

const { default: TitleBar } = await import('./TitleBar.svelte');
const { render } = await import('../lib/test/render');

describe('the title bar', () => {
  it('minimises, maximises and closes the window with its buttons', () => {
    const { target } = render(TitleBar, {});
    for (const button of target.querySelectorAll('button')) {
      button.click();
    }
    expect(invoke.mock.calls.map(([name]) => name)).toEqual([
      'window_minimize',
      'window_toggle_maximize',
      'window_close',
    ]);
  });

  it('moves the window from its empty area, and maximises on a double click', () => {
    invoke.mockClear();
    const { target } = render(TitleBar, {});
    const area = target.querySelector('.titlebar__drag') as HTMLElement;
    area.dispatchEvent(new MouseEvent('mousedown', { button: 0, detail: 1, bubbles: true }));
    area.dispatchEvent(new MouseEvent('mousedown', { button: 0, detail: 2, bubbles: true }));
    area.dispatchEvent(new MouseEvent('dblclick', { bubbles: true }));
    expect(invoke.mock.calls.map(([name]) => name)).toEqual(['window_start_drag', 'window_toggle_maximize']);
  });
});
