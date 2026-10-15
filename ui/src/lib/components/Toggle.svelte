<!-- Toggle (design: Toggle.md): a switch that takes effect at once. A native checkbox with role="switch", so Space
     flips it. It never asks "Are you sure?". -->
<script lang="ts">
  import { tooltip } from './tooltip';
  import FieldError from './FieldError.svelte';
  import Spinner from './Spinner.svelte';

  interface Props {
    /** Visible label; inside rows use `ariaLabel` instead. */
    label?: string;
    ariaLabel?: string;
    checked?: boolean;
    disabled?: boolean;
    /** The change is being applied (a copy, not a link): spinner in the thumb, no flipping meanwhile. */
    busy?: boolean;
    /** What happened, why and what to do. */
    error?: string;
    /** Tooltip text, e.g. why it is disabled. */
    title?: string;
    /** -1 inside a row that does not hold the list's tab stop. */
    tabindex?: 0 | -1;
    onchange?: (checked: boolean) => void;
  }

  let {
    label,
    ariaLabel,
    checked = $bindable(false),
    disabled = false,
    busy = false,
    error,
    title,
    tabindex,
    onchange,
  }: Props = $props();

  const uid = $props.id();
  const errorId = `${uid}-error`;

  function refuseWhileBusy(event: MouseEvent) {
    if (busy) {
      event.preventDefault();
    }
  }
</script>

<label class={['dl-switch', error && 'is-error']} aria-busy={busy ? 'true' : undefined} {@attach tooltip({ text: title ?? '' })}>
  <input
    class="dl-switch__input"
    type="checkbox"
    role="switch"
    bind:checked
    {disabled}
    {tabindex}
    aria-label={ariaLabel}
    aria-busy={busy ? 'true' : undefined}
    aria-invalid={error ? 'true' : undefined}
    aria-describedby={error ? errorId : undefined}
    onclick={refuseWhileBusy}
    onchange={() => onchange?.(checked)}
  />
  <span class="dl-switch__track"><span class="dl-switch__thumb">{#if busy}<Spinner />{/if}</span></span>
  {#if label}{label}{/if}
</label>
{#if error}<FieldError id={errorId} message={error} />{/if}
