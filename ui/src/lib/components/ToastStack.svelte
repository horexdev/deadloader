<!-- The toast stack (design: Toast.md): bottom-right, 16px from the edges, newest at the bottom, at most three
     visible; older ones live on in the recent-notifications list. -->
<script lang="ts">
  import Toast, { type ToastAction, type ToastDetails, type ToastKind } from './Toast.svelte';
  import type { IconComponent } from './types';

  export interface ToastData {
    id: string;
    kind?: ToastKind;
    title: string;
    text?: string;
    icon?: IconComponent;
    action?: ToastAction;
    details?: ToastDetails;
    duration?: number | null;
  }

  interface Props {
    /** Oldest first. */
    toasts: ToastData[];
    dismissLabel: string;
    ondismiss: (id: string) => void;
    max?: number;
  }

  const { toasts, dismissLabel, ondismiss, max = 3 }: Props = $props();

  const shown = $derived(toasts.slice(-max));
</script>

<div class="dl-toast-stack stack">
  {#each shown as toast (toast.id)}
    <Toast
      kind={toast.kind}
      title={toast.title}
      text={toast.text}
      icon={toast.icon}
      action={toast.action}
      details={toast.details}
      duration={toast.duration}
      {dismissLabel}
      ondismiss={() => ondismiss(toast.id)}
    />
  {/each}
</div>

<style>
  /* Placement from the design's screens. */
  .stack {
    position: fixed;
    right: var(--space-4);
    bottom: var(--space-4);
    z-index: var(--z-toast);
  }
</style>
