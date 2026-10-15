<!-- S1, the first-run wizard (107–111, design: Wizard.dc.html): six steps with the language picker on each, the
     acknowledgement, the catalog / local-mode choice (108), the update check (157) and the setup checks. The same
     frame shows the re-shown notice with "what changed", the catalog's network step (F13) and "Read it again". -->
<script lang="ts">
  import Text from '../lib/components/Text.svelte';
  import AppWindow from '@lucide/svelte/icons/app-window';
  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import Check from '@lucide/svelte/icons/check';
  import ChevronLeft from '@lucide/svelte/icons/chevron-left';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';
  import CircleArrowUp from '@lucide/svelte/icons/circle-arrow-up';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import CodeXml from '@lucide/svelte/icons/code-xml';
  import ExternalLink from '@lucide/svelte/icons/external-link';
  import FileText from '@lucide/svelte/icons/file-text';
  import Folder from '@lucide/svelte/icons/folder';
  import Info from '@lucide/svelte/icons/info';
  import LayoutGrid from '@lucide/svelte/icons/layout-grid';
  import Package from '@lucide/svelte/icons/package';
  import Plus from '@lucide/svelte/icons/plus';
  import ShieldCheck from '@lucide/svelte/icons/shield-check';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import Undo2 from '@lucide/svelte/icons/undo-2';
  import Unplug from '@lucide/svelte/icons/unplug';
  import UserX from '@lucide/svelte/icons/user-x';
  import { tick } from 'svelte';
  import lockup from '../assets/brand/lockup-dark.svg';
  import Button from '../lib/components/Button.svelte';
  import Select from '../lib/components/Select.svelte';
  import { maintenance, type CatalogChoice } from '../lib/api';
  import type { AppState } from '../lib/app.svelte';
  import { openLink } from '../lib/links';
  import { NOTICE, STEPS, type NoticeMode, type Step } from '../lib/notice.svelte';
  import { LANGS, LANGUAGE_NAMES, formatDate, language, setLanguage, t } from '../i18n';
  import { saveSettings } from '../screens/settings/save';
  import TitleBar from '../shell/TitleBar.svelte';
  import SetupStep from './SetupStep.svelte';
  import './wizard.css';

  interface Props {
    app: AppState;
    mode: NoticeMode;
    /** The wizard is done (acknowledged, consented, or closed); `route` is where the app goes next, if anywhere. */
    ondone: (route: 'catalog' | 'library' | null) => void;
  }

  const { app, mode, ondone }: Props = $props();

  const REPO = 'https://github.com/horexdev/deadloader';
  const TOS = 'https://gamebanana.com/wikis/334';

  /** The steps this mode walks through. */
  const pages: Step[] = $derived(
    mode.kind === 'changed'
      ? STEPS.filter((s) => mode.changes.some((c) => c.steps.includes(s)))
      : mode.kind === 'catalog'
        ? ['network']
        : [...STEPS],
  );
  const review = $derived(mode.kind === 'review');
  const partial = $derived(mode.kind === 'changed' || mode.kind === 'catalog');

  let index = $state(0);
  let agreed = $state(false);
  let agreeError = $state(false);
  let choice = $state<CatalogChoice | null>(null);
  let choiceError = $state(false);
  let updates = $state(false);
  let saving = $state(false);

  // "Read it again" shows the choices on record, without letting them change here.
  $effect.pre(() => {
    if (review && app.settings) {
      agreed = true;
      choice = app.settings.catalog?.enabled === true ? 'catalog' : 'local_mode';
      updates = app.settings.general?.check_app_updates === true;
    }
  });

  const step = $derived(pages[index] ?? 'welcome');
  const last = $derived(index === pages.length - 1);
  const local = $derived(choice === 'local_mode');

  /** What keeps Next from going on: the acknowledgement after step 2, the choice on step 3. */
  const gate = $derived.by((): 'agree' | 'choice' | null => {
    if (review) {
      return null;
    }
    if (step === 'risks' && !agreed) {
      return 'agree';
    }
    if (step === 'network' && choice === null) {
      return 'choice';
    }
    return null;
  });

  const caption = $derived(
    mode.kind === 'changed'
      ? t('wizard-caption-changed')
      : mode.kind === 'catalog'
        ? t('wizard-caption-catalog')
        : mode.kind === 'review'
          ? t('wizard-caption-review')
          : t('wizard-caption-setup'),
  );

  const stepOf = $derived(
    mode.kind === 'changed' ? t('wizard-changed-of', { count: pages.length }) : t('wizard-step-of', { n: STEPS.indexOf(step) + 1 }),
  );

  const nav = $derived(
    STEPS.map((s, i) => {
      const n = i + 1;
      const pageIndex = pages.indexOf(s);
      const current = s === step;
      let state: 'current' | 'done' | 'todo' | 'kept';
      if (partial) {
        state = current ? 'current' : pageIndex >= 0 && pageIndex < index ? 'done' : pageIndex >= 0 ? 'todo' : 'kept';
      } else {
        state = current ? 'current' : pageIndex < index ? 'done' : review ? 'done' : 'todo';
      }
      const clickable = !current && pageIndex >= 0 && (review || pageIndex < index);
      const meta = mode.kind === 'changed' ? (pageIndex >= 0 ? t('wizard-meta-changed') : t('wizard-meta-kept')) : '';
      return { s, n, state, current, clickable, meta, pageIndex };
    }),
  );

  const languageOptions = $derived(LANGS.map((code) => ({ value: code, label: LANGUAGE_NAMES[code] })));

  async function chooseLanguage(code: string): Promise<void> {
    // Saved like any Settings change; the texts switch at once.
    void saveSettings(app, (s) => (s.general.language = code));
    await setLanguage(code);
  }

  function go(to: number): void {
    index = Math.max(0, Math.min(pages.length - 1, to));
    agreeError = false;
    choiceError = false;
  }

  async function reveal(id: string): Promise<void> {
    await tick();
    document.getElementById(id)?.scrollIntoView?.({ block: 'nearest' });
  }

  async function next(): Promise<void> {
    if (gate === 'agree') {
      agreeError = true;
      await reveal('wz-agree-err');
      return;
    }
    if (gate === 'choice') {
      choiceError = true;
      await reveal('wz-choice-err');
      return;
    }
    if (!last) {
      go(index + 1);
      return;
    }
    await finish();
  }

  async function finish(): Promise<void> {
    // Saving the acknowledgement replaces the wizard with the app at once, and `mode` with it (the parent's view has
    // none then): everything after the save works from this copy.
    const current = mode;
    if (current.kind === 'review') {
      ondone(null);
      return;
    }
    saving = true;
    try {
      if (current.kind === 'catalog') {
        if (choice === 'catalog') {
          app.settings = await maintenance.chooseCatalog(true);
          ondone('catalog');
        } else {
          ondone(null);
        }
        return;
      }
      const catalog = choice ?? (current.kind === 'changed' ? current.previous.catalog : 'local_mode');
      const checkUpdates = current.kind === 'changed' ? app.settings?.general?.check_app_updates === true : updates;
      const next = current.kind === 'first' ? (catalog === 'catalog' ? 'catalog' : 'library') : null;
      app.settings = await maintenance.acknowledgeNotice(NOTICE.version, language(), catalog, checkUpdates);
      ondone(next);
    } catch (error) {
      app.fail(error);
    } finally {
      saving = false;
    }
  }

  function link(url: string): void {
    openLink(url).catch((error: unknown) => app.fail(error));
  }

  const nextLabel = $derived(
    !last ? t('wizard-next') : review ? t('wizard-close') : partial ? t('wizard-continue') : t('wizard-finish'),
  );
  /** The hosts the catalog contacts (the network allowlist), with what each is for. */
  const HOSTS = [
    { name: 'gamebanana.com', what: 'wizard-host-api' },
    { name: 'images.gamebanana.com', what: 'wizard-host-images' },
    { name: 'files.gamebanana.com', what: 'wizard-host-files' },
    { name: 'filecacheNN.gamebanana.com', what: 'wizard-host-cache' },
  ] as const;
  /** Hosts a newer notice added since the acknowledged one (111). */
  const newHosts = $derived(mode.kind === 'changed' ? mode.changes.flatMap((c) => c.hosts ?? []) : []);
  const changeNote = $derived(mode.kind === 'changed' ? mode.changes.filter((c) => c.steps.includes(step)) : []);
</script>

<div class="wz-window dl-root">
  <TitleBar />
  <div class="wz-stage">
    <div class="wz-panel" role="dialog" aria-modal="false" aria-labelledby="wz-title">
      <aside class="wz-aside">
        <img class="wz-lockup" src={lockup} alt="deadloader" />
        <div class="wz-nav">
          <div class="wz-caption">{caption}</div>
          <nav aria-label={t('wizard-progress')}>
            <ol class="wz-steps">
              {#each nav as item (item.s)}
                <li>
                  <button
                    type="button"
                    class="wz-step"
                    data-s={item.state}
                    aria-current={item.current ? 'step' : undefined}
                    aria-disabled={item.clickable || item.current ? 'false' : 'true'}
                    onclick={() => {
                      if (item.clickable) {
                        go(item.pageIndex);
                      }
                    }}
                  >
                    <span class="wz-step__n"
                      >{#if item.state === 'done' || item.state === 'kept'}<Check class="dl-icon" />{:else}{item.n}{/if}</span
                    >
                    <span class="wz-step__label"
                      >{t(`wizard-step-${item.s}`)}{#if item.meta}<span class="wz-step__meta">{item.meta}</span>{/if}</span
                    >
                  </button>
                </li>
              {/each}
            </ol>
          </nav>
        </div>
        <div class="wz-spacer"></div>
        <div class="wz-lang">
          <span class="wz-lang__label">{t('wizard-language')}</span>
          <Select
            label={t('wizard-language')}
            options={languageOptions}
            value={language()}
            onchange={(code) => void chooseLanguage(code)}
          />
        </div>
      </aside>

      <section class="wz-main">
        <div class="wz-scroll">
          <div class="wz-body">
            <div class="wz-stepof">{stepOf}</div>

            {#if step === 'welcome'}
              <h1 class="wz-title" id="wz-title"><Text value={t('wizard-s1-title')} /></h1>
              <p><Text value={t('wizard-s1-lead')} /></p>
              <ul class="wz-facts">
                <li>
                  <Package class="dl-icon" /><span
                    >{t('wizard-f1a')} <span class="wz-code">gameinfo.gi</span> {t('wizard-f1b')}</span
                  >
                </li>
                <li><Undo2 class="dl-icon" /><span>{t('wizard-f2')}</span></li>
                <li>
                  <Folder class="dl-icon" /><span
                    >{t('wizard-f3a')} <span class="wz-code">deadloader-data</span>
                    {t('wizard-f3b')} <span class="wz-code">deadloader.exe</span>{t('wizard-f3c')}</span
                  >
                </li>
                <li><Info class="dl-icon" /><span><Text value={t('wizard-f4')} /></span></li>
                <li><CodeXml class="dl-icon" /><span>{t('wizard-f5')}</span></li>
              </ul>
              <div class="wz-links">
                <button type="button" class="dl-link" onclick={() => link(`${REPO}/blob/main/LICENSE`)}
                  >{t('wizard-licence')}<ExternalLink class="dl-icon" /></button
                >
                <button type="button" class="dl-link" onclick={() => link(REPO)}
                  >{t('wizard-source')}<ExternalLink class="dl-icon" /></button
                >
              </div>
            {:else if step === 'risks'}
              <h1 class="wz-title" id="wz-title">{t('wizard-s2-title')}</h1>
              <ul class="wz-facts">
                <li><Info class="dl-icon" /><span>{t('wizard-r1')}</span></li>
                <li><TriangleAlert class="dl-icon" /><span><Text value={t('wizard-r2')} /></span></li>
                <li><ShieldCheck class="dl-icon" /><span><Text value={t('wizard-r3')} /></span></li>
              </ul>
              <div class="wz-consent">
                <label class="dl-check">
                  <input
                    class="dl-check__input"
                    type="checkbox"
                    bind:checked={agreed}
                    disabled={review}
                    aria-invalid={agreeError && !agreed ? 'true' : 'false'}
                    aria-describedby="wz-agree-note wz-agree-err"
                    onchange={() => (agreeError = false)}
                  />
                  <span class="dl-check__box"><Check class="dl-icon dl-check__tick" /></span>
                  <span><Text value={t('wizard-agree')} /></span>
                </label>
                <span class="wz-consent__note" id="wz-agree-note">{t('wizard-agree-note')}</span>
                {#if agreeError && !agreed}
                  <span class="dl-field__error wz-consent__note" id="wz-agree-err" role="alert"
                    ><CircleX class="dl-icon" />{t('wizard-agree-error')}</span
                  >
                {/if}
              </div>
            {:else if step === 'network'}
              <h1 class="wz-title" id="wz-title">{t('wizard-s3-title')}</h1>
              {#each changeNote as change (change.version)}
                <div class="wz-changed" role="note">
                  <div class="wz-changed__title">
                    <Info class="dl-icon dl-icon--md" /><span
                      >{t('wizard-changed-title', { date: formatDate(new Date(`${change.date}T00:00:00`)) })}</span
                    >
                  </div>
                  <p class="wz-changed__text">{change.text()}</p>
                </div>
              {/each}
              <p>{t('wizard-s3-lead')}</p>
              <ul class="wz-hosts" aria-label={t('wizard-hosts')}>
                {#each HOSTS as host (host.name)}
                  <li>
                    <span class="dl-mono">{host.name}</span><span class="wz-hosts__what">{t(host.what)}</span>
                    {#if newHosts.includes(host.name)}
                      <span class="dl-badge dl-badge--info"><Plus class="dl-icon" />{t('wizard-host-new')}</span>
                    {/if}
                  </li>
                {/each}
              </ul>
              <p>
                {t('wizard-s3-images')} <strong class="wz-activity">{t('wizard-activity')}</strong>{t('wizard-s3-images-end')}
              </p>
              <p>
                {t('wizard-s3-tos')}
                <button type="button" class="dl-link wz-inline-link" onclick={() => link(TOS)}
                  >{t('wizard-tos')}<ExternalLink class="dl-icon" /></button
                >
              </p>
              <h2 class="wz-legend wz-legend--choice" id="wz-choice">{t('wizard-choice')}</h2>
              <div class="wz-opts" role="radiogroup" aria-labelledby="wz-choice" aria-describedby="wz-choice-err">
                <label class="wz-opt">
                  <input
                    class="dl-variant__radio"
                    type="radio"
                    name="wz-net"
                    value="catalog"
                    disabled={review}
                    checked={choice === 'catalog'}
                    onchange={() => {
                      choice = 'catalog';
                      choiceError = false;
                    }}
                  /><span class="dl-variant__dot"></span>
                  <span class="wz-opt__title"><LayoutGrid class="dl-icon" />{t('wizard-use-catalog')}</span>
                  <span class="wz-opt__text"><Text value={t('wizard-use-catalog-text')} /></span>
                  {#if mode.kind === 'changed' && mode.previous.catalog === 'catalog'}
                    <span class="dl-badge dl-badge--outline wz-opt__tag">{t('wizard-last-time')}</span>
                  {/if}
                </label>
                <label class="wz-opt">
                  <input
                    class="dl-variant__radio"
                    type="radio"
                    name="wz-net"
                    value="local"
                    disabled={review}
                    checked={choice === 'local_mode'}
                    onchange={() => {
                      choice = 'local_mode';
                      choiceError = false;
                    }}
                  /><span class="dl-variant__dot"></span>
                  <span class="wz-opt__title"><Unplug class="dl-icon" />{t('wizard-local')}</span>
                  <span class="wz-opt__text">{t('wizard-local-text')}</span>
                  {#if mode.kind === 'changed' && mode.previous.catalog === 'local_mode'}
                    <span class="dl-badge dl-badge--outline wz-opt__tag">{t('wizard-last-time')}</span>
                  {/if}
                </label>
              </div>
              {#if choiceError && choice === null}
                <span class="dl-field__error" id="wz-choice-err" role="alert"
                  ><CircleX class="dl-icon" />{t('wizard-choice-error')}</span
                >
              {/if}
            {:else if step === 'privacy'}
              <h1 class="wz-title" id="wz-title">{t('wizard-s4-title')}</h1>
              <ul class="wz-facts">
                <li><UserX class="dl-icon" /><span><Text value={local ? t('wizard-p1-local') : t('wizard-p1')} /></span></li>
                <li>
                  <FileText class="dl-icon" /><span
                    >{t('wizard-p2a')} <span class="wz-code">deadloader-data</span>
                    {t('wizard-p2b')} <span class="wz-code">C:\Users\…\</span></span
                  >
                </li>
                <li><AppWindow class="dl-icon" /><span><Text value={t('wizard-p3')} /></span></li>
              </ul>
              <p class="wz-small">
                <Text value={t('wizard-p4')} /> <strong class="wz-activity">{t('wizard-activity')}</strong>{t('wizard-p4-end')}
              </p>
            {:else if step === 'updates'}
              <h1 class="wz-title" id="wz-title"><Text value={t('wizard-s5-title')} /></h1>
              <p><Text value={t('wizard-u1')} /></p>
              <div class="wz-consent">
                <label class="dl-switch wz-upd">
                  <input
                    class="dl-switch__input"
                    type="checkbox"
                    role="switch"
                    checked={updates && !local}
                    disabled={local || review}
                    aria-describedby="wz-upd-state wz-upd-text"
                    onchange={(event) => (updates = event.currentTarget.checked)}
                  />
                  <span class="dl-switch__track"><span class="dl-switch__thumb"></span></span>
                  <span
                    ><Text value={t('wizard-updates-label')} /><span class="wz-upd__state" id="wz-upd-state"
                      >{local ? t('wizard-updates-na') : updates ? t('wizard-updates-on') : t('wizard-updates-off')}</span
                    ></span
                  >
                </label>
                <span class="wz-upd__text" id="wz-upd-text"
                  ><Text value={t('wizard-u2a')} /> <span class="wz-code">api.github.com</span> {t('wizard-u2b')}</span
                >
                {#if local}
                  <span class="wz-note wz-upd__note"><Info class="dl-icon wz-info" />{t('wizard-u4')}</span>
                {/if}
              </div>
              <ul class="wz-facts">
                <li><CircleArrowUp class="dl-icon" /><span><Text value={t('wizard-u3')} /></span></li>
                <li><Info class="dl-icon" /><span>{t('wizard-u5')}</span></li>
              </ul>
            {:else}
              <h1 class="wz-title" id="wz-title">{t('wizard-s6-title')}</h1>
              <p><Text value={t('wizard-s6-lead')} /></p>
              <SetupStep {app} {review} />
            {/if}
          </div>
        </div>

        <footer class="wz-foot">
          {#if mode.kind === 'catalog'}
            <Button size="lg" label={t('wizard-cancel')} onclick={() => ondone(null)} />
          {:else if index > 0}
            <Button size="lg" icon={ChevronLeft} label={t('wizard-back')} onclick={() => go(index - 1)} />
          {/if}
          {#if mode.kind === 'changed'}
            <span class="wz-foot__note"
              >{t('wizard-changed-foot', { date: formatDate(new Date(mode.previous.at)) })}</span
            >
          {/if}
          <span class="wz-grow"></span>
          {#if gate}
            <span class="wz-reason" id="wz-next-reason"
              ><Info class="dl-icon" />{gate === 'agree' ? t('wizard-agree-reason') : t('wizard-choice-reason')}</span
            >
          {:else if last && mode.kind === 'first'}
            <span class="wz-reason">{local ? t('wizard-to-library') : t('wizard-to-catalog')}</span>
          {/if}
          <Button
            variant="primary"
            size="lg"
            busy={saving}
            aria-disabled={gate ? 'true' : undefined}
            aria-describedby={gate ? 'wz-next-reason' : undefined}
            label={nextLabel}
            onclick={() => void next()}
          >
            {#if last && !gate && !review}<ArrowRight class="dl-icon" />{:else}<ChevronRight class="dl-icon" />{/if}
          </Button>
        </footer>
      </section>
    </div>
  </div>
</div>
