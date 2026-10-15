<!-- E1, the blocking screen (decision 57, design: Wizard.dc.html board WizardNoWrite): deadloader can't write next to
     its exe, so it says what happened, why and what to do, and offers Open folder and Exit. Nothing else runs. -->
<script lang="ts">
  import Text from '../lib/components/Text.svelte';
  import Check from '@lucide/svelte/icons/check';
  import Copy from '@lucide/svelte/icons/copy';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import FolderX from '@lucide/svelte/icons/folder-x';
  import LogOut from '@lucide/svelte/icons/log-out';
  import Button from '../lib/components/Button.svelte';
  import PathTooltip from '../lib/components/PathTooltip.svelte';
  import { shell, startup } from '../lib/api';
  import { t } from '../i18n';
  import TitleBar from '../shell/TitleBar.svelte';
  import './wizard.css';

  interface Props {
    folder: string;
    detail: string;
  }

  const { folder, detail }: Props = $props();

  let copied = $state(false);
  let opened = $state(false);

  function copy(): void {
    navigator.clipboard
      .writeText(detail)
      .then(() => (copied = true))
      .catch(() => undefined);
  }

  function open(): void {
    startup
      .openFolder()
      .then(() => (opened = true))
      .catch(() => undefined);
  }
</script>

<div class="wz-window dl-root">
  <TitleBar />
  <div class="wz-stage">
    <div class="wz-nw" role="alert" aria-labelledby="wz-nw-title">
      <div class="wz-nw-icon"><FolderX class="dl-icon" /></div>
      <h1 class="wz-title" id="wz-nw-title"><Text value={t('wizard-nw-title')} /></h1>
      <p>{t('wizard-nw-why-a')} <span class="wz-code">deadloader.exe</span> {t('wizard-nw-why-b')}</p>
      <div class="wz-well">
        <span class="wz-well__label"><Text value={t('wizard-nw-path')} /></span>
        <div class="wz-nw-path"><PathTooltip path={folder} /></div>
      </div>
      <div class="wz-nw-do">
        <h2>{t('wizard-nw-do')}</h2>
        <ol>
          <li><Text value={t('wizard-nw-s1')} /></li>
          <li>{t('wizard-nw-s2a')} <span class="wz-code">D:\Games\deadloader</span>{t('wizard-nw-s2b')}</li>
          <li>{t('wizard-nw-s3a')} <span class="wz-code">deadloader.exe</span> {t('wizard-nw-s3b')}</li>
        </ol>
      </div>
      <p class="wz-small"><Text value={t('wizard-nw-why-folder')} /></p>
      <div class="dl-dialog__details wz-nw-details">
        <span>{t('wizard-nw-details')}</span><span class="dl-mono">{detail}</span>
        <Button
          variant="ghost"
          size="sm"
          iconOnly
          icon={Copy}
          label={t('wizard-nw-copy')}
          title={t('wizard-nw-copy')}
          onclick={copy}
        />
        {#if copied}<span class="wz-ack" role="status"><Check class="dl-icon" />{t('wizard-nw-copied')}</span>{/if}
        <span class="wz-nw-full"><Text value={t('wizard-nw-no-log')} /></span>
      </div>
      <div class="wz-nw-actions">
        {#if opened}<span class="wz-ack" role="status"><Check class="dl-icon" />{t('wizard-opened')}</span>{/if}
        <span class="wz-grow"></span>
        <Button size="lg" icon={FolderOpen} label={t('wizard-open-folder')} onclick={open} />
        <Button variant="primary" size="lg" icon={LogOut} label={t('wizard-exit')} onclick={() => void shell.close()} />
      </div>
    </div>
  </div>
</div>
