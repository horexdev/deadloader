<!-- Skeleton (design: Skeleton.md): a static placeholder block, no shimmer. -->
<script lang="ts">
  export type SkeletonKind = 'block' | 'thumb' | 'title' | 'text' | 'circle';

  interface Props {
    kind?: SkeletonKind;
    /** CSS width, e.g. "60%" or "40px"; vary widths so placeholders don't read as content. */
    width?: string;
    /** CSS height, for blocks and circles. */
    height?: string;
  }

  const { kind = 'block', width, height }: Props = $props();
</script>

<span class={['dl-skel', kind !== 'block' && `dl-skel--${kind}`]} style:width style:height aria-hidden="true"></span>
