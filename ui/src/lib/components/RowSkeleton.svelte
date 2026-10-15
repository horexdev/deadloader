<!-- A library row placeholder with the real row's geometry (design: LibraryLoading). Hidden from assistive technology:
     announce loading once with a status line above the list. -->
<script lang="ts">
  interface Props {
    /** Position in the list; picks one of the design's width patterns. */
    index?: number;
  }

  const { index = 0 }: Props = $props();

  // Name and sub-line widths in % (the design's LibraryLoading patterns).
  const patterns = [
    [44, 26],
    [30, 20],
    [52, 30],
    [36, 22],
    [40, 18],
    [28, 24],
    [48, 28],
    [34, 20],
    [42, 26],
    [30, 22],
  ] as const;

  const widths = $derived(patterns[Math.abs(Math.trunc(index)) % patterns.length] ?? patterns[0]);
</script>

<div class="dl-row skeleton" aria-hidden="true">
  <span></span>
  <span class="dl-skel dl-skel--circle toggle"></span>
  <span class="dl-skel thumb"></span>
  <div class="dl-row__main">
    <span class="dl-skel dl-skel--title" style:width={`${widths[0]}%`}></span>
    <span class="dl-skel dl-skel--text" style:width={`${widths[1]}%`}></span>
  </div>
  <span></span>
  <span class="actions"></span>
</div>

<style>
  .skeleton {
    cursor: default;
  }

  .toggle {
    width: 36px;
    height: 20px;
  }

  .thumb {
    width: 64px;
    height: 36px;
  }

  .actions {
    width: 28px;
  }
</style>
