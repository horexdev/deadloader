<!-- Select (design: Select.md): a trigger that opens a `.dl-menu` listbox. A select-only combobox: focus stays on the
     trigger (or on the filter input) and the active option is announced through aria-activedescendant.
     Keyboard: Enter/Space/↓/↑ open; ↑↓ Home End and type-ahead move; Enter selects; Esc closes. -->
<script lang="ts">
  import Check from '@lucide/svelte/icons/check';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import { flushSync, onDestroy, tick } from 'svelte';
  import FieldError from './FieldError.svelte';
  import Spinner from './Spinner.svelte';
  import { placeMenu, type MenuPlace } from './menu-place';
  import { edgeEnabled, stepEnabled, typeahead } from './roving';

  export interface SelectOption {
    value: string;
    label: string;
    /** Locale-formatted count shown on the right. */
    count?: string;
    disabled?: boolean;
    /** Heading of the group this option belongs to; options of one group must be adjacent. */
    group?: string;
  }

  interface Props {
    options: SelectOption[];
    value: string;
    /** Accessible name of the control. */
    label: string;
    /** Visible lead inside the trigger, e.g. "Sort". */
    prefix?: string;
    /** Shown while no option matches `value`. */
    placeholder?: string;
    disabled?: boolean;
    /** Options are loading: spinner and `busyText` in the trigger and in an open menu. */
    busy?: boolean;
    busyText?: string;
    error?: string;
    /** Turns on the filter input at the top of the menu (for long lists such as the 44 heroes). */
    filterLabel?: string;
    filterPlaceholder?: string;
    /** Shown when the filter matches nothing. */
    emptyText?: string;
    onchange?: (value: string) => void;
  }

  let {
    options,
    value = $bindable(),
    label,
    prefix,
    placeholder,
    disabled = false,
    busy = false,
    busyText,
    error,
    filterLabel,
    filterPlaceholder,
    emptyText,
    onchange,
  }: Props = $props();

  const uid = $props.id();
  const listId = `${uid}-list`;
  const errorId = `${uid}-error`;
  const optionId = (index: number) => `${uid}-opt-${index}`;

  let open = $state(false);
  let filter = $state('');
  /** Index into `visible` of the keyboard-active option, or -1. */
  let active = $state(-1);
  let root: HTMLElement | undefined = $state();
  let trigger: HTMLButtonElement | undefined = $state();
  let filterInput: HTMLInputElement | undefined = $state();
  let menuEl: HTMLElement | undefined = $state();
  /** Where the open menu sits in the window (fixed, so it never makes the page or a panel scroll). */
  let place: MenuPlace | null = $state(null);
  let typed = '';
  let typedTimer: ReturnType<typeof setTimeout> | undefined;

  const selected = $derived(options.find((option) => option.value === value));
  const visible = $derived.by(() => {
    const needle = filter.trim().toLocaleLowerCase();
    return options
      .map((option, index) => ({ option, index }))
      .filter(({ option }) => needle === '' || option.label.toLocaleLowerCase().includes(needle));
  });
  const steppable = $derived(visible.map(({ option }) => ({ disabled: option.disabled, label: option.label })));
  const activeId = $derived.by(() => {
    const entry = visible[active];
    return open && entry ? optionId(entry.index) : undefined;
  });
  /** Visible options split into runs of the same group. */
  const groups = $derived.by(() => {
    const runs: { name: string | undefined; entries: { option: SelectOption; index: number; position: number }[] }[] =
      [];
    visible.forEach((entry, position) => {
      const last = runs.at(-1);
      if (last && last.name === entry.option.group) {
        last.entries.push({ ...entry, position });
      } else {
        runs.push({ name: entry.option.group, entries: [{ ...entry, position }] });
      }
    });
    return runs;
  });

  function openMenu(from: 'selected' | 'last' = 'selected') {
    if (disabled || open) {
      return;
    }
    filter = '';
    open = true;
    const current = visible.findIndex(({ option }) => option.value === value);
    active = current >= 0 && from === 'selected' ? current : edgeEnabled(steppable, from === 'last' ? 'last' : 'first');
    // Render the menu now so the filter input can take focus within this key or click.
    flushSync();
    measure();
    filterInput?.focus();
    scrollActiveIntoView();
  }

  /** Opens the menu below its trigger, or above when there is more room there. */
  function measure() {
    if (!trigger || !menuEl) {
      return;
    }
    const rect = trigger.getBoundingClientRect();
    place = placeMenu(rect, menuEl.scrollHeight, { width: window.innerWidth, height: window.innerHeight }, menuEl.offsetWidth);
    flushSync();
  }

  /** A fixed menu would drift from its trigger when anything around it scrolls: close it, like a native select. */
  function handleScroll(event: Event) {
    if (open && !(event.target instanceof Node && menuEl?.contains(event.target))) {
      closeMenu(false);
    }
  }

  function closeMenu(refocus = true) {
    open = false;
    place = null;
    active = -1;
    if (refocus) {
      trigger?.focus();
    }
  }

  function choose(position: number) {
    const entry = visible[position];
    if (!entry || entry.option.disabled) {
      return;
    }
    const changed = entry.option.value !== value;
    value = entry.option.value;
    closeMenu();
    if (changed) {
      onchange?.(entry.option.value);
    }
  }

  function scrollActiveIntoView() {
    if (activeId) {
      document.getElementById(activeId)?.scrollIntoView?.({ block: 'nearest' });
    }
  }

  function moveTo(position: number) {
    if (position >= 0) {
      active = position;
      void tick().then(scrollActiveIntoView);
    }
  }

  function handleTypeahead(key: string) {
    clearTimeout(typedTimer);
    typed += key;
    typedTimer = setTimeout(() => (typed = ''), 500);
    moveTo(typeahead(steppable, active, typed));
  }

  function handleKeydown(event: KeyboardEvent, fromFilter: boolean) {
    if (!open) {
      if (['ArrowDown', 'Enter', ' '].includes(event.key)) {
        event.preventDefault();
        openMenu();
      } else if (event.key === 'ArrowUp') {
        event.preventDefault();
        openMenu('last');
      }
      return;
    }
    const keyed: Record<string, () => void> = {
      ArrowDown: () => moveTo(stepEnabled(steppable, active, 1, false)),
      ArrowUp: () => moveTo(stepEnabled(steppable, active, -1, false)),
      Home: () => moveTo(edgeEnabled(steppable, 'first')),
      End: () => moveTo(edgeEnabled(steppable, 'last')),
      Enter: () => choose(active),
      Escape: () => closeMenu(),
    };
    if (!fromFilter) {
      keyed[' '] = () => choose(active);
    }
    const action = keyed[event.key];
    if (action) {
      if (fromFilter && (event.key === 'Home' || event.key === 'End')) {
        return;
      }
      event.preventDefault();
      action();
    } else if (event.key === 'Tab') {
      closeMenu(false);
    } else if (!fromFilter && event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
      handleTypeahead(event.key);
    }
  }

  function handleFilterInput() {
    active = edgeEnabled(steppable, 'first');
  }

  function handleDocumentPointerDown(event: PointerEvent) {
    if (open && root && event.target instanceof Node && !root.contains(event.target)) {
      closeMenu(false);
    }
  }

  onDestroy(() => clearTimeout(typedTimer));
</script>

<svelte:document onpointerdown={handleDocumentPointerDown} />
<svelte:window onscrollcapture={handleScroll} onresize={() => open && closeMenu(false)} />

<div class="dl-select" bind:this={root}>
  <button
    bind:this={trigger}
    type="button"
    class="dl-select__trigger"
    role="combobox"
    aria-label={label}
    aria-haspopup="listbox"
    aria-expanded={open ? 'true' : 'false'}
    aria-controls={open ? listId : undefined}
    aria-activedescendant={filterLabel ? undefined : activeId}
    aria-busy={busy ? 'true' : undefined}
    aria-invalid={error ? 'true' : undefined}
    aria-describedby={error ? errorId : undefined}
    {disabled}
    onclick={() => (open ? closeMenu() : openMenu())}
    onkeydown={(event) => handleKeydown(event, false)}
  >
    {#if prefix}<span class="dl-select__prefix">{prefix}</span>{/if}
    {#if busy}<Spinner />{/if}
    <span class="dl-select__value">{busy && busyText ? busyText : (selected?.label ?? placeholder ?? '')}</span>
    <ChevronDown class="dl-icon dl-select__chevron" />
  </button>

  {#if open}
    {#if filterLabel}
      <div
        class="dl-menu menu"
        bind:this={menuEl}
        data-placement={place?.above ? 'above' : 'below'}
        style:top={place?.top === undefined ? undefined : `${place.top}px`}
        style:bottom={place?.bottom === undefined ? undefined : `${place.bottom}px`}
        style:left={place ? `${place.left}px` : undefined}
        style:min-width={place ? `${place.minWidth}px` : undefined}
        style:max-height={place ? `${place.maxHeight}px` : undefined}
      >
        <div class="dl-input filter">
          <input
            bind:this={filterInput}
            type="text"
            bind:value={filter}
            aria-label={filterLabel}
            placeholder={filterPlaceholder}
            aria-controls={listId}
            aria-activedescendant={activeId}
            autocomplete="off"
            oninput={handleFilterInput}
            onkeydown={(event) => handleKeydown(event, true)}
          />
        </div>
        <div id={listId} role="listbox" aria-label={label}>
          {@render body()}
        </div>
      </div>
    {:else}
      <div
        class="dl-menu menu"
        id={listId}
        role="listbox"
        aria-label={label}
        bind:this={menuEl}
        data-placement={place?.above ? 'above' : 'below'}
        style:top={place?.top === undefined ? undefined : `${place.top}px`}
        style:bottom={place?.bottom === undefined ? undefined : `${place.bottom}px`}
        style:left={place ? `${place.left}px` : undefined}
        style:min-width={place ? `${place.minWidth}px` : undefined}
        style:max-height={place ? `${place.maxHeight}px` : undefined}
      >
        {@render body()}
      </div>
    {/if}
  {/if}
</div>
{#if error}<FieldError id={errorId} message={error} />{/if}

{#snippet body()}
  {#if busy}
    <div class="dl-menu__status" role="status"><Spinner />{busyText}</div>
  {:else if visible.length === 0 && emptyText}
    <div class="dl-menu__status" role="status">{emptyText}</div>
  {:else}
    {#each groups as group, g (g)}
      {#if group.name !== undefined}
        {#if g > 0}<div class="dl-menu__sep" aria-hidden="true"></div>{/if}
        <div role="group" aria-labelledby={`${uid}-group-${g}`}>
          <div class="dl-menu__group" id={`${uid}-group-${g}`} role="presentation">{group.name}</div>
          {@render items(group.entries)}
        </div>
      {:else}
        {@render items(group.entries)}
      {/if}
    {/each}
  {/if}
{/snippet}

{#snippet items(entries: { option: SelectOption; index: number; position: number }[])}
  {#each entries as { option, index, position } (index)}
    <div
      class="dl-option"
      id={optionId(index)}
      role="option"
      tabindex="-1"
      aria-selected={option.value === value ? 'true' : 'false'}
      aria-disabled={option.disabled ? 'true' : undefined}
      data-active={position === active ? 'true' : undefined}
      onclick={() => choose(position)}
      onkeydown={(event) => handleKeydown(event, false)}
      onpointermove={() => {
        if (!option.disabled) active = position;
      }}
    >
      <Check class="dl-icon dl-option__check" />{option.label}{#if option.count}<span class="dl-option__count"
          >{option.count}</span
        >{/if}
    </div>
  {/each}
{/snippet}

<style>
  /* Below the trigger, left-aligned, 4px apart (the design's screens) — or above when there is more room there.
     Fixed to the window (menu-place.ts), so an open menu never makes the page or a panel scroll. */
  .menu {
    position: fixed;
  }

  .filter {
    margin-bottom: var(--space-1);
  }
</style>
