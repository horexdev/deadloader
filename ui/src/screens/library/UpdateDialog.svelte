<!-- D5 Update a mod (decisions 39, 137, 164, 169; flow F8; board DialogD5): installed vs new, the files that replace
     the installed ones (picked by name and size before anything is downloaded), Install / Later. When an update
     arrived without a clear partner for every old file, the same dialog asks the user to map them. -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import { tooltip } from '../../lib/components/tooltip';
  import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
  import Download from '@lucide/svelte/icons/download';
  import type { ModView, Proposed, RemoteModFile, UpdatePick, UpdateStatus, UpdateStep } from '../../lib/api';
  import { Button, Dialog, Select } from '../../lib/components';
  import { formatDate, formatSize, t } from '../../i18n';
  import { fileLabel, modVersion, versionText } from './model';

  type Available = Extract<UpdateStatus, { kind: 'update_available' }>;
  type Mapping = Extract<UpdateStep, { kind: 'needs_mapping' }>;

  interface Props {
    mod: ModView;
    /** Pick the files to download (from the update check)… */
    status?: Available | null;
    /** …or map an update that arrived (169). */
    step?: Mapping | null;
    busy?: boolean;
    oninstall: (picks: UpdatePick[]) => void;
    onmap: (mappings: { old: string; new: string | null }[], version: string | null) => void;
    oncancel: () => void;
  }

  const { mod, status = null, step = null, busy = false, oninstall, onmap, oncancel }: Props = $props();

  const KEEP = 'keep';

  const listed: RemoteModFile[] = $derived(
    status
      ? [
          ...status.replaced.flatMap((r) => (r.candidate ? [r.candidate.file] : [])),
          ...status.others,
        ].filter((f, i, all) => all.findIndex((x) => x.id === f.id) === i)
      : [],
  );

  // svelte-ignore state_referenced_locally
  let picks: Record<number, string> = $state(
    Object.fromEntries((status?.replaced ?? []).map((r) => [r.remote_file_id, r.candidate ? String(r.candidate.file.id) : KEEP])),
  );

  function proposed(p: Proposed): string {
    return p.kind === 'pair' || p.kind === 'overlap' ? p.new : p.kind === 'tie' ? '' : KEEP;
  }

  // svelte-ignore state_referenced_locally
  let mapped: Record<string, string> = $state(Object.fromEntries((step?.proposals ?? []).map((p) => [p.old, proposed(p.proposed)])));

  const installedBytes = $derived(mod.files.filter((f) => mod.add_ons.some(([id, on]) => on && id === f.id) || mod.variant_groups.some((g) => g.active === f.id)).reduce((s, f) => s + f.len, 0));
  const downloadBytes = $derived(
    listed.filter((f) => Object.values(picks).includes(String(f.id))).reduce((s, f) => s + f.size_bytes, 0),
  );
  const newVersion = $derived(versionText(status?.version ?? step?.version ?? null));
  const changedSet = $derived(
    status !== null &&
      (status.others.length > 0 ||
        status.replaced.some((r) => !r.candidate || r.candidate.file.file_name.toLowerCase() !== r.archive_name.toLowerCase())),
  );
  const unmapped = $derived(Object.values(mapped).some((v) => v === ''));

  function size(f: RemoteModFile): string {
    return `${f.file_name} · ${formatSize(f.size_bytes)}`;
  }
</script>

<Dialog title={t('library-d5-title', { name: mod.name })} size="md" icon={CircleArrowUp} {oncancel}>
  <div class="versions">
    <div class="card">
      <span class="caption">{t('library-d5-installed')}</span>
      <strong>{modVersion(mod) ?? t('library-no-version')}</strong>
      <span class="meta">{formatSize(installedBytes)}</span>
    </div>
    <span class="arrow" aria-hidden="true">→</span>
    <div class="card new">
      <span class="caption">{step ? t('library-d5-downloaded') : t('library-d5-new')}</span>
      <strong>{newVersion ?? t('library-no-version')}</strong>
      <span class="meta">
        {#if status}
          {status.date_updated
            ? t('library-d5-new-meta', { date: formatDate(status.date_updated * (status.date_updated < 1e12 ? 1000 : 1)), size: formatSize(downloadBytes) })
            : t('library-d5-download', { size: formatSize(downloadBytes) })}
        {:else}
          {t('library-d5-arrived')}
        {/if}
      </span>
    </div>
  </div>

  {#if status}
    <p><Text value={changedSet ? t('library-d5-changed') : t('library-d5-same')} /></p>
    <div class="table" role="table" aria-label={t('library-d5-map-label')}>
      <div class="row header" role="row">
        <span role="columnheader">{t('library-d5-col-old')}</span><span></span><span role="columnheader">{t('library-d5-col-new')}</span>
      </div>
      {#each status.replaced as r (r.remote_file_id)}
        <div class="row" role="row">
          <span role="cell" class="dl-mono old" {@attach tooltip({ text: r.archive_name ?? '' })}>{r.archive_name}</span>
          <span aria-hidden="true" class="arrow">→</span>
          <span role="cell" class="cell">
            <Select
              label={t('library-d5-pick', { file: r.archive_name })}
              options={[
                ...listed.map((f) => ({ value: String(f.id), label: size(f) })),
                { value: KEEP, label: t('library-d5-keep') },
              ]}
              value={picks[r.remote_file_id] ?? KEEP}
              onchange={(v) => (picks = { ...picks, [r.remote_file_id]: v })}
            />
            {#if r.candidate && picks[r.remote_file_id] === String(r.candidate.file.id)}
              <span class="dl-badge dl-badge--outline tag">
                {r.candidate.file.file_name.toLowerCase() === r.archive_name.toLowerCase() ? t('library-d5-tag-same') : t('library-d5-tag-replaces')}
              </span>
            {/if}
          </span>
        </div>
      {/each}
      {#each status.others.filter((f) => !Object.values(picks).includes(String(f.id))) as f (f.id)}
        <div class="row" role="row">
          <span role="cell" class="dl-mono none">—</span>
          <span aria-hidden="true" class="arrow">→</span>
          <span role="cell" class="cell">
            <span class="dl-mono new-file" {@attach tooltip({ text: f.file_name ?? '' })}>{f.file_name}</span>
            <span class="dl-badge dl-badge--info tag">{t('library-d5-tag-new')}</span>
          </span>
        </div>
      {/each}
    </div>
    <p class="caption">{t('library-d5-profiles', { old: modVersion(mod) ?? '', new: newVersion ?? '' })}</p>
  {:else if step}
    <p><Text value={t('library-d5-map-text')} /></p>
    <div class="table" role="table" aria-label={t('library-d5-map-label')}>
      <div class="row header" role="row">
        <span role="columnheader">{t('library-d5-col-old')}</span><span></span><span role="columnheader">{t('library-d5-col-new')}</span>
      </div>
      {#each step.proposals as p (p.old)}
        {@const options = p.proposed.kind === 'tie' ? p.proposed.candidates : step.new_files}
        <div class="row" role="row">
          <span role="cell" class="dl-mono old" {@attach tooltip({ text: fileLabel(mod, p.old) ?? '' })}>{fileLabel(mod, p.old)}</span>
          <span aria-hidden="true" class="arrow">→</span>
          <span role="cell" class="cell">
            <Select
              label={t('library-d5-pick', { file: fileLabel(mod, p.old) })}
              placeholder={t('library-d5-choose')}
              options={[
                ...options.map((id) => ({ value: id, label: fileLabel(mod, id) })),
                { value: KEEP, label: t('library-d5-keep') },
              ]}
              value={mapped[p.old] ?? ''}
              onchange={(v) => (mapped = { ...mapped, [p.old]: v })}
            />
          </span>
        </div>
      {/each}
    </div>
  {/if}

  {#snippet footer()}
    <Button label={t('library-later')} onclick={oncancel} />
    {#if status}
      <Button
        variant="primary"
        icon={Download}
        label={t('library-d5-install')}
        {busy}
        onclick={() =>
          oninstall(
            status.replaced.map((r) => ({
              replaced: r.remote_file_id,
              file: picks[r.remote_file_id] === KEEP ? null : Number(picks[r.remote_file_id]),
            })),
          )}
      />
    {:else if step}
      <Button
        variant="primary"
        label={t('library-d5-confirm-map')}
        {busy}
        unavailable={unmapped}
        onclick={() =>
          onmap(
            step.proposals.map((p) => ({ old: p.old, new: mapped[p.old] === KEEP ? null : (mapped[p.old] ?? null) })),
            step.version,
          )}
      />
    {/if}
  {/snippet}
</Dialog>

<style>
  /* From the design's D5. */
  p {
    margin: 0;
  }

  .versions {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto minmax(0, 1fr);
    gap: 12px;
    align-items: stretch;
  }

  .card {
    display: grid;
    gap: 2px;
    padding: 10px 12px;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: 6px;
  }

  .card.new {
    background: var(--accent-soft);
    border-color: var(--accent-line);
  }

  .card strong {
    font: var(--type-body-strong);
    color: var(--ink);
  }

  .caption {
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  p.caption {
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .meta {
    font: var(--type-caption);
    color: var(--ink-muted);
  }

  .arrow {
    align-self: center;
    color: var(--ink-subtle);
  }

  .table {
    display: grid;
    border: 1px solid var(--line);
    border-radius: 6px;
    overflow: hidden;
  }

  .row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 16px minmax(0, 1.4fr);
    gap: 8px;
    align-items: center;
    min-height: 36px;
    padding: 4px 10px;
    border-top: 1px solid var(--line);
  }

  .header {
    padding: 6px 10px;
    background: var(--surface);
    border-top: 0;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .old {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--ink-muted);
  }

  .none {
    color: var(--ink-subtle);
  }

  .cell {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
  }

  .new-file {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--ink);
  }

  .tag {
    flex: none;
  }
</style>
