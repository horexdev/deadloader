<!-- D12 for the whole window: the error `app.showError` was given, in the user's language. -->
<script lang="ts">
  import ErrorDialog from '../lib/components/ErrorDialog.svelte';
  import type { AppState } from '../lib/app.svelte';
  import { describe, t } from '../i18n';

  interface Props {
    app: AppState;
  }

  const { app }: Props = $props();

  const labels = $derived({
    close: t('errors-close'),
    closeEsc: t('errors-close-esc'),
    details: t('errors-details'),
    detailsLog: t('errors-details-log'),
    copy: t('errors-copy'),
    copied: t('errors-copied'),
    openActivity: t('errors-open-activity'),
  });
  const shown = $derived(app.error);
  const text = $derived(shown ? describe(shown.message) : null);
</script>

{#if shown && text}
  <ErrorDialog
    what={text.what}
    why={text.why ?? ''}
    todo={text.todo ?? ''}
    detail={text.detail}
    logId={shown.message.args.log_id}
    actions={shown.actions}
    {labels}
    activityHref={shown.message.args.log_id ? `#/activity/${encodeURIComponent(shown.message.args.log_id)}` : undefined}
    onclose={() => app.closeError()}
  />
{/if}
