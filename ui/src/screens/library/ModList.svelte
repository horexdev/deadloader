<!-- The library's ordered list (S4, LibraryRow.md): virtualized rows, "higher wins" note, live region, reorder by
     pointer (the grip) and keyboard (Space grab, ↑↓, Space drop, Esc, Alt+↑↓), parts of a mod as one block. -->
<script lang="ts">
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import ChevronUp from '@lucide/svelte/icons/chevron-up';
  import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
  import Ellipsis from '@lucide/svelte/icons/ellipsis';
  import Files from '@lucide/svelte/icons/files';
  import GitCompareArrows from '@lucide/svelte/icons/git-compare-arrows';
  import HardDrive from '@lucide/svelte/icons/hard-drive';
  import Layers from '@lucide/svelte/icons/layers';
  import Link from '@lucide/svelte/icons/link';
  import ListTodo from '@lucide/svelte/icons/list-todo';
  import Plus from '@lucide/svelte/icons/plus';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import ArrowUp from '@lucide/svelte/icons/arrow-up';
  import { tick } from 'svelte';
  import { SvelteMap } from 'svelte/reactivity';
  import type { ModView } from '../../lib/api';
  import { Button, LibraryRow, RowAddon, RowParts, RowVariant, ROW_H, LIST_GAP } from '../../lib/components';
  import type { RowBadge } from '../../lib/components/LibraryRow.svelte';
  import { formatSize, t } from '../../i18n';
  import {
    badges,
    file,
    fileLabel,
    isLocal,
    modVersion,
    moved,
    parts as splitParts,
    partsKind,
    targetIndex,
    variantGroups,
    variantName,
    type BadgeKind,
    type Context,
  } from './model';
  import { dropIndex, layout, scrollFor, visible } from './vlist';

  interface Props {
    /** The mods shown (the tab and search applied), in priority order. */
    mods: ModView[];
    /** Every mod id of the library view, in its order: where a move lands. */
    order: string[];
    ctx: Context;
    profile: string;
    /** The selected rows (details or a conflict pair). */
    selected?: string[];
    /** Game not found: toggles are off, with why. */
    toggleDisabled?: boolean;
    /** Mods whose change is running (busy toggle). */
    busy?: string[];
    ontoggle: (mod: ModView, enabled: boolean) => void;
    /** `index` in the whole order (0 = wins over all). */
    onmove: (mod: ModView, index: number) => void;
    onopen: (mod: ModView) => void;
    onvariant: (mod: ModView, file: string) => void;
    onremovefile: (mod: ModView, file: string) => void;
  }

  const {
    mods,
    order,
    ctx,
    profile,
    selected = [],
    toggleDisabled = false,
    busy = [],
    ontoggle,
    onmove,
    onopen,
    onvariant,
    onremovefile,
  }: Props = $props();

  const OVERSCAN = 4;

  /** The shown order while a row is picked up with the keyboard (null otherwise). */
  let preview = $state<string[] | null>(null);
  let grabbed = $state<string | null>(null);
  let saved: string[] | null = null;
  let live = $state('');
  let expanded: Record<string, boolean> = $state({});
  /** Measured heights by mod: only an expanded block differs from `--row-h`. */
  const measured = new SvelteMap<string, number>();
  let viewport: HTMLElement | undefined = $state();
  let scrollTop = $state(0);
  let viewHeight = $state(0);
  /** Pointer drag: the row, where it started, and where the pointer is in the list. */
  let drag = $state<{ id: string; from: number; y: number; offset: number } | null>(null);

  const shown = $derived.by(() => {
    if (!preview) {
      return mods;
    }
    return preview.map((id) => mods.find((m) => m.id === id)).filter((m): m is ModView => m !== undefined);
  });
  const ids = $derived(shown.map((m) => m.id));
  const total = $derived(order.length);
  const heights: ReadonlyMap<number, number> = $derived(
    new Map(
      shown.flatMap((m, i) => {
        const h = expanded[m.id] ? measured.get(m.id) : undefined;
        return h === undefined ? [] : [[i, h] as [number, number]];
      }),
    ),
  );
  const at = $derived(layout(shown.length, heights, ROW_H, LIST_GAP));
  const range = $derived(visible(at, scrollTop, viewHeight, OVERSCAN));
  const target = $derived(drag ? dropIndex(at, heights, ROW_H, drag.from, drag.y) : null);

  /** The row whose grip keeps keyboard focus while rows move under it (DOM moves drop focus). */
  let focusId: string | null = null;

  $effect(() => {
    void shown;
    const id = focusId;
    if (id) {
      void tick().then(() => {
        viewport?.querySelector<HTMLElement>(`[data-id="${CSS.escape(id)}"] .dl-row__grip`)?.focus();
      });
    }
  });

  function keepFocus(id: string): void {
    focusId = id;
    // The library comes back from the core a moment later; keep the focus through that too.
    setTimeout(() => {
      if (focusId === id) {
        focusId = null;
      }
    }, 2000);
  }

  function position(id: string): number {
    return order.indexOf(id) + 1;
  }

  const ICONS: Record<BadgeKind, RowBadge['icon']> = {
    pending: ListTodo,
    damaged: TriangleAlert,
    update: CircleArrowUp,
    wins: GitCompareArrows,
    loses: GitCompareArrows,
    variants: Layers,
    local: HardDrive,
    copied: Files,
  };

  function badgeList(mod: ModView): RowBadge[] {
    return badges(mod, ctx).map((kind) => {
      const variant: RowBadge['variant'] =
        kind === 'update'
          ? 'info'
          : kind === 'wins' || kind === 'loses'
            ? 'warning'
            : kind === 'damaged'
              ? 'danger'
              : kind === 'local'
                ? 'outline'
                : 'neutral';
      const label =
        kind === 'variants'
          ? t('library-badge-variants', { count: variantGroups(mod)[0]?.files.length ?? 0 })
          : kind === 'copied'
            ? t('library-badge-copied', { size: formatSize(mod.in_game?.bytes ?? 0) })
            : t(`library-badge-${kind}`);
      return { icon: ICONS[kind], label, variant };
    });
  }

  function sub(mod: ModView): string[] {
    const out = [
      isLocal(mod) ? t('library-local') : (mod.author ?? t('library-unknown-author')),
      isLocal(mod) ? t('library-from-disk') : t('library-gamebanana'),
    ];
    const version = modVersion(mod);
    const variant = variantName(mod);
    const addOns = splitParts(mod).addOns.length;
    const tail = [version, variant, addOns > 0 ? t('library-addons-sub', { count: addOns }) : null].filter(
      (p): p is string => p !== null,
    );
    if (tail.length > 0) {
      out.push(tail.join(' · '));
    }
    return out;
  }

  function announce(key: string, mod: ModView, index: number): void {
    live = t(key, { name: mod.name, position: index + 1, total: shown.length });
  }

  function grab(mod: ModView): void {
    keepFocus(mod.id);
    saved = [...ids];
    preview = [...ids];
    grabbed = mod.id;
    announce('library-live-picked', mod, ids.indexOf(mod.id));
  }

  function step(mod: ModView, delta: -1 | 1): void {
    keepFocus(mod.id);
    const from = ids.indexOf(mod.id);
    const to = from + delta;
    if (from < 0 || to < 0 || to >= ids.length) {
      return;
    }
    const next = moved(ids, from, to);
    if (grabbed === mod.id) {
      preview = next;
      announce('library-live-moved-grabbed', mod, to);
    } else {
      announce('library-live-moved', mod, to);
      onmove(mod, targetIndex(order, next, mod.id));
    }
    keepInView(to);
  }

  function drop(mod: ModView): void {
    keepFocus(mod.id);
    const next = preview ?? ids;
    announce('library-live-dropped', mod, next.indexOf(mod.id));
    const changed = saved !== null && saved.join() !== next.join();
    grabbed = null;
    preview = null;
    saved = null;
    if (changed) {
      onmove(mod, targetIndex(order, next, mod.id));
    }
  }

  function cancel(mod: ModView): void {
    keepFocus(mod.id);
    const back = saved ?? ids;
    announce('library-live-back', mod, back.indexOf(mod.id));
    grabbed = null;
    preview = null;
    saved = null;
  }

  function keepInView(index: number): void {
    if (viewport) {
      viewport.scrollTop = scrollFor(at, heights, ROW_H, index, viewport.scrollTop, viewport.clientHeight);
    }
  }

  // ---- pointer drag on the grip

  function listY(event: PointerEvent): number {
    const box = viewport?.getBoundingClientRect();
    return event.clientY - (box?.top ?? 0) + (viewport?.scrollTop ?? 0);
  }

  function pointerDown(event: PointerEvent): void {
    const grip = (event.target as HTMLElement | null)?.closest('.dl-row__grip');
    const item = grip?.closest<HTMLElement>('[data-index]');
    if (!grip || !item || event.button !== 0 || grabbed) {
      return;
    }
    const from = Number(item.dataset.index);
    const mod = shown[from];
    if (!mod) {
      return;
    }
    event.preventDefault();
    (event.currentTarget as HTMLElement).setPointerCapture?.(event.pointerId);
    const y = listY(event);
    drag = { id: mod.id, from, y, offset: y - (at.tops[from] ?? 0) };
  }

  function pointerMove(event: PointerEvent): void {
    if (!drag) {
      return;
    }
    drag = { ...drag, y: listY(event) };
    const mod = shown[drag.from];
    const index = target ?? drag.from;
    const next = moved(ids, drag.from, index);
    const above = next[index + 1];
    const aboveName = shown.find((m) => m.id === above)?.name;
    if (mod) {
      live = aboveName
        ? t('library-live-drag', { name: mod.name, above: aboveName, position: index + 1, total: shown.length })
        : t('library-live-moved', { name: mod.name, position: index + 1, total: shown.length });
    }
  }

  function pointerUp(): void {
    if (!drag) {
      return;
    }
    const { from } = drag;
    const index = target ?? from;
    const mod = shown[from];
    drag = null;
    if (mod && index !== from) {
      const next = moved(ids, from, index);
      announce('library-live-dropped', mod, index);
      onmove(mod, targetIndex(order, next, mod.id));
    }
  }

  function dropLineTop(): number | null {
    if (!drag || target === null) {
      return null;
    }
    // The line sits above the row that will follow the dropped one.
    const others = ids.filter((_, i) => i !== drag?.from);
    const follower = others[target];
    const i = follower === undefined ? -1 : ids.indexOf(follower);
    return i >= 0 ? (at.tops[i] ?? 0) - LIST_GAP / 2 : at.total + LIST_GAP / 2;
  }

  function measure(node: HTMLElement, id: string) {
    const observer = new ResizeObserver(() => {
      const h = node.offsetHeight;
      if (h > 0 && measured.get(id) !== h) {
        measured.set(id, h);
      }
    });
    observer.observe(node);
    return { destroy: () => observer.disconnect() };
  }
</script>

<div class="list">
  <div class="dl-list__note note">
    <ArrowUp class="dl-icon" /><span>{t('library-higher-wins')}</span><span class="hint">{t('library-reorder-hint')}</span>
  </div>
  <div class={['dl-live', live && 'live']} aria-live="polite">{live}</div>
  <div
    class="viewport"
    bind:this={viewport}
    bind:clientHeight={viewHeight}
    onscroll={() => (scrollTop = viewport?.scrollTop ?? 0)}
    onpointerdown={pointerDown}
    onpointermove={pointerMove}
    onpointerup={pointerUp}
    onpointercancel={() => (drag = null)}
    role="list"
    aria-label={t('library-list-label', { name: profile })}
  >
    <div class="space" style:height={`${at.total}px`}>
      {#each shown.slice(range.start, range.end) as mod, offset (mod.id)}
        {@const index = range.start + offset}
        {@const kind = partsKind(mod)}
        {@const open = expanded[mod.id] === true && kind !== null}
        <div
          class="item"
          role="listitem"
          data-index={index}
          data-id={mod.id}
          style:top={`${at.tops[index] ?? 0}px`}
          style:transform={drag?.id === mod.id ? `translateY(${drag.y - drag.offset - (at.tops[index] ?? 0)}px)` : null}
          use:measure={mod.id}
        >
          <LibraryRow
            name={mod.name}
            sub={sub(mod)}
            enabled={mod.enabled}
            toggleLabel={t('library-toggle', { name: mod.name })}
            {toggleDisabled}
            toggleTitle={toggleDisabled ? t('library-toggle-missing') : undefined}
            gripLabel={t('library-grip', { name: mod.name, position: position(mod.id), total })}
            badges={badgeList(mod)}
            busy={busy.includes(mod.id)}
            error={mod.files.some((f) => f.damaged)}
            selected={selected.includes(mod.id)}
            grabbed={grabbed === mod.id}
            dragging={drag?.id === mod.id}
            expanded={open}
            ontoggle={(on) => ontoggle(mod, on)}
            ongrab={() => grab(mod)}
            onmove={(delta) => step(mod, delta)}
            ondrop={() => drop(mod)}
            oncancel={() => cancel(mod)}
            onopen={() => onopen(mod)}
          >
            {#snippet actions()}
              {#if kind}
                {@const label = t(`library-${open ? 'hide' : 'show'}-${kind}`)}
                <Button
                  variant="ghost"
                  size="sm"
                  iconOnly
                  icon={open ? ChevronUp : ChevronDown}
                  {label}
                  title={label}
                  aria-expanded={open ? 'true' : 'false'}
                  onclick={() => (expanded = { ...expanded, [mod.id]: !open })}
                />
              {/if}
              <Button
                variant="ghost"
                size="sm"
                iconOnly
                icon={Ellipsis}
                label={t('library-more', { name: mod.name })}
                onclick={() => onopen(mod)}
              />
            {/snippet}
            {#snippet parts()}
              {#if kind === 'variants'}
                {#each variantGroups(mod) as group, g (g)}
                  <RowParts label={t('library-variants')} meta={t('library-variants-downloaded', { count: group.files.length })} radio>
                    {#each group.files as id (id)}
                      <RowVariant
                        name={fileLabel(mod, id)}
                        size={formatSize(file(mod, id)?.len ?? 0)}
                        hint={group.active === id ? '' : t('library-select-to-switch')}
                        group={`${mod.id}-variants-${g}`}
                        checked={group.active === id}
                        activeLabel={t('library-active')}
                        onselect={() => onvariant(mod, id)}
                      />
                    {/each}
                  </RowParts>
                {/each}
              {:else if kind === 'addons'}
                {@const split = splitParts(mod)}
                <RowParts label={t('library-parts')}>
                  {#each split.main as id (id)}
                    <RowAddon
                      icon={Link}
                      name={t('library-main-files')}
                      file={`· ${file(mod, id)?.inner_path ?? ''} · ${t('library-from', { archive: file(mod, id)?.archive_name ?? '' })}`}
                      state={formatSize(file(mod, id)?.len ?? 0)}
                    />
                  {/each}
                  {#each split.addOns as [id] (id)}
                    <RowAddon
                      icon={Plus}
                      name={fileLabel(mod, id)}
                      file={`· ${file(mod, id)?.inner_path ?? ''} · ${t('library-from', { archive: file(mod, id)?.archive_name ?? '' })}`}
                      state={t('library-on-with-mod', { size: formatSize(file(mod, id)?.len ?? 0) })}
                      removeLabel={t('library-remove-addon')}
                      onremove={() => onremovefile(mod, id)}
                    />
                  {/each}
                </RowParts>
              {/if}
            {/snippet}
          </LibraryRow>
        </div>
      {/each}
      {#if drag}
        <div class="dl-row-placeholder ghost" style:top={`${at.tops[drag.from] ?? 0}px`} aria-hidden="true"></div>
      {/if}
      {#if dropLineTop() !== null}
        <div class="dl-drop-line line" style:top={`${dropLineTop()}px`} aria-hidden="true"></div>
      {/if}
    </div>
  </div>
</div>

<style>
  .list {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    flex: 1 1 auto;
    min-height: 0;
  }

  .note {
    flex-wrap: wrap;
  }

  .hint {
    margin-left: auto;
  }

  .dl-live:empty {
    display: none;
  }

  /* From the design's keyboard and drag boards. */
  .live {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    min-height: 28px;
    background: var(--accent-soft);
    border: 1px solid var(--accent-line);
    border-radius: var(--radius-md);
    color: var(--ink);
  }

  .viewport {
    flex: 1 1 auto;
    min-height: 0;
    overflow: auto;
    padding-bottom: var(--space-4);
  }

  .space {
    position: relative;
  }

  .item {
    position: absolute;
    left: 0;
    right: 0;
  }

  .ghost {
    position: absolute;
    left: 0;
    right: 0;
  }

  .line {
    position: absolute;
    left: 0;
    right: 0;
  }
</style>
