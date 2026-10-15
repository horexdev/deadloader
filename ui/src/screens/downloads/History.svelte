<!-- "Finished this session" (board Downloads): what left the queue since deadloader started, newest first. New lines
     fade in; the list is cleared when deadloader closes. -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import { tooltip } from '../../lib/components/tooltip';
  import Ban from '@lucide/svelte/icons/ban';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import LibraryBig from '@lucide/svelte/icons/library-big';
  import RefreshCw from '@lucide/svelte/icons/refresh-cw';
  import Button from '../../lib/components/Button.svelte';
  import Count from '../../lib/components/Count.svelte';
  import type { FinishedDownload } from '../../lib/app.svelte';
  import { formatDate, formatNumber, formatTime, t } from '../../i18n';
  import { agoText, historyWord } from './text';

  interface Props {
    entries: FinishedDownload[];
    /** Unix ms, for "6 min ago". */
    now: number;
    /** Entries finished after this moment fade in. */
    freshAfter: number;
    onclear: () => void;
    onagain: (entry: FinishedDownload) => void;
  }

  const { entries, now, freshAfter, onclear, onagain }: Props = $props();
  const uid = $props.id();
</script>

<section class="history" aria-labelledby={`${uid}-title`}>
  <div class="head">
    <h2 id={`${uid}-title`}>{t('downloads-history')}</h2>
    {#if entries.length > 0}
      <Count value={formatNumber(entries.length)} muted />
      <Button variant="ghost" size="sm" class="clear" label={t('downloads-clear')} onclick={onclear} />
    {/if}
  </div>
  <p class="note"><Text value={t('downloads-history-note')} /></p>
  {#if entries.length === 0}
    <p class="empty">{t('downloads-history-empty')}</p>
  {/if}
  <div role="list" aria-labelledby={`${uid}-title`} class="list">
    {#each entries as entry (entry.key)}
      {@const ok = entry.outcome === 'installed' || entry.outcome === 'updated'}
      {@const word = historyWord(entry)}
      <div role="listitem" class={['line', entry.at > freshAfter && 'fresh']}>
        <span class={['icon', ok && 'icon--ok']}>
          {#if ok}<CircleCheck class="dl-icon" />{:else}<Ban class="dl-icon" />{/if}
        </span>
        <div class="text">
          <span class="mod" {@attach tooltip({ text: entry.name ?? '' })}>{entry.name}</span>
          <span class="word" {@attach tooltip({ text: word ?? '' })}
            >{word} · <span {@attach tooltip({ text: `${formatDate(entry.at)}, ${formatTime(entry.at)}` })}>{agoText(entry.at, now)}</span></span
          >
        </div>
        {#if ok || entry.outcome === 'kept'}
          <a
            class="dl-btn dl-btn--ghost dl-btn--icon dl-btn--sm"
            href="#/library"
            aria-label={t('downloads-show-in-library-of', { name: entry.name })}
            {@attach tooltip({ text: t('downloads-show-in-library') ?? '' })}><LibraryBig class="dl-icon" /></a
          >
        {:else if entry.outcome === 'cancelled' && entry.again}
          <Button
            variant="ghost"
            size="sm"
            iconOnly
            icon={RefreshCw}
            label={t('downloads-again-of', { file: entry.file })}
            title={t('downloads-again')}
            onclick={() => onagain(entry)}
          />
        {:else}
          <span></span>
        {/if}
      </div>
    {/each}
  </div>
</section>

<style>
  /* From DownloadsScreen.dc.html: the history panel. */
  .history {
    flex: 1 1 280px;
    min-width: 0;
    position: sticky;
    top: 0;
    align-self: flex-start;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 8px;
  }

  .head {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 8px 6px 12px;
  }

  .head :global(.clear) {
    margin-left: auto;
  }

  h2 {
    margin: 0;
    font: var(--type-title-sm);
    color: var(--ink);
  }

  .note {
    margin: 0;
    padding: 0 12px 8px;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .empty {
    margin: 0;
    padding: 12px;
    border-top: 1px solid var(--line);
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .list {
    border-top: 1px solid var(--line);
  }

  .line {
    display: grid;
    grid-template-columns: 16px minmax(0, 1fr) auto;
    gap: 10px;
    align-items: center;
    height: 52px;
    padding: 0 8px 0 12px;
    border-top: 1px solid var(--line);
  }

  .line:first-child {
    border-top: 0;
  }

  .fresh {
    animation: fresh var(--duration-slow) var(--ease-standard);
  }

  @keyframes fresh {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }

    to {
      opacity: 1;
      transform: none;
    }
  }

  .icon {
    display: grid;
    place-items: center;
    color: var(--ink-subtle);
  }

  .icon--ok {
    color: var(--success);
  }

  .text {
    display: grid;
    min-width: 0;
  }

  .mod,
  .word {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .mod {
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .word {
    font: var(--type-caption);
    color: var(--ink-subtle);
  }
</style>
