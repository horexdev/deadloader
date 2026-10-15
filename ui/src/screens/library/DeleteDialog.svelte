<!-- D6 Delete mod (decision 42): what goes (files, size), the profiles using it, and where it goes — permanently or
     to the recycle bin, the setting's choice switchable for this deletion. -->
<script lang="ts">
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import type { RemovalPlan } from '../../lib/api';
  import { Button, Dialog, SegmentedControl } from '../../lib/components';
  import { formatList, formatSize, t } from '../../i18n';

  export type Deletion = 'permanent' | 'recycle_bin';

  interface Props {
    name: string;
    plan: RemovalPlan;
    /** The setting (Settings → Library). */
    deletion: Deletion;
    busy?: boolean;
    onconfirm: (deletion: Deletion) => void;
    oncancel: () => void;
  }

  const { name, plan, deletion, busy = false, onconfirm, oncancel }: Props = $props();

  // svelte-ignore state_referenced_locally
  let where: string = $state(deletion);

  const title = $derived(plan.removes_mod ? t('library-d6-title', { name }) : t('library-d6-files-title', { name }));
</script>

<Dialog {title} kind="danger" icon={Trash2} {oncancel}>
  <p>
    {t('library-d6-what', { count: plan.files.length, size: formatSize(plan.bytes) })}
    {plan.profiles.length > 0
      ? t('library-d6-profiles', { count: plan.profiles.length, names: formatList(plan.profiles) })
      : t('library-d6-no-profiles')}
  </p>
  <div class="dl-field">
    <span class="dl-field__label">{t('library-d6-where')}</span>
    <SegmentedControl
      label={t('library-d6-where')}
      options={[
        { value: 'permanent', label: t('library-d6-permanently') },
        { value: 'recycle_bin', label: t('library-d6-recycle-bin') },
      ]}
      bind:value={where}
    />
    <span class="dl-field__hint">{where === 'permanent' ? t('library-d6-permanent-hint') : t('library-d6-bin-hint')}</span>
  </div>
  {#snippet footer()}
    <Button label={t('library-cancel')} onclick={oncancel} />
    <Button
      variant="danger"
      icon={Trash2}
      label={plan.removes_mod ? t('library-d6-confirm') : t('library-d6-confirm-files')}
      {busy}
      onclick={() => onconfirm(where as Deletion)}
    />
  {/snippet}
</Dialog>
