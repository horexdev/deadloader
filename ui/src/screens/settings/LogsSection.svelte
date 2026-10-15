<!-- Settings → Logs: retention and size (51), hiding the Windows user name (94), the logs folder. -->
<script lang="ts">
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import { onMount } from 'svelte';
  import Button from '../../lib/components/Button.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import Select from '../../lib/components/Select.svelte';
  import Toggle from '../../lib/components/Toggle.svelte';
  import { maintenance, type WipeSummary } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { formatDate, formatNumber, formatSize, t } from '../../i18n';
  import Group from './Group.svelte';
  import Row from './Row.svelte';
  import { CHOICES, DEFAULTS, maskUser, withCurrent, type SettingsData } from './model';
  import { saveSettings } from './save';

  interface Props {
    app: AppState;
    data: SettingsData;
    folder: string;
  }

  const { app, data, folder }: Props = $props();

  let logs = $state<WipeSummary['logs'] | null>(null);
  onMount(() => {
    maintenance
      .wipeSummary()
      .then((s) => (logs = s.logs))
      .catch(() => undefined);
  });

  const EXAMPLE = String.raw`C:\Users\Alex\Downloads\haze_v2.zip`;
  const daysLabel = (d: number) => (d === 365 ? t('settings-year') : t('settings-days', { count: d }));
  const dayOptions = $derived(
    withCurrent(CHOICES.logDays, data.logs.retention_days).map((d) => ({ value: String(d), label: daysLabel(d) })),
  );
  const sizeLabel = (m: number) => t('settings-size-mb', { value: formatNumber(m) });
  const sizeOptions = $derived(
    withCurrent(CHOICES.logMib, data.logs.size_limit_mib).map((m) => ({ value: String(m), label: sizeLabel(m) })),
  );
  const logsNow = $derived(
    logs === null
      ? ''
      : logs.oldest
        ? t('settings-logs-now', {
            size: formatSize(logs.bytes),
            files: logs.files,
            date: formatDate(new Date(`${logs.oldest}T00:00:00`)),
          })
        : t('settings-d8-logs-none'),
  );
</script>

<Group title={t('settings-keep-group')}>
  <Row
    label={t('settings-log-days')}
    help={t('settings-log-days-help')}
    changed={data.logs.retention_days !== DEFAULTS.logDays}
    defaultText={daysLabel(DEFAULTS.logDays)}
    onrestore={() => void saveSettings(app, (s) => (s.logs.retention_days = DEFAULTS.logDays))}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-log-days')}
          options={dayOptions}
          value={String(data.logs.retention_days)}
          onchange={(v) => void saveSettings(app, (s) => (s.logs.retention_days = Number(v)))}
        />
      </div>
    {/snippet}
  </Row>
  <Row
    label={t('settings-log-size')}
    help={t('settings-log-size-help')}
    changed={data.logs.size_limit_mib !== DEFAULTS.logMib}
    defaultText={sizeLabel(DEFAULTS.logMib)}
    onrestore={() => void saveSettings(app, (s) => (s.logs.size_limit_mib = DEFAULTS.logMib))}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-log-size')}
          options={sizeOptions}
          value={String(data.logs.size_limit_mib)}
          onchange={(v) => void saveSettings(app, (s) => (s.logs.size_limit_mib = Number(v)))}
        />
      </div>
    {/snippet}
  </Row>
</Group>

<Group title={t('settings-privacy-group')}>
  <Row
    label={t('settings-mask')}
    help={t('settings-mask-help', { masked: String.raw`C:\Users\…\ ` })}
    changed={data.logs.mask_user_path !== DEFAULTS.mask}
    defaultText={t('settings-on')}
    onrestore={() => void saveSettings(app, (s) => (s.logs.mask_user_path = DEFAULTS.mask))}
  >
    {#snippet control()}
      <Toggle
        ariaLabel={t('settings-mask')}
        checked={data.logs.mask_user_path}
        onchange={(on) => void saveSettings(app, (s) => (s.logs.mask_user_path = on))}
      />
    {/snippet}
    {#snippet full()}
      <div class="st-example">
        <span class="st-meta">{t('settings-example')}</span>
        <span class="dl-mono st-mono-ink">{data.logs.mask_user_path ? maskUser(EXAMPLE) : EXAMPLE}</span>
      </div>
    {/snippet}
  </Row>
</Group>

<Group title={t('settings-logs-folder-group')}>
  <Row label={t('settings-logs-folder')} help={logsNow}>
    {#snippet control()}
      <Button
        icon={FolderOpen}
        label={t('settings-open-logs')}
        onclick={() => maintenance.openFolder({ place: 'logs' }).catch((error: unknown) => app.fail(error))}
      />
    {/snippet}
    {#snippet full()}
      {#if folder}<div class="st-meta st-pathline"><PathTooltip path={`${folder}\\logs`} /></div>{/if}
    {/snippet}
  </Row>
</Group>
