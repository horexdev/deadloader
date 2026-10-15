<!-- The error line under a control (design: `.dl-field__error`): icon plus a message that says what to do. -->
<script lang="ts">
  import CircleX from '@lucide/svelte/icons/circle-x';

  interface Props {
    id: string;
    message: string;
  }

  const { id, message }: Props = $props();
</script>

<span class="dl-field__error" {id}><CircleX class="dl-icon" /><span>{message}</span></span>
