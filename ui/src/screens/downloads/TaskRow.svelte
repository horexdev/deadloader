<!-- One row of the queue (board Downloads): the mod, its stage, the progress bar and the lines under it, an inline
     error for a failed task (155), and the row's actions. -->
<script lang="ts">
  import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
  import CircleDot from '@lucide/svelte/icons/circle-dot';
  import CirclePlay from '@lucide/svelte/icons/circle-play';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Clock from '@lucide/svelte/icons/clock';
  import Download from '@lucide/svelte/icons/download';
  import Package from '@lucide/svelte/icons/package';
  import Pause from '@lucide/svelte/icons/pause';
  import RefreshCw from '@lucide/svelte/icons/refresh-cw';
  import Trash from '@lucide/svelte/icons/trash';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import WifiOff from '@lucide/svelte/icons/wifi-off';
  import X from '@lucide/svelte/icons/x';
  import Badge from '../../lib/components/Badge.svelte';
  import Button from '../../lib/components/Button.svelte';
  import Spinner from '../../lib/components/Spinner.svelte';
  import type { IconComponent } from '../../lib/components/types';
  import { describe, t } from '../../i18n';
  import { actionsOf, modHref, type Row, type Stage } from './tasks';
  import { metaOf, stageWord } from './text';

  interface Props {
    row: Row;
    /** Place among queued tasks (0 = next). */
    position: number | null;
    parallel: number;
    offline: boolean;
    onpause: () => void;
    onresume: () => void;
    oncancel: () => void;
    onretry: () => void;
    ondetails: () => void;
    onchoose: () => void;
  }

  const { row, position, parallel, offline, onpause, onresume, oncancel, onretry, ondetails, onchoose }: Props =
    $props();

  const ICONS: Partial<Record<Stage, IconComponent>> = {
    downloading: Download,
    extracting: Package,
    choice: CircleDot,
    paused: Pause,
    offline: WifiOff,
    disk: TriangleAlert,
    retrying: Clock,
    queued: Clock,
    failed: CircleX,
  };

  const name = $derived(row.kind === 'choice' ? row.choice.name : row.task.meta.mod_name || row.task.file_name);
  const file = $derived(row.kind === 'choice' ? row.choice.file : row.task.file_name);
  const href = $derived(row.kind === 'task' ? modHref(row.task) : null);
  const word = $derived(stageWord(row.stage));
  const meta = $derived(metaOf(row, position, parallel, offline));
  const actions = $derived(actionsOf(row.stage));
  const StageIcon = $derived(ICONS[row.stage]);
  const update = $derived(row.kind === 'task' && row.task.meta.replaces !== undefined ? row.task.meta.version : null);
  const failure = $derived(row.kind === 'task' && row.task.state.state === 'failed' ? describe(row.task.state.message) : null);
  const barState = $derived(
    meta.indeterminate ? 'dl-progress--indeterminate' : ['paused', 'offline', 'disk', 'retrying'].includes(row.stage) ? 'dl-progress--paused' : '',
  );
</script>

<div role="listitem" aria-label={`${name} — ${word}`} class={['row', `row--${row.stage}`]}>
  <span class="thumb" aria-hidden="true"></span>
  <div
    class={['dl-progress', barState]}
    role={meta.track ? 'progressbar' : 'group'}
    aria-valuemin={meta.track ? 0 : undefined}
    aria-valuemax={meta.track ? 100 : undefined}
    aria-valuenow={meta.track && !meta.indeterminate ? Math.round(meta.value * 100) : undefined}
    aria-label={file}
  >
    <div class="dl-progress__meta top">
      <span class="title">
        {#if href}
          <a class="name" {href} title={t('downloads-open-mod', { name })}>{name}</a>
        {:else}
          <span class="name">{name}</span>
        {/if}
        {#if update !== null}
          <Badge
            variant="info"
            icon={CircleArrowUp}
            label={update ? t('downloads-update', { version: update }) : t('downloads-update-plain')}
          />
        {/if}
      </span>
      <span class={['stage', `stage--${row.stage}`]}>
        {#if row.stage === 'verifying'}<Spinner />{:else if StageIcon}<StageIcon class="dl-icon" />{/if}{word}
      </span>
    </div>
    {#if meta.track}
      <div class="dl-progress__track"><div class="dl-progress__fill" style:--value={meta.value.toFixed(3)}></div></div>
    {/if}
    <div class="dl-progress__meta bottom">
      <span class="left">
        <span class="dl-mono file" title={file}>{file}</span>
        {#if meta.left}<span class="amounts" title={meta.left}>· {meta.left}</span>{/if}
      </span>
      {#if meta.right}<span class="right">{meta.right}</span>{/if}
    </div>
    {#if failure}
      <div class="error">
        <span><strong>{failure.what}</strong> {failure.why}</span>
        {#if failure.todo}<span class="todo">{failure.todo}</span>{/if}
        <div class="error-actions">
          <Button size="sm" icon={RefreshCw} label={t('downloads-retry')} onclick={onretry} />
          <Button size="sm" variant="ghost" label={t('downloads-show-details')} aria-haspopup="dialog" onclick={ondetails} />
        </div>
      </div>
    {/if}
  </div>
  <div class={['actions', failure && 'actions--top']}>
    {#if actions.choose}
      <Button variant="primary" size="sm" icon={CircleDot} label={t('downloads-choose')} aria-haspopup="dialog" onclick={onchoose} />
    {/if}
    {#if actions.pause}
      <Button
        variant="ghost"
        size="sm"
        iconOnly
        icon={Pause}
        label={t('downloads-pause-of', { file })}
        title={t('downloads-pause')}
        onclick={onpause}
      />
    {/if}
    {#if actions.resume}
      <Button
        variant="ghost"
        size="sm"
        iconOnly
        icon={CirclePlay}
        label={t('downloads-resume-of', { file })}
        title={t('downloads-resume')}
        onclick={onresume}
      />
    {/if}
    {#if actions.cancel}
      {@const label = row.kind === 'choice' ? t('downloads-aside-of', { file }) : t('downloads-cancel-of', { file })}
      <Button variant="ghost" size="sm" iconOnly icon={X} {label} title={label} onclick={oncancel} />
    {/if}
    {#if actions.remove}
      <Button
        variant="ghost"
        size="sm"
        iconOnly
        icon={Trash}
        label={t('downloads-remove-of', { file })}
        title={t('downloads-remove-of', { file })}
        onclick={oncancel}
      />
    {/if}
  </div>
</div>

<style>
  /* From DownloadsScreen.dc.html: the queue row. */
  .row {
    display: grid;
    grid-template-columns: 64px minmax(0, 1fr) auto;
    gap: 12px;
    align-items: start;
    min-height: 72px;
    padding: 10px 8px 10px 12px;
    box-sizing: border-box;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 8px;
  }

  .row--failed {
    border-color: var(--danger);
  }

  .row--choice {
    border-color: var(--info);
  }

  .thumb {
    display: block;
    width: 64px;
    height: 36px;
    margin-top: 2px;
    box-sizing: border-box;
    border-radius: 4px;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
  }

  .dl-progress {
    grid-template-columns: minmax(0, 1fr);
  }

  .top {
    flex-wrap: nowrap;
    align-items: center;
    gap: 12px;
  }

  .title {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
    flex: 1 1 auto;
  }

  .title :global(.dl-badge) {
    flex: none;
  }

  .name {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font: var(--type-body-strong);
    color: var(--ink);
    text-decoration: none;
    border-radius: 2px;
  }

  a.name:hover {
    text-decoration: underline;
    text-underline-offset: 2px;
    text-decoration-thickness: 1px;
  }

  a.name:focus-visible {
    outline: 2px solid var(--focus);
    outline-offset: 2px;
  }

  .stage {
    flex: none;
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font: var(--type-caption-strong);
    color: var(--ink-muted);
    white-space: nowrap;
  }

  .stage--choice {
    color: var(--info);
  }

  .stage--paused,
  .stage--queued {
    color: var(--ink-subtle);
  }

  .stage--disk {
    color: var(--warning);
  }

  .stage--failed {
    color: var(--danger);
  }

  .bottom {
    align-items: center;
  }

  .left {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
    flex: 1 1 auto;
  }

  .file {
    font: var(--type-mono-sm);
    color: var(--ink-muted);
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .amounts {
    flex: 0 1 auto;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .right {
    flex: none;
  }

  .error {
    display: grid;
    gap: 4px;
    margin-top: 6px;
    padding: 8px 10px;
    background: var(--danger-soft);
    border-radius: 6px;
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .error strong {
    font-weight: 600;
    color: var(--ink);
  }

  .todo {
    color: var(--ink);
  }

  .error-actions {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px 12px;
    margin-top: 4px;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 4px;
    align-self: center;
  }

  .actions--top {
    align-self: start;
  }
</style>
