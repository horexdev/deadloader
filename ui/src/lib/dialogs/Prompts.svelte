<!-- Shows the questions asked through `prompts` (D1, D3); mounted once by the shell, above every screen. -->
<script lang="ts">
  import { prompts } from '../prompts.svelte';
  import AgeDialog from './AgeDialog.svelte';
  import LinkDialog from './LinkDialog.svelte';
</script>

{#if prompts.link}
  {#key prompts.link}
    <LinkDialog url={prompts.link.url} onanswer={(answer) => prompts.answerLink(answer)} />
  {/key}
{/if}
{#if prompts.age}
  <AgeDialog mode={prompts.age.mode} onanswer={(confirmed) => prompts.answerAge(confirmed)} />
{/if}
