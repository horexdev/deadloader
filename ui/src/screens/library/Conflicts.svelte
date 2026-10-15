<!-- C1 conflicts panel (decisions 133, 150, 154; board LibraryConflicts): pairs of enabled mods changing the same game
     files, who wins, the files by folder, and actions in place. After an action the pair stays with "Done — … · Undo"
     and no reverse swap. -->
<script lang="ts">
  import ArrowDown from '@lucide/svelte/icons/arrow-down';
  import ArrowUp from '@lucide/svelte/icons/arrow-up';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import Undo2 from '@lucide/svelte/icons/undo-2';
  import X from '@lucide/svelte/icons/x';
  import type { ModView } from '../../lib/api';
  import { Badge, Button, EmptyState } from '../../lib/components';
  import { t } from '../../i18n';
  import { byFolder, pairs, variantGroups, type Context, type Pair } from './model';

  /** What the panel did to a pair, so it can say so and undo it. */
  export interface Done {
    pair: Pair;
    kind: 'move' | 'disable';
    /** The mod that moved above (move) or was switched off (disable). */
    who: string;
    /** Its index in the whole order before (move). */
    before: number;
  }

  interface Props {
    mods: ModView[];
    ctx: Context;
    /** The pairs acted on in this panel, newest last. */
    done: Done[];
    onclose: () => void;
    /** Move `loser` right above `winner`. */
    onmove: (pair: Pair) => void;
    ondisable: (pair: Pair) => void;
    onvariant: (mod: ModView) => void;
    onundo: (done: Done) => void;
  }

  const { mods, ctx, done, onclose, onmove, ondisable, onvariant, onundo }: Props = $props();

  const SHOWN_FILES = 3;
  let open: Record<string, boolean> = $state({});
  let more: Record<string, boolean> = $state({});

  const key = (p: Pair) => `${p.winner}|${p.loser}`;
  const mod = (id: string) => mods.find((m) => m.id === id);
  const position = (id: string) => mods.findIndex((m) => m.id === id) + 1;

  const same = (a: Pair, b: Pair) =>
    (a.winner === b.winner && a.loser === b.loser) || (a.winner === b.loser && a.loser === b.winner);

  /** The current pairs, plus pairs an action here resolved (the overlap is a fact, 150; a disabled one is gone, 154). */
  const shown = $derived.by(() => {
    const out: { pair: Pair; done: Done | null }[] = pairs(ctx.conflicts).map((pair) => ({
      pair,
      done: done.findLast((d) => same(d.pair, pair)) ?? null,
    }));
    for (const d of done) {
      if (!out.some((o) => same(o.pair, d.pair))) {
        out.push({ pair: d.pair, done: d });
      }
    }
    return out;
  });
</script>

<aside class="panel" aria-label={t('library-tab-conflicts')}>
  <div class="head">
    <div class="title">
      <h2>{t('library-tab-conflicts')}</h2>
      <span class="sub">{t('library-conflicts-sub')}</span>
    </div>
    <Button variant="ghost" size="sm" iconOnly icon={X} label={t('library-close-conflicts')} onclick={onclose} />
  </div>
  <div class="body">
    {#if shown.length === 0}
      <EmptyState compact icon={CircleCheck} title={t('library-no-conflicts-title')} text={t('library-no-conflicts')} />
    {/if}
    {#each shown as { pair, done: d } (key(pair))}
      {@const off = d?.kind === 'disable'}
      <!-- After a disable the pair keeps its order: the one switched off was the winner (154). -->
      {@const winner = mod(pair.winner)}
      {@const loser = mod(pair.loser)}
      {@const variantMod = [winner, loser].find((m) => m && variantGroups(m).length > 0)}
      <div class="pair">
        {#each [{ m: winner, wins: true }, { m: loser, wins: false }] as side (side.wins)}
          <div class="side">
            <span class="dl-badge dl-badge--outline">#{side.m ? position(side.m.id) : '?'}</span>
            <span class="name" title={side.m?.name}>{side.m?.name}</span>
            {#if off}
              <span class="dl-badge {side.wins ? 'dl-badge--outline' : 'dl-badge--success'}">
                {side.wins ? t('library-off') : t('library-in-use')}
              </span>
            {:else}
              <Badge
                icon={side.wins ? ArrowUp : ArrowDown}
                label={side.wins ? t('library-wins') : t('library-loses')}
                variant={side.wins ? 'success' : 'warning'}
              />
            {/if}
          </div>
        {/each}
        <p class="text">
          {off
            ? t('library-conflict-off', { count: pair.paths.length, off: winner?.name ?? '', other: loser?.name ?? '' })
            : t('library-conflict-text', { count: pair.paths.length, winner: winner?.name ?? '', loser: loser?.name ?? '' })}
        </p>
        <div class="folders">
          {#each byFolder(pair.paths) as group, g (group.folder)}
            {@const id = `${key(pair)}|${group.folder}`}
            {@const expanded = open[id] ?? g === 0}
            <button
              type="button"
              class={['dl-focusable', 'folder', g > 0 && 'border']}
              aria-expanded={expanded ? 'true' : 'false'}
              onclick={() => (open = { ...open, [id]: !expanded })}
            >
              {#if expanded}<ChevronDown class="dl-icon" />{:else}<ChevronRight class="dl-icon" />{/if}
              <span class="dl-mono grow">{group.folder || '/'}</span>
              <span class="caption">{t('library-files-count', { count: group.files.length })}</span>
            </button>
            {#if expanded}
              {#each more[id] ? group.files : group.files.slice(0, SHOWN_FILES) as name (name)}
                <div class="dl-part indent"><span>{name}</span><span></span></div>
              {/each}
              {#if !more[id] && group.files.length > SHOWN_FILES}
                <div class="more">
                  <button type="button" class="dl-link morelink" onclick={() => (more = { ...more, [id]: true })}>
                    {t('library-show-more', { count: group.files.length - SHOWN_FILES })}
                  </button>
                </div>
              {/if}
            {/if}
          {/each}
        </div>
        {#if d}
          <div class="done" role="status">
            <CircleCheck class="dl-icon" />
            <span class="grow wrap">
              {off
                ? t('library-done-off', { off: winner?.name ?? '', other: loser?.name ?? '' })
                : t('library-done-wins', { name: mod(d.who)?.name ?? '' })}
            </span>
            <Button variant="ghost" size="sm" icon={Undo2} label={t('library-undo')} onclick={() => onundo(d)} />
          </div>
        {:else}
          <div class="actions">
            <Button
              variant="primary"
              size="sm"
              icon={ArrowUp}
              label={t('library-move-above', { name: loser?.name ?? '' })}
              title={t('library-move-above', { name: loser?.name ?? '' })}
              onclick={() => onmove(pair)}
            />
            <Button
              variant="ghost"
              size="sm"
              label={t('library-disable', { name: winner?.name ?? '' })}
              title={t('library-disable', { name: winner?.name ?? '' })}
              onclick={() => ondisable(pair)}
            />
            <Button
              variant="ghost"
              size="sm"
              label={t('library-pick-variant')}
              unavailable={!variantMod}
              title={variantMod ? undefined : t('library-no-variants')}
              onclick={() => variantMod && onvariant(variantMod)}
            />
          </div>
        {/if}
      </div>
    {/each}
    <p class="caption">{t('library-identical')}</p>
  </div>
</aside>

<style>
  /* From the design's conflicts panel (LibraryStates `conflicts`). */
  .panel {
    flex: none;
    width: 400px;
    display: flex;
    flex-direction: column;
    background: var(--surface);
    border-left: 1px solid var(--line);
    overflow: hidden;
  }

  @media (max-width: 1099px) {
    .panel {
      position: absolute;
      top: 0;
      right: 0;
      bottom: 0;
      z-index: 40;
      width: 380px;
      box-shadow: var(--shadow-3);
    }
  }

  .head {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 16px 12px 14px 20px;
    border-bottom: 1px solid var(--line);
  }

  .title {
    flex: 1 1 auto;
    display: grid;
    gap: 2px;
  }

  h2 {
    margin: 0;
    font: var(--type-title);
    color: var(--ink);
  }

  .sub {
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .body {
    flex: 1 1 auto;
    overflow: auto;
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    align-content: start;
    gap: 12px;
    padding: 16px 20px;
  }

  .pair {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: 12px;
    padding: 12px;
    background: var(--surface-raised);
    border: 1px solid var(--line);
    border-radius: 8px;
  }

  .side {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto;
    gap: 10px;
    align-items: center;
  }

  .name {
    font: var(--type-body-strong);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .text {
    margin: 0;
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .folders {
    display: grid;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: 6px;
  }

  .folder {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 8px;
    background: transparent;
    border: 0;
    color: var(--ink);
    cursor: pointer;
    text-align: left;
  }

  .border {
    border-top: 1px solid var(--line);
  }

  .grow {
    flex: 1 1 auto;
    min-width: 0;
  }

  .wrap {
    overflow-wrap: anywhere;
  }

  .caption {
    margin: 0;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .indent {
    padding-left: 32px;
  }

  .more {
    padding: 6px 8px 8px 32px;
    border-top: 1px solid var(--line);
  }

  .morelink {
    font: var(--type-caption);
    padding: 0;
    background: none;
    border: 0;
    cursor: pointer;
  }

  .done {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
    padding: 8px 10px;
    background: var(--success-soft);
    border-radius: 6px;
    font: var(--type-body-sm);
    color: var(--ink);
  }

  .done > :global(.dl-icon) {
    flex: none;
    color: var(--success);
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
</style>
