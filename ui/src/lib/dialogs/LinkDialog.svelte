<!-- D1 External link (decision 31; design: ModPage, board DialogD1): the full address with its host highlighted,
     Open / Cancel, and "always open links immediately". -->
<script lang="ts">
  import ExternalLink from '@lucide/svelte/icons/external-link';
  import Button from '../components/Button.svelte';
  import Checkbox from '../components/Checkbox.svelte';
  import Dialog from '../components/Dialog.svelte';
  import { splitUrl } from '../links';
  import type { LinkAnswer } from '../prompts.svelte';
  import { t } from '../../i18n';

  interface Props {
    url: string;
    onanswer: (answer: LinkAnswer) => void;
  }

  const { url, onanswer }: Props = $props();

  let always = $state(false);
  const parts = $derived(splitUrl(url));
</script>

<Dialog title={t('catalog-link-title')} icon={ExternalLink} oncancel={() => onanswer({ open: false, always: false })}>
  <p>{t('catalog-link-body')}</p>
  <div class="dl-url">{parts.before}<b>{parts.host}</b>{parts.after}</div>
  <Checkbox label={t('catalog-link-always')} bind:checked={always} />
  {#snippet footer()}
    <Button label={t('catalog-link-cancel')} onclick={() => onanswer({ open: false, always: false })} />
    <Button
      label={t('catalog-link-open')}
      variant="primary"
      icon={ExternalLink}
      data-autofocus
      onclick={() => onanswer({ open: true, always })}
    />
  {/snippet}
</Dialog>
