<!-- S4 Library (spec 02-screens, boards Library*, Pending*, DialogD5/D6/D10/D11): the selected profile's mods in
     priority order, with the header, the states (empty, game not found, game running, profile switch, applying), tabs
     and search, the mod details, the conflicts panel (C1), deleting (D6), the pending changes (S9), foreign files
     (F9, D11), gameinfo.gi changed by another program (D10) and updates (D5). `#/library/<mod id>` opens that mod's
     details, and its D5 when an update is available (the catalog's Update button). -->
<script lang="ts">
  import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
  import FolderPlus from '@lucide/svelte/icons/folder-plus';
  import Gamepad2 from '@lucide/svelte/icons/gamepad-2';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import Layers from '@lucide/svelte/icons/layers';
  import LayoutGrid from '@lucide/svelte/icons/layout-grid';
  import LibraryBig from '@lucide/svelte/icons/library-big';
  import RefreshCw from '@lucide/svelte/icons/refresh-cw';
  import SearchX from '@lucide/svelte/icons/search-x';
  import Undo2 from '@lucide/svelte/icons/undo-2';
  import { onMount } from 'svelte';
  import {
    api,
    library as libraryApi,
    links,
    updates as updatesApi,
    type Changed,
    type ForeignItem,
    type GameinfoStatus,
    type ModView,
    type Pending,
    type PendingChange,
    type RemovalPlan,
    type UpdatePick,
  } from '../../lib/api';
  import { app } from '../../lib/app.svelte';
  import { Banner, Button, Count, EmptyState, PathTooltip, ProgressBar, SearchField, Tabs } from '../../lib/components';
  import { router } from '../../lib/router.svelte';
  import { formatDate, formatNumber, formatSize, formatTime, t } from '../../i18n';
  import Conflicts, { type Done } from './Conflicts.svelte';
  import DeleteDialog, { type Deletion } from './DeleteDialog.svelte';
  import Details from './Details.svelte';
  import Foreign from './Foreign.svelte';
  import GameinfoDialog from './GameinfoDialog.svelte';
  import ModList from './ModList.svelte';
  import PendingPanel from './PendingPanel.svelte';
  import TakeDialog from './TakeDialog.svelte';
  import UpdateDialog from './UpdateDialog.svelte';
  import { progressFiles, progressLeft, progressStep, progressValue } from './progress';
  import { FILTERS, counts, fileLabel, hasUpdate, matches, updateOf, type Context, type FilterId, type Pair } from './model';

  let filter = $state<FilterId>('all');
  let query = $state('');
  let conflictsClosed = $state(false);
  let busy: string[] = $state([]);
  let done: Done[] = $state([]);
  let deleting = $state<{ mod: ModView; plan: RemovalPlan; files: string[] | null } | null>(null);
  let removing = $state(false);
  let search: SearchField | undefined = $state();
  let pending = $state<Pending | null>(null);
  let foreign = $state<ForeignItem[]>([]);
  let taking = $state<ForeignItem | null>(null);
  let takeBusy = $state(false);
  let gameinfo = $state<GameinfoStatus | null>(null);
  /** D10 is open; `waiting` after Steam was asked to verify. */
  let d10 = $state<{ waiting: boolean } | null>(null);
  let d10Dismissed = false;
  let checkingGameinfo = $state(false);
  let updating = $state<ModView | null>(null);
  let updateBusy = $state(false);
  /** The mod whose D5 the address already opened, so it opens once per visit. */
  let updateShownFor: string | null = null;

  const view = $derived(app.library);
  const mods = $derived(view?.mods ?? []);
  const order = $derived(mods.map((m) => m.id));
  const ctx: Context = $derived({ conflicts: view?.conflicts ?? [], updates: app.updates });
  const tally = $derived(counts(mods, ctx));
  const shown = $derived(mods.filter((m) => matches(m, filter, query, ctx)));
  const profile = $derived(view?.selected_profile ?? '');
  const running = $derived(app.game?.running === true);
  const missing = $derived(app.game !== null && !app.game.root);
  const local = $derived(app.settings?.catalog?.enabled !== true);
  const switched = $derived(
    view !== null && view.pending_count > 0 && view.applied_profile !== view.selected_profile,
  );
  const details = $derived(router.path[0] ? (mods.find((m) => m.id === router.path[0]) ?? null) : null);
  const conflictsOpen = $derived(filter === 'conflicts' && !conflictsClosed && !details);
  const updatesAvailable = $derived(mods.filter((m) => hasUpdate(m.id, app.updates)).length);
  const diskBytes = $derived(mods.reduce((sum, m) => sum + m.bytes, 0));

  const checkedText = $derived.by(() => {
    const at = app.updatesAt;
    if (at === null) {
      return t('library-not-checked');
    }
    const today = new Date(at).toDateString() === new Date().toDateString();
    return today
      ? t('library-checked-today', { time: formatTime(at) })
      : t('library-checked-on', { date: formatDate(at) });
  });

  onMount(() => {
    void loadForeign();
    void checkGameinfo(false);
    const key = (event: KeyboardEvent) => {
      const typing = event.target instanceof HTMLElement && event.target.closest('input, textarea, [contenteditable]');
      if (event.key === '/' && !typing && !event.ctrlKey && !event.altKey && !event.metaKey) {
        event.preventDefault();
        search?.focus();
      }
    };
    window.addEventListener('keydown', key);
    return () => window.removeEventListener('keydown', key);
  });

  // S9 follows the library while it is open.
  $effect(() => {
    if (app.pendingOpen) {
      void view;
      void app.game?.running;
      libraryApi
        .pending()
        .then((p) => (pending = p))
        .catch((error: unknown) => app.fail(error));
    }
  });

  // Apply or Play found gameinfo.gi changed by someone else (E3): D10.
  $effect(() => {
    if (app.gameinfoForeign) {
      app.gameinfoForeign = false;
      d10Dismissed = false;
      void checkGameinfo(false);
    }
  });

  // The catalog's Update button lands here with the mod (`#/library/<id>`): its D5.
  $effect(() => {
    const id = router.path[0] ?? null;
    if (id && id !== updateShownFor && app.updates) {
      const mod = mods.find((m) => m.id === id);
      if (mod && hasUpdate(mod.id, app.updates)) {
        updateShownFor = id;
        updating = mod;
      }
    }
    if (!id) {
      updateShownFor = null;
    }
  });

  // An update that arrived needs its files mapped (169).
  $effect(() => {
    const step = app.mapping;
    if (step) {
      updating = mods.find((m) => m.id === step.mod_id) ?? null;
    }
  });

  async function loadForeign(): Promise<void> {
    try {
      foreign = (await libraryApi.foreign()).items;
    } catch (error) {
      app.fail(error);
    }
  }

  async function leave(item: ForeignItem): Promise<void> {
    try {
      foreign = (await libraryApi.leaveForeign(item.name)).items;
    } catch (error) {
      app.fail(error);
    }
  }

  async function take(name: string): Promise<void> {
    if (!taking) {
      return;
    }
    const item = taking;
    takeBusy = true;
    try {
      const taken = await libraryApi.takeForeign(item.name, name);
      app.library = taken.changed.library;
      app.toast({ kind: 'success', title: t('library-taken', { name: item.name }) });
      taking = null;
      await loadForeign();
    } catch (error) {
      app.fail(error);
    } finally {
      takeBusy = false;
    }
  }

  function foreignFolder(item: ForeignItem): void {
    libraryApi.show({ kind: 'foreign', name: item.name }, false).catch((error: unknown) => app.fail(error));
  }

  /** D10: opens on a gameinfo.gi changed by another program, once per visit unless something finds it again. */
  async function checkGameinfo(fix: boolean): Promise<void> {
    checkingGameinfo = true;
    try {
      gameinfo = await libraryApi.gameinfo(fix);
      if (gameinfo.state === 'foreign') {
        if (!d10 && !d10Dismissed) {
          d10 = { waiting: false };
        }
      } else if (d10) {
        d10 = null;
        if (fix) {
          app.toast({ kind: 'success', title: t('library-d10-fixed'), text: t('library-d10-fixed-text') });
        }
      }
    } catch (error) {
      app.fail(error);
    } finally {
      checkingGameinfo = false;
    }
  }

  async function verify(): Promise<void> {
    try {
      await libraryApi.verifyGameFiles();
      d10 = { waiting: true };
    } catch (error) {
      app.fail(error);
    }
  }

  function gameinfoFile(asFile: boolean): void {
    libraryApi.show({ kind: 'gameinfo' }, asFile).catch((error: unknown) => app.fail(error));
  }

  // ---- S9

  async function pendingEdit(call: () => Promise<Changed>): Promise<void> {
    try {
      const changed = await call();
      app.library = changed.library;
    } catch (error) {
      app.fail(error);
    }
  }

  function revert(change: PendingChange): void {
    const aspect = change.kind === 'enable' || change.kind === 'disable' ? 'enabled' : change.kind === 'order' ? 'order' : 'files';
    void pendingEdit(() => libraryApi.revert(change.mod_id, aspect));
  }

  // ---- D5

  function updateAll(): void {
    updating = mods.find((m) => hasUpdate(m.id, app.updates)) ?? null;
  }

  async function install(mod: ModView, picks: UpdatePick[]): Promise<void> {
    updateBusy = true;
    try {
      const started = await updatesApi.install(mod.id, picks);
      updating = null;
      for (const step of started.steps) {
        app.updateStep(step);
      }
      if (started.tasks.length > 0) {
        app.toast({ kind: 'info', title: t('library-update-downloading', { name: mod.name }) });
      }
    } catch (error) {
      app.fail(error);
    } finally {
      updateBusy = false;
    }
  }

  async function map(mod: ModView, mappings: { old: string; new: string | null }[], version: string | null): Promise<void> {
    updateBusy = true;
    try {
      const changed = await updatesApi.map(mod.id, mappings, version);
      app.library = changed.library;
      app.mapping = null;
      updating = null;
      app.toast({ kind: 'success', title: t('library-updated', { name: mod.name }), text: t('library-updated-text') });
    } catch (error) {
      app.fail(error);
    } finally {
      updateBusy = false;
    }
  }

  function closeUpdate(): void {
    updating = null;
    app.mapping = null;
  }

  function name(id: string): string {
    return mods.find((m) => m.id === id)?.name ?? id;
  }

  /** An edit: the library comes back; with `undo`, a toast says what changed and can take it back (138). */
  async function edit(id: string, call: () => Promise<Changed>, said?: { title: string; undo: () => void }) {
    busy = [...busy, id];
    try {
      const changed = await call();
      app.library = changed.library;
      if (said && changed.applied) {
        app.toast({
          kind: 'success',
          title: said.title,
          action: { label: t('library-undo'), icon: Undo2, onclick: said.undo },
        });
      }
    } catch (error) {
      app.fail(error);
    } finally {
      busy = busy.filter((x) => x !== id);
    }
  }

  function toggle(mod: ModView, enabled: boolean): void {
    void edit(mod.id, () => api.setEnabled(mod.id, enabled), {
      title: t(enabled ? 'library-toast-enabled' : 'library-toast-disabled', { name: mod.name }),
      undo: () => void edit(mod.id, () => api.setEnabled(mod.id, !enabled)),
    });
  }

  function move(mod: ModView, index: number): void {
    void edit(mod.id, () => api.moveMod(mod.id, index));
  }

  function variant(mod: ModView, file: string): void {
    const before = mod.variant_groups.find((g) => g.files.includes(file))?.active;
    void edit(mod.id, () => api.switchVariant(mod.id, file), {
      title: t('library-toast-variant', { name: mod.name, variant: fileLabel(mod, file) }),
      undo: () => {
        if (before) {
          void edit(mod.id, () => api.switchVariant(mod.id, before));
        }
      },
    });
  }

  async function addFromDisk(): Promise<void> {
    try {
      const outcomes = await api.addFromDisk({ title: t('library-add-title'), filter: t('library-add-filter') });
      app.imported(outcomes);
      await app.reloadLibrary();
    } catch (error) {
      app.fail(error);
    }
  }

  function open(mod: ModView): void {
    router.open('library', mod.id);
  }

  function closeDetails(): void {
    router.go('library');
  }

  async function askDelete(mod: ModView, files: string[] | null): Promise<void> {
    try {
      const plan = await api.removalPlan(mod.id, files ?? undefined);
      deleting = { mod, plan, files };
    } catch (error) {
      app.fail(error);
    }
  }

  async function confirmDelete(deletion: Deletion): Promise<void> {
    if (!deleting) {
      return;
    }
    const { mod, files } = deleting;
    removing = true;
    try {
      await libraryApi.remove(mod.id, files, deletion);
      app.toast({
        kind: 'success',
        title: files ? t('library-deleted-files', { name: mod.name, count: files.length }) : t('library-deleted', { name: mod.name }),
      });
      deleting = null;
      if (!files) {
        router.go('library');
      }
      await app.reloadLibrary();
    } catch (error) {
      app.fail(error);
    } finally {
      removing = false;
    }
  }

  async function rename(mod: ModView, next: string): Promise<void> {
    try {
      app.library = await links.rename(mod.id, next);
    } catch (error) {
      app.fail(error);
    }
  }

  async function link(mod: ModView, url: string) {
    try {
      const report = await links.link(mod.id, url);
      await app.reloadLibrary();
      return { matched: report.archives.filter((a) => a.remote_file_id !== null).length, total: report.archives.length };
    } catch (error) {
      return error as import('../../lib/api').LinkFailure;
    }
  }

  function showFolder(mod: ModView): void {
    libraryApi.show({ kind: 'mod_files', mod_id: mod.id }, false).catch((error: unknown) => app.fail(error));
  }

  function modPage(mod: ModView): void {
    if (mod.source.kind === 'gamebanana') {
      router.open('catalog', mod.source.section, String(mod.source.mod_id));
    }
  }

  // ---- C1

  function conflictMove(pair: Pair): void {
    const loser = mods.find((m) => m.id === pair.loser);
    const winnerIndex = order.indexOf(pair.winner);
    if (!loser || winnerIndex < 0) {
      return;
    }
    done = [...done, { pair, kind: 'move', who: loser.id, before: order.indexOf(loser.id) }];
    void edit(loser.id, () => api.moveMod(loser.id, winnerIndex));
  }

  function conflictDisable(pair: Pair): void {
    done = [...done, { pair, kind: 'disable', who: pair.winner, before: order.indexOf(pair.winner) }];
    void edit(pair.winner, () => api.setEnabled(pair.winner, false));
  }

  function conflictUndo(d: Done): void {
    done = done.filter((x) => x !== d);
    if (d.kind === 'move') {
      void edit(d.who, () => api.moveMod(d.who, d.before));
    } else {
      void edit(d.who, () => api.setEnabled(d.who, true));
    }
  }

  function selectFilter(id: string): void {
    filter = id as FilterId;
    conflictsClosed = false;
    done = [];
  }

  const tabs = $derived(
    FILTERS.map((id) => ({ id, label: t(`library-tab-${id}`), count: formatNumber(tally[id]) })),
  );
  const selectedRows = $derived(
    details ? [details.id] : conflictsOpen ? [...new Set(ctx.conflicts.flatMap((c) => [c.winner_mod, c.loser_mod]))] : [],
  );
</script>

<div class="screen">
  <div class="main">
    <div class="header">
      <div class="heading">
        <h1>{t('library-title')}</h1>
        <div class="meta">
          <span>{t('library-profile')} <strong>{profile}</strong></span>
          {#if mods.length > 0}
            <span class="sep" aria-hidden="true">·</span>
            <span>{t('library-enabled-count', { enabled: tally.enabled, total: tally.all })}</span>
            <span class="sep" aria-hidden="true">·</span>
            <span>{t('library-disk', { size: formatSize(diskBytes) })}</span>
          {/if}
          <span class="sep" aria-hidden="true">·</span>
          {#if app.game?.root}
            <span class="path"><PathTooltip path={app.game.root} /></span>
          {:else if missing}
            <span class="danger">{t('library-game-not-found')}</span>
          {/if}
        </div>
      </div>
      {#if mods.length > 0}
        <div class="buttons">
          <Button icon={FolderPlus} label={t('library-add-disk')} onclick={() => void addFromDisk()} />
          {#if !local}
            <Button
              icon={RefreshCw}
              label={app.checking ? t('library-checking') : t('library-check')}
              busy={app.checking}
              title={checkedText}
              onclick={() => void app.checkUpdates()}
            />
          {/if}
          <Button
            icon={CircleArrowUp}
            label={t('library-update-all')}
            unavailable={updatesAvailable === 0 || app.checking || app.applying}
            aria-haspopup="dialog"
            onclick={updateAll}
          >
            {#if updatesAvailable > 0}<Count value={formatNumber(updatesAvailable)} muted />{/if}
          </Button>
        </div>
      {/if}
    </div>

    {#if missing}
      <Banner kind="error" title={t('library-missing-what')} text={t('library-missing-why')}>
        {#snippet actions()}
          <Button variant="primary" size="sm" icon={FolderOpen} label={t('library-choose-folder')} onclick={() => router.go('settings')} />
          <Button variant="ghost" size="sm" label={t('library-search-again')} onclick={() => void app.refreshGame()} />
        {/snippet}
      </Banner>
    {/if}
    {#if running}
      <Banner kind="warning" icon={Gamepad2} title={t('library-running-what')} text={t('library-running-why')}>
        {#snippet actions()}
          <Button size="sm" label={t('library-pending')} aria-haspopup="dialog" onclick={() => (app.pendingOpen = !app.pendingOpen)}>
            {#if view?.pending_count}<Count value={formatNumber(view.pending_count)} muted />{/if}
          </Button>
        {/snippet}
      </Banner>
    {:else if switched && !app.playing && !app.applying}
      <Banner
        kind="neutral"
        icon={Layers}
        title={t('library-switch-what', { name: profile })}
        text={view?.applied_profile
          ? t('library-switch-why', { name: profile, applied: view.applied_profile })
          : t('library-switch-why-first', { name: profile })}
      >
        {#snippet actions()}
          <Button size="sm" label={t('library-review-switch')} aria-haspopup="dialog" onclick={() => (app.pendingOpen = true)} />
        {/snippet}
      </Banner>
    {/if}
    {#if app.playing || app.applying || app.progress}
      <div class="dl-banner dl-banner--neutral applying" role="status" aria-busy="true">
        <span class="dl-banner__text">
          <strong>{t('library-applying-what', { name: profile })}</strong>
          <span class="dl-banner__why">{t('library-applying-why')}</span>
        </span>
        <div class="progress">
          {#if app.progress}
            <ProgressBar
              value={progressValue(app.progress)}
              label={t('library-applying-what', { name: profile })}
              lead={progressStep(app.progress)}
              detail={progressFiles(app.progress) || undefined}
              rate={progressLeft(app.progress) || undefined}
            />
          {:else}
            <ProgressBar value={0} state="indeterminate" label={t('library-applying-what', { name: profile })} />
          {/if}
        </div>
      </div>
    {/if}

    {#if view && mods.length === 0}
      <div class="empty">
        <EmptyState
          icon={LibraryBig}
          title={t('library-empty-title')}
          text={local ? t('library-empty-local-text') : t('library-empty-text')}
          level={2}
        >
          {#snippet actions()}
            {#if !local}
              <Button variant="primary" icon={LayoutGrid} label={t('library-browse')} onclick={() => router.go('catalog')} />
            {/if}
            <Button
              variant={local ? 'primary' : 'secondary'}
              icon={FolderPlus}
              label={t('library-add-disk')}
              onclick={() => void addFromDisk()}
            />
          {/snippet}
        </EmptyState>
      </div>
    {:else if view}
      <div class="bar">
        <div class="tabs"><Tabs {tabs} selected={filter} label={t('library-show')} onchange={selectFilter} /></div>
        <div class="search">
          <SearchField
            bind:this={search}
            label={t('library-search')}
            placeholder={t('library-search')}
            clearLabel={t('library-search-clear')}
            shortcut="/"
            debounceMs={150}
            onsearch={(q) => (query = q)}
          />
        </div>
      </div>
      {#if shown.length === 0}
        <EmptyState compact icon={SearchX} title={t('library-no-match-title')} text={t('library-no-match-text')} />
      {:else}
        <ModList
          mods={shown}
          {order}
          {ctx}
          {profile}
          selected={selectedRows}
          toggleDisabled={missing}
          {busy}
          ontoggle={toggle}
          onmove={move}
          onopen={open}
          onvariant={variant}
          onremovefile={(mod, file) => void askDelete(mod, [file])}
        />
      {/if}
    {/if}
    {#if foreign.length > 0}
      <Foreign items={foreign} ontake={(item) => (taking = item)} onleave={(item) => void leave(item)} onfolder={foreignFolder} />
    {/if}
  </div>

  {#if details}
    <Details
      mod={details}
      {profile}
      position={order.indexOf(details.id) + 1}
      total={order.length}
      {ctx}
      names={name}
      toggleDisabled={missing}
      ontoggle={(on) => toggle(details, on)}
      onvariant={(file) => variant(details, file)}
      onclose={closeDetails}
      ondelete={(files) => void askDelete(details, files)}
      onfolder={() => showFolder(details)}
      onmodpage={() => modPage(details)}
      onconflicts={() => {
        closeDetails();
        selectFilter('conflicts');
      }}
      onrename={(next) => rename(details, next)}
      onlink={(url) => link(details, url)}
      update={updateOf(details.id, app.updates)?.kind === 'update_available'}
      onupdate={() => (updating = details)}
    />
  {:else if conflictsOpen}
    <Conflicts
      {mods}
      {ctx}
      {done}
      onclose={() => (conflictsClosed = true)}
      onmove={conflictMove}
      ondisable={conflictDisable}
      onvariant={open}
      onundo={conflictUndo}
    />
  {/if}
  {#if app.pendingOpen}
    <PendingPanel
      {pending}
      {mods}
      {ctx}
      {profile}
      {running}
      {missing}
      applying={app.applying}
      onclose={() => (app.pendingOpen = false)}
      onrevert={revert}
      ondiscard={() => void pendingEdit(() => libraryApi.discard())}
      onapply={() => void app.apply()}
      onundoall={() => void app.undoApplied()}
    />
  {/if}
</div>

{#if taking}
  <TakeDialog item={taking} {profile} busy={takeBusy} onconfirm={(n) => void take(n)} oncancel={() => (taking = null)} />
{/if}
{#if d10 && gameinfo}
  <GameinfoDialog
    status={gameinfo}
    waiting={d10.waiting}
    checking={checkingGameinfo}
    onverify={() => void verify()}
    oncheck={() => void checkGameinfo(true)}
    onfile={() => gameinfoFile(true)}
    onfolder={() => gameinfoFile(false)}
    oncancel={() => {
      d10 = null;
      d10Dismissed = true;
    }}
  />
{/if}
{#if updating}
  {@const status = updateOf(updating.id, app.updates)}
  {@const mod = updating}
  <UpdateDialog
    {mod}
    status={app.mapping ? null : status?.kind === 'update_available' ? status : null}
    step={app.mapping && app.mapping.mod_id === mod.id ? app.mapping : null}
    busy={updateBusy}
    oninstall={(picks) => void install(mod, picks)}
    onmap={(mappings, version) => void map(mod, mappings, version)}
    oncancel={closeUpdate}
  />
{/if}

{#if deleting}
  <DeleteDialog
    name={deleting.mod.name}
    plan={deleting.plan}
    deletion={app.settings?.library?.deletion === 'recycle_bin' ? 'recycle_bin' : 'permanent'}
    busy={removing}
    onconfirm={(d) => void confirmDelete(d)}
    oncancel={() => (deleting = null)}
  />
{/if}

<style>
  /* From the design's library screen (LibraryStates, Library, LibraryEmpty). */
  .screen {
    position: relative;
    display: flex;
    height: 100%;
    min-height: 0;
  }

  .main {
    display: flex;
    flex-direction: column;
    gap: 12px;
    flex: 1 1 auto;
    min-width: 0;
    padding: 20px 24px 0;
    overflow: hidden;
  }

  .header {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
    gap: 12px 16px;
  }

  .heading {
    display: grid;
    gap: 4px;
    flex: 1 1 320px;
    min-width: 0;
  }

  h1 {
    margin: 0;
    font: var(--type-title-lg);
    color: var(--ink);
  }

  .meta {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
    font: var(--type-body-sm);
    color: var(--ink-muted);
    white-space: nowrap;
  }

  .meta strong {
    font-weight: 500;
    color: var(--ink);
  }

  .sep {
    color: var(--ink-disabled);
  }

  .path {
    min-width: 0;
  }

  .danger {
    color: var(--danger);
  }

  .buttons {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
  }

  .applying {
    flex: none;
  }

  .progress {
    flex: 1 0 100%;
    padding-bottom: 4px;
  }

  .bar {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-end;
    gap: 8px 16px;
    border-bottom: 1px solid var(--line);
  }

  .tabs :global(.dl-tabs) {
    border-bottom: 0;
    flex-wrap: wrap;
  }

  .search {
    flex: 0 1 260px;
    margin-left: auto;
    margin-bottom: 6px;
  }

  .empty {
    flex: 1 1 auto;
    display: grid;
    align-content: start;
    padding-top: 56px;
    border-top: 1px solid var(--line);
  }
</style>
