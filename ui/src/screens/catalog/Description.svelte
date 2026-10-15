<!-- A mod's sanitised description (S3, R07): the core's HTML with its links, pictures, YouTube embeds and spoilers
     made usable (description.ts), long ones collapsed, and D2 before a picture from a new host loads. -->
<script lang="ts">
  import { catalog, imageUrl, type ConsentPrompt, type Description, type DescriptionImage } from '../../lib/api';
  import { app } from '../../lib/app.svelte';
  import { openLink } from '../../lib/links';
  import { t } from '../../i18n';
  import ImageConsent from './ImageConsent.svelte';
  import { enhanceDescription, type DescriptionController } from './description';

  interface Props {
    description: Description;
    /** The language of the author's text, when known (GameBanana descriptions are mostly English). */
    lang?: string;
    /** Long descriptions start collapsed (S3: from one line to 16 KB). */
    collapsible?: boolean;
    onmod: (id: number) => void;
  }

  const { description, lang, collapsible = false, onmod }: Props = $props();

  /** HTML longer than this starts collapsed. */
  const LONG = 3000;

  let root: HTMLElement | undefined = $state();
  let controller: DescriptionController | undefined;
  let expanded = $state(false);
  let consent: { image: DescriptionImage; prompt: ConsentPrompt } | null = $state(null);

  const long = $derived(collapsible && description.html.length > LONG);

  $effect(() => {
    const element = root;
    const current = description;
    if (!element) {
      return;
    }
    // The core's sanitised HTML: a fixed vocabulary without addresses, scripts or styles (crate deadloader-html).
    element.innerHTML = current.html;
    controller = enhanceDescription(
      element,
      current,
      {
        opensBrowser: t('catalog-opens-browser'),
        opensHere: t('catalog-opens-here'),
        imageFrom: t('catalog-image-from'),
        clickToLoad: t('catalog-image-click'),
        imageFailed: t('catalog-image-failed'),
        imageAlt: t('catalog-image-alt'),
        youtube: t('catalog-youtube'),
        spoiler: t('catalog-spoiler'),
        spoilerHide: t('catalog-spoiler-hide'),
      },
      {
        onlink: (url) => void openLink(url),
        onmod,
        onimage: (image) => void load(image),
        imageUrl,
      },
    );
  });

  async function load(image: DescriptionImage): Promise<void> {
    try {
      if (image.access === 'consent') {
        consent = { image, prompt: await catalog.consentPrompt(image.id) };
        return;
      }
      await catalog.activate(image.id);
      controller?.show(image.id);
    } catch (error) {
      if (typeof error === 'object' && error !== null && (error as { kind?: string }).kind === 'needs_consent') {
        await load({ ...image, access: 'consent' });
        return;
      }
      app.fail(error);
    }
  }

  async function answer(scope: 'once' | 'always' | null): Promise<void> {
    const asked = consent;
    consent = null;
    if (!asked || scope === null) {
      return;
    }
    try {
      const granted = await catalog.grant(asked.image.id, scope);
      for (const id of granted.activated) {
        controller?.show(id);
      }
    } catch (error) {
      app.fail(error);
    }
  }
</script>

<div class={['prose', long && !expanded && 'collapsed']} {lang} bind:this={root}></div>
{#if description.truncated}
  <p class="note">{t('catalog-description-truncated')}</p>
{/if}
{#if long}
  <button type="button" class="dl-link more" aria-expanded={expanded ? 'true' : 'false'} onclick={() => (expanded = !expanded)}
    >{expanded ? t('catalog-description-less') : t('catalog-description-more')}</button
  >
{/if}
{#if consent}
  <ImageConsent
    host={consent.prompt.host}
    prechecked={consent.prompt.always_allow_prechecked}
    onanswer={(scope) => void answer(scope)}
  />
{/if}

<style>
  /* The design's description: body text in muted ink, 10px between blocks; headings are demoted so they never
     outshine the mod's name (S3). */
  .prose {
    display: grid;
    gap: 10px;
    font: var(--type-body);
    color: var(--ink-muted);
    overflow-wrap: anywhere;
  }

  .collapsed {
    max-height: 480px;
    overflow: hidden;
    mask-image: linear-gradient(to bottom, #000 80%, transparent);
  }

  .prose :global(p),
  .prose :global(ul),
  .prose :global(ol),
  .prose :global(pre),
  .prose :global(blockquote),
  .prose :global(table) {
    margin: 0;
  }

  .prose :global(ul),
  .prose :global(ol) {
    display: grid;
    gap: 4px;
    padding-left: 20px;
  }

  .prose :global(:is(h1, h2, h3, h4, h5, h6)) {
    margin: 4px 0 0;
    font: var(--type-body-strong);
    color: var(--ink);
  }

  .prose :global(:is(strong, b)) {
    font-weight: 600;
    color: var(--ink);
  }

  .prose :global(code) {
    font: var(--type-mono);
    color: var(--ink);
  }

  .prose :global(pre) {
    padding: var(--space-2) var(--space-3);
    font: var(--type-mono);
    color: var(--ink);
    white-space: pre-wrap;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: var(--radius-sm);
  }

  .prose :global(hr) {
    width: 100%;
    margin: 4px 0;
    border: 0;
    border-top: 1px solid var(--line);
  }

  .prose :global(blockquote) {
    padding-left: var(--space-3);
    border-left: 2px solid var(--line-strong);
  }

  .prose :global(table) {
    border-collapse: collapse;
    font: var(--type-body-sm);
  }

  .prose :global(:is(th, td)) {
    padding: 4px 8px;
    text-align: left;
    border: 1px solid var(--line);
  }

  .prose :global(.dl-green) {
    color: var(--success);
  }

  .prose :global(.dl-red) {
    color: var(--danger);
  }

  .prose :global(.dl-image),
  .prose :global(.dl-youtube) {
    display: block;
  }

  /* The design's image placeholder: a dashed 16:9 box naming the host. */
  .prose :global(.dl-image__load) {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    width: 100%;
    max-width: 530px;
    aspect-ratio: 16 / 9;
    padding: 12px;
    font: var(--type-body-sm);
    color: var(--ink-muted);
    text-align: center;
    cursor: pointer;
    background: var(--surface-sunken);
    border: 1px dashed var(--line-strong);
    border-radius: 8px;
  }

  .prose :global(.dl-image__load b) {
    font-weight: 600;
    color: var(--ink);
  }

  .prose :global(.dl-image__figure) {
    display: grid;
    gap: 4px;
    max-width: 530px;
    margin: 0;
  }

  .prose :global(.dl-image__figure img) {
    display: block;
    max-width: 100%;
    max-height: 600px;
    object-fit: contain;
    border-radius: 8px;
    background: var(--surface-sunken);
  }

  /* Loading: the placeholder's 16:9 box as a skeleton with the spinner in its middle, the picture drawn over both. */
  .prose :global(.dl-image__frame) {
    display: block;
  }

  .prose :global(.dl-image__frame[aria-busy='true']) {
    position: relative;
    width: 100%;
    max-width: 530px;
    aspect-ratio: 16 / 9;
  }

  .prose :global(.dl-image__frame[aria-busy='true'] .dl-skel) {
    position: absolute;
    inset: 0;
    border-radius: 8px;
  }

  .prose :global(.dl-image__frame[aria-busy='true'] .dl-spinner) {
    position: absolute;
    inset: 0;
    margin: auto;
    color: var(--ink-subtle);
  }

  .prose :global(.dl-image__frame[aria-busy='true'] img) {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    background: none;
  }

  .prose :global(.dl-image__figure figcaption) {
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .prose :global(.dl-youtube__card),
  .prose :global(.dl-spoiler__toggle) {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    max-width: 530px;
    padding: 8px 12px;
    font: var(--type-body-sm);
    color: var(--ink);
    text-align: left;
    cursor: pointer;
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 8px;
  }

  .prose :global(.dl-youtube__text) {
    display: grid;
    gap: 2px;
  }

  .prose :global(.dl-youtube__text .dl-mono) {
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .prose :global(.dl-spoiler__body) {
    display: grid;
    gap: 10px;
    margin-top: 8px;
    padding-left: var(--space-3);
    border-left: 2px solid var(--line-strong);
  }

  .prose :global(.dl-spoiler__body[hidden]) {
    display: none;
  }

  .prose :global(:is(.dl-image__load, .dl-youtube__card, .dl-spoiler__toggle):hover) {
    background: var(--surface-hover);
  }

  .prose :global(:is(.dl-image__load, .dl-youtube__card, .dl-spoiler__toggle):focus-visible) {
    outline: 2px solid var(--focus);
    outline-offset: 2px;
  }

  .note {
    margin: 0;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .more {
    justify-self: start;
    padding: 0;
    font: var(--type-body-sm);
    background: none;
    border: 0;
  }
</style>
