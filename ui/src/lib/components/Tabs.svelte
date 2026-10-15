<!-- Tabs (design: Tabs.md): views of the same data on one page. Roving tabindex; ← → move and select, Home/End.
     Not for top-level navigation (Sidebar) or two options of one setting (SegmentedControl). -->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import { edgeEnabled, stepEnabled } from './roving';

  export interface TabItem {
    id: string;
    label: string;
    /** Locale-formatted count or a plural-aware text such as "2 errors". */
    count?: string;
    /** Shows the count in danger; the tab itself stays neutral. */
    countError?: boolean;
    /** E.g. Network in local mode; it stays visible, cannot be chosen and is skipped by the arrows. */
    disabled?: boolean;
  }

  interface Props {
    tabs: TabItem[];
    selected: string;
    /** Accessible name of the tab list. */
    label: string;
    /** Renders the selected tab's panel; it gets the tab id. */
    panel?: Snippet<[string]>;
    onchange?: (id: string) => void;
  }

  let { tabs, selected = $bindable(), label, panel, onchange }: Props = $props();

  const uid = $props.id();
  const panelId = `${uid}-panel`;
  const tabId = (id: string) => `${uid}-tab-${id}`;
  const buttons: HTMLButtonElement[] = $state([]);
  // The tab reached with Tab: the selected one, or the first enabled one while nothing is selected.
  const tabStop = $derived.by(() => {
    const current = tabs.findIndex((tab) => tab.id === selected);
    return current >= 0 ? current : stepEnabled(tabs, -1, 1);
  });

  function select(index: number) {
    const tab = tabs[index];
    if (!tab || tab.disabled) {
      return;
    }
    if (tab.id !== selected) {
      selected = tab.id;
      onchange?.(tab.id);
    }
    buttons[index]?.focus();
  }

  function handleKeydown(event: KeyboardEvent, index: number) {
    const moves: Record<string, () => number> = {
      ArrowRight: () => stepEnabled(tabs, index, 1),
      ArrowLeft: () => stepEnabled(tabs, index, -1),
      Home: () => edgeEnabled(tabs, 'first'),
      End: () => edgeEnabled(tabs, 'last'),
    };
    const move = moves[event.key];
    if (!move) {
      return;
    }
    event.preventDefault();
    const next = move();
    if (next >= 0) {
      select(next);
    }
  }
</script>

<div class="dl-tabs" role="tablist" aria-label={label}>
  {#each tabs as tab, index (tab.id)}
    <button
      bind:this={buttons[index]}
      type="button"
      class="dl-tab"
      role="tab"
      id={tabId(tab.id)}
      aria-selected={tab.id === selected ? 'true' : 'false'}
      aria-controls={panel ? panelId : undefined}
      aria-disabled={tab.disabled ? 'true' : undefined}
      tabindex={index === tabStop ? 0 : -1}
      onclick={() => select(index)}
      onkeydown={(event) => handleKeydown(event, index)}
    >
      {tab.label}
      {#if tab.count}<span class={['dl-tab__count', tab.countError && 'dl-tab__count--error']}>{tab.count}</span>{/if}
    </button>
  {/each}
</div>
{#if panel}
  <div role="tabpanel" id={panelId} aria-labelledby={tabId(selected)}>{@render panel(selected)}</div>
{/if}
