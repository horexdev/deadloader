<!-- D2 Description image from a new host (decision 122; design: ModPage, board DialogD2): which host the picture comes
     from, that the request shows in Activity, and "always allow images from this host" (checked by default, a
     setting). -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import ImageIcon from '@lucide/svelte/icons/image';
  import Button from '../../lib/components/Button.svelte';
  import Checkbox from '../../lib/components/Checkbox.svelte';
  import Dialog from '../../lib/components/Dialog.svelte';
  import type { Scope } from '../../lib/api';
  import { t } from '../../i18n';

  interface Props {
    host: string;
    prechecked: boolean;
    onanswer: (scope: Scope | null) => void;
  }

  const { host, prechecked, onanswer }: Props = $props();

  // The setting's default for the box; the user's tick decides from here.
  // svelte-ignore state_referenced_locally
  let always = $state(prechecked);
</script>

<Dialog title={t('catalog-image-title')} icon={ImageIcon} size="md" oncancel={() => onanswer(null)}>
  <p><Text value={t('catalog-image-body-before')} /> <b class="dl-mono host">{host}</b>{t('catalog-image-body-after')}</p>
  <div class="dl-url"><b>{host}</b></div>
  <p class="note">{t('catalog-image-note')}</p>
  <Checkbox label={t('catalog-image-always')} bind:checked={always} />
  {#snippet footer()}
    <Button label={t('catalog-image-cancel')} onclick={() => onanswer(null)} />
    <Button
      label={t('catalog-image-load')}
      variant="primary"
      icon={ImageIcon}
      data-autofocus
      onclick={() => onanswer(always ? 'always' : 'once')}
    />
  {/snippet}
</Dialog>

<style>
  .host {
    color: var(--ink);
  }

  .note {
    font: var(--type-caption);
    color: var(--ink-subtle);
  }
</style>
