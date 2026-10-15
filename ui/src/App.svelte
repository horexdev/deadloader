<script lang="ts">
  import { sidebarMode } from './lib/layout';

  let width = $state(window.innerWidth);
  const mode = $derived(sidebarMode(width));
</script>

<svelte:window bind:innerWidth={width} />

<div class="shell">
  <aside class="sidebar" data-mode={mode}></aside>
  <main class="content"></main>
</div>

<style>
  .shell {
    display: flex;
    height: 100%;
    min-width: var(--window-min-w);
    min-height: var(--window-min-h);
  }

  .sidebar {
    flex: none;
    width: var(--sidebar-w);
    background: var(--bg-chrome);
    border-right: 1px solid var(--line);
  }

  .sidebar[data-mode='rail'] {
    width: var(--sidebar-rail-w);
  }

  .content {
    flex: 1;
    min-width: 0;
    background: var(--bg-app);
  }
</style>
