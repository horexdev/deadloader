import { flushSync } from 'svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

// The screen talks to the mock core, the same one `pnpm dev:mock` uses; calls are recorded.
const calls: { command: string; args: Record<string, unknown> }[] = [];
vi.mock('@tauri-apps/api/core', async () => {
  const mock = await import('../../mock/core');
  return {
    ...mock,
    invoke: (command: string, args: Record<string, unknown> = {}) => {
      calls.push({ command, args });
      return mock.invoke(command, args);
    },
  };
});

const { AppState } = await import('../../lib/app.svelte');
const { resetMock } = await import('../../mock/core');
const { default: Screen } = await import('./Screen.svelte');
const { all, click, one, render } = await import('../../lib/test/render');

async function start(scenario: string | null) {
  history.replaceState(null, '', scenario ? `/?scenario=${scenario}` : '/');
  resetMock();
  const app = new AppState();
  await app.start();
  await vi.waitFor(() => expect(app.network.kind).not.toBe('local_mode'));
  calls.length = 0;
  return app;
}

const plain = (text: string | null | undefined) => (text ?? '').replaceAll(String.fromCharCode(0xa0), ' ').replace(/\s+/g, ' ').trim();

function rowNamed(target: HTMLElement, name: string): HTMLElement {
  const row = all(target, '[role="listitem"]').find((r) => r.getAttribute('aria-label')?.startsWith(name));
  if (!row) {
    throw new Error(`no row ${name}`);
  }
  return row;
}

beforeEach(() => {
  calls.length = 0;
});

describe('the Downloads screen (S6)', () => {
  it('shows the empty state with the catalog and Add from disk', async () => {
    const app = await start(null);
    const { target } = render(Screen, { app });
    expect(one(target, '.dl-empty__title').textContent).toBe('No downloads');
    expect(one<HTMLAnchorElement>(target, '.dl-empty a').getAttribute('href')).toBe('#/catalog');
    expect(target.querySelector('.queue')).toBeNull();
  });

  it('lists the queue in the design’s order with totals and limits', async () => {
    const app = await start('downloads');
    const { target } = render(Screen, { app });
    const stats = plain(one(target, '.stats').textContent);
    expect(stats).toContain('Active 6');
    expect(stats).toContain('Queued 2');
    expect(stats).toContain('Up to 4 downloads at a time · no speed limit');
    const stages = all(target, '.queue [role="listitem"] .stage').map((s) => plain(s.textContent));
    expect(stages[0]).toBe('Waiting for your choice');
    expect(stages.at(-1)).toBe('Queued');
    expect(stages).toContain('Failed');
    expect(stages).toContain('Verifying');
    const update = rowNamed(target, 'Seven — Storm Crown');
    expect(plain(update.querySelector('.dl-badge')?.textContent)).toBe('Update v2');
    expect(update.querySelector<HTMLAnchorElement>('a.name')?.getAttribute('href')).toMatch(/^#\/catalog\/mod\/\d+$/);
  });

  it('pauses, resumes and cancels a download', async () => {
    const app = await start('downloads');
    const { target } = render(Screen, { app });
    click(one(rowNamed(target, 'Vindicta — Coral Reef'), 'button[aria-label^="Pause:"]'));
    click(one(rowNamed(target, 'Haze — Midnight Bloom — Paused'), 'button[aria-label^="Resume:"]'));
    click(one(rowNamed(target, 'Kelvin — Porcelain'), 'button[aria-label^="Cancel download of"]'));
    await vi.waitFor(() => expect(calls.filter((c) => c.command === 'download_control')).toHaveLength(3));
    expect(calls.map((c) => c.args.action)).toEqual(['pause', 'resume', 'cancel']);
  });

  it('shows a failure in the row with Retry, and its details in D12 on request', async () => {
    const app = await start('downloads');
    const { target } = render(Screen, { app });
    const row = rowNamed(target, 'Dynamo — Neon Rider');
    expect(plain(one(row, '.error').textContent)).toContain('Can’t unpack RAR archives on this PC.');
    click(one(row, '.error button'));
    await vi.waitFor(() => expect(calls.at(-1)?.args).toMatchObject({ action: 'retry' }));
    const big = rowNamed(target, 'Lady Geist — Ember Coat');
    const details = all(big, '.error button').find((b) => b.textContent?.includes('Show details'));
    click(details as HTMLElement);
    expect(app.error?.message.id).toBe('E9');
    expect(app.error?.actions.map((a) => a.label)).toEqual(['Remove task', 'Open Settings → Downloads']);
    expect(app.error?.actions[1]?.href).toBe('#/settings/downloads');
  });

  it('lets the user pick a variant in D4 and turns it on', async () => {
    const app = await start('downloads-choice');
    const { target } = render(Screen, { app });
    click(one(rowNamed(target, 'Pocket — Four Outfits'), 'button.dl-btn--primary'));
    const dialog = one(document.body, '.dl-dialog');
    expect(one(dialog, '.dl-dialog__title').textContent).toBe('Choose a variant');
    const options = all<HTMLInputElement>(dialog, 'input[type="radio"]');
    expect(options).toHaveLength(4);
    expect(plain(dialog.querySelector('.dl-variant')?.textContent)).toContain('Tracksuit');
    click(options[2] as HTMLElement);
    click(one(dialog, '.dl-dialog__foot .dl-btn--primary'));
    await vi.waitFor(() => expect(app.choices).toEqual([]));
    expect(calls.find((c) => c.command === 'choose')?.args).toEqual({ modId: 'gb-700042', files: ['gb-700042-1-2'] });
    expect(app.history[0]?.variant).toBe('Racing silks');
    expect(app.toasts.at(-1)?.title).toBe('Pocket — Four Outfits installed and enabled.');
  });

  it('remembers what finished this session, and clears it', async () => {
    const app = await start('downloads');
    const { target } = render(Screen, { app });
    const words = all(target, '.history [role="listitem"]').map((r) => plain(r.textContent));
    expect(words.some((w) => w.includes('Updated to v2'))).toBe(true);
    expect(words.some((w) => w.includes('Cancelled'))).toBe(true);
    click(one(target, '.history button'));
    flushSync();
    expect(app.history).toEqual([]);
    expect(plain(one(target, '.history').textContent)).toContain('Nothing finished yet in this session.');
  });

  it('says downloads wait while offline, and tries again on request', async () => {
    const app = await start('downloads-offline');
    const { target } = render(Screen, { app });
    expect(plain(one(target, '.dl-banner').textContent)).toContain('Offline — downloads are paused.');
    expect(all(target, '.stage').map((s) => plain(s.textContent))).toContain('Paused — offline');
    click(one(target, '.dl-banner button'));
    await vi.waitFor(() => expect(calls.some((c) => c.command === 'catalog_retry_now')).toBe(true));
  });

  it('says the disk is full and retries those downloads', async () => {
    const app = await start('downloads-disk-full');
    const { target } = render(Screen, { app });
    const banner = one(target, '.dl-banner--error');
    expect(plain(banner.textContent)).toContain('Not enough disk space.');
    expect(one<HTMLAnchorElement>(banner, 'a').getAttribute('href')).toBe('#/library');
    click(one(banner, 'button'));
    await vi.waitFor(() => expect(calls.filter((c) => c.args.action === 'retry').length).toBeGreaterThan(0));
  });
});
