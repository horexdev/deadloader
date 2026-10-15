<script lang="ts">
  import { onMount, type Component } from 'svelte';
  import { app } from './lib/app.svelte';
  import { router, type Route } from './lib/router.svelte';
  import Shell from './shell/Shell.svelte';

  // Each screen is src/screens/<route>/Screen.svelte; a route without one shows an empty area.
  const screens = import.meta.glob<{ default: Component }>('./screens/*/Screen.svelte', { eager: true });

  function screenOf(route: Route): Component | undefined {
    return screens[`./screens/${route}/Screen.svelte`]?.default;
  }

  const Screen = $derived(screenOf(router.current));

  onMount(() => {
    void app.start();
  });
</script>

<Shell {app} {router}>
  {#if Screen}
    <Screen />
  {/if}
</Shell>
