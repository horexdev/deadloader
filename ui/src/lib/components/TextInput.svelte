<!-- TextInput (design: TextInput.md): a labelled single-line field. Validate on blur or submit, never per keystroke,
     and never clear the user's input on error. -->
<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import FieldError from './FieldError.svelte';
  import Spinner from './Spinner.svelte';

  interface Props extends Omit<HTMLInputAttributes, 'value' | 'children' | 'class' | 'id'> {
    /** Always visible: no placeholder-as-label. */
    label: string;
    value?: string;
    hint?: string;
    /** What is wrong and what to do. */
    error?: string;
    /** JetBrains Mono for paths, URLs and hosts. */
    mono?: boolean;
    /** Asynchronous validation is running. */
    busy?: boolean;
  }

  let {
    label,
    value = $bindable(''),
    hint,
    error,
    mono = false,
    busy = false,
    type = 'text',
    ...rest
  }: Props = $props();

  const uid = $props.id();
  const inputId = `${uid}-input`;
  const hintId = `${uid}-hint`;
  const errorId = `${uid}-error`;
  const describedBy = $derived([hint && hintId, error && errorId].filter(Boolean).join(' ') || undefined);
</script>

<div class="dl-field">
  <label class="dl-field__label" for={inputId}>{label}</label>
  <div class={['dl-input', mono && 'dl-input--mono']} aria-busy={busy ? 'true' : undefined}>
    <input
      {...rest}
      id={inputId}
      {type}
      bind:value
      aria-invalid={error ? 'true' : undefined}
      aria-describedby={describedBy}
    />
    {#if busy}<Spinner />{/if}
  </div>
  {#if hint}<span class="dl-field__hint" id={hintId}>{hint}</span>{/if}
  {#if error}<FieldError id={errorId} message={error} />{/if}
</div>
