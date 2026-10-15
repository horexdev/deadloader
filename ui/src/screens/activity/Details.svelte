<!-- Activity's entry details (design: ActivityDetails, ActivityBlocked): what happened and why for a failure, the hops
     of a request, the full record, the note that the user name is hidden, Copy and "open logs folder". -->
<script lang="ts">
  import { tooltip } from '../../lib/components/tooltip';
  import Check from '@lucide/svelte/icons/check';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Copy from '@lucide/svelte/icons/copy';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import Info from '@lucide/svelte/icons/info';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import X from '@lucide/svelte/icons/x';
  import { onDestroy } from 'svelte';
  import type { LogEntry } from '../../lib/api';
  import { describe, t } from '../../i18n';
  import {
    changeText,
    clock,
    durationText,
    hopStatusText,
    initiatorLabel,
    instant,
    levelLabel,
    opLabel,
    purposeLabel,
    sizeText,
    sourceLabel,
    sourceLong,
    statusText,
  } from './labels';
  import { copyText, durationOf, entryProblem, hopCounts, hopStatus, hostPath, sizeOf } from './model';

  interface Props {
    entry: LogEntry;
    /** "Hide the Windows user name" is on: the note says so. */
    masking: boolean;
    /** Over the list instead of beside it (narrow windows). */
    over: boolean;
    onclose: () => void;
    onopenlogs: () => void;
    /** Opens Settings (→ Advanced or → Logs). */
    onsettings: () => void;
  }

  const { entry, masking, over, onclose, onopenlogs, onsettings }: Props = $props();

  let copied = $state(false);
  let copiedTimer: ReturnType<typeof setTimeout> | undefined;

  const problem = $derived(entryProblem(entry));
  const text = $derived(problem ? describe(problem) : null);
  /** Temporary trouble deadloader works around is a warning; the rest is an error. */
  const warnOnly = $derived(problem !== null && ['E5', 'E19'].includes(problem.id));
  const tone = $derived.by(() => {
    if (entry.kind === 'app') {
      return entry.record.lvl === 'error' ? 'err' : entry.record.lvl === 'warn' ? 'warn' : 'info';
    }
    return problem ? (warnOnly ? 'warn' : 'err') : 'ok';
  });
  const settingsFix = $derived(problem?.id === 'E16' && problem.args['reason'] === 'not_listed');

  interface Field {
    label: string;
    value: string;
    mono?: boolean;
    note?: string;
    url?: { pre: string; host: string; rest: string };
    pre?: boolean;
  }

  const head = $derived.by(() => {
    switch (entry.kind) {
      case 'network': {
        const first = entry.lines[0];
        return {
          title: `${first ? purposeLabel(first.purpose) : ''} · ${statusText(entry)}`,
          sub: `${t('activity-kind-network')} · ${first ? clock(first.ts) : ''}`,
        };
      }
      case 'files':
        return {
          title: `${opLabel(entry.record.op)} · ${entry.record.result === 'ok' ? t('activity-ok') : t('activity-lv-error')}`,
          sub: `${t('activity-kind-files')} · ${clock(entry.record.ts)}`,
        };
      case 'app':
        return { title: levelLabel(entry.record.lvl), sub: `${t('activity-kind-app')} · ${clock(entry.record.ts)}` };
    }
  });

  const fields = $derived.by((): Field[] => {
    switch (entry.kind) {
      case 'network': {
        const first = entry.lines[0];
        const last = entry.lines.at(-1);
        if (!first || !last) {
          return [];
        }
        const { host, path } = hostPath(first.url);
        const redirects = hopCounts(entry).redirects;
        const out: Field[] = [
          { label: t('activity-d-time'), value: instant(first.ts) },
          { label: t('activity-d-purpose'), value: purposeLabel(first.purpose) },
          { label: t('activity-d-started-by'), value: initiatorLabel(first.initiator) },
          { label: t('activity-d-request'), value: '', url: { pre: `${first.method} ${first.url.split('://')[0] ?? 'https'}://`, host, rest: path } },
          { label: t('activity-d-source'), value: sourceLong(first.host_source) },
          {
            label: t('activity-d-status'),
            value:
              redirects > 0
                ? t('activity-status-after', { status: hopStatusText(last, true), count: redirects })
                : hopStatusText(last, true),
          },
          { label: t('activity-d-size'), value: sizeText(sizeOf(entry)) },
          { label: t('activity-d-duration'), value: durationText(durationOf(entry)) },
        ];
        if (last.range) {
          out.push({ label: t('activity-d-range'), value: last.range, mono: true });
        }
        if (last.error) {
          out.push({ label: t('activity-d-error'), value: last.error, mono: true });
        }
        out.push({ label: t('activity-d-request-id'), value: entry.req, mono: true });
        out.push({ label: t('activity-d-id'), value: last.id, mono: true });
        return out;
      }
      case 'files': {
        const r = entry.record;
        const out: Field[] = [
          { label: t('activity-d-time'), value: instant(r.ts) },
          { label: t('activity-d-operation'), value: opLabel(r.op), note: r.msg },
        ];
        if (r.src) {
          out.push({ label: r.dst ? t('activity-d-from') : t('activity-d-file'), value: r.src, mono: true });
        }
        if (r.dst) {
          out.push({ label: r.src ? t('activity-d-to') : t('activity-d-file'), value: r.dst, mono: true });
        }
        out.push({ label: t('activity-d-size'), value: sizeText(r.bytes) });
        out.push({ label: t('activity-d-result'), value: r.result === 'ok' ? t('activity-ok') : t('activity-lv-error') });
        if (r.error) {
          out.push({ label: t('activity-d-error'), value: r.error, mono: true });
        }
        // The change in words, with its number to find its other operations (186).
        out.push({
          label: t('activity-d-change'),
          value: r.txn === undefined ? '—' : r.change ? `${changeText(r)} · ${r.txn}` : String(r.txn),
          mono: r.txn !== undefined && !r.change,
        });
        if (r.sha256) {
          out.push({ label: t('activity-d-sha256'), value: r.sha256, mono: true });
        }
        out.push({ label: t('activity-d-id'), value: r.id, mono: true });
        return out;
      }
      case 'app': {
        const r = entry.record;
        const out: Field[] = [
          { label: t('activity-d-time'), value: instant(r.ts) },
          { label: t('activity-d-level'), value: levelLabel(r.lvl) },
          { label: t('activity-d-message'), value: r.msg },
        ];
        if (r.fields && Object.keys(r.fields).length > 0) {
          out.push({ label: t('activity-d-fields'), value: JSON.stringify(r.fields, null, 2), pre: true });
        }
        out.push({ label: t('activity-d-id'), value: r.id, mono: true });
        return out;
      }
    }
  });

  const hops = $derived(entry.kind === 'network' && entry.lines.length > 1 ? entry.lines : []);

  async function copy() {
    try {
      await navigator.clipboard.writeText(copyText(entry));
      copied = true;
      clearTimeout(copiedTimer);
      copiedTimer = setTimeout(() => (copied = false), 1800);
    } catch {
      copied = false;
    }
  }

  onDestroy(() => clearTimeout(copiedTimer));
</script>

<aside class={['act-aside', over && 'act-aside--over']} aria-label={t('activity-details')}>
  <div class="act-aside__head">
    <span
      class={[
        'act-aside__icon',
        tone === 'err' && 'act-st--err',
        tone === 'warn' && 'act-st--warn',
        tone === 'info' && 'act-lv--info',
      ]}
    >
      {#if tone === 'err'}<CircleX class="dl-icon dl-icon--md" />{:else if tone === 'warn'}<TriangleAlert
          class="dl-icon dl-icon--md"
        />{:else if tone === 'info'}<Info class="dl-icon dl-icon--md" />{:else}<CircleCheck
          class="dl-icon dl-icon--md"
        />{/if}
    </span>
    <div class="act-aside__titles">
      <h2 class="act-aside__title">{head.title}</h2>
      <span class="act-aside__sub">{head.sub}</span>
    </div>
    <button
      type="button"
      class="dl-btn dl-btn--ghost dl-btn--icon dl-btn--sm"
      aria-label={t('activity-close-details')}
      {@attach tooltip({ text: t('activity-close-esc') ?? '' })}
      onclick={onclose}><X class="dl-icon" /></button
    >
  </div>
  <div class="act-aside__body">
    {#if text}
      <div class={['act-err', warnOnly && 'act-err--warn']} role="alert">
        <h3>{text.what}</h3>
        {#if text.why}<p><b>{t('activity-why')}</b> {text.why}</p>{/if}
        {#if text.todo}<p><b>{t('activity-todo')}</b> {text.todo}</p>{/if}
        {#if settingsFix}
          <button type="button" class="dl-btn dl-btn--sm" onclick={onsettings}>{t('activity-open-advanced')}</button>
        {/if}
      </div>
    {/if}
    {#if hops.length > 0}
      <section class="act-section">
        <h3 class="act-sec">{t('activity-hops-title', { count: hopCounts(entry).redirects })}</h3>
        <div class="act-hopl">
          {#each hops as line, i (line.id)}
            {@const parts = hostPath(line.url)}
            {@const status = hopStatus(line)}
            <div class="act-hopl__i">
              <span class="act-hopl__n">{i + 1}</span>
              <span class="act-hopl__u">{line.method} <b>{parts.host}</b>{parts.path}</span>
              <span class="act-hopl__s"
                ><span class={['act-st', status.kind !== 'ok' && 'act-st--err']}
                  >{#if status.kind !== 'ok'}<CircleX class="dl-icon" />{/if}{hopStatusText(line, true)}</span
                ><span
                  >{status.kind === 'blocked'
                    ? sourceLong(null)
                    : `${sizeText(line.bytes)} · ${durationText(line.ms)} · ${sourceLabel(line.host_source)}`}</span
                ></span
              >
            </div>
          {/each}
        </div>
      </section>
    {/if}
    <section class="act-section">
      <h3 class="act-sec">{t('activity-full-record')}</h3>
      <dl class="act-dl">
        {#each fields as f (f.label)}
          <dt>{f.label}</dt>
          <dd>
            {#if f.url}<span class="act-mono act-mono--muted">{f.url.pre}<b>{f.url.host}</b>{f.url.rest}</span>
            {:else if f.pre}<pre class="act-pre">{f.value}</pre>
            {:else if f.mono}<span class="act-mono">{f.value}</span>
            {:else}{f.value}{/if}
            {#if f.note}<small>{f.note}</small>{/if}
          </dd>
        {/each}
      </dl>
    </section>
    {#if masking}
      <p class="act-note">
        <Info class="dl-icon" /><span
          >{t('activity-user-hidden')} <span class="act-mono">C:\Users\…\</span>.
          <button type="button" class="act-linkbtn" onclick={onsettings}>{t('activity-settings-logs')}</button></span
        >
      </p>
    {/if}
  </div>
  <div class="act-aside__foot">
    <button type="button" class="dl-btn" aria-live="polite" onclick={() => void copy()}>
      {#if copied}<Check class="dl-icon act-ok-icon" />{t('activity-copied')}{:else}<Copy class="dl-icon" />{t(
          'activity-copy',
        )}{/if}
    </button>
    <button type="button" class="dl-btn dl-btn--ghost" onclick={onopenlogs}
      ><FolderOpen class="dl-icon" />{t('activity-open-logs')}</button
    >
  </div>
</aside>
