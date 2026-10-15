import { flushSync } from 'svelte';
import { describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/core', () => import('../mock/core'));

const { AppState } = await import('../lib/app.svelte');
const { default: ErrorHost } = await import('./ErrorHost.svelte');
const { click, one, render } = await import('../lib/test/render');

describe('the error dialog of the window (D12)', () => {
  it('shows nothing until an error is shown', () => {
    const app = new AppState();
    const { target } = render(ErrorHost, { app });
    expect(target.querySelector('.dl-dialog')).toBeNull();
    app.showError({ id: 'E12', args: { reason: 'full', log_id: 'file-20261015-1412-0031' } });
    flushSync();
    expect(one(target, '.dl-dialog__title').textContent).toBe('Not enough disk space.');
    expect(one(target, '.dl-dialog__body').textContent).toContain('The disk is full.');
    expect(one(target, '.dl-dialog__details .dl-mono').textContent).toBe('file-20261015-1412-0031');
    // The entry itself, not just Activity.
    expect(one<HTMLAnchorElement>(target, '.dl-dialog__details a').getAttribute('href')).toBe(
      '#/activity/file-20261015-1412-0031',
    );
  });

  it('offers no Activity link when the error names no log entry', () => {
    const app = new AppState();
    const { target } = render(ErrorHost, { app });
    app.showError({ id: 'Internal', args: { detail: 'boom' } });
    flushSync();
    expect(target.querySelector('.dl-dialog__details a')).toBeNull();
  });

  it('closes through the app state', () => {
    const app = new AppState();
    const { target } = render(ErrorHost, { app });
    app.showError({ id: 'E11', args: {} });
    flushSync();
    click(one(target, '.dl-dialog__head button'));
    expect(app.error).toBeNull();
    expect(target.querySelector('.dl-dialog')).toBeNull();
  });
});
