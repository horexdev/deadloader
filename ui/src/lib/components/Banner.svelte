<!-- Banner (design: Banner.md): a condition that lasts until it is resolved, at the top of the area it affects.
     One per area; no dismiss button. -->
<script lang="ts">
  import Text from './Text.svelte';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Info from '@lucide/svelte/icons/info';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import type { Snippet } from 'svelte';
  import type { IconComponent } from './types';

  export type BannerKind = 'info' | 'neutral' | 'warning' | 'error' | 'success';

  interface Props {
    kind?: BannerKind;
    /** The bold lead: what is the case. */
    title: string;
    /** Why, or what happens next. */
    text?: string;
    /** Overrides the kind's default icon (e.g. wifi-off for offline, unplug for local mode). */
    icon?: IconComponent;
    /** Buttons on the right; they wrap below the text on narrow windows. */
    actions?: Snippet;
  }

  const { kind = 'info', title, text, icon, actions }: Props = $props();

  const defaults: Record<BannerKind, IconComponent> = {
    info: Info,
    neutral: Info,
    warning: TriangleAlert,
    error: CircleX,
    success: CircleCheck,
  };
  const Icon = $derived(icon ?? defaults[kind]);
</script>

<div class={['dl-banner', kind !== 'info' && `dl-banner--${kind}`]} role={kind === 'error' ? 'alert' : 'status'}>
  <Icon class="dl-icon dl-icon--md dl-banner__icon" />
  <span class="dl-banner__text"
    ><strong><Text value={title} /></strong> {#if text}<span class="dl-banner__why"><Text value={text} /></span>{/if}</span
  >
  {#if actions}<div class="dl-banner__actions">{@render actions()}</div>{/if}
</div>
