<!-- Settings → About: version and build commit (158), licence, how to verify this download (91), the window engine and
     what is switched off (ADR 0001), the data folder, and the first-run notice to read again (111). -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import Copy from '@lucide/svelte/icons/copy';
  import ExternalLink from '@lucide/svelte/icons/external-link';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import Info from '@lucide/svelte/icons/info';
  import icon from '../../assets/brand/icon-32.svg';
  import Button from '../../lib/components/Button.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import { maintenance, type About } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { openLink } from '../../lib/links';
  import { notice } from '../../lib/notice.svelte';
  import { LANGUAGE_NAMES, formatDate, formatSize, isLang, t } from '../../i18n';
  import Group from './Group.svelte';
  import Row from './Row.svelte';
  import type { SettingsData } from './model';

  interface Props {
    app: AppState;
    data: SettingsData;
    about: About | null;
  }

  const { app, data, about }: Props = $props();

  const REPO = 'https://github.com/horexdev/deadloader';
  const HASH_COMMAND = String.raw`Get-FileHash .\deadloader.exe -Algorithm SHA256`;
  const ATTEST_COMMAND = 'gh attestation verify deadloader.exe --repo horexdev/deadloader';

  function link(url: string): void {
    openLink(url).catch((error: unknown) => app.fail(error));
  }

  function copy(value: string): void {
    navigator.clipboard
      .writeText(value)
      .then(() => app.toast({ kind: 'info', title: t('settings-copied') }))
      .catch((error: unknown) => app.fail(error));
  }

  const ack = $derived(data.acknowledgements.notice);
  const noticeHelp = $derived(
    ack
      ? t('settings-notice-help', {
          version: ack.version,
          date: formatDate(new Date(ack.at)),
          language: isLang(ack.language) ? LANGUAGE_NAMES[ack.language] : ack.language,
          catalog: data.catalog.enabled ? t('settings-on') : t('settings-off'),
          updates: data.general.check_app_updates ? t('settings-on') : t('settings-off'),
        })
      : t('settings-notice-none'),
  );
</script>

<div class="st-card st-about-head">
  <img src={icon} alt="" width="48" height="48" class="st-about-icon" />
  <div class="st-about-name">
    <div class="st-about-title">
      deadloader <span class="st-about-version">{t('settings-version', { version: about?.version ?? '' })}</span>
    </div>
    <div class="st-meta st-head-row">
      {#if about?.commit}
        {t('settings-commit')} <span class="dl-mono st-mono-ink">{about.commit.slice(0, 7)}</span>
        <Button
          variant="ghost"
          size="sm"
          iconOnly
          icon={Copy}
          label={t('settings-copy-commit')}
          title={t('settings-copy-commit')}
          onclick={() => copy(about?.commit ?? '')}
        />
      {:else}
        {t('settings-commit-local')}
      {/if}
    </div>
  </div>
  <button type="button" class="dl-link st-linkbtn" onclick={() => link(`${REPO}/releases/tag/v${about?.version ?? ''}`)}
    >{t('settings-release-page')}<ExternalLink class="dl-icon" /></button
  >
</div>

<Group title={t('settings-licence-group')}>
  <Row label="GNU General Public License v3.0" help={t('settings-licence-help')}>
    {#snippet control()}
      <div class="st-actions st-gap16">
        <button type="button" class="dl-link st-linkbtn" onclick={() => link(`${REPO}/blob/main/LICENSE`)}
          >{t('settings-read-licence')}<ExternalLink class="dl-icon" /></button
        >
        <button type="button" class="dl-link st-linkbtn" onclick={() => link(REPO)}
          >{t('settings-source')}<ExternalLink class="dl-icon" /></button
        >
      </div>
    {/snippet}
  </Row>
</Group>

<Group title={t('settings-verify-group')}>
  <div class="st-row st-gap12">
    <p class="st-row__help st-row__full">{t('settings-verify-lead')}</p>
    <ol class="st-steps st-row__full">
      <li>
        <span class="st-stepnum" aria-hidden="true">1</span>
        <div class="st-step">
          <div class="st-row__label">{t('settings-verify-hash')}</div>
          <p class="st-row__help">{t('settings-verify-hash-help')}</p>
          {#if about?.exe_sha256}
            <div class="st-hash">
              <span>{about.exe_sha256}</span>
              <Button
                variant="ghost"
                size="sm"
                iconOnly
                icon={Copy}
                label={t('settings-copy-hash')}
                title={t('settings-copy-hash')}
                onclick={() => copy(about?.exe_sha256 ?? '')}
              />
            </div>
          {/if}
          <p class="st-row__help">{t('settings-verify-hash-cmd')}</p>
          <div class="st-hash">
            <span>{HASH_COMMAND}</span>
            <Button
              variant="ghost"
              size="sm"
              iconOnly
              icon={Copy}
              label={t('settings-copy-command')}
              title={t('settings-copy-command')}
              onclick={() => copy(HASH_COMMAND)}
            />
          </div>
        </div>
      </li>
      <li>
        <span class="st-stepnum" aria-hidden="true">2</span>
        <div class="st-step">
          <div class="st-row__label">{t('settings-verify-attest')}</div>
          <p class="st-row__help">{t('settings-verify-attest-help')}</p>
          <div class="st-hash">
            <span>{ATTEST_COMMAND}</span>
            <Button
              variant="ghost"
              size="sm"
              iconOnly
              icon={Copy}
              label={t('settings-copy-command')}
              title={t('settings-copy-command')}
              onclick={() => copy(ATTEST_COMMAND)}
            />
          </div>
        </div>
      </li>
    </ol>
    <div class="st-note st-row__full">
      <Info class="dl-icon" /><span><strong>{t('settings-unsigned')}</strong> {t('settings-unsigned-text')}</span>
    </div>
  </div>
</Group>

<Group title={t('settings-webview-group')}>
  <div class="st-row">
    <div class="st-row__text">
      <div class="st-row__label">
        Microsoft Edge WebView2 <span class="dl-mono st-about-version">{about?.webview2 ?? t('settings-webview-unknown')}</span>
      </div>
      <p class="st-row__help"><Text value={t('settings-webview-help')} /></p>
      <ul class="st-offlist">
        {#each [1, 2, 3, 4, 5, 6] as n (n)}<li>{t(`settings-webview-off-${n}`)}</li>{/each}
      </ul>
      <p class="st-row__help st-mono-ink"><strong class="st-strong">{t('settings-webview-no-route')}</strong></p>
    </div>
  </div>
</Group>

<Group title={t('settings-data-group')}>
  <Row label={t('settings-data-folder')} help={t('settings-data-folder-help')}>
    {#snippet control()}
      <Button
        icon={FolderOpen}
        label={t('settings-open-folder')}
        onclick={() => maintenance.openFolder({ place: 'data' }).catch((error: unknown) => app.fail(error))}
      />
    {/snippet}
    {#snippet full()}
      {#if about}
        <div class="st-meta st-pathline st-pathline--wrap">
          <PathTooltip path={about.data_folder} /><span>· {formatSize(about.data_bytes)}</span>
        </div>
      {/if}
    {/snippet}
  </Row>
  <Row label={t('settings-notice')} help={noticeHelp}>
    {#snippet control()}
      <Button label={t('settings-notice-again')} onclick={() => notice.open({ mode: 'review' })} />
    {/snippet}
  </Row>
</Group>
