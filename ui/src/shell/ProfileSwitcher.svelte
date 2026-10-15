<!-- The sidebar's profile switcher (Sidebar.dc.html: the profile button opens a list). Choosing a profile selects it;
     the switch waits for Apply or Play (F6, 138), and Play asks first when mods would leave the game (D13). -->
<script lang="ts">
  import Check from '@lucide/svelte/icons/check';
  import Layers from '@lucide/svelte/icons/layers';
  import { onDestroy, onMount, tick } from 'svelte';
  import { api, shell } from '../lib/api';
  import type { AppState } from '../lib/app.svelte';
  import { t } from '../i18n';

  interface Props {
    app: AppState;
    /** The profile button it opens from. */
    anchor: HTMLElement;
    onclose: () => void;
    /** "Manage profiles": the Profiles screen. */
    onmanage: () => void;
  }

  const { app, anchor, onclose, onmanage }: Props = $props();

  const GAP = 4;
  const MIN_W = 220;
  const uid = $props.id();
  let names: string[] = $state([]);
  let list: HTMLElement | undefined = $state();
  let left = $state(0);
  let bottom = $state(0);
  let width = $state(MIN_W);

  const selected = $derived(app.library?.selected_profile ?? '');
  const applied = $derived(app.library?.applied_profile ?? null);

  function place() {
    const rect = anchor.getBoundingClientRect();
    // The rail's button is an icon: the list opens to its right; the wide sidebar's opens above the button.
    const rail = rect.width < MIN_W / 2;
    width = rail ? MIN_W : Math.max(MIN_W, rect.width);
    left = rail ? rect.right + GAP * 2 : rect.left;
    bottom = window.innerHeight - (rail ? rect.bottom : rect.top - GAP);
  }

  function outside(event: MouseEvent) {
    const target = event.target as Node | null;
    if (target && !list?.contains(target) && !anchor.contains(target)) {
      onclose();
    }
  }

  onMount(() => {
    place();
    document.addEventListener('mousedown', outside, true);
    void api
      .profileNames()
      .then(async (found) => {
        names = found;
        await tick();
        const current = list?.querySelector<HTMLElement>('[aria-selected="true"]') ?? options()[0];
        current?.focus();
      })
      .catch((error: unknown) => {
        app.fail(error);
        onclose();
      });
  });

  onDestroy(() => {
    document.removeEventListener('mousedown', outside, true);
    if (anchor.isConnected) {
      anchor.focus();
    }
  });

  function options(): HTMLElement[] {
    return list ? [...list.querySelectorAll<HTMLElement>('[role="option"], .manage')] : [];
  }

  async function choose(name: string) {
    if (name !== selected) {
      try {
        app.library = await api.selectProfile(name);
        app.game = await shell.gameStatus().catch(() => app.game);
      } catch (error) {
        app.fail(error);
      }
    }
    onclose();
  }

  function keydown(event: KeyboardEvent) {
    const items = options();
    const index = items.indexOf(document.activeElement as HTMLElement);
    if (event.key === 'Escape') {
      event.preventDefault();
      onclose();
    } else if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      const step = event.key === 'ArrowDown' ? 1 : -1;
      items[(index + step + items.length) % items.length]?.focus();
    } else if (event.key === 'Home' || event.key === 'End') {
      event.preventDefault();
      (event.key === 'Home' ? items[0] : items.at(-1))?.focus();
    } else if (event.key === 'Tab') {
      onclose();
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  bind:this={list}
  class="dl-menu switcher"
  style:left={`${left}px`}
  style:bottom={`${bottom}px`}
  style:width={`${width}px`}
  onkeydown={keydown}
>
  <div role="listbox" id={`${uid}-list`} aria-label={t('nav-profiles')}>
    {#each names as name (name)}
      <div
        class="dl-option option"
        role="option"
        tabindex="-1"
        aria-selected={name === selected ? 'true' : 'false'}
        onclick={() => void choose(name)}
        onkeydown={(event) => {
          if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            void choose(name);
          }
        }}
      >
        <span class="check">{#if name === selected}<Check class="dl-icon" />{/if}</span>
        <span class="name">{name}</span>
        {#if name === applied}<span class="state">{t('profiles-applied')}</span>{/if}
      </div>
    {/each}
  </div>
  <div class="dl-menu__sep" role="separator"></div>
  <button type="button" class="dl-option manage" tabindex="-1" onclick={onmanage}
    ><Layers class="dl-icon" />{t('profiles-switcher')}</button
  >
</div>

<style>
  /* The design draws the button, not the list: dl-menu and dl-option from the design system. */
  .switcher {
    position: fixed;
    z-index: var(--z-dialog);
    max-height: 60vh;
    overflow: auto;
    box-sizing: border-box;
  }

  .option,
  .manage {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    cursor: pointer;
  }

  .option:focus-visible,
  .manage:focus-visible {
    outline: none;
    box-shadow: inset 0 0 0 2px var(--focus);
  }

  .manage {
    border: 0;
    background: transparent;
    text-align: left;
    font: var(--type-body);
    color: var(--ink);
  }

  .check {
    display: grid;
    place-items: center;
    width: 16px;
    flex: none;
    color: var(--accent-text);
  }

  .name {
    flex: 1 1 auto;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .state {
    flex: none;
    font: var(--type-caption);
    color: var(--success);
  }
</style>
