<!-- S1 step 6, Setup: what deadloader found, each with its action (design: Wizard.dc.html, boards WizardStep6 and
     WizardSetupIssues). Nothing here changes the game; choosing a folder sets the selected profile's game folder. -->
<script lang="ts">
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import CircleQuestionMark from '@lucide/svelte/icons/circle-question-mark';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import Check from '@lucide/svelte/icons/check';
  import Files from '@lucide/svelte/icons/files';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import Info from '@lucide/svelte/icons/info';
  import Link from '@lucide/svelte/icons/link';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import { onMount } from 'svelte';
  import Button from '../lib/components/Button.svelte';
  import PathTooltip from '../lib/components/PathTooltip.svelte';
  import { maintenance, type ForeignFile, type SetupCheck } from '../lib/api';
  import type { AppState } from '../lib/app.svelte';
  import { formatSize, t } from '../i18n';
  import { maskUser } from '../screens/settings/model';

  interface Props {
    app: AppState;
    /** Read only: "Read it again" shows what was found without offering changes. */
    review?: boolean;
  }

  const { app, review = false }: Props = $props();

  let check = $state<SetupCheck | null>(null);
  let gameChosen = $state(false);
  let exeOpened = $state(false);
  let help = $state(false);
  let drives = $state(false);
  let foreign = $state(false);
  let gameinfo = $state(false);

  async function load(): Promise<void> {
    try {
      check = await maintenance.setupCheck();
    } catch (error) {
      app.fail(error);
    }
  }

  onMount(() => {
    void load();
  });

  async function chooseGame(): Promise<void> {
    try {
      const root = await maintenance.gameFolderDialog({ title: t('wizard-browse-title'), filter: '' }, true);
      if (root) {
        gameChosen = true;
        help = false;
        await load();
        await app.reloadLibrary();
      }
    } catch (error) {
      app.fail(error);
    }
  }

  async function openExe(): Promise<void> {
    try {
      await maintenance.openFolder({ place: 'exe' });
      exeOpened = true;
    } catch (error) {
      app.fail(error);
    }
  }

  /** Paths are shown the way the logs write them when the user name is hidden (94). */
  function masked(path: string): string {
    return app.settings?.logs?.mask_user_path === false ? path : maskUser(path);
  }

  /** A foreign file's size, and where it probably came from. */
  function fileMeta(file: ForeignFile): string {
    const size = formatSize(file.bytes);
    return file.from_dmm ? `${size} · ${t('wizard-from-dmm')}` : size;
  }

  const exeWarn = $derived(check !== null && check.exe_location !== 'permanent');
</script>

{#if check === null}
  <p>{t('wizard-checking')}</p>
{:else}
  <div class="wz-list" role="list">
    <div class="wz-row" role="listitem">
      {#if check.game_root}<CircleCheck class="dl-icon wz-ok" />{:else}<CircleX class="dl-icon wz-bad" />{/if}
      <div class="wz-text">
        <div class="wz-row__topic">{t('wizard-topic-game')}</div>
        {#if check.game_root}
          <div class="wz-row__title">{t('wizard-game-found')}</div>
          <div class="wz-row__path"><PathTooltip path={check.game_root} /></div>
          {#if gameChosen}
            <div class="wz-ack wz-mt4" role="status"><Check class="dl-icon" />{t('wizard-game-chosen')}</div>
          {/if}
        {:else}
          <div class="wz-row__title">{t('wizard-game-missing')}</div>
          <div class="wz-row__text">{t('wizard-game-missing-text')}</div>
        {/if}
      </div>
      <div class="wz-row__actions">
        {#if !review}
          {#if check.game_root}
            <Button label={t('wizard-change')} onclick={() => void chooseGame()} />
          {:else}
            <Button
              variant="ghost"
              icon={CircleQuestionMark}
              label={t('wizard-help')}
              aria-expanded={help ? 'true' : 'false'}
              onclick={() => (help = !help)}
            />
            <Button label={t('wizard-browse')} onclick={() => void chooseGame()} />
          {/if}
        {/if}
      </div>
      {#if !check.game_root && help}
        <div class="wz-row__more">
          {t('wizard-help-a')} <span class="wz-code">steamapps\common\Deadlock</span>{t('wizard-help-b')}
        </div>
      {/if}
    </div>

    <div class="wz-row" role="listitem">
      {#if exeWarn}<TriangleAlert class="dl-icon wz-warn" />{:else}<CircleCheck class="dl-icon wz-ok" />{/if}
      <div class="wz-text">
        <div class="wz-row__topic">{t('wizard-topic-exe')}</div>
        <div class="wz-row__title">
          {check.exe_location === 'downloads'
            ? t('wizard-exe-downloads')
            : check.exe_location === 'desktop'
              ? t('wizard-exe-desktop')
              : t('wizard-exe-ok')}
        </div>
        <div class="wz-row__path"><PathTooltip path={masked(check.exe_folder)} /></div>
        {#if exeWarn}
          <div class="wz-row__text">
            {t('wizard-exe-warn-a')} <span class="wz-code">D:\Games\deadloader</span>{t('wizard-exe-warn-b')}
          </div>
          {#if exeOpened}<div class="wz-ack wz-mt4" role="status"><Check class="dl-icon" />{t('wizard-opened')}</div>{/if}
        {/if}
      </div>
      <div class="wz-row__actions">
        {#if exeWarn}<Button icon={FolderOpen} label={t('wizard-open-folder')} onclick={() => void openExe()} />{/if}
      </div>
    </div>

    {#if check.same_volume !== null}
      <div class="wz-row" role="listitem">
        {#if check.same_volume}<Link class="dl-icon wz-ok" />{:else}<Files class="dl-icon wz-info" />{/if}
        <div class="wz-text">
          <div class="wz-row__topic">{t('wizard-topic-disk')}</div>
          {#if check.same_volume}
            <div class="wz-row__title">{t('wizard-linked')}</div>
            <div class="wz-row__text">{t('wizard-linked-text', { drive: check.game_volume ?? '' })}</div>
          {:else}
            <div class="wz-row__title">{t('wizard-copied')}</div>
            <div class="wz-row__text">
              {t('wizard-copied-text', { data: check.data_volume ?? '', game: check.game_volume ?? '' })}
            </div>
          {/if}
        </div>
        <div class="wz-row__actions">
          {#if !check.same_volume}
            <Button variant="ghost" label={t('wizard-learn-more')} aria-expanded={drives ? 'true' : 'false'} onclick={() => (drives = !drives)}>
              <ChevronDown class={drives ? 'dl-icon wz-chev wz-chev--open' : 'dl-icon wz-chev'} />
            </Button>
          {/if}
        </div>
        {#if !check.same_volume && drives}
          <div class="wz-row__more">{t('wizard-drives-more', { game: check.game_volume ?? '' })}</div>
        {/if}
      </div>
    {/if}

    {#if check.game_root || check.foreign.length > 0 || check.gameinfo !== 'missing'}
      <div class="wz-row" role="listitem">
        {#if check.foreign.length > 0}<Info class="dl-icon wz-info" />{:else}<CircleCheck class="dl-icon wz-ok" />{/if}
        <div class="wz-text">
          <div class="wz-row__topic">{t('wizard-topic-foreign')}</div>
          {#if check.foreign.length > 0}
            <div class="wz-row__title">{t('wizard-foreign-some', { count: check.foreign.length })}</div>
            <div class="wz-row__text">{t('wizard-foreign-some-text')}</div>
          {:else}
            <div class="wz-row__title">{t('wizard-foreign-none')}</div>
            <div class="wz-row__text">{t('wizard-foreign-none-text')}</div>
          {/if}
        </div>
        <div class="wz-row__actions">
          {#if check.foreign.length > 0}
            <Button
              variant="ghost"
              label={foreign ? t('wizard-hide-files') : t('wizard-show-files')}
              aria-expanded={foreign ? 'true' : 'false'}
              onclick={() => (foreign = !foreign)}
            >
              <ChevronDown class={foreign ? 'dl-icon wz-chev wz-chev--open' : 'dl-icon wz-chev'} />
            </Button>
          {/if}
        </div>
        {#if check.foreign.length > 0 && foreign}
          <div class="wz-row__more">
            <ul>
              {#each check.foreign as file (file.name)}
                <li>
                  <span>{file.name}</span><span>{fileMeta(file)}</span>
                </li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>

      <div class="wz-row" role="listitem">
        {#if check.gameinfo === 'foreign'}<TriangleAlert class="dl-icon wz-warn" />{:else}<CircleCheck
            class="dl-icon wz-ok"
          />{/if}
        <div class="wz-text">
          <div class="wz-row__topic">{t('wizard-topic-gameinfo')}</div>
          {#if check.gameinfo === 'foreign'}
            <div class="wz-row__title">{t('wizard-gi-foreign')}</div>
            <div class="wz-row__text">{t('wizard-gi-foreign-text')}</div>
          {:else if check.gameinfo === 'ours'}
            <div class="wz-row__title">{t('wizard-gi-ours')}</div>
            <div class="wz-row__text">{t('wizard-gi-ours-text')}</div>
          {:else}
            <div class="wz-row__title">{t('wizard-gi-setup')}</div>
            <div class="wz-row__text">
              {t('wizard-gi-setup-a')} <span class="wz-code">gameinfo.gi</span>
              {t('wizard-gi-setup-b')}
            </div>
          {/if}
        </div>
        <div class="wz-row__actions">
          {#if check.gameinfo === 'foreign'}
            <Button
              variant="ghost"
              icon={CircleQuestionMark}
              label={t('wizard-what-is')}
              aria-expanded={gameinfo ? 'true' : 'false'}
              onclick={() => (gameinfo = !gameinfo)}
            />
          {/if}
        </div>
        {#if check.gameinfo === 'foreign' && gameinfo}
          <div class="wz-row__more"><span class="wz-code">gameinfo.gi</span> {t('wizard-gi-more')}</div>
        {/if}
      </div>
    {/if}
  </div>
{/if}
