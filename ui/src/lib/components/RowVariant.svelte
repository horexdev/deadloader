<!-- One variant of a library mod (design: LibraryRow.md, `.dl-variant`). A downloaded variant is a native radio
     (selecting it switches at once, or becomes pending while the game runs); a remote one links to the mod page. -->
<script lang="ts">
  import Check from '@lucide/svelte/icons/check';
  import Badge from './Badge.svelte';

  export interface RemoteVariant {
    label: string;
    href: string;
  }

  interface Props {
    name: string;
    /** Size on disk, or "On GameBanana" for remote variants. */
    size: string;
    /** E.g. "Select to switch". */
    hint?: string;
    /** Radio group name, shared by the variants of one mod. */
    group: string;
    checked?: boolean;
    /** Badge text on the active variant. */
    activeLabel?: string;
    /** Not downloaded: shown with a dashed dot and a link to the mod page. */
    remote?: RemoteVariant;
    onselect?: () => void;
  }

  const { name, size, hint, group, checked = false, activeLabel, remote, onselect }: Props = $props();
</script>

{#if remote}
  <div class="dl-variant dl-variant--remote">
    <span class="dl-variant__dot"></span><span>{name}</span><span class="dl-variant__size">{size}</span><a
      class="dl-btn dl-btn--sm link"
      href={remote.href}>{remote.label}</a
    >
  </div>
{:else}
  <label class="dl-variant">
    <input class="dl-variant__radio" type="radio" name={group} {checked} onchange={onselect} />
    <span class="dl-variant__dot"></span>
    <span
      >{name}{#if checked && activeLabel}
        <span class="badge"><Badge icon={Check} label={activeLabel} variant="accent" /></span>{/if}</span
    >
    <span class="dl-variant__size">{size}</span>
    <span class="hint">{hint ?? ''}</span>
  </label>
{/if}

<style>
  /* From the design's library screens. */
  .link {
    text-decoration: none;
  }

  .badge {
    margin-left: 6px;
  }

  .hint {
    font: var(--type-caption);
    color: var(--ink-subtle);
  }
</style>
