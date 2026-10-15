// Questions the app asks from anywhere, answered in one place: D1 (open an external link, decision 31) and D3 (legal
// age before mods with content ratings are shown, 109). `openLink` and `chooseNsfw` ask here; `dialogs/Prompts.svelte`
// (mounted once by the shell) shows the question and answers it.

import { api, type Settings } from './api';
import { app } from './app.svelte';

export interface LinkAnswer {
  open: boolean;
  /** "Always open links immediately". */
  always: boolean;
}

export type NsfwMode = 'hidden' | 'blurred' | 'shown';

export interface LinkPrompt {
  url: string;
}

export interface AgePrompt {
  mode: Exclude<NsfwMode, 'hidden'>;
}

class Prompts {
  link: LinkPrompt | null = $state(null);
  age: AgePrompt | null = $state(null);
  #answerLink: ((answer: LinkAnswer) => void) | null = null;
  #answerAge: ((confirmed: boolean) => void) | null = null;

  /** Shows D1; a newer question replaces an unanswered one, which counts as cancelled. */
  askLink(url: string): Promise<LinkAnswer> {
    this.#answerLink?.({ open: false, always: false });
    this.link = { url };
    return new Promise((resolve) => {
      this.#answerLink = resolve;
    });
  }

  answerLink(answer: LinkAnswer): void {
    const resolve = this.#answerLink;
    this.#answerLink = null;
    this.link = null;
    resolve?.(answer);
  }

  /** Shows D3 for the mode the user picked. */
  askAge(mode: AgePrompt['mode']): Promise<boolean> {
    this.#answerAge?.(false);
    this.age = { mode };
    return new Promise((resolve) => {
      this.#answerAge = resolve;
    });
  }

  answerAge(confirmed: boolean): void {
    const resolve = this.#answerAge;
    this.#answerAge = null;
    this.age = null;
    resolve?.(confirmed);
  }
}

export const prompts = new Prompts();

/** Saves one change of the catalog settings through the app state; false (and a toast) when it was refused. */
export async function saveCatalogSetting(change: Record<string, unknown>): Promise<boolean> {
  const settings = app.settings;
  if (!settings) {
    return false;
  }
  // A plain copy: the state's proxies can't be cloned or sent.
  const plain = $state.snapshot(settings) as Settings;
  const next: Settings = { ...plain, catalog: { ...plain.catalog, ...change } };
  app.settings = next;
  try {
    app.settings = await api.updateSettings(next);
    return true;
  } catch (error) {
    app.settings = settings;
    app.fail(error);
    return false;
  }
}
