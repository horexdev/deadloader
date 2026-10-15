<!-- Settings → Game: installations (21/69), the gameinfo.gi auto-fix (18), Rollback (D7) and Delete all data (D8). -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import FolderPlus from '@lucide/svelte/icons/folder-plus';
  import Gamepad2 from '@lucide/svelte/icons/gamepad-2';
  import Info from '@lucide/svelte/icons/info';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import Undo2 from '@lucide/svelte/icons/undo-2';
  import { onMount } from 'svelte';
  import Banner from '../../lib/components/Banner.svelte';
  import Button from '../../lib/components/Button.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import Toggle from '../../lib/components/Toggle.svelte';
  import { api, maintenance, type InstallationView, type RollbackPreview, type RolledBack } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { formatList, formatNumber, formatSize, t } from '../../i18n';
  import Group from './Group.svelte';
  import RollbackDialog from './RollbackDialog.svelte';
  import Row from './Row.svelte';
  import WipeDialog from './WipeDialog.svelte';
  import { DEFAULTS, type SettingsData } from './model';
  import { saveSettings } from './save';

  interface Props {
    app: AppState;
    data: SettingsData;
    /** Opens D7 or D8 right away (the boards SettingsD7, SettingsD8). */
    initialDialog?: 'd7' | 'd8' | null;
  }

  const { app, data, initialDialog = null }: Props = $props();

  let installs = $state<InstallationView[]>([]);
  let preview = $state<RollbackPreview | null>(null);
  let noGame = $state(false);
  let dataBytes = $state<number | null>(null);
  let rolled = $state<RolledBack | null>(null);
  let dialog = $state<'d7' | 'd8' | null>(null);

  async function refresh(): Promise<void> {
    installs = await maintenance.installations().catch(() => installs);
    try {
      preview = await maintenance.rollbackPreview();
      noGame = false;
    } catch {
      preview = null;
      noGame = true;
    }
  }

  onMount(() => {
    dialog = initialDialog;
    void refresh();
    maintenance
      .wipeSummary()
      .then((s) => (dataBytes = s.total_bytes))
      .catch(() => undefined);
  });

  const drive = (path: string) => /^[A-Za-z]:/.exec(path)?.[0] ?? path;

  function used(i: InstallationView): string {
    const names = i.profiles.map((name) =>
      name === app.library?.selected_profile
        ? t('settings-install-selected', { name })
        : name === i.applied_profile
          ? t('settings-install-applied', { name })
          : name,
    );
    return t('settings-install-used', { count: names.length, names: formatList(names) });
  }

  function meta(i: InstallationView): string {
    const parts = [
      i.source === 'manual' ? t('settings-install-manual-meta') : i.library ? t('settings-install-library', { library: i.library }) : null,
      i.build_id !== null ? t('settings-install-build', { build: formatNumber(i.build_id) }) : null,
      used(i),
    ];
    return parts.filter((p): p is string => p !== null).join(' · ');
  }

  function gameinfoText(i: InstallationView): { text: string; ok: boolean; warn: boolean } {
    switch (i.gameinfo) {
      case 'ours':
        return { text: t('settings-gi-ours'), ok: true, warn: false };
      case 'vanilla':
        return {
          text: i.profiles.length > 0 ? t('settings-gi-rolled') : t('settings-gi-wait'),
          ok: false,
          warn: false,
        };
      case 'foreign':
        return { text: t('settings-gi-foreign'), ok: false, warn: true };
      default:
        return { text: t('settings-gi-missing'), ok: false, warn: true };
    }
  }

  async function addManually(): Promise<void> {
    try {
      const root = await maintenance.gameFolderDialog({ title: t('settings-add-manual-title'), filter: '' }, false);
      if (root) {
        app.settings = await api.settings();
        app.toast({ kind: 'success', title: t('settings-install-added'), text: root });
        await refresh();
      }
    } catch (error) {
      app.fail(error);
    }
  }

  async function remove(root: string): Promise<void> {
    try {
      app.settings = await maintenance.removeInstallation(root);
      app.toast({ kind: 'success', title: t('settings-install-removed'), text: t('settings-install-removed-text') });
      await refresh();
    } catch (error) {
      app.fail(error);
    }
  }

  function openGame(root: string): void {
    maintenance.openFolder({ place: 'game', root }).catch((error: unknown) => app.fail(error));
  }

  async function rolledBack(done: RolledBack): Promise<void> {
    dialog = null;
    rolled = done;
    await app.reloadLibrary();
    await refresh();
  }

  const rollbackHelp = $derived.by(() => {
    if (noGame || !preview) {
      return t('settings-rollback-no-game');
    }
    if (preview.files === 0 && preview.gameinfo !== 'ours') {
      return t('settings-rollback-nothing', { drive: drive(preview.game_root) });
    }
    const main = t('settings-rollback-help', {
      profile: preview.selected_profile,
      drive: drive(preview.game_root),
      files: preview.files,
    });
    return preview.others.length > 0 ? `${main} ${t('settings-rollback-help-others')}` : main;
  });
  const nothingToRollBack = $derived(noGame || !preview || (preview.files === 0 && preview.gameinfo !== 'ours'));
</script>

<Group title={t('settings-installs-group')} lead={t('settings-installs-lead')}>
  {#each installs as install (install.game_root)}
    {@const gi = gameinfoText(install)}
    <div class="st-item st-item--top">
      <span class="st-item__icon"><Gamepad2 class="dl-icon" /></span>
      <div class="st-install">
        <div class="st-pathline st-pathline--wrap"><PathTooltip path={install.game_root} /></div>
        <div class="st-meta">{meta(install)}</div>
        <div class="st-meta st-meta-row">
          <span class="st-stat st-stat--ok">
            <CircleCheck class="dl-icon" /><Text value={install.source === 'steam'
              ? t('settings-install-steam')
              : install.source === 'manual'
                ? t('settings-install-manual')
                : t('settings-install-known')} />
          </span>
          <span class={['st-stat', gi.ok && 'st-stat--ok']}>
            {#if gi.ok}<CircleCheck class="dl-icon" />{:else if gi.warn}<TriangleAlert class="dl-icon" />{:else}<Info
                class="dl-icon"
              />{/if}<Text value={gi.text} />
          </span>
        </div>
      </div>
      <div class="st-install__actions">
        <Button
          variant="ghost"
          size="sm"
          iconOnly
          icon={FolderOpen}
          label={t('settings-open-folder-at', { path: install.game_root })}
          title={t('settings-open-folder')}
          onclick={() => openGame(install.game_root)}
        />
        {#if install.source === 'manual'}
          <Button
            variant="ghost"
            size="sm"
            label={t('settings-remove')}
            aria-label={t('settings-remove-item', { item: install.game_root })}
            onclick={() => void remove(install.game_root)}
          />
        {/if}
      </div>
    </div>
  {:else}
    <div class="st-item"><span class="st-meta">{t('settings-installs-none')}</span></div>
  {/each}
  <div class="st-item">
    <Button icon={FolderPlus} label={t('settings-add-manual')} onclick={() => void addManually()} />
    <span class="st-meta">{t('settings-add-manual-hint')}</span>
  </div>
</Group>

<Group title={t('settings-gameinfo-group')}>
  <Row
    label={t('settings-autofix')}
    help={t('settings-autofix-help')}
    changed={data.game.auto_fix_gameinfo !== DEFAULTS.autoFix}
    defaultText={t('settings-on')}
    onrestore={() => void saveSettings(app, (s) => (s.game.auto_fix_gameinfo = DEFAULTS.autoFix))}
  >
    {#snippet control()}
      <Toggle
        ariaLabel={t('settings-autofix')}
        checked={data.game.auto_fix_gameinfo}
        onchange={(on) => void saveSettings(app, (s) => (s.game.auto_fix_gameinfo = on))}
      />
    {/snippet}
  </Row>
</Group>

<Group title={t('settings-danger-group')} danger>
  {#if rolled}
    <div class="st-item st-banner-row">
      <div class="st-grow">
        <Banner
          kind="success"
          title={t('settings-rolled-title', { drive: drive(rolled.game_root) })}
          text={t('settings-rolled-text', {
            drive: drive(rolled.game_root),
            files: rolled.removed,
            foreign: rolled.foreign_removed.length,
          })}
        />
      </div>
    </div>
  {/if}
  <Row label={t('settings-rollback')} help={rollbackHelp}>
    {#snippet control()}
      <Button
        icon={Undo2}
        label={t('settings-rollback-button')}
        disabled={nothingToRollBack}
        onclick={() => (dialog = 'd7')}
      />
    {/snippet}
  </Row>
  <Row
    label={t('settings-wipe')}
    help={t('settings-wipe-help', { size: dataBytes === null ? '…' : formatSize(dataBytes) })}
  >
    {#snippet control()}
      <Button icon={Trash2} class="st-btn-danger-text" label={t('settings-wipe-button')} onclick={() => (dialog = 'd8')} />
    {/snippet}
  </Row>
</Group>

{#if dialog === 'd7' && preview}
  <RollbackDialog {app} {preview} oncancel={() => (dialog = null)} ondone={(done) => void rolledBack(done)} />
{/if}
{#if dialog === 'd8'}
  <WipeDialog {app} oncancel={() => (dialog = null)} />
{/if}
