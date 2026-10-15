import Settings from '@lucide/svelte/icons/settings';
import { createRawSnippet, flushSync } from 'svelte';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { all, click, keydown, one, render } from '../test/render';
import ErrorDialog from './ErrorDialog.svelte';

const labels = {
  close: 'Close',
  closeEsc: 'Close (Esc)',
  details: 'Details',
  detailsLog: 'Details · log entry',
  copy: 'Copy',
  copied: 'Copied',
  openActivity: 'Open in Activity',
};

const text = {
  what: 'The archive is too large to unpack safely.',
  why: 'It unpacks to more than 2 GB.',
  todo: 'If you trust it, raise the limit in Settings → Downloads.',
};

afterEach(() => {
  vi.useRealTimers();
});

describe('ErrorDialog (D12)', () => {
  it('says what happened in the title, then why and what to do', () => {
    const { target } = render(ErrorDialog, { ...text, labels, onclose: vi.fn() });
    const dialog = one(target, '.dl-dialog');
    expect(dialog.getAttribute('role')).toBe('alertdialog');
    expect(dialog.classList).toContain('dl-dialog--danger');
    expect(dialog.classList).toContain('dl-dialog--md');
    expect(one(dialog, '.dl-dialog__title').textContent).toBe(text.what);
    const paragraphs = all(dialog, '.dl-dialog__body > p').map((p) => p.textContent);
    expect(paragraphs).toEqual([text.why, text.todo]);
  });

  it('shows extra content between why and what to do', () => {
    const extra = createRawSnippet(() => ({ render: () => '<table class="limits"></table>' }));
    const { target } = render(ErrorDialog, { ...text, detail: 'x', labels, onclose: vi.fn(), children: extra });
    const body = one(target, '.dl-dialog__body');
    expect([...body.children].map((c) => c.tagName)).toEqual(['P', 'TABLE', 'P', 'DIV']);
  });

  it('shows the log entry with Copy and a link to Activity', async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    vi.spyOn(navigator.clipboard, 'writeText').mockImplementation(writeText);
    const { target } = render(ErrorDialog, {
      ...text,
      logId: 'file-20261015-1411-0058',
      labels,
      activityHref: '#/activity',
      onclose: vi.fn(),
    });
    const details = one(target, '.dl-dialog__details');
    expect(details.textContent).toContain('Details · log entry');
    expect(one(details, '.dl-mono').textContent).toBe('file-20261015-1411-0058');
    expect(one<HTMLAnchorElement>(details, 'a').getAttribute('href')).toBe('#/activity');
    click(one(details, 'button'));
    expect(writeText).toHaveBeenCalledWith('file-20261015-1411-0058');
    await vi.waitFor(() => {
      flushSync();
      expect(details.querySelector('[role="status"]')?.textContent).toBe('Copied');
    });
  });

  it('shows the technical detail when there is no log entry', () => {
    const { target } = render(ErrorDialog, { ...text, detail: 'os error 112', labels, onclose: vi.fn() });
    const details = one(target, '.dl-dialog__details');
    expect(details.textContent).toContain('Details');
    expect(details.textContent).not.toContain('log entry');
    expect(one(details, '.dl-mono').textContent).toBe('os error 112');
  });

  it('runs an action and closes; the primary one is last and focused', () => {
    const onclose = vi.fn();
    const retry = vi.fn();
    const remove = vi.fn();
    const { target } = render(ErrorDialog, {
      ...text,
      labels,
      onclose,
      actions: [
        { label: 'Remove task', place: 'start', run: remove },
        { label: 'Retry', place: 'primary', run: retry },
      ],
    });
    const foot = one(target, '.dl-dialog__foot');
    expect(one(foot, '.dl-dialog__foot-start').textContent).toContain('Remove task');
    const buttons = all(foot, ':scope > button, :scope > a');
    expect(buttons.map((b) => b.textContent?.trim())).toEqual(['Close', 'Retry']);
    const primary = buttons[1] as HTMLElement;
    expect(primary.classList).toContain('dl-btn--primary');
    expect(document.activeElement).toBe(primary);
    click(primary);
    expect(retry).toHaveBeenCalledOnce();
    expect(onclose).toHaveBeenCalledOnce();
  });

  it('offers a link as an action', () => {
    const onclose = vi.fn();
    const { target } = render(ErrorDialog, {
      ...text,
      labels,
      onclose,
      actions: [{ label: 'Open Settings → Downloads', place: 'primary', href: '#/settings/downloads', icon: Settings }],
    });
    const link = one<HTMLAnchorElement>(target, '.dl-dialog__foot a.dl-btn');
    expect(link.getAttribute('href')).toBe('#/settings/downloads');
    link.addEventListener('click', (event) => event.preventDefault());
    click(link);
    expect(onclose).toHaveBeenCalledOnce();
  });

  it('closes with Close, the head button and Esc', () => {
    const onclose = vi.fn();
    const { target } = render(ErrorDialog, { ...text, labels, onclose });
    click(one(target, '.dl-dialog__head button'));
    expect(one(target, '.dl-dialog__head button').getAttribute('aria-label')).toBe('Close (Esc)');
    const close = all(target, '.dl-dialog__foot button').find((b) => b.textContent?.trim() === 'Close');
    click(close as HTMLElement);
    keydown(one(target, '.dl-dialog'), 'Escape');
    expect(onclose).toHaveBeenCalledTimes(3);
  });
});
