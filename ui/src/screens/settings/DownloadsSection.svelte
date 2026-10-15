<!-- Settings → Downloads: parallel downloads (44), speed limit, keeping archives (45), unpacking limits (127). The RAR
     method of the design is left out: decision 123 made the built-in readers the only method. -->
<script lang="ts">
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Select from '../../lib/components/Select.svelte';
  import Toggle from '../../lib/components/Toggle.svelte';
  import type { AppState } from '../../lib/app.svelte';
  import { formatNumber, t } from '../../i18n';
  import Group from './Group.svelte';
  import NumberField from './NumberField.svelte';
  import Row from './Row.svelte';
  import { CHOICES, DEFAULTS, RANGES, speedKib, speedMb, wholeNumber, withCurrent, type SettingsData, without } from './model';
  import { saveSettings } from './save';

  interface Props {
    app: AppState;
    data: SettingsData;
  }

  const { app, data }: Props = $props();

  type Limit = 'size' | 'ratio' | 'files';
  const LIMITS = {
    size: { range: RANGES.extractedGb, label: 'settings-limit-size' },
    ratio: { range: RANGES.factor, label: 'settings-limit-ratio' },
    files: { range: RANGES.vpkFiles, label: 'settings-limit-files' },
  } as const;
  /** What was typed and isn't valid yet; the saved value stays in use. */
  let typed = $state<Partial<Record<Limit, string>>>({});

  const parallelOptions = $derived(
    withCurrent(CHOICES.parallel, data.downloads.parallel).map((n) => ({ value: String(n), label: formatNumber(n) })),
  );
  const speedLabel = (mb: number | null) =>
    mb === null ? t('settings-speed-none') : t('settings-speed-value', { value: formatNumber(mb) });
  const speedOptions = $derived(
    withCurrent<number | null>(CHOICES.speedMb, speedMb(data.downloads.speed_limit_kib_per_s)).map((mb) => ({
      value: mb === null ? 'none' : String(mb),
      label: speedLabel(mb),
    })),
  );
  const saved = $derived({
    size: String(Math.round((data.downloads.max_extracted_mib / 1024) * 100) / 100),
    ratio: String(data.downloads.max_extracted_factor),
    files: String(data.downloads.max_vpk_files),
  });
  const invalid = $derived(
    (Object.keys(LIMITS) as Limit[]).find((k) => typed[k] !== undefined && !wholeNumber(typed[k] ?? '', LIMITS[k].range).ok),
  );
  const limitsChanged = $derived(
    data.downloads.max_extracted_mib !== DEFAULTS.extractedMib ||
      data.downloads.max_extracted_factor !== DEFAULTS.factor ||
      data.downloads.max_vpk_files !== DEFAULTS.vpkFiles,
  );

  function commit(limit: Limit, text: string): void {
    const checked = wholeNumber(text, LIMITS[limit].range);
    if (!checked.ok) {
      typed = { ...typed, [limit]: text };
      return;
    }
    typed = without(typed, limit);
    void saveSettings(app, (s) => {
      if (limit === 'size') {
        s.downloads.max_extracted_mib = checked.value * 1024;
      } else if (limit === 'ratio') {
        s.downloads.max_extracted_factor = checked.value;
      } else {
        s.downloads.max_vpk_files = checked.value;
      }
    });
  }

  function restoreLimits(): void {
    typed = {};
    void saveSettings(app, (s) => {
      s.downloads.max_extracted_mib = DEFAULTS.extractedMib;
      s.downloads.max_extracted_factor = DEFAULTS.factor;
      s.downloads.max_vpk_files = DEFAULTS.vpkFiles;
    });
  }
</script>

<Group title={t('settings-queue-group')}>
  <Row
    label={t('settings-parallel')}
    help={t('settings-parallel-help')}
    changed={data.downloads.parallel !== DEFAULTS.parallel}
    defaultText={formatNumber(DEFAULTS.parallel)}
    onrestore={() => void saveSettings(app, (s) => (s.downloads.parallel = DEFAULTS.parallel))}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-parallel')}
          options={parallelOptions}
          value={String(data.downloads.parallel)}
          onchange={(v) => void saveSettings(app, (s) => (s.downloads.parallel = Number(v)))}
        />
      </div>
    {/snippet}
  </Row>
  <Row
    label={t('settings-speed')}
    help={t('settings-speed-help')}
    changed={data.downloads.speed_limit_kib_per_s !== DEFAULTS.speed}
    defaultText={t('settings-speed-none')}
    onrestore={() => void saveSettings(app, (s) => (s.downloads.speed_limit_kib_per_s = null))}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-speed')}
          options={speedOptions}
          value={data.downloads.speed_limit_kib_per_s === null
            ? 'none'
            : String(speedMb(data.downloads.speed_limit_kib_per_s))}
          onchange={(v) =>
            void saveSettings(app, (s) => (s.downloads.speed_limit_kib_per_s = v === 'none' ? null : speedKib(Number(v))))}
        />
      </div>
    {/snippet}
  </Row>
  <Row
    label={t('settings-keep-archives')}
    help={t('settings-keep-archives-help')}
    changed={data.downloads.keep_archives !== DEFAULTS.keepArchives}
    defaultText={t('settings-off')}
    onrestore={() => void saveSettings(app, (s) => (s.downloads.keep_archives = DEFAULTS.keepArchives))}
  >
    {#snippet control()}
      <Toggle
        ariaLabel={t('settings-keep-archives')}
        checked={data.downloads.keep_archives}
        onchange={(on) => void saveSettings(app, (s) => (s.downloads.keep_archives = on))}
      />
    {/snippet}
  </Row>
</Group>

<Group title={t('settings-unpack-group')}>
  <Row
    label={t('settings-limits')}
    help={t('settings-limits-help')}
    changed={limitsChanged}
    defaultText={t('settings-limits-default')}
    onrestore={restoreLimits}
    labelId="st-limits"
  >
    {#snippet full()}
      <div class="st-nums" role="group" aria-labelledby="st-limits">
        <NumberField
          label={t('settings-limit-size')}
          value={typed.size ?? saved.size}
          unit={t('settings-unit-gb')}
          invalid={invalid === 'size'}
          oncommit={(text) => commit('size', text)}
        />
        <NumberField
          label={t('settings-limit-ratio')}
          value={typed.ratio ?? saved.ratio}
          unit={t('settings-unit-x')}
          invalid={invalid === 'ratio'}
          oncommit={(text) => commit('ratio', text)}
        />
        <NumberField
          label={t('settings-limit-files')}
          value={typed.files ?? saved.files}
          unit={t('settings-unit-files')}
          width="st-num--wide"
          invalid={invalid === 'files'}
          oncommit={(text) => commit('files', text)}
        />
      </div>
      {#if invalid}
        <span class="dl-field__error st-mt8" role="alert"
          ><CircleX class="dl-icon" /><span
            >{t('settings-number-error', {
              field: t(LIMITS[invalid].label),
              min: formatNumber(LIMITS[invalid].range.min),
              max: formatNumber(LIMITS[invalid].range.max),
            })}</span
          ></span
        >
      {/if}
    {/snippet}
  </Row>
</Group>
