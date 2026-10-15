import { flushSync } from 'svelte';
import { describe, expect, it, vi } from 'vitest';

const calls: { command: string; args: Record<string, unknown> }[] = [];
vi.mock('@tauri-apps/api/core', async () => {
  const mock = await import('../mock/core');
  return {
    ...mock,
    invoke: (command: string, args: Record<string, unknown> = {}) => {
      calls.push({ command, args });
      return mock.invoke(command, args);
    },
  };
});

const { AppState } = await import('../lib/app.svelte');
const { resetMock } = await import('../mock/core');
const { default: ProfileSwitcher } = await import('./ProfileSwitcher.svelte');
const { all, click, keydown, one, render } = await import('../lib/test/render');

async function open() {
  resetMock();
  const app = new AppState();
  await app.start();
  const anchor = document.createElement('button');
  document.body.append(anchor);
  const onclose = vi.fn();
  const onmanage = vi.fn();
  const { target } = render(ProfileSwitcher, { app, anchor, onclose, onmanage });
  await vi.waitFor(() => expect(target.querySelectorAll('[role="option"]').length).toBeGreaterThan(0));
  flushSync();
  calls.length = 0;
  return { app, target, onclose, onmanage };
}

describe('the sidebar’s profile switcher', () => {
  it('lists the profiles with the selected one marked and focused', async () => {
    const { target } = await open();
    const options = all(target, '[role="option"]');
    expect(options.map((o) => o.querySelector('.name')?.textContent)).toEqual(['Default', 'Evening', 'Ranked', 'Sasha']);
    expect(options[0]?.getAttribute('aria-selected')).toBe('true');
    expect(document.activeElement).toBe(options[0]);
  });

  it('selects another profile without applying it (F6)', async () => {
    const { app, target, onclose } = await open();
    click(all(target, '[role="option"]')[1] as HTMLElement);
    await vi.waitFor(() => expect(onclose).toHaveBeenCalled());
    expect(calls.map((c) => c.command)).toContain('select_profile');
    expect(calls.map((c) => c.command)).not.toContain('apply');
    expect(app.library?.selected_profile).toBe('Evening');
  });

  it('moves with the arrow keys, closes on Esc and opens the Profiles screen', async () => {
    const { target, onclose, onmanage } = await open();
    const options = all(target, '[role="option"]');
    keydown(options[0] as HTMLElement, 'ArrowDown');
    expect(document.activeElement).toBe(options[1]);
    keydown(options[1] as HTMLElement, 'ArrowUp');
    expect(document.activeElement).toBe(options[0]);
    keydown(options[0] as HTMLElement, 'Escape');
    expect(onclose).toHaveBeenCalledOnce();
    click(one(target, '.manage'));
    expect(onmanage).toHaveBeenCalledOnce();
  });
});
