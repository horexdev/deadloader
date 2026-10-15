<!-- S9 "Show the file list": each of our files that applying adds to, removes from or renumbers in the game's addons
     folder, grouped like the summary line, with its pak slot before and after. Up to three times 99 rows, so only the
     rows in view are drawn (decision 120). Not drawn in the design (open point): built from the summary box's style. -->
<script lang="ts">
  import { tooltip } from '../../lib/components/tooltip';
  import type { FolderFile, ModView } from '../../lib/api';
  import { t } from '../../i18n';
  import { fileLabel } from './model';
  import { layout, visible } from './vlist';

  interface Props {
    files: FolderFile[];
    mods: ModView[];
  }

  const { files, mods }: Props = $props();

  const KINDS = ['added', 'removed', 'renumbered'] as const;
  const LABEL_H = 28;
  const ROW_H = 40;
  const OVERSCAN = 6;

  type Item = { kind: 'label'; label: string } | { kind: 'file'; file: FolderFile };

  const items = $derived.by(() => {
    const out: Item[] = [];
    for (const kind of KINDS) {
      const group = files.filter((f) => f.kind === kind);
      if (group.length > 0) {
        out.push({ kind: 'label', label: t(`library-files-${kind}`, { count: group.length }) });
        out.push(...group.map((file): Item => ({ kind: 'file', file })));
      }
    }
    return out;
  });
  const heights = $derived(new Map(items.flatMap((item, i) => (item.kind === 'label' ? [[i, LABEL_H] as const] : []))));
  const at = $derived(layout(items.length, heights, ROW_H, 0));

  let viewport: HTMLElement | undefined = $state();
  let scrollTop = $state(0);
  let viewHeight = $state(0);
  const range = $derived(visible(at, scrollTop, viewHeight, OVERSCAN));

  function pak(slot: number): string {
    return `pak${String(slot).padStart(2, '0')}_dir.vpk`;
  }

  function slots(file: FolderFile): string {
    if (file.from_slot !== null && file.to_slot !== null) {
      return `${pak(file.from_slot)} → ${pak(file.to_slot)}`;
    }
    const slot = file.to_slot ?? file.from_slot;
    return slot === null ? '' : pak(slot);
  }

  function owner(file: FolderFile): string {
    const mod = mods.find((m) => m.id === file.mod_id);
    if (!mod) {
      return file.name ?? file.file;
    }
    return `${mod.name} · ${fileLabel(mod, file.file)}`;
  }
</script>

<div
  class="viewport"
  bind:this={viewport}
  bind:clientHeight={viewHeight}
  onscroll={() => (scrollTop = viewport?.scrollTop ?? 0)}
  role="list"
  aria-label={t('library-file-list')}
>
  <div class="space" style:height={`${at.total}px`}>
    {#each items.slice(range.start, range.end) as item, offset (range.start + offset)}
      {@const index = range.start + offset}
      {#if item.kind === 'label'}
        <span class="label" role="presentation" style:top={`${at.tops[index] ?? 0}px`}>{item.label}</span>
      {:else}
        <div class="file" role="listitem" style:top={`${at.tops[index] ?? 0}px`}>
          <span class="dl-mono slots">{slots(item.file)}</span>
          <span class="owner" {@attach tooltip({ text: owner(item.file) ?? '' })}>{owner(item.file)}</span>
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  .viewport {
    max-height: 240px;
    overflow: auto;
    border-top: 1px solid var(--line);
  }

  .space {
    position: relative;
  }

  .label,
  .file {
    position: absolute;
    left: 0;
    right: 0;
  }

  .label {
    display: flex;
    align-items: flex-end;
    height: 28px;
    padding-bottom: 4px;
    font: var(--type-caption-strong);
    color: var(--ink-subtle);
  }

  .file {
    display: grid;
    align-content: center;
    height: 40px;
    min-width: 0;
  }

  .slots {
    color: var(--ink);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .owner {
    font: var(--type-caption);
    color: var(--ink-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
