<!-- Dialog (design: Dialog.md): a modal decision. Render it while it is open ({#if}); it focuses its first control
     (or the one marked data-autofocus, e.g. Cancel in destructive dialogs), traps Tab, cancels on Esc and gives focus
     back to the opener when it closes. The scrim has no blur and does not close the dialog. -->
<script lang="ts">
  import X from '@lucide/svelte/icons/x';
  import { onDestroy, onMount, type Snippet } from 'svelte';
  import Button from './Button.svelte';
  import { initialFocus, trapTab } from './focus';
  import type { IconComponent } from './types';

  export type DialogKind = 'default' | 'danger';
  export type DialogSize = 'sm' | 'md' | 'lg';

  interface Props {
    /** The question or what happened, never just "Warning". */
    title: string;
    /** `danger` tints the head icon and defaults the role to alertdialog. */
    kind?: DialogKind;
    size?: DialogSize;
    role?: 'dialog' | 'alertdialog';
    /** Head icon (warning-tinted, or danger with `kind="danger"`). */
    icon?: IconComponent;
    /** Esc cancels; turn it off while an action cannot be interrupted (D8 while deleting). */
    closeOnEscape?: boolean;
    /** With a label, a close button in the head ("Close (Esc)") that cancels. */
    closeLabel?: string;
    oncancel: () => void;
    /** The body; it scrolls while head and foot stay. */
    children: Snippet;
    /** Footer buttons, the primary on the right. */
    footer?: Snippet;
    /** Secondary actions on the left of the footer (e.g. Open folder). */
    footerStart?: Snippet;
  }

  const {
    title,
    kind = 'default',
    size = 'sm',
    role,
    icon: Icon,
    closeOnEscape = true,
    closeLabel,
    oncancel,
    children,
    footer,
    footerStart,
  }: Props = $props();

  const uid = $props.id();
  const titleId = `${uid}-title`;
  const bodyId = `${uid}-body`;
  const opener = document.activeElement instanceof HTMLElement ? document.activeElement : null;
  let dialog: HTMLElement | undefined = $state();

  onMount(() => {
    if (dialog) {
      initialFocus(dialog).focus();
    }
  });

  onDestroy(() => {
    if (opener?.isConnected) {
      opener.focus();
    }
  });

  function handleKeydown(event: KeyboardEvent) {
    if (!dialog) {
      return;
    }
    if (event.key === 'Escape') {
      if (closeOnEscape && !event.defaultPrevented) {
        event.preventDefault();
        oncancel();
      }
      return;
    }
    trapTab(event, dialog);
  }
</script>

<div class="dl-scrim"></div>
<div class="layer">
  <div
    bind:this={dialog}
    class={['dl-dialog', size !== 'sm' && `dl-dialog--${size}`, kind === 'danger' && 'dl-dialog--danger']}
    role={role ?? (kind === 'danger' ? 'alertdialog' : 'dialog')}
    aria-modal="true"
    aria-labelledby={titleId}
    aria-describedby={bodyId}
    tabindex="-1"
    onkeydown={handleKeydown}
  >
    <div class="dl-dialog__head">
      {#if Icon}<Icon class="dl-icon dl-icon--md dl-dialog__icon" />{/if}
      <h2 class="dl-dialog__title" id={titleId}>{title}</h2>
      {#if closeLabel}
        <Button variant="ghost" size="sm" iconOnly icon={X} label={closeLabel} title={closeLabel} onclick={oncancel} />
      {/if}
    </div>
    <div class="dl-dialog__body" id={bodyId}>{@render children()}</div>
    {#if footer || footerStart}
      <div class="dl-dialog__foot">
        {#if footerStart}<div class="dl-dialog__foot-start">{@render footerStart()}</div>{/if}
        {@render footer?.()}
      </div>
    {/if}
  </div>
</div>

<style>
  /* Centres the dialog above the scrim, as in the design's screens. */
  .layer {
    position: fixed;
    inset: 0;
    z-index: var(--z-dialog);
    display: grid;
    place-items: center;
  }

  .dl-dialog:focus {
    outline: none;
  }
</style>
