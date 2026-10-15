<!-- ProgressBar (design: ProgressBar.md). The fill grows with transform: scaleX, never width. Update it at most four
     times a second. -->
<script lang="ts">
  export type ProgressState = 'active' | 'indeterminate' | 'paused' | 'error' | 'done';

  interface Props {
    /** Progress from 0 to 1 (clamped); 0 while queued. Ignored while indeterminate. */
    value: number;
    /** The accessible name, usually the file name. */
    label: string;
    state?: ProgressState;
    /** Spoken instead of the percentage, e.g. "5.2 of 8.4 MB". */
    valueText?: string;
    /** 4px bar without meta lines, inside library rows and the sidebar. */
    thin?: boolean;
    /** Top line: file name (mono) and stage word. */
    name?: string;
    stage?: string;
    /** Bottom line: amounts and percentage, then speed and time left. */
    detail?: string;
    /** Bold start of the bottom line, before `detail` (the applying banner's "Step 4 of 10"). */
    lead?: string;
    rate?: string;
  }

  const { value, label, state = 'active', valueText, thin = false, name, stage, detail, lead, rate }: Props = $props();

  const fraction = $derived(Number.isFinite(value) ? Math.min(1, Math.max(0, value)) : 0);
  const indeterminate = $derived(state === 'indeterminate');
</script>

<div
  class={['dl-progress', state !== 'active' && `dl-progress--${state}`, thin && 'dl-progress--thin']}
  role="progressbar"
  aria-label={label}
  aria-valuemin={0}
  aria-valuemax={100}
  aria-valuenow={indeterminate ? undefined : Math.round(fraction * 100)}
  aria-valuetext={indeterminate ? undefined : valueText}
>
  {#if !thin && (name || stage)}
    <div class="dl-progress__meta">
      {#if name}<strong class="dl-mono">{name}</strong>{/if}{#if stage}<span>{stage}</span>{/if}
    </div>
  {/if}
  <div class="dl-progress__track"><div class="dl-progress__fill" style:--value={fraction}></div></div>
  {#if !thin && (detail || lead || rate)}
    <div class="dl-progress__meta">
      {#if lead}<span><strong>{lead}</strong>{#if detail}{' · ' + detail}{/if}</span>{:else if detail}<span>{detail}</span>{/if}{#if rate}<span>{rate}</span>{/if}
    </div>
  {/if}
</div>
