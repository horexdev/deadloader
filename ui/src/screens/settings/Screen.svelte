<!-- S8 Settings (design: SettingsScreen.dc.html): a section list on the left, the chosen section on the right, "Reset all
     settings…" under the list. The section is kept in the address (`#/settings/network`). -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { maintenance, type About } from '../../lib/api';
  import { app } from '../../lib/app.svelte';
  import { t } from '../../i18n';
  import AboutSection from './AboutSection.svelte';
  import AdvancedSection from './AdvancedSection.svelte';
  import CacheSection from './CacheSection.svelte';
  import CatalogSection from './CatalogSection.svelte';
  import DownloadsSection from './DownloadsSection.svelte';
  import GameSection from './GameSection.svelte';
  import GeneralSection from './GeneralSection.svelte';
  import LibrarySection from './LibrarySection.svelte';
  import LogsSection from './LogsSection.svelte';
  import NetworkSection from './NetworkSection.svelte';
  import ResetDialog from './ResetDialog.svelte';
  import { SECTIONS, asData, type Section } from './model';
  import './settings.css';

  function sectionOf(hash: string): Section {
    const name = hash.replace(/^#\/?/, '').split('/')[1] ?? '';
    return (SECTIONS as readonly string[]).includes(name) ? (name as Section) : 'general';
  }

  let section = $state<Section>(sectionOf(location.hash));
  let resetOpen = $state(false);
  let about = $state<About | null>(null);
  let content = $state<HTMLElement | undefined>();

  onMount(() => {
    const follow = () => (section = sectionOf(location.hash));
    window.addEventListener('hashchange', follow);
    maintenance
      .about()
      .then((a) => (about = a))
      .catch(() => undefined);
    return () => window.removeEventListener('hashchange', follow);
  });

  function go(next: Section): void {
    section = next;
    history.replaceState(null, '', `#/settings/${next}`);
    content?.scrollTo({ top: 0 });
  }

  const data = $derived(app.settings ? asData(app.settings) : null);
  const folder = $derived(about?.data_folder ?? '');
</script>

<div class="st-main">
  <h1>{t('settings-title')}</h1>
  <div class="st-body">
    <nav class="st-nav" aria-label={t('settings-nav')}>
      {#each SECTIONS as id (id)}
        <button
          type="button"
          class="dl-nav-item"
          aria-current={section === id ? 'page' : undefined}
          onclick={() => go(id)}><span class="dl-nav-item__label">{t(`settings-section-${id}`)}</span></button
        >
      {/each}
      <div class="st-nav__sep" role="separator"></div>
      <button type="button" class="dl-nav-item" onclick={() => (resetOpen = true)}
        ><span class="dl-nav-item__label">{t('settings-reset-nav')}</span></button
      >
    </nav>
    <div class="st-content" bind:this={content}>
      <div class="st-inner">
        <header class="st-head">
          <h2>{t(`settings-section-${section}`)}</h2>
          <p>{t(`settings-section-${section}-lead`)}</p>
        </header>
        {#if data}
          {#if section === 'general'}
            <GeneralSection {app} {data} />
          {:else if section === 'game'}
            <GameSection {app} {data} />
          {:else if section === 'catalog'}
            <CatalogSection {app} {data} />
          {:else if section === 'library'}
            <LibrarySection {app} {data} />
          {:else if section === 'downloads'}
            <DownloadsSection {app} {data} />
          {:else if section === 'network'}
            <NetworkSection {app} {data} />
          {:else if section === 'cache'}
            <CacheSection {app} {data} {folder} />
          {:else if section === 'logs'}
            <LogsSection {app} {data} {folder} />
          {:else if section === 'advanced'}
            <AdvancedSection {app} {data} {folder} />
          {:else}
            <AboutSection {app} {data} {about} />
          {/if}
        {/if}
      </div>
    </div>
  </div>
</div>

{#if resetOpen}
  <ResetDialog {app} onclose={() => (resetOpen = false)} />
{/if}
