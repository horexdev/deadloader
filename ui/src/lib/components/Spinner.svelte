<!-- The design's spinner (Lucide loader-circle at stroke 2.25). It turns only while it is in the DOM, so render it only
     while work is happening; reduced motion stops it (components.css). -->
<script lang="ts">
  interface Props {
    class?: string;
  }

  const { class: className }: Props = $props();
</script>

<svg
  class={['dl-spinner', className]}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2.25"
  stroke-linecap="round"
  aria-hidden="true"><path d="M21 12a9 9 0 1 1-6.219-8.56"></path></svg>
