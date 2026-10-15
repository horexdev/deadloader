import { flushSync } from 'svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

// The real mock core answers; every call is recorded on the way.
const calls = vi.hoisted(() => [] as [string, unknown][]);
vi.mock('@tauri-apps/api/core', async () => {
  const core = await import('../mock/core');
  return {
    ...core,
    invoke: (command: string, args?: Record<string, unknown>) => {
      calls.push([command, args]);
      return core.invoke(command, args);
    },
  };
});

const { app } = await import('./app.svelte');
const { resetMock } = await import('../mock/core');
const { openLink } = await import('./links');
const { prompts } = await import('./prompts.svelte');
const { default: Prompts } = await import('./dialogs/Prompts.svelte');
const { click, one, render } = await import('./test/render');

const URL = 'https://gamebanana.com/mods/655767';

function opened(): unknown[] {
  return calls.filter(([command]) => command === 'open_link').map(([, args]) => args);
}

async function settle(): Promise<void> {
  for (let i = 0; i < 5; i += 1) {
    await Promise.resolve();
  }
  flushSync();
}

beforeEach(async () => {
  resetMock();
  calls.length = 0;
  app.settings = await import('../mock/core').then((core) => core.invoke('settings'));
  app.toasts = [];
});

describe('openLink and D1 (decision 31)', () => {
  it('asks first, showing the full address with its host, and opens only on Open', async () => {
    const { target } = render(Prompts, {});
    const done = openLink(URL);
    flushSync();
    const dialog = one(document.body, '[role="dialog"]');
    expect(dialog.textContent).toContain('Open this link in your browser?');
    expect(one(dialog, '.dl-url').textContent).toBe(URL);
    expect(one(dialog, '.dl-url b').textContent).toBe('gamebanana.com');
    expect(opened()).toEqual([]);
    const open = [...dialog.querySelectorAll('button')].find((b) => b.textContent?.trim() === 'Open');
    click(open as HTMLElement);
    await done;
    expect(opened()).toEqual([{ url: URL }]);
    expect(target.querySelector('[role="dialog"]')).toBeNull();
  });

  it('does nothing on Cancel', async () => {
    render(Prompts, {});
    const done = openLink(URL);
    flushSync();
    const cancel = [...document.querySelectorAll('[role="dialog"] button')].find(
      (b) => b.textContent?.trim() === 'Cancel',
    );
    click(cancel as HTMLElement);
    await done;
    expect(opened()).toEqual([]);
  });

  it('remembers "always open links immediately" and then opens at once', async () => {
    const done = openLink(URL);
    prompts.answerLink({ open: true, always: true });
    await done;
    expect(app.settings?.catalog?.external_links).toBe('open_immediately');
    expect(opened()).toEqual([{ url: URL }]);
    calls.length = 0;
    await openLink('https://gamebanana.com/');
    expect(prompts.link).toBeNull();
    expect(opened()).toEqual([{ url: 'https://gamebanana.com/' }]);
  });

  it('shows why a link could not be opened', async () => {
    const done = openLink('http://example.org/');
    prompts.answerLink({ open: true, always: false });
    await done;
    await settle();
    expect(app.toasts.map((toast) => toast.kind)).toEqual(['error']);
  });
});
