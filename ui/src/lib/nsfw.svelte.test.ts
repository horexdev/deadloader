import { flushSync } from 'svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/core', () => import('../mock/core'));

const { app } = await import('./app.svelte');
const core = await import('../mock/core');
const { chooseNsfw } = await import('./nsfw');
const { prompts } = await import('./prompts.svelte');
const { default: Prompts } = await import('./dialogs/Prompts.svelte');
const { click, one, render } = await import('./test/render');

beforeEach(async () => {
  core.resetMock();
  app.settings = await core.invoke('settings');
});

describe('chooseNsfw and D3 (decision 109)', () => {
  it('asks for the age the first time, then saves the confirmation and the choice', async () => {
    render(Prompts, {});
    const chosen = chooseNsfw('blurred');
    flushSync();
    const dialog = one(document.body, '[role="dialog"]');
    expect(dialog.textContent).toContain('Show mods with sensitive content?');
    expect(dialog.textContent).toContain('You chose: Blurred.');
    const confirm = [...dialog.querySelectorAll('button')].find((b) => b.textContent?.includes('legal age'));
    click(confirm as HTMLElement);
    expect(await chosen).toBe(true);
    expect(app.settings?.catalog?.nsfw).toBe('blurred');
    expect(app.settings?.acknowledgements?.age_confirmed_at).toEqual(expect.any(String));
    const saved = await core.invoke<Record<string, Record<string, unknown>>>('settings');
    expect(saved.catalog?.nsfw).toBe('blurred');
  });

  it('keeps the current choice when the user cancels', async () => {
    const chosen = chooseNsfw('shown');
    expect(prompts.age?.mode).toBe('shown');
    prompts.answerAge(false);
    expect(await chosen).toBe(false);
    expect(app.settings?.catalog?.nsfw).toBe('hidden');
    expect(app.settings?.acknowledgements?.age_confirmed_at).toBeNull();
  });

  it('asks only once', async () => {
    const first = chooseNsfw('shown');
    prompts.answerAge(true);
    await first;
    expect(await chooseNsfw('blurred')).toBe(true);
    expect(prompts.age).toBeNull();
    expect(app.settings?.catalog?.nsfw).toBe('blurred');
  });

  it('goes back to hidden without asking', async () => {
    expect(await chooseNsfw('hidden')).toBe(true);
    expect(prompts.age).toBeNull();
    expect(app.settings?.catalog?.nsfw).toBe('hidden');
  });
});
