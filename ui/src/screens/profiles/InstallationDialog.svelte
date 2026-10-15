<!-- Change game installation (153): the Deadlock folder a profile is applied to. Nothing changes in either folder
     until the profile is applied. -->
<script lang="ts">
  import FolderCog from '@lucide/svelte/icons/folder-cog';
  import Button from '../../lib/components/Button.svelte';
  import Dialog from '../../lib/components/Dialog.svelte';
  import type { ProfileOverview } from '../../lib/api';
  import { t } from '../../i18n';
  import { driveOf, rootOf } from './profiles';

  interface Props {
    profile: ProfileOverview;
    installations: string[];
    profiles: ProfileOverview[];
    onsave: (root: string) => void;
    oncancel: () => void;
  }

  const { profile, installations, profiles, onsave, oncancel }: Props = $props();

  const uid = $props.id();
  const current = $derived(rootOf(profile, installations));
  // svelte-ignore state_referenced_locally
  let choice: string | null = $state(rootOf(profile, installations));
  const usedBy = (root: string) => profiles.filter((p) => rootOf(p, installations) === root).length;
</script>

<Dialog
  title={t('profiles-installation-title', { name: profile.name })}
  size="md"
  icon={FolderCog}
  closeLabel={t('profiles-close-esc')}
  {oncancel}
>
  <p>{t('profiles-installation-lead')}</p>
  <div class="pf-opts" role="radiogroup" aria-label={t('profiles-installation-title', { name: profile.name })}>
    {#each installations as root (root)}
      {@const count = usedBy(root)}
      <label class="dl-variant"
        ><input class="dl-variant__radio" type="radio" name={`${uid}-root`} checked={choice === root} onchange={() => (choice = root)} /><span
          class="dl-variant__dot"
        ></span><span class="pf-opt"
          ><span class="dl-mono path">{root}</span><small
            >{count === 0 ? t('profiles-used-by-none') : t('profiles-used-by', { count })}</small
          ></span
        ></label
      >
    {/each}
  </div>
  <p>
    {profile.applied
      ? t('profiles-installation-applied', { name: profile.name, drive: driveOf(current) })
      : t('profiles-installation-not-applied', { name: profile.name })}
  </p>
  {#snippet footerStart()}
    <a class="dl-link manage" href="#/settings/game" onclick={oncancel}>{t('profiles-manage-installations')}</a>
  {/snippet}
  {#snippet footer()}
    <Button label={t('profiles-cancel')} onclick={oncancel} />
    <Button
      variant="primary"
      label={t('profiles-use-installation')}
      data-autofocus=""
      onclick={() => choice && onsave(choice)}
    />
  {/snippet}
</Dialog>

<style>
  .path {
    color: var(--ink);
    overflow-wrap: anywhere;
  }

  .manage {
    font: var(--type-body-sm);
  }
</style>
