<!-- Toast (design: Toast.md): reports something that just happened without blocking. Neutral, success and info toasts
     leave after 6 s; warnings and errors stay until dismissed. The timer pauses while the toast is hovered or focused. -->
<script lang="ts">
  import Text from './Text.svelte';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Info from '@lucide/svelte/icons/info';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import X from '@lucide/svelte/icons/x';
  import { onDestroy, onMount } from 'svelte';
  import Button from './Button.svelte';
  import type { IconComponent } from './types';

  export type ToastKind = 'neutral' | 'success' | 'info' | 'warning' | 'error';

  export interface ToastAction {
    label: string;
    icon?: IconComponent;
    onclick: () => void;
  }

  export interface ToastDetails {
    label: string;
    /** Where the log entry is shown, e.g. the Activity route with the entry id. */
    href: string;
  }

  interface Props {
    kind?: ToastKind;
    /** What happened, one sentence. */
    title: string;
    /** Why, or what to do. */
    text?: string;
    /** A file the text names, after it in a monospace font ("Saved as D:\Shared\Evening.json"). */
    file?: string;
    /** Overrides the kind's icon (e.g. gamepad-2 for "Deadlock was closed"). */
    icon?: IconComponent;
    /** At most one action, e.g. Undo. */
    action?: ToastAction;
    details?: ToastDetails;
    /** Accessible name of the dismiss button. */
    dismissLabel: string;
    /** Called on the dismiss button and when the lifetime ends. */
    ondismiss: () => void;
    /** Lifetime in ms; null keeps it until dismissed. Defaults by kind (6 s, or null for warnings and errors). */
    duration?: number | null;
  }

  const {
    kind = 'neutral',
    title,
    text,
    file,
    icon,
    action,
    details,
    dismissLabel,
    ondismiss,
    duration,
  }: Props = $props();

  const defaults: Record<ToastKind, IconComponent> = {
    neutral: Info,
    success: CircleCheck,
    info: Info,
    warning: TriangleAlert,
    error: CircleX,
  };
  const Icon = $derived(icon ?? defaults[kind]);
  const lifetime = $derived(duration !== undefined ? duration : kind === 'warning' || kind === 'error' ? null : 6000);

  let timer: ReturnType<typeof setTimeout> | undefined;
  let remaining = 0;
  let startedAt = 0;
  let hovered = false;
  let focused = false;

  function start() {
    if (lifetime === null || timer !== undefined || hovered || focused) {
      return;
    }
    startedAt = Date.now();
    timer = setTimeout(() => {
      timer = undefined;
      ondismiss();
    }, remaining);
  }

  function pause() {
    if (timer === undefined) {
      return;
    }
    clearTimeout(timer);
    timer = undefined;
    remaining = Math.max(0, remaining - (Date.now() - startedAt));
  }

  onMount(() => {
    remaining = lifetime ?? 0;
    start();
  });

  onDestroy(pause);
</script>

<div
  class={['dl-toast', kind !== 'neutral' && `dl-toast--${kind}`]}
  role={kind === 'error' ? 'alert' : 'status'}
  onpointerenter={() => {
    hovered = true;
    pause();
  }}
  onpointerleave={() => {
    hovered = false;
    start();
  }}
  onfocusin={() => {
    focused = true;
    pause();
  }}
  onfocusout={() => {
    focused = false;
    start();
  }}
>
  <Icon class="dl-icon dl-icon--md dl-toast__icon" />
  <div class="dl-toast__body">
    <div class="dl-toast__title"><Text value={title} /></div>
    {#if text}<div class="dl-toast__text"><Text value={file ? `${text} ` : text} />{#if file}<span class="dl-mono file">{file}</span>{/if}</div>{/if}
    {#if action || details}
      <div class="dl-toast__actions">
        {#if action}<Button size="sm" label={action.label} icon={action.icon} onclick={action.onclick} />{/if}
        {#if details}<a class="dl-link" href={details.href}>{details.label}</a>{/if}
      </div>
    {/if}
  </div>
  <Button variant="ghost" size="sm" iconOnly icon={X} label={dismissLabel} title={dismissLabel} onclick={ondismiss} />
</div>

<style>
  /* From ProfilesScreen.dc.html: the exported file in the toast's text. */
  .file {
    color: var(--ink);
    overflow-wrap: anywhere;
  }
</style>
