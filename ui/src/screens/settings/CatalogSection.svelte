<!-- Settings → Catalog: catalog on/off (108, F13), page freshness, prefetch, NSFW (D3 through `chooseNsfw`), images in
     descriptions and their allowed hosts (122), GIFs, external links (31). -->
<script lang="ts">
  import Check from '@lucide/svelte/icons/check';
  import Lock from '@lucide/svelte/icons/lock';
  import Undo2 from '@lucide/svelte/icons/undo-2';
  import X from '@lucide/svelte/icons/x';
  import Banner from '../../lib/components/Banner.svelte';
  import Button from '../../lib/components/Button.svelte';
  import SegmentedControl from '../../lib/components/SegmentedControl.svelte';
  import Select from '../../lib/components/Select.svelte';
  import Toggle from '../../lib/components/Toggle.svelte';
  import { maintenance } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { notice } from '../../lib/notice.svelte';
  import { chooseNsfw } from '../../lib/nsfw';
  import { formatDate, formatNumber, t } from '../../i18n';
  import Group from './Group.svelte';
  import Row from './Row.svelte';
  import { CHOICES, DEFAULTS, withCurrent, type NsfwMode, type SettingsData } from './model';
  import { saveSettings } from './save';

  interface Props {
    app: AppState;
    data: SettingsData;
  }

  const { app, data }: Props = $props();

  let consent = $state(false);
  /** Turning the switch on only asks first; this puts it back off. */
  let switchKey = $state(0);

  const ttlLabel = (m: number) =>
    m % 60 === 0 && m >= 60 ? t('settings-hours', { count: formatNumber(m / 60) }) : t('settings-minutes', { count: formatNumber(m) });
  const ttlOptions = $derived(
    withCurrent(CHOICES.ttl, data.catalog.page_ttl_minutes).map((m) => ({ value: String(m), label: ttlLabel(m) })),
  );
  const nsfwOptions = $derived([
    { value: 'hidden', label: t('settings-nsfw-hidden') },
    { value: 'blurred', label: t('settings-nsfw-blurred') },
    { value: 'shown', label: t('settings-nsfw-shown') },
  ]);
  const imagesOptions = $derived([
    { value: 'on_click', label: t('settings-images-click') },
    { value: 'always', label: t('settings-images-always') },
  ]);
  const linksOptions = $derived([
    { value: 'ask', label: t('settings-links-ask') },
    { value: 'open_immediately', label: t('settings-links-open') },
  ]);
  const ageConfirmed = $derived(data.acknowledgements.age_confirmed_at);
  const nsfwHelp = $derived(
    `${t('settings-nsfw-help')} ${
      ageConfirmed ? t('settings-nsfw-age-done', { date: formatDate(new Date(ageConfirmed)) }) : t('settings-nsfw-age-ask')
    }`,
  );

  async function toggleCatalog(on: boolean): Promise<void> {
    if (on) {
      // F13: the network step comes first; the switch stays off until the user agrees there.
      consent = true;
      switchKey += 1;
      return;
    }
    consent = false;
    try {
      app.settings = await maintenance.chooseCatalog(false);
      app.toast({ kind: 'info', title: t('settings-local-on'), text: t('settings-local-on-text') });
    } catch (error) {
      app.fail(error);
    }
  }

  function removeHost(host: string): void {
    const before = data.catalog.allowed_image_hosts;
    void saveSettings(app, (s) => {
      s.catalog.allowed_image_hosts = s.catalog.allowed_image_hosts.filter((h) => h !== host);
    }).then((saved) => {
      if (saved) {
        app.toast({
          kind: 'success',
          title: t('settings-host-removed', { host }),
          text: t('settings-host-removed-text'),
          action: {
            label: t('settings-undo'),
            icon: Undo2,
            onclick: () => void saveSettings(app, (s) => (s.catalog.allowed_image_hosts = [...before])),
          },
        });
      }
    });
  }
</script>

<Group title={t('settings-catalog-group')}>
  <Row label={t('settings-catalog')} help={data.catalog.enabled ? t('settings-catalog-help-on') : t('settings-catalog-help-off')}>
    {#snippet control()}
      {#key switchKey}
        <Toggle
          ariaLabel={t('settings-catalog')}
          checked={data.catalog.enabled}
          onchange={(on) => void toggleCatalog(on)}
        />
      {/key}
    {/snippet}
    {#snippet full()}
      {#if consent && !data.catalog.enabled}
        <div class="st-mt4">
          <Banner kind="neutral" icon={Lock} title={t('settings-consent-title')} text={t('settings-consent-text')}>
            {#snippet actions()}
              <Button
                variant="primary"
                size="sm"
                label={t('settings-consent-go')}
                onclick={() => {
                  consent = false;
                  notice.open({ mode: 'catalog' });
                }}
              />
              <Button variant="ghost" size="sm" label={t('settings-cancel')} onclick={() => (consent = false)} />
            {/snippet}
          </Banner>
        </div>
      {/if}
    {/snippet}
  </Row>
  <Row
    label={t('settings-ttl')}
    help={t('settings-ttl-help')}
    changed={data.catalog.page_ttl_minutes !== DEFAULTS.ttl}
    defaultText={ttlLabel(DEFAULTS.ttl)}
    onrestore={() => void saveSettings(app, (s) => (s.catalog.page_ttl_minutes = DEFAULTS.ttl))}
  >
    {#snippet control()}
      <div class="st-select">
        <Select
          label={t('settings-ttl')}
          options={ttlOptions}
          value={String(data.catalog.page_ttl_minutes)}
          onchange={(v) => void saveSettings(app, (s) => (s.catalog.page_ttl_minutes = Number(v)))}
        />
      </div>
    {/snippet}
  </Row>
  <Row
    label={t('settings-prefetch')}
    help={t('settings-prefetch-help')}
    changed={data.catalog.prefetch_next_page !== DEFAULTS.prefetch}
    defaultText={t('settings-on')}
    onrestore={() => void saveSettings(app, (s) => (s.catalog.prefetch_next_page = DEFAULTS.prefetch))}
  >
    {#snippet control()}
      <Toggle
        ariaLabel={t('settings-prefetch')}
        checked={data.catalog.prefetch_next_page}
        onchange={(on) => void saveSettings(app, (s) => (s.catalog.prefetch_next_page = on))}
      />
    {/snippet}
  </Row>
</Group>

<Group title={t('settings-nsfw-group')}>
  <Row
    label={t('settings-nsfw')}
    help={nsfwHelp}
    changed={data.catalog.nsfw !== DEFAULTS.nsfw}
    defaultText={t('settings-nsfw-hidden')}
    onrestore={() => void chooseNsfw('hidden')}
  >
    {#snippet control()}
      <div class="st-seg">
        <SegmentedControl
          label={t('settings-nsfw')}
          options={nsfwOptions}
          value={data.catalog.nsfw}
          onchange={(v) => {
            // D3 decides; the value follows the saved settings.
            void chooseNsfw(v as NsfwMode);
            return false;
          }}
        />
      </div>
    {/snippet}
  </Row>
</Group>

<Group title={t('settings-images-group')}>
  <Row
    label={t('settings-description-images')}
    help={t('settings-description-images-help')}
    changed={data.catalog.description_images !== DEFAULTS.descriptionImages}
    defaultText={t('settings-images-click')}
    onrestore={() => void saveSettings(app, (s) => (s.catalog.description_images = 'on_click'))}
  >
    {#snippet control()}
      <SegmentedControl
        label={t('settings-description-images')}
        options={imagesOptions}
        value={data.catalog.description_images}
        onchange={(v) => {
          void saveSettings(app, (s) => (s.catalog.description_images = v as 'on_click' | 'always'));
          return undefined;
        }}
      />
    {/snippet}
  </Row>
  <Row
    label={t('settings-prechecked')}
    help={t('settings-prechecked-help')}
    changed={data.catalog.always_allow_host_prechecked !== DEFAULTS.prechecked}
    defaultText={t('settings-on')}
    onrestore={() => void saveSettings(app, (s) => (s.catalog.always_allow_host_prechecked = DEFAULTS.prechecked))}
  >
    {#snippet control()}
      <Toggle
        ariaLabel={t('settings-prechecked')}
        checked={data.catalog.always_allow_host_prechecked}
        onchange={(on) => void saveSettings(app, (s) => (s.catalog.always_allow_host_prechecked = on))}
      />
    {/snippet}
  </Row>
  <Row label={t('settings-hosts')} help={t('settings-hosts-help')} labelId="st-hosts" />
  <div class="st-list" role="list" aria-labelledby="st-hosts">
    <div class="st-item" role="listitem">
      <span class="dl-mono st-hostname">images.gamebanana.com</span>
      <span class="dl-badge"><Check class="dl-icon" />{t('settings-hosts-built-in')}</span>
    </div>
    {#each data.catalog.allowed_image_hosts as host (host)}
      <div class="st-item" role="listitem">
        <span class="dl-mono st-hostname">{host}</span>
        <Button
          variant="ghost"
          size="sm"
          icon={X}
          label={t('settings-remove')}
          aria-label={t('settings-remove-item', { item: host })}
          onclick={() => removeHost(host)}
        />
      </div>
    {:else}
      <div class="st-item" role="listitem"><span class="st-meta">{t('settings-hosts-none')}</span></div>
    {/each}
  </div>
  <Row
    label={t('settings-gif')}
    help={t('settings-gif-help')}
    changed={data.catalog.gif_autoplay !== DEFAULTS.gif}
    defaultText={t('settings-off')}
    onrestore={() => void saveSettings(app, (s) => (s.catalog.gif_autoplay = DEFAULTS.gif))}
  >
    {#snippet control()}
      <Toggle
        ariaLabel={t('settings-gif')}
        checked={data.catalog.gif_autoplay}
        onchange={(on) => void saveSettings(app, (s) => (s.catalog.gif_autoplay = on))}
      />
    {/snippet}
  </Row>
  <Row
    label={t('settings-links')}
    help={t('settings-links-help')}
    changed={data.catalog.external_links !== DEFAULTS.links}
    defaultText={t('settings-links-ask')}
    onrestore={() => void saveSettings(app, (s) => (s.catalog.external_links = 'ask'))}
  >
    {#snippet control()}
      <SegmentedControl
        label={t('settings-links')}
        options={linksOptions}
        value={data.catalog.external_links}
        onchange={(v) => {
          void saveSettings(app, (s) => (s.catalog.external_links = v as 'ask' | 'open_immediately'));
          return undefined;
        }}
      />
    {/snippet}
  </Row>
</Group>
