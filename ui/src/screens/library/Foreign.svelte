<!-- Foreign files (decision 16, F9; board LibraryForeign): what is in the game's mod folder that deadloader didn't put
     there, what it seems to be, and per item Take under management (D11) / Leave as is / Open folder. Hidden when
     there is nothing. -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';
  import File from '@lucide/svelte/icons/file';
  import Folder from '@lucide/svelte/icons/folder';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import type { ForeignItem } from '../../lib/api';
  import { Button, Count } from '../../lib/components';
  import { formatDate, formatNumber, formatSize, t } from '../../i18n';
  import { heroName } from './model';

  interface Props {
    items: ForeignItem[];
    ontake: (item: ForeignItem) => void;
    onleave: (item: ForeignItem) => void;
    onfolder: (item: ForeignItem) => void;
  }

  const { items, ontake, onleave, onfolder }: Props = $props();

  let open = $state(true);

  function what(item: ForeignItem): string {
    switch (item.kind) {
      case 'folder':
        return t('library-foreign-folder', { count: item.files ?? 0 });
      case 'other_file':
        return item.origin === 'deadlock_mod_manager' ? t('library-foreign-dmm') : t('library-foreign-other');
      case 'replaced_ours':
        return t('library-foreign-replaced');
      case 'mod_file':
        if (!item.vpk) {
          return t('library-foreign-unreadable');
        }
        return item.vpk.hero
          ? t('library-foreign-vpk-hero', { count: item.vpk.entries, hero: heroName(item.vpk.hero) })
          : t('library-foreign-vpk', { count: item.vpk.entries });
    }
  }

  function meta(item: ForeignItem): string {
    const size = formatSize(item.bytes);
    return item.modified_unix_ms === null
      ? size
      : t('library-foreign-meta', { size, date: formatDate(item.modified_unix_ms) });
  }
</script>

<section class="foreign" aria-labelledby="foreign-title">
  <div class="head">
    <Button
      variant="ghost"
      size="sm"
      iconOnly
      icon={open ? ChevronDown : ChevronRight}
      label={open ? t('library-collapse') : t('library-expand')}
      aria-expanded={open ? 'true' : 'false'}
      aria-controls="foreign-list"
      onclick={() => (open = !open)}
    />
    <h2 id="foreign-title">{t('library-foreign-title')}</h2>
    <Count value={formatNumber(items.length)} muted />
    <span class="note"><Text value={t('library-foreign-note')} /></span>
  </div>
  {#if open}
    <div id="foreign-list" class="items">
      <!-- Mod files first: they are the ones to act on. -->
      {#each [...items].sort((a, b) => Number(b.takeable) - Number(a.takeable)) as item (item.name)}
        <div class="item">
          <span class="icon" aria-hidden="true">
            {#if item.kind === 'folder'}<Folder class="dl-icon" />{:else}<File class="dl-icon" />{/if}
          </span>
          <div class="text">
            <span class="dl-mono name">{item.kind === 'folder' ? `${item.name}\\` : item.name}</span>
            <span class="what"><Text value={what(item)} /> <span class="meta">· {meta(item)}</span></span>
          </div>
          <div class="actions">
            {#if item.takeable}
              <Button size="sm" label={t('library-take')} aria-haspopup="dialog" onclick={() => ontake(item)} />
            {/if}
            {#if item.left}
              <span class="left">{t('library-left')}</span>
            {:else}
              <Button variant="ghost" size="sm" label={t('library-leave')} onclick={() => onleave(item)} />
            {/if}
            <Button
              variant="ghost"
              size="sm"
              iconOnly
              icon={FolderOpen}
              label={t('library-foreign-open', { name: item.name })}
              title={t('library-open-folder')}
              onclick={() => onfolder(item)}
            />
          </div>
        </div>
      {/each}
    </div>
  {/if}
</section>

<style>
  /* From the design's foreign-files section (LibraryStates `foreign`). */
  .foreign {
    flex: none;
    max-height: 45%;
    overflow: auto;
    margin: 8px 0 12px;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 8px;
  }

  .head {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px 12px;
    padding: 8px 12px;
  }

  h2 {
    margin: 0;
    font: var(--type-title-sm);
  }

  .note {
    margin-left: auto;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .item {
    display: grid;
    grid-template-columns: 32px minmax(0, 1fr) auto;
    gap: 12px;
    align-items: center;
    padding: 10px 12px;
    border-top: 1px solid var(--line);
  }

  .icon {
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    border-radius: 6px;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    color: var(--ink-subtle);
  }

  .text {
    display: grid;
    gap: 2px;
    min-width: 0;
  }

  .name {
    color: var(--ink);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .what {
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .meta {
    color: var(--ink-subtle);
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    align-items: center;
    gap: 4px;
  }

  .left {
    padding: 0 8px;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }
</style>
