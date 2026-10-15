<!-- S9 Pending changes (decisions 17, 138, 143, 147; boards Pending*): the changes not in the game yet, grouped, each
     with Undo; Apply (off while the game runs), Discard all; a profile switch as one change that waits for Apply or
     Play; what applying does to the game folder; the result of the automatic apply after the game closed, with Undo. -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import { tooltip } from '../../lib/components/tooltip';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import Layers from '@lucide/svelte/icons/layers';
  import Undo2 from '@lucide/svelte/icons/undo-2';
  import X from '@lucide/svelte/icons/x';
  import type { ModView, Pending, PendingChange } from '../../lib/api';
  import { Button, Count, EmptyState } from '../../lib/components';
  import { formatNumber, formatTime, t } from '../../i18n';
  import FolderFiles from './FolderFiles.svelte';
  import { fileLabel, versionText, type Context } from './model';

  interface Props {
    pending: Pending | null;
    mods: ModView[];
    ctx: Context;
    profile: string;
    running: boolean;
    /** Deadlock wasn't found: nothing can be applied. */
    missing: boolean;
    applying: boolean;
    onclose: () => void;
    onrevert: (change: PendingChange) => void;
    ondiscard: () => void;
    onapply: () => void;
    onundoall: () => void;
  }

  const { pending, mods, ctx, profile, running, missing, applying, onclose, onrevert, ondiscard, onapply, onundoall }: Props =
    $props();

  const KINDS = ['enable', 'disable', 'order', 'variant', 'version'] as const;

  const applied = $derived(pending?.applied_after_game ?? null);
  const undoing = $derived(pending?.undoing ?? null);
  const sw = $derived(pending?.switch ?? null);
  const changes = $derived(pending?.changes ?? []);
  /** What the list shows: the waiting changes, else the changes applied after the game closed. */
  const mode = $derived(
    sw ? 'switch' : changes.length > 0 ? (undoing !== null ? 'undoing' : 'changes') : applied ? 'applied' : 'empty',
  );
  const count = $derived(sw ? 1 : changes.length);

  function mod(id: string): ModView | undefined {
    return mods.find((m) => m.id === id);
  }

  function name(id: string | null): string {
    return id ? (mod(id)?.name ?? id) : '';
  }

  function conflicting(a: string, b: string | null): boolean {
    return b !== null && ctx.conflicts.some((c) => (c.winner_mod === a && c.loser_mod === b) || (c.winner_mod === b && c.loser_mod === a));
  }

  function what(change: PendingChange): string {
    const m = mod(change.mod_id);
    const label = (id: string) => (m ? fileLabel(m, id) : id);
    switch (change.kind) {
      case 'enable':
        return t('library-change-on');
      case 'disable':
        return t('library-change-off');
      case 'order':
        if (change.above) {
          return conflicting(change.mod_id, change.above)
            ? t('library-change-above-wins', { name: name(change.above) })
            : t('library-change-above', { name: name(change.above) });
        }
        return t('library-change-below', { name: name(change.below) });
      case 'variant':
        return `${change.from.map(label).join(', ') || '—'} → ${change.to.map(label).join(', ') || '—'}`;
      case 'version': {
        const version = (id: string) => versionText(m?.files.find((f) => f.id === id)?.version) ?? label(id);
        return `${version(change.from)} → ${version(change.to)}`;
      }
    }
  }

  function groups(list: PendingChange[]) {
    return KINDS.map((kind) => ({ kind, items: list.filter((c) => c.kind === kind) })).filter((g) => g.items.length > 0);
  }

  const shown = $derived(mode === 'applied' ? (applied?.changes ?? []) : changes);
  const sub = $derived.by(() => {
    switch (mode) {
      case 'applied':
        return t('library-applied-sub', { time: formatTime(applied?.at_unix_ms ?? 0), count: applied?.changes.length ?? 0 });
      case 'undoing':
        return t('library-undoing-sub', { time: formatTime(undoing ?? 0), count: changes.length });
      case 'switch':
        return t('library-switch-sub');
      default:
        return running ? t('library-running-sub') : t('library-waiting-sub');
    }
  });
  const dot = $derived(mode === 'applied' ? 'success' : running ? 'warning' : 'info');
  const folder = $derived(pending?.folder ?? null);
  /** "Show the file list" is open. */
  let listOpen = $state(false);

  /** The end of a long path, as the design shows it: "…\Deadlock\game\citadel\addons". */
  function tail(path: string): string {
    const parts = path.split(/[\\/]/);
    return parts.length > 4 ? `…\\${parts.slice(-4).join('\\')}` : path;
  }
</script>

<div class="panel" role="dialog" aria-labelledby="pending-title">
  <div class="head">
    <h2 id="pending-title">{mode === 'applied' ? t('library-applied-title') : t('library-pending')}</h2>
    {#if mode !== 'applied' && mode !== 'empty'}<Count value={formatNumber(count)} />{/if}
    <span class="push"></span>
    <Button variant="ghost" size="sm" iconOnly icon={X} label={t('library-close')} onclick={onclose} />
  </div>
  {#if mode === 'empty'}
    <div class="empty">
      <EmptyState compact icon={CircleCheck} title={t('library-nothing-pending')} text={t('library-all-in-game', { name: profile })} />
    </div>
  {:else}
    <div class="intro">
      <p><span class={['dot', dot]} aria-hidden="true">●</span><span><Text value={sub} /></span></p>
    </div>
    <div class="list">
      {#if sw}
        <div class="group">
          <span class="label">{t('library-group-switch', { count: 1 })}</span>
          <div class="item">
            <span class="thumb profile" aria-hidden="true"><Layers class="dl-icon" /></span>
            <div class="text">
              <span class="name">{sw.from ? `${sw.from} → ${sw.to}` : sw.to}</span>
              <span class="what">{t('library-switch-what-happens', { on: sw.turned_on, off: sw.turned_off })}</span>
            </div>
            {#if sw.from}
              <Button variant="ghost" size="sm" iconOnly icon={Undo2} label={t('library-undo-this')} title={t('library-undo-this')} onclick={ondiscard} />
            {/if}
          </div>
        </div>
      {:else if mode === 'undoing'}
        <div class="group">
          <span class="label">{t('library-group-undo', { count: changes.length, time: formatTime(undoing ?? 0) })}</span>
          {#each changes as change (`${change.mod_id}-${change.kind}`)}
            {@render item(change, true)}
          {/each}
        </div>
      {:else}
        {#each groups(shown) as group (group.kind)}
          <div class="group">
            <span class="label">{t(`library-group-${group.kind}`, { count: group.items.length })}</span>
            {#each group.items as change (`${change.mod_id}-${change.kind}`)}
              {@render item(change, mode !== 'applied')}
            {/each}
          </div>
        {/each}
      {/if}
      {#if mode !== 'applied' && !running && folder}
        <div class="summary">
          <span class="label">{t('library-what-happens')}</span>
          <span class="body">{t('library-folder-summary', { added: folder.added, removed: folder.removed, renumbered: folder.renumbered })}</span>
          {#if folder.addons}
            <span class="dl-mono path" {@attach tooltip({ text: folder.addons ?? '' })}>{tail(folder.addons)}</span>
          {/if}
          {#if folder.files.length > 0}
            <button type="button" class="dl-link show" aria-expanded={listOpen ? 'true' : 'false'} onclick={() => (listOpen = !listOpen)}>
              {listOpen ? t('library-hide-file-list') : t('library-show-file-list')}
            </button>
            {#if listOpen}
              <FolderFiles files={folder.files} {mods} />
            {/if}
          {/if}
        </div>
      {/if}
    </div>
  {/if}
  <div class="foot">
    {#if mode === 'applied'}
      <Button icon={Undo2} label={t('library-undo-all')} onclick={onundoall} />
      <span class="push"></span>
      <Button variant="ghost" label={t('library-close')} onclick={onclose} />
    {:else if mode !== 'empty'}
      {#if !sw || sw.from}
        <Button
          variant="ghost"
          label={sw ? t('library-keep', { name: sw.from ?? '' }) : mode === 'undoing' ? t('library-keep-applied') : t('library-discard-all')}
          onclick={ondiscard}
        />
      {/if}
      <span class="push note">
        {running ? t('library-close-to-apply') : missing ? t('library-toggle-missing') : sw ? t('library-play-applies') : ''}
      </span>
      <Button
        variant="primary"
        label={sw ? t('library-apply-profile', { name: sw.to }) : t('library-apply')}
        unavailable={running || missing}
        busy={applying}
        onclick={onapply}
      />
    {:else}
      <span class="push"></span>
      <Button variant="ghost" label={t('library-close')} onclick={onclose} />
    {/if}
  </div>
</div>

{#snippet item(change: PendingChange, undo: boolean)}
  <div class="item">
    <span class="thumb" aria-hidden="true"></span>
    <div class="text">
      <span class="name">{change.name}</span>
      <span class="what"><Text value={what(change)} /></span>
    </div>
    {#if undo}
      <Button
        variant="ghost"
        size="sm"
        iconOnly
        icon={Undo2}
        label={t('library-undo-this')}
        title={t('library-undo-this')}
        onclick={() => onrevert(change)}
      />
    {/if}
  </div>
{/snippet}

<style>
  /* From the design's pending-changes panel (LibraryStates `pendingRunning`, `applied`, `profileSwitch`). */
  .panel {
    position: absolute;
    left: 8px;
    bottom: 12px;
    z-index: 300;
    display: flex;
    flex-direction: column;
    width: 420px;
    max-width: calc(100% - 24px);
    max-height: calc(100% - 24px);
    background: var(--surface-raised);
    border: 1px solid var(--line);
    border-radius: 12px;
    box-shadow: var(--shadow-3);
    overflow: hidden;
  }

  .head {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 16px 12px 8px 20px;
  }

  h2 {
    margin: 0;
    font: var(--type-title);
    color: var(--ink);
  }

  .push {
    margin-left: auto;
  }

  .intro {
    padding: 0 20px 12px;
  }

  .intro p {
    display: flex;
    gap: 8px;
    align-items: flex-start;
    margin: 0;
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .dot {
    flex: none;
    font-size: 10px;
    line-height: 18px;
  }

  .dot.success {
    color: var(--success);
  }

  .dot.warning {
    color: var(--warning);
  }

  .dot.info {
    color: var(--info);
  }

  .list {
    flex: 1 1 auto;
    overflow: auto;
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    align-content: start;
    gap: 12px;
    padding: 4px 12px 12px 20px;
    border-top: 1px solid var(--line);
  }

  .group {
    display: grid;
    gap: 4px;
    padding-top: 8px;
  }

  .label {
    font: var(--type-caption-strong);
    color: var(--ink-subtle);
  }

  .item {
    display: grid;
    grid-template-columns: 48px minmax(0, 1fr) auto;
    gap: 10px;
    align-items: center;
    min-height: 44px;
  }

  .thumb {
    display: grid;
    place-items: center;
    width: 48px;
    height: 27px;
    border-radius: 4px;
    background: var(--surface-sunken);
  }

  .profile {
    background: var(--accent-soft);
    border: 1px solid var(--accent-line);
    color: var(--accent-text);
  }

  .text {
    display: grid;
    min-width: 0;
  }

  .name {
    font: var(--type-body-strong);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .what {
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .summary {
    display: grid;
    gap: 6px;
    margin-top: 4px;
    padding: 10px 12px;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: 6px;
  }

  .body {
    font: var(--type-body-sm);
    color: var(--ink);
  }

  .show {
    justify-self: start;
    padding: 0;
    font: var(--type-caption);
    background: none;
    border: 0;
  }

  .path {
    color: var(--ink-subtle);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .empty {
    padding: 16px 20px 24px;
  }

  .foot {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
    padding: 12px 20px;
    background: var(--surface);
    border-top: 1px solid var(--line);
  }

  .note {
    font: var(--type-caption);
    color: var(--ink-subtle);
    max-width: 220px;
    text-align: right;
  }
</style>
