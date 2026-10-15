<!-- D7 Rollback (F11, decision 20): what leaves the selected profile's game folder, what stays, and the unchecked
     option to delete foreign files too. -->
<script lang="ts">
  import Check from '@lucide/svelte/icons/check';
  import CircleMinus from '@lucide/svelte/icons/circle-minus';
  import Minus from '@lucide/svelte/icons/minus';
  import Undo2 from '@lucide/svelte/icons/undo-2';
  import Button from '../../lib/components/Button.svelte';
  import Dialog from '../../lib/components/Dialog.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import { maintenance, type RollbackPreview, type RolledBack } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { formatList, formatSize, t } from '../../i18n';

  interface Props {
    app: AppState;
    preview: RollbackPreview;
    oncancel: () => void;
    ondone: (done: RolledBack) => void;
  }

  const { app, preview, oncancel, ondone }: Props = $props();

  let removeForeign = $state(false);
  let busy = $state(false);

  const drive = $derived(/^[A-Za-z]:/.exec(preview.game_root)?.[0] ?? preview.game_root);
  const otherProfiles = $derived(preview.profiles.filter((p) => p !== preview.selected_profile));
  const foreignCount = $derived(preview.foreign.length);

  async function confirm(): Promise<void> {
    busy = true;
    try {
      ondone(await maintenance.rollback(removeForeign));
    } catch (error) {
      app.fail(error);
    } finally {
      busy = false;
    }
  }
</script>

<Dialog
  title={t('settings-d7-title', { drive })}
  kind="danger"
  size="md"
  icon={Undo2}
  closeOnEscape={!busy}
  {oncancel}
>
  <div class="st-meta st-pathline st-pathline--wrap">
    {t('settings-d7-from', { profile: preview.selected_profile, others: otherProfiles.length > 0 ? formatList(otherProfiles) : 0 })}
    <PathTooltip path={preview.game_root} />
  </div>
  <ul class="st-impact">
    <li>
      <CircleMinus class="dl-icon st-impact__del" /><span
        ><strong>{t('settings-d7-files', { count: preview.files })}</strong> <span class="dl-mono">game\citadel\addons</span></span
      >
    </li>
    <li>
      <CircleMinus class="dl-icon st-impact__del" /><span
        ><strong>{t('settings-d7-line')}</strong> {t('settings-d7-line-why')}</span
      >
    </li>
  </ul>
  <div class="st-meta st-mt4">{t('settings-d7-stays')}</div>
  <ul class="st-impact">
    <li>
      <Check class="dl-icon st-impact__keep" /><span
        >{t('settings-d7-library', { mods: preview.library_mods, size: formatSize(preview.library_bytes) })}</span
      >
    </li>
    <li>
      <Check class="dl-icon st-impact__keep" /><span
        >{t('settings-d7-profiles', { count: preview.all_profiles.length, names: formatList(preview.all_profiles) })}</span
      >
    </li>
    {#each preview.others as other (other.game_root)}
      <li>
        <Check class="dl-icon st-impact__keep" /><span
          >{other.applied_profile
            ? t('settings-d7-other', { path: other.game_root, profile: other.applied_profile })
            : t('settings-d7-other-plain', { path: other.game_root })}</span
        >
      </li>
    {/each}
  </ul>
  <p>{t('settings-d7-back')}</p>
  {#if foreignCount > 0}
    <label class="dl-check st-option">
      <input class="dl-check__input" type="checkbox" bind:checked={removeForeign} disabled={busy} />
      <span class="dl-check__box"><Check class="dl-icon dl-check__tick" /><Minus class="dl-icon dl-check__dash" /></span>
      <span class="st-step">
        <span class="st-mono-ink">{t('settings-d7-foreign', { count: foreignCount })}</span>
        <span class="st-meta"
          >{#each preview.foreign as file, i (file.name)}{i > 0 ? ', ' : ''}<span class="dl-mono"
              >{file.name}</span
            >{/each} — {t('settings-d7-foreign-why')}</span
        >
      </span>
    </label>
  {/if}
  {#snippet footer()}
    <Button label={t('settings-cancel')} disabled={busy} data-autofocus onclick={oncancel} />
    <Button
      variant="danger"
      icon={Undo2}
      {busy}
      label={removeForeign ? t('settings-d7-confirm-foreign', { count: foreignCount }) : t('settings-d7-confirm')}
      onclick={() => void confirm()}
    />
  {/snippet}
</Dialog>
