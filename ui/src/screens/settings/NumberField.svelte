<!-- A whole-number field with its unit (design: .st-numfield). It reports the typed text when the field is left or
     Enter is pressed; the section checks it and saves. -->
<script lang="ts">
  interface Props {
    label: string;
    value: string;
    unit?: string;
    invalid?: boolean;
    /** `st-num`, `st-num--wide` or `st-num--port`. */
    width?: string;
    mono?: boolean;
    inputmode?: 'numeric' | 'text';
    placeholder?: string;
    describedby?: string;
    oncommit: (text: string) => void;
  }

  const {
    label,
    value,
    unit,
    invalid = false,
    width = 'st-num',
    mono = false,
    inputmode = 'numeric',
    placeholder,
    describedby,
    oncommit,
  }: Props = $props();

  const uid = $props.id();
</script>

<span class="st-numfield">
  <label class="dl-field__label" for={uid}>{label}</label>
  <span class={['dl-input', mono && 'dl-input--mono', width]}>
    <input
      id={uid}
      {inputmode}
      {value}
      {placeholder}
      aria-invalid={invalid ? 'true' : 'false'}
      aria-describedby={describedby}
      onchange={(event) => oncommit(event.currentTarget.value)}
    />
    {#if unit}<span class="st-unit">{unit}</span>{/if}
  </span>
</span>
