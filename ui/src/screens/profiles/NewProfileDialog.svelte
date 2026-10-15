<!-- New profile (board ProfilesNew): a name, empty or a copy of the selected profile, in the selected profile's game
     installation. Creating doesn't select or apply it. -->
<script lang="ts">
  import Info from '@lucide/svelte/icons/info';
  import Layers from '@lucide/svelte/icons/layers';
  import Button from '../../lib/components/Button.svelte';
  import Dialog from '../../lib/components/Dialog.svelte';
  import FieldError from '../../lib/components/FieldError.svelte';
  import PathTooltip from '../../lib/components/PathTooltip.svelte';
  import type { ProfileOverview } from '../../lib/api';
  import { t } from '../../i18n';
  import { nameProblem } from './profiles';

  interface Props {
    names: string[];
    selected: ProfileOverview | undefined;
    root: string | null;
    /** A problem the core found with the name (E21, E22), shown under the field. */
    refused: string | null;
    oncreate: (name: string, copy: boolean) => void;
    oncancel: () => void;
  }

  const { names, selected, root, refused, oncreate, oncancel }: Props = $props();

  const uid = $props.id();
  let name = $state('');
  let copy = $state(false);
  let problem: string | null = $state(null);
  const shown = $derived(problem ?? refused);

  function create() {
    const found = nameProblem(name, names);
    if (found === 'empty') {
      problem = t('profiles-name-empty');
      return;
    }
    if (found === 'taken') {
      problem = `${t('e21', { name: name.trim() })} ${t('e21.todo')}`;
      return;
    }
    oncreate(name.trim(), copy);
  }
</script>

<Dialog title={t('profiles-new-title')} icon={Layers} closeLabel={t('profiles-close-esc')} {oncancel}>
  <div class="dl-field">
    <label class="dl-field__label" for={`${uid}-name`}>{t('profiles-name-label')}</label>
    <div class="dl-input">
      <input
        id={`${uid}-name`}
        type="text"
        data-autofocus=""
        value={name}
        aria-invalid={shown ? 'true' : 'false'}
        aria-describedby={`${uid}-msg`}
        oninput={(event) => {
          name = event.currentTarget.value;
          problem = null;
        }}
        onkeydown={(event) => {
          if (event.key === 'Enter') {
            event.preventDefault();
            create();
          }
        }}
      />
    </div>
    {#if shown}<FieldError id={`${uid}-msg`} message={shown} />{/if}
  </div>
  <div class="dl-field" role="radiogroup" aria-labelledby={`${uid}-start`}>
    <span class="dl-field__label" id={`${uid}-start`}>{t('profiles-start-with')}</span>
    <div class="pf-opts">
      <label class="dl-variant"
        ><input class="dl-variant__radio" type="radio" name={`${uid}-start`} checked={!copy} onchange={() => (copy = false)} /><span
          class="dl-variant__dot"
        ></span><span class="pf-opt"><b>{t('profiles-empty')}</b><small>{t('profiles-empty-hint')}</small></span></label
      >
      {#if selected}
        <label class="dl-variant"
          ><input class="dl-variant__radio" type="radio" name={`${uid}-start`} checked={copy} onchange={() => (copy = true)} /><span
            class="dl-variant__dot"
          ></span><span class="pf-opt"
            ><b>{t('profiles-copy-of', { name: selected.name })}</b><small
              >{t('profiles-copy-hint', {
                mods: selected.mods === 0 ? t('profiles-mods-none') : t('profiles-mods', { count: selected.mods }),
              })}</small
            ></span
          ></label
        >
      {/if}
    </div>
  </div>
  {#if root && selected}
    <div class="dl-field">
      <span class="dl-field__label">{t('profiles-installation')}</span>
      <span class="path"><PathTooltip path={root} /></span>
      <span class="dl-field__hint">{t('profiles-installation-hint', { name: selected.name })}</span>
    </div>
  {/if}
  <p class="pf-note"><Info class="dl-icon" /><span>{t('profiles-new-note')}</span></p>
  {#snippet footer()}
    <Button label={t('profiles-cancel')} onclick={oncancel} />
    <Button variant="primary" label={t('profiles-create')} onclick={create} />
  {/snippet}
</Dialog>

<style>
  /* From ProfilesScreen.dc.html (new profile); pf-opts and pf-note are in profiles.css. */
  .path {
    display: flex;
    width: fit-content;
    max-width: 100%;
  }
</style>
