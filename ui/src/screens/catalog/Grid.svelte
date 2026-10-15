<!-- S2 Catalog (design: Main, CatalogLoading, CatalogEmpty, CatalogOffline, CatalogStates): section, search, sort,
     category and hero, the filters, and the virtualized grid with infinite scroll; every state of the boards. -->
<script lang="ts">
  import { tooltip } from '../../lib/components/tooltip';
  import { notice } from '../../lib/notice.svelte';
  import Check from '@lucide/svelte/icons/check';
  import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Copy from '@lucide/svelte/icons/copy';
  import Eye from '@lucide/svelte/icons/eye';
  import EyeOff from '@lucide/svelte/icons/eye-off';
  import RefreshCw from '@lucide/svelte/icons/refresh-cw';
  import SearchX from '@lucide/svelte/icons/search-x';
  import WifiOff from '@lucide/svelte/icons/wifi-off';
  import { onMount } from 'svelte';
  import Banner from '../../lib/components/Banner.svelte';
  import Button from '../../lib/components/Button.svelte';
  import CardSkeleton from '../../lib/components/CardSkeleton.svelte';
  import CatalogCard, { type CardBadge } from '../../lib/components/CatalogCard.svelte';
  import EmptyState from '../../lib/components/EmptyState.svelte';
  import SearchField from '../../lib/components/SearchField.svelte';
  import SegmentedControl from '../../lib/components/SegmentedControl.svelte';
  import Select from '../../lib/components/Select.svelte';
  import Spinner from '../../lib/components/Spinner.svelte';
  import VirtualGrid from '../../lib/components/VirtualGrid.svelte';
  import { CARD_MIN_W, GRID_GAP, catalogCardHeight, thumbVariant } from '../../lib/components/geometry';
  import { gridGeometry } from '../../lib/components/virtual';
  import { catalog, imageUrl, isUserMessage, type Card, type Section, type Sort } from '../../lib/api';
  import { app } from '../../lib/app.svelte';
  import { router } from '../../lib/router.svelte';
  import { describe, formatNumber, t } from '../../i18n';
  import Filters from './Filters.svelte';
  import { browser } from './browse.svelte';
  import {
    byline,
    clampSearch,
    exactTime,
    installedVersion,
    libraryModOf,
    relativeTime,
    updateAvailable,
  } from './model';

  /** Space around the grid so focus rings at its edges are not clipped. */
  const PADDING = 4;
  const SORTS: Sort[] = [
    'newest',
    'latest_updated',
    'most_downloaded',
    'most_liked',
    'most_viewed',
    'most_commented',
    'oldest',
  ];

  let search: ReturnType<typeof SearchField> | undefined = $state();
  let width = $state(0);
  let activeIndex = $state(0);
  let now = $state(Date.now());
  let searchValue = $state(browser.text);

  const network = $derived(app.network);
  const local = $derived(
    network.kind === 'local_mode' || (browser.error !== null && !isUserMessage(browser.error) && browser.error.kind === 'local_mode'),
  );
  const errorKind = $derived(
    browser.error && !isUserMessage(browser.error) ? browser.error.kind : browser.error ? 'message' : null,
  );
  const apiChanged = $derived(network.kind === 'api_changed' || errorKind === 'api_changed');
  const offline = $derived(network.kind === 'offline');
  const items = $derived(browser.items);
  const nsfw = $derived((app.settings?.catalog?.nsfw as string | undefined) ?? 'hidden');

  const columnWidth = $derived(
    gridGeometry({
      width: Math.max(0, width - 2 * PADDING),
      count: 1,
      minItemWidth: CARD_MIN_W,
      gap: GRID_GAP,
      itemHeight: catalogCardHeight,
    }).columnWidth,
  );
  const variant = $derived(thumbVariant(columnWidth, typeof window === 'undefined' ? 1 : window.devicePixelRatio || 1));

  const sortOptions = $derived(SORTS.map((value) => ({ value, label: t(`catalog-sort-${value}`) })));
  const categoryOptions = $derived([
    { value: '', label: t('catalog-category-all') },
    ...browser.roots.map((c) => ({ value: String(c.id), label: c.name, count: formatNumber(c.item_count) })),
  ]);
  const heroOptions = $derived([
    { value: '', label: t('catalog-hero-all') },
    ...browser.heroes.map((c) => ({ value: String(c.id), label: c.name, count: formatNumber(c.item_count) })),
  ]);
  // Offline only the shown section's saved pages can be listed (design: CatalogOffline).
  const sectionOptions = $derived([
    { value: 'mod', label: t('catalog-section-mods'), disabled: offline && browser.section !== 'mod' },
    { value: 'sound', label: t('catalog-section-sounds'), disabled: offline && browser.section !== 'sound' },
  ]);

  /** Seconds until GameBanana is asked again (E5). */
  const waitSeconds = $derived(
    network.kind === 'rate_limited' ? Math.max(0, Math.ceil((network.until_unix_ms - now) / 1000)) : 0,
  );

  // The countdown ticks only while GameBanana asked to wait (no timers while idle).
  $effect(() => {
    if (network.kind !== 'rate_limited') {
      return;
    }
    now = Date.now();
    const timer = setInterval(() => (now = Date.now()), 1000);
    return () => clearInterval(timer);
  });

  // Back online, the catalog turned on, or a new NSFW choice: list again.
  let lastNetwork = app.network.kind;
  $effect(() => {
    const kind = network.kind;
    const before = lastNetwork;
    lastNetwork = kind;
    if (kind === 'online' && before !== 'online' && (browser.error || browser.pages.length === 0 || before === 'local_mode')) {
      void browser.load();
      void browser.loadCategories();
    }
  });
  let lastNsfw: string | null = null;
  $effect(() => {
    const mode = nsfw;
    if (lastNsfw !== null && mode !== lastNsfw) {
      void browser.load();
    }
    lastNsfw = mode;
  });

  // Pages the core refreshed in the background.
  $effect(() => {
    const event = app.catalogEvent;
    if (event?.event === 'page_updated') {
      void browser.pageUpdated(event.data);
    }
  });

  onMount(() => {
    if (browser.pages.length === 0 && !browser.loading && !browser.favouritesOnly) {
      void browser.load();
    }
    if (browser.roots.length === 0) {
      void browser.loadCategories();
    }
  });

  function handleWindowKeydown(event: KeyboardEvent): void {
    const target = event.target as HTMLElement | null;
    const typing = target?.closest('input, textarea, [contenteditable="true"]');
    if ((event.key === 'f' && event.ctrlKey && !event.altKey) || (event.key === '/' && !typing && !event.ctrlKey)) {
      event.preventDefault();
      search?.focus();
    }
  }

  async function retry(): Promise<void> {
    await catalog.retryNow().catch(() => undefined);
    await browser.load();
  }

  function enableCatalog(): void {
    // F13: the wizard's network step says what turning the catalog on means, then turns it on.
    notice.open({ mode: 'catalog' });
  }

  function badges(card: Card): CardBadge[] {
    const mod = libraryModOf(app.library, card.section, card.id);
    if (!mod) {
      return [];
    }
    const list: CardBadge[] = [
      { icon: Check, label: installedVersion(mod) ?? t('catalog-badge-installed'), variant: 'accent' },
    ];
    if (updateAvailable(app.updates, mod.id)) {
      list.push({ icon: CircleArrowUp, label: t('catalog-badge-update'), variant: 'info' });
    }
    return list;
  }

  function progressOf(card: Card): string | undefined {
    const task = app.tasks.find(
      (task) => task.meta['mod_id'] === String(card.id) && (task.meta['section'] ?? 'mod') === card.section,
    );
    if (!task || task.state.state === 'failed' || task.state.state === 'done') {
      return undefined;
    }
    const percent = task.total ? Math.floor((task.bytes / task.total) * 100) : 0;
    return t('catalog-card-installing', { percent });
  }

  function thumbOf(card: Card): string | null {
    if (!card.thumbnail) {
      return null;
    }
    return imageUrl(variant === 220 ? card.thumbnail.small : card.thumbnail.large);
  }

  function dateOf(card: Card): number | null {
    return card.date_updated ?? card.date_added;
  }

  const filterNames = $derived.by(() => {
    const names: string[] = [];
    const root = browser.roots.find((c) => c.id === browser.category);
    if (root) {
      names.push(root.name);
    }
    const hero = browser.heroes.find((c) => c.id === browser.hero);
    if (hero) {
      names.push(hero.name);
    }
    if (browser.favouritesOnly) {
      names.push(t('catalog-filter-favourites-short'));
    }
    if (browser.hideInstalled) {
      names.push(t('catalog-filter-installed-short'));
    }
    return names;
  });

  const errorText = $derived.by(() => {
    const error = browser.error;
    if (!error) {
      return null;
    }
    if (isUserMessage(error)) {
      return describe(error);
    }
    if ('message' in error && error.message) {
      return describe(error.message);
    }
    if (error.kind === 'offline') {
      return { what: t('catalog-offline-empty-title'), why: t('catalog-offline-empty-text'), todo: null, detail: null };
    }
    return { what: t('catalog-error-title'), why: null, todo: null, detail: null };
  });

  const apiMessage = $derived.by(() => {
    if (network.kind === 'api_changed') {
      return network.message;
    }
    const error = browser.error;
    return error && !isUserMessage(error) && error.kind === 'api_changed' ? error.message : null;
  });

  const savedAt = $derived(
    browser.origin?.kind === 'cache'
      ? browser.origin.fetched_at_unix_ms
      : network.kind === 'offline'
        ? network.since_unix_ms
        : null,
  );

  const toolbarOff = $derived(apiChanged);
  const searchOff = $derived(toolbarOff || offline);

</script>

<svelte:window onkeydown={handleWindowKeydown} />

<div class="browse">
  <div class="head">
    <h1>{t('catalog-title')}</h1>
    {#if !local}
      <SegmentedControl
        options={sectionOptions}
        value={browser.section}
        label={t('catalog-section')}
        disabled={toolbarOff}
        onchange={(value) => {
          void browser.setSection(value as Section);
          return undefined;
        }}
      />
      <div class="search">
        <SearchField
          bind:this={search}
          bind:value={searchValue}
          label={t('catalog-search')}
          placeholder={apiChanged
            ? t('catalog-search-off')
            : offline
              ? t('catalog-search-offline')
              : browser.section === 'sound'
                ? t('catalog-search-sounds')
                : t('catalog-search')}
          clearLabel={t('catalog-search-clear')}
          shortcut="/"
          disabled={searchOff}
          busy={browser.loading && browser.searching}
          error={browser.searching && !browser.loading && items.length === 0 && !browser.error}
          resultText={browser.searching && !browser.loading ? t('catalog-results', { count: items.length }) : undefined}
          onsearch={(text) => {
            const clamped = clampSearch(text);
            searchValue = clamped;
            void browser.search(clamped);
          }}
        />
      </div>
    {/if}
  </div>

  {#if !local}
    <div class="toolbar">
      {#if browser.searching}
        <span {@attach tooltip({ text: t('catalog-sort-best-title') ?? '' })}>
          <Select
            options={[{ value: 'best', label: t('catalog-sort-best') }]}
            value="best"
            label={t('catalog-sort')}
            prefix={t('catalog-sort')}
            disabled
          />
        </span>
      {:else}
        <Select
          options={sortOptions}
          value={browser.sort}
          label={t('catalog-sort')}
          prefix={t('catalog-sort')}
          disabled={toolbarOff || offline}
          onchange={(value) => void browser.setSort(value as Sort)}
        />
      {/if}
      <Select
        options={categoryOptions}
        value={browser.category === null ? '' : String(browser.category)}
        label={t('catalog-category')}
        prefix={t('catalog-category')}
        disabled={toolbarOff || offline}
        onchange={(value) => void browser.setCategory(value === '' ? null : Number(value))}
      />
      {#if browser.heroes.length > 0}
        <Select
          options={heroOptions}
          value={browser.hero === null ? '' : String(browser.hero)}
          label={t('catalog-hero')}
          prefix={t('catalog-hero')}
          filterLabel={t('catalog-hero-filter')}
          filterPlaceholder={t('catalog-hero-filter')}
          emptyText={t('catalog-hero-none')}
          disabled={toolbarOff || offline}
          onchange={(value) => void browser.setHero(value === '' ? null : Number(value))}
        />
      {/if}
      <Filters {browser} disabled={toolbarOff} />
      {#if browser.loading}
        <span class="caption" role="status"><Spinner />{t(browser.section === 'sound' ? 'catalog-loading-sounds' : 'catalog-loading')}</span>
      {:else if nsfw === 'hidden'}
        <span class="caption"><EyeOff class="dl-icon" />{t('catalog-nsfw-caption-hidden')}</span>
      {:else if nsfw === 'blurred'}
        <span class="caption"><Eye class="dl-icon" />{t('catalog-nsfw-caption-blurred')}</span>
      {:else}
        <span class="caption"><Eye class="dl-icon" />{t('catalog-nsfw-caption-shown')}</span>
      {/if}
    </div>
  {/if}

  {#if local}
    <div class="state state--local">
      <div class="dl-empty narrow">
        <div class="dl-empty__glyph"><WifiOff class="dl-icon dl-icon--lg" /></div>
        <h2 class="dl-empty__title">{t('catalog-local-title')}</h2>
        <p class="dl-empty__text">{t('catalog-local-why')}</p>
        <p class="dl-empty__text">{t('catalog-local-do')}</p>
        <div class="dl-empty__actions">
          <Button label={t('catalog-local-enable')} variant="primary" onclick={enableCatalog} />
          <a class="dl-btn dl-btn--ghost" href="#/library">{t('catalog-local-library')}</a>
        </div>
      </div>
    </div>
  {:else if apiChanged}
    {@const text = describe(apiMessage ?? { id: 'E4', args: {} })}
    <div class="state">
      <div class="dl-empty dl-empty--error narrow" role="alert">
        <div class="dl-empty__glyph"><CircleX class="dl-icon dl-icon--lg" /></div>
        <h2 class="dl-empty__title">{text.what}</h2>
        <p class="dl-empty__text">{text.why}</p>
        <p class="dl-empty__text">{text.todo}</p>
        <div class="dl-empty__actions">
          <Button label={t('catalog-try-again')} variant="primary" icon={RefreshCw} onclick={() => void retry()} />
          <Button label={t('catalog-check-update')} icon={CircleArrowUp} onclick={() => router.open('settings', 'about')} />
        </div>
        {#if apiMessage?.args['log_id']}
          <div class="dl-dialog__details details">
            <span>{t('catalog-details')}</span><span class="dl-mono log">{apiMessage.args['log_id']}</span>
            <Button
              label={t('catalog-copy-id')}
              title={t('catalog-copy-id')}
              icon={Copy}
              iconOnly
              size="sm"
              variant="ghost"
              onclick={() => void navigator.clipboard?.writeText(String(apiMessage?.args['log_id']))}
            />
            <a class="dl-link" href="#/activity">{t('catalog-open-activity')}</a>
          </div>
        {/if}
      </div>
    </div>
  {:else}
    {#if network.kind === 'rate_limited'}
      {@const text = describe(network.message)}
      <Banner kind="warning" title={text.what} text={`${text.why ?? ''} ${t('catalog-rate-retry', { seconds: waitSeconds })}`}>
        {#snippet actions()}
          <Button label={t('catalog-retry-now')} size="sm" icon={RefreshCw} onclick={() => void retry()} />
        {/snippet}
      </Banner>
    {:else if savedAt !== null && offline}
      {@const text = describe({ id: 'N7', args: { saved: savedAt } })}
      <Banner kind="neutral" icon={WifiOff} title={text.what} text={text.why ?? undefined}>
        {#snippet actions()}
          <Button label={t('catalog-try-again')} size="sm" icon={RefreshCw} onclick={() => void retry()} />
        {/snippet}
      </Banner>
    {/if}

    {#if browser.loading}
      <div class="dl-card-grid skeletons" aria-hidden="true">
        {#each Array.from({ length: 12 }, (_, i) => i) as i (i)}
          <CardSkeleton index={i} />
        {/each}
      </div>
    {:else if browser.error && errorText}
      <div class="state" role="alert">
        <EmptyState icon={offline ? WifiOff : CircleX} title={errorText.what} text={errorText.why ?? undefined} error level={2}>
          {#snippet actions()}
            <Button label={t('catalog-try-again')} variant="primary" icon={RefreshCw} onclick={() => void retry()} />
          {/snippet}
        </EmptyState>
      </div>
    {:else if items.length === 0}
      <div class="state" role="status">
        <EmptyState
          icon={SearchX}
          title={browser.searching
            ? t('catalog-empty-search', { text: browser.searchText ?? '' })
            : browser.favouritesOnly
              ? t('catalog-empty-favourites')
              : t('catalog-empty')}
          text={filterNames.length > 0
            ? t('catalog-empty-filters', { filters: filterNames.join(', ') })
            : browser.searching
              ? t('catalog-empty-shorter')
              : undefined}
          level={2}
        >
          {#snippet actions()}
            {#if filterNames.length > 0}
              <Button label={t('catalog-clear-filters')} onclick={() => void browser.clearFilters()} />
            {/if}
            {#if browser.searching}
              <Button
                label={t('catalog-search-clear')}
                variant="ghost"
                onclick={() => {
                  searchValue = '';
                  void browser.clearSearch();
                }}
              />
            {/if}
          {/snippet}
        </EmptyState>
      </div>
    {:else}
      <div class="grid" bind:clientWidth={width}>
        <VirtualGrid
          {items}
          key={(card) => `${card.section}-${card.id}`}
          itemHeight={catalogCardHeight}
          minItemWidth={CARD_MIN_W}
          gap={GRID_GAP}
          padding={PADDING}
          label={t(browser.section === 'sound' ? 'catalog-grid-sounds' : 'catalog-grid')}
          bind:activeIndex
          initialScrollTop={browser.scrollTop}
          onscroll={(top) => (browser.scrollTop = top)}
          onrangechange={(start, end) => void browser.rangeChanged(start, end)}
        >
          <!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
          {#snippet item(card, _index, state)}
            {@const key = `${card.section}-${card.id}`}
            {@const blurred = card.nsfw && nsfw === 'blurred' && !browser.revealed.has(key)}
            {@const favourite = browser.isFavourite(card)}
            {@const date = dateOf(card)}
            <CatalogCard
              href={`#/catalog/${card.section}/${card.id}`}
              title={card.name}
              byline={byline(card)}
              thumb={thumbOf(card)}
              likes={browser.favouritesOnly ? '—' : formatNumber(card.likes)}
              likesLabel={t('catalog-likes', { count: card.likes })}
              date={date === null ? '' : relativeTime(date)}
              dateTitle={date === null ? '' : exactTime(date)}
              badges={badges(card)}
              {favourite}
              favouriteLabel={favourite ? t('catalog-favourite-remove') : t('catalog-favourite-add')}
              progressText={progressOf(card)}
              nsfw={blurred && card.thumbnail
                ? {
                    blurredThumb: imageUrl(card.thumbnail.small),
                    label: t('catalog-sensitive'),
                    hint: t('catalog-click-show'),
                  }
                : null}
              tabindex={state.active ? 0 : -1}
              onreveal={() => browser.revealed.add(key)}
              onfavourite={() => void browser.toggleFavourite(card)}
              onthumberror={() => void browser.thumbFailed(card)}
            />
          {/snippet}
        </VirtualGrid>
      </div>
      {#if browser.loadingMore}
        <p class="more" role="status"><Spinner />{t('catalog-loading-more')}</p>
      {:else if browser.moreError}
        <p class="more" role="status">
          {t('catalog-more-failed')}
          <button type="button" class="dl-link linkbtn" onclick={() => void browser.rangeChanged(0, items.length)}
            >{t('catalog-try-again')}</button
          >
        </p>
      {/if}
    {/if}
  {/if}
</div>

<style>
  /* The design's catalog <main>: a column with 12px gaps, 20px 24px padding, no bottom padding (the grid scrolls). */
  .browse {
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: 100%;
    min-width: 0;
    padding: 20px 24px 0;
    overflow: hidden;
  }

  .head {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 12px 16px;
  }

  h1 {
    margin: 0;
    font: var(--type-title-lg);
    color: var(--ink);
  }

  .search {
    flex: 1 1 260px;
    min-width: 0;
  }

  .toolbar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
  }

  .caption {
    margin-left: auto;
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .caption :global(.dl-icon) {
    width: 12px;
    height: 12px;
  }

  .caption :global(.dl-spinner) {
    margin-right: 4px;
  }

  .grid {
    flex: 1 1 auto;
    min-height: 0;
    margin: 0 -4px;
  }

  /* Full-size placeholder cards; the window clips the rows that don't fit. */
  .skeletons {
    flex: none;
    padding-top: 4px;
  }

  .state {
    flex: 1 1 auto;
    display: grid;
    align-content: start;
    justify-items: center;
    gap: 8px;
    padding-top: 32px;
  }

  .state--local {
    padding-top: 48px;
    border-top: 1px solid var(--line);
  }

  .narrow {
    max-width: 480px;
  }

  .details {
    margin-top: 8px;
    flex-wrap: wrap;
    justify-content: center;
  }

  .log {
    color: var(--ink-muted);
  }

  .more {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    margin: 0;
    padding: 8px 0 12px;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .linkbtn {
    padding: 0;
    background: none;
    border: 0;
    font: inherit;
  }
</style>
