import Undo2 from '@lucide/svelte/icons/undo-2';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { all, click, fire, one, render } from '../test/render';
import Toast from './Toast.svelte';
import ToastStack from './ToastStack.svelte';

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

describe('Toast', () => {
  it('says what happened with a polite status role, a kind icon and a dismiss button', () => {
    const ondismiss = vi.fn();
    const { target } = render(Toast, {
      kind: 'success',
      title: 'Haze V2 installed and enabled.',
      text: 'It is first in the list.',
      dismissLabel: 'Dismiss',
      ondismiss,
    });
    const toast = one(target, '.dl-toast');
    expect(toast.classList).toContain('dl-toast--success');
    expect(toast.getAttribute('role')).toBe('status');
    expect(one(toast, 'svg.dl-toast__icon').classList).toContain('lucide-circle-check');
    expect(one(toast, '.dl-toast__title').textContent).toBe('Haze V2 installed and enabled.');
    expect(one(toast, '.dl-toast__text').textContent).toBe('It is first in the list.');
    const dismiss = one(toast, 'button[aria-label="Dismiss"]');
    click(dismiss);
    expect(ondismiss).toHaveBeenCalledOnce();
  });

  it('names a file after the text in a monospace font', () => {
    const { target } = render(Toast, {
      title: 'Profile “Evening” exported.',
      text: 'Saved as',
      file: String.raw`D:\Shared\Evening.json`,
      dismissLabel: 'Dismiss',
      ondismiss: vi.fn(),
    });
    const text = one(target, '.dl-toast__text');
    expect(text.textContent).toBe(String.raw`Saved as D:\Shared\Evening.json`);
    expect(one(text, '.dl-mono').textContent).toBe(String.raw`D:\Shared\Evening.json`);
  });

  it('uses the alert role for errors and never times them out', () => {
    const ondismiss = vi.fn();
    const { target } = render(Toast, { kind: 'error', title: 'Download failed.', dismissLabel: 'Dismiss', ondismiss });
    expect(one(target, '.dl-toast').getAttribute('role')).toBe('alert');
    vi.advanceTimersByTime(60_000);
    expect(ondismiss).not.toHaveBeenCalled();
  });

  it('keeps warnings until dismissed', () => {
    const ondismiss = vi.fn();
    render(Toast, { kind: 'warning', title: 'Antivirus check pending.', dismissLabel: 'Dismiss', ondismiss });
    vi.advanceTimersByTime(60_000);
    expect(ondismiss).not.toHaveBeenCalled();
  });

  it('times out neutral, success and info toasts after 6 s', () => {
    for (const kind of ['neutral', 'success', 'info'] as const) {
      const ondismiss = vi.fn();
      render(Toast, { kind, title: 'x', dismissLabel: 'Dismiss', ondismiss });
      vi.advanceTimersByTime(5_999);
      expect(ondismiss).not.toHaveBeenCalled();
      vi.advanceTimersByTime(1);
      expect(ondismiss).toHaveBeenCalledOnce();
    }
  });

  it('pauses its timer while hovered or focused', () => {
    const ondismiss = vi.fn();
    const { target } = render(Toast, { title: 'x', dismissLabel: 'Dismiss', ondismiss });
    const toast = one(target, '.dl-toast');
    vi.advanceTimersByTime(4_000);
    fire(toast, 'pointerenter', { bubbles: false });
    vi.advanceTimersByTime(10_000);
    expect(ondismiss).not.toHaveBeenCalled();
    fire(toast, 'pointerleave', { bubbles: false });
    fire(toast, 'focusin');
    vi.advanceTimersByTime(10_000);
    expect(ondismiss).not.toHaveBeenCalled();
    fire(toast, 'focusout');
    vi.advanceTimersByTime(1_999);
    expect(ondismiss).not.toHaveBeenCalled();
    vi.advanceTimersByTime(1);
    expect(ondismiss).toHaveBeenCalledOnce();
  });

  it('offers one action and a details link', () => {
    const onclick = vi.fn();
    const { target } = render(Toast, {
      kind: 'success',
      icon: Undo2,
      title: 'Deadlock was closed — 4 changes applied.',
      action: { label: 'Undo', icon: Undo2, onclick },
      details: { label: 'Details', href: '#/activity/L-12' },
      dismissLabel: 'Dismiss',
      ondismiss: vi.fn(),
    });
    const actions = one(target, '.dl-toast__actions');
    click(one(actions, 'button.dl-btn.dl-btn--sm'));
    expect(onclick).toHaveBeenCalledOnce();
    const link = one<HTMLAnchorElement>(actions, 'a.dl-link');
    expect(link.textContent).toBe('Details');
    expect(link.getAttribute('href')).toBe('#/activity/L-12');
  });
});

describe('ToastStack', () => {
  const toasts = [1, 2, 3, 4].map((n) => ({ id: `t${n}`, kind: 'warning' as const, title: `Toast ${n}` }));

  it('shows the newest three, newest at the bottom', () => {
    const { target } = render(ToastStack, { toasts, dismissLabel: 'Dismiss', ondismiss: vi.fn() });
    const titles = all(target, '.dl-toast-stack .dl-toast__title').map((t) => t.textContent);
    expect(titles).toEqual(['Toast 2', 'Toast 3', 'Toast 4']);
  });

  it('reports which toast was dismissed', () => {
    const ondismiss = vi.fn();
    const { target } = render(ToastStack, { toasts, dismissLabel: 'Dismiss', ondismiss });
    click(all(target, 'button[aria-label="Dismiss"]')[0] as HTMLElement);
    expect(ondismiss).toHaveBeenCalledWith('t2');
  });
});
