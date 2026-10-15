<!-- Settings → Advanced: the settings and manifest files as text (50, 156: checked before saving, never saved when
     invalid, a dated backup each time) and the additional allowed domains (53). -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Plus from '@lucide/svelte/icons/plus';
  import Undo2 from '@lucide/svelte/icons/undo-2';
  import X from '@lucide/svelte/icons/x';
  import { onMount } from 'svelte';
  import Banner from '../../lib/components/Banner.svelte';
  import Button from '../../lib/components/Button.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import SegmentedControl from '../../lib/components/SegmentedControl.svelte';
  import { api, maintenance, type EditProblem, type EditRefused, type EditedFile } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { t } from '../../i18n';
  import Group from './Group.svelte';
  import { BUILT_IN_HOSTS, checkHost, excerpt, type HostProblem, type SettingsData, without } from './model';
  import { saveSettings } from './save';

  interface Props {
    app: AppState;
    data: SettingsData;
    folder: string;
  }

  const { app, data, folder }: Props = $props();

  let file = $state<EditedFile>('settings');
  /** The text on disk and the user's edits, per file. */
  let loaded = $state<Partial<Record<EditedFile, string>>>({});
  let edits = $state<Partial<Record<EditedFile, string>>>({});
  let refused = $state<Partial<Record<EditedFile, EditRefused>>>({});
  let saved = $state<Partial<Record<EditedFile, string | null>>>({});
  let saving = $state(false);
  let domain = $state('');
  let domainProblem: { problem: HostProblem; host: string } | null = $state(null);

  async function load(which: EditedFile): Promise<void> {
    try {
      loaded = { ...loaded, [which]: await maintenance.fileText(which) };
    } catch (error) {
      app.fail(error);
    }
  }

  onMount(() => {
    void load('settings');
  });

  const name = $derived(file === 'settings' ? 'settings.json' : 'manifest.json');
  const text = $derived(edits[file] ?? loaded[file] ?? '');
  const error = $derived(refused[file]);
  const fileOptions = $derived([
    { value: 'settings', label: t('settings-file-settings') },
    { value: 'manifest', label: t('settings-file-manifest') },
  ]);

  function problemText(p: EditProblem): string {
    switch (p.problem) {
      case 'unknown_field':
        return t('settings-problem-unknown-field', { path: p.path });
      case 'invalid_value':
        return t('settings-problem-invalid-value', { path: p.path });
      case 'out_of_range':
        return t('settings-problem-out-of-range', { path: p.path, found: p.found, min: p.min, max: p.max });
      case 'invalid_entry':
        return t('settings-problem-invalid-entry', { path: p.path, entry: p.entry });
      case 'duplicate_entry':
        return t('settings-problem-duplicate-entry', { path: p.path, entry: p.entry });
      case 'needs_age_confirmation':
        return t('settings-problem-needs-age', { path: p.path });
      case 'unavailable_in_local_mode':
        return t('settings-problem-local-mode', { path: p.path });
    }
  }

  function choose(which: string): void {
    file = which as EditedFile;
    if (loaded[file] === undefined) {
      void load(file);
    }
  }

  async function save(): Promise<void> {
    const which = file;
    saving = true;
    try {
      const result = await maintenance.saveFile(which, text);
      if ('Ok' in result) {
        refused = { ...refused, [which]: undefined };
        saved = { ...saved, [which]: result.Ok.backup };
        edits = without(edits, which);
        await load(which);
        if (which === 'settings') {
          app.settings = await api.settings();
        } else {
          await app.reloadLibrary();
        }
      } else {
        refused = { ...refused, [which]: result.Err };
        saved = { ...saved, [which]: undefined };
      }
    } catch (failure) {
      app.fail(failure);
    } finally {
      saving = false;
    }
  }

  function discard(): void {
    edits = without(edits, file);
    refused = { ...refused, [file]: undefined };
    saved = { ...saved, [file]: undefined };
  }

  function domainText(p: { problem: HostProblem; host: string }): string {
    switch (p.problem) {
      case 'empty':
        return t('settings-domain-empty');
      case 'not_a_host_only':
        return t('settings-domain-not-host');
      case 'invalid':
        return t('settings-domain-invalid', { host: p.host });
      case 'built_in':
        return t('settings-domain-built-in', { host: p.host });
      case 'listed':
        return t('settings-domain-listed', { host: p.host });
    }
  }

  async function addDomain(): Promise<void> {
    const checked = checkHost(domain, data.advanced.extra_allowed_domains);
    if (!checked.ok) {
      domainProblem = { problem: checked.problem, host: domain.trim().toLowerCase() };
      return;
    }
    if (await saveSettings(app, (s) => s.advanced.extra_allowed_domains.push(checked.host))) {
      domain = '';
      domainProblem = null;
    }
  }
</script>

<Banner kind="warning" title={t('settings-advanced-warning')} text={t('settings-advanced-warning-text')} />

<Group title={t('settings-files-group')}>
  <div class="st-row st-gap12">
    <div class="st-row__full st-filehead">
      <SegmentedControl
        label={t('settings-file-label')}
        options={fileOptions}
        value={file}
        onchange={(v) => {
          choose(v);
          return undefined;
        }}
      />
      {#if folder}<span class="st-meta st-pathline"><PathTooltip path={`${folder}\\${name}`} /></span>{/if}
    </div>
    <p class="st-row__help st-row__full">
      <Text value={file === 'settings' ? t('settings-file-settings-lead') : t('settings-file-manifest-lead')} />
    </p>
    <div class="dl-field st-row__full">
      <label class="dl-visually-hidden" for="st-json">{name}</label>
      <textarea
        id="st-json"
        class="st-code"
        spellcheck="false"
        rows="14"
        value={text}
        aria-invalid={error ? 'true' : 'false'}
        aria-describedby="st-json-msg"
        oninput={(event) => {
          edits = { ...edits, [file]: event.currentTarget.value };
          saved = { ...saved, [file]: undefined };
        }}
      ></textarea>
      <div id="st-json-msg" aria-live="polite">
        {#if error}
          <div class="st-err">
            {#if error.kind === 'json'}
              <span class="dl-field__error"
                ><CircleX class="dl-icon" /><span
                  ><strong>{t('settings-json-error')}</strong>
                  {t('settings-json-where', { line: error.line, column: error.column })}
                  <Text value={t('settings-json-fix')} /></span
                ></span
              >
              <div class="st-excerpt" aria-hidden="true">
                {#each excerpt(text, error.line) as line (line.n)}
                  <div class={line.error ? 'is-err' : ''}>
                    <span class="st-excerpt__n">{line.n}</span><span>{line.text || ' '}</span>
                  </div>
                {/each}
              </div>
            {:else if error.kind === 'problems'}
              <span class="dl-field__error"><CircleX class="dl-icon" /><strong>{t('settings-json-refused')}</strong></span>
              <ul class="st-offlist">
                {#each error.problems as problem (problem.path + problem.problem)}
                  <li>{problemText(problem)}</li>
                {/each}
              </ul>
            {:else if error.kind === 'newer_schema'}
              <span class="dl-field__error"
                ><CircleX class="dl-icon" /><span
                  ><Text value={t('settings-json-newer', { found: error.found, supported: error.supported })} /></span
                ></span
              >
            {:else if error.kind === 'invalid'}
              <span class="dl-field__error"><CircleX class="dl-icon" /><span><Text value={t('settings-json-invalid')} /></span></span>
              <div class="dl-dialog__details">
                <span>{t('settings-json-details')}</span><span class="dl-mono">{error.detail}</span>
              </div>
            {:else}
              <span class="dl-field__error"><CircleX class="dl-icon" /><span><Text value={t('settings-json-invalid')} /></span></span>
            {/if}
          </div>
        {:else if saved[file] !== undefined}
          <span class="st-stat st-stat--ok"
            ><CircleCheck class="dl-icon" />{saved[file]
              ? t('settings-json-saved', { backup: saved[file] ?? '' })
              : t('settings-json-saved-plain')}</span
          >
        {:else}
          <span class="st-meta">{t('settings-json-idle')}</span>
        {/if}
      </div>
    </div>
    <div class="st-actions st-row__full">
      <Button variant="primary" label={t('settings-json-save')} busy={saving} onclick={() => void save()} />
      <Button variant="ghost" icon={Undo2} label={t('settings-json-discard')} onclick={discard} />
      <span class="st-meta st-grow">{t('settings-json-backup')}</span>
    </div>
  </div>
</Group>

<Group title={t('settings-domains-group')} lead={t('settings-domains-lead')} titleId="st-domains">
  <div class="st-item st-banner-row st-wrap">
    <span class="st-meta">{t('settings-domains-built-in')}</span>
    {#each BUILT_IN_HOSTS as host (host)}<span class="dl-mono st-mono-ink">{host}</span>{/each}
  </div>
  {#each data.advanced.extra_allowed_domains as host (host)}
    <div class="st-item">
      <span class="dl-mono st-hostname">{host}</span>
      <span class="dl-badge dl-badge--outline">{t('settings-domain-user')}</span>
      <Button
        variant="ghost"
        size="sm"
        icon={X}
        label={t('settings-remove')}
        aria-label={t('settings-remove-item', { item: host })}
        onclick={() =>
          void saveSettings(app, (s) => {
            s.advanced.extra_allowed_domains = s.advanced.extra_allowed_domains.filter((h) => h !== host);
          })}
      />
    </div>
  {:else}
    <div class="st-item"><span class="st-meta">{t('settings-domains-none')}</span></div>
  {/each}
  <div class="st-item st-item--top st-wrap">
    <div class="dl-field st-grow">
      <label class="dl-field__label" for="st-domain">{t('settings-domain-add')}</label>
      <span class="dl-input dl-input--mono">
        <input
          id="st-domain"
          bind:value={domain}
          placeholder="files.example.com"
          aria-invalid={domainProblem ? 'true' : 'false'}
          aria-describedby="st-domain-msg"
          oninput={() => (domainProblem = null)}
          onkeydown={(event) => {
            if (event.key === 'Enter') {
              event.preventDefault();
              void addDomain();
            }
          }}
        />
      </span>
      <span id="st-domain-msg">
        {#if domainProblem}
          <span class="dl-field__error"><CircleX class="dl-icon" /><span>{domainText(domainProblem)}</span></span>
        {:else}
          <span class="dl-field__hint">{t('settings-domain-hint')}</span>
        {/if}
      </span>
    </div>
    <div class="st-add"><Button icon={Plus} label={t('settings-add')} onclick={() => void addDomain()} /></div>
  </div>
</Group>
