// Mods with content ratings (decisions 27, 78, 109): hidden by default; blurred or shown only after the user confirmed
// legal age once (D3). The catalog's NSFW filter and Settings → Catalog both choose through `chooseNsfw`.

import { api } from './api';
import { app } from './app.svelte';
import { prompts, saveCatalogSetting, type NsfwMode } from './prompts.svelte';

export type { NsfwMode };

/**
 * Applies the user's NSFW choice. The first time it goes beyond "hidden", D3 asks for the age; the confirmation is
 * saved and logged by the core. False when the user cancels or saving failed (the current choice stays).
 */
export async function chooseNsfw(mode: NsfwMode): Promise<boolean> {
  if (!app.settings) {
    return false;
  }
  if (mode !== 'hidden' && !app.settings.acknowledgements?.age_confirmed_at) {
    if (!(await prompts.askAge(mode))) {
      return false;
    }
    try {
      app.settings = await api.confirmAge();
    } catch (error) {
      app.fail(error);
      return false;
    }
  }
  return saveCatalogSetting({ nsfw: mode });
}
