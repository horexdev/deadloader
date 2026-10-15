// Form controls: Checkbox, Toggle, TextInput, SearchField.
import { flushSync } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import { click, fire, keydown, one, render } from '../test/render';
import Checkbox from './Checkbox.svelte';
import SearchField from './SearchField.svelte';
import TextInput from './TextInput.svelte';
import Toggle from './Toggle.svelte';

describe('Checkbox', () => {
  it('is a native checkbox (Space toggles it) named by its label', () => {
    const { target } = render(Checkbox, { label: 'Also remove foreign files' });
    const label = one(target, 'label.dl-check');
    const input = one<HTMLInputElement>(label, 'input.dl-check__input');
    expect(input.type).toBe('checkbox');
    expect(input.tabIndex).toBe(0);
    expect(label.textContent.trim()).toBe('Also remove foreign files');
    expect(one(label, '.dl-check__box .dl-check__tick')).toBeTruthy();
    expect(one(label, '.dl-check__box .dl-check__dash')).toBeTruthy();
  });

  it('reports changes and keeps the bound value', () => {
    const onchange = vi.fn();
    const props = $state({ label: 'Agree', checked: false, onchange });
    const { target } = render(Checkbox, props);
    click(one(target, 'input'));
    expect(props.checked).toBe(true);
    expect(onchange).toHaveBeenCalledWith(true);
  });

  it('shows the indeterminate state on the input', () => {
    const props = $state({ label: 'Select all', indeterminate: true });
    const { target } = render(Checkbox, props);
    const input = one<HTMLInputElement>(target, 'input');
    expect(input.indeterminate).toBe(true);
    props.indeterminate = false;
    flushSync();
    expect(input.indeterminate).toBe(false);
  });

  it('wires an error message to the input', () => {
    const { target } = render(Checkbox, { label: 'I understand', error: 'Tick this box to continue.' });
    const input = one(target, 'input');
    expect(input.getAttribute('aria-invalid')).toBe('true');
    const error = one(target, '.dl-field__error');
    expect(input.getAttribute('aria-describedby')).toBe(error.id);
    expect(error.textContent.trim()).toBe('Tick this box to continue.');
  });

  it('can be disabled', () => {
    const { target } = render(Checkbox, { label: 'x', disabled: true });
    expect(one<HTMLInputElement>(target, 'input').disabled).toBe(true);
  });
});

describe('Toggle', () => {
  it('is a native checkbox with the switch role, named by its label', () => {
    const { target } = render(Toggle, { label: 'Auto-fix gameinfo.gi' });
    const input = one<HTMLInputElement>(target, 'label.dl-switch input.dl-switch__input');
    expect(input.type).toBe('checkbox');
    expect(input.getAttribute('role')).toBe('switch');
    expect(one(target, '.dl-switch').textContent.trim()).toBe('Auto-fix gameinfo.gi');
    expect(one(target, '.dl-switch__track .dl-switch__thumb')).toBeTruthy();
  });

  it('takes an aria-label instead of a visible label inside rows', () => {
    const { target } = render(Toggle, { ariaLabel: 'Enabled' });
    expect(one(target, 'input').getAttribute('aria-label')).toBe('Enabled');
  });

  it('flips at once and reports the new state', () => {
    const onchange = vi.fn();
    const props = $state({ ariaLabel: 'Enabled', checked: true, onchange });
    const { target } = render(Toggle, props);
    click(one(target, 'input'));
    expect(props.checked).toBe(false);
    expect(onchange).toHaveBeenCalledWith(false);
  });

  it('shows a spinner and refuses to flip while busy', () => {
    const onchange = vi.fn();
    const props = $state({ ariaLabel: 'Enabled', checked: true, busy: true, onchange });
    const { target } = render(Toggle, props);
    const label = one(target, '.dl-switch');
    expect(label.getAttribute('aria-busy')).toBe('true');
    expect(one(target, 'input').getAttribute('aria-busy')).toBe('true');
    expect(one(target, '.dl-switch__thumb .dl-spinner')).toBeTruthy();
    click(one(target, 'input'));
    expect(props.checked).toBe(true);
    expect(onchange).not.toHaveBeenCalled();
  });

  it('shows the error state with a message', () => {
    const { target } = render(Toggle, { label: 'x', error: 'Could not apply. Details: L-12.' });
    expect(one(target, '.dl-switch').classList).toContain('is-error');
    const input = one(target, 'input');
    expect(input.getAttribute('aria-invalid')).toBe('true');
    expect(input.getAttribute('aria-describedby')).toBe(one(target, '.dl-field__error').id);
  });

  it('can be disabled', () => {
    const { target } = render(Toggle, { label: 'x', disabled: true });
    expect(one<HTMLInputElement>(target, 'input').disabled).toBe(true);
  });
});

describe('TextInput', () => {
  it('labels the input and binds its value', () => {
    const props = $state({ label: 'GameBanana link', value: '' });
    const { target } = render(TextInput, props);
    const input = one<HTMLInputElement>(target, '.dl-field .dl-input input');
    const label = one<HTMLLabelElement>(target, 'label.dl-field__label');
    expect(label.htmlFor).toBe(input.id);
    expect(label.textContent).toBe('GameBanana link');
    input.value = 'https://gamebanana.com/mods/1';
    fire(input, 'input');
    expect(props.value).toBe('https://gamebanana.com/mods/1');
  });

  it('describes the input with its hint and error', () => {
    const { target } = render(TextInput, {
      label: 'Link',
      value: 'x',
      hint: 'Paste a mod page link.',
      error: 'This is not a Deadlock mod page.',
    });
    const input = one(target, 'input');
    const hint = one(target, '.dl-field__hint');
    const error = one(target, '.dl-field__error');
    expect(input.getAttribute('aria-invalid')).toBe('true');
    expect(input.getAttribute('aria-describedby')).toBe(`${hint.id} ${error.id}`);
  });

  it('has the mono, read-only, disabled and busy states', () => {
    const { target } = render(TextInput, { label: 'Path', value: 'D:\\x', mono: true, readonly: true, busy: true });
    const box = one(target, '.dl-input');
    expect(box.classList).toContain('dl-input--mono');
    expect(box.getAttribute('aria-busy')).toBe('true');
    expect(one(box, '.dl-spinner')).toBeTruthy();
    expect(one<HTMLInputElement>(box, 'input').readOnly).toBe(true);
    const disabled = render(TextInput, { label: 'x', value: '', disabled: true });
    expect(one<HTMLInputElement>(disabled.target, 'input').disabled).toBe(true);
  });
});

describe('SearchField', () => {
  const base = {
    label: 'Search mods',
    placeholder: 'Search mods',
    clearLabel: 'Clear search',
  };

  it('is a labelled search input with the shortcut hint while empty', () => {
    const { target } = render(SearchField, { ...base, value: '', shortcut: '/' });
    const input = one<HTMLInputElement>(target, '.dl-search .dl-input input');
    expect(input.type).toBe('search');
    expect(input.getAttribute('aria-label')).toBe('Search mods');
    expect(input.placeholder).toBe('Search mods');
    expect(one(target, '.dl-kbd').textContent).toBe('/');
    expect(target.querySelector('.dl-search__clear')).toBeNull();
  });

  it('shows the result count and the clear button once there is text', () => {
    const props = $state({ ...base, value: 'haze', resultText: '126 results', shortcut: '/' });
    const { target } = render(SearchField, props);
    expect(target.querySelector('.dl-kbd')).toBeNull();
    expect(one(target, '.dl-search__meta').textContent).toBe('126 results');
    expect(one(target, '.dl-search__clear').getAttribute('aria-label')).toBe('Clear search');
  });

  it('debounces searches while typing', () => {
    vi.useFakeTimers();
    try {
      const onsearch = vi.fn();
      const props = $state({ ...base, value: '', onsearch });
      const { target } = render(SearchField, props);
      const input = one<HTMLInputElement>(target, 'input');
      input.value = 'ha';
      fire(input, 'input');
      input.value = 'haze';
      fire(input, 'input');
      expect(onsearch).not.toHaveBeenCalled();
      vi.advanceTimersByTime(299);
      expect(onsearch).not.toHaveBeenCalled();
      vi.advanceTimersByTime(1);
      expect(onsearch).toHaveBeenCalledOnce();
      expect(onsearch).toHaveBeenCalledWith('haze');
    } finally {
      vi.useRealTimers();
    }
  });

  it('clears with the button, at once, and keeps focus in the field', () => {
    const onsearch = vi.fn();
    const props = $state({ ...base, value: 'haze', onsearch });
    const { target } = render(SearchField, props);
    click(one(target, '.dl-search__clear'));
    expect(props.value).toBe('');
    expect(onsearch).toHaveBeenCalledWith('');
    expect(document.activeElement).toBe(one(target, 'input'));
  });

  it('clears on the first Esc and leaves the field on the second', () => {
    const props = $state({ ...base, value: 'haze' });
    const { target } = render(SearchField, props);
    const input = one<HTMLInputElement>(target, 'input');
    input.focus();
    keydown(input, 'Escape');
    expect(props.value).toBe('');
    expect(document.activeElement).toBe(input);
    keydown(input, 'Escape');
    expect(document.activeElement).not.toBe(input);
  });

  it('shows the busy, error and disabled states', () => {
    const busy = render(SearchField, { ...base, value: 'haze', busy: true, resultText: 'Searching…' });
    expect(one(busy.target, '.dl-input').getAttribute('aria-busy')).toBe('true');
    expect(one(busy.target, '.dl-spinner')).toBeTruthy();
    const error = render(SearchField, { ...base, value: 'zzz', error: true });
    expect(one(error.target, '.dl-input').classList).toContain('is-error');
    expect(one(error.target, '.dl-input > svg').classList).toContain('lucide-search-x');
    const off = render(SearchField, { ...base, value: '', disabled: true, hint: 'The catalog is off.' });
    expect(one(off.target, '.dl-input').classList).toContain('is-disabled');
    const input = one<HTMLInputElement>(off.target, 'input');
    expect(input.disabled).toBe(true);
    expect(input.getAttribute('aria-describedby')).toBe(one(off.target, '.dl-field__hint').id);
  });

  it('can be focused from outside, for the Ctrl+F and / shortcuts', () => {
    const { target, instance } = render(SearchField, { ...base, value: '' });
    instance.focus();
    expect(document.activeElement).toBe(one(target, 'input'));
  });
});
