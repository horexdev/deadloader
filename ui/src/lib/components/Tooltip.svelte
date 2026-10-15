<!-- Tooltip (design: Tooltip.md): a short, non-interactive label. Usually shown through the `tooltip` attachment
     (tooltip.ts), which handles the delay, focus, Esc and placement. Never put links or buttons in it. -->
<script lang="ts">
  interface Props {
    id: string;
    text: string;
    /** Second line in ink-subtle: what to do. */
    sub?: string;
    /** Two lines; on automatically with `sub`. */
    multiline?: boolean;
    /** Positioned by the attachment in window coordinates. */
    floating?: boolean;
  }

  const { id, text, sub, multiline = false, floating = false }: Props = $props();
</script>

<span
  class={['dl-tooltip', (multiline || sub) && 'dl-tooltip--multiline', floating && 'floating']}
  role="tooltip"
  {id}
>
  {text}{#if sub}<span class="dl-tooltip__sub">{sub}</span>{/if}
</span>

<style>
  .floating {
    position: fixed;
    pointer-events: none;
  }

  .dl-tooltip__sub {
    display: block;
  }
</style>
