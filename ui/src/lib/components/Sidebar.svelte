<!-- Sidebar (design: Sidebar.md, Sidebar.dc.html): the main navigation and the global controls in three modes.
     expanded: 240px with labels; rail: 56px icons with tooltips (automatically below 1100px, see layout.ts);
     overlay: the expanded sidebar over the content, opened from the rail; Esc, the close button, the scrim or choosing
     an item closes it. The page composes them: the rail stays in place and the overlay is rendered beside it. -->
<script lang="ts" module>
  export type SidebarRoute = 'catalog' | 'library' | 'profiles' | 'downloads' | 'activity' | 'settings';
  export type SidebarMode = 'expanded' | 'rail' | 'overlay';

  export interface SidebarLabels {
    /** Accessible name of the navigation. */
    nav: string;
    catalog: string;
    library: string;
    profiles: string;
    downloads: string;
    activity: string;
    settings: string;
    pending: string;
    collapse: string;
    expand: string;
    close: string;
    /** Alternative text of the brand images. */
    appName: string;
  }

  export interface SidebarPending {
    /** Locale-formatted number; hidden when absent. */
    count?: string;
    /** Accessible name, e.g. "Pending changes: 3". */
    label: string;
    /** The pending-changes panel is open. */
    open?: boolean;
  }

  export interface SidebarDownloads {
    count: string;
    /** Accessible name, e.g. "Downloads · 2". */
    label: string;
  }

  export interface SidebarProfile {
    name: string;
    /** "Applied to the game" or "Selected · not applied". */
    state: string;
    /** The selected profile is not applied yet (warning colour). */
    pending: boolean;
    label: string;
  }

  export interface SidebarGame {
    state: 'found' | 'running' | 'missing';
    /** Words in expanded mode, e.g. "Deadlock found · D:". */
    text: string;
    /** Tooltip and the rail's accessible name, with the installation path. */
    title: string;
  }

  export interface SidebarNetwork {
    state: 'online' | 'offline' | 'local';
    /** Requests are running (online only). */
    active: boolean;
    /** "Online", "Offline", "Local mode". */
    label: string;
    /** After the word when not online: "saved catalog from 14:20", "no requests". */
    detail?: string;
    /** Number of running requests. */
    count?: string;
    /** Full status, used as accessible name and tooltip. */
    title: string;
  }

  export interface SidebarPlay {
    label: string;
    /** Accessible name and tooltip; says why Play is unavailable. */
    title: string;
    busy?: boolean;
    unavailable?: boolean;
  }
</script>

<script lang="ts">
  import Activity from '@lucide/svelte/icons/activity';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import Download from '@lucide/svelte/icons/download';
  import Layers from '@lucide/svelte/icons/layers';
  import LayoutGrid from '@lucide/svelte/icons/layout-grid';
  import LibraryBig from '@lucide/svelte/icons/library-big';
  import ListTodo from '@lucide/svelte/icons/list-todo';
  import PanelLeftClose from '@lucide/svelte/icons/panel-left-close';
  import PanelLeftOpen from '@lucide/svelte/icons/panel-left-open';
  import Play from '@lucide/svelte/icons/play';
  import Settings from '@lucide/svelte/icons/settings';
  import Unplug from '@lucide/svelte/icons/unplug';
  import User from '@lucide/svelte/icons/user';
  import Wifi from '@lucide/svelte/icons/wifi';
  import WifiOff from '@lucide/svelte/icons/wifi-off';
  import X from '@lucide/svelte/icons/x';
  import { onDestroy, onMount } from 'svelte';
  import appIcon from '../../assets/brand/icon-32.svg';
  import lockup from '../../assets/brand/lockup-dark.svg';
  import Button from './Button.svelte';
  import Count from './Count.svelte';
  import { trapTab } from './focus';
  import { tooltip } from './tooltip';
  import type { IconComponent } from './types';

  interface Props {
    mode: SidebarMode;
    current?: SidebarRoute;
    labels: SidebarLabels;
    /** The link target of each area. */
    hrefFor?: (route: SidebarRoute) => string;
    /** Quiet number after Library, e.g. the mod count. */
    libraryMeta?: string;
    downloads?: SidebarDownloads;
    pending: SidebarPending;
    profile: SidebarProfile;
    game: SidebarGame;
    network: SidebarNetwork;
    play: SidebarPlay;
    /** Rail on a short window: the expand button moves into the head. */
    compact?: boolean;
    onnavigate?: (route: SidebarRoute, event: MouseEvent) => void;
    /** Collapse (expanded) or expand (rail). */
    ontoggle?: () => void;
    /** Close the overlay. */
    onclose?: () => void;
    onpending?: () => void;
    onprofile?: () => void;
    onplay?: () => void;
  }

  const {
    mode,
    current,
    labels,
    hrefFor = (route) => `#/${route}`,
    libraryMeta,
    downloads,
    pending,
    profile,
    game,
    network,
    play,
    compact = false,
    onnavigate,
    ontoggle,
    onclose,
    onpending,
    onprofile,
    onplay,
  }: Props = $props();

  const rail = $derived(mode === 'rail');
  const overlay = $derived(mode === 'overlay');
  const areas: { route: SidebarRoute; icon: IconComponent }[] = [
    { route: 'catalog', icon: LayoutGrid },
    { route: 'library', icon: LibraryBig },
    { route: 'profiles', icon: Layers },
    { route: 'downloads', icon: Download },
    { route: 'activity', icon: Activity },
    { route: 'settings', icon: Settings },
  ];
  const NetIcon = $derived(network.state === 'offline' ? WifiOff : network.state === 'local' ? Unplug : Wifi);
  /** Tooltips only where the labels are hidden. */
  const tip = (text: string) => tooltip({ text: rail ? text : '' });

  let nav: HTMLElement | undefined = $state();
  let closeButton: HTMLElement | undefined = $state();
  const opener = document.activeElement instanceof HTMLElement ? document.activeElement : null;

  onMount(() => {
    if (overlay) {
      closeButton?.querySelector('button')?.focus();
    }
  });

  onDestroy(() => {
    if (overlay && opener?.isConnected) {
      opener.focus();
    }
  });

  function navigate(route: SidebarRoute, event: MouseEvent) {
    onnavigate?.(route, event);
    if (overlay) {
      onclose?.();
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!overlay || !nav) {
      return;
    }
    if (event.key === 'Escape') {
      event.preventDefault();
      onclose?.();
      return;
    }
    trapTab(event, nav);
  }
</script>

{#if overlay}
  <div class="dl-sidebar-scrim" role="presentation" onclick={onclose}></div>
{/if}
<!-- The overlay listens for Esc and keeps Tab inside while it is open. -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<nav
  bind:this={nav}
  class={['dl-sidebar', rail && 'dl-sidebar--rail', overlay && 'dl-sidebar--overlay']}
  aria-label={labels.nav}
  onkeydown={handleKeydown}
>
  <div class="dl-sidebar__head">
    {#if !rail}
      <img class="dl-sidebar__lockup" src={lockup} alt={labels.appName} />
      {#if overlay}
        <span bind:this={closeButton} class="contents"
          ><Button variant="ghost" size="sm" iconOnly icon={X} label={labels.close} title={labels.close} onclick={onclose}
          /></span
        >
      {:else}
        <Button
          variant="ghost"
          size="sm"
          iconOnly
          icon={PanelLeftClose}
          label={labels.collapse}
          title={labels.collapse}
          onclick={ontoggle}
        />
      {/if}
    {:else if compact}
      <button
        type="button"
        class="dl-btn dl-btn--ghost dl-btn--icon head-expand"
        aria-label={labels.expand}
        {@attach tip(labels.expand)}
        onclick={ontoggle}><img class="dl-sidebar__appicon" src={appIcon} alt="" /></button
      >
    {:else}
      <img class="dl-sidebar__appicon" src={appIcon} alt={labels.appName} />
    {/if}
  </div>

  <div class="dl-sidebar__nav">
    {#each areas as area (area.route)}
      {@const Icon = area.icon}
      {@const label = area.route === 'downloads' && downloads ? downloads.label : labels[area.route]}
      <a
        class="dl-nav-item"
        href={hrefFor(area.route)}
        aria-current={current === area.route ? 'page' : undefined}
        aria-label={label}
        {@attach tip(labels[area.route])}
        onclick={(event) => navigate(area.route, event)}
      >
        <Icon class="dl-icon" /><span class="dl-nav-item__label">{labels[area.route]}</span>
        {#if area.route === 'library' && libraryMeta}<span class="dl-nav-item__meta">{libraryMeta}</span>{/if}
        {#if area.route === 'downloads' && downloads}<Count value={downloads.count} muted />{/if}
      </a>
    {/each}
  </div>

  <div class="dl-sidebar__foot">
    {#if rail && !compact}
      <button
        type="button"
        class="dl-nav-item"
        aria-label={labels.expand}
        {@attach tip(labels.expand)}
        onclick={ontoggle}><PanelLeftOpen class="dl-icon" /></button
      >
    {/if}
    <button
      type="button"
      class={['dl-nav-item', !rail && 'full']}
      aria-label={pending.label}
      aria-haspopup="dialog"
      aria-expanded={pending.open ? 'true' : 'false'}
      {@attach tip(labels.pending)}
      onclick={onpending}
    >
      <ListTodo class="dl-icon" /><span class="dl-nav-item__label">{labels.pending}</span>
      {#if pending.count}<Count value={pending.count} />{/if}
    </button>
    {#if rail}
      <button
        type="button"
        class="dl-nav-item"
        aria-haspopup="listbox"
        aria-label={profile.label}
        {@attach tip(profile.label)}
        onclick={onprofile}><User class="dl-icon" /></button
      >
      <span
        class={['dl-game-status', 'rail-status', game.state !== 'found' && `dl-game-status--${game.state}`]}
        role="img"
        aria-label={game.title}
        {@attach tip(game.title)}><span class="dl-game-status__dot"></span></span
      >
    {:else}
      <button type="button" class="dl-profile" aria-haspopup="listbox" aria-label={profile.label} onclick={onprofile}>
        <span class="dl-profile__name">{profile.name}</span><ChevronDown class="dl-icon" /><span
          class={['dl-profile__state', profile.pending && 'dl-profile__state--pending']}>{profile.state}</span
        >
      </button>
      <div
        class={['dl-game-status', game.state !== 'found' && `dl-game-status--${game.state}`]}
        {@attach tooltip({ text: game.title })}
      >
        <span class="dl-game-status__dot"></span><span class="status-text">{game.text}</span>
      </div>
    {/if}
    <a
      class={['dl-net', network.state !== 'online' && `dl-net--${network.state}`, 'net-link']}
      href={hrefFor('activity')}
      data-active={network.active ? 'true' : 'false'}
      aria-label={network.title}
      {@attach tooltip({ text: network.title })}
      onclick={(event) => navigate('activity', event)}
    >
      <span class="dl-net__icon">
        <NetIcon class="dl-icon" />
      </span>
      <span class="dl-net__label"
        >{#if network.state === 'online'}{network.label}{:else}<b>{network.label}</b
          >{#if network.detail}{` · ${network.detail}`}{/if}{/if}</span
      >
      {#if network.state === 'online'}
        <span class="dl-net__activity"
          ><Activity class="dl-icon" />{#if network.active && network.count}{network.count}{/if}</span
        >
      {/if}
    </a>
    <Button
      variant="play"
      label={play.label}
      icon={Play}
      busy={play.busy}
      unavailable={play.unavailable}
      aria-label={play.title}
      title={play.title}
      onclick={onplay}
    />
  </div>
</nav>

<style>
  /* Small layout details from Sidebar.dc.html. */
  .contents {
    display: contents;
  }

  .head-expand {
    width: 40px;
    height: 40px;
    padding: 0;
  }

  .full {
    width: 100%;
  }

  .rail-status {
    padding: 4px 0;
  }

  .status-text {
    min-width: 0;
  }

  .net-link {
    text-decoration: none;
  }
</style>
