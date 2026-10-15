import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
import { createRawSnippet } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import { destroy, keydown, one, render } from '../test/render';
import Dialog from './Dialog.svelte';

const body = createRawSnippet(() => ({ render: () => '<p>2 versions · 18.4 MB. Used in 2 profiles.</p>' }));
const footer = createRawSnippet(() => ({
  render: () => '<div class="buttons"><button class="cancel" data-autofocus>Cancel</button><button class="go">Delete mod</button></div>',
}));
const plainFooter = createRawSnippet(() => ({
  render: () => '<div class="buttons"><button class="cancel">Cancel</button><button class="go">Open</button></div>',
}));

function opener(): HTMLButtonElement {
  const button = document.createElement('button');
  button.textContent = 'Delete';
  document.body.append(button);
  button.focus();
  return button;
}

describe('Dialog', () => {
  it('is a labelled, described modal dialog', () => {
    const { target } = render(Dialog, { title: 'Open this link?', children: body, footer: plainFooter, oncancel: vi.fn() });
    const dialog = one(target, '.dl-dialog');
    expect(dialog.getAttribute('role')).toBe('dialog');
    expect(dialog.getAttribute('aria-modal')).toBe('true');
    const title = one(dialog, 'h2.dl-dialog__title');
    expect(title.textContent).toBe('Open this link?');
    expect(dialog.getAttribute('aria-labelledby')).toBe(title.id);
    expect(dialog.getAttribute('aria-describedby')).toBe(one(dialog, '.dl-dialog__body').id);
    expect(target.querySelector('.dl-scrim')).not.toBeNull();
    expect(one(dialog, '.dl-dialog__foot .go')).toBeTruthy();
  });

  it('uses the alertdialog role, the danger look and the size classes', () => {
    const { target } = render(Dialog, {
      title: 'Delete “Faun Celeste”?',
      kind: 'danger',
      size: 'md',
      icon: TriangleAlert,
      children: body,
      oncancel: vi.fn(),
    });
    const dialog = one(target, '.dl-dialog');
    expect(dialog.getAttribute('role')).toBe('alertdialog');
    expect(dialog.classList).toContain('dl-dialog--danger');
    expect(dialog.classList).toContain('dl-dialog--md');
    expect(one(dialog, '.dl-dialog__head svg.dl-dialog__icon').getAttribute('aria-hidden')).toBe('true');
  });

  it('focuses the element marked data-autofocus, else the first focusable one', () => {
    render(Dialog, { title: 'Delete?', kind: 'danger', children: body, footer, oncancel: vi.fn() });
    expect((document.activeElement as HTMLElement).className).toBe('cancel');
  });

  it('falls back to the first focusable element', () => {
    render(Dialog, { title: 'Open?', children: body, footer: plainFooter, oncancel: vi.fn() });
    expect((document.activeElement as HTMLElement).className).toBe('cancel');
  });

  it('traps Tab and Shift+Tab inside', () => {
    const { target } = render(Dialog, { title: 'Open?', children: body, footer: plainFooter, oncancel: vi.fn() });
    const cancel = one(target, '.cancel');
    const go = one(target, '.go');
    go.focus();
    expect(keydown(go, 'Tab')).toBe(false);
    expect(document.activeElement).toBe(cancel);
    expect(keydown(cancel, 'Tab', { shiftKey: true })).toBe(false);
    expect(document.activeElement).toBe(go);
    // Between the ends the browser moves focus itself.
    cancel.focus();
    expect(keydown(cancel, 'Tab')).toBe(true);
  });

  it('cancels on Esc unless that is turned off', () => {
    const oncancel = vi.fn();
    const props = $state({ title: 'Open?', children: body, footer: plainFooter, oncancel, closeOnEscape: true });
    const { target } = render(Dialog, props);
    keydown(one(target, '.cancel'), 'Escape');
    expect(oncancel).toHaveBeenCalledOnce();
    props.closeOnEscape = false;
    keydown(one(target, '.cancel'), 'Escape');
    expect(oncancel).toHaveBeenCalledOnce();
  });

  it('leaves Esc to a control inside that already handled it', () => {
    const oncancel = vi.fn();
    const { target } = render(Dialog, { title: 'Open?', children: body, footer: plainFooter, oncancel });
    const cancel = one(target, '.cancel');
    cancel.addEventListener('keydown', (event) => event.preventDefault());
    keydown(cancel, 'Escape');
    expect(oncancel).not.toHaveBeenCalled();
  });

  it('returns focus to the element that opened it', () => {
    const button = opener();
    const { instance } = render(Dialog, { title: 'Delete?', children: body, footer, oncancel: vi.fn() });
    expect(document.activeElement).not.toBe(button);
    destroy(instance);
    expect(document.activeElement).toBe(button);
  });

  it('has a close button in the head when it is given a label', () => {
    const oncancel = vi.fn();
    const plain = render(Dialog, { title: 'Open?', children: body, footer: plainFooter, oncancel });
    expect(plain.target.querySelector('.dl-dialog__head button')).toBeNull();
    const { target } = render(Dialog, { title: 'x', children: body, footer, oncancel, closeLabel: 'Close (Esc)' });
    const close = one<HTMLButtonElement>(target, '.dl-dialog__head button');
    expect(close.getAttribute('aria-label')).toBe('Close (Esc)');
    expect(close.title).toBe('Close (Esc)');
    close.click();
    expect(oncancel).toHaveBeenCalledOnce();
  });

  it('puts secondary actions at the start of the footer', () => {
    const footerStart = createRawSnippet(() => ({ render: () => '<button class="folder">Open folder</button>' }));
    const { target } = render(Dialog, { title: 'x', children: body, footer, footerStart, oncancel: vi.fn() });
    expect(one(target, '.dl-dialog__foot .dl-dialog__foot-start .folder')).toBeTruthy();
  });
});
