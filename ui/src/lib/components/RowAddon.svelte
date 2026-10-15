<!-- One add-on (or the main files) of a library mod (design: LibraryRow.md, `.dl-addon`): enabled, disabled and
     reordered together with its mod; shows the installed file, where it came from and its size or state. -->
<script lang="ts">
  import X from '@lucide/svelte/icons/x';
  import Button from './Button.svelte';
  import type { IconComponent } from './types';

  interface Props {
    icon: IconComponent;
    name: string;
    /** "· pak_x.vpk · from x.zip" (mono). */
    file: string;
    /** Size, or "On with the mod". */
    state: string;
    /** Shows the remove button when set. */
    removeLabel?: string;
    onremove?: () => void;
  }

  const { icon: Icon, name, file, state, removeLabel, onremove }: Props = $props();
</script>

<div class="dl-addon">
  <Icon class="dl-icon" />
  <span>{name} <span class="dl-addon__file">{file}</span></span>
  <span class="dl-addon__state">{state}</span>
  {#if removeLabel}
    <Button variant="ghost" size="sm" iconOnly icon={X} label={removeLabel} title={removeLabel} onclick={onremove} />
  {:else}
    <span></span>
  {/if}
</div>
