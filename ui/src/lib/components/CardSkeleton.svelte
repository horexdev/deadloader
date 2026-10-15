<!-- A catalog card placeholder with the real card's geometry (design: Skeleton.md, CatalogLoading). It is hidden from
     assistive technology: announce loading once with a status line next to the grid. -->
<script lang="ts">
  interface Props {
    /** Position in the grid; picks one of the design's width patterns so neighbours differ. */
    index?: number;
  }

  const { index = 0 }: Props = $props();

  // Title line 1, title line 2 (0 = one-line title), author line, in % (the design's CatalogLoading patterns).
  const patterns = [
    [92, 40, 58],
    [84, 0, 46],
    [90, 66, 62],
    [76, 0, 40],
    [88, 52, 70],
    [80, 0, 50],
    [94, 44, 56],
    [70, 0, 64],
    [86, 58, 42],
    [92, 0, 60],
    [78, 48, 52],
    [88, 0, 44],
  ] as const;

  const widths = $derived(patterns[Math.abs(Math.trunc(index)) % patterns.length] ?? patterns[0]);
</script>

<div class="dl-card skeleton" aria-hidden="true">
  <span class="dl-skel dl-skel--thumb"></span>
  <div class="dl-card__body">
    <div>
      <span class="dl-skel dl-skel--title" style:width={`${widths[0]}%`}></span>
      <span class={['dl-skel dl-skel--title', widths[1] === 0 && 'dl-skel--hidden']} style:width={`${widths[1]}%`}
      ></span>
    </div>
    <span class="dl-skel dl-skel--text" style:width={`${widths[2]}%`}></span>
    <div class="meta"><span class="dl-skel dl-skel--text short"></span></div>
  </div>
</div>

<style>
  .skeleton {
    cursor: default;
  }

  .dl-skel--hidden {
    visibility: hidden;
  }

  .meta {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .short {
    width: 40px;
  }
</style>
