import { flushSync, type ComponentProps } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import { all, click, destroy, keydown, one, render } from '../test/render';
import Sidebar from './Sidebar.svelte';

type SidebarProps = ComponentProps<typeof Sidebar>;

const labels = {
  nav: 'Main',
  catalog: 'Catalog',
  library: 'Library',
  profiles: 'Profiles',
  downloads: 'Downloads',
  activity: 'Activity',
  settings: 'Settings',
  pending: 'Pending changes',
  collapse: 'Collapse sidebar',
  expand: 'Expand sidebar',
  close: 'Close sidebar',
  appName: 'deadloader',
};

const base: SidebarProps = {
  mode: 'expanded',
  current: 'catalog',
  labels,
  libraryMeta: '11',
  pending: { count: '3', label: 'Pending changes: 3' },
  profile: { name: 'Evening', state: 'Applied to the game', pending: false, label: 'Profile Evening · Applied to the game' },
  game: { state: 'found', text: 'Deadlock found · D:', title: 'Deadlock found — D:\\SteamLibrary\\steamapps\\common\\Deadlock' },
  network: { state: 'online', active: false, label: 'Online', title: 'Online — open Activity' },
  play: { label: 'Play', title: 'Play' },
};

function setup(extra: Partial<SidebarProps> = {}) {
  const props: SidebarProps = $state({ ...base, ...extra });
  const rendered = render(Sidebar, props);
  return { ...rendered, props, nav: one(rendered.target, 'nav.dl-sidebar') };
}

const items = (nav: HTMLElement) => all<HTMLAnchorElement>(nav, '.dl-sidebar__nav a.dl-nav-item');

describe('Sidebar', () => {
  it('is the labelled main navigation with the six areas in order, marking the current page', () => {
    const { nav } = setup();
    expect(nav.getAttribute('aria-label')).toBe('Main');
    const links = items(nav);
    expect(links.map((a) => a.getAttribute('aria-label'))).toEqual([
      'Catalog',
      'Library',
      'Profiles',
      'Downloads',
      'Activity',
      'Settings',
    ]);
    expect(links.map((a) => a.getAttribute('href'))).toEqual([
      '#/catalog',
      '#/library',
      '#/profiles',
      '#/downloads',
      '#/activity',
      '#/settings',
    ]);
    expect(links[0]?.getAttribute('aria-current')).toBe('page');
    expect(links.slice(1).every((a) => !a.hasAttribute('aria-current'))).toBe(true);
    expect(one(links[1] as HTMLElement, '.dl-nav-item__meta').textContent).toBe('11');
  });

  it('shows the lockup and the collapse button when expanded', () => {
    const ontoggle = vi.fn();
    const { nav } = setup({ ontoggle });
    expect(nav.classList).not.toContain('dl-sidebar--rail');
    expect(one<HTMLImageElement>(nav, 'img.dl-sidebar__lockup').alt).toBe('deadloader');
    click(one(nav, '.dl-sidebar__head button[aria-label="Collapse sidebar"]'));
    expect(ontoggle).toHaveBeenCalledOnce();
  });

  it('reports navigation and supports custom hrefs', () => {
    const onnavigate = vi.fn((...args: [string, MouseEvent]) => args[1].preventDefault());
    const { nav } = setup({ onnavigate, hrefFor: (route) => `/${route}` });
    const library = items(nav)[1] as HTMLAnchorElement;
    expect(library.getAttribute('href')).toBe('/library');
    click(library);
    expect(onnavigate).toHaveBeenCalledWith('library', expect.any(MouseEvent));
  });

  it('shows the downloads count and names it', () => {
    const { nav } = setup({ downloads: { count: '2', label: 'Downloads · 2' } });
    const downloads = items(nav)[3] as HTMLElement;
    expect(downloads.getAttribute('aria-label')).toBe('Downloads · 2');
    expect(one(downloads, '.dl-count.dl-count--muted').textContent).toBe('2');
  });

  it('has the foot: pending changes with the accent count, profile, game status, network and Play', () => {
    const onpending = vi.fn();
    const onprofile = vi.fn();
    const onplay = vi.fn();
    const { nav } = setup({ onpending, onprofile, onplay, pending: { count: '3', label: 'Pending changes: 3', open: true } });
    const foot = one(nav, '.dl-sidebar__foot');
    const pending = one(foot, 'button.dl-nav-item[aria-haspopup="dialog"]');
    expect(pending.getAttribute('aria-label')).toBe('Pending changes: 3');
    expect(pending.getAttribute('aria-expanded')).toBe('true');
    expect(one(pending, '.dl-count').className).toBe('dl-count');
    click(pending);
    expect(onpending).toHaveBeenCalledOnce();
    const profile = one(foot, 'button.dl-profile');
    expect(profile.getAttribute('aria-label')).toBe('Profile Evening · Applied to the game');
    expect(one(profile, '.dl-profile__name').textContent).toBe('Evening');
    expect(one(profile, '.dl-profile__state').textContent).toBe('Applied to the game');
    click(profile);
    expect(onprofile).toHaveBeenCalledOnce();
    expect(one(foot, '.dl-game-status').textContent).toBe('Deadlock found · D:');
    const net = one<HTMLAnchorElement>(foot, 'a.dl-net');
    expect(net.getAttribute('href')).toBe('#/activity');
    expect(net.getAttribute('aria-label')).toBe('Online — open Activity');
    const play = one(foot, 'button.dl-btn--play');
    click(play);
    expect(onplay).toHaveBeenCalledOnce();
  });

  it('hides the pending count when there is nothing pending and marks a pending profile switch', () => {
    const { nav } = setup({
      pending: { label: 'Pending changes: 0' },
      profile: { name: 'Evening', state: 'Selected · not applied', pending: true, label: 'x' },
    });
    expect(nav.querySelector('.dl-sidebar__foot .dl-count')).toBeNull();
    expect(one(nav, '.dl-profile__state').classList).toContain('dl-profile__state--pending');
  });

  it.each([
    ['running', 'dl-game-status--running'],
    ['missing', 'dl-game-status--missing'],
  ] as const)('shows the %s game state', (state, cls) => {
    const { nav } = setup({ game: { state, text: 'x', title: 'x' } });
    expect(one(nav, '.dl-game-status').classList).toContain(cls);
  });

  it('shows the network states: requests running, offline and local mode', () => {
    const { nav, props } = setup({
      network: { state: 'online', active: true, count: '3', label: 'Online', title: 'Online · 3 requests running' },
    });
    const net = () => one(nav, '.dl-net');
    expect(net().dataset['active']).toBe('true');
    expect(one(net(), '.dl-net__activity').textContent).toBe('3');
    props.network = { state: 'offline', active: false, label: 'Offline', detail: 'saved catalog from 14:20', title: 'x' };
    flushSync();
    expect(net().classList).toContain('dl-net--offline');
    expect(one(net(), '.dl-net__label b').textContent).toBe('Offline');
    expect(one(net(), '.dl-net__label').textContent).toBe('Offline · saved catalog from 14:20');
    expect(net().querySelector('.dl-net__activity')).toBeNull();
    props.network = { state: 'local', active: false, label: 'Local mode', detail: 'no requests', title: 'x' };
    flushSync();
    expect(net().classList).toContain('dl-net--local');
    expect(one(net(), 'svg').classList).toContain('lucide-unplug');
  });

  it('shows Play busy or unavailable with its reason', () => {
    const { nav } = setup({ play: { label: 'Applying…', title: 'Applying…', busy: true } });
    expect(one(nav, '.dl-btn--play').getAttribute('aria-busy')).toBe('true');
    const other = setup({ play: { label: 'Play', title: 'Deadlock is already running', unavailable: true } });
    const play = one(other.nav, '.dl-btn--play');
    expect(play.getAttribute('aria-disabled')).toBe('true');
    expect(play.getAttribute('aria-label')).toBe('Deadlock is already running');
  });

  it('collapses to the rail: app icon, icon-only items with tooltips, expand button in the foot', () => {
    const ontoggle = vi.fn();
    const { nav } = setup({ mode: 'rail', ontoggle });
    expect(nav.classList).toContain('dl-sidebar--rail');
    expect(nav.querySelector('.dl-sidebar__lockup')).toBeNull();
    expect(one<HTMLImageElement>(nav, '.dl-sidebar__head img.dl-sidebar__appicon').alt).toBe('deadloader');
    expect(nav.querySelector('.dl-profile')).toBeNull();
    expect(one(nav, '.dl-sidebar__foot button[aria-label="Profile Evening · Applied to the game"]')).toBeTruthy();
    expect(one(nav, '.dl-sidebar__foot .dl-game-status[role="img"]').getAttribute('aria-label')).toBe(base.game.title);
    click(one(nav, '.dl-sidebar__foot button[aria-label="Expand sidebar"]'));
    expect(ontoggle).toHaveBeenCalledOnce();
    const library = items(nav)[1] as HTMLElement;
    library.focus();
    flushSync();
    expect(document.querySelector('body > .dl-tooltip')?.textContent).toBe('Library');
  });

  it('moves the expand button into the head on short windows', () => {
    const { nav } = setup({ mode: 'rail', compact: true });
    expect(one(nav, '.dl-sidebar__head button[aria-label="Expand sidebar"] img.dl-sidebar__appicon').getAttribute('alt')).toBe('');
    expect(nav.querySelector('.dl-sidebar__foot button[aria-label="Expand sidebar"]')).toBeNull();
  });

  it('opens as an overlay with a scrim that closes on click, on Esc, on the close button and after choosing', () => {
    const opener = document.createElement('button');
    document.body.append(opener);
    opener.focus();
    const onclose = vi.fn();
    const onnavigate = vi.fn((...args: [string, MouseEvent]) => args[1].preventDefault());
    const { nav, target, instance } = setup({ mode: 'overlay', onclose, onnavigate });
    expect(nav.classList).toContain('dl-sidebar--overlay');
    const close = one(nav, '.dl-sidebar__head button[aria-label="Close sidebar"]');
    expect(document.activeElement).toBe(close);
    click(one(target, '.dl-sidebar-scrim'));
    keydown(close, 'Escape');
    click(close);
    click(items(nav)[2] as HTMLElement);
    expect(onclose).toHaveBeenCalledTimes(4);
    expect(onnavigate).toHaveBeenCalledWith('profiles', expect.any(MouseEvent));
    // Tab stays inside the overlay.
    const play = one(nav, '.dl-btn--play');
    play.focus();
    keydown(play, 'Tab');
    expect(document.activeElement).toBe(close);
    destroy(instance);
    expect(document.activeElement).toBe(opener);
  });

  it('does not close the docked sidebar on Esc', () => {
    const onclose = vi.fn();
    const { nav } = setup({ onclose });
    keydown(items(nav)[0] as HTMLElement, 'Escape');
    expect(onclose).not.toHaveBeenCalled();
  });
});
