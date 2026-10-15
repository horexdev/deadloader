<!-- EmptyState (design: EmptyState.md): fills an area that has nothing to show and offers the next step. -->
<script lang="ts">
  import Text from './Text.svelte';
  import type { Snippet } from 'svelte';
  import type { IconComponent } from './types';

  interface Props {
    icon: IconComponent;
    title: string;
    text?: string;
    /** 48px disc and a smaller title, for panels and tabs. */
    compact?: boolean;
    /** Danger-tinted disc for "not found", "removed" and "could not load". */
    error?: boolean;
    /** Heading level of the title. */
    level?: 2 | 3 | 4;
    actions?: Snippet;
  }

  const { icon: Icon, title, text, compact = false, error = false, level = 3, actions }: Props = $props();
</script>

<div class={['dl-empty', compact && 'dl-empty--compact', error && 'dl-empty--error']}>
  <div class="dl-empty__glyph"><Icon class="dl-icon dl-icon--lg" /></div>
  <svelte:element this={`h${level}`} class="dl-empty__title"><Text value={title} /></svelte:element>
  {#if text}<p class="dl-empty__text"><Text value={text} /></p>{/if}
  {#if actions}<div class="dl-empty__actions">{@render actions()}</div>{/if}
</div>
