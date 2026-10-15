<!-- Settings → Cache: the thumbnail cache limit (58), what the cache holds now, "Clear cache". -->
<script lang="ts">
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import { onMount } from 'svelte';
  import Button from '../../lib/components/Button.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import Select from '../../lib/components/Select.svelte';
  import { maintenance, type CacheCleared, type CacheUsage } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { formatNumber, formatSize, t } from '../../i18n';
  import Group from './Group.svelte';
  import Row from './Row.svelte';
  import { CHOICES, DEFAULTS, withCurrent, type SettingsData } from './model';
  import { saveSettings } from './save';

  interface Props {
    app: AppState;
    data: SettingsData;
    folder: string;
  }

  const { app, data, folder }: Props = $props();

  let usage = $state<CacheUsage | null>(null);
  let cleared = $state<CacheCleared | null>(null);
  let clearing = $state(false);

  onMount(() => {
    maintenance
      .cacheUsage()
      .then((u) => (usage = u))
      .catch((error: unknown) => app.fail(error));
  });

  const MIB = 1024 * 1024;
  const limitLabel = (mib: number) =>
    mib >= 1000 && mib % 1000 === 0
      ? t('settings-size-gb', { value: formatNumber(mib / 1000) })
      : t('settings-size-mb', { value: formatNumber(mib) });
  const limitOptions = $derived(
    withCurrent(CHOICES.thumbnailMib, data.cache.thumbnail_limit_mib).map((m) => ({ value: String(m), label: limitLabel(m) })),
  );
  const fraction = $derived(usage ? usage.bytes / (data.cache.thumbnail_limit_mib * MIB) : 0);
  const empty = $derived(usage !== null && usage.bytes === 0);

  async function clear(): Promise<void> {
    clearing = true;
    try {
      cleared = await maintenance.clearCache();
      usage = await maintenance.cacheUsage();
    } catch (error) {
      app.fail(error);
    } finally {
      clearing = false;
    }
  }
</script>

<Group title={t('settings-thumbs-group')}>
  <Row
    label={t('settings-cache-limit')}
    help={t('settings-cache-limit-help')}
    changed={data.cache.thumbnail_limit_mib !== DEFAULTS.thumbnailMib}
    defaultText={limitLabel(DEFAULTS.thumbnailMib)}
    onrestore={() => void saveSettings(app, (s) => (s.cache.thumbnail_limit_mib = DEFAULTS.thumbnailMib))}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-cache-limit')}
          options={limitOptions}
          value={String(data.cache.thumbnail_limit_mib)}
          onchange={(v) => void saveSettings(app, (s) => (s.cache.thumbnail_limit_mib = Number(v)))}
        />
      </div>
    {/snippet}
  </Row>
  <Row
    label={t('settings-cache-now')}
    help={usage === null
      ? '…'
      : empty
        ? t('settings-cache-empty')
        : t('settings-cache-breakdown', {
            thumbnails: formatNumber(usage.thumbnails + usage.images),
            pages: formatNumber(usage.pages),
          })}
  >
    {#snippet control()}
      <Button icon={Trash2} label={t('settings-cache-clear')} busy={clearing} disabled={empty} onclick={() => void clear()} />
    {/snippet}
    {#snippet full()}
      {@const pct = Math.min(100, Math.round(fraction * 100))}
      <div
        class="dl-progress"
        role="progressbar"
        aria-label={t('settings-cache-now')}
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={pct}
      >
        <div class="dl-progress__meta">
          <strong
            >{t('settings-cache-used', {
              used: formatSize(usage?.bytes ?? 0),
              limit: limitLabel(data.cache.thumbnail_limit_mib),
            })}</strong
          ><span>{t('settings-percent', { value: formatNumber(pct) })}</span>
        </div>
        <div class="dl-progress__track"><div class="dl-progress__fill" style:--value={Math.min(1, fraction)}></div></div>
      </div>
      {#if cleared}
        <p class="st-stat st-stat--ok st-mt8">
          <CircleCheck class="dl-icon" />{t('settings-cache-cleared', { size: formatSize(cleared.bytes) })}
          {#if cleared.failed > 0}{t('settings-cache-cleared-partly', { failed: cleared.failed })}{/if}
        </p>
      {/if}
      {#if folder}
        <div class="st-meta st-pathline st-mt8">{t('settings-folder')} <PathTooltip path={`${folder}\\cache`} /></div>
      {/if}
    {/snippet}
  </Row>
</Group>
