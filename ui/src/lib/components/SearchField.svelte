<!-- SearchField (design: SearchField.md): search for the catalog, library and Activity log. Esc clears, a second Esc
     leaves the field. Searches are debounced; clearing searches at once. -->
<script lang="ts">
  import { tooltip } from './tooltip';
  import Search from '@lucide/svelte/icons/search';
  import SearchX from '@lucide/svelte/icons/search-x';
  import X from '@lucide/svelte/icons/x';
  import { onDestroy } from 'svelte';
  import Spinner from './Spinner.svelte';

  interface Props {
    /** Accessible name, naming what is searched ("Search mods"). */
    label: string;
    placeholder: string;
    /** Accessible name of the clear button. */
    clearLabel: string;
    value?: string;
    /** Result count or "Searching…", shown while there is text. */
    resultText?: string;
    /** Key hint shown while empty, e.g. "/". */
    shortcut?: string;
    /** Line under the field, e.g. why it is disabled or "clear filters". */
    hint?: string;
    busy?: boolean;
    /** No results: the search-x icon and danger border. */
    error?: boolean;
    /** Offline or local mode; say why in `hint`. */
    disabled?: boolean;
    maxlength?: number;
    debounceMs?: number;
    /** Called with the query after typing stops, and at once when cleared. */
    onsearch?: (query: string) => void;
  }

  let {
    label,
    placeholder,
    clearLabel,
    value = $bindable(''),
    resultText,
    shortcut,
    hint,
    busy = false,
    error = false,
    disabled = false,
    maxlength,
    debounceMs = 300,
    onsearch,
  }: Props = $props();

  const uid = $props.id();
  const hintId = `${uid}-hint`;
  let input: HTMLInputElement | undefined = $state();
  let timer: ReturnType<typeof setTimeout> | undefined;

  /** Moves keyboard focus into the field (Ctrl+F and / are handled by the page). */
  export function focus(): void {
    input?.focus();
  }

  function cancelPending() {
    if (timer !== undefined) {
      clearTimeout(timer);
      timer = undefined;
    }
  }

  function handleInput() {
    cancelPending();
    timer = setTimeout(() => {
      timer = undefined;
      onsearch?.(value);
    }, debounceMs);
  }

  function clear() {
    cancelPending();
    value = '';
    onsearch?.('');
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key !== 'Escape') {
      return;
    }
    event.preventDefault();
    if (value !== '') {
      clear();
    } else {
      input?.blur();
    }
  }

  function clearAndRefocus() {
    clear();
    input?.focus();
  }

  onDestroy(cancelPending);
</script>

<div class="dl-search">
  <div class={['dl-input', error && 'is-error', disabled && 'is-disabled']} aria-busy={busy ? 'true' : undefined}>
    {#if error}<SearchX class="dl-icon" />{:else}<Search class="dl-icon" />{/if}
    <input
      bind:this={input}
      type="search"
      bind:value
      {placeholder}
      {disabled}
      {maxlength}
      aria-label={label}
      aria-describedby={hint ? hintId : undefined}
      oninput={handleInput}
      onkeydown={handleKeydown}
    />
    {#if value !== ''}
      {#if busy}<Spinner />{/if}
      {#if resultText}<span class="dl-search__meta">{resultText}</span>{/if}
      <button
        type="button"
        class="dl-btn dl-btn--ghost dl-btn--icon dl-search__clear"
        aria-label={clearLabel}
        {@attach tooltip({ text: clearLabel ?? '' })}
        {disabled}
        onclick={clearAndRefocus}><X class="dl-icon" /></button
      >
    {:else if shortcut}
      <kbd class="dl-kbd">{shortcut}</kbd>
    {/if}
  </div>
  {#if hint}<span class="dl-field__hint" id={hintId}>{hint}</span>{/if}
</div>
