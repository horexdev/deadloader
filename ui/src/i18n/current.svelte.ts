import { FALLBACK, type Lang } from './languages';

// The one reactive value of the i18n module: everything that formats text reads it, so components re-render when
// the language changes.
let current = $state<Lang>(FALLBACK);

/** The language shown now. Reactive: components that read it (or call `t`) update when it changes. */
export function language(): Lang {
  return current;
}

export function showLanguage(lang: Lang): void {
  current = lang;
}
