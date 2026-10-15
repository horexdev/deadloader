import Download from '@lucide/svelte/icons/download';
import { createRawSnippet, flushSync } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import { click, one, render } from '../test/render';
import Button from './Button.svelte';

describe('Button', () => {
  it('renders a native button with its label and the secondary look by default', () => {
    const { target } = render(Button, { label: 'Check for updates' });
    const button = one<HTMLButtonElement>(target, 'button');
    expect(button.type).toBe('button');
    expect(button.className).toBe('dl-btn');
    expect(button.textContent.trim()).toBe('Check for updates');
  });

  it.each([
    ['primary', 'dl-btn--primary'],
    ['ghost', 'dl-btn--ghost'],
    ['danger', 'dl-btn--danger'],
    ['play', 'dl-btn--play'],
  ] as const)('maps the %s variant to %s', (variant, cls) => {
    const { target } = render(Button, { label: 'Go', variant });
    expect(one(target, 'button').classList).toContain(cls);
  });

  it('maps the sizes to the design classes', () => {
    const sm = render(Button, { label: 'Undo', size: 'sm' });
    const lg = render(Button, { label: 'Next', size: 'lg' });
    expect(one(sm.target, 'button').classList).toContain('dl-btn--sm');
    expect(one(lg.target, 'button').classList).toContain('dl-btn--lg');
  });

  it('renders the leading icon hidden from assistive technology', () => {
    const { target } = render(Button, { label: 'Install', icon: Download });
    const icon = one(target, 'button > svg');
    expect(icon.classList).toContain('dl-icon');
    expect(icon.getAttribute('aria-hidden')).toBe('true');
  });

  it('names an icon-only button through aria-label and shows no text', () => {
    const { target } = render(Button, { label: 'Open folder', icon: Download, iconOnly: true });
    const button = one(target, 'button');
    expect(button.classList).toContain('dl-btn--icon');
    expect(button.getAttribute('aria-label')).toBe('Open folder');
    expect(button.textContent.trim()).toBe('');
  });

  it('calls onclick when pressed', () => {
    const onclick = vi.fn();
    const { target } = render(Button, { label: 'Retry', onclick });
    click(one(target, 'button'));
    expect(onclick).toHaveBeenCalledOnce();
  });

  it('shows a spinner and ignores presses while busy', () => {
    const onclick = vi.fn();
    const { target } = render(Button, { label: 'Installing…', icon: Download, busy: true, onclick });
    const button = one(target, 'button');
    expect(button.getAttribute('aria-busy')).toBe('true');
    expect(button.querySelector('.dl-spinner')).not.toBeNull();
    click(button);
    expect(onclick).not.toHaveBeenCalled();
  });

  it('stays focusable but inert when unavailable (aria-disabled), so its tooltip can explain why', () => {
    const onclick = vi.fn();
    const { target } = render(Button, { label: 'Play', variant: 'play', unavailable: true, onclick });
    const button = one<HTMLButtonElement>(target, 'button');
    expect(button.getAttribute('aria-disabled')).toBe('true');
    expect(button.disabled).toBe(false);
    button.focus();
    expect(document.activeElement).toBe(button);
    click(button);
    expect(onclick).not.toHaveBeenCalled();
  });

  it('uses the native disabled attribute when disabled', () => {
    const { target } = render(Button, { label: 'Apply', disabled: true });
    expect(one<HTMLButtonElement>(target, 'button').disabled).toBe(true);
  });

  it('marks the error state for the retry look', () => {
    const { target } = render(Button, { label: 'Retry download', error: true });
    expect(one(target, 'button').dataset['state']).toBe('error');
  });

  it('renders trailing content such as a count after the label', () => {
    const children = createRawSnippet(() => ({ render: () => '<span class="dl-count">3</span>' }));
    const { target } = render(Button, { label: 'Pending changes', children });
    const button = one(target, 'button');
    expect(button.lastElementChild?.className).toBe('dl-count');
  });

  it('passes other attributes through to the button', () => {
    const { target } = render(Button, { label: 'Install', title: 'Install the mod', type: 'submit' });
    const button = one<HTMLButtonElement>(target, 'button');
    expect(button.title).toBe('Install the mod');
    expect(button.type).toBe('submit');
    flushSync();
  });
});
