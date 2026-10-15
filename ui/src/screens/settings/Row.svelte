<!-- One setting (design: .st-row): label, help, "Default: … · Restore default" while changed, the control on the
     right and an optional full-width part below. -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import type { Snippet } from 'svelte';
  import { t } from '../../i18n';

  interface Props {
    label: string;
    help?: string;
    /** Help with markup (keys, paths); replaces `help`. */
    helpContent?: Snippet;
    /** The default's label, shown only while the value differs from it. */
    defaultText?: string;
    changed?: boolean;
    onrestore?: () => void;
    control?: Snippet;
    full?: Snippet;
    /** The label's element id, for controls that name themselves by it. */
    labelId?: string;
  }

  const { label, help, helpContent, defaultText, changed = false, onrestore, control, full, labelId }: Props = $props();
</script>

<div class="st-row">
  <div class="st-row__text">
    <div class="st-row__label" id={labelId}><Text value={label} /></div>
    {#if helpContent}
      <p class="st-row__help">{@render helpContent()}</p>
    {:else if help}
      <p class="st-row__help"><Text value={help} /></p>
    {/if}
    {#if changed && defaultText !== undefined && onrestore}
      <p class="st-row__default">
        {t('settings-default', { value: defaultText })} ·
        <button type="button" class="dl-link st-linkbtn" onclick={onrestore}>{t('settings-restore')}</button>
      </p>
    {/if}
  </div>
  {#if control}<div class="st-row__ctl">{@render control()}</div>{/if}
  {#if full}<div class="st-row__full">{@render full()}</div>{/if}
</div>
