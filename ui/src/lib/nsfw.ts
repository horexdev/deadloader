// Stand-in until the Catalog screen brings D3 (the age confirmation, decision 109). It saves the NSFW mode through
// `app` when no confirmation is needed (hidden, or the age was confirmed before) and otherwise answers "cancelled",
// because only D3 may record the confirmation. The Catalog owner's version replaces this file.

import { api } from './api';
import { app } from './app.svelte';

export async function chooseNsfw(mode: 'hidden' | 'blurred' | 'shown'): Promise<boolean> {
  const settings = app.settings;
  if (!settings) {
    return false;
  }
  const confirmed = typeof settings.acknowledgements?.age_confirmed_at === 'string';
  if (mode !== 'hidden' && !confirmed) {
    return false;
  }
  const next = { ...settings, catalog: { ...settings.catalog, nsfw: mode } };
  try {
    app.settings = await api.updateSettings(next);
    return true;
  } catch (error) {
    app.fail(error);
    return false;
  }
}
