<!-- S6 Downloads (boards Downloads, DownloadsEmpty, DownloadsOffline, DownloadsDiskFull, DownloadsD4, DownloadsD12 and
     the German ones): the queue with stage, progress, speed and time left; pause, resume, cancel, retry; totals; the
     session's history; D4 for downloads waiting for a variant; offline and disk-full banners. -->
<script lang="ts">
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import CirclePlay from '@lucide/svelte/icons/circle-play';
  import Download from '@lucide/svelte/icons/download';
  import FolderPlus from '@lucide/svelte/icons/folder-plus';
  import LayoutGrid from '@lucide/svelte/icons/layout-grid';
  import LibraryBig from '@lucide/svelte/icons/library-big';
  import Pause from '@lucide/svelte/icons/pause';
  import RefreshCw from '@lucide/svelte/icons/refresh-cw';
  import Settings from '@lucide/svelte/icons/settings';
  import Trash from '@lucide/svelte/icons/trash';
  import WifiOff from '@lucide/svelte/icons/wifi-off';
  import { onDestroy } from 'svelte';
  import Banner from '../../lib/components/Banner.svelte';
  import Button from '../../lib/components/Button.svelte';
  import Count from '../../lib/components/Count.svelte';
  import EmptyState from '../../lib/components/EmptyState.svelte';
  import type { ErrorAction } from '../../lib/components/ErrorDialog.svelte';
  import { api, catalog, downloads, type DownloadAction, type TaskSnapshot } from '../../lib/api';
  import { app as shared, type AppState, type FinishedDownload, type PendingChoice } from '../../lib/app.svelte';
  import { describe, formatNumber, formatSize, t } from '../../i18n';
  import History from './History.svelte';
  import TaskRow from './TaskRow.svelte';
  import VariantDialog from './VariantDialog.svelte';
  import { diskProblem, modHref, queuePosition, rowsOf, totals } from './tasks';

  interface Props {
    /** The app state (tests pass their own). */
    app?: AppState;
  }

  const { app = shared }: Props = $props();

  const BACK_ONLINE_MS = 6000;
  const MINUTE_MS = 60_000;
  const SETTINGS_HREF = '#/settings/downloads';

  const offline = $derived(app.network.kind === 'offline');
  const rows = $derived(rowsOf(app.tasks, app.choices, offline));
  const sum = $derived(totals(app.tasks));
  const parallel = $derived(Number(app.settings?.downloads?.parallel ?? 4));
  const speedLimit = $derived(app.settings?.downloads?.speed_limit_kib_per_s as number | null | undefined);
  const limits = $derived(
    [
      t('downloads-limit-parallel', { count: parallel }),
      speedLimit ? t('downloads-limit-speed', { speed: formatSize(speedLimit * 1024) }) : t('downloads-limit-none'),
    ].join(' · '),
  );
  const allEmpty = $derived(app.tasks.length === 0 && app.choices.length === 0 && app.history.length === 0);
  const running = $derived(app.tasks.some((task) => ['downloading', 'queued', 'waiting'].includes(task.state.state)));
  const userPaused = $derived(app.tasks.some((task) => task.state.state === 'paused'));
  const disk = $derived(diskProblem(app.tasks));
  const diskText = $derived(disk ? describe(disk) : null);
  const mountedAt = Date.now();

  let choosing: PendingChoice | null = $state(null);
  let adding = $state(false);
  let back = $state(false);
  let backTimer: ReturnType<typeof setTimeout> | undefined;
  // svelte-ignore state_referenced_locally
  let wasOffline = offline;

  // "Back online." for a few seconds after the network returns while the screen is open.
  $effect(() => {
    const now = offline;
    if (wasOffline && !now) {
      back = true;
      clearTimeout(backTimer);
      backTimer = setTimeout(() => (back = false), BACK_ONLINE_MS);
    }
    wasOffline = now;
  });
  onDestroy(() => clearTimeout(backTimer));

  // "6 min ago" stays right while the history is on screen: one tick a minute, only then.
  let now = $state(Date.now());
  $effect(() => {
    if (app.history.length === 0) {
      return;
    }
    const timer = setInterval(() => (now = Date.now()), MINUTE_MS);
    return () => clearInterval(timer);
  });

  function control(task: TaskSnapshot, action: DownloadAction): void {
    downloads.control(task.id, action).catch((error: unknown) => app.fail(error));
  }

  function pauseAll(): void {
    for (const task of app.tasks) {
      if (['downloading', 'queued', 'waiting'].includes(task.state.state)) {
        control(task, 'pause');
      }
    }
  }

  function resumeAll(): void {
    for (const task of app.tasks.filter((x) => x.state.state === 'paused')) {
      control(task, 'resume');
    }
  }

  /** D12 for a failed task, with what fits its error (155: only on "Show details"). */
  function details(task: TaskSnapshot): void {
    if (task.state.state !== 'failed') {
      return;
    }
    const message = task.state.message;
    const remove: ErrorAction = {
      label: t('downloads-remove-task'),
      place: 'start',
      icon: Trash,
      run: () => control(task, 'cancel'),
    };
    const href = modHref(task);
    let primary: ErrorAction;
    if (message.id === 'E9') {
      primary = { label: t('downloads-open-settings'), place: 'primary', href: SETTINGS_HREF, icon: Settings };
    } else if (message.id === 'E24' && href) {
      primary = { label: t('downloads-open-mod-page'), place: 'primary', href };
    } else {
      primary = { label: t('downloads-retry'), place: 'primary', icon: RefreshCw, run: () => control(task, 'retry') };
    }
    app.showError(message, [remove, primary]);
  }

  function retryDisk(): void {
    for (const task of app.tasks) {
      const state = task.state;
      if ((state.state === 'waiting' || state.state === 'failed') && state.message.id === 'E12') {
        control(task, 'retry');
      }
    }
  }

  async function again(entry: FinishedDownload): Promise<void> {
    const source = entry.again;
    if (!source) {
      return;
    }
    try {
      await downloads.install(source.section, source.modId, source.fileId, entry.name, source.author);
      app.history = app.history.filter((h) => h.key !== entry.key);
    } catch (error) {
      app.fail(error);
    }
  }

  async function addFromDisk(): Promise<void> {
    adding = true;
    try {
      const results = await api.addFromDisk({ title: t('downloads-add-title'), filter: t('downloads-add-filter') });
      await app.reloadLibrary();
      for (const result of results) {
        if ('Err' in result) {
          app.fail(result.Err);
          continue;
        }
        const outcome = result.Ok;
        const mod = app.library?.mods.find((m) => m.id === outcome.mod_id);
        const name = mod?.name ?? outcome.mod_id;
        const file = mod?.files.find((f) => outcome.new_files.includes(f.id))?.archive_name ?? name;
        const key = `import-${outcome.mod_id}-${Date.now()}`;
        if (outcome.choices.length > 0) {
          const size = mod ? mod.files.filter((f) => outcome.new_files.includes(f.id)).reduce((n, f) => n + f.len, 0) : null;
          app.addChoice({ key, modId: outcome.mod_id, choices: outcome.choices, name, file, size });
        } else {
          app.remember({ key, name, file, outcome: 'installed', at: Date.now() });
        }
      }
    } catch (error) {
      app.fail(error);
    } finally {
      adding = false;
    }
  }

  async function pick(choice: PendingChoice, files: string[], variant: string): Promise<void> {
    choosing = null;
    const profile = app.library?.selected_profile ?? '';
    if (await app.chooseVariant(choice.key, files, variant)) {
      app.toast({
        kind: 'success',
        title: describe({ id: 'N2', args: { name: choice.name } }).what,
        text: t('downloads-d4-installed', { variant, profile }),
      });
    }
  }
</script>

<div class="screen">
  <div class="header">
    <div class="heading">
      <h1>{t('downloads-title')}</h1>
      <div class="stats">
        {#if !allEmpty}
          <span>{t('downloads-active')} <strong>{formatNumber(sum.active)}</strong></span>
          <span>{t('downloads-queued')} <strong>{formatNumber(sum.queued)}</strong></span>
          <span
            >{t('downloads-speed')}
            <strong>{sum.speed > 0 ? t('downloads-per-second', { size: formatSize(sum.speed) }) : '—'}</strong></span
          >
        {/if}
        <span class="limits"
          ><Settings class="dl-icon limits-icon" /><span>{limits}</span><a class="dl-link" href={SETTINGS_HREF}
            >{t('downloads-change-settings')}</a
          ></span
        >
      </div>
    </div>
    {#if !allEmpty}
      <div class="buttons">
        {#if running || !userPaused}
          <Button icon={Pause} label={t('downloads-pause-all')} unavailable={!running} onclick={pauseAll} />
        {:else}
          <Button icon={CirclePlay} label={t('downloads-resume-all')} onclick={resumeAll} />
        {/if}
        <Button icon={FolderPlus} label={t('downloads-add-disk')} busy={adding} onclick={() => void addFromDisk()} />
      </div>
    {/if}
  </div>

  {#if offline && !allEmpty}
    <Banner kind="neutral" icon={WifiOff} title={t('downloads-offline-title')} text={t('downloads-offline-why')}>
      {#snippet actions()}
        <Button
          size="sm"
          icon={RefreshCw}
          label={t('downloads-try-again')}
          onclick={() => void catalog.retryNow().catch(() => undefined)}
        />
      {/snippet}
    </Banner>
  {:else if back}
    <Banner kind="success" icon={CircleCheck} title={t('downloads-back-title')} text={t('downloads-back-why')} />
  {/if}
  {#if disk && diskText}
    <Banner kind="error" title={diskText.what} text={[diskText.why, diskText.todo].filter(Boolean).join(' ')}>
      {#snippet actions()}
        <a class="dl-btn dl-btn--sm link" href="#/library"><LibraryBig class="dl-icon" />{t('downloads-open-library')}</a>
        <Button size="sm" icon={RefreshCw} label={t('downloads-try-again')} onclick={retryDisk} />
      {/snippet}
    </Banner>
  {/if}

  <div class="scroll">
    {#if allEmpty}
      <div class="empty">
        <EmptyState icon={Download} title={t('downloads-empty-title')} text={t('downloads-empty-text')} level={2}>
          {#snippet actions()}
            <a class="dl-btn dl-btn--primary link" href="#/catalog"><LayoutGrid class="dl-icon" />{t('downloads-browse')}</a>
            <Button icon={FolderPlus} label={t('downloads-add-disk')} busy={adding} onclick={() => void addFromDisk()} />
          {/snippet}
        </EmptyState>
      </div>
    {:else}
      <div class="columns">
        <section class="queue" aria-labelledby="dl-queue-title">
          <div class="queue-head">
            <h2 id="dl-queue-title">{t('downloads-queue')}</h2>
            <Count value={formatNumber(rows.length)} muted />
          </div>
          {#if rows.length === 0}
            <div class="queue-empty">{t('downloads-queue-empty')}</div>
          {/if}
          <div role="list" aria-labelledby="dl-queue-title" class="rows">
            {#each rows as row (row.kind === 'task' ? `task-${row.task.id}` : row.choice.key)}
              <TaskRow
                {row}
                position={queuePosition(rows, row)}
                {parallel}
                {offline}
                onpause={() => row.kind === 'task' && control(row.task, 'pause')}
                onresume={() => row.kind === 'task' && control(row.task, 'resume')}
                oncancel={() => (row.kind === 'task' ? control(row.task, 'cancel') : app.putChoiceAside(row.choice.key))}
                onretry={() => row.kind === 'task' && control(row.task, 'retry')}
                ondetails={() => row.kind === 'task' && details(row.task)}
                onchoose={() => (choosing = row.kind === 'choice' ? row.choice : null)}
              />
            {/each}
          </div>
        </section>
        <History
          entries={app.history}
          {now}
          freshAfter={mountedAt}
          onclear={() => app.clearHistory()}
          onagain={(entry) => void again(entry)}
        />
      </div>
    {/if}
  </div>
</div>

{#if choosing}
  {@const choice = choosing}
  <VariantDialog
    {choice}
    mod={app.library?.mods.find((m) => m.id === choice.modId)}
    profile={app.library?.selected_profile ?? ''}
    onpick={(files, variant) => void pick(choice, files, variant)}
    oncancel={() => (choosing = null)}
  />
{/if}

<style>
  /* From DownloadsScreen.dc.html: the page layout. */
  .screen {
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: 100%;
    box-sizing: border-box;
    min-width: 0;
    padding: 20px 24px 0;
    overflow: hidden;
  }

  .header {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
    gap: 12px 16px;
    flex: none;
  }

  .heading {
    display: grid;
    gap: 6px;
    flex: 1 1 380px;
    min-width: 0;
  }

  h1 {
    margin: 0;
    font: var(--type-title-lg);
    color: var(--ink);
  }

  .stats {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 4px 16px;
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .stats strong {
    font-weight: 600;
    color: var(--ink);
  }

  .limits {
    display: inline-flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 4px 8px;
    color: var(--ink-subtle);
  }

  .limits :global(.limits-icon) {
    width: 14px;
    height: 14px;
  }

  .buttons {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
  }

  .screen > :global(.dl-banner) {
    flex: none;
  }

  .link {
    text-decoration: none;
  }

  .scroll {
    flex: 1 1 auto;
    min-height: 0;
    overflow: auto;
    margin: 0 -24px;
    padding: 4px 24px 24px;
  }

  .empty {
    padding-top: 72px;
  }

  .empty :global(.dl-empty) {
    padding-top: 0;
  }

  .columns {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
    gap: 16px 20px;
  }

  .queue {
    flex: 999 1 600px;
    min-width: 0;
    display: grid;
    gap: 8px;
  }

  .queue-head {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 28px;
  }

  h2 {
    margin: 0;
    font: var(--type-title-sm);
    color: var(--ink);
  }

  .queue-empty {
    padding: 16px;
    font: var(--type-body-sm);
    color: var(--ink-muted);
    background: var(--surface);
    border: 1px dashed var(--line-strong);
    border-radius: 8px;
  }

  .rows {
    display: grid;
    gap: 6px;
  }
</style>
