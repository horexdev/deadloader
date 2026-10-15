<!-- LibraryRow (design: LibraryRow.md): one installed mod in the profile's priority list, always `--row-h` high.
     Keyboard reorder on the grip: Space/Enter picks up, ↑/↓ move, Space/Enter drops, Esc puts it back; Alt+↑/↓ moves
     one step without picking up. The list owns the order and announces each step in its live region. -->
<script lang="ts">
  import GripVertical from '@lucide/svelte/icons/grip-vertical';
  import HardDrive from '@lucide/svelte/icons/hard-drive';
  import type { Snippet } from 'svelte';
  import Badge, { type BadgeVariant } from './Badge.svelte';
  import ProgressBar from './ProgressBar.svelte';
  import Toggle from './Toggle.svelte';
  import type { IconComponent } from './types';

  export interface RowBadge {
    icon: IconComponent;
    label: string;
    variant: BadgeVariant;
  }

  interface Props {
    name: string;
    /** Author, source, "version · variant" and so on, shown with separators. */
    sub: string[];
    /** Thumbnail URL; null for local mods (hard-drive placeholder). */
    thumb?: string | null;
    /** On in this profile; off dims the row's content. */
    enabled: boolean;
    /** Accessible name of the toggle. */
    toggleLabel: string;
    /** Game not found: the library works, applying is off; say why in `toggleTitle`. */
    toggleDisabled?: boolean;
    toggleTitle?: string;
    /** "Reorder <name>, position 1 of 12". */
    gripLabel: string;
    /** At most two are shown; the rest belong in the details. */
    badges?: RowBadge[];
    /** A copy is being applied. */
    busy?: boolean;
    /** 0–1, shown as a thin bar under the text while busy. */
    progress?: number;
    progressLabel?: string;
    error?: boolean;
    /** Bulk selection. */
    selected?: boolean;
    /** Picked up with the keyboard. */
    grabbed?: boolean;
    /** Lifted by a pointer drag. */
    dragging?: boolean;
    /** -1 when another row holds the list's tab stop. */
    tabindex?: 0 | -1;
    /** Buttons on the right (expand, More, Repair). */
    actions?: Snippet;
    /** Variants or add-ons, shown under the row while `expanded`. */
    parts?: Snippet;
    expanded?: boolean;
    ontoggle?: (enabled: boolean) => void;
    ongrab?: () => void;
    onmove?: (delta: -1 | 1) => void;
    ondrop?: () => void;
    oncancel?: () => void;
    /** Enter on the row: open the details. */
    onopen?: () => void;
  }

  const {
    name,
    sub,
    thumb = null,
    enabled,
    toggleLabel,
    toggleDisabled = false,
    toggleTitle,
    gripLabel,
    badges = [],
    busy = false,
    progress,
    progressLabel = '',
    error = false,
    selected = false,
    grabbed = false,
    dragging = false,
    tabindex = 0,
    actions,
    parts,
    expanded = false,
    ontoggle,
    ongrab,
    onmove,
    ondrop,
    oncancel,
    onopen,
  }: Props = $props();

  const uid = $props.id();
  const nameId = `${uid}-name`;

  function handleGripKeydown(event: KeyboardEvent) {
    const actions: Record<string, (() => void) | undefined> = grabbed
      ? {
          ArrowUp: () => onmove?.(-1),
          ArrowDown: () => onmove?.(1),
          ' ': ondrop,
          Enter: ondrop,
          Escape: oncancel,
        }
      : { ' ': ongrab, Enter: ongrab };
    const action = event.altKey ? undefined : actions[event.key];
    if (action) {
      event.preventDefault();
      action();
    }
  }

  function handleRowKeydown(event: KeyboardEvent) {
    if (event.defaultPrevented) {
      return;
    }
    if (event.altKey && (event.key === 'ArrowUp' || event.key === 'ArrowDown')) {
      event.preventDefault();
      onmove?.(event.key === 'ArrowUp' ? -1 : 1);
    } else if (event.key === 'Enter' && event.target === event.currentTarget) {
      event.preventDefault();
      onopen?.();
    }
  }
</script>

{#if parts && expanded}
  <div class={['dl-row-block', !enabled && 'is-off']}>
    {@render row()}
    {@render parts()}
  </div>
{:else}
  {@render row()}
{/if}

{#snippet row()}
  <!-- The row itself is focusable (design: LibraryRow.md): Enter opens the details, Alt+arrows move it. -->
  <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
  <div
    class={[
      'dl-row',
      !enabled && 'is-off',
      error && 'is-error',
      selected && 'is-selected',
      grabbed && 'is-grabbed',
      dragging && 'is-dragging',
    ]}
    role="group"
    aria-labelledby={nameId}
    aria-busy={busy ? 'true' : undefined}
    {tabindex}
    onkeydown={handleRowKeydown}
  >
    <button
      type="button"
      class="dl-row__grip"
      aria-label={gripLabel}
      aria-pressed={grabbed ? 'true' : 'false'}
      {tabindex}
      onkeydown={handleGripKeydown}><GripVertical class="dl-icon" /></button
    >
    <Toggle
      ariaLabel={toggleLabel}
      checked={enabled}
      disabled={toggleDisabled}
      title={toggleTitle}
      {busy}
      {tabindex}
      onchange={(next) => ontoggle?.(next)}
    />
    {#if thumb}
      <img class="dl-row__thumb thumb" src={thumb} alt="" />
    {:else}
      <div class="dl-row__thumb local"><HardDrive class="dl-icon" /></div>
    {/if}
    <div class="dl-row__main">
      <span class="dl-row__name" id={nameId}>{name}</span>
      <span class="dl-row__sub">
        {#each sub as part, index (index)}
          {#if index > 0}<span class="dl-row__sep" aria-hidden="true">·</span>{/if}<span>{part}</span>
        {/each}
      </span>
    </div>
    <div class="dl-row__badges">
      {#each badges.slice(0, 2) as badge (badge.label)}
        <Badge icon={badge.icon} label={badge.label} variant={badge.variant} />
      {/each}
    </div>
    <div class="dl-row__actions">{@render actions?.()}</div>
    {#if busy && progress !== undefined}
      <div class="dl-row__progress"><ProgressBar thin value={progress} label={progressLabel} /></div>
    {/if}
  </div>
{/snippet}

<style>
  /* Thumbnail anchoring and the local-mod placeholder, from the design's library screens. */
  .thumb {
    object-position: 50% 30%;
  }

  .local {
    display: grid;
    place-items: center;
    color: var(--ink-subtle);
  }
</style>
