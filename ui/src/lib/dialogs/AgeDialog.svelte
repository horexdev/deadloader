<!-- D3 NSFW age confirmation (decision 109; design: SettingsScreen, board SettingsD3): asked once, when mods with
     content ratings are first set to blurred or shown. -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import EyeOff from '@lucide/svelte/icons/eye-off';
  import Button from '../components/Button.svelte';
  import Dialog from '../components/Dialog.svelte';
  import type { AgePrompt } from '../prompts.svelte';
  import { t } from '../../i18n';

  interface Props {
    mode: AgePrompt['mode'];
    onanswer: (confirmed: boolean) => void;
  }

  const { mode, onanswer }: Props = $props();
</script>

<Dialog title={t('catalog-age-title')} icon={EyeOff} oncancel={() => onanswer(false)}>
  <p>{t('catalog-age-body')}</p>
  <p>{t('catalog-age-choice', { mode: t(`catalog-nsfw-${mode}`) })}</p>
  <p class="once"><Text value={t('catalog-age-once')} /></p>
  {#snippet footer()}
    <Button label={t('catalog-age-cancel')} variant="ghost" onclick={() => onanswer(false)} />
    <Button label={t('catalog-age-confirm')} variant="primary" data-autofocus onclick={() => onanswer(true)} />
  {/snippet}
</Dialog>

<style>
  /* The design's .st-meta: the quiet line under the question. */
  .once {
    font: var(--type-body-sm);
    color: var(--ink-subtle);
  }
</style>
