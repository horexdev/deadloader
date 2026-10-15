<!-- D11 Take a foreign file under management (decision 16, F9): what will happen — copied into the library as a local
     mod, the original replaced by deadloader's own linked copy, enabled at the bottom of the profile — and its name. -->
<script lang="ts">
  import File from '@lucide/svelte/icons/file';
  import type { ForeignItem } from '../../lib/api';
  import { Button, Dialog, TextInput } from '../../lib/components';
  import { formatSize, t } from '../../i18n';
  import { heroName } from './model';

  interface Props {
    item: ForeignItem;
    profile: string;
    busy?: boolean;
    onconfirm: (name: string) => void;
    oncancel: () => void;
  }

  const { item, profile, busy = false, onconfirm, oncancel }: Props = $props();

  function suggested(): string {
    const stem = item.name.replace(/\.vpk$/i, '');
    return item.vpk?.hero ? t('library-d11-suggested', { file: stem, hero: heroName(item.vpk.hero) }) : stem;
  }

  let name = $state(suggested());
</script>

<Dialog title={t('library-d11-title', { name: item.name })} icon={File} {oncancel}>
  <p>{t('library-d11-lead')}</p>
  <ol class="steps">
    <li>{t('library-d11-step-copy', { size: formatSize(item.bytes) })}</li>
    <li>{t('library-d11-step-replace')}</li>
    <li>{t('library-d11-step-enable', { profile })}</li>
  </ol>
  <TextInput label={t('library-d11-name')} hint={t('library-d11-hint')} bind:value={name} />
  {#snippet footer()}
    <Button label={t('library-cancel')} onclick={oncancel} />
    <Button variant="primary" label={t('library-take')} {busy} onclick={() => onconfirm(name.trim())} />
  {/snippet}
</Dialog>

<style>
  /* From the design's D11. */
  .steps {
    margin: 0;
    padding-left: 20px;
    display: grid;
    gap: 4px;
    color: var(--ink);
  }
</style>
