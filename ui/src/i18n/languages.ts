/** The UI languages (decision 106), with the codes `settings.json` stores (`core::settings::Lang`). */
export const LANGS = [
  'en',
  'pt-BR',
  'cs',
  'fr',
  'de',
  'it',
  'id',
  'ja',
  'ko',
  'es-419',
  'pl',
  'ru',
  'zh-Hans',
  'es-ES',
  'tr',
  'uk',
] as const;

export type Lang = (typeof LANGS)[number];

/** `general.language` in settings: the system language if we have it, otherwise English — or a fixed one. */
export type LanguageSetting = 'system' | Lang;

export const FALLBACK: Lang = 'en';

/** Each language in its own name, for the language picker. */
export const LANGUAGE_NAMES: Readonly<Record<Lang, string>> = {
  en: 'English',
  'pt-BR': 'Português (Brasil)',
  cs: 'Čeština',
  fr: 'Français',
  de: 'Deutsch',
  it: 'Italiano',
  id: 'Bahasa Indonesia',
  ja: '日本語',
  ko: '한국어',
  'es-419': 'Español (Latinoamérica)',
  pl: 'Polski',
  ru: 'Русский',
  'zh-Hans': '简体中文',
  'es-ES': 'Español (España)',
  tr: 'Türkçe',
  uk: 'Українська',
};

export function isLang(code: string): code is Lang {
  return (LANGS as readonly string[]).includes(code);
}

/** Languages whose regional variant does not matter: any region gets the one translation. */
const BY_LANGUAGE: Readonly<Record<string, Lang>> = {
  en: 'en',
  cs: 'cs',
  fr: 'fr',
  de: 'de',
  it: 'it',
  id: 'id',
  ja: 'ja',
  ko: 'ko',
  pl: 'pl',
  ru: 'ru',
  tr: 'tr',
  uk: 'uk',
};

/** Spanish-speaking countries of the Americas (and the US), which read Latin American Spanish. */
const LATIN_AMERICA = new Set([
  '419',
  'AR',
  'BO',
  'BZ',
  'CL',
  'CO',
  'CR',
  'CU',
  'DO',
  'EC',
  'GT',
  'HN',
  'MX',
  'NI',
  'PA',
  'PE',
  'PR',
  'PY',
  'SV',
  'US',
  'UY',
  'VE',
]);

function match(tag: string): Lang | null {
  let locale: Intl.Locale;
  try {
    // `maximize` fills in the likely script and region: "zh" → zh-Hans-CN, "pt" → pt-Latn-BR, "es" → es-Latn-ES.
    locale = new Intl.Locale(tag).maximize();
  } catch {
    return null;
  }
  switch (locale.language) {
    case 'zh':
      // Traditional Chinese readers are not given Simplified Chinese.
      return locale.script === 'Hans' ? 'zh-Hans' : null;
    case 'pt':
      // European Portuguese differs enough from Brazilian that English is the safer default.
      return locale.region === 'BR' ? 'pt-BR' : null;
    case 'es':
      return locale.region !== undefined && LATIN_AMERICA.has(locale.region) ? 'es-419' : 'es-ES';
    default:
      return BY_LANGUAGE[locale.language] ?? null;
  }
}

/** The first of the user's preferred languages (BCP 47 tags, best first) that we have, otherwise English. */
export function negotiate(preferred: readonly string[]): Lang {
  for (const tag of preferred) {
    const lang = match(tag);
    if (lang) {
      return lang;
    }
  }
  return FALLBACK;
}

/** The language to show for the `general.language` setting; an unknown value counts as "system", like in the core. */
export function resolveLanguage(setting: string, preferred: readonly string[]): Lang {
  return isLang(setting) ? setting : negotiate(preferred);
}
