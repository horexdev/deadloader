<!-- The Catalog area: the grid (S2) at `#/catalog`, a mod page (S3) at `#/catalog/<mod|sound>/<id>`. Back returns to
     the grid where it was left: the grid's state lives in `browser`, its scroll position included. -->
<script lang="ts">
  import { router } from '../../lib/router.svelte';
  import Grid from './Grid.svelte';
  import ModPage from './ModPage.svelte';
  import type { Section } from '../../lib/api';

  const opened = $derived.by(() => {
    const [section, id] = router.path;
    if ((section === 'mod' || section === 'sound') && id !== undefined && /^\d+$/.test(id)) {
      return { section: section as Section, id: Number(id) };
    }
    return null;
  });
</script>

{#if opened}
  {#key `${opened.section}-${opened.id}`}
    <ModPage section={opened.section} id={opened.id} />
  {/key}
{:else}
  <Grid />
{/if}
