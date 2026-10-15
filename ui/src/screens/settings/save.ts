// Saving a change made on the Settings screen: shown at once, saved through the core, set back with a toast when the
// core refuses it. The settings live in `app` (one copy for every screen).

import { api } from '../../lib/api';
import type { AppState } from '../../lib/app.svelte';
import { changed, type SettingsData } from './model';

export async function saveSettings(app: AppState, change: (s: SettingsData) => void): Promise<boolean> {
  const before = app.settings;
  if (!before) {
    return false;
  }
  const next = changed(before, change);
  app.settings = next;
  try {
    app.settings = await api.updateSettings(next);
    return true;
  } catch (error) {
    app.settings = before;
    app.fail(error);
    return false;
  }
}
