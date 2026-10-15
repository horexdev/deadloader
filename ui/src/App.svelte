<script lang="ts">
  import { onMount, type Component } from 'svelte';
  import { startup, type Startup } from './lib/api';
  import { app } from './lib/app.svelte';
  import { notice, windowView, type Acknowledged } from './lib/notice.svelte';
  import { router, type Route } from './lib/router.svelte';
  import { screenLoader } from './lib/screens';
  import Shell from './shell/Shell.svelte';
  import NoWrite from './wizard/NoWrite.svelte';

  // Each screen is src/screens/<route>/Screen.svelte, loaded the first time it is shown; a route without one shows an
  // empty area.
  const screens = screenLoader(import.meta.glob<{ default: Component }>('./screens/*/Screen.svelte'));

  function screenOf(route: Route): Component | Promise<Component> | undefined {
    const screen = screens.get(route);
    if (screen instanceof Promise) {
      screen.catch((error: unknown) => app.fail(error));
    }
    return screen;
  }

  const Screen = $derived(screenOf(router.current));

  /** The first-run wizard is needed once, so it isn't part of every start either. */
  function wizard(): Promise<{ default: typeof import('./wizard/Wizard.svelte').default }> {
    const loading = import('./wizard/Wizard.svelte');
    loading.catch((error: unknown) => app.fail(error));
    return loading;
  }

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
  {#await wizard() then { default: Wizard }}
    <Wizard {app} mode={view.mode} ondone={wizardDone} />
  {:catch}
    <!-- The toast says what failed. -->
  {/await}
{:else if view.kind === 'shell'}
  <Shell {app} {router}>
    {#await Screen then Loaded}
      {#if Loaded}
        <Loaded />
      {/if}
    {:catch}
      <!-- The toast says what failed. -->
    {/await}
  </Shell>
{/if}
