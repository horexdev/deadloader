<!-- A group of settings (design: .st-group with its card). -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    lead?: string;
    danger?: boolean;
    titleId?: string;
    children: Snippet;
  }

  const { title, lead, danger = false, titleId, children }: Props = $props();
</script>

<section class="st-group">
  <h3 class="st-group__title" id={titleId}><Text value={title} /></h3>
  {#if lead}<p class="st-group__lead"><Text value={lead} /></p>{/if}
  <div class={['st-card', danger && 'st-card--danger']}>
    {@render children()}
  </div>
</section>
