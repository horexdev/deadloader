<!-- CatalogCard (design: CatalogCard.md): one GameBanana mod in the virtualized catalog grid. The title link is
     stretched over the whole card, so the card is one click target with one focus ring; the favourite and quick
     install buttons sit above it. Its size comes from the grid cell (16:9 thumbnail + fixed info block). -->
<script lang="ts">
  import { tooltip } from './tooltip';
  import Download from '@lucide/svelte/icons/download';
  import EyeOff from '@lucide/svelte/icons/eye-off';
  import Heart from '@lucide/svelte/icons/heart';
  import ImageIcon from '@lucide/svelte/icons/image';
  import RefreshCw from '@lucide/svelte/icons/refresh-cw';
  import ThumbsUp from '@lucide/svelte/icons/thumbs-up';
  import WifiOff from '@lucide/svelte/icons/wifi-off';
  import Badge, { type BadgeVariant } from './Badge.svelte';
  import Button from './Button.svelte';
  import Spinner from './Spinner.svelte';
  import type { IconComponent } from './types';

  export interface CardBadge {
    icon: IconComponent;
    label: string;
    variant: BadgeVariant;
  }

  export interface QuickInstall {
    /** Accessible name and tooltip: "Install", "Installing, 62 %", "Retry". */
    label: string;
    busy?: boolean;
    /** The install failed: the button becomes Retry. */
    error?: boolean;
    disabled?: boolean;
  }

  export interface SensitiveThumb {
    /** The tiny downscaled thumbnail, drawn stretched (no CSS blur). */
    blurredThumb: string;
    /** The generic "Sensitive content"; cards never name a rating. */
    label: string;
    /** "Click to show". */
    hint: string;
  }

  interface Props {
    href: string;
    title: string;
    /** "author · category". */
    byline: string;
    /** Thumbnail URL (pick the 220 or 530 px variant with `thumbVariant`); null shows the missing-image box. */
    thumb?: string | null;
    /** Visible like count, locale-formatted. */
    likes: string;
    /** Spoken and shown as tooltip, e.g. "221 likes". */
    likesLabel: string;
    /** Relative date on the card, exact date as tooltip. */
    date: string;
    dateTitle: string;
    /** Installed version, update available and so on (at most two). */
    badges?: CardBadge[];
    /** Shows the favourite button when set; it stays visible while true. */
    favourite?: boolean;
    favouriteLabel?: string;
    /** Only for mods with exactly one file. */
    quickInstall?: QuickInstall;
    /** "Installing · 62 %" in the meta line. */
    progressText?: string;
    /** Offline and not saved: dims the card, says why in the meta line and stops the link. */
    offline?: string;
    error?: boolean;
    /** Blurred tile until revealed for this session. */
    nsfw?: SensitiveThumb | null;
    /** -1 when another card holds the grid's tab stop. */
    tabindex?: 0 | -1;
    onopen?: (event: MouseEvent) => void;
    onreveal?: () => void;
    onfavourite?: () => void;
    oninstall?: () => void;
    /** The thumbnail could not be loaded (e.g. the core forgot its picture id: fetch the page again). */
    onthumberror?: () => void;
  }

  const {
    href,
    title,
    byline,
    thumb = null,
    likes,
    likesLabel,
    date,
    dateTitle,
    badges = [],
    favourite,
    favouriteLabel = '',
    quickInstall,
    progressText,
    offline,
    error = false,
    nsfw = null,
    tabindex = 0,
    onopen,
    onreveal,
    onfavourite,
    oninstall,
    onthumberror,
  }: Props = $props();

  let failedSrc: string | null = $state(null);
  let loadedSrc: string | null = $state(null);
  const src = $derived(nsfw ? nsfw.blurredThumb : thumb);
  const missing = $derived(src === null || src === failedSrc);
  // The thumbnail's skeleton and spinner sit under the picture until it has loaded; then they leave the DOM, so
  // nothing turns while idle.
  const loading = $derived(!missing && src !== loadedSrc);

  function handleLinkClick(event: MouseEvent) {
    if (offline) {
      event.preventDefault();
      return;
    }
    if (nsfw) {
      event.preventDefault();
      onreveal?.();
      return;
    }
    onopen?.(event);
  }
</script>

<!-- aria-disabled on the card is the design stylesheet's hook for the dimmed image; the link carries it for AT. -->
<!-- svelte-ignore a11y_role_supports_aria_props_implicit -->
<article class={['dl-card', error && 'is-error']} aria-disabled={offline ? 'true' : undefined}>
  <div class={['dl-card__thumb', nsfw && 'dl-card__thumb--blurred']} aria-busy={loading ? 'true' : undefined}>
    {#if missing}
      <div class="dl-card__thumb-missing"><ImageIcon class="dl-icon dl-icon--lg" /></div>
    {:else}
      {#if loading}
        <span class="dl-skel dl-skel--thumb placeholder" aria-hidden="true"></span>
        <Spinner class="placeholder-spinner" />
      {/if}
      <img
        src={src ?? undefined}
        alt=""
        loading="lazy"
        decoding="async"
        onload={(event) => (loadedSrc = event.currentTarget.getAttribute('src'))}
        onerror={(event) => {
          failedSrc = event.currentTarget.getAttribute('src');
          onthumberror?.();
        }}
      />
    {/if}
    {#if nsfw}
      <div class="dl-card__nsfw"><span><EyeOff class="dl-icon" /> {nsfw.label}</span><small>{nsfw.hint}</small></div>
    {:else if favourite !== undefined}
      <div class="dl-card__overlay">
        <button
          type="button"
          class="dl-card__fav"
          aria-pressed={favourite ? 'true' : 'false'}
          aria-label={favouriteLabel}
          {@attach tooltip({ text: favouriteLabel ?? '' })}
          {tabindex}
          onclick={onfavourite}><Heart class="dl-icon" /></button
        >
      </div>
    {/if}
    {#if badges.length > 0}
      <div class="dl-card__badges">
        {#each badges as badge (badge.label)}<Badge icon={badge.icon} label={badge.label} variant={badge.variant} />{/each}
      </div>
    {/if}
  </div>
  <div class="dl-card__body">
    <h3 class="dl-card__title">
      <a
        class="dl-card__link"
        {href}
        {tabindex}
        data-grid-focus
        aria-disabled={offline ? 'true' : undefined}
        onclick={handleLinkClick}>{title}</a
      >
    </h3>
    <div class="dl-card__author">{byline}</div>
    <div class="dl-card__meta">
      <span {@attach tooltip({ text: likesLabel ?? '' })}
        ><ThumbsUp class="dl-icon" /><span aria-hidden="true">{likes}</span><span class="dl-visually-hidden"
          >{likesLabel}</span
        ></span
      >
      <span {@attach tooltip({ text: dateTitle ?? '' })}>{date}</span>
      {#if offline}<span><WifiOff class="dl-icon" />{offline}</span>{/if}
      {#if progressText}<span>{progressText}</span>{/if}
      {#if quickInstall}
        <Button
          class="dl-card__install dl-card__quick"
          variant="primary"
          size="sm"
          iconOnly
          icon={quickInstall.error ? RefreshCw : Download}
          label={quickInstall.label}
          title={quickInstall.label}
          busy={quickInstall.busy}
          error={quickInstall.error}
          disabled={quickInstall.disabled || Boolean(offline)}
          {tabindex}
          onclick={oninstall}
        />
      {/if}
    </div>
  </div>
</article>

<style>
  /* Loading (design: Skeleton.md, the card's `--thumb` block): the skeleton fills the 16:9 box with the spinner in
     its middle, and the picture is drawn over both as soon as it has pixels. */
  .placeholder {
    position: absolute;
    inset: 0;
    min-width: 0;
  }

  .dl-card__thumb :global(.placeholder-spinner) {
    position: absolute;
    inset: 0;
    margin: auto;
    color: var(--ink-subtle);
  }

  .dl-card__thumb img {
    position: relative;
  }
</style>
