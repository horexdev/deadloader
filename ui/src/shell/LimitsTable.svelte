<!-- E9 in D12 (board DownloadsD12): why deadloader stops, and each safety limit with what the archive measured. -->
<script lang="ts">
  import CircleCheck from '@lucide/svelte/icons/circle-check';
  import CircleX from '@lucide/svelte/icons/circle-x';
  import type { LimitRow } from './limits';
  import { t } from '../i18n';

  interface Props {
    rows: LimitRow[];
  }

  const { rows }: Props = $props();
</script>

<p>{t('errors-limits-why')}</p>
<table class="limits">
  <thead>
    <tr>
      <th scope="col">{t('errors-limit-check')}</th>
      <th scope="col">{t('errors-limit-archive')}</th>
      <th scope="col">{t('errors-limit-value')}</th>
      <th scope="col"><span class="dl-visually-hidden">{t('errors-limit-result')}</span></th>
    </tr>
  </thead>
  <tbody>
    {#each rows as row (row.check)}
      <tr>
        <th scope="row">{row.check}</th>
        <td class="value">{row.value}</td>
        <td class="limit">{row.limit}</td>
        <td class="result">
          {#if row.result === 'over'}
            <span class="mark over"><CircleX class="dl-icon" />{t('errors-limit-over')}</span>
          {:else if row.result === 'within'}
            <span class="mark within"><CircleCheck class="dl-icon" />{t('errors-limit-within')}</span>
          {/if}
        </td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  /* From DownloadsScreen.dc.html (D12): the limits table. */
  .limits {
    width: 100%;
    border-collapse: collapse;
    font: var(--type-body-sm);
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: 6px;
    overflow: hidden;
  }

  thead tr {
    font: var(--type-caption-strong);
    color: var(--ink-subtle);
    text-align: left;
  }

  th,
  td {
    padding: 6px 10px;
  }

  thead th {
    font-weight: 600;
  }

  tbody tr {
    border-top: 1px solid var(--line);
  }

  tbody th {
    font-weight: 400;
    color: var(--ink-muted);
    text-align: left;
  }

  .value {
    color: var(--ink);
  }

  .limit {
    color: var(--ink-muted);
  }

  .result {
    white-space: nowrap;
  }

  .mark {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font: var(--type-caption-strong);
  }

  .over {
    color: var(--danger);
  }

  .within {
    color: var(--success);
  }
</style>
