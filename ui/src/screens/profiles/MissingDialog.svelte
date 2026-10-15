<!-- D9, import a profile with missing mods (boards ProfilesD9, ProfilesDeD9): what the library lacks, which of it
     GameBanana has; download those (catalog on) or import without them. Either way the profile is selected, not
     applied (F14). -->
<script lang="ts">
  import { tooltip } from '../../lib/components/tooltip';
  import Download from '@lucide/svelte/icons/download';
  import HardDrive from '@lucide/svelte/icons/hard-drive';
  import Package from '@lucide/svelte/icons/package';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import Unplug from '@lucide/svelte/icons/unplug';
  import Button from '../../lib/components/Button.svelte';
  import Dialog from '../../lib/components/Dialog.svelte';
  import type { ProfileImported } from '../../lib/api';
  import { t } from '../../i18n';
  import { downloadsFor } from './profiles';

  interface Props {
    imported: ProfileImported;
    catalogOn: boolean;
    ondownload: () => void;
    onwithout: () => void;
    oncancel: () => void;
  }

  const { imported, catalogOn, ondownload, onwithout, oncancel }: Props = $props();

  const count = $derived(imported.missing.length);
  const downloadable = $derived(downloadsFor(imported.missing));
  const fromGameBanana = $derived(imported.missing.filter((m) => m.source.kind === 'gamebanana').length);
  const local = $derived(count - fromGameBanana);
  const summary = $derived(
    [
      fromGameBanana > 0 ? t('profiles-d9-from-gamebanana', { count: fromGameBanana }) : '',
      local > 0 ? t('profiles-d9-local-count', { count: local }) : '',
    ]
      .filter(Boolean)
      .join(' · '),
  );
  const canDownload = $derived(catalogOn && downloadable.length > 0);
  const versionOf = (m: ProfileImported['missing'][number]) => m.files.find((f) => f.version)?.version ?? null;
</script>

<Dialog
  title={t('profiles-d9-title', { count, name: imported.name })}
  size="md"
  icon={TriangleAlert}
  closeLabel={t('profiles-close-esc')}
  {oncancel}
>
  <p>
    {#if imported.file}{t('profiles-d9-file')} <span class="dl-mono file">{imported.file}</span>{/if}
    {t('profiles-d9-lead', { listed: imported.listed, present: Math.max(0, imported.listed - count), count })}
  </p>
  <div class="pf-opts" role="list" aria-label={t('profiles-d9-list')}>
    <div class="dl-row-block__head head"><span>{t('profiles-d9-list')}</span><span class="summary">{summary}</span></div>
    {#each imported.missing as m, i (i)}
      {@const gamebanana = m.source.kind === 'gamebanana'}
      {@const version = versionOf(m)}
      <div class="pf-miss" role="listitem">
        <span class="pf-miss__local" aria-hidden="true">
          {#if gamebanana}<Package class="dl-icon" />{:else}<HardDrive class="dl-icon" />{/if}
        </span>
        <div class="text">
          <span class={['pf-miss__name', !gamebanana && 'dl-mono local-name']} {@attach tooltip({ text: m.name ?? '' })}>{m.name}</span>
          <span class="pf-miss__sub">
            {#if gamebanana}
              <span>{[t('profiles-gamebanana'), version].filter(Boolean).join(' · ')}</span>
              {#if catalogOn && downloadable.some((d) => d.name === m.name)}
                <span class="pf-state can"><Download class="dl-icon" />{t('profiles-can-download')}</span>
              {/if}
            {:else}
              <span>{t('profiles-local-mod')}</span>
              <span class="pf-state cant"><TriangleAlert class="dl-icon" />{t('profiles-cant-download')}</span>
            {/if}
          </span>
        </div>
        {#if !gamebanana}<span></span><span class="why">{t('profiles-local-why')}</span>{/if}
      </div>
    {/each}
  </div>
  {#if !catalogOn}
    <p class="pf-note local-mode" role="note"><Unplug class="dl-icon" /><span>{t('profiles-d9-local-mode')}</span></p>
  {/if}
  <p>{t('profiles-d9-after', { name: imported.name })}</p>
  {#snippet footer()}
    <Button label={t('profiles-cancel')} onclick={oncancel} />
    <Button
      variant={canDownload ? 'secondary' : 'primary'}
      label={t('profiles-import-without')}
      data-autofocus={canDownload ? undefined : ''}
      onclick={onwithout}
    />
    {#if canDownload}
      <Button
        variant="primary"
        icon={Download}
        label={t('profiles-download-missing', { count: downloadable.length })}
        data-autofocus=""
        onclick={ondownload}
      />
    {/if}
  {/snippet}
</Dialog>

<style>
  /* From ProfilesScreen.dc.html (D9). */
  .file {
    color: var(--ink);
  }

  .head {
    justify-content: space-between;
    border-bottom: 1px solid var(--line);
  }

  .summary {
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .text {
    display: grid;
    gap: 2px;
    min-width: 0;
  }

  .local-name {
    font: var(--type-mono);
    font-weight: 600;
  }

  .can {
    color: var(--info);
  }

  .cant {
    color: var(--warning);
  }

  .why {
    font: var(--type-body-sm);
    color: var(--ink-subtle);
  }

  .local-mode {
    color: var(--ink);
  }
</style>
