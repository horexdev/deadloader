<!-- Settings → General: language, profile at start, deadloader's update check (157), zoom. -->
<script lang="ts">
  import { onMount } from 'svelte';
  import Select from '../../lib/components/Select.svelte';
  import Toggle from '../../lib/components/Toggle.svelte';
  import { api } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { LANGS, LANGUAGE_NAMES, formatNumber, language, setLanguage, t } from '../../i18n';
  import Group from './Group.svelte';
  import Row from './Row.svelte';
  import { CHOICES, DEFAULTS, withCurrent, type SettingsData } from './model';
  import { saveSettings } from './save';

  interface Props {
    app: AppState;
    data: SettingsData;
  }

  const { app, data }: Props = $props();

  let profiles = $state<string[]>([]);
  onMount(() => {
    api
      .profileNames()
      .then((names) => (profiles = names))
      .catch(() => undefined);
  });

  const local = $derived(!data.catalog.enabled);
  const languageOptions = $derived([
    { value: 'system', label: t('settings-language-system', { language: LANGUAGE_NAMES[language()] }) },
    ...LANGS.map((code) => ({ value: code, label: LANGUAGE_NAMES[code] })),
  ]);
  const lastSelected = $derived(app.library?.selected_profile);
  const profileValue = $derived(
    typeof data.general.startup_profile === 'string' ? 'last_selected' : `named:${data.general.startup_profile.named}`,
  );
  const profileOptions = $derived([
    {
      value: 'last_selected',
      label: lastSelected ? t('settings-startup-last', { name: lastSelected }) : t('settings-startup-last-plain'),
    },
    ...profiles.map((name) => ({ value: `named:${name}`, label: name })),
  ]);
  const percent = (value: number) => t('settings-percent', { value: formatNumber(value) });
  const zoomOptions = $derived(
    withCurrent(CHOICES.zoom, data.general.zoom_percent).map((z) => ({ value: String(z), label: percent(z) })),
  );

  async function chooseLanguage(value: string): Promise<void> {
    if (await saveSettings(app, (s) => (s.general.language = value))) {
      const system = await api.systemLocale().catch(() => ({ languages: [...navigator.languages], region: null }));
      await setLanguage(value, system.languages.length > 0 ? system.languages : navigator.languages);
    }
  }

  function chooseProfile(value: string): void {
    void saveSettings(app, (s) => {
      s.general.startup_profile = value === 'last_selected' ? 'last_selected' : { named: value.slice('named:'.length) };
    });
  }
</script>

<Group title={t('settings-general-group')}>
  <Row
    label={t('settings-language')}
    help={t('settings-language-help')}
    changed={data.general.language !== DEFAULTS.language}
    defaultText={languageOptions[0]?.label}
    onrestore={() => void chooseLanguage(DEFAULTS.language)}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-language')}
          options={languageOptions}
          value={data.general.language}
          onchange={(value) => void chooseLanguage(value)}
        />
      </div>
    {/snippet}
  </Row>
  <Row
    label={t('settings-startup-profile')}
    help={t('settings-startup-profile-help')}
    changed={profileValue !== 'last_selected'}
    defaultText={profileOptions[0]?.label}
    onrestore={() => chooseProfile('last_selected')}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-startup-profile')}
          options={profileOptions}
          value={profileValue}
          onchange={chooseProfile}
        />
      </div>
    {/snippet}
  </Row>
  <Row
    label={t('settings-app-updates')}
    help={local ? t('settings-app-updates-local') : t('settings-app-updates-help')}
    changed={!local && data.general.check_app_updates !== DEFAULTS.checkAppUpdates}
    defaultText={t('settings-off')}
    onrestore={() => void saveSettings(app, (s) => (s.general.check_app_updates = DEFAULTS.checkAppUpdates))}
  >
    {#snippet control()}
      <Toggle
        ariaLabel={t('settings-app-updates')}
        checked={data.general.check_app_updates && !local}
        disabled={local}
        onchange={(on) => void saveSettings(app, (s) => (s.general.check_app_updates = on))}
      />
    {/snippet}
  </Row>
  <Row
    label={t('settings-zoom')}
    changed={data.general.zoom_percent !== DEFAULTS.zoom}
    defaultText={percent(DEFAULTS.zoom)}
    onrestore={() => void saveSettings(app, (s) => (s.general.zoom_percent = DEFAULTS.zoom))}
  >
    {#snippet helpContent()}
      {t('settings-zoom-keys-1')} <span class="dl-kbd">{t('settings-ctrl')}</span> + <span class="dl-kbd">+</span> /
      <span class="dl-kbd">−</span>
      {t('settings-zoom-keys-2')} <span class="dl-kbd">{t('settings-ctrl')}</span> + <span class="dl-kbd">0</span>
      {t('settings-zoom-keys-3')}
    {/snippet}
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-zoom')}
          options={zoomOptions}
          value={String(data.general.zoom_percent)}
          onchange={(value) => void saveSettings(app, (s) => (s.general.zoom_percent = Number(value)))}
        />
      </div>
    {/snippet}
  </Row>
</Group>
