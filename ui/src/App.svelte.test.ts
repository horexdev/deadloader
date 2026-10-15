import { flushSync, tick } from 'svelte';
import { describe, expect, it, vi } from 'vitest';

// The whole app against the mock core, the same one `pnpm dev:mock` uses.
vi.mock('@tauri-apps/api/core', () => import('./mock/core'));

class FakeResizeObserver {
  observe(): void {}
  unobserve(): void {}
  disconnect(): void {}
}
vi.stubGlobal('ResizeObserver', FakeResizeObserver);

const { router } = await import('./lib/router.svelte');
const { resetMock } = await import('./mock/core');
const { t } = await import('./i18n');
const { default: App } = await import('./App.svelte');
const { one, render } = await import('./lib/test/render');

/** Lets the start and the commands it sends finish. */
async function settle(): Promise<void> {
  for (let i = 0; i < 10; i += 1) {
    await tick();
    await new Promise((resolve) => setTimeout(resolve, 0));
  }
  flushSync();
}

/** Waits until a lazily loaded part has appeared (the first import of a module takes a while in tests). */
async function shown(target: HTMLElement, selector: string): Promise<Element> {
  return vi.waitFor(
    () => {
      flushSync();
      const found = target.querySelector(selector);
      if (!found) {
        throw new Error(`${selector} not shown yet`);
      }
      return found;
    },
    { timeout: 10_000, interval: 20 },
  );
}

function heading(target: HTMLElement): string {
  return one(target, 'h1').textContent?.trim() ?? '';
}

describe('the app', () => {
  it('loads a screen when its area is opened and shows it at once when it comes back', async () => {
    history.replaceState(null, '', '/#/downloads');
    resetMock();
    router.go('downloads');
    const { target } = render(App, {});
    await settle();
    await shown(target, 'h1');
    expect(heading(target)).toBe(t('downloads-title'));

    router.go('activity');
    await shown(target, '.act-screen');
    expect(heading(target)).toBe(t('activity-title'));

    // Loaded before: no blank frame while it would load again.
    router.go('downloads');
    flushSync();
    expect(heading(target)).toBe(t('downloads-title'));
  });

  it('loads the first-run wizard when the notice was never acknowledged', async () => {
    history.replaceState(null, '', '/?scenario=wizard#/catalog');
    resetMock();
    router.go('catalog');
    const { target } = render(App, {});
    await settle();
    expect(await shown(target, '.wz-window')).not.toBeNull();
  });

  it('asks before opening a link from the first-run wizard, over the wizard (D1)', async () => {
    history.replaceState(null, '', '/?scenario=wizard#/catalog');
    resetMock();
    router.go('catalog');
    const { target } = render(App, {});
    await settle();
    await shown(target, '.wz-window');
    const source = [...target.querySelectorAll<HTMLElement>('a, button')].find((e) => e.textContent?.includes('Source code'));
    source?.click();
    // D1 shows at once, while the wizard is still on screen.
    const dialog = await shown(target, '.dl-dialog');
    expect(dialog.textContent).toContain('github.com');
    expect(target.querySelector('.wz-window')).not.toBeNull();
    const cancel = [...dialog.querySelectorAll<HTMLButtonElement>('button')].find((b) => b.textContent?.trim() === 'Cancel');
    cancel?.click();
    flushSync();
    expect(target.querySelector('.dl-dialog')).toBeNull();
  });

  it('shows one D1 in the main window, not one per frame', async () => {
    history.replaceState(null, '', '/#/catalog');
    resetMock();
    router.go('catalog');
    const { target } = render(App, {});
    await settle();
    await shown(target, '.dl-sidebar, nav');
    const { openLink } = await import('./lib/links');
    void openLink('https://gamebanana.com/');
    await shown(target, '.dl-dialog');
    expect(target.querySelectorAll('.dl-dialog')).toHaveLength(1);
    const { prompts } = await import('./lib/prompts.svelte');
    prompts.answerLink({ open: false, always: false });
    flushSync();
  });
});
