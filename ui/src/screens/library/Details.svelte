<!-- Mod details (S4, board LibraryDetails): enabled + position, versions (old ones kept, 22/137/170), variants, the
     installed files with size and "from <archive>", linked or copied, local mods' name and GameBanana link (F10),
     conflicts, delete (D6) and the library folder. -->
<script lang="ts">
  import Check from '@lucide/svelte/icons/check';
  import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import Files from '@lucide/svelte/icons/files';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import GitCompareArrows from '@lucide/svelte/icons/git-compare-arrows';
  import Link from '@lucide/svelte/icons/link';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import X from '@lucide/svelte/icons/x';
  import type { LinkFailure, ModView } from '../../lib/api';
  import { Badge, Button, RowVariant, TextInput, Toggle } from '../../lib/components';
  import { formatSize, t } from '../../i18n';
  import { fileLabel, isLocal, parts, variantGroups, versionText, type Context } from './model';

  interface Props {
    mod: ModView;
    profile: string;
    position: number;
    total: number;
    ctx: Context;
    /** Names of the other mods, by id. */
    names: (id: string) => string;
    toggleDisabled?: boolean;
    ontoggle: (enabled: boolean) => void;
    onvariant: (file: string) => void;
    onclose: () => void;
    ondelete: (files: string[] | null) => void;
    onfolder: () => void;
    onmodpage: () => void;
    onconflicts: () => void;
    onrename: (name: string) => Promise<void>;
    /** F10: resolves to the number of archives matched, or a problem with the address. */
    onlink: (url: string) => Promise<LinkFailure | { matched: number; total: number }>;
    /** An update is available (D5). */
    update?: boolean;
    onupdate?: () => void;
  }

  const {
    mod,
    profile,
    position,
    total,
    ctx,
    names,
    toggleDisabled = false,
    ontoggle,
    onvariant,
    onclose,
    ondelete,
    onfolder,
    onmodpage,
    onconflicts,
    onrename,
    onlink,
    update = false,
    onupdate,
  }: Props = $props();

  const local = $derived(isLocal(mod));
  const split = $derived(parts(mod));
  const active = $derived(new Set([...split.main, ...split.addOns.filter(([, on]) => on).map(([f]) => f)]));
  /** Files kept as older versions: replaced by an update (137). */
  const older = $derived(mod.files.filter((f) => f.newer_in_library !== null && !active.has(f.id)));
  /** The active file is an older version and a newer one is in the library (170). */
  const newer = $derived(
    mod.files.find((f) => active.has(f.id) && f.newer_in_library !== null)?.newer_in_library ?? null,
  );
  const mainFile = $derived(mod.files.find((f) => f.id === split.main[0]));
  const conflicts = $derived(
    ctx.conflicts
      .filter((c) => c.winner_mod === mod.id || c.loser_mod === mod.id)
      .map((c) =>
        c.winner_mod === mod.id
          ? { wins: true, other: c.loser_mod, count: c.paths.length }
          : { wins: false, other: c.winner_mod, count: c.paths.length },
      ),
  );

  // svelte-ignore state_referenced_locally
  let name = $state(mod.name);
  let url = $state('');
  let linkError: string | undefined = $state();
  let linkNote: string | undefined = $state();
  let linking = $state(false);

  async function link(): Promise<void> {
    linking = true;
    linkError = undefined;
    linkNote = undefined;
    const result = await onlink(url.trim());
    linking = false;
    if ('matched' in result) {
      linkNote = t('library-link-done', { matched: result.matched, total: result.total });
    } else if ('kind' in result && result.kind === 'not_a_page') {
      linkError = t('library-link-not-a-page');
    } else if ('kind' in result && result.kind === 'removed') {
      linkError = t('library-link-removed');
    } else {
      linkError = t('library-link-failed');
    }
  }
</script>

<aside class="panel" aria-label={t('library-details')}>
  <div class="head">
    <div class="title">
      <h2>{mod.name}</h2>
      <span class="sub">{local ? t('library-local') : (mod.author ?? t('library-unknown-author'))} · {local ? t('library-from-disk') : t('library-gamebanana')}</span>
      {#if !local}
        <button type="button" class="dl-link link" onclick={onmodpage}>{t('library-open-mod-page')}</button>
      {/if}
    </div>
    <Button variant="ghost" size="sm" iconOnly icon={X} label={t('library-close-details')} onclick={onclose} />
  </div>
  <div class="body">
    <div class="row">
      <Toggle
        checked={mod.enabled}
        label={t('library-enabled-in', { profile })}
        disabled={toggleDisabled}
        title={toggleDisabled ? t('library-toggle-missing') : undefined}
        onchange={ontoggle}
      />
      <span class="caption">{t('library-position', { position, total })}</span>
    </div>

    <section class="section">
      <h3>{t('library-version')}</h3>
      <div class="version">
        <div class="stack">
          <strong>{versionText(mainFile?.version) ?? t('library-no-version')}</strong>
          <span class="caption">{local ? t('library-imported') : t('library-from-gamebanana')}</span>
        </div>
        <Badge icon={Check} label={t('library-active')} variant="accent" />
      </div>
      {#if update}
        <div class="actions">
          <span class="caption">{t('library-update-on-gb')}</span>
          <Button size="sm" icon={CircleArrowUp} label={t('library-update')} aria-haspopup="dialog" onclick={onupdate} />
        </div>
      {/if}
      {#if newer}
        <div class="actions">
          <span class="caption">{t('library-newer-in-library')}</span>
          <Button size="sm" label={t('library-switch-to')} onclick={() => onvariant(newer)} />
        </div>
      {/if}
      <div class="actions">
        <Button
          size="sm"
          label={t('library-delete-old')}
          unavailable={older.length === 0}
          title={older.length === 0 ? t('library-no-older') : undefined}
          onclick={() => older.length > 0 && ondelete(older.map((f) => f.id))}
        />
        <span class="caption">
          {older.length === 0
            ? t('library-no-older')
            : t('library-older', { count: older.length, size: formatSize(older.reduce((s, f) => s + f.len, 0)) })}
        </span>
      </div>
    </section>

    {#each variantGroups(mod) as group, g (g)}
      <div class="variants" role="radiogroup" aria-label={t('library-variants')}>
        <div class="dl-row-block__head">
          <span>{t('library-variants')}</span><span>{t('library-variants-downloaded', { count: group.files.length })}</span>
        </div>
        {#each group.files as id (id)}
          <RowVariant
            name={fileLabel(mod, id)}
            size={formatSize(mod.files.find((f) => f.id === id)?.len ?? 0)}
            group={`${mod.id}-panel-${g}`}
            checked={group.active === id}
            activeLabel={t('library-active')}
            onselect={() => onvariant(id)}
          />
        {/each}
      </div>
    {/each}

    <section class="section">
      <h3>{t('library-files')}</h3>
      {#each mod.files.filter((f) => active.has(f.id)) as f (f.id)}
        <div class="file">
          <div class="stack">
            <span class="dl-mono name">{f.inner_path.split('/').pop()}</span>
            <span class="dl-mono from">{t('library-from', { archive: f.archive_name })}</span>
          </div>
          <span class="caption">{t('library-on-disk', { size: formatSize(f.len) })}</span>
        </div>
      {/each}
      {#if mod.in_game}
        <span class="fit">
          <Badge
            icon={mod.in_game.copied ? Files : Link}
            label={mod.in_game.copied
              ? t('library-copied', { size: formatSize(mod.in_game.bytes) })
              : t('library-linked')}
          />
        </span>
      {:else}
        <span class="caption">{t('library-not-in-game')}</span>
      {/if}
    </section>

    {#if local}
      <section class="section">
        <h3>{t('library-local-mod')}</h3>
        <TextInput label={t('library-name')} bind:value={name} />
        <div class="actions">
          <Button
            size="sm"
            label={t('library-rename')}
            unavailable={name.trim() === '' || name.trim() === mod.name}
            onclick={() => void onrename(name.trim())}
          />
        </div>
        <TextInput
          label={t('library-link-label')}
          hint={linkNote ?? t('library-link-hint')}
          error={linkError}
          mono
          busy={linking}
          placeholder="https://gamebanana.com/mods/…"
          bind:value={url}
        />
        <div class="actions">
          <Button size="sm" icon={Link} label={t('library-link')} busy={linking} unavailable={url.trim() === ''} onclick={() => void link()} />
        </div>
        {#if mod.source.kind === 'local' && mod.source.imported_from}
          <span class="caption">{t('library-imported-from')} <span class="dl-mono">{mod.source.imported_from}</span></span>
        {/if}
      </section>
    {/if}

    <section class="section">
      <h3>{t('library-tab-conflicts')}</h3>
      {#if conflicts.length === 0}
        <span class="ok"><CircleCheck class="dl-icon" />{t('library-no-conflicts')}</span>
      {:else}
        {#each conflicts as c (c.other)}
          <span class="conflict">
            <GitCompareArrows class="dl-icon" />
            {c.wins
              ? t('library-conflict-wins', { name: names(c.other), count: c.count })
              : t('library-conflict-loses', { name: names(c.other), count: c.count })}
          </span>
        {/each}
        <span class="fit"><Button size="sm" label={t('library-show-conflicts')} onclick={onconflicts} /></span>
      {/if}
    </section>
  </div>
  <div class="foot">
    <Button icon={Trash2} label={t('library-delete-mod')} aria-haspopup="dialog" onclick={() => ondelete(null)} />
    <span class="push"></span>
    <Button variant="ghost" icon={FolderOpen} label={t('library-open-folder')} onclick={onfolder} />
  </div>
</aside>

<style>
  /* From the design's mod details panel (LibraryStates `details`). */
  .panel {
    flex: none;
    width: 400px;
    display: flex;
    flex-direction: column;
    background: var(--surface);
    border-left: 1px solid var(--line);
    overflow: hidden;
  }

  @media (max-width: 1099px) {
    .panel {
      position: absolute;
      top: 0;
      right: 0;
      bottom: 0;
      z-index: 40;
      width: 380px;
      box-shadow: var(--shadow-3);
    }
  }

  .head {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 16px 12px 14px 20px;
    border-bottom: 1px solid var(--line);
  }

  .title {
    flex: 1 1 auto;
    min-width: 0;
    display: grid;
    gap: 2px;
  }

  h2 {
    margin: 0;
    font: var(--type-title-sm);
    color: var(--ink);
  }

  h3 {
    margin: 0;
    font: var(--type-caption-strong);
    color: var(--ink-subtle);
  }

  .sub {
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .link {
    font: var(--type-body-sm);
    width: fit-content;
    padding: 0;
    background: none;
    border: 0;
    cursor: pointer;
  }

  .body {
    flex: 1 1 auto;
    overflow: auto;
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    align-content: start;
    gap: 18px;
    padding: 16px 20px;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .caption {
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .section {
    display: grid;
    gap: 8px;
  }

  .version {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    background: var(--accent-soft);
    border: 1px solid var(--accent-line);
    border-radius: 6px;
  }

  .stack {
    display: grid;
    gap: 2px;
    flex: 1 1 auto;
    min-width: 0;
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px 12px;
  }

  .variants {
    display: grid;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: 6px;
  }

  .file {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: 12px;
    align-items: center;
    padding: 8px;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: 6px;
  }

  .name {
    color: var(--ink);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .from {
    font: var(--type-mono-sm);
    color: var(--ink-subtle);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .fit {
    width: fit-content;
  }

  .ok,
  .conflict {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .ok :global(.dl-icon) {
    color: var(--success);
  }

  .conflict :global(.dl-icon) {
    color: var(--warning);
  }

  .foot {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 20px;
    border-top: 1px solid var(--line);
  }

  .push {
    margin-left: auto;
  }
</style>
