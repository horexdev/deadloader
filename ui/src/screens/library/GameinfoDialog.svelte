<!-- D10 gameinfo.gi changed by another program (decisions 19, 132; flow F7; board DialogD10): a plain explanation,
     the expected and the found SearchPaths block (collapsed), "What is this file?", and Verify game files in Steam;
     then deadloader checks again, on its own when the window gets focus or with Check again. -->
<script lang="ts">
  import Text from '../../lib/components/Text.svelte';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';
  import File from '@lucide/svelte/icons/file';
  import FolderOpen from '@lucide/svelte/icons/folder-open';
  import RefreshCw from '@lucide/svelte/icons/refresh-cw';
  import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
  import { onMount } from 'svelte';
  import type { GameinfoStatus } from '../../lib/api';
  import { Button, Dialog } from '../../lib/components';
  import { t } from '../../i18n';

  interface Props {
    status: GameinfoStatus;
    /** Steam was asked to verify; the dialog waits for it. */
    waiting: boolean;
    checking?: boolean;
    onverify: () => void;
    oncheck: () => void;
    onfile: () => void;
    onfolder: () => void;
    oncancel: () => void;
  }

  const { status, waiting, checking = false, onverify, oncheck, onfile, onfolder, oncancel }: Props = $props();

  let diff = $state(false);

  /** The block's lines without the indentation they all share, tabs as two spaces (the design's diff). */
  function dedent(lines: { text: string }[]): string[] {
    const texts = lines.map((l) => l.text.replaceAll('\t', '  '));
    const indents = texts.filter((s) => s.trim() !== '').map((s) => s.length - s.trimStart().length);
    const cut = indents.length > 0 ? Math.min(...indents) : 0;
    return texts.map((s) => s.slice(Math.min(cut, s.length - s.trimStart().length)));
  }
  let help = $state(false);

  onMount(() => {
    // F7: coming back from Steam is the moment to look again.
    const focus = () => {
      if (waiting) {
        oncheck();
      }
    };
    window.addEventListener('focus', focus);
    return () => window.removeEventListener('focus', focus);
  });
</script>

<Dialog title={t('library-d10-title')} size="lg" role="alertdialog" icon={TriangleAlert} {oncancel}>
  {#if waiting}
    <p class="ink">{t('library-d10-wait-title')}</p>
    <p><Text value={t('library-d10-wait-text')} /></p>
  {:else}
    <p class="ink"><Text value={t('library-d10-what')} /></p>
    <p><Text value={t('library-d10-do')} /></p>
    <div class="diff">
      <button
        type="button"
        class="dl-focusable toggle"
        aria-expanded={diff ? 'true' : 'false'}
        aria-controls="d10diff"
        onclick={() => (diff = !diff)}
      >
        <span class={['chevron', diff && 'open']}><ChevronRight class="dl-icon" /></span>
        <span class="grow">{t('library-d10-diff')}</span>
        <span class="dl-mono file">gameinfo.gi</span>
      </button>
      {#if diff && status.diff}
        <div id="d10diff" class="columns">
          {#each [{ head: t('library-d10-expected'), lines: status.diff.expected, kind: 'ours' }, { head: t('library-d10-found'), lines: status.diff.found, kind: 'theirs' }] as column (column.kind)}
            <div class={['column', column.kind === 'ours' && 'divider']}>
              <span class="head"><Text value={column.head} /></span>
              <pre class="dl-mono">{#each column.lines as line, i (i)}<span class={['line', line.marked && column.kind]}>{dedent(column.lines)[i]}</span>{/each}</pre>
            </div>
          {/each}
        </div>
        <p class="note"><Text value={t('library-d10-diff-note')} /></p>
      {/if}
    </div>
    <button type="button" class="dl-link help" aria-expanded={help ? 'true' : 'false'} onclick={() => (help = !help)}>
      {t('library-d10-help')}
    </button>
    {#if help}<p class="help-text"><Text value={t('library-d10-help-text')} /></p>{/if}
  {/if}
  {#snippet footerStart()}
    {#if !waiting}
      <Button variant="ghost" icon={File} label={t('library-open-file')} onclick={onfile} />
      <Button variant="ghost" icon={FolderOpen} label={t('library-open-folder')} onclick={onfolder} />
    {/if}
  {/snippet}
  {#snippet footer()}
    {#if waiting}
      <Button label={t('library-close')} onclick={oncancel} />
      <Button variant="primary" icon={RefreshCw} label={t('library-check-again')} busy={checking} onclick={oncheck} />
    {:else}
      <Button label={t('library-not-now')} onclick={oncancel} />
      <Button variant="primary" label={t('library-verify-steam')} onclick={onverify} />
    {/if}
  {/snippet}
</Dialog>

<style>
  /* From the design's D10. */
  p {
    margin: 0;
  }

  .ink {
    color: var(--ink);
  }

  .diff {
    display: grid;
    background: var(--surface-sunken);
    border: 1px solid var(--line);
    border-radius: 6px;
  }

  .toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 36px;
    padding: 0 10px;
    background: transparent;
    border: 0;
    color: var(--ink);
    cursor: pointer;
    text-align: left;
    font: var(--type-body-sm);
  }

  .chevron {
    display: inline-flex;
    flex: none;
    transition: transform var(--duration-base) var(--ease-standard);
  }

  .chevron.open {
    transform: rotate(90deg);
  }

  .grow {
    flex: 1 1 auto;
  }

  .file {
    color: var(--ink-subtle);
  }

  .columns {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    border-top: 1px solid var(--line);
  }

  .column {
    display: grid;
    align-content: start;
    gap: 2px;
    padding: 8px 10px;
  }

  .divider {
    border-right: 1px solid var(--line);
  }

  .head {
    font: var(--type-caption-strong);
    color: var(--ink-muted);
  }

  pre {
    margin: 0;
    white-space: pre;
    overflow: auto;
    color: var(--ink-muted);
  }

  .line {
    display: block;
  }

  .line.ours {
    background: var(--success-soft);
    color: var(--ink);
  }

  .line.theirs {
    background: var(--warning-soft);
    color: var(--ink);
  }

  .note {
    padding: 0 10px 8px;
    font: var(--type-caption);
    color: var(--ink-subtle);
  }

  .help {
    justify-self: start;
    padding: 0;
    background: none;
    border: 0;
    cursor: pointer;
    font: var(--type-body-sm);
  }

  .help-text {
    padding: 10px 12px;
    font: var(--type-body-sm);
    color: var(--ink-muted);
    background: var(--surface);
    border: 1px solid var(--line);
    border-radius: 6px;
  }
</style>
