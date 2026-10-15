<script lang="ts">
  import { onMount, type Component } from 'svelte';
  import { startup, type Startup } from './lib/api';
  import { app } from './lib/app.svelte';
  import { notice, windowView, type Acknowledged } from './lib/notice.svelte';
  import { router, type Route } from './lib/router.svelte';
  import Shell from './shell/Shell.svelte';
  import NoWrite from './wizard/NoWrite.svelte';
  import Wizard from './wizard/Wizard.svelte';

  // Each screen is src/screens/<route>/Screen.svelte; a route without one shows an empty area.
  const screens = import.meta.glob<{ default: Component }>('./screens/*/Screen.svelte', { eager: true });

  function screenOf(route: Route): Component | undefined {
    return screens[`./screens/${route}/Screen.svelte`]?.default;
  }

  const Screen = $derived(screenOf(router.current));

  /** How the core started (E1 blocks everything else), and whether the app state has loaded. */
  let started = $state<Startup | null>(null);
  let loaded = $state(false);

  onMount(() => {
    void startup
      .state()
      .catch((): Startup => ({ kind: 'ready' }))
      .then(async (state) => {
        started = state;
        if (state.kind === 'ready') {
          await app.start();
          loaded = true;
        }
      });
  });

  /** E1, the wizard (S1, F13, "Read it again") or the app. */
  const view = $derived(
    windowView({
      started,
      loaded,
      settingsRead: app.settings !== null,
      acknowledged: app.settings?.acknowledgements?.notice as Acknowledged | null | undefined,
      request: notice.current,
    }),
  );

  function wizardDone(route: Route | null): void {
    notice.close();
    if (route) {
      router.go(route);
    }
  }
</script>

{#if view.kind === 'blocked'}
  <NoWrite folder={view.folder} detail={view.detail} />
{:else if view.kind === 'wizard'}
  <Wizard {app} mode={view.mode} ondone={wizardDone} />
{:else if view.kind === 'shell'}
  <Shell {app} {router}>
    {#if Screen}
      <Screen />
    {/if}
  </Shell>
{/if}
