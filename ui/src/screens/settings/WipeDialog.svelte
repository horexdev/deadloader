<!-- D8 Delete all deadloader data (F11): what goes, with sizes; roll back first (recommended); typing the word unlocks
     the button. The app closes and its data folder is deleted when it has exited. -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import Check from '@lucide/svelte/icons/check';
  import Minus from '@lucide/svelte/icons/minus';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import { onMount } from 'svelte';
  import Button from '../../lib/components/Button.svelte';
  import Dialog from '../../lib/components/Dialog.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import { maintenance, type WipeSummary } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { formatDate, formatList, formatSize, t } from '../../i18n';

  interface Props {
    app: AppState;
    oncancel: () => void;
  }

  const { app, oncancel }: Props = $props();

  let summary = $state<WipeSummary | null>(null);
  let folder = $state('');
  let rollback = $state(true);
  let typed = $state('');
  let deleting = $state(false);

  onMount(() => {
    maintenance
      .wipeSummary()
      .then((s) => (summary = s))
      .catch((error: unknown) => app.fail(error));
    maintenance
      .about()
      .then((a) => (folder = a.data_folder))
      .catch(() => undefined);
  });

  const word = $derived(t('settings-d8-word'));
  const matches = $derived(typed.trim().toLocaleLowerCase() === word.toLocaleLowerCase());

  const rows = $derived.by(() => {
    if (!summary) {
      return [];
    }
    const s = summary;
    return [
      {
        what: t('settings-d8-library'),
        has: t('settings-d8-library-has', { mods: s.library.mods, files: s.library.files }),
        bytes: s.library.bytes,
      },
      { what: t('settings-d8-profiles'), has: formatList(s.profiles.names), bytes: s.profiles.bytes },
      {
        what: t('settings-d8-logs'),
        has: s.logs.oldest
          ? t('settings-d8-logs-has', { files: s.logs.files, date: formatDate(new Date(`${s.logs.oldest}T00:00:00`)) })
          : t('settings-d8-logs-none'),
        bytes: s.logs.bytes,
      },
      { what: t('settings-d8-cache'), has: t('settings-d8-cache-has'), bytes: s.cache_bytes },
      { what: t('settings-d8-settings'), has: t('settings-d8-settings-has', { backups: s.settings.backups }), bytes: s.settings.bytes },
      { what: t('settings-d8-other'), has: t('settings-d8-other-has'), bytes: s.other_bytes },
    ];
  });

  async function confirm(): Promise<void> {
    if (!matches || deleting) {
      return;
    }
    deleting = true;
    try {
      // The window closes when the core has marked the folder; nothing else happens here.
      await maintenance.deleteAllData(rollback);
    } catch (error) {
      deleting = false;
      app.fail(error);
    }
  }
</script>

<Dialog title={t('settings-d8-title')} kind="danger" size="lg" icon={Trash2} closeOnEscape={!deleting} {oncancel}>
  <div class="st-meta st-pathline st-pathline--wrap">
    <Text value={t('settings-d8-from')} />
    {#if folder}<PathTooltip path={folder} />{/if}
    <span>{t('settings-d8-from-2')}</span>
  </div>
  <div class="st-table" role="table" aria-label={t('settings-d8-what')}>
    <div class="st-tr st-tr--head" role="row">
      <span role="columnheader">{t('settings-d8-what')}</span>
      <span role="columnheader">{t('settings-d8-contains')}</span>
      <span role="columnheader" class="st-num-cell">{t('settings-d8-size')}</span>
    </div>
    {#each rows as row (row.what)}
      <div class="st-tr" role="row">
        <span role="cell" class="st-mono-ink"><Text value={row.what} /></span>
        <span role="cell">{row.has}</span>
        <span role="cell" class="st-num-cell">{formatSize(row.bytes)}</span>
      </div>
    {/each}
    <div class="st-tr st-tr--foot" role="row">
      <span role="cell" class="st-mono-ink">{t('settings-d8-total')}</span>
      <span role="cell"></span>
      <span role="cell" class="st-num-cell st-mono-ink">{summary ? formatSize(summary.total_bytes) : '…'}</span>
    </div>
  </div>
  <p class="st-meta">{t('settings-d8-exe')}</p>
  <label class="dl-check st-option">
    <input class="dl-check__input" type="checkbox" bind:checked={rollback} disabled={deleting} />
    <span class="dl-check__box"><Check class="dl-icon dl-check__tick" /><Minus class="dl-icon dl-check__dash" /></span>
    <span class="st-step">
      <span class="st-mono-ink"><Text value={t('settings-d8-rollback')} /></span>
      <span class="st-meta"><Text value={rollback ? t('settings-d8-rollback-why') : t('settings-d8-rollback-off')} /></span>
    </span>
  </label>
  <div class="dl-field">
    <label class="dl-field__label" for="d8-word"
      >{t('settings-d8-type-1')} <span class="dl-mono st-d8-word">{word}</span> {t('settings-d8-type-2')}</label
    >
    <span class="dl-input dl-input--mono">
      <input id="d8-word" autocomplete="off" spellcheck="false" bind:value={typed} disabled={deleting} aria-describedby="d8-hint" />
    </span>
    <span class="dl-field__hint" id="d8-hint">{matches ? t('settings-d8-hint-ok') : t('settings-d8-hint')}</span>
  </div>
  {#snippet footer()}
    <Button label={t('settings-cancel')} disabled={deleting} data-autofocus onclick={oncancel} />
    <Button
      variant="danger"
      icon={Trash2}
      busy={deleting}
      disabled={!matches || deleting}
      title={matches ? undefined : t('settings-d8-type-first', { word })}
      label={deleting
        ? t('settings-d8-deleting')
        : rollback
          ? t('settings-d8-confirm-rollback')
          : t('settings-d8-confirm')}
      onclick={() => void confirm()}
    />
  {/snippet}
</Dialog>
