<!-- The window: title bar, sidebar (expanded, rail, or the rail with the overlay, decisions 139/141), the current
     screen, D13 and the toasts. Screens live in src/screens/<route>/Screen.svelte and are found by their folder. -->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import Sidebar, { type SidebarMode, type SidebarRoute } from '../lib/components/Sidebar.svelte';
  import ToastStack from '../lib/components/ToastStack.svelte';
  import { api } from '../lib/api';
  import type { AppState } from '../lib/app.svelte';
  import { SIDEBAR_AUTO_COLLAPSE_PX } from '../lib/layout';
  import type { Router } from '../lib/router.svelte';
  import { formatNumber, t } from '../i18n';
  import ErrorHost from './ErrorHost.svelte';
  import PlayConfirm from './PlayConfirm.svelte';
  import Prompts from '../lib/dialogs/Prompts.svelte';
  import ProfileSwitcher from './ProfileSwitcher.svelte';
  import TitleBar from './TitleBar.svelte';
  import { sidebarNetwork } from './network';

  interface Props {
    app: AppState;
    router: Router;
    children: Snippet;
  }

  const { app, router, children }: Props = $props();

  /** Below this sidebar height the rail moves its expand button into the head (design: Sidebar). */
  const COMPACT_RAIL_PX = 640;

  let width = $state(window.innerWidth);
  let height = $state(window.innerHeight);
  let overlayOpen = $state(false);
  /** The profile button the switcher is open from. */
  let switcher: HTMLElement | null = $state(null);

  const narrow = $derived(width < SIDEBAR_AUTO_COLLAPSE_PX);
  const collapsed = $derived(app.settings?.ui?.sidebar_collapsed === true);
  const mode: SidebarMode = $derived(narrow || collapsed ? 'rail' : 'expanded');

  // The overlay belongs to a narrow window; widening it closes the overlay for good.
  $effect(() => {
    if (!narrow) {
      overlayOpen = false;
    }
  });

  const labels = $derived({
    nav: t('nav-main'),
    catalog: t('nav-catalog'),
    library: t('nav-library'),
    profiles: t('nav-profiles'),
    downloads: t('nav-downloads'),
    activity: t('nav-activity'),
    settings: t('nav-settings'),
    pending: t('nav-pending'),
    collapse: t('sidebar-collapse'),
    expand: t('sidebar-expand'),
    close: t('sidebar-close'),
    appName: 'deadloader',
  });

  const library = $derived(app.library);
  const switched = $derived(library !== null && library.applied_profile !== library.selected_profile);
  const profile = $derived.by(() => {
    const name = library?.selected_profile ?? '';
    const state = switched ? t('profile-not-applied') : t('profile-applied');
    return { name, state, pending: switched, label: t('profile-button', { name, state }) };
  });

  const game = $derived.by(() => {
    const status = app.game;
    const state = status?.running ? 'running' : status?.root ? 'found' : 'missing';
    const word = t(state === 'running' ? 'game-running' : state === 'found' ? 'game-found' : 'game-missing');
    const root = status?.root ?? null;
    const drive = root?.match(/^[A-Za-z]:/)?.[0];
    // The drive only when Steam knows more than one installation (153).
    const text = root && drive && (status?.installations ?? 0) > 1 ? t('game-on-drive', { status: word, drive }) : word;
    const title = root ? t('game-at-path', { status: word, path: root }) : word;
    return { state, text, title } as const;
  });

  const network = $derived(sidebarNetwork(app.network, app.running));

  const play = $derived.by(() => {
    const running = app.game?.running === true;
    const missing = !app.game?.root;
    const label = app.playing ? t('play-applying') : t('play');
    const title = app.playing ? label : running ? t('play-running') : missing ? t('play-missing') : label;
    return { label, title, busy: app.playing, unavailable: running || missing };
  });

  const downloads = $derived(
    app.downloads > 0
      ? { count: formatNumber(app.downloads), label: t('nav-downloads-count', { count: app.downloads }) }
      : undefined,
  );

  function navigate(route: SidebarRoute, event: MouseEvent): void {
    event.preventDefault();
    overlayOpen = false;
    router.go(route);
  }

  async function toggle(): Promise<void> {
    if (narrow) {
      overlayOpen = true;
      return;
    }
    // Wide windows remember the choice (139).
    const settings = app.settings;
    if (!settings) {
      return;
    }
    const next = { ...settings, ui: { ...settings.ui, sidebar_collapsed: !collapsed } };
    app.settings = next;
    try {
      app.settings = await api.updateSettings(next);
    } catch (error) {
      app.settings = settings;
      app.fail(error);
    }
  }

  const common = $derived({
    current: router.current,
    labels,
    libraryMeta: library ? formatNumber(library.mods.length) : undefined,
    downloads,
    pending: {
      label: library?.pending_count ? t('nav-pending-count', { count: library.pending_count }) : t('nav-pending'),
      count: library?.pending_count ? formatNumber(library.pending_count) : undefined,
      open: app.pendingOpen,
    },
    profile,
    game,
    network,
    play,
    compact: height - 36 < COMPACT_RAIL_PX,
    onnavigate: navigate,
    onpending: () => {
      app.pendingOpen = !app.pendingOpen;
      if (app.pendingOpen && router.current !== 'library') {
        router.go('library');
      }
    },
    onprofile: (event?: MouseEvent) => {
      const anchor = event?.currentTarget instanceof HTMLElement ? event.currentTarget : null;
      switcher = switcher ? null : anchor;
    },
    onplay: () => void app.requestPlay(),
  });
</script>

<svelte:window bind:innerWidth={width} bind:innerHeight={height} />

<div class="window dl-root">
  <TitleBar />
  <div class="body">
    <Sidebar mode={mode} {...common} ontoggle={() => void toggle()} />
    {#if narrow && overlayOpen}
      <Sidebar mode="overlay" {...common} onclose={() => (overlayOpen = false)} />
    {/if}
    <main class="content">
      {@render children()}
    </main>
  </div>
  {#if app.confirm}
    <PlayConfirm
      confirmation={app.confirm}
      onconfirm={() => void app.confirmPlay()}
      oncancel={() => (app.confirm = null)}
    />
  {/if}
  <ErrorHost {app} />
  <Prompts />
  {#if switcher}
    <ProfileSwitcher
      {app}
      anchor={switcher}
      onclose={() => (switcher = null)}
      onmanage={() => {
        switcher = null;
        router.go('profiles');
      }}
    />
  {/if}
  <ToastStack toasts={app.toasts} dismissLabel={t('toast-dismiss')} ondismiss={(id) => app.dismiss(id)} />
</div>

<style>
  .window {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-width: var(--window-min-w);
    min-height: var(--window-min-h);
    background: var(--bg-app);
  }

  .body {
    position: relative;
    display: flex;
    flex: 1 1 auto;
    min-height: 0;
  }

  .content {
    flex: 1;
    min-width: 0;
    overflow: auto;
    background: var(--bg-app);
  }
</style>
