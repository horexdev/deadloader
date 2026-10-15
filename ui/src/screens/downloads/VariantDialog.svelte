<!-- D4, choose a variant (board DownloadsD4): the archive held files that change the same game files, so one of each
     group is turned on. Names come from the folders inside the archive; GameBanana's file description is shown when
     the mod page is at hand. -->
<script lang="ts">
  import CircleDot from '@lucide/svelte/icons/circle-dot';
  import Download from '@lucide/svelte/icons/download';
  import { onMount } from 'svelte';
  import Button from '../../lib/components/Button.svelte';
  import Dialog from '../../lib/components/Dialog.svelte';
  import { catalog, type FileView, type ModView } from '../../lib/api';
  import type { PendingChoice } from '../../lib/app.svelte';
  import { formatSize, t } from '../../i18n';
  import { fileName, variantName } from './variants';

  interface Props {
    choice: PendingChoice;
    /** The library mod the download went into. */
    mod: ModView | undefined;
    profile: string;
    /** The picked file of every group, and the name of the first one (for the toast). */
    onpick: (files: string[], name: string) => void;
    oncancel: () => void;
  }

  const { choice, mod, profile, onpick, oncancel }: Props = $props();

  const uid = $props.id();
  const groups = $derived(
    choice.choices.map((group) =>
      group.map((id) => mod?.files.find((f) => f.id === id) ?? ({ id, inner_path: id, len: 0 } as FileView)),
    ),
  );
  const count = $derived(choice.choices.reduce((n, g) => n + g.length, 0));
  // svelte-ignore state_referenced_locally
  let picked: string[] = $state(choice.choices.map((group) => group[0] ?? ''));
  let description: string | null = $state(null);

  onMount(() => {
    const source = choice.source;
    if (!source) {
      return;
    }
    catalog
      .details(source.section, source.modId, null)
      .then((page) => {
        const file = [...page.files, ...page.archived_files].find((f) => f.id === source.fileId);
        description = file?.description?.trim() || null;
      })
      .catch(() => undefined);
  });

  function install() {
    const first = groups[0]?.find((f) => f.id === picked[0]);
    onpick(picked, first ? variantName(first.inner_path) : '');
  }
</script>

<Dialog title={t('downloads-d4-title')} size="md" icon={CircleDot} closeLabel={t('downloads-close-esc')} {oncancel}>
  <p>
    <strong class="strong">{choice.name}</strong> · <span class="dl-mono muted">{choice.file}</span><br />{t(
      'downloads-d4-lead',
      { count },
    )}
  </p>
  {#if description}
    <figure class="description">
      <figcaption>{t('downloads-d4-description')}</figcaption>
      <blockquote>{description}</blockquote>
    </figure>
  {/if}
  {#each groups as group, g (g)}
    <div class="list" role="radiogroup" aria-labelledby={`${uid}-list-${g}`}>
      <div class="dl-row-block__head" id={`${uid}-list-${g}`}>
        <span>{t('downloads-d4-list')}</span><span>{t('downloads-d4-list-meta', { count: group.length })}</span>
      </div>
      {#each group as file (file.id)}
        <label class="dl-variant option">
          <input
            class="dl-variant__radio"
            type="radio"
            name={`${uid}-group-${g}`}
            value={file.id}
            checked={picked[g] === file.id}
            onchange={() => (picked[g] = file.id)}
          /><span class="dl-variant__dot"></span><span class="names"
            ><span class="name">{variantName(file.inner_path)}</span><span class="dl-mono vpk" title={file.inner_path}>{fileName(file.inner_path)}</span></span
          ><span class="dl-variant__size">{formatSize(file.len)}</span><span></span>
        </label>
      {/each}
    </div>
  {/each}
  <p class="hint">{t('downloads-d4-hint', { count, profile })}</p>
  {#snippet footer()}
    <Button label={t('downloads-cancel')} onclick={oncancel} />
    <Button variant="primary" icon={Download} label={t('downloads-d4-install')} data-autofocus="" onclick={install} />
  {/snippet}
</Dialog>

<style>
  /* From DownloadsScreen.dc.html (D4). */
  .strong {
    font-weight: 600;
    color: var(--ink);
  }

  .muted {
    color: var(--ink-muted);
  }

  .description {
    display: grid;
    gap: 4px;
    margin: 0;
    padding: 8px 12px;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: 6px;
  }

  .description figcaption {
    font: var(--type-caption-strong);
    color: var(--ink-subtle);
  }

  .description blockquote {
    margin: 0;
    font: var(--type-body-sm);
    color: var(--ink);
  }

  .list {
    display: grid;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 6px;
  }

  .option {
    min-height: 42px;
  }

  .names {
    display: grid;
    min-width: 0;
  }

  .name {
    font: var(--type-body-strong);
    color: var(--ink);
  }

  .vpk {
    font: var(--type-mono-sm);
    color: var(--ink-subtle);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .hint {
    font: var(--type-body-sm);
  }
</style>
