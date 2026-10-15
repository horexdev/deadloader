<!-- ErrorDialog, the D12 pattern (design: DownloadsScreen, board DownloadsD12): the title says what happened, the body
     why and what to do, then Details with the log entry (or the technical detail) and Copy, and the actions that fit
     the error. Opened by the user (e.g. "Show details" on a failed download), never by itself (decision 155). -->
<script lang="ts" module>
  import type { IconComponent } from './types';

  export interface ErrorAction {
    label: string;
    icon?: IconComponent;
    /** `primary` is last and focused; `secondary` (default) sits before it; `start` is a ghost at the left. */
    place?: 'primary' | 'secondary' | 'start';
    /** A link instead of a button, e.g. to Settings → Downloads. */
    href?: string;
    /** Runs before the dialog closes. */
    run?: () => void;
  }

  export interface ErrorDialogLabels {
    close: string;
    /** The head's close button, e.g. "Close (Esc)". */
    closeEsc: string;
    /** "Details" before a technical detail. */
    details: string;
    /** "Details · log entry" before a log entry id. */
    detailsLog: string;
    copy: string;
    copied: string;
    openActivity: string;
  }
</script>

<script lang="ts">
  import Text from './Text.svelte';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Copy from '@lucide/svelte/icons/copy';
  import { onDestroy, type Snippet } from 'svelte';
  import Button from './Button.svelte';
  import Dialog from './Dialog.svelte';

  interface Props {
    /** What happened: the title. */
    what: string;
    why: string;
    todo: string;
    /** The log entry of the failure, when known. */
    logId?: string | null;
    /** The technical detail, shown when there is no log entry. */
    detail?: string | null;
    actions?: ErrorAction[];
    labels: ErrorDialogLabels;
    /** Where the log entry is shown. */
    activityHref?: string;
    onclose: () => void;
    /** Extra content between why and what to do (e.g. the limits table of E9). */
    children?: Snippet;
  }

  const { what, why, todo, logId, detail, actions = [], labels, activityHref, onclose, children }: Props = $props();

  const COPIED_MS = 2400;
  const copyText = $derived(logId || detail || '');
  const start = $derived(actions.filter((a) => a.place === 'start'));
  const secondary = $derived(actions.filter((a) => (a.place ?? 'secondary') === 'secondary'));
  const primary = $derived(actions.filter((a) => a.place === 'primary'));
  let copied = $state(false);
  let timer: ReturnType<typeof setTimeout> | undefined;

  onDestroy(() => clearTimeout(timer));

  async function copy() {
    try {
      await navigator.clipboard.writeText(copyText);
    } catch {
      return;
    }
    copied = true;
    clearTimeout(timer);
    timer = setTimeout(() => (copied = false), COPIED_MS);
  }

  function choose(action: ErrorAction) {
    action.run?.();
    onclose();
  }
</script>

{#snippet button(action: ErrorAction, variant: 'primary' | 'secondary' | 'ghost')}
  {#if action.href}
    {@const Icon = action.icon}
    <a
      class={['dl-btn', variant !== 'secondary' && `dl-btn--${variant}`, 'link']}
      href={action.href}
      data-autofocus={variant === 'primary' ? '' : undefined}
      onclick={() => onclose()}>{#if Icon}<Icon class="dl-icon" />{/if}<span class="dl-btn__label">{action.label}</span></a
    >
  {:else}
    <Button
      {variant}
      label={action.label}
      icon={action.icon}
      data-autofocus={variant === 'primary' ? '' : undefined}
      onclick={() => choose(action)}
    />
  {/if}
{/snippet}

{#snippet footerStart()}
  {#each start as action (action.label)}{@render button(action, 'ghost')}{/each}
{/snippet}
{#snippet footer()}
  <Button label={labels.close} onclick={onclose} />
  {#each secondary as action (action.label)}{@render button(action, 'secondary')}{/each}
  {#each primary as action (action.label)}{@render button(action, 'primary')}{/each}
{/snippet}

<Dialog
  title={what}
  kind="danger"
  size="md"
  icon={CircleX}
  closeLabel={labels.closeEsc}
  oncancel={onclose}
  {footer}
  footerStart={start.length > 0 ? footerStart : undefined}
>
  <p><Text value={why} /></p>
  {@render children?.()}
  <p class="todo"><Text value={todo} /></p>
  {#if copyText || activityHref}
    <div class="dl-dialog__details details">
      {#if copyText}
        <span>{logId ? labels.detailsLog : labels.details}</span><span class="dl-mono value">{copyText}</span>
        <Button variant="ghost" size="sm" icon={Copy} label={labels.copy} onclick={() => void copy()} />
        {#if copied}<span class="copied" role="status">{labels.copied}</span>{/if}
      {/if}
      {#if activityHref}<a class="dl-link" href={activityHref} onclick={() => onclose()}>{labels.openActivity}</a>{/if}
    </div>
  {/if}
</Dialog>

<style>
  /* Small layout details from DownloadsScreen.dc.html (D12). */
  .todo {
    color: var(--ink);
  }

  .details {
    flex-wrap: wrap;
  }

  .value {
    color: var(--ink-muted);
    overflow-wrap: anywhere;
  }

  .copied {
    color: var(--success);
  }

  .link {
    text-decoration: none;
  }
</style>
