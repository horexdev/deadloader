<!-- The window's title bar (design: TitleBar.dc.html): the window has no system frame. Pressing the empty area moves
     the window, a double click maximises or restores it; the buttons minimise, maximise and close. -->
<script lang="ts">
  import Minus from '@lucide/svelte/icons/minus';
  import Square from '@lucide/svelte/icons/square';
  import X from '@lucide/svelte/icons/x';
  import icon from '../assets/brand/icon-16.svg';
  import { shell } from '../lib/api';
  import { t } from '../i18n';

  function drag(event: MouseEvent): void {
    // Only the primary button, and not the second press of a double click (that one maximises).
    if (event.button === 0 && event.detail < 2) {
      void shell.startDrag();
    }
  }
</script>

<header class="dl-root titlebar">
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="titlebar__drag" onmousedown={drag} ondblclick={() => void shell.toggleMaximize()}>
    <img class="titlebar__icon" src={icon} alt="" />
    <span class="titlebar__name">deadloader</span>
  </div>
  <div class="titlebar__buttons">
    <button type="button" class="titlebar__button" aria-label={t('window-minimize')} onclick={() => void shell.minimize()}>
      <Minus class="dl-icon" aria-hidden="true" />
    </button>
    <button
      type="button"
      class="titlebar__button"
      aria-label={t('window-maximize')}
      onclick={() => void shell.toggleMaximize()}
    >
      <Square class="dl-icon titlebar__square" aria-hidden="true" />
    </button>
    <button
      type="button"
      class="titlebar__button titlebar__button--close"
      aria-label={t('window-close')}
      onclick={() => void shell.close()}
    >
      <X class="dl-icon" aria-hidden="true" />
    </button>
  </div>
</header>

<style>
  .titlebar {
    position: relative;
    z-index: var(--z-titlebar);
    display: flex;
    flex: none;
    align-items: center;
    height: var(--titlebar-h);
    background: var(--bg-chrome);
    border-bottom: 1px solid var(--line);
    color: var(--ink);
    user-select: none;
  }

  .titlebar__drag {
    display: flex;
    flex: 1 1 auto;
    align-items: center;
    align-self: stretch;
    min-width: 0;
  }

  .titlebar__icon {
    display: block;
    width: 16px;
    height: 16px;
    margin-left: 20px;
  }

  .titlebar__name {
    margin-left: 8px;
    font: var(--type-caption);
    color: var(--ink-muted);
  }

  .titlebar__buttons {
    display: flex;
    align-self: stretch;
  }

  .titlebar__button {
    display: grid;
    place-items: center;
    width: 46px;
    height: var(--titlebar-h);
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--ink-muted);
    cursor: default;
  }

  .titlebar__button:hover {
    background: var(--surface-hover);
    color: var(--ink);
  }

  .titlebar__button--close:hover {
    background: var(--danger);
    color: var(--ink);
  }

  .titlebar__button:focus-visible {
    outline: 2px solid var(--focus);
    outline-offset: -2px;
  }

  .titlebar__button :global(.titlebar__square) {
    width: 14px;
    height: 14px;
  }
</style>
