<!-- Activity's summary (design: ActivityScreen, "Today at a glance"): the hosts contacted today and how many requests
     each, what was blocked, and today's changes in the game folder. Two columns when there is room, one line each in
     the compact form (narrow windows, low heights). -->
<script lang="ts">
  import { tooltip } from '../../lib/components/tooltip';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import Folder from '@lucide/svelte/icons/folder';
  import Globe from '@lucide/svelte/icons/globe';
  import Unplug from '@lucide/svelte/icons/unplug';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import type { LogSummary } from '../../lib/api';
  import { formatList, formatNumber, t } from '../../i18n';
  import { sourceLong } from './labels';
  import { gameParts } from './model';

  interface Props {
    summary: LogSummary;
    /** The catalog is off: no requests now (today's earlier ones are still counted, 185). */
    local: boolean;
    compact: boolean;
    /** Two columns side by side (else stacked). */
    two: boolean;
    /** The host filter, to show its chip pressed. */
    host: string;
    onhost: (host: string) => void;
    onentry: (id: string) => void;
    onfiles: () => void;
  }

  const { summary, local, compact, two, host, onhost, onentry, onfiles }: Props = $props();

  const fresh = $derived(summary.empty);
  const hasHosts = $derived(summary.contacted.length > 0);
  const blockedTotal = $derived(summary.blocked.reduce((n, b) => n + b.requests, 0));
  const netLine = $derived(
    hasHosts
      ? t('activity-hosts-contacted', { count: summary.contacted.length })
      : local
        ? t('activity-net-none-local')
        : t('activity-net-none-yet'),
  );
  const noHostsWhat = $derived(local ? t('activity-no-requests-today') : t('activity-no-requests-yet'));
  const noHostsWhy = $derived(local ? t('activity-local-short') : fresh ? t('activity-never-contacted') : '');
  const game = $derived(summary.game);
  const gameLine = $derived(
    game.changes === 0
      ? t('activity-game-none')
      : [
          `${t('activity-game-changes', { count: game.changes })} ${t('activity-game-batches', { count: game.batches })}`,
          game.failed > 0 ? t('activity-game-failed', { count: game.failed }) : '',
        ]
          .filter(Boolean)
          .join(' · '),
  );
  const gameBreak = $derived(
    [
      ...gameParts(game).map(([op, count]) => t(`activity-game-${op}`, { count: formatNumber(count) })),
      ...(game.gameinfo > 0 ? [t('activity-game-gameinfo', { count: game.gameinfo })] : []),
    ].join(' · '),
  );
  const folder = $derived(game.folders[0] ?? null);
  const hostsLine = $derived(summary.contacted.map((h) => `${h.host} ${formatNumber(h.requests)}`).join(' · '));
  const lastBlocked = $derived(summary.blocked[0]?.last ?? null);
</script>

{#snippet netIcon()}
  {#if local}<Unplug class="dl-icon" />{:else}<Globe class="dl-icon" />{/if}
{/snippet}

{#if compact}
  <section class="act-sum act-sum--compact" aria-label={t('activity-summary')}>
    <div class="act-sum__head">
      {@render netIcon()}
      <strong>{t('activity-net-today')}</strong><span>{netLine}</span>
      <span class="act-sum__ok"
        ><CircleCheck class="dl-icon act-ok-icon" />{hasHosts ? t('activity-nothing-else') : noHostsWhat}</span
      >
      {#if hasHosts && lastBlocked}
        <span
          >{t('activity-blocked-short', { count: blockedTotal })}
          <button type="button" class="act-linkbtn" onclick={() => onentry(lastBlocked)}>{t('activity-show-entry')}</button
          ></span
        >
      {/if}
    </div>
    {#if hasHosts}<div class="act-mono act-sum__hosts">{hostsLine}</div>{/if}
    <div class="act-sum__head">
      <Folder class="dl-icon" />
      <strong>{t('activity-game-today')}</strong><span {@attach tooltip({ text: gameBreak ?? '' })}>{gameLine}</span>
      {#if game.changes > 0}
        <button type="button" class="act-linkbtn" onclick={onfiles}>{t('activity-show-in-files')}</button>
      {/if}
    </div>
  </section>
{:else}
  <section class={['act-sum', two ? 'act-sum--two' : 'act-sum--one']} aria-label={t('activity-summary')}>
    <div class="act-sum__col">
      <div class="act-sum__head">
        {@render netIcon()}
        <strong>{t('activity-net-today')}</strong><span>{netLine}</span>
      </div>
      {#if hasHosts}
        <div class="act-chips" role="group" aria-label={t('activity-hosts-label')}>
          {#each summary.contacted as h (h.host)}
            <button
              type="button"
              class="act-chip"
              aria-pressed={host === h.host ? 'true' : 'false'}
              {@attach tooltip({ text: (t('activity-host-title', { count: h.requests, source: sourceLong(h.source) })) ?? '' })}
              onclick={() => onhost(h.host)}>{h.host}<span class="act-chip__n">{formatNumber(h.requests)}</span></button
            >
          {/each}
        </div>
        <div class="act-only">
          <CircleCheck class="dl-icon act-ok-icon" />
          <span>
            <strong>{t('activity-nothing-else')}</strong>
            {#each summary.blocked as b (b.host)}
              {t('activity-blocked-lead', { count: b.requests })}
              <span class="act-mono act-strong">{b.host}</span>
              {t('activity-blocked-tail', { count: b.requests })}
              <button type="button" class="act-linkbtn" onclick={() => onentry(b.last)}>{t('activity-show-entry')}</button>
            {/each}
          </span>
        </div>
      {:else}
        <div class="act-only">
          <CircleCheck class="dl-icon act-ok-icon" />
          <span><strong>{noHostsWhat}</strong> {noHostsWhy}</span>
        </div>
      {/if}
    </div>
    <div class="act-sum__col">
      <div class="act-sum__head">
        <Folder class="dl-icon" />
        <strong>{t('activity-game-today')}</strong><span>{gameLine}</span>
      </div>
      {#if game.changes > 0}
        {#if gameBreak}<span class="act-sum__break">{gameBreak}</span>{/if}
        <div class="act-sum__folder">
          {#if folder}<PathTooltip path={folder} />{/if}
          <button type="button" class="act-linkbtn" onclick={onfiles}>{t('activity-show-in-files')}</button>
        </div>
        {#if game.folders.length > 1}<span class="act-sum__none">{formatList(game.folders.slice(1))}</span>{/if}
      {:else}
        <span class="act-sum__none">{t('activity-no-game-changes')}</span>
      {/if}
    </div>
  </section>
{/if}
