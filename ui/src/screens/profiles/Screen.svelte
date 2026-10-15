<!-- S5 Profiles (boards Profiles, ProfilesNew, ProfilesRename, ProfilesDelete, ProfilesImportError, ProfilesD9 and the
     German ones): every profile with selected / applied markers, game installation, mods and last apply; select,
     apply, new, rename, duplicate, export, import (D9), change installation (153), delete. -->
<script lang="ts">
  import Download from '@lucide/svelte/icons/download';
  import FileInput from '@lucide/svelte/icons/file-input';
  import Gamepad from '@lucide/svelte/icons/gamepad-2';
  import Info from '@lucide/svelte/icons/info';
  import ListTodo from '@lucide/svelte/icons/list-todo';
  import Plus from '@lucide/svelte/icons/plus';
  import Undo from '@lucide/svelte/icons/undo-2';
  import { onMount } from 'svelte';
  import Banner from '../../lib/components/Banner.svelte';
  import Button from '../../lib/components/Button.svelte';
  import { api, isUserMessage, shell, type ProfileImported, type ProfileOverview, type ProfilesView } from '../../lib/api';
  import { app as shared, type AppState } from '../../lib/app.svelte';
  import { describe, t } from '../../i18n';
  import DeleteDialog from './DeleteDialog.svelte';
  import InstallationDialog from './InstallationDialog.svelte';
  import MissingDialog from './MissingDialog.svelte';
  import NewProfileDialog from './NewProfileDialog.svelte';
  import ProfileRow from './ProfileRow.svelte';
  import { copyName, downloadsFor, driveOf, folderHolder, inGame, nameProblem, nextSelected, rootOf } from './profiles';
  import './profiles.css';

  interface Props {
    /** The app state (tests pass their own). */
    app?: AppState;
  }

  const { app = shared }: Props = $props();

  type Dialog =
    | { kind: 'new' }
    | { kind: 'delete'; profile: ProfileOverview }
    | { kind: 'installation'; profile: ProfileOverview }
    | { kind: 'missing'; imported: ProfileImported; previous: string };

  let view = $state<ProfilesView | null>(null);
  let now = $state(Date.now());
  let dialog = $state<Dialog | null>(null);
  let renaming = $state<{ name: string; value: string; error: string | null } | null>(null);
  let refused = $state<string | null>(null);

  const names = $derived(view?.profiles.map((p) => p.name) ?? []);
  const selected = $derived(view?.profiles.find((p) => p.name === view?.selected));
  const selectedRoot = $derived(selected && view ? rootOf(selected, view.installations) : null);
  const holder = $derived(view ? folderHolder(view) : null);
  const pending = $derived(selected !== undefined && !selected.applied);
  const first = $derived(view?.installations[0] ?? null);
  const elsewhere = $derived(view !== null && view.installations.length > 1 && selectedRoot !== null && selectedRoot !== first);
  const firstHolder = $derived(view?.profiles.find((p) => p.applied && view && rootOf(p, view.installations) === first));
  const catalogOn = $derived(app.settings?.catalog?.enabled === true);
  const summary = $derived.by(() => {
    if (!view) {
      return '';
    }
    const playing = inGame(view).map((g) => t('profiles-in-game-item', { name: g.name, drive: g.drive }));
    return [
      t('profiles-count', { count: view.profiles.length }),
      playing.length > 0 ? t('profiles-in-game', { list: playing.join(', ') }) : t('profiles-in-game-none'),
    ];
  });
  const dialogText = $derived({ title: t('profiles-import-dialog'), filter: t('profiles-file-filter') });

  onMount(() => {
    void load();
  });

  async function load(): Promise<void> {
    try {
      view = await api.profilesOverview();
      now = Date.now();
    } catch (error) {
      app.fail(error);
    }
  }

  /** After anything that may change the selected profile: the library and the sidebar follow. */
  async function refresh(): Promise<void> {
    await Promise.all([load(), app.reloadLibrary()]);
    app.game = await shell.gameStatus().catch(() => app.game);
  }

  async function run(work: () => Promise<void>): Promise<void> {
    try {
      await work();
    } catch (error) {
      app.fail(error);
    }
  }

  function select(name: string): Promise<void> {
    return run(async () => {
      app.library = await api.selectProfile(name);
      await refresh();
    });
  }

  /** Apply the selected profile (F6); N8 says what changed, Undo puts the previous one back. */
  function applySelected(): Promise<void> {
    return run(async () => {
      if (!selected) {
        return;
      }
      const name = selected.name;
      const previous = holder?.name ?? null;
      const preview = await shell.applyPreview().catch(() => null);
      const result = await api.apply();
      app.library = result.library;
      await refresh();
      const enabled = view?.profiles.find((p) => p.name === name)?.mods ?? 0;
      const parts = [
        preview && preview.turned_off.length > 0 ? t('profiles-turned-off', { count: preview.turned_off.length }) : '',
        preview && preview.turned_on.length > 0 ? t('profiles-turned-on', { count: preview.turned_on.length }) : '',
      ].filter(Boolean);
      app.toast({
        kind: 'success',
        title: describe({ id: 'N8', args: { name, count: enabled } }).what,
        text: parts.length > 0 ? `${parts.join(', ')}. ${t('profiles-differ')}` : t('profiles-same'),
        ...(previous
          ? {
              action: {
                label: t('profiles-undo'),
                icon: Undo,
                onclick: () => void select(previous).then(() => applySelected()),
              },
            }
          : {}),
      });
    });
  }

  function startRename(profile: ProfileOverview): void {
    renaming = { name: profile.name, value: profile.name, error: null };
  }

  function saveRename(): Promise<void> {
    const edit = renaming;
    if (!edit) {
      return Promise.resolve();
    }
    const problem = nameProblem(edit.value, names, edit.name);
    if (problem === 'empty') {
      renaming = { ...edit, error: t('profiles-name-empty') };
      return Promise.resolve();
    }
    if (problem === 'taken') {
      renaming = { ...edit, error: `${t('e21', { name: edit.value.trim() })} ${t('e21.todo')}` };
      return Promise.resolve();
    }
    const next = edit.value.trim();
    if (next === edit.name) {
      renaming = null;
      return Promise.resolve();
    }
    return api
      .renameProfile(edit.name, next)
      .then(async () => {
        renaming = null;
        await refresh();
      })
      .catch((error: unknown) => {
        const text = describe(isUserMessage(error) ? error : { id: 'Internal', args: {} });
        renaming = { ...edit, error: [text.what, text.todo].filter(Boolean).join(' ') };
      });
  }

  function duplicate(profile: ProfileOverview): Promise<void> {
    return run(async () => {
      const name = copyName(profile.name, names, (base, n) =>
        n === 1 ? t('profiles-copy-name', { name: base }) : t('profiles-copy-name-n', { name: base, number: n }),
      );
      await api.duplicateProfile(profile.name, name);
      await load();
      renaming = { name, value: name, error: null };
    });
  }

  function exportProfile(profile: ProfileOverview): Promise<void> {
    return run(async () => {
      const saved = await api.exportProfileDialog(profile.name, {
        title: t('profiles-export-dialog'),
        filter: t('profiles-file-filter'),
      });
      if (saved) {
        app.toast({ kind: 'neutral', title: t('profiles-exported', { name: profile.name }) });
      }
    });
  }

  function remove(profile: ProfileOverview): void {
    if (profile.applied) {
      dialog = { kind: 'delete', profile };
    } else {
      void deleteNow(profile);
    }
  }

  /** Deletes a profile; one that wasn't applied can be brought back (Undo re-imports its export). */
  function deleteNow(profile: ProfileOverview): Promise<void> {
    dialog = null;
    return run(async () => {
      const wasSelected = view?.selected === profile.name;
      const before = view?.selected ?? null;
      const copy = profile.applied ? null : await api.exportProfile(profile.name).catch(() => null);
      await api.deleteProfile(profile.name);
      await refresh();
      const undo = async () => {
        if (!copy) {
          return;
        }
        const back = await api.importProfile(copy);
        if (profile.game_root) {
          await api.setProfileInstallation(back.name, profile.game_root);
        }
        if (!wasSelected && before) {
          app.library = await api.selectProfile(before);
        }
        await refresh();
      };
      app.toast({
        kind: 'neutral',
        title: t('profiles-deleted', { name: profile.name }),
        text: profile.applied
          ? profile.mods > 0
            ? t('profiles-deleted-applied', { count: profile.mods })
            : undefined
          : t('profiles-deleted-not-applied'),
        ...(copy ? { action: { label: t('profiles-undo'), icon: Undo, onclick: () => void run(undo) } } : {}),
      });
    });
  }

  function saveInstallation(profile: ProfileOverview, root: string): Promise<void> {
    dialog = null;
    return run(async () => {
      view = await api.setProfileInstallation(profile.name, root);
      await refresh();
    });
  }

  async function createProfile(name: string, copy: boolean): Promise<void> {
    refused = null;
    try {
      if (copy && selected) {
        await api.duplicateProfile(selected.name, name);
      } else {
        await api.createProfile(name);
      }
      if (selectedRoot) {
        await api.setProfileInstallation(name, selectedRoot);
      }
      dialog = null;
      await load();
    } catch (error) {
      const text = describe(isUserMessage(error) ? error : { id: 'Internal', args: { detail: String(error) } });
      refused = [text.what, text.todo].filter(Boolean).join(' ');
    }
  }

  async function importProfile(): Promise<void> {
    const previous = view?.selected ?? '';
    let imported: ProfileImported | null;
    try {
      imported = await api.importProfileDialog(dialogText);
    } catch (error) {
      if (isUserMessage(error) && error.id === 'E17') {
        app.showError(error, [
          { label: t('profiles-check-update'), place: 'start', href: '#/settings/about' },
          { label: t('profiles-choose-another'), place: 'primary', icon: FileInput, run: () => void importProfile() },
        ]);
      } else {
        app.fail(error);
      }
      return;
    }
    if (!imported) {
      return;
    }
    await refresh();
    if (imported.missing.length > 0) {
      dialog = { kind: 'missing', imported, previous };
    } else {
      app.toast({ kind: 'success', title: t('profiles-imported', { name: imported.name }), text: t('profiles-imported-text') });
    }
  }

  /** Cancel in D9: the import is undone and the previous profile selected again. */
  function cancelImport(imported: ProfileImported, previous: string): Promise<void> {
    dialog = null;
    return run(async () => {
      await api.deleteProfile(imported.name);
      if (previous) {
        app.library = await api.selectProfile(previous);
      }
      await refresh();
    });
  }

  function importWithout(imported: ProfileImported): void {
    dialog = null;
    app.toast({
      kind: 'success',
      title: t('profiles-imported-without', { name: imported.name, count: imported.missing.length }),
      text: t('profiles-imported-without-text'),
    });
  }

  function downloadMissing(imported: ProfileImported): Promise<void> {
    dialog = null;
    return run(async () => {
      const wanted = downloadsFor(imported.missing);
      const started = await api.downloadMissing(wanted);
      for (const failure of started.failed) {
        app.fail(failure.message);
      }
      const count = wanted.length - new Set(started.failed.map((f) => f.name)).size;
      if (count > 0) {
        app.toast({
          kind: 'info',
          icon: Download,
          title: t('profiles-imported-downloading', { name: imported.name, count }),
          text: t('profiles-imported-downloading-text'),
          action: { label: t('profiles-open-downloads'), onclick: () => (location.hash = '#/downloads') },
        });
      }
    });
  }

</script>

<div class="screen">
  <div class="header">
    <div class="heading">
      <h1>{t('profiles-title')}</h1>
      {#if view}
        <div class="summary"><span>{summary[0]}</span><span class="pf-sep">·</span><span>{summary[1]}</span></div>
      {/if}
    </div>
    <div class="buttons">
      <Button icon={FileInput} label={t('profiles-import')} title={t('profiles-import-title')} onclick={() => void importProfile()} />
      <Button icon={Plus} label={t('profiles-new')} aria-haspopup="dialog" onclick={() => ((refused = null), (dialog = { kind: 'new' }))} />
    </div>
  </div>

  {#if pending && selected}
    <div class="pending">
      <Banner
        kind="info"
        icon={ListTodo}
        title={t('profiles-pending-what', { name: selected.name })}
        text={holder
          ? t('profiles-pending-why-other', { drive: driveOf(selectedRoot), other: holder.name })
          : t('profiles-pending-why-empty', { drive: driveOf(selectedRoot) })}
      >
        {#snippet actions()}
          {#if holder}
            {@const keep = holder.name}
            <Button variant="ghost" size="sm" label={t('profiles-keep', { name: keep })} onclick={() => void select(keep)} />
          {/if}
          <Button variant="primary" size="sm" label={t('profiles-apply', { name: selected.name })} onclick={() => void applySelected()} />
        {/snippet}
      </Banner>
    </div>
  {/if}

  {#if elsewhere && selected && selectedRoot && first}
    <Banner
      kind="neutral"
      icon={Gamepad}
      title={t('profiles-other-what', { name: selected.name })}
      text={firstHolder
        ? t('profiles-other-why', { path: selectedRoot, drive: driveOf(first), other: firstHolder.name })
        : t('profiles-other-why-empty', { path: selectedRoot, drive: driveOf(first) })}
    />
  {/if}

  <div class="scroll">
    {#if view}
      {@const current = view}
      <div class="pf-list" role="list" aria-label={t('profiles-title')}>
        {#each current.profiles as profile (profile.name)}
          <ProfileRow
            {profile}
            selected={profile.name === current.selected}
            root={rootOf(profile, current.installations)}
            {now}
            deletable={current.profiles.length > 1}
            renaming={renaming?.name === profile.name ? { value: renaming.value, error: renaming.error } : null}
            onselect={() => void select(profile.name)}
            onrename={() => startRename(profile)}
            onrenameinput={(value) => renaming && (renaming = { ...renaming, value, error: null })}
            onrenamesave={() => void saveRename()}
            onrenamecancel={() => (renaming = null)}
            onduplicate={() => void duplicate(profile)}
            onexport={() => void exportProfile(profile)}
            oninstallation={() => (dialog = { kind: 'installation', profile })}
            ondelete={() => remove(profile)}
          />
        {/each}
      </div>
      <p class="pf-note footnote"><Info class="dl-icon" /><span>{t('profiles-footnote')}</span></p>
    {/if}
  </div>
</div>

{#if view && dialog?.kind === 'new'}
  <NewProfileDialog
    {names}
    {selected}
    root={selectedRoot}
    {refused}
    oncreate={(name, copy) => void createProfile(name, copy)}
    oncancel={() => (dialog = null)}
  />
{:else if view && dialog?.kind === 'delete'}
  {@const profile = dialog.profile}
  <DeleteDialog
    {profile}
    root={rootOf(profile, view.installations)}
    next={view.selected === profile.name ? nextSelected(names, profile.name) : null}
    onexport={() => void exportProfile(profile)}
    ondelete={() => void deleteNow(profile)}
    oncancel={() => (dialog = null)}
  />
{:else if view && dialog?.kind === 'installation'}
  {@const profile = dialog.profile}
  <InstallationDialog
    {profile}
    installations={view.installations}
    profiles={view.profiles}
    onsave={(root) => void saveInstallation(profile, root)}
    oncancel={() => (dialog = null)}
  />
{:else if dialog?.kind === 'missing'}
  {@const imported = dialog.imported}
  {@const previous = dialog.previous}
  <MissingDialog
    {imported}
    {catalogOn}
    ondownload={() => void downloadMissing(imported)}
    onwithout={() => importWithout(imported)}
    oncancel={() => void cancelImport(imported, previous)}
  />
{/if}

<style>
  /* From ProfilesScreen.dc.html: the page layout. */
  .screen {
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: 100%;
    box-sizing: border-box;
    min-width: 0;
    padding: 20px 24px 0;
    overflow: hidden;
  }

  .header {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
    gap: 12px 16px;
    flex: none;
  }

  .heading {
    display: grid;
    gap: 4px;
    flex: 1 1 320px;
    min-width: 0;
  }

  h1 {
    margin: 0;
    font: var(--type-title-lg);
    color: var(--ink);
  }

  .summary {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 2px 8px;
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .pf-sep {
    color: var(--ink-disabled);
  }

  .buttons {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
  }

  .pending,
  .screen > :global(.dl-banner) {
    flex: none;
  }

  .scroll {
    flex: 1 1 auto;
    min-height: 0;
    overflow: auto;
    margin: 0 -4px;
    padding: 2px 4px 24px;
  }

  .pf-list {
    display: grid;
    gap: 8px;
    align-content: start;
  }

  .footnote {
    margin: 16px 0 0;
    max-width: 720px;
  }
</style>
