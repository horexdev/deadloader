<!-- D13 (decision 146): Play with a profile switch that takes mods out of the game asks first. No "don't ask again". -->
<script lang="ts">
  import Layers from '@lucide/svelte/icons/layers';
  import PlayIcon from '@lucide/svelte/icons/play';
  import Button from '../lib/components/Button.svelte';
  import Dialog from '../lib/components/Dialog.svelte';
  import type { PlayConfirmation } from '../lib/app.svelte';
  import { formatList, t } from '../i18n';

  interface Props {
    confirmation: PlayConfirmation;
    onconfirm: () => void;
    oncancel: () => void;
  }

  const { confirmation, onconfirm, oncancel }: Props = $props();
</script>

<Dialog title={t('play-confirm-title', { profile: confirmation.profile })} icon={Layers} role="alertdialog" {oncancel}>
  <p>{t('play-confirm-off', { count: confirmation.turned_off.length })}</p>
  <ul class="mods">
    {#each confirmation.turned_off as name (name)}
      <li>{name}</li>
    {/each}
  </ul>
  {#if confirmation.turned_on.length > 0}
    <p>{t('play-confirm-on', { count: confirmation.turned_on.length, mods: formatList(confirmation.turned_on) })}</p>
  {/if}
  {#if confirmation.applied}
    <p>{t('play-confirm-keep', { applied: confirmation.applied })}</p>
  {/if}
  {#snippet footer()}
    <Button label={t('play-confirm-cancel')} onclick={oncancel} />
    <Button label={t('play-confirm-play')} variant="primary" icon={PlayIcon} onclick={onconfirm} />
  {/snippet}
</Dialog>

<style>
  .mods {
    display: grid;
    gap: 4px;
    margin: 0;
    padding: 8px 12px 8px 28px;
    color: var(--ink);
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: 6px;
  }
</style>
