<!-- Test harness: a VirtualGrid over numbered items whose snippet uses the roving tabindex like the real cards. -->
<script lang="ts">
  import VirtualGrid from '../components/VirtualGrid.svelte';

  interface Props {
    count: number;
    itemHeight: number | ((columnWidth: number) => number);
    minItemWidth?: number;
    gap?: number;
    overscan?: number;
    padding?: number;
    activeIndex?: number;
    onrangechange?: (start: number, end: number) => void;
    initialScrollTop?: number;
    onscroll?: (top: number) => void;
  }

  let { count, activeIndex = $bindable(0), ...rest }: Props = $props();

  const items = $derived(Array.from({ length: count }, (_, id) => ({ id, name: `Item ${id}` })));
  let grid: ReturnType<typeof VirtualGrid> | undefined = $state();

  export function scrollToIndex(index: number): void {
    grid?.scrollToIndex(index);
  }
</script>

<VirtualGrid bind:this={grid} bind:activeIndex {items} key={(item) => item.id} label="Mods" {...rest}>
  {#snippet item(entry, index, state)}
    <a class="card" href={`#/mod/${entry.id}`} data-n={index} data-grid-focus tabindex={state.active ? 0 : -1}>{entry.name}</a>
    <button type="button" class="fav" tabindex={state.active ? 0 : -1}>Fav</button>
  {/snippet}
</VirtualGrid>
