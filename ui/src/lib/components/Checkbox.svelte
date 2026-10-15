<!-- Checkbox (design: Checkbox.md): a choice confirmed later. Use Toggle when the change applies at once.
     A native checkbox, so Space toggles it and the label click works. -->
<script lang="ts">
  import Check from '@lucide/svelte/icons/check';
  import Minus from '@lucide/svelte/icons/minus';
  import FieldError from './FieldError.svelte';

  interface Props {
    /** The label; it may wrap, the box stays top-aligned. */
    label: string;
    checked?: boolean;
    /** Some but not all rows are selected (bulk "select all"). */
    indeterminate?: boolean;
    disabled?: boolean;
    /** What to do, e.g. "Tick this box to continue."; turns the box red. */
    error?: string;
    name?: string;
    value?: string;
    onchange?: (checked: boolean) => void;
  }

  let {
    label,
    checked = $bindable(false),
    indeterminate = $bindable(false),
    disabled = false,
    error,
    name,
    value,
    onchange,
  }: Props = $props();

  const uid = $props.id();
  const errorId = `${uid}-error`;
</script>

<label class="dl-check">
  <input
    class="dl-check__input"
    type="checkbox"
    bind:checked
    bind:indeterminate
    {disabled}
    {name}
    {value}
    aria-invalid={error ? 'true' : undefined}
    aria-describedby={error ? errorId : undefined}
    onchange={() => onchange?.(checked)}
  />
  <span class="dl-check__box"><Check class="dl-icon dl-check__tick" /><Minus class="dl-icon dl-check__dash" /></span>
  <span>{label}</span>
</label>
{#if error}<FieldError id={errorId} message={error} />{/if}
