<!-- PathTooltip (design: PathTooltip.md): a long path on one line, the folder part ellipsized and the last segment
     always visible, with the full path in a tooltip on hover and keyboard focus (CSS only; Esc hides it). -->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import FieldError from './FieldError.svelte';
  import { splitPath } from './tooltip';

  interface Props {
    /** The full path, already masked when the "hide user name" setting is on. */
    path: string;
    /** E.g. "This folder no longer exists."; shown under the path. */
    error?: string;
    /** Copy, Open folder, Change: next to the path, never inside the tooltip. */
    actions?: Snippet;
  }

  const { path, error, actions }: Props = $props();

  const uid = $props.id();
  const tipId = `${uid}-tip`;
  const errorId = `${uid}-error`;
  const parts = $derived(splitPath(path));
  let suppressed = $state(false);

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      suppressed = true;
    }
  }
</script>

<span class={['dl-path', error && 'is-error']}>
  <!-- Focusable on purpose, so keyboard users get the full path in the tooltip (design: PathTooltip.md); Esc hides it. -->
  <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_static_element_interactions -->
  <span
    class="dl-path__text"
    tabindex="0"
    aria-describedby={error ? `${tipId} ${errorId}` : tipId}
    onkeydown={handleKeydown}
    onfocusout={() => (suppressed = false)}
    onpointerleave={() => (suppressed = false)}
    ><span class="dl-path__head">{parts.head}</span><span class="dl-path__tail">{parts.tail}</span></span
  >
  <span class="dl-path__tip" role="tooltip" id={tipId} style:display={suppressed ? 'none' : undefined}>{path}</span>
</span>
{@render actions?.()}
{#if error}<FieldError id={errorId} message={error} />{/if}
