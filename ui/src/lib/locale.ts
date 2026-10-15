// The language and regional format the app starts in: the `general.language` setting, Windows' preferred UI languages
// for "system" (decision 179) and Windows' regional format for the clock and numbers (178).

import { setLanguage, setRegion, type Lang } from '../i18n';
import type { SystemLocale } from './api';

/** Where the values come from: the core, or fakes in tests. */
export interface LocaleSource {
  systemLocale: () => Promise<SystemLocale>;
  settings: () => Promise<Record<string, Record<string, unknown>>>;
}

/** Applies them before the app mounts. Whatever the core can't tell falls back to the WebView's own list and "system". */
export async function applyLocale(source: LocaleSource): Promise<Lang> {
  const system = await source
    .systemLocale()
    .catch((): SystemLocale => ({ languages: [...navigator.languages], region: null }));
  setRegion(system.region ?? undefined);
  const setting = await source
    .settings()
    .then((settings) => {
      const value = settings.general?.language;
      return typeof value === 'string' ? value : 'system';
    })
    .catch(() => 'system');
  const preferred = system.languages.length > 0 ? system.languages : navigator.languages;
  return setLanguage(setting, preferred);
}
