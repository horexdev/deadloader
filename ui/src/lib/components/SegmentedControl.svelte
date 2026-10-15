<!-- SegmentedControl (design: SegmentedControl.md): one of 2–4 short options that switch a view or a mode. Radio
     semantics: the arrow keys move and select. When an option needs confirmation (NSFW shown → D3), return false
     from `onchange` and set `value` only after the user confirms. -->
<script lang="ts">
  import FieldError from './FieldError.svelte';
  import Spinner from './Spinner.svelte';
  import { stepEnabled } from './roving';

  export interface SegmentOption {
    value: string;
    /** At most two words in English; the control grows (and wraps as a whole) in longer languages. */
    label: string;
    disabled?: boolean;
  }

  interface Props {
    options: SegmentOption[];
    value: string;
    /** Accessible name of the group. */
    label: string;
    disabled?: boolean;
    /** The new choice is being applied: spinner in the selected option. */
    busy?: boolean;
    error?: string;
    /** Called before the choice is applied; return false to keep the current one. */
    onchange?: (value: string) => boolean | undefined;
  }

  let { options, value = $bindable(), label, disabled = false, busy = false, error, onchange }: Props = $props();

  const uid = $props.id();
  const errorId = `${uid}-error`;
  const buttons: HTMLButtonElement[] = $state([]);
  const steppable = $derived(options.map((option) => ({ disabled: disabled || option.disabled })));
  // The option reached with Tab: the checked one, or the first enabled one while nothing is checked.
  const tabStop = $derived.by(() => {
    const checked = options.findIndex((option) => option.value === value);
    return checked >= 0 ? checked : stepEnabled(steppable, -1, 1);
  });

  function select(index: number) {
    const option = options[index];
    if (!option || steppable[index]?.disabled) {
      return;
    }
    buttons[index]?.focus();
    if (option.value === value || onchange?.(option.value) === false) {
      return;
    }
    value = option.value;
  }

  function handleKeydown(event: KeyboardEvent, index: number) {
    const delta = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 }[event.key] as 1 | -1 | undefined;
    if (delta === undefined) {
      return;
    }
    event.preventDefault();
    const next = stepEnabled(steppable, index, delta);
    if (next >= 0) {
      select(next);
    }
  }
</script>

<div
  class={['dl-seg', error && 'is-error']}
  role="radiogroup"
  aria-label={label}
  aria-busy={busy ? 'true' : undefined}
  aria-invalid={error ? 'true' : undefined}
  aria-describedby={error ? errorId : undefined}
>
  {#each options as option, index (option.value)}
    {@const checked = option.value === value}
    <button
      bind:this={buttons[index]}
      type="button"
      class="dl-seg__item"
      role="radio"
      aria-checked={checked ? 'true' : 'false'}
      tabindex={index === tabStop ? 0 : -1}
      disabled={steppable[index]?.disabled}
      onclick={() => select(index)}
      onkeydown={(event) => handleKeydown(event, index)}
    >
      {#if checked && busy}<Spinner />{/if}{option.label}
    </button>
  {/each}
</div>
{#if error}<FieldError id={errorId} message={error} />{/if}
