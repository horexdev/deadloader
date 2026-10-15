<!-- Button (design: Button.md). One `primary` per dialog, panel, banner or empty state; `play` is for Play only. -->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import Spinner from './Spinner.svelte';
  import { tooltip } from './tooltip';
  import type { IconComponent } from './types';

  export type ButtonVariant = 'secondary' | 'primary' | 'ghost' | 'danger' | 'play';
  export type ButtonSize = 'sm' | 'md' | 'lg';

  interface Props extends Omit<HTMLButtonAttributes, 'children' | 'disabled'> {
    /** The visible label; for an icon-only button it becomes the accessible name. */
    label: string;
    variant?: ButtonVariant;
    size?: ButtonSize;
    /** Optional leading Lucide icon. */
    icon?: IconComponent;
    /** Square button that shows only the icon (give it a tooltip too). */
    iconOnly?: boolean;
    /** Working: the leading icon is replaced by a spinner and presses are ignored. */
    busy?: boolean;
    /** Not available, with the native `disabled` (not focusable). */
    disabled?: boolean;
    /** Not available but still focusable (`aria-disabled`), so a tooltip next to it can say why. */
    unavailable?: boolean;
    /** The action failed: the button turns into its retry. */
    error?: boolean;
    /** Trailing content after the label, e.g. a Count. */
    children?: Snippet;
  }

  const {
    label,
    variant = 'secondary',
    size = 'md',
    icon: Icon,
    iconOnly = false,
    busy = false,
    disabled = false,
    unavailable = false,
    error = false,
    children,
    type = 'button',
    class: className,
    // The design system's tooltip, never Windows' own (`title` on the native button).
    title,
    onclick,
    ...rest
  }: Props = $props();

  function handleClick(event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }) {
    if (busy || unavailable) {
      event.preventDefault();
      return;
    }
    onclick?.(event);
  }
</script>

<button
  {...rest}
  {type}
  class={[
    'dl-btn',
    variant !== 'secondary' && `dl-btn--${variant}`,
    size !== 'md' && `dl-btn--${size}`,
    iconOnly && 'dl-btn--icon',
    className,
  ]}
  {disabled}
  aria-label={iconOnly ? label : rest['aria-label']}
  aria-disabled={unavailable ? 'true' : undefined}
  aria-busy={busy ? 'true' : undefined}
  data-state={error ? 'error' : undefined}
  onclick={handleClick}
  {@attach tooltip({ text: title ?? '' })}
>
  {#if Icon}<Icon class="dl-icon" />{/if}
  {#if busy}<Spinner />{/if}
  {#if !iconOnly}<span class="dl-btn__label">{label}</span>{/if}
  {@render children?.()}
</button>
