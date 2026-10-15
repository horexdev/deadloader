import GitCompareArrows from '@lucide/svelte/icons/git-compare-arrows';
import Link from '@lucide/svelte/icons/link';
import { createRawSnippet, flushSync, type ComponentProps } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import { all, click, keydown, one, render } from '../test/render';
import LibraryRow from './LibraryRow.svelte';
import RowAddon from './RowAddon.svelte';
import RowParts from './RowParts.svelte';
import RowVariant from './RowVariant.svelte';

type RowProps = ComponentProps<typeof LibraryRow>;

const base: RowProps = {
  name: 'Faun Celeste',
  sub: ['basilisken', 'GameBanana', 'v1 · Bubblegum'],
  thumb: 'thumbs/00.jpg',
  enabled: true,
  toggleLabel: 'Enabled',
  gripLabel: 'Reorder Faun Celeste, position 1 of 12',
};

function setup(extra: Partial<RowProps> = {}) {
  const props: RowProps = $state({ ...base, ...extra });
  const rendered = render(LibraryRow, props);
  return { ...rendered, props, row: one(rendered.target, '.dl-row') };
}

describe('LibraryRow', () => {
  it('lays out grip, toggle, thumbnail, name and sub-line, badges and actions', () => {
    const actions = createRawSnippet(() => ({ render: () => '<button class="more">More</button>' }));
    const { row } = setup({
      badges: [{ icon: GitCompareArrows, label: 'Conflict · wins', variant: 'warning' }],
      actions,
    });
    expect(row.tabIndex).toBe(0);
    expect(row.getAttribute('role')).toBe('group');
    expect(document.getElementById(row.getAttribute('aria-labelledby') ?? '')?.textContent).toBe('Faun Celeste');
    expect(one(row, 'button.dl-row__grip').getAttribute('aria-label')).toBe('Reorder Faun Celeste, position 1 of 12');
    expect(one<HTMLInputElement>(row, '.dl-switch input').checked).toBe(true);
    expect(one(row, 'img.dl-row__thumb').getAttribute('alt')).toBe('');
    expect(one(row, '.dl-row__name').textContent).toBe('Faun Celeste');
    const sub = one(row, '.dl-row__sub');
    expect(sub.textContent).toBe('basilisken·GameBanana·v1 · Bubblegum');
    expect(all(sub, '.dl-row__sep')).toHaveLength(2);
    expect(one(row, '.dl-row__badges .dl-badge--warning').textContent).toBe('Conflict · wins');
    expect(one(row, '.dl-row__actions .more')).toBeTruthy();
  });

  it('shows at most two badges', () => {
    const badge = { icon: Link, label: 'Linked', variant: 'neutral' as const };
    const { row } = setup({ badges: [badge, { ...badge, label: 'Two' }, { ...badge, label: 'Three' }] });
    expect(all(row, '.dl-badge')).toHaveLength(2);
  });

  it('shows the local-mod placeholder instead of a thumbnail', () => {
    const { row } = setup({ thumb: null });
    expect(row.querySelector('img')).toBeNull();
    expect(one(row, '.dl-row__thumb svg')).toBeTruthy();
  });

  it('dims its content when off in this profile and flips with the toggle', () => {
    const ontoggle = vi.fn();
    const { row, props } = setup({ enabled: false, ontoggle });
    expect(row.classList).toContain('is-off');
    click(one(row, '.dl-switch input'));
    expect(ontoggle).toHaveBeenCalledWith(true);
    props.enabled = true;
    flushSync();
    expect(row.classList).not.toContain('is-off');
  });

  it('shows the busy state with a thin progress bar', () => {
    const { row } = setup({ busy: true, progress: 0.6, progressLabel: 'Copying Faun Celeste' });
    expect(row.getAttribute('aria-busy')).toBe('true');
    expect(one(row, '.dl-switch').getAttribute('aria-busy')).toBe('true');
    const bar = one(row, '.dl-row__progress .dl-progress--thin');
    expect(bar.getAttribute('aria-valuenow')).toBe('60');
    expect(bar.getAttribute('aria-label')).toBe('Copying Faun Celeste');
  });

  it('shows the error, selected, grabbed and dragging looks', () => {
    const { row, props } = setup({ error: true, selected: true });
    expect(row.classList).toContain('is-error');
    expect(row.classList).toContain('is-selected');
    props.grabbed = true;
    props.dragging = true;
    flushSync();
    expect(row.classList).toContain('is-grabbed');
    expect(row.classList).toContain('is-dragging');
    expect(one(row, '.dl-row__grip').getAttribute('aria-pressed')).toBe('true');
  });

  it('disables the toggle with a reason when changes cannot be applied', () => {
    const { row } = setup({ toggleDisabled: true, toggleTitle: 'Deadlock was not found' });
    expect(one<HTMLInputElement>(row, '.dl-switch input').disabled).toBe(true);
    expect(one(row, '.dl-switch').title).toBe('Deadlock was not found');
  });

  it('reorders by keyboard: Space picks up, arrows move, Space drops, Esc cancels', () => {
    const ongrab = vi.fn();
    const onmove = vi.fn();
    const ondrop = vi.fn();
    const oncancel = vi.fn();
    const { row, props } = setup({ ongrab, onmove, ondrop, oncancel });
    const grip = one(row, '.dl-row__grip');
    expect(keydown(grip, ' ')).toBe(false);
    expect(ongrab).toHaveBeenCalledOnce();
    props.grabbed = true;
    flushSync();
    expect(keydown(grip, 'ArrowDown')).toBe(false);
    expect(keydown(grip, 'ArrowUp')).toBe(false);
    expect(onmove.mock.calls).toEqual([[1], [-1]]);
    keydown(grip, 'Enter');
    expect(ondrop).toHaveBeenCalledOnce();
    keydown(grip, 'Escape');
    expect(oncancel).toHaveBeenCalledOnce();
  });

  it('moves one step with Alt+arrows without picking up, from the grip or the row', () => {
    const onmove = vi.fn();
    const { row } = setup({ onmove });
    keydown(one(row, '.dl-row__grip'), 'ArrowUp', { altKey: true });
    keydown(row, 'ArrowDown', { altKey: true });
    expect(onmove.mock.calls).toEqual([[-1], [1]]);
    // Plain arrows on a row that is not picked up belong to the list.
    expect(keydown(row, 'ArrowDown')).toBe(true);
  });

  it('opens the details with Enter on the row itself', () => {
    const onopen = vi.fn();
    const { row } = setup({ onopen });
    keydown(row, 'Enter');
    expect(onopen).toHaveBeenCalledOnce();
    keydown(one(row, '.dl-switch input'), 'Enter');
    expect(onopen).toHaveBeenCalledOnce();
  });

  it('takes its controls out of the tab order when another row holds the list tab stop', () => {
    const actions = createRawSnippet(() => ({ render: () => '<span></span>' }));
    const { row } = setup({ tabindex: -1, actions });
    expect(row.tabIndex).toBe(-1);
    expect(one(row, '.dl-row__grip').tabIndex).toBe(-1);
    expect(one(row, '.dl-switch input').tabIndex).toBe(-1);
  });

  it('wraps expanded parts in a block under the row', () => {
    const parts = createRawSnippet(() => ({ render: () => '<div class="dl-row-block__parts">parts</div>' }));
    const { target, props } = setup({ parts, expanded: false });
    expect(target.querySelector('.dl-row-block')).toBeNull();
    props.expanded = true;
    props.enabled = false;
    flushSync();
    const block = one(target, '.dl-row-block');
    expect(block.classList).toContain('is-off');
    expect(one(block, '.dl-row + .dl-row-block__parts').textContent).toBe('parts');
  });
});

describe('row parts', () => {
  it('groups variants as a labelled radiogroup with a head', () => {
    const children = createRawSnippet(() => ({ render: () => '<span>variants</span>' }));
    const { target } = render(RowParts, { label: '7 variants', meta: '2 downloaded', radio: true, children });
    const parts = one(target, '.dl-row-block__parts');
    expect(parts.getAttribute('role')).toBe('radiogroup');
    expect(parts.getAttribute('aria-label')).toBe('7 variants');
    expect(one(parts, '.dl-row-block__head').textContent).toBe('7 variants2 downloaded');
  });

  it('switches variants with native radios and shows remote ones as links to the mod page', () => {
    const onselect = vi.fn();
    const { target } = render(RowVariant, {
      name: 'Crimson',
      size: '2.2 MB',
      hint: 'Select to switch',
      group: 'faun',
      checked: false,
      onselect,
    });
    const radio = one<HTMLInputElement>(target, 'label.dl-variant input.dl-variant__radio');
    expect(radio.type).toBe('radio');
    expect(radio.name).toBe('faun');
    click(radio);
    expect(onselect).toHaveBeenCalledOnce();
    const active = render(RowVariant, { name: 'Bubblegum', size: '8 MB', group: 'faun', checked: true, activeLabel: 'Active' });
    expect(one(active.target, '.dl-badge--accent').textContent).toBe('Active');
    const remote = render(RowVariant, {
      name: 'Base · Leather',
      size: 'On GameBanana',
      group: 'faun',
      remote: { label: 'Mod page', href: '#/mod/652300' },
    });
    const row = one(remote.target, '.dl-variant--remote');
    expect(row.querySelector('input')).toBeNull();
    expect(one<HTMLAnchorElement>(row, 'a.dl-btn.dl-btn--sm').getAttribute('href')).toBe('#/mod/652300');
  });

  it('lists add-ons with their file and a remove button', () => {
    const onremove = vi.fn();
    const { target } = render(RowAddon, {
      icon: Link,
      name: 'Team add-on',
      file: '· pak_mcginnis_team.vpk · from mcginnis_team.zip',
      state: 'On with the mod',
      removeLabel: 'Remove add-on',
      onremove,
    });
    const addon = one(target, '.dl-addon');
    expect(one(addon, '.dl-addon__file').textContent).toBe('· pak_mcginnis_team.vpk · from mcginnis_team.zip');
    expect(one(addon, '.dl-addon__state').textContent).toBe('On with the mod');
    click(one(addon, 'button[aria-label="Remove add-on"]'));
    expect(onremove).toHaveBeenCalledOnce();
  });
});
