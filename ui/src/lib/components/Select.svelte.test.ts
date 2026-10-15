import { flushSync, type ComponentProps } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import { all, click, fire, keydown, one, render } from '../test/render';
import Select from './Select.svelte';

const options = [
  { value: 'liked', label: 'Most liked' },
  { value: 'downloaded', label: 'Most downloaded' },
  { value: 'new', label: 'Newest' },
  { value: 'old', label: 'Oldest', disabled: true },
  { value: 'updated', label: 'Recently updated', count: '1 204' },
];

type SelectProps = ComponentProps<typeof Select>;

function setup(extra: Partial<SelectProps> = {}) {
  const props: SelectProps = $state({ options, value: 'liked', label: 'Sort', prefix: 'Sort', ...extra });
  const rendered = render(Select, props);
  const trigger = one<HTMLButtonElement>(rendered.target, '.dl-select__trigger');
  return { ...rendered, props, trigger };
}

const menu = () => document.querySelector<HTMLElement>('.dl-menu');
const activeOption = (trigger: HTMLElement) => {
  const id = trigger.getAttribute('aria-activedescendant');
  return id ? document.getElementById(id) : null;
};

describe('Select', () => {
  it('renders a collapsed select-only combobox showing the prefix and the selected label', () => {
    const { trigger } = setup();
    expect(trigger.getAttribute('role')).toBe('combobox');
    expect(trigger.getAttribute('aria-haspopup')).toBe('listbox');
    expect(trigger.getAttribute('aria-expanded')).toBe('false');
    expect(trigger.getAttribute('aria-label')).toBe('Sort');
    expect(one(trigger, '.dl-select__prefix').textContent).toBe('Sort');
    expect(one(trigger, '.dl-select__value').textContent).toBe('Most liked');
    expect(menu()).toBeNull();
  });

  it('opens with ArrowDown, Enter or Space on the selected option', () => {
    for (const key of ['ArrowDown', 'Enter', ' ']) {
      const { trigger, target } = setup({ value: 'new' });
      trigger.focus();
      keydown(trigger, key);
      expect(trigger.getAttribute('aria-expanded')).toBe('true');
      const list = one(target, '.dl-menu');
      expect(list.getAttribute('role')).toBe('listbox');
      expect(trigger.getAttribute('aria-controls')).toBe(list.id);
      expect(activeOption(trigger)?.textContent).toContain('Newest');
      expect(activeOption(trigger)?.dataset['active']).toBe('true');
      target.remove();
    }
  });

  it('marks the selected option and shows counts', () => {
    const { trigger } = setup({ value: 'updated' });
    click(trigger);
    const opts = all(menu() as HTMLElement, '.dl-option');
    expect(opts.map((o) => o.getAttribute('role'))).toEqual(Array(5).fill('option'));
    expect(opts.map((o) => o.getAttribute('aria-selected'))).toEqual(['false', 'false', 'false', 'false', 'true']);
    expect(opts[3]?.getAttribute('aria-disabled')).toBe('true');
    expect(one(opts[4] as HTMLElement, '.dl-option__count').textContent).toBe('1 204');
  });

  it('moves the active option with the arrows (skipping disabled ones), Home and End, and selects with Enter', () => {
    const onchange = vi.fn();
    const { trigger, props } = setup({ onchange });
    trigger.focus();
    keydown(trigger, 'ArrowDown');
    keydown(trigger, 'ArrowDown');
    keydown(trigger, 'ArrowDown');
    expect(activeOption(trigger)?.textContent).toContain('Newest');
    keydown(trigger, 'ArrowDown');
    expect(activeOption(trigger)?.textContent).toContain('Recently updated');
    keydown(trigger, 'ArrowDown');
    expect(activeOption(trigger)?.textContent).toContain('Recently updated');
    keydown(trigger, 'Home');
    expect(activeOption(trigger)?.textContent).toContain('Most liked');
    keydown(trigger, 'End');
    keydown(trigger, 'ArrowUp');
    expect(activeOption(trigger)?.textContent).toContain('Newest');
    keydown(trigger, 'Enter');
    expect(props.value).toBe('new');
    expect(onchange).toHaveBeenCalledWith('new');
    expect(trigger.getAttribute('aria-expanded')).toBe('false');
    expect(document.activeElement).toBe(trigger);
  });

  it('jumps by type-ahead', () => {
    const { trigger } = setup();
    trigger.focus();
    keydown(trigger, 'ArrowDown');
    keydown(trigger, 'r');
    expect(activeOption(trigger)?.textContent).toContain('Recently updated');
  });

  it('closes with Esc without changing the value and keeps focus on the trigger', () => {
    const { trigger, props } = setup();
    trigger.focus();
    keydown(trigger, 'ArrowDown');
    keydown(trigger, 'ArrowDown');
    keydown(trigger, 'Escape');
    expect(trigger.getAttribute('aria-expanded')).toBe('false');
    expect(props.value).toBe('liked');
    expect(document.activeElement).toBe(trigger);
  });

  it('selects an option by click and ignores disabled ones', () => {
    const { trigger, props } = setup();
    click(trigger);
    click(all(menu() as HTMLElement, '.dl-option')[3] as HTMLElement);
    expect(props.value).toBe('liked');
    expect(menu()).not.toBeNull();
    click(all(menu() as HTMLElement, '.dl-option')[2] as HTMLElement);
    expect(props.value).toBe('new');
    expect(menu()).toBeNull();
  });

  it('closes when the pointer goes down outside', () => {
    const { trigger } = setup();
    click(trigger);
    expect(menu()).not.toBeNull();
    fire(document.body, 'pointerdown');
    expect(menu()).toBeNull();
  });

  it('shows the busy state in the trigger and in an open menu', () => {
    const { trigger, props } = setup({ busy: true, busyText: 'Loading heroes…' });
    expect(trigger.getAttribute('aria-busy')).toBe('true');
    expect(one(trigger, '.dl-select__value').textContent).toBe('Loading heroes…');
    expect(one(trigger, '.dl-spinner')).toBeTruthy();
    click(trigger);
    expect(one(menu() as HTMLElement, '.dl-menu__status').textContent).toContain('Loading heroes…');
    expect((menu() as HTMLElement).querySelector('.dl-option')).toBeNull();
    props.busy = false;
    flushSync();
    expect(all(menu() as HTMLElement, '.dl-option')).toHaveLength(5);
  });

  it('shows the error state and can be disabled', () => {
    const { trigger, target } = setup({ error: 'Choose a game installation.' });
    expect(trigger.getAttribute('aria-invalid')).toBe('true');
    expect(trigger.getAttribute('aria-describedby')).toBe(one(target, '.dl-field__error').id);
    const off = setup({ disabled: true });
    expect(off.trigger.disabled).toBe(true);
  });

  it('shows a placeholder when nothing is selected', () => {
    const { trigger } = setup({ value: '', placeholder: 'Any hero' });
    expect(one(trigger, '.dl-select__value').textContent).toBe('Any hero');
  });

  it('renders group headings as labelled groups', () => {
    const grouped = [
      { value: 'a', label: 'Abrams', group: 'Heroes' },
      { value: 'b', label: 'Bebop', group: 'Heroes' },
      { value: 'hud', label: 'HUD', group: 'Other' },
    ];
    const { trigger } = setup({ options: grouped, value: 'a' });
    click(trigger);
    const groups = all(menu() as HTMLElement, '[role="group"]');
    expect(groups).toHaveLength(2);
    const heading = document.getElementById(groups[1]?.getAttribute('aria-labelledby') ?? '');
    expect(heading?.textContent).toBe('Other');
    expect(heading?.classList).toContain('dl-menu__group');
    expect(all(menu() as HTMLElement, '.dl-menu__sep')).toHaveLength(1);
  });

  it('filters long lists through an input at the top of the menu', () => {
    const { trigger, props } = setup({ filterLabel: 'Filter heroes', emptyText: 'No match' });
    click(trigger);
    const filter = one<HTMLInputElement>(menu() as HTMLElement, 'input');
    expect(document.activeElement).toBe(filter);
    expect(filter.getAttribute('aria-label')).toBe('Filter heroes');
    expect(filter.getAttribute('aria-controls')).toBe((menu() as HTMLElement).querySelector('[role="listbox"]')?.id);
    filter.value = 'most';
    fire(filter, 'input');
    expect(all(menu() as HTMLElement, '.dl-option').map((o) => o.textContent.trim())).toEqual([
      'Most liked',
      'Most downloaded',
    ]);
    keydown(filter, 'ArrowDown');
    const id = filter.getAttribute('aria-activedescendant') ?? '';
    expect(document.getElementById(id)?.textContent).toContain('Most downloaded');
    keydown(filter, 'Enter');
    expect(props.value).toBe('downloaded');
    expect(document.activeElement).toBe(trigger);
    click(trigger);
    const again = one<HTMLInputElement>(menu() as HTMLElement, 'input');
    again.value = 'zzz';
    fire(again, 'input');
    expect(one(menu() as HTMLElement, '.dl-menu__status').textContent).toContain('No match');
  });
});
