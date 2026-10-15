<!-- Delete the applied profile (board ProfilesDelete): its mods leave the game folder, they stay in the library;
     another profile becomes selected; "Export first" keeps a copy. -->
<script lang="ts">
  import FileOutput from '@lucide/svelte/icons/file-output';
  import Trash from '@lucide/svelte/icons/trash';
  import Button from '../../lib/components/Button.svelte';
  import Dialog from '../../lib/components/Dialog.svelte';
  import type { ProfileOverview } from '../../lib/api';
  import { t } from '../../i18n';

  interface Props {
    profile: ProfileOverview;
    root: string | null;
    /** The profile selected afterwards, when this one is the selected one. */
    next: string | null;
    onexport: () => void;
    ondelete: () => void;
    oncancel: () => void;
  }

  const { profile, root, next, onexport, ondelete, oncancel }: Props = $props();
</script>

<Dialog title={t('profiles-delete-title', { name: profile.name })} kind="danger" icon={Trash} {oncancel}>
  {#if profile.mods > 0}
    <p>{t('profiles-delete-applied', { name: profile.name, count: profile.mods })}</p>
    {#if root}<div class="dl-url">{`${root}\\game\\citadel\\addons`}</div>{/if}
  {:else}
    <p>{t('profiles-delete-applied-empty', { name: profile.name })}</p>
  {/if}
  <p>{t('profiles-delete-stay')}</p>
  {#if next}<p class="next">{t('profiles-delete-next', { name: next })}</p>{/if}
  <p>{t('profiles-delete-undoable')}</p>
  {#snippet footerStart()}
    <Button variant="ghost" icon={FileOutput} label={t('profiles-export-first')} onclick={onexport} />
  {/snippet}
  {#snippet footer()}
    <Button label={t('profiles-cancel')} data-autofocus="" onclick={oncancel} />
    <Button variant="danger" icon={Trash} label={t('profiles-delete-profile')} onclick={ondelete} />
  {/snippet}
</Dialog>

<style>
  .next {
    color: var(--ink);
  }
</style>
