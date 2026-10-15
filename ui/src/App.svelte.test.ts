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
const { app } = await import('./lib/app.svelte');
const { resetMock } = await import('./mock/core');
const { t } = await import('./i18n');
const { default: App } = await import('./App.svelte');
const { all, click, one, render } = await import('./lib/test/render');

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

  it('opens the catalog without an error when the first-run wizard is finished with the catalog on', async () => {
    history.replaceState(null, '', '/?scenario=wizard#/library');
    resetMock();
    router.go('library');
    app.toasts = [];
    const { target } = render(App, {});
    await settle();
    await shown(target, '.wz-window');
    const button = (text: string): HTMLButtonElement => {
      const found = all<HTMLButtonElement>(target, 'button').find((b) => b.textContent?.trim() === text);
      if (!found) {
        throw new Error(`no button "${text}"`);
      }
      return found;
    };
    click(button('Next'));
    click(one(target, '.wz-consent input'));
    click(button('Next'));
    click(one(target, 'input[value="catalog"]'));
    while (target.querySelector('input[role="switch"]') === null) {
      click(button('Next'));
      await settle();
    }
    // The update check on, as in the report.
    click(one(target, 'input[role="switch"]'));
    click(button('Next'));
    await settle();
    click(button('Finish setup'));
    await settle();
    await vi.waitFor(() => {
      flushSync();
      expect(target.querySelector('.wz-window')).toBeNull();
    });

    // Finishing replaced the wizard with the app: nothing read the wizard's mode once it was gone (TypeError).
    expect(app.toasts.filter((toast) => toast.kind === 'error')).toEqual([]);
    expect(router.current).toBe('catalog');
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

  it('slides the content from where it was when the sidebar collapses or expands', async () => {
    const viewport = (window as unknown as { happyDOM: { setViewport(size: { width: number; height: number }): void } })
      .happyDOM;
    // A wide window: the user's choice, not the automatic rail.
    viewport.setViewport({ width: 1280, height: 800 });
    try {
      history.replaceState(null, '', '/#/profiles');
      resetMock();
      router.go('profiles');
      const { target } = render(App, {});
      await settle();
      await shown(target, '.dl-sidebar');
      const content = one(target, 'main.content');
      // happy-dom has no layout: the content starts where the sidebar ends.
      vi.spyOn(content, 'getBoundingClientRect').mockImplementation(
        () => ({ left: target.querySelector('.dl-sidebar--rail') ? 56 : 240 }) as DOMRect,
      );

      click(one(target, `button[aria-label="${t('sidebar-collapse')}"]`));
      await settle();
      expect(one(target, '.dl-sidebar').classList).toContain('dl-sidebar--rail');
      expect(content.hasAttribute('data-sliding')).toBe(true);
      expect(content.style.getPropertyValue('--slide-from')).toBe('184px');

      // Something inside finishing its own animation doesn't end the slide.
      const inner = content.appendChild(document.createElement('div'));
      inner.dispatchEvent(new Event('animationend', { bubbles: true }));
      flushSync();
      expect(content.hasAttribute('data-sliding')).toBe(true);
      content.dispatchEvent(new Event('animationend'));
      flushSync();
      expect(content.hasAttribute('data-sliding')).toBe(false);

      click(one(target, `button[aria-label="${t('sidebar-expand')}"]`));
      await settle();
      expect(content.hasAttribute('data-sliding')).toBe(true);
      expect(content.style.getPropertyValue('--slide-from')).toBe('-184px');
    } finally {
      viewport.setViewport({ width: 1024, height: 768 });
    }
  });
});
