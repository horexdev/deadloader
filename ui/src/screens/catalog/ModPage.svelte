<!-- S3 Mod page (design: ModPage, boards ModPage, ModPageDe, ModPageLoading, ModPageOffline, ModPageRemoved): gallery,
     meta and counts, the files with their install state (136, 144), the description, credits, ratings, licence and
     alternate sources; favourite and "Open on GameBanana" (D1). -->
<script lang="ts">
  import ArrowLeft from '@lucide/svelte/icons/arrow-left';
  import Check from '@lucide/svelte/icons/check';
  import ChevronLeft from '@lucide/svelte/icons/chevron-left';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';
  import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Download from '@lucide/svelte/icons/download';
  import ExternalLink from '@lucide/svelte/icons/external-link';
  import EyeOff from '@lucide/svelte/icons/eye-off';
  import Heart from '@lucide/svelte/icons/heart';
  import ImageIcon from '@lucide/svelte/icons/image';
  import RefreshCw from '@lucide/svelte/icons/refresh-cw';
  import ShieldAlert from '@lucide/svelte/icons/shield-alert';
  import ShieldCheck from '@lucide/svelte/icons/shield-check';
  import ShieldQuestion from '@lucide/svelte/icons/shield-question';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import WifiOff from '@lucide/svelte/icons/wifi-off';
  import Badge from '../../lib/components/Badge.svelte';
  import Banner from '../../lib/components/Banner.svelte';
  import Button from '../../lib/components/Button.svelte';
  import EmptyState from '../../lib/components/EmptyState.svelte';
  import Spinner from '../../lib/components/Spinner.svelte';
  import {
    api,
    catalog,
    downloads,
    imageUrl,
    isUserMessage,
    type Antivirus,
    type Card,
    type CatalogFile,
    type ContentRating,
    type ModPage,
    type Section,
  } from '../../lib/api';
  import { app } from '../../lib/app.svelte';
  import { openLink } from '../../lib/links';
  import { router } from '../../lib/router.svelte';
  import { describe, formatDate, formatList, formatNumber, formatSize, t } from '../../i18n';
  import Description from './Description.svelte';
  import { browser, type BrowseError } from './browse.svelte';
  import { exactTime, fileAction, installedVersion, isActive, libraryModOf, relativeTime } from './model';

  interface Props {
    section: Section;
    id: number;
  }

  const { section, id }: Props = $props();

  let page = $state<ModPage | null>(null);
  let loading = $state(true);
  let error = $state<BrowseError | null>(null);
  let picture = $state(0);
  let original = $state(false);
  let revealed = $state(false);
  let busyFile = $state<number | null>(null);

  const mod = $derived(libraryModOf(app.library, section, id));
  const offline = $derived(app.network.kind === 'offline');
  const local = $derived(app.network.kind === 'local_mode');
  const saved = $derived(page?.origin.kind === 'cache' && (offline || local) ? page.origin.fetched_at_unix_ms : null);
  const canInstall = $derived(!offline && !local);
  const nsfw = $derived((app.settings?.catalog?.nsfw as string | undefined) ?? 'hidden');
  const errorKind = $derived(error && !isUserMessage(error) ? error.kind : error ? 'message' : null);
  const gameBananaUrl = $derived(`https://gamebanana.com/${section === 'sound' ? 'sounds' : 'mods'}/${id}`);
  const listed = $derived(browser.pages.flat().find((card) => card.id === id && card.section === section));

  async function load(quiet = false): Promise<void> {
    if (!quiet) {
      loading = true;
      error = null;
    }
    try {
      const answer = await catalog.details(section, id, listed?.date_updated ?? null);
      page = answer;
      error = null;
    } catch (failure) {
      if (!quiet) {
        error = failure as BrowseError;
        page = null;
      }
    } finally {
      loading = false;
    }
  }

  // A new mod opened from a link in the description: start from its first picture.
  $effect(() => {
    void [section, id];
    picture = 0;
    original = false;
    revealed = false;
    void load();
  });

  // The core refreshed this page in the background.
  $effect(() => {
    const event = app.catalogEvent;
    if (event?.event === 'details_updated' && event.data.section === section && event.data.id === id) {
      void load(true);
    }
  });

  function back(): void {
    router.go('catalog');
  }

  function handleWindowKeydown(event: KeyboardEvent): void {
    if (event.altKey && event.key === 'ArrowLeft') {
      event.preventDefault();
      back();
    }
  }

  async function retry(): Promise<void> {
    await catalog.retryNow().catch(() => undefined);
    await load();
  }

  function asCard(p: ModPage): Card {
    return {
      id: p.id,
      section: p.section,
      name: p.name,
      author: p.author,
      root_category: p.category_path[0] ?? null,
      subcategory: p.category_path[1] ?? null,
      likes: p.likes,
      views: p.views,
      date_added: p.date_added,
      date_updated: p.date_updated,
      thumbnail: null,
      has_audio_preview: p.has_audio_preview,
      nsfw: p.nsfw,
      obsolete: p.obsolete,
      featured: false,
      has_files: p.files.length > 0,
    };
  }

  async function install(file: CatalogFile): Promise<void> {
    if (!page) {
      return;
    }
    busyFile = file.id;
    try {
      await downloads.install(section, id, file.id, page.name, page.author);
    } catch (failure) {
      app.fail(failure);
    } finally {
      busyFile = null;
    }
  }

  async function switchTo(file: string): Promise<void> {
    if (!mod) {
      return;
    }
    try {
      const changed = await api.switchVariant(mod.id, file);
      app.library = changed.library;
    } catch (failure) {
      app.fail(failure);
    }
  }

  function openInLibrary(): void {
    if (mod) {
      router.open('library', mod.id);
    }
  }

  function antivirus(value: Antivirus): { icon: typeof ShieldCheck; label: string; title: string; variant: 'success' | 'danger' | 'neutral' | 'outline' } {
    if (value === 'clean') {
      return { icon: ShieldCheck, label: t('catalog-av-clean'), title: t('catalog-av-clean-title'), variant: 'success' };
    }
    if (value === 'not_reported') {
      return { icon: ShieldQuestion, label: t('catalog-av-none'), title: t('catalog-av-none-title'), variant: 'outline' };
    }
    if ('flagged' in value) {
      return {
        icon: ShieldAlert,
        label: t('catalog-av-flagged'),
        title: t('catalog-av-flagged-title', { verdict: value.flagged }),
        variant: 'danger',
      };
    }
    return { icon: ShieldQuestion, label: t('catalog-av-pending'), title: t('catalog-av-pending-title'), variant: 'neutral' };
  }

  function rating(value: ContentRating): string {
    return typeof value === 'string' ? t(`catalog-rating-${value}`) : value.other.label;
  }

  /** The listed file a library file came from. */
  function listedFile(p: ModPage, remote: number | null): CatalogFile | undefined {
    return [...p.files, ...p.archived_files].find((f) => f.id === remote);
  }

  const installedLabel = $derived.by(() => {
    if (!mod || !page) {
      return null;
    }
    const active = mod.files.find((f) => isActive(mod, f.id));
    const variant = active ? listedFile(page, active.remote_file_id) : undefined;
    return [t('catalog-installed'), installedVersion(mod), variant?.description ?? variant?.file_name]
      .filter(Boolean)
      .join(' · ');
  });

  const variantsNote = $derived.by(() => {
    if (!mod || !page) {
      return false;
    }
    const listedIds = new Set(page.files.map((f) => f.id));
    return mod.variant_groups.some(
      (group) => group.files.filter((f) => listedIds.has(mod.files.find((x) => x.id === f)?.remote_file_id ?? -1)).length > 1,
    );
  });

  const avSummary = $derived.by(() => {
    if (!page || page.files.length === 0) {
      return null;
    }
    if (page.files.every((f) => f.antivirus === 'clean')) {
      const newest = Math.max(...page.files.map((f) => f.date_added ?? 0));
      return newest > 0
        ? t('catalog-files-clean-dated', { count: page.files.length, date: formatDate(newest * 1000) })
        : t('catalog-files-clean', { count: page.files.length });
    }
    return t('catalog-files-av');
  });

  const missingParts = $derived.by(() => {
    if (!page) {
      return [];
    }
    const parts: string[] = [];
    if (page.content_ratings.length === 0) {
      parts.push(t('catalog-no-ratings'));
    }
    if (!page.licence) {
      parts.push(t('catalog-no-licence'));
    }
    if (page.alternate_sources.length === 0) {
      parts.push(t('catalog-no-sources'));
    }
    return parts;
  });

  const errorText = $derived.by(() => {
    if (!error) {
      return null;
    }
    if (isUserMessage(error)) {
      return describe(error);
    }
    if ('message' in error && error.message) {
      return describe(error.message);
    }
    if (error.kind === 'offline') {
      return { what: t('catalog-page-offline-empty-title'), why: t('catalog-page-offline-empty-text'), todo: null };
    }
    if (error.kind === 'local_mode') {
      return { what: t('catalog-page-local-title'), why: t('catalog-page-local-text'), todo: null };
    }
    return { what: t('catalog-error-title'), why: null, todo: null };
  });
</script>

<svelte:window onkeydown={handleWindowKeydown} />

<div class="page">
  <nav aria-label={t('catalog-breadcrumb')} class="crumbs">
    <a href="#/catalog" class="dl-btn dl-btn--ghost dl-btn--sm back"><ArrowLeft class="dl-icon" />{t('catalog-title')}</a>
    <span aria-hidden="true">/</span><span>{t(section === 'sound' ? 'catalog-section-sounds' : 'catalog-section-mods')}</span>
    {#if page && page.category_path.length > 0}
      <span aria-hidden="true">/</span><span>{page.category_path.map((c) => c.name).join(' → ')}</span>
    {/if}
    {#if loading}
      <span role="status" class="loading"><Spinner />{t('catalog-page-loading')}</span>
    {/if}
  </nav>

  {#if saved !== null && page}
    <Banner kind="neutral" icon={WifiOff} title={t('catalog-page-offline', { date: formatDate(saved) })} text={t('catalog-page-offline-why')}>
      {#snippet actions()}
        <Button label={t('catalog-try-again')} size="sm" icon={RefreshCw} onclick={() => void retry()} />
      {/snippet}
    </Banner>
  {/if}

  {#if loading && !page}
    <div class="skel-head" aria-hidden="true">
      <span class="dl-skel skel-title"></span>
      <span class="dl-skel dl-skel--text w30"></span>
      <span class="dl-skel dl-skel--text w58"></span>
    </div>
    <div class="columns" aria-hidden="true">
      <div class="column">
        <span class="dl-skel skel-picture"></span>
        <span class="dl-skel dl-skel--title w30"></span>
        <span class="dl-skel dl-skel--text w92"></span>
        <span class="dl-skel dl-skel--text w84"></span>
      </div>
      <div class="files skel-files">
        <span class="dl-skel dl-skel--title w30 skel-files-title"></span>
        {#each [72, 64, 80, 58, 70, 66] as w (w)}
          <div class="skel-row">
            <div><span class={['dl-skel dl-skel--text', `w${w}`]}></span><span class="dl-skel dl-skel--text w40"></span></div>
            <span class="dl-skel skel-button"></span>
          </div>
        {/each}
      </div>
    </div>
  {:else if errorKind === 'not_found'}
    <div class="state">
      <div class="dl-empty dl-empty--error narrow">
        <div class="dl-empty__glyph"><CircleX class="dl-icon dl-icon--lg" /></div>
        <h2 class="dl-empty__title">{t('catalog-removed-title')}</h2>
        <p class="dl-empty__text">{t('catalog-removed-why')}</p>
        {#if mod}
          <p class="dl-empty__text">
            {t('catalog-removed-keep', { copy: [installedVersion(mod), mod.name].filter(Boolean).join(' · ') })}
          </p>
        {/if}
        <div class="dl-empty__actions">
          {#if mod}
            <Button label={t('catalog-show-in-library')} variant="primary" onclick={openInLibrary} />
          {/if}
          <Button label={t('catalog-back')} onclick={back} />
        </div>
      </div>
    </div>
  {:else if error && errorText}
    <div class="state" role="alert">
      <EmptyState icon={errorKind === 'offline' ? WifiOff : CircleX} title={errorText.what} text={errorText.why ?? undefined} error level={2}>
        {#snippet actions()}
          {#if errorKind !== 'local_mode'}
            <Button label={t('catalog-try-again')} variant="primary" icon={RefreshCw} onclick={() => void retry()} />
          {/if}
          <Button label={t('catalog-back')} onclick={back} />
        {/snippet}
      </EmptyState>
    </div>
  {:else if page}
    {@const current = page}
    {@const image = current.gallery[picture]}
    {@const blurred = current.nsfw && nsfw === 'blurred' && !revealed}
    {@const favourite = browser.isFavourite(current)}
    <div class="head">
      <div class="title">
        <div class="name">
          <h1>{current.name}</h1>
          {#if installedLabel}<Badge icon={Check} label={installedLabel} variant="accent" />{/if}
        </div>
        {#if current.short_description}<p class="summary">{current.short_description}</p>{/if}
        <div class="meta">
          {#if current.author}<span>{t('catalog-by', { author: current.author })}</span>{/if}
          {#if current.version}<span>{t('catalog-version', { version: current.version })}</span>{/if}
          <span>{t('catalog-likes', { count: current.likes })}</span>
          <span>{t('catalog-views', { count: current.views })}</span>
          <span>{t('catalog-downloads', { count: current.downloads })}</span>
          {#if current.date_added}
            <span title={exactTime(current.date_added)}>{t('catalog-added', { when: relativeTime(current.date_added) })}</span>
          {/if}
          {#if current.date_updated}
            <span title={exactTime(current.date_updated)}>{t('catalog-updated', { when: relativeTime(current.date_updated) })}</span>
          {/if}
        </div>
      </div>
      <div class="actions">
        <button
          type="button"
          class={['dl-btn', favourite && 'fav-on']}
          aria-pressed={favourite ? 'true' : 'false'}
          onclick={() => void browser.toggleFavourite(asCard(current))}><Heart class="dl-icon" />{t('catalog-favourite')}</button
        >
        <Button
          label={t('catalog-open-gamebanana')}
          icon={ExternalLink}
          aria-haspopup="dialog"
          onclick={() => void openLink(gameBananaUrl)}
        />
      </div>
    </div>

    <div class="columns">
      <div class="column">
        {#if image && !current.nsfw_hidden}
          <figure class="gallery">
            <img
              src={imageUrl(original ? image.original : image.preview)}
              alt={image.caption ?? t('catalog-gallery-alt', { name: current.name, index: picture + 1 })}
              class={[blurred && 'blurred']}
            />
            {#if blurred}
              <button type="button" class="reveal" onclick={() => (revealed = true)}
                ><span><EyeOff class="dl-icon" /> {t('catalog-sensitive')}</span><small>{t('catalog-click-show')}</small></button
              >
            {/if}
            <figcaption>
              {#if current.gallery.length > 1}
                <button
                  type="button"
                  class="chip"
                  aria-label={t('catalog-gallery-previous')}
                  title={t('catalog-gallery-previous')}
                  onclick={() => {
                    picture = (picture - 1 + current.gallery.length) % current.gallery.length;
                    original = false;
                  }}><ChevronLeft class="dl-icon" /></button
                >
              {/if}
              <span class="chip">{t('catalog-gallery-count', { index: picture + 1, count: current.gallery.length })}</span>
              {#if current.gallery.length > 1}
                <button
                  type="button"
                  class="chip"
                  aria-label={t('catalog-gallery-next')}
                  title={t('catalog-gallery-next')}
                  onclick={() => {
                    picture = (picture + 1) % current.gallery.length;
                    original = false;
                  }}><ChevronRight class="dl-icon" /></button
                >
              {/if}
              {#if !original && !blurred}
                <button type="button" class="chip" onclick={() => (original = true)}>{t('catalog-gallery-original')}</button>
              {/if}
            </figcaption>
          </figure>
        {:else}
          <div class="gallery gallery--empty">
            <ImageIcon class="dl-icon dl-icon--lg" />
            {#if current.nsfw_hidden}<span>{t('catalog-gallery-hidden')}</span>{/if}
          </div>
        {/if}
        {#if current.has_audio_preview}
          <p class="note">{t('catalog-audio-preview')}</p>
        {/if}
        <section class="description">
          <h2>{t('catalog-description')}</h2>
          <Description
            description={current.description}
            lang="en"
            collapsible
            onmod={(target) => router.open('catalog', 'mod', String(target))}
          />
        </section>
      </div>

      <div class="column">
        <section class="files" aria-labelledby="catalog-files-title">
          <div class="files-head">
            <div class="files-title">
              <h2 id="catalog-files-title">{t('catalog-files')}</h2>
              <span class="dl-count dl-count--muted">{formatNumber(current.files.length)}</span>
            </div>
            {#if avSummary}<span class="caption">{avSummary}</span>{/if}
            {#if variantsNote}<span class="caption muted">{t('catalog-variants-note')}</span>{/if}
          </div>
          {#if current.files.length === 0}
            <p class="none">{t('catalog-no-files')}</p>
          {/if}
          {#each current.files as file (file.id)}
            {@render fileRow(current, file)}
          {/each}
          {#if current.archived_files.length > 0}
            <details class="older">
              <summary>{t('catalog-older-files', { count: current.archived_files.length })}</summary>
              {#each current.archived_files as file (file.id)}
                {@render fileRow(current, file)}
              {/each}
            </details>
          {/if}
        </section>

        <div class="side">
          {#if current.credits.length > 0}
            <span
              ><strong>{t('catalog-credits')}</strong> · {current.credits
                .map(
                  (group) =>
                    `${group.name}: ${formatList(group.authors.map((a) => (a.role ? `${a.name} (${a.role})` : a.name)))}`,
                )
                .join('; ')}</span
            >
          {/if}
          {#if current.content_ratings.length > 0}
            <div class="ratings">
              {#each current.content_ratings as value (rating(value))}
                <Badge icon={TriangleAlert} label={rating(value)} variant="warning" />
              {/each}
            </div>
          {/if}
          {#if missingParts.length > 0}
            <span class="subtle">{missingParts.join(' · ')}</span>
          {/if}
          {#if current.licence}
            <details class="more-block">
              <summary>{t('catalog-licence')}</summary>
              <Description description={current.licence} lang="en" onmod={(target) => router.open('catalog', 'mod', String(target))} />
            </details>
          {/if}
          {#if current.alternate_sources.length > 0}
            <div class="links">
              <strong>{t('catalog-sources')}</strong>
              {#each current.alternate_sources as source (source.url)}
                <button type="button" class="dl-link linkbtn" title={source.url} onclick={() => void openLink(source.url)}
                  >{source.description ?? source.url}<ExternalLink class="dl-icon" aria-label={t('catalog-opens-browser')} /></button
                >
              {/each}
            </div>
          {/if}
          {#if current.embedded_media.length > 0}
            <div class="links">
              <strong>{t('catalog-videos')}</strong>
              {#each current.embedded_media as url (url)}
                <button type="button" class="dl-link linkbtn" title={url} onclick={() => void openLink(url)}
                  >{url}<ExternalLink class="dl-icon" aria-label={t('catalog-opens-browser')} /></button
                >
              {/each}
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

{#snippet fileRow(current: ModPage, file: CatalogFile)}
  {@const action = fileAction({ fileId: file.id, modId: current.id, mod, updates: app.updates, tasks: app.tasks })}
  {@const av = antivirus(file.antivirus)}
  {@const state =
    action.kind === 'installed'
      ? t('catalog-file-active')
      : action.kind === 'switch'
        ? t('catalog-file-downloaded')
        : ''}
  <div class={['file', action.kind === 'installed' && 'file--active']}>
    <div class="file-text">
      <span class="dl-mono file-name" title={file.file_name}>{file.file_name}</span>
      <span class="file-meta"
        >{[file.description, formatSize(file.size_bytes), t('catalog-file-downloads', { count: file.download_count })]
          .filter(Boolean)
          .join(' · ')}{state ? ` · ${state}` : ''}<span title={av.title}><Badge icon={av.icon} label={av.label} variant={av.variant} /></span></span
      >
    </div>
    {#if action.kind === 'installed'}
      <a href={mod ? `#/library/${encodeURIComponent(mod.id)}` : '#/library'} class="dl-btn dl-btn--ghost dl-btn--sm installed"
        ><Check class="dl-icon" />{t('catalog-file-installed')}</a
      >
    {:else if action.kind === 'switch'}
      <Button
        label={t('catalog-file-switch')}
        size="sm"
        aria-label={t('catalog-file-switch-label', { file: file.file_name })}
        title={t('catalog-file-switch-title')}
        onclick={() => void switchTo(action.file)}
      />
    {:else if action.kind === 'update'}
      <Button label={t('catalog-file-update')} size="sm" icon={CircleArrowUp} title={t('catalog-file-update-title')} onclick={openInLibrary} />
    {:else if action.kind === 'installing'}
      <Button
        label={action.percent === null ? t('catalog-file-installing') : t('catalog-card-installing', { percent: action.percent })}
        size="sm"
        busy
      />
    {:else}
      <Button
        label={t('catalog-file-install')}
        size="sm"
        icon={Download}
        busy={busyFile === file.id}
        unavailable={!canInstall}
        aria-label={t('catalog-file-install-label', { file: file.file_name })}
        title={canInstall ? t('catalog-file-install-title') : t('catalog-file-install-offline')}
        onclick={() => void install(file)}
      />
    {/if}
  </div>
{/snippet}

<style>
  /* The design's mod page <main>: a column with 16px gaps; here it scrolls as a whole. */
  .page {
    display: flex;
    flex-direction: column;
    gap: 16px;
    height: 100%;
    min-width: 0;
    padding: 16px 24px 24px;
    overflow-y: auto;
  }

  .crumbs {
    display: flex;
    align-items: center;
    gap: 8px;
    font: var(--type-body-sm);
    color: var(--ink-subtle);
  }

  .back {
    padding-left: 4px;
    text-decoration: none;
  }

  .loading {
    margin-left: auto;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font: var(--type-caption);
  }

  .head {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
    gap: 12px 24px;
  }

  .title {
    display: grid;
    gap: 6px;
    flex: 1 1 420px;
    min-width: 0;
  }

  .name {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px 12px;
  }

  h1 {
    margin: 0;
    font: var(--type-title-lg);
    color: var(--ink);
  }

  .summary {
    margin: 0;
    font: var(--type-body);
    color: var(--ink-muted);
  }

  .meta {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 4px 16px;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .fav-on :global(.dl-icon) {
    color: var(--accent-to);
    fill: currentColor;
  }

  /* 530 px gallery and description, the files beside them (280 px minimum below ~1100 px, as on the German board). */
  .columns {
    display: grid;
    grid-template-columns: minmax(0, 530px) minmax(320px, 1fr);
    gap: 24px;
    min-height: 0;
  }

  @media (max-width: 1099px) {
    .columns {
      grid-template-columns: minmax(0, 530px) minmax(280px, 1fr);
    }
  }

  .column {
    display: grid;
    align-content: start;
    gap: 16px;
    min-width: 0;
  }

  .gallery {
    position: relative;
    margin: 0;
    width: 100%;
    aspect-ratio: 530 / 298;
    overflow: hidden;
    border-radius: 8px;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
  }

  .gallery img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: 50% 30%;
  }

  .gallery img.blurred {
    filter: blur(24px);
    transform: scale(1.1);
  }

  .gallery--empty {
    display: grid;
    place-content: center;
    justify-items: center;
    gap: 8px;
    color: var(--ink-subtle);
    font: var(--type-caption);
  }

  .reveal {
    position: absolute;
    inset: 0;
    display: grid;
    place-content: center;
    justify-items: center;
    gap: 4px;
    color: var(--ink);
    font: var(--type-body-sm);
    background: none;
    border: 0;
    cursor: pointer;
  }

  .reveal span,
  .reveal small {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 8px;
    background: var(--scrim);
    border-radius: 4px;
  }

  .reveal small {
    font: var(--type-caption);
    color: var(--ink-muted);
  }

  figcaption {
    position: absolute;
    right: 8px;
    bottom: 8px;
    display: flex;
    gap: 4px;
  }

  .chip {
    display: inline-flex;
    align-items: center;
    padding: 2px 8px;
    font: var(--type-caption);
    color: var(--ink);
    background: var(--scrim);
    border: 0;
    border-radius: 4px;
  }

  button.chip {
    cursor: pointer;
  }

  .chip :global(.dl-icon) {
    width: 14px;
    height: 14px;
  }

  .description {
    display: grid;
    gap: 8px;
  }

  h2 {
    margin: 0;
    font: var(--type-title-sm);
    color: var(--ink);
  }

  .files {
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 8px;
    overflow: hidden;
  }

  .files-head {
    display: grid;
    gap: 2px;
    padding: 12px 16px;
    border-bottom: 1px solid var(--line);
  }

  .files-title {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .caption {
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .caption.muted {
    color: var(--ink-muted);
  }

  .file {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: 12px;
    align-items: center;
    min-height: 56px;
    padding: 8px 12px 8px 16px;
    border-top: 1px solid var(--line);
  }

  .files-head + .file {
    border-top: 0;
  }

  .file--active {
    background: var(--accent-soft);
  }

  .file-text {
    display: grid;
    gap: 2px;
    min-width: 0;
  }

  .file-name {
    color: var(--ink);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-meta {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 4px 8px;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .installed {
    text-decoration: none;
    color: var(--accent-text);
  }

  .older summary,
  .more-block summary {
    padding: 8px 16px;
    font: var(--type-body-sm);
    color: var(--ink-muted);
    cursor: pointer;
    border-top: 1px solid var(--line);
  }

  .more-block summary {
    padding: 0;
    border: 0;
  }

  .more-block[open] summary {
    margin-bottom: 8px;
  }

  .none {
    margin: 0;
    padding: 12px 16px;
    font: var(--type-body-sm);
    color: var(--ink-subtle);
  }

  .side {
    display: grid;
    gap: 8px;
    font: var(--type-body-sm);
    color: var(--ink-muted);
  }

  .side strong {
    font-weight: 500;
    color: var(--ink);
  }

  .subtle,
  .note {
    margin: 0;
    color: var(--ink-subtle);
  }

  .note {
    font: var(--type-caption);
  }

  .ratings {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }

  .links {
    display: grid;
    justify-items: start;
    gap: 4px;
  }

  .linkbtn {
    padding: 0;
    font: inherit;
    text-align: left;
    background: none;
    border: 0;
    overflow-wrap: anywhere;
  }

  .state {
    display: grid;
    align-content: start;
    justify-items: center;
    padding-top: 40px;
    border-top: 1px solid var(--line);
  }

  .narrow {
    max-width: 500px;
  }

  .skel-head {
    display: grid;
    gap: 8px;
  }

  .skel-title {
    width: 46%;
    height: 24px;
  }

  .skel-picture {
    width: 100%;
    aspect-ratio: 530 / 298;
    border-radius: 8px;
  }

  .skel-files {
    display: grid;
    align-content: start;
    gap: 4px;
    padding: 12px;
  }

  .skel-files-title {
    margin-bottom: 8px;
  }

  .skel-row {
    display: grid;
    grid-template-columns: 1fr 72px;
    gap: 12px;
    align-items: center;
    height: 52px;
    border-top: 1px solid var(--line);
  }

  .skel-button {
    height: 28px;
    border-radius: 6px;
  }

  .w30 {
    width: 30%;
  }
  .w40 {
    width: 40%;
  }
  .w58 {
    width: 58%;
  }
  .w64 {
    width: 64%;
  }
  .w66 {
    width: 66%;
  }
  .w70 {
    width: 70%;
  }
  .w72 {
    width: 72%;
  }
  .w80 {
    width: 80%;
  }
  .w84 {
    width: 84%;
  }
  .w92 {
    width: 92%;
  }
</style>
