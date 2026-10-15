<!-- Count (design: Badge.md, `.dl-count`): the numeric pill. The accent one is only for pending changes. -->
<script lang="ts">
  interface Props {
    /** The number, already formatted for the locale. */
    value: string;
    muted?: boolean;
  }

  const { value, muted = false }: Props = $props();
</script>

<span class={['dl-count', muted && 'dl-count--muted']}>{value}</span>
