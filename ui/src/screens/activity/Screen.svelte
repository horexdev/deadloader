<!-- S7 Activity (design: ActivityScreen and its boards): what deadloader did — today's summary, then the network,
     files and app logs as virtualized tables with filters, search and a live view, and the details of an entry. The
     address can name an entry: `#/activity/<id>` opens it (error messages point at their log entry this way). -->
<script lang="ts">
  import { tooltip } from '../../lib/components/tooltip';
  import { notice } from '../../lib/notice.svelte';
  import ActivityIcon from '@lucide/svelte/icons/activity';
  import Archive from '@lucide/svelte/icons/archive';
  import ArchiveRestore from '@lucide/svelte/icons/archive-restore';
  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import CornerDownRight from '@lucide/svelte/icons/corner-down-right';
  import FilePenLine from '@lucide/svelte/icons/file-pen-line';
  import Files from '@lucide/svelte/icons/files';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import Info from '@lucide/svelte/icons/info';
  import Link from '@lucide/svelte/icons/link';
  import PencilLine from '@lucide/svelte/icons/pencil-line';
  import Search from '@lucide/svelte/icons/search';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import Unplug from '@lucide/svelte/icons/unplug';
  import { onMount } from 'svelte';
  import './activity.css';
  import Banner from '../../lib/components/Banner.svelte';
  import EmptyState from '../../lib/components/EmptyState.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import SearchField from '../../lib/components/SearchField.svelte';
  import SegmentedControl from '../../lib/components/SegmentedControl.svelte';
  import Select, { type SelectOption } from '../../lib/components/Select.svelte';
  import Toggle from '../../lib/components/Toggle.svelte';
  import VirtualGrid from '../../lib/components/VirtualGrid.svelte';
  import { ROW_H_COMPACT } from '../../lib/components/geometry';
  import type { IconComponent } from '../../lib/components/types';
  import { activity, type FileOp, type LogEntry } from '../../lib/api';
  import { app } from '../../lib/app.svelte';
  import { router } from '../../lib/router.svelte';
  import { formatNumber, t } from '../../i18n';
  import Details from './Details.svelte';
  import Summary from './Summary.svelte';
  import {
    changeText,
    clock,
    durationText,
    hopStatusText,
    initiatorLabel,
    instant,
    levelLabel,
    opLabel,
    purposeLabel,
    sizeText,
    sourceLabel,
    statusText,
  } from './labels';
  import {
    LEVELS,
    OPS,
    PURPOSES,
    RANGES,
    anyFilter,
    durationOf,
    entryId,
    hasHops,
    hopCounts,
    hopStatus,
    hostPath,
    networkStatus,
    rowKey,
    sizeOf,
    startTs,
    type Filters,
    type Row,
    type Tab,
  } from './model';
  import { ActivityView } from './view.svelte';

  const view = new ActivityView({ fail: (error) => app.fail(error) });

  /** F13: the wizard's network step, then the catalog is on. */
  function enableCatalog(): void {
    notice.open({ mode: 'catalog' });
  }

  let innerWidth = $state(typeof window === 'undefined' ? 1280 : window.innerWidth);
  let innerHeight = $state(typeof window === 'undefined' ? 800 : window.innerHeight);
  let mainWidth = $state(0);
  let search: SearchField | undefined = $state();
  let list: VirtualGrid<Row> | undefined = $state();
  let activeIndex = $state(0);

  const masking = $derived(app.settings?.logs?.mask_user_path !== false);
  const local = $derived(app.settings?.catalog?.enabled === false);
  const retention = $derived(Number(app.settings?.logs?.retention_days ?? 30));
  const empty = $derived(view.summary?.empty === true);
  const localNet = $derived(local && view.tab === 'network');
  // 185: in local mode the requests made before it was turned on stay listed under the local-mode banner; the panel
  // alone shows only when nothing was found without filters. The filters stay, so a longer range finds older ones.
  const localPanel = $derived(localNet && view.entries.length === 0 && !view.loading && !anyFilter(view.tab, view.filters) && !view.text);
  const showFilters = $derived(!empty);
  const narrow = $derived(innerWidth < 1100);
  const entry = $derived(view.selectedEntry);
  // The design's breakpoints are on the list's width: the window minus the sidebar, the details and the padding.
  const listW = $derived(Math.max(0, mainWidth - 48));
  const cols = $derived({ size: listW >= 660, trig: listW >= 800, src: listW >= 940 });
  const fcols = $derived({ size: listW >= 760, change: listW >= 680 });
  const compactSummary = $derived(listW < 760 || innerHeight < 700);
  const gridNet = $derived(
    ['20px', '64px', '128px', 'minmax(0, 1fr)', '100px', ...(cols.size ? ['64px', '64px'] : []), ...(cols.trig ? ['76px'] : []), ...(cols.src ? ['104px'] : [])].join(' '),
  );
  const gridFiles = $derived(
    ['64px', '128px', 'minmax(0, 1fr)', ...(fcols.size ? ['64px'] : []), '68px', ...(fcols.change ? [listW >= 900 ? '220px' : '150px'] : [])].join(' '),
  );
  const gridApp = '64px 104px minmax(0, 1fr)';

  const tabs = $derived.by(() => {
    const c = view.counts;
    const make = (key: Tab, label: string, count: string, errors: number) => ({ key, label, count, errors });
    return [
      make(
        'network',
        t('activity-tab-network'),
        local && !c?.network.requests ? t('activity-tab-off') : formatNumber(c?.network.requests ?? 0),
        c?.network.failed ?? 0,
      ),
      make('files', t('activity-tab-files'), formatNumber(c?.files.operations ?? 0), c?.files.failed ?? 0),
      make('app', t('activity-tab-app'), formatNumber(c?.app.events ?? 0), c?.app.errors ?? 0),
    ];
  });
  const tabButtons: HTMLButtonElement[] = $state([]);

  const rangeOptions = $derived(RANGES.map((r) => ({ value: r, label: t(`activity-range-${r}`) })));
  const purposeOptions = $derived<SelectOption[]>([
    { value: 'all', label: t('activity-all'), count: formatNumber(view.counts?.network.requests ?? 0) },
    ...PURPOSES.map((p) => ({ value: p, label: purposeLabel(p), count: formatNumber(view.counts?.network.purposes[p] ?? 0) })),
  ]);
  const hostOptions = $derived<SelectOption[]>([
    { value: 'all', label: t('activity-all'), count: formatNumber(view.counts?.network.requests ?? 0) },
    ...Object.entries(view.counts?.network.hosts ?? {})
      .sort((a, b) => b[1] - a[1])
      .map(([host, n]) => ({ value: host, label: host, count: formatNumber(n) })),
  ]);
  const opOptions = $derived<SelectOption[]>([
    { value: 'all', label: t('activity-all'), count: formatNumber(view.counts?.files.operations ?? 0) },
    ...OPS.map((op) => ({ value: op, label: opLabel(op), count: formatNumber(view.counts?.files.ops[op] ?? 0) })),
  ]);
  const levelOptions = $derived<SelectOption[]>([
    { value: 'all', label: t('activity-all'), count: formatNumber(view.counts?.app.events ?? 0) },
    ...LEVELS.map((l) => ({ value: l, label: levelLabel(l), count: formatNumber(view.counts?.app.levels[l] ?? 0) })),
  ]);
  const resultOptions = $derived([
    { value: 'all', label: t('activity-all') },
    { value: 'ok', label: t('activity-ok') },
    { value: 'error', label: t('activity-errors') },
  ]);
  const matches = $derived(
    view.cursor ? t('activity-matches-more', { count: formatNumber(view.entries.length) }) : t('activity-matches', { count: view.entries.length }),
  );
  const rows = $derived(view.rows);

  const OP_ICONS: Record<FileOp, IconComponent> = {
    hardlink: Link,
    copy: Files,
    rename: PencilLine,
    delete: Trash2,
    write: FilePenLine,
    backup: Archive,
    restore: ArchiveRestore,
  };

  onMount(() => {
    const target = decodeURIComponent(location.hash.replace(/^#\/?activity\/?/, '').split('/')[0] ?? '');
    void view.open(target || undefined);
    return app.onLog((kinds) => void view.onLog(kinds));
  });

  function setFilter<K extends keyof Filters>(key: K, value: Filters[K]) {
    void view.setFilter(key, value);
  }

  function pickTab(tab: Tab, focus = false) {
    void view.setTab(tab);
    if (focus) {
      tabButtons[['network', 'files', 'app'].indexOf(tab)]?.focus();
    }
  }

  function tabKey(event: KeyboardEvent) {
    const order: Tab[] = ['network', 'files', 'app'];
    const i = order.indexOf(view.tab);
    const next = { ArrowRight: (i + 1) % 3, ArrowLeft: (i + 2) % 3, Home: 0, End: 2 }[event.key];
    if (next !== undefined) {
      event.preventDefault();
      pickTab(order[next] as Tab, true);
    }
  }

  function toggleRow(id: string) {
    view.select(view.selected === id ? null : id);
  }

  function rowKeydown(event: KeyboardEvent, e: LogEntry) {
    const id = entryId(e);
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      view.select(id);
    } else if ((event.key === 'ArrowRight' || event.key === 'ArrowLeft') && hasHops(e)) {
      const want = event.key === 'ArrowRight';
      if (view.openHops.has(e.req) !== want) {
        event.preventDefault();
        view.toggleHops(e.req, want);
      }
    }
  }

  function windowKeydown(event: KeyboardEvent) {
    if (event.defaultPrevented) {
      return;
    }
    const target = event.target as HTMLElement | null;
    const typing = target?.tagName === 'INPUT' || target?.tagName === 'TEXTAREA';
    if (event.key === 'Escape' && view.selected !== null && !typing) {
      event.preventDefault();
      view.select(null);
    } else if ((event.key === '/' && !typing) || ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'f')) {
      event.preventDefault();
      search?.focus();
    }
  }

  async function openLogs() {
    try {
      const folder = await activity.openFolder();
      app.toast({ kind: 'neutral', icon: FolderOpen, title: t('activity-logs-opened'), text: folder });
    } catch (error) {
      app.fail(error);
    }
  }

  function onRange(_start: number, end: number) {
    if (end >= rows.length - 10) {
      void view.more();
    }
  }

  function showEntry(id: string) {
    void view.jump(id).then(() => {
      const index = rows.findIndex((r) => r.type === 'entry' && entryId(r.entry) === view.selected);
      if (index >= 0) {
        list?.scrollToIndex(index);
      }
    });
  }

  function showFiles() {
    view.select(null);
    void view.setTab('files');
  }

  function pickHost(host: string) {
    void (async () => {
      await view.setTab('network');
      await view.setFilter('host', view.filters.host === host ? 'all' : host);
    })();
  }
</script>

<svelte:window bind:innerWidth bind:innerHeight onkeydown={windowKeydown} />

<div class="act-screen">
  <div class="act-main" bind:clientWidth={mainWidth}>
    <div class="act-headline">
      <div class="act-headline__text">
        <h1 class="act-title">{t('activity-title')}</h1>
        {#if innerHeight >= 700}<p class="act-sub">{t('activity-sub', { days: retention })}</p>{/if}
      </div>
      <button type="button" class="dl-btn dl-btn--ghost" onclick={() => void openLogs()}
        ><FolderOpen class="dl-icon" />{t('activity-open-logs')}</button
      >
    </div>

    {#if view.summary}
      <Summary
        summary={view.summary}
        {local}
        compact={compactSummary}
        two={listW >= 760}
        host={view.filters.host}
        onhost={pickHost}
        onentry={showEntry}
        onfiles={showFiles}
      />
    {/if}

    <div class="act-tabbar">
      <div class="dl-tabs" role="tablist" aria-label={t('activity-title')}>
        {#each tabs as tab, i (tab.key)}
          <button
            bind:this={tabButtons[i]}
            type="button"
            class="dl-tab"
            role="tab"
            aria-selected={view.tab === tab.key ? 'true' : 'false'}
            tabindex={view.tab === tab.key ? 0 : -1}
            onclick={() => pickTab(tab.key)}
            onkeydown={tabKey}
            >{tab.label} <span class="dl-tab__count">{tab.count}</span>{#if tab.errors > 0}<span
                class="dl-tab__count dl-tab__count--error">{t('activity-tab-errors', { count: tab.errors })}</span
              >{/if}</button
          >
        {/each}
      </div>
      {#if showFilters}
        <div class="act-follow">
          {#if !view.follow && view.pending > 0}
            <button type="button" class="dl-btn dl-btn--sm" onclick={() => view.showNew()}
              >{t('activity-show-new', { count: view.pending })}</button
            >
          {/if}
          {#if !view.follow && view.pausedAt}
            <span class="act-follow__note" aria-live="polite">{t('activity-follow-paused', { time: clock(view.pausedAt) })}</span>
          {:else if listW >= 900}
            <span class="act-follow__note" aria-live="polite">{t('activity-follow-on')}</span>
          {/if}
          <Toggle
            label={t('activity-follow')}
            title={t('activity-follow-tip')}
            checked={view.follow}
            onchange={(on) => view.setFollow(on)}
          />
        </div>
      {/if}
    </div>

    <div class="act-filters">
      {#if showFilters}
        <Select
          options={rangeOptions}
          value={view.filters.range}
          label={t('activity-f-time')}
          prefix={t('activity-f-time')}
          onchange={(v) => setFilter('range', v as Filters['range'])}
        />
        {#if view.tab === 'network'}
          <Select
            options={purposeOptions}
            value={view.filters.purpose}
            label={t('activity-f-purpose')}
            prefix={t('activity-f-purpose')}
            onchange={(v) => setFilter('purpose', v as Filters['purpose'])}
          />
          <Select
            options={hostOptions}
            value={view.filters.host}
            label={t('activity-f-host')}
            prefix={t('activity-f-host')}
            onchange={(v) => setFilter('host', v)}
          />
        {:else if view.tab === 'files'}
          <Select
            options={opOptions}
            value={view.filters.op}
            label={t('activity-f-op')}
            prefix={t('activity-f-op')}
            onchange={(v) => setFilter('op', v as Filters['op'])}
          />
        {:else}
          <Select
            options={levelOptions}
            value={view.filters.level}
            label={t('activity-f-level')}
            prefix={t('activity-f-level')}
            onchange={(v) => setFilter('level', v as Filters['level'])}
          />
        {/if}
        {#if view.tab !== 'app'}
          <SegmentedControl
            options={resultOptions}
            value={view.filters.result}
            label={t('activity-f-result')}
            onchange={(v) => {
              setFilter('result', v as Filters['result']);
              return undefined;
            }}
          />
        {/if}
        {#if anyFilter(view.tab, view.filters)}
          <button type="button" class="dl-btn dl-btn--ghost dl-btn--sm" onclick={() => void view.clearFilters()}
            >{t('activity-clear-filters')}</button
          >
        {/if}
      {/if}
      <SearchField
        bind:this={search}
        label={t('activity-search')}
        placeholder={t('activity-search')}
        clearLabel={t('activity-search-clear')}
        value={view.text}
        resultText={view.text ? matches : undefined}
        shortcut="/"
        disabled={!showFilters}
        onsearch={(q) => void view.search(q)}
      />
    </div>

    {#if empty}
      <div class="act-table act-table--center">
        <EmptyState icon={ActivityIcon} level={2} title={t('activity-empty-title')} text={t('activity-empty-text')} />
      </div>
    {:else if localPanel}
      <div class="act-table act-table--center">
        <!-- EmptyState with the design's two paragraphs (why, then what to do). -->
        <div class="dl-empty">
          <div class="dl-empty__glyph"><Unplug class="dl-icon dl-icon--lg" /></div>
          <h2 class="dl-empty__title">{t('activity-local-title')}</h2>
          <p class="dl-empty__text">{t('activity-local-why')}</p>
          <p class="dl-empty__text">{t('activity-local-do')}</p>
          <div class="dl-empty__actions">
            <button type="button" class="dl-btn dl-btn--primary" onclick={enableCatalog}
              >{t('activity-enable-catalog')}</button
            >
          </div>
        </div>
      </div>
    {:else}
      {#if localNet}
        <Banner kind="neutral" icon={Unplug} title={t('activity-local-title')} text={t('activity-local-history')}>
          {#snippet actions()}
            <button type="button" class="dl-btn" onclick={enableCatalog}>{t('activity-enable-catalog')}</button>
          {/snippet}
        </Banner>
      {/if}
      {@const aria = t(`activity-${view.tab === 'network' ? 'net' : view.tab}-aria`)}
      {@const grid = view.tab === 'network' ? gridNet : view.tab === 'files' ? gridFiles : gridApp}
      <div class="act-table" aria-label={aria}>
        <div class="act-head" style:grid-template-columns={grid} aria-hidden="true">
          {#if view.tab === 'network'}
            <span><span class="dl-visually-hidden">{t('activity-c-redirects')}</span></span>
            <span>{t('activity-c-time')}</span>
            <span>{t('activity-c-purpose')}</span>
            <span>{t('activity-c-request')}</span>
            <span>{t('activity-c-status')}</span>
            {#if cols.size}<span class="act-num">{t('activity-c-size')}</span><span class="act-num"
                >{t('activity-c-duration')}</span
              >{/if}
            {#if cols.trig}<span class="act-pl">{t('activity-c-started-by')}</span>{/if}
            {#if cols.src}<span>{t('activity-c-source')}</span>{/if}
          {:else if view.tab === 'files'}
            <span>{t('activity-c-time')}</span>
            <span>{t('activity-c-op')}</span>
            <span>{t('activity-c-from-to')}</span>
            {#if fcols.size}<span class="act-num">{t('activity-c-size')}</span>{/if}
            <span>{t('activity-c-result')}</span>
            {#if fcols.change}<span>{t('activity-c-change')}</span>{/if}
          {:else}
            <span>{t('activity-c-time')}</span>
            <span>{t('activity-c-level')}</span>
            <span>{t('activity-c-message')}</span>
          {/if}
        </div>
        <div class="act-body">
          {#if rows.length === 0 && !view.loading}
            <EmptyState icon={Search} compact title={t('activity-no-match')} text={t('activity-no-match-text')}>
              {#snippet actions()}
                <button type="button" class="dl-btn" onclick={() => void view.clearAll()}>{t('activity-clear-all')}</button>
              {/snippet}
            </EmptyState>
          {:else}
            <VirtualGrid
              bind:this={list}
              bind:activeIndex
              items={rows}
              key={(row) => rowKey(row)}
              itemHeight={ROW_H_COMPACT}
              label={aria}
              item={rowView}
              onrangechange={onRange}
            />
          {/if}
        </div>
      </div>
    {/if}
  </div>

  {#if entry}
    <Details
      {entry}
      {masking}
      over={narrow}
      onclose={() => view.select(null)}
      onopenlogs={() => void openLogs()}
      onsettings={() => router.go('settings')}
    />
  {/if}
</div>

{#snippet rowView(row: Row, index: number, state: { active: boolean })}
  {#if row.type === 'hop'}
    {@const line = row.entry.lines[row.index]}
    {#if line}
      {@const parts = hostPath(line.url)}
      {@const status = hopStatus(line)}
      <div class="act-hop" data-row={index} style:grid-template-columns={gridNet}>
        <span></span>
        <span class="act-c act-hop__mark"><CornerDownRight class="dl-icon" /></span>
        <span class="act-c"
          >{row.index === 0
            ? t('activity-hop-request')
            : (line.attempt ?? 1) > 1
              ? t('activity-hop-retry', { n: line.hop + 1, try: line.attempt ?? 1 })
              : t('activity-hop-redirect', { n: row.index + 1 })}</span
        >
        <span class="act-url" {@attach tooltip({ text: line.url ?? '' })}
          ><span class="act-url__m">{line.method}</span><span class="act-url__x"><b>{parts.host}</b>{parts.path}</span></span
        >
        <span class="act-c"
          ><span class={['act-st', status.kind !== 'ok' && 'act-st--err']}
            >{#if status.kind !== 'ok'}<CircleX class="dl-icon" />{/if}{hopStatusText(line)}</span
          ></span
        >
        {#if cols.size}<span class="act-c act-num">{status.kind === 'blocked' ? '—' : sizeText(line.bytes)}</span><span
            class="act-c act-num">{status.kind === 'blocked' ? '—' : durationText(line.ms)}</span
          >{/if}
        {#if cols.trig}<span></span>{/if}
        {#if cols.src}<span class="act-c">{sourceLabel(line.host_source)}</span>{/if}
      </div>
    {/if}
  {:else}
    {@const e = row.entry}
    {@const id = entryId(e)}
    {@const ts = startTs(e)}
    <div
      class="act-row"
      data-row={index}
      role="button"
      tabindex={state.active ? 0 : -1}
      data-grid-focus
      aria-current={view.selected === id ? 'true' : undefined}
      style:grid-template-columns={e.kind === 'network' ? gridNet : e.kind === 'files' ? gridFiles : gridApp}
      onclick={() => toggleRow(id)}
      onkeydown={(event) => rowKeydown(event, e)}
    >
      {#if e.kind === 'network'}
        {@const first = e.lines[0]}
        {@const status = networkStatus(e)}
        {@const parts = hostPath(first?.url ?? '')}
        {@const open = view.openHops.has(e.req)}
        <span
          >{#if hasHops(e)}<button
              type="button"
              class="act-chev"
              tabindex="-1"
              aria-expanded={open ? 'true' : 'false'}
              aria-label={open ? t('activity-hide-redirects') : t('activity-show-redirects')}
              {@attach tooltip({ text: (open ? t('activity-hide-redirects') : t('activity-show-redirects')) ?? '' })}
              onclick={(event) => {
                event.stopPropagation();
                view.toggleHops(e.req);
              }}
              >{#if open}<ChevronDown class="dl-icon" />{:else}<ChevronRight class="dl-icon" />{/if}</button
            >{/if}</span
        >
        <span class="act-c act-t" {@attach tooltip({ text: `${instant(ts)} · ${id}` })}>{clock(ts)}</span>
        <span class="act-c act-strong">{first ? purposeLabel(first.purpose) : ''}</span>
        <span class="act-url" {@attach tooltip({ text: first?.url ?? '' })}
          ><span class="act-url__m">{first?.method}</span><span class="act-url__x"><b>{parts.host}</b>{parts.path}</span
          >{#if hasHops(e)}{@const counts = hopCounts(e)}<span class="act-url__hops"
              ><CornerDownRight class="dl-icon" />{[
                counts.redirects > 0 ? t('activity-redirects', { count: counts.redirects }) : '',
                counts.retries > 0 ? t('activity-retries', { count: counts.retries }) : '',
              ]
                .filter(Boolean)
                .join(' · ')}</span
            >{/if}</span
        >
        <span class="act-c"
          ><span class={['act-st', status.kind !== 'ok' && 'act-st--err']}
            >{#if status.kind !== 'ok'}<CircleX class="dl-icon" />{/if}{statusText(e)}</span
          ></span
        >
        {#if cols.size}<span class="act-c act-num">{sizeText(sizeOf(e))}</span><span class="act-c act-num"
            >{durationText(durationOf(e))}</span
          >{/if}
        {#if cols.trig}<span class="act-c act-pl">{first ? initiatorLabel(first.initiator) : ''}</span>{/if}
        {#if cols.src}<span class="act-c">{sourceLabel(first?.host_source)}</span>{/if}
      {:else if e.kind === 'files'}
        {@const r = e.record}
        {@const Icon = OP_ICONS[r.op]}
        <span class="act-c act-t" {@attach tooltip({ text: `${instant(ts)} · ${id}` })}>{clock(ts)}</span>
        <span class="act-c"><span class="act-op"><Icon class="dl-icon" />{opLabel(r.op)}</span></span>
        <span class="act-ft"
          >{#if r.src}<PathTooltip path={r.src} />{/if}{#if r.src && r.dst}<ArrowRight
              class="dl-icon"
              role="img"
              aria-label={t('activity-to')}
            />{/if}{#if r.dst}<PathTooltip path={r.dst} />{/if}</span
        >
        {#if fcols.size}<span class="act-c act-num">{sizeText(r.bytes)}</span>{/if}
        <span class="act-c"
          ><span class={['act-st', r.result !== 'ok' && 'act-st--err']}
            >{#if r.result !== 'ok'}<CircleX class="dl-icon" />{/if}{r.result === 'ok'
              ? t('activity-ok')
              : t('activity-lv-error')}</span
          ></span
        >
        {#if fcols.change}{@const change = changeText(r)}<span
            class={['act-c', !r.change && 'act-t']}
            {@attach tooltip({ text: (r.txn === undefined ? undefined : `${change} · ${r.txn}`) ?? '' })}>{change ?? '—'}</span
          >{/if}
      {:else}
        {@const r = e.record}
        <span class="act-c act-t" {@attach tooltip({ text: `${instant(ts)} · ${id}` })}>{clock(ts)}</span>
        <span class="act-c"
          ><span class={['act-st', `act-lv--${r.lvl}`]}
            >{#if r.lvl === 'error'}<CircleX class="dl-icon" />{:else if r.lvl === 'warn'}<TriangleAlert
                class="dl-icon"
              />{:else}<Info class="dl-icon" />{/if}{levelLabel(r.lvl)}</span
          ></span
        >
        <span class="act-c act-strong" {@attach tooltip({ text: r.msg ?? '' })}>{r.msg}</span>
      {/if}
    </div>
  {/if}
{/snippet}
