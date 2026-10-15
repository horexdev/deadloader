<!-- Settings → Library: update checks at start (38), automatic downloads of updates (39), deleting mods (42), old
     versions (22). -->
<script lang="ts">
  import SegmentedControl from '../../lib/components/SegmentedControl.svelte';
  import Select from '../../lib/components/Select.svelte';
  import Toggle from '../../lib/components/Toggle.svelte';
  import type { AppState } from '../../lib/app.svelte';
  import { t } from '../../i18n';
  import Group from './Group.svelte';
  import Row from './Row.svelte';
  import { DEFAULTS, type SettingsData } from './model';
  import { saveSettings } from './save';

  interface Props {
    app: AppState;
    data: SettingsData;
  }

  const { app, data }: Props = $props();

  const deletionOptions = $derived([
    { value: 'permanent', label: t('settings-deletion-permanent') },
    { value: 'recycle_bin', label: t('settings-deletion-bin') },
  ]);
  // Only "keep all" exists (22); old versions are deleted by hand in a mod's details.
  const oldOptions = $derived([{ value: 'keep_all', label: t('settings-old-keep-all') }]);
</script>

<Group title={t('settings-updates-group')}>
  <Row
    label={t('settings-check-updates')}
    help={t('settings-check-updates-help')}
    changed={data.library.check_updates_at_start !== DEFAULTS.checkUpdates}
    defaultText={t('settings-on')}
    onrestore={() => void saveSettings(app, (s) => (s.library.check_updates_at_start = DEFAULTS.checkUpdates))}
  >
    {#snippet control()}
      <Toggle
        ariaLabel={t('settings-check-updates')}
        checked={data.library.check_updates_at_start}
        onchange={(on) => void saveSettings(app, (s) => (s.library.check_updates_at_start = on))}
      />
    {/snippet}
  </Row>
  <Row
    label={t('settings-auto-download')}
    help={t('settings-auto-download-help')}
    changed={data.library.download_updates_automatically !== DEFAULTS.autoDownload}
    defaultText={t('settings-off')}
    onrestore={() =>
      void saveSettings(app, (s) => (s.library.download_updates_automatically = DEFAULTS.autoDownload))}
  >
    {#snippet control()}
      <Toggle
        ariaLabel={t('settings-auto-download')}
        checked={data.library.download_updates_automatically}
        onchange={(on) => void saveSettings(app, (s) => (s.library.download_updates_automatically = on))}
      />
    {/snippet}
  </Row>
</Group>

<Group title={t('settings-disk-group')}>
  <Row
    label={t('settings-deletion')}
    help={t('settings-deletion-help')}
    changed={data.library.deletion !== DEFAULTS.deletion}
    defaultText={t('settings-deletion-permanent')}
    onrestore={() => void saveSettings(app, (s) => (s.library.deletion = 'permanent'))}
  >
    {#snippet control()}
      <SegmentedControl
        label={t('settings-deletion')}
        options={deletionOptions}
        value={data.library.deletion}
        onchange={(v) => {
          void saveSettings(app, (s) => (s.library.deletion = v as 'permanent' | 'recycle_bin'));
          return undefined;
        }}
      />
    {/snippet}
  </Row>
  <Row label={t('settings-old-versions')} help={t('settings-old-versions-help')}>
    {#snippet control()}
      <div class="st-select">
        <Select label={t('settings-old-versions')} options={oldOptions} value={data.library.old_versions} />
      </div>
    {/snippet}
  </Row>
</Group>
