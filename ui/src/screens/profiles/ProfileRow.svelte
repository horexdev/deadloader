<!-- One profile (board Profiles): its markers (selected, applied, not applied yet), mods, game installation and last
     apply, the first mods by name, Select, and the "more" menu. Renaming happens in place. -->
<script lang="ts">
  import CircleDot from '@lucide/svelte/icons/circle-dot';
  import CirclePlus from '@lucide/svelte/icons/copy-plus';
  import Ellipsis from '@lucide/svelte/icons/ellipsis';
  import FileOutput from '@lucide/svelte/icons/file-output';
  import FolderCog from '@lucide/svelte/icons/folder-cog';
  import Gamepad from '@lucide/svelte/icons/gamepad-2';
  import Layers from '@lucide/svelte/icons/layers';
  import ListTodo from '@lucide/svelte/icons/list-todo';
  import Pencil from '@lucide/svelte/icons/pencil';
  import Trash from '@lucide/svelte/icons/trash';
  import { tick } from 'svelte';
  import Button from '../../lib/components/Button.svelte';
  import FieldError from '../../lib/components/FieldError.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import type { ProfileOverview } from '../../lib/api';
  import { formatDate, formatTime, language, t } from '../../i18n';
  import { capitalize, driveOf, since } from './profiles';

  interface Props {
    profile: ProfileOverview;
    selected: boolean;
    /** The game folder it goes into. */
    root: string | null;
    now: number;
    /** The last profile can't be deleted. */
    deletable: boolean;
    /** Rename in place: the field's value and its problem. */
    renaming: { value: string; error: string | null } | null;
    onselect: () => void;
    onrename: () => void;
    onrenameinput: (value: string) => void;
    onrenamesave: () => void;
    onrenamecancel: () => void;
    onduplicate: () => void;
    onexport: () => void;
    oninstallation: () => void;
    ondelete: () => void;
  }

  const {
    profile,
    selected,
    root,
    now,
    deletable,
    renaming,
    onselect,
    onrename,
    onrenameinput,
    onrenamesave,
    onrenamecancel,
    onduplicate,
    onexport,
    oninstallation,
    ondelete,
  }: Props = $props();

  const uid = $props.id();
  let menuOpen = $state(false);
  let menu: HTMLElement | undefined = $state();
  let field: HTMLInputElement | undefined = $state();

  const mods = $derived(profile.mods === 0 ? t('profiles-mods-none') : t('profiles-mods', { count: profile.mods }));
  const when = $derived.by(() => {
    const at = profile.last_applied_unix_ms;
    if (at === null) {
      return { text: t('profiles-never-applied'), exact: '' };
    }
    const relative = since(at, now, language()) ?? t('profiles-just-now');
    const text = profile.applied ? t('profiles-applied-when', { when: relative }) : t('profiles-last-applied', { when: relative });
    return { text: capitalize(text), exact: `${formatDate(at)}, ${formatTime(at)}` };
  });
  const preview = $derived(profile.mod_names.length > 0 ? profile.mod_names.join(', ') : t('profiles-preview-empty'));
  const pending = $derived(selected && !profile.applied);

  $effect(() => {
    if (renaming && field) {
      const input = field;
      void tick().then(() => {
        input.focus();
        input.select();
      });
    }
  });

  async function toggleMenu() {
    menuOpen = !menuOpen;
    if (menuOpen) {
      await tick();
      menu?.querySelector<HTMLElement>('[role="menuitem"]')?.focus();
    }
  }

  function choose(action: () => void) {
    menuOpen = false;
    action();
  }

  function menuKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      event.preventDefault();
      event.stopPropagation();
      menuOpen = false;
    }
  }

  function renameKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      onrenamesave();
    } else if (event.key === 'Escape') {
      event.preventDefault();
      event.stopPropagation();
      onrenamecancel();
    }
  }
</script>

<div class={['pf-row', selected && 'is-sel', menuOpen && 'is-menu']} role="listitem" aria-current={selected ? 'true' : undefined}>
  <span class="pf-tile" aria-hidden="true"><Layers class="dl-icon" /></span>
  <div class="pf-main">
    {#if renaming}
      <div class="dl-field rename">
        <div class="rename-line">
          <div class="dl-input rename-input">
            <input
              bind:this={field}
              type="text"
              data-rename="true"
              value={renaming.value}
              aria-label={t('profiles-name-label')}
              aria-invalid={renaming.error ? 'true' : 'false'}
              aria-describedby={`${uid}-msg`}
              oninput={(event) => onrenameinput(event.currentTarget.value)}
              onkeydown={renameKeydown}
            />
          </div>
          <Button size="sm" label={t('profiles-save')} onclick={onrenamesave} />
          <Button size="sm" variant="ghost" label={t('profiles-cancel')} onclick={onrenamecancel} />
        </div>
        {#if renaming.error}
          <FieldError id={`${uid}-msg`} message={renaming.error} />
        {:else}
          <span class="dl-field__hint" id={`${uid}-msg`}>{t('profiles-rename-hint')}</span>
        {/if}
      </div>
    {:else}
      <div class="pf-nameline">
        <span class="pf-name" title={profile.name}>{profile.name}</span>
        {#if selected}
          <span class="pf-mark pf-mark--sel" title={t('profiles-selected-title')}
            ><CircleDot class="dl-icon" />{t('profiles-selected')}</span
          >
        {/if}
        {#if profile.applied}
          <span class="pf-mark pf-mark--app" title={t('profiles-applied-title', { drive: driveOf(root) })}
            ><Gamepad class="dl-icon" />{t('profiles-applied')}</span
          >
        {/if}
        {#if pending}
          <span class="pf-mark pf-mark--pend" title={t('profiles-pending-title')}
            ><ListTodo class="dl-icon" />{t('profiles-not-applied')}</span
          >
        {/if}
      </div>
    {/if}
    <div class="pf-meta">
      <span>{mods}</span>
      {#if root}<span class="pf-sep">·</span><span class="pf-path"><PathTooltip path={root} /></span>{/if}
      <span class="pf-sep">·</span><span title={when.exact || undefined}>{when.text}</span>
    </div>
    <div class="pf-preview" title={preview}>{preview}</div>
  </div>
  <div class="pf-actions">
    {#if !selected && !renaming}
      <Button size="sm" class="select" label={t('profiles-select')} aria-label={t('profiles-select-of', { name: profile.name })} onclick={onselect} />
    {/if}
    <button
      type="button"
      class="dl-btn dl-btn--ghost dl-btn--icon dl-btn--sm"
      aria-haspopup="menu"
      aria-expanded={menuOpen ? 'true' : 'false'}
      aria-label={t('profiles-more-of', { name: profile.name })}
      title={t('profiles-more-of', { name: profile.name })}
      onclick={() => void toggleMenu()}><Ellipsis class="dl-icon" /></button
    >
    {#if menuOpen}
      <div class="pf-catcher" role="presentation" onclick={() => (menuOpen = false)}></div>
      <!-- svelte-ignore a11y_interactive_supports_focus -->
      <div class="dl-menu pf-menu" role="menu" aria-label={t('profiles-more-of', { name: profile.name })} bind:this={menu} onkeydown={menuKeydown}>
        <button type="button" class="dl-option" role="menuitem" onclick={() => choose(onrename)}
          ><Pencil class="dl-icon" />{t('profiles-rename')}</button
        >
        <button type="button" class="dl-option" role="menuitem" onclick={() => choose(onduplicate)}
          ><CirclePlus class="dl-icon" />{t('profiles-duplicate')}</button
        >
        <button type="button" class="dl-option" role="menuitem" onclick={() => choose(onexport)}
          ><FileOutput class="dl-icon" />{t('profiles-export')}</button
        >
        <button type="button" class="dl-option" role="menuitem" onclick={() => choose(oninstallation)}
          ><FolderCog class="dl-icon" />{t('profiles-change-installation')}</button
        >
        <div class="dl-menu__sep" role="separator"></div>
        <button
          type="button"
          class="dl-option pf-danger"
          role="menuitem"
          aria-disabled={deletable ? 'false' : 'true'}
          title={deletable ? undefined : t('profiles-delete-last')}
          onclick={() => deletable && choose(ondelete)}
          ><Trash class="dl-icon" />{profile.applied ? t('profiles-delete-confirm') : t('profiles-delete')}</button
        >
      </div>
    {/if}
  </div>
</div>

<style>
  /* From ProfilesScreen.dc.html: the profile row and its menu. */
  .pf-row {
    position: relative;
    display: grid;
    grid-template-columns: 40px minmax(0, 1fr) auto;
    gap: 12px;
    align-items: center;
    min-height: 84px;
    padding: 12px 12px 12px 16px;
    box-sizing: border-box;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: var(--radius-md);
  }

  .pf-row.is-sel {
    background: var(--gradient-selected);
    border-color: var(--accent-line);
  }

  .pf-row.is-menu {
    z-index: 410;
  }

  .pf-tile {
    display: grid;
    place-items: center;
    width: 40px;
    height: 40px;
    box-sizing: border-box;
    border-radius: var(--radius-sm);
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    color: var(--ink-subtle);
  }

  .pf-row.is-sel .pf-tile {
    color: var(--accent-text);
    border-color: var(--accent-line);
  }

  .pf-main {
    display: grid;
    gap: 4px;
    min-width: 0;
  }

  .pf-nameline {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 4px 8px;
    min-width: 0;
  }

  .pf-name {
    font: var(--type-title-sm);
    color: var(--ink);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
  }

  .pf-mark {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    height: 20px;
    padding: 0 8px 0 6px;
    border-radius: var(--radius-xs);
    font: var(--type-caption-strong);
    white-space: nowrap;
    cursor: default;
  }

  .pf-mark :global(.dl-icon) {
    width: 12px;
    height: 12px;
  }

  .pf-mark--sel {
    color: var(--accent-text);
    background: var(--accent-soft);
  }

  .pf-mark--app {
    color: var(--success);
    background: var(--success-soft);
  }

  .pf-mark--pend {
    color: var(--warning);
    background: var(--warning-soft);
  }

  .pf-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
    font: var(--type-body-sm);
    color: var(--ink-muted);
    white-space: nowrap;
  }

  .pf-meta > span {
    flex: none;
  }

  .pf-meta > .pf-path {
    flex: 0 1 auto;
    min-width: 0;
    display: flex;
  }

  .pf-path :global(.dl-path__text) {
    max-width: none;
  }

  .pf-sep {
    color: var(--ink-disabled);
  }

  .pf-preview {
    font: var(--type-caption);
    color: var(--ink-subtle);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .pf-actions {
    position: relative;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .pf-menu {
    position: absolute;
    top: calc(100% + 4px);
    right: 0;
    z-index: 100;
    min-width: 248px;
  }

  .pf-menu .dl-option {
    width: 100%;
    border: 0;
    background: transparent;
    text-align: left;
    font: var(--type-body);
    color: var(--ink);
    white-space: nowrap;
  }

  .pf-menu .dl-option:hover {
    background: var(--surface-hover);
  }

  .pf-menu .dl-option:focus-visible {
    outline: none;
    background: var(--surface-hover);
    box-shadow: inset 0 0 0 2px var(--focus);
  }

  .pf-menu .dl-option.pf-danger {
    color: var(--danger);
  }

  .pf-menu .dl-option[aria-disabled='true'] {
    color: var(--ink-disabled);
    cursor: not-allowed;
    background: transparent;
  }

  .pf-catcher {
    position: fixed;
    inset: 0;
    z-index: 90;
  }

  .rename {
    max-width: 480px;
  }

  .rename-line {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
  }

  .rename-input {
    flex: 1 1 200px;
    min-width: 0;
  }
</style>
