<!-- The expanded parts of a library row (design: LibraryRow.md, `.dl-row-block__parts`): its variants or add-ons,
     under a head such as "7 variants, 2 downloaded". Put RowVariant or RowAddon lines inside. -->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    /** Head text, also the accessible name of the variant group. */
    label: string;
    /** Right side of the head, e.g. "2 downloaded". */
    meta?: string;
    /** Variants are one-of-many radios. */
    radio?: boolean;
    children: Snippet;
  }

  const { label, meta, radio = false, children }: Props = $props();
</script>

<div
  class="dl-row-block__parts"
  role={radio ? 'radiogroup' : undefined}
  aria-label={radio ? label : undefined}
>
  <div class="dl-row-block__head"><span>{label}</span>{#if meta}<span>{meta}</span>{/if}</div>
  {@render children()}
</div>
