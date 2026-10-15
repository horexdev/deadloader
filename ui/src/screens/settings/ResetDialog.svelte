<!-- "Reset all settings?" (100, 156): a normal primary action, because a dated backup and Undo exist. -->
<script lang="ts">
  import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
  import Undo2 from '@lucide/svelte/icons/undo-2';
  import Button from '../../lib/components/Button.svelte';
  import Dialog from '../../lib/components/Dialog.svelte';
  import { api, maintenance } from '../../lib/api';
  import type { AppState } from '../../lib/app.svelte';
  import { t } from '../../i18n';

  interface Props {
    app: AppState;
    onclose: () => void;
  }

  const { app, onclose }: Props = $props();

  let busy = $state(false);

  async function reset(): Promise<void> {
    const before = app.settings;
    busy = true;
    try {
      const done = await maintenance.reset();
      app.settings = done.settings;
      onclose();
      app.toast({
        kind: 'success',
        title: t('settings-reset-done'),
        text: done.backup ? t('settings-reset-done-text', { backup: done.backup }) : undefined,
        action: before
          ? {
              label: t('settings-undo'),
              icon: Undo2,
              onclick: () => {
                api
                  .updateSettings(before)
                  .then((settings) => (app.settings = settings))
                  .catch((error: unknown) => app.fail(error));
              },
            }
          : undefined,
      });
    } catch (error) {
      app.fail(error);
    } finally {
      busy = false;
    }
  }
</script>

<Dialog title={t('settings-reset-title')} icon={RotateCcw} role="alertdialog" closeOnEscape={!busy} oncancel={onclose}>
  <p>{t('settings-reset-body')}</p>
  <p>{t('settings-reset-keeps')}</p>
  <p class="st-meta">{t('settings-reset-backup')}</p>
  {#snippet footer()}
    <Button variant="ghost" label={t('settings-cancel')} data-autofocus onclick={onclose} />
    <Button variant="primary" icon={RotateCcw} {busy} label={t('settings-reset-confirm')} onclick={() => void reset()} />
  {/snippet}
</Dialog>
