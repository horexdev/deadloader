<!-- A UI text with the product name set apart (decision 193): use it wherever a translated text that can name
     deadloader is shown as text (attributes and tooltips take the plain string). -->
<script lang="ts">
  import { brandParts } from '../brand';

  interface Props {
    value: string | null | undefined;
  }

  const { value }: Props = $props();
  const parts = $derived(brandParts(value ?? ''));
</script>

{#each parts as part, index (index)}{#if part.brand}<span class="dl-brand">{part.text}</span>{:else}{part.text}{/if}{/each}
