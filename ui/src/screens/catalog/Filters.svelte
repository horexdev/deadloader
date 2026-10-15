<!-- The Filters button of the catalog toolbar and its menu: mods with content ratings (hidden / blurred / shown, D3 on
     first use), favourites only, hide installed (S2). The boards show only the button and its count; the menu is
     built from the design system's menu, segmented control and checkboxes. -->
<script lang="ts">
  import SlidersHorizontal from '@lucide/svelte/icons/sliders-horizontal';
  import { tick } from 'svelte';
  import Checkbox from '../../lib/components/Checkbox.svelte';
  import SegmentedControl from '../../lib/components/SegmentedControl.svelte';
  import { app } from '../../lib/app.svelte';
  import { chooseNsfw, type NsfwMode } from '../../lib/nsfw';
  import { t } from '../../i18n';
  import type { CatalogBrowser } from './browse.svelte';

  interface Props {
    browser: CatalogBrowser;
    disabled?: boolean;
  }

  const { browser, disabled = false }: Props = $props();

  const uid = $props.id();
  let open = $state(false);
  let root: HTMLElement | undefined = $state();
  let button: HTMLButtonElement | undefined = $state();

  const nsfw = $derived((app.settings?.catalog?.nsfw as NsfwMode | undefined) ?? 'hidden');
  const options = $derived(
    (['hidden', 'blurred', 'shown'] as const).map((value) => ({ value, label: t(`catalog-nsfw-${value}`) })),
  );

  async function toggle(): Promise<void> {
    open = !open;
    if (open) {
      await tick();
      root?.querySelector<HTMLElement>('.menu [role="radio"][tabindex="0"]')?.focus();
    }
  }

  function close(refocus: boolean): void {
    open = false;
    if (refocus) {
      button?.focus();
    }
  }

  function chooseMode(value: string): boolean {
    // The choice is applied once D3 and saving are through; the control shows the saved setting.
    void chooseNsfw(value as NsfwMode);
    return false;
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === 'Escape' && open) {
      event.preventDefault();
      close(true);
    }
  }

  function handlePointerDown(event: PointerEvent): void {
    if (open && root && event.target instanceof Node && !root.contains(event.target)) {
      close(false);
    }
  }
</script>

<svelte:document onpointerdown={handlePointerDown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="filters" bind:this={root} onkeydown={handleKeydown}>
  <button
    bind:this={button}
    type="button"
    class="dl-btn"
    aria-expanded={open ? 'true' : 'false'}
    aria-controls={open ? `${uid}-menu` : undefined}
    {disabled}
    onclick={() => void toggle()}
  >
    <SlidersHorizontal class="dl-icon" />{t('catalog-filters')}{#if browser.activeFilters > 0}<span
        class="dl-count dl-count--muted">{browser.activeFilters}</span
      >{/if}
  </button>
  {#if open}
    <div class="dl-menu menu" id={`${uid}-menu`} role="group" aria-label={t('catalog-filters')}>
      <div class="dl-menu__group" id={`${uid}-nsfw`}>{t('catalog-filter-nsfw')}</div>
      <div class="row">
        <SegmentedControl options={options} value={nsfw} label={t('catalog-filter-nsfw')} onchange={chooseMode} />
      </div>
      <div class="dl-menu__sep" aria-hidden="true"></div>
      <div class="row">
        <Checkbox
          label={t('catalog-filter-favourites')}
          checked={browser.favouritesOnly}
          onchange={(checked) => void browser.setFavouritesOnly(checked)}
        />
      </div>
      <div class="row">
        <Checkbox
          label={t('catalog-filter-installed')}
          checked={browser.hideInstalled}
          onchange={(checked) => browser.setHideInstalled(checked)}
        />
      </div>
    </div>
  {/if}
</div>

<style>
  .filters {
    position: relative;
  }

  .menu {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    max-height: none;
  }

  .row {
    padding: var(--space-1) var(--space-2);
    white-space: nowrap;
  }
</style>
