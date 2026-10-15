<!-- Settings → Network: parallel API requests, timeouts, retries, proxy (182). In local mode a banner says these apply
     once the catalog is on. -->
<script lang="ts">
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Unplug from '@lucide/svelte/icons/unplug';
  import Banner from '../../lib/components/Banner.svelte';
  import SegmentedControl from '../../lib/components/SegmentedControl.svelte';
  import Select from '../../lib/components/Select.svelte';
  import type { AppState } from '../../lib/app.svelte';
  import { formatNumber, t } from '../../i18n';
  import Group from './Group.svelte';
  import NumberField from './NumberField.svelte';
  import Row from './Row.svelte';
  import { CHOICES, DEFAULTS, RANGES, proxyAddress, proxyParts, wholeNumber, withCurrent, type SettingsData, without } from './model';
  import { saveSettings } from './save';

  interface Props {
    app: AppState;
    data: SettingsData;
  }

  const { app, data }: Props = $props();

  type Timeout = 'connect' | 'read';
  const TIMEOUTS = {
    connect: { range: RANGES.connect, label: 'settings-timeout-connect' },
    read: { range: RANGES.read, label: 'settings-timeout-read' },
  } as const;
  let typed = $state<Partial<Record<Timeout, string>>>({});
  /** Manual proxy fields while they don't make an address yet. */
  let proxyDraft: { host: string; port: string } | null = $state(null);
  let proxyError = $state(false);

  const apiOptions = $derived(
    withCurrent(CHOICES.apiParallel, data.network.parallel_api_requests).map((n) => ({
      value: String(n),
      label: formatNumber(n),
    })),
  );
  const retryOptions = $derived(
    withCurrent(CHOICES.retries, data.network.retries).map((n) => ({ value: String(n), label: formatNumber(n) })),
  );
  const proxyOptions = $derived([
    { value: 'none', label: t('settings-proxy-none') },
    { value: 'system', label: t('settings-proxy-system') },
    { value: 'manual', label: t('settings-proxy-manual') },
  ]);
  const proxyMode = $derived(proxyDraft ? 'manual' : data.network.proxy.mode);
  const manual = $derived(
    proxyDraft ?? (data.network.proxy.mode === 'manual' ? proxyParts(data.network.proxy.address) : { host: '', port: '8080' }),
  );
  const invalid = $derived(
    (Object.keys(TIMEOUTS) as Timeout[]).find(
      (k) => typed[k] !== undefined && !wholeNumber(typed[k] ?? '', TIMEOUTS[k].range).ok,
    ),
  );

  function commitTimeout(which: Timeout, text: string): void {
    const checked = wholeNumber(text, TIMEOUTS[which].range);
    if (!checked.ok) {
      typed = { ...typed, [which]: text };
      return;
    }
    typed = without(typed, which);
    void saveSettings(app, (s) => {
      if (which === 'connect') {
        s.network.connect_timeout_s = checked.value;
      } else {
        s.network.read_timeout_s = checked.value;
      }
    });
  }

  async function saveManual(host: string, port: string): Promise<void> {
    const portNumber = wholeNumber(port, RANGES.proxyPort);
    if (host.trim() === '' || !portNumber.ok) {
      proxyDraft = { host, port };
      return;
    }
    const ok = await saveSettings(app, (s) => {
      s.network.proxy = { mode: 'manual', address: proxyAddress(host, portNumber.value) };
    });
    proxyError = !ok;
    proxyDraft = ok ? null : { host, port };
  }

  function chooseProxy(mode: string): void {
    proxyError = false;
    if (mode === 'manual') {
      proxyDraft = { ...manual };
      if (manual.host.trim() !== '') {
        void saveManual(manual.host, manual.port);
      }
      return;
    }
    proxyDraft = null;
    void saveSettings(app, (s) => (s.network.proxy = { mode: mode as 'none' | 'system' }));
  }
</script>

{#if !data.catalog.enabled}
  <Banner kind="neutral" icon={Unplug} title={t('settings-network-local')} text={t('settings-network-local-text')} />
{/if}

<Group title={t('settings-requests-group')}>
  <Row
    label={t('settings-api-parallel')}
    help={t('settings-api-parallel-help')}
    changed={data.network.parallel_api_requests !== DEFAULTS.apiParallel}
    defaultText={formatNumber(DEFAULTS.apiParallel)}
    onrestore={() => void saveSettings(app, (s) => (s.network.parallel_api_requests = DEFAULTS.apiParallel))}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-api-parallel')}
          options={apiOptions}
          value={String(data.network.parallel_api_requests)}
          onchange={(v) => void saveSettings(app, (s) => (s.network.parallel_api_requests = Number(v)))}
        />
      </div>
    {/snippet}
  </Row>
  <Row
    label={t('settings-timeouts')}
    help={t('settings-timeouts-help')}
    changed={data.network.connect_timeout_s !== DEFAULTS.connect || data.network.read_timeout_s !== DEFAULTS.read}
    defaultText={t('settings-timeouts-default')}
    onrestore={() => {
      typed = {};
      void saveSettings(app, (s) => {
        s.network.connect_timeout_s = DEFAULTS.connect;
        s.network.read_timeout_s = DEFAULTS.read;
      });
    }}
    labelId="st-timeouts"
  >
    {#snippet full()}
      <div class="st-nums" role="group" aria-labelledby="st-timeouts">
        <NumberField
          label={t('settings-timeout-connect')}
          value={typed.connect ?? String(data.network.connect_timeout_s)}
          unit={t('settings-unit-s')}
          invalid={invalid === 'connect'}
          oncommit={(text) => commitTimeout('connect', text)}
        />
        <NumberField
          label={t('settings-timeout-read')}
          value={typed.read ?? String(data.network.read_timeout_s)}
          unit={t('settings-unit-s')}
          invalid={invalid === 'read'}
          oncommit={(text) => commitTimeout('read', text)}
        />
      </div>
      {#if invalid}
        <span class="dl-field__error st-mt8" role="alert"
          ><CircleX class="dl-icon" /><span
            >{t('settings-number-error', {
              field: t(TIMEOUTS[invalid].label),
              min: formatNumber(TIMEOUTS[invalid].range.min),
              max: formatNumber(TIMEOUTS[invalid].range.max),
            })}</span
          ></span
        >
      {/if}
    {/snippet}
  </Row>
  <Row
    label={t('settings-retries')}
    help={t('settings-retries-help')}
    changed={data.network.retries !== DEFAULTS.retries}
    defaultText={formatNumber(DEFAULTS.retries)}
    onrestore={() => void saveSettings(app, (s) => (s.network.retries = DEFAULTS.retries))}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-retries')}
          options={retryOptions}
          value={String(data.network.retries)}
          onchange={(v) => void saveSettings(app, (s) => (s.network.retries = Number(v)))}
        />
      </div>
    {/snippet}
  </Row>
</Group>

<Group title={t('settings-connection-group')}>
  <Row
    label={t('settings-proxy')}
    help={t('settings-proxy-help')}
    changed={proxyMode !== DEFAULTS.proxy}
    defaultText={t('settings-proxy-none')}
    onrestore={() => chooseProxy('none')}
  >
    {#snippet control()}
      <div class="st-seg">
        <SegmentedControl
          label={t('settings-proxy')}
          options={proxyOptions}
          value={proxyMode}
          onchange={(v) => {
            chooseProxy(v);
            return undefined;
          }}
        />
      </div>
    {/snippet}
    {#snippet full()}
      {#if proxyMode === 'manual'}
        <div class="st-nums st-mt4">
          <NumberField
            label={t('settings-proxy-host')}
            value={manual.host}
            width="st-grow"
            mono
            inputmode="text"
            placeholder="proxy.example.com"
            invalid={proxyError}
            describedby="st-proxy-hint"
            oncommit={(text) => void saveManual(text, manual.port)}
          />
          <NumberField
            label={t('settings-proxy-port')}
            value={manual.port}
            width="st-num--port"
            mono
            oncommit={(text) => void saveManual(manual.host, text)}
          />
        </div>
        <p class="dl-field__hint" id="st-proxy-hint">{proxyError ? t('settings-proxy-invalid') : t('settings-proxy-hint')}</p>
      {/if}
    {/snippet}
  </Row>
</Group>
