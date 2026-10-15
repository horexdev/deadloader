import { flushSync } from 'svelte';
import { describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/core', () => import('../mock/core'));

const { AppState } = await import('../lib/app.svelte');
const { default: ErrorHost } = await import('./ErrorHost.svelte');
const { all, click, one, render } = await import('../lib/test/render');

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

  it('shows E9 with what the archive measured against each limit', () => {
    const app = new AppState();
    const { target } = render(ErrorHost, { app });
    const MiB = 1024 * 1024;
    app.showError({
      id: 'E9',
      args: {
        reason: 'too_large',
        limit: String(2048 * MiB),
        entry: 'mina/pak01_dir.vpk',
        unpacked: String(3892 * MiB),
        files: '41',
        size_limit: String(2048 * MiB),
        factor_limit: '20',
        files_limit: '256',
        ratio_limit: '100',
      },
    });
    flushSync();
    const body = one(target, '.dl-dialog__body');
    expect(body.textContent).toContain('It would unpack to 3.80');
    expect(body.textContent).toContain('so that a broken or malicious archive can’t fill your disk');
    const rows = all(body, 'tbody tr').map((r) => all(r, 'th, td').map((c) => (c.textContent ?? '').replace(/\s+/g, ' ').trim()));
    expect(rows[0]?.[0]).toBe('Unpacked size');
    expect(rows[0]?.[3]).toBe('Over the limit');
    expect(rows[1]).toEqual(['Compression ratio', '—', '20×', '']);
    expect(rows[2]).toEqual(['Mod files', '41', '256', 'Within the limit']);
  });

  it('has no limits table for other errors', () => {
    const app = new AppState();
    const { target } = render(ErrorHost, { app });
    app.showError({ id: 'E8', args: { reason: 'damaged' } });
    flushSync();
    expect(target.querySelector('table')).toBeNull();
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
