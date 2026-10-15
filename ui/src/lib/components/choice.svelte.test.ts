// One-of-many controls: Tabs and SegmentedControl.
import { createRawSnippet } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import { all, click, keydown, one, render } from '../test/render';
import SegmentedControl from './SegmentedControl.svelte';
import Tabs from './Tabs.svelte';

const tabs = [
  { id: 'network', label: 'Network', count: '972' },
  { id: 'files', label: 'Files' },
  { id: 'app', label: 'App', count: '2 errors', countError: true },
];

describe('Tabs', () => {
  it('renders a labelled tablist with one selected tab in the tab order', () => {
    const { target } = render(Tabs, { tabs, selected: 'files', label: 'Activity' });
    const list = one(target, '.dl-tabs');
    expect(list.getAttribute('role')).toBe('tablist');
    expect(list.getAttribute('aria-label')).toBe('Activity');
    const buttons = all(list, 'button.dl-tab');
    expect(buttons.map((b) => b.getAttribute('role'))).toEqual(['tab', 'tab', 'tab']);
    expect(buttons.map((b) => b.getAttribute('aria-selected'))).toEqual(['false', 'true', 'false']);
    expect(buttons.map((b) => b.tabIndex)).toEqual([-1, 0, -1]);
  });

  it('shows counts, with the error colour when asked', () => {
    const { target } = render(Tabs, { tabs, selected: 'network', label: 'Activity' });
    const counts = all(target, '.dl-tab__count');
    expect(counts.map((c) => c.textContent)).toEqual(['972', '2 errors']);
    expect(counts[1]?.classList).toContain('dl-tab__count--error');
  });

  it('moves and selects with the arrow keys, wrapping, and Home/End', () => {
    const onchange = vi.fn();
    const props = $state({ tabs, selected: 'network', label: 'Activity', onchange });
    const { target } = render(Tabs, props);
    const [first, , last] = all(target, 'button.dl-tab');
    if (!first || !last) throw new Error('tabs missing');
    first.focus();
    keydown(first, 'ArrowLeft');
    expect(props.selected).toBe('app');
    expect(document.activeElement).toBe(last);
    keydown(last, 'ArrowRight');
    expect(props.selected).toBe('network');
    keydown(first, 'End');
    expect(props.selected).toBe('app');
    keydown(last, 'Home');
    expect(props.selected).toBe('network');
    expect(onchange).toHaveBeenLastCalledWith('network');
  });

  it('selects on click', () => {
    const props = $state({ tabs, selected: 'network', label: 'Activity' });
    const { target } = render(Tabs, props);
    click(all(target, 'button.dl-tab')[1] as HTMLElement);
    expect(props.selected).toBe('files');
  });

  it('skips disabled tabs and ignores clicks on them', () => {
    const withDisabled = [{ id: 'a', label: 'A' }, { id: 'b', label: 'B', disabled: true }, { id: 'c', label: 'C' }];
    const props = $state({ tabs: withDisabled, selected: 'a', label: 'x' });
    const { target } = render(Tabs, props);
    const buttons = all(target, 'button.dl-tab');
    expect(buttons[1]?.getAttribute('aria-disabled')).toBe('true');
    click(buttons[1] as HTMLElement);
    expect(props.selected).toBe('a');
    (buttons[0] as HTMLElement).focus();
    keydown(buttons[0] as HTMLElement, 'ArrowRight');
    expect(props.selected).toBe('c');
  });

  it('renders the selected panel, linked to its tab', () => {
    const panel = createRawSnippet((id: () => string) => ({ render: () => `<p>panel ${id()}</p>` }));
    const { target } = render(Tabs, { tabs, selected: 'files', label: 'Activity', panel });
    const region = one(target, '[role="tabpanel"]');
    const tab = all(target, 'button.dl-tab')[1] as HTMLElement;
    expect(region.textContent).toBe('panel files');
    expect(region.getAttribute('aria-labelledby')).toBe(tab.id);
    expect(tab.getAttribute('aria-controls')).toBe(region.id);
  });
});

const options = [
  { value: 'hidden', label: 'Hidden' },
  { value: 'blurred', label: 'Blurred' },
  { value: 'shown', label: 'Shown' },
];

describe('SegmentedControl', () => {
  it('is a labelled radiogroup with roving tabindex', () => {
    const { target } = render(SegmentedControl, { options, value: 'blurred', label: 'NSFW' });
    const group = one(target, '.dl-seg');
    expect(group.getAttribute('role')).toBe('radiogroup');
    expect(group.getAttribute('aria-label')).toBe('NSFW');
    const items = all(group, 'button.dl-seg__item');
    expect(items.map((i) => i.getAttribute('role'))).toEqual(['radio', 'radio', 'radio']);
    expect(items.map((i) => i.getAttribute('aria-checked'))).toEqual(['false', 'true', 'false']);
    expect(items.map((i) => i.tabIndex)).toEqual([-1, 0, -1]);
  });

  it('moves and selects with all four arrow keys', () => {
    const props = $state({ options, value: 'hidden', label: 'NSFW' });
    const { target } = render(SegmentedControl, props);
    const items = all(target, 'button.dl-seg__item');
    (items[0] as HTMLElement).focus();
    keydown(items[0] as HTMLElement, 'ArrowRight');
    expect(props.value).toBe('blurred');
    expect(document.activeElement).toBe(items[1]);
    keydown(items[1] as HTMLElement, 'ArrowDown');
    expect(props.value).toBe('shown');
    keydown(items[2] as HTMLElement, 'ArrowDown');
    expect(props.value).toBe('hidden');
    keydown(items[0] as HTMLElement, 'ArrowUp');
    expect(props.value).toBe('shown');
    keydown(items[2] as HTMLElement, 'ArrowLeft');
    expect(props.value).toBe('blurred');
  });

  it('lets the consumer hold a choice back until it is confirmed', () => {
    const onchange = vi.fn((next: string) => (next === 'shown' ? false : undefined));
    const props = $state({ options, value: 'hidden', label: 'NSFW', onchange });
    const { target } = render(SegmentedControl, props);
    const items = all(target, 'button.dl-seg__item');
    click(items[2] as HTMLElement);
    expect(onchange).toHaveBeenCalledWith('shown');
    expect(props.value).toBe('hidden');
    click(items[1] as HTMLElement);
    expect(props.value).toBe('blurred');
  });

  it('shows busy and error states', () => {
    const { target } = render(SegmentedControl, {
      options,
      value: 'shown',
      label: 'NSFW',
      busy: true,
      error: 'Could not save the setting.',
    });
    const group = one(target, '.dl-seg');
    expect(group.getAttribute('aria-busy')).toBe('true');
    expect(group.classList).toContain('is-error');
    expect(one(target, 'button[aria-checked="true"] .dl-spinner')).toBeTruthy();
    expect(group.getAttribute('aria-describedby')).toBe(one(target, '.dl-field__error').id);
  });

  it('disables single options or the whole control', () => {
    const some = render(SegmentedControl, {
      options: [...options.slice(0, 2), { value: 'shown', label: 'Shown', disabled: true }],
      value: 'hidden',
      label: 'x',
    });
    expect(all<HTMLButtonElement>(some.target, 'button').map((b) => b.disabled)).toEqual([false, false, true]);
    const whole = render(SegmentedControl, { options, value: 'hidden', label: 'x', disabled: true });
    expect(all<HTMLButtonElement>(whole.target, 'button').every((b) => b.disabled)).toBe(true);
  });
});
