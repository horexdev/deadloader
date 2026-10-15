import { flushSync } from 'svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

// The screen runs against the mock core, the same one `pnpm dev:mock` uses.
vi.mock('@tauri-apps/api/core', () => import('../../mock/core'));

const { default: Screen } = await import('./Screen.svelte');
const { app } = await import('../../lib/app.svelte');
const { devtools, resetMock } = await import('../../mock/core');
const { all, click, keydown, one, render } = await import('../../lib/test/render');

// happy-dom has no layout: give the list a size so the virtual list renders rows.
beforeEach(() => {
  vi.stubGlobal(
    'ResizeObserver',
    class {
      #callback: ResizeObserverCallback;
      constructor(callback: ResizeObserverCallback) {
        this.#callback = callback;
      }
      observe(target: Element) {
        queueMicrotask(() => this.#callback([{ target } as ResizeObserverEntry], this as unknown as ResizeObserver));
      }
      unobserve() {}
      disconnect() {}
    },
  );
  // The screen at 1280 × 800 less the sidebar: the list is 992 px wide, so every column shows.
  vi.spyOn(HTMLElement.prototype, 'clientWidth', 'get').mockImplementation(function (this: HTMLElement) {
    return this.hasAttribute('data-virtual-viewport') ? 1000 : this.classList.contains('act-main') ? 1040 : 0;
  });
  vi.spyOn(HTMLElement.prototype, 'clientHeight', 'get').mockImplementation(function (this: HTMLElement) {
    return this.hasAttribute('data-virtual-viewport') ? 600 : 0;
  });
  Object.defineProperty(navigator, 'clipboard', { value: { writeText: vi.fn().mockResolvedValue(undefined) }, configurable: true });
});

afterEach(() => {
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
  history.replaceState(null, '', '/');
  app.toasts = [];
});

async function open(scenario?: string, hash = '#/activity') {
  history.replaceState(null, '', `/${scenario ? `?scenario=${scenario}` : ''}${hash}`);
  resetMock();
  await app.start();
  const rendered = render(Screen, {});
  await settle();
  return rendered.target;
}

async function settle() {
  for (let i = 0; i < 10; i += 1) {
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
  flushSync();
}

const text = (root: ParentNode) => (root as HTMLElement).textContent ?? '';

describe('the Activity screen', () => {
  it('shows today’s summary, the tabs with their counts and the newest requests', async () => {
    const root = await open();
    expect(one(root, 'h1').textContent).toBe('Activity');
    expect(all(root, '.act-chip').map((c) => c.firstChild?.textContent)).toContain('gamebanana.com');
    expect(text(one(root, '.act-sum'))).toContain('dl.fastmirror.example');
    const tabs = all(root, '[role="tab"]');
    expect(tabs.map((t) => t.textContent?.split(' ')[0])).toEqual(['Network', 'Files', 'App']);
    expect(text(tabs[0] as HTMLElement)).toMatch(/errors?/);
    expect(all(root, '.act-row').length).toBeGreaterThan(5);
  });

  it('opens an entry’s details, copies it and closes with Esc', async () => {
    const root = await open();
    click(one(root, '.act-row'));
    const aside = one(root, 'aside');
    expect(text(aside)).toContain('Full record');
    expect(text(aside)).toContain('Your Windows user name is hidden in logs');
    click(all(aside, 'button').find((b) => b.textContent?.includes('Copy entry')) as HTMLElement);
    await settle();
    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(expect.stringContaining('"req"'));
    expect(text(one(root, 'aside'))).toContain('Copied');
    keydown(document.body, 'Escape');
    expect(root.querySelector('aside')).toBeNull();
  });

  it('switches to the files tab from the summary', async () => {
    const root = await open();
    click(all(root, '.act-linkbtn').find((b) => b.textContent === 'Show in Files') as HTMLElement);
    await settle();
    expect(one(root, '[role="tab"][aria-selected="true"]').textContent).toContain('Files');
    expect(text(one(root, '.act-head'))).toContain('Part of change');
    // 186: the change in words, not its number.
    expect(text(one(root, '.act-body'))).toContain('Profile “Default” applied');
  });

  it('explains a blocked request named in the address, with the way to allow its host', async () => {
    const root = await open('activity-blocked', '#/activity/r-blocked');
    const aside = one(root, 'aside');
    expect(text(aside)).toContain('Request blocked.');
    expect(text(aside)).toContain('dl.fastmirror.example isn’t on the list of allowed hosts');
    expect(text(aside)).toContain('Redirects · 2');
    expect(text(aside)).toContain('Open Settings → Advanced');
  });

  it('says that no requests are made in local mode', async () => {
    const root = await open('activity-local');
    expect(text(root)).toContain('Local mode — no requests are made');
    expect(one(root, '[role="tab"]').textContent).toContain('off');
    expect(all(root, '.act-row')).toHaveLength(0);
    // The filters stay, so a longer range can show requests made before local mode (185).
    expect(one<HTMLInputElement>(root, 'input[type="search"]').disabled).toBe(false);
  });

  it('keeps the requests made before local mode in view, under the local-mode banner (185)', async () => {
    const root = await open('activity-local-history');
    const banner = one(root, '.dl-banner');
    expect(text(banner)).toContain('Local mode — no requests are made');
    expect(text(banner)).toContain('made before local mode was turned on');
    expect(all(root, '.act-row').length).toBeGreaterThan(0);
    expect(one(root, '[role="tab"]').textContent).not.toContain('off');
    // Today's summary counts them too instead of saying there were none.
    expect(text(one(root, '.act-sum'))).toContain('images.gamebanana.com');
    expect(text(one(root, '.act-sum'))).not.toContain('No requests today');
  });

  it('says there is nothing yet on a fresh install', async () => {
    const root = await open('activity-empty');
    expect(text(root)).toContain('Nothing yet');
    expect(text(one(root, '.act-sum'))).toContain('none yet');
  });

  it('shows a new request at the top while following', async () => {
    const root = await open();
    const before = all(root, '.act-row').length;
    const first = one(root, '.act-row .act-url__x').textContent;
    devtools.addLogRequest();
    await settle();
    expect(all(root, '.act-row').length).toBeGreaterThanOrEqual(before);
    expect(one(root, '.act-row .act-url__x').textContent).not.toBe(first);
  });

  it('opens the logs folder and says where it is', async () => {
    const root = await open();
    click(all(root, 'button').find((b) => b.textContent?.includes('Open logs folder')) as HTMLElement);
    await settle();
    expect(app.toasts.at(-1)).toMatchObject({ title: 'Logs folder opened in File Explorer.', text: expect.stringContaining('logs') });
  });
});
