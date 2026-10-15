import { describe, expect, it } from 'vitest';
import settingsRs from '../../../crates/core/src/settings.rs?raw';
import { LANGS, isLang, negotiate, resolveLanguage } from './languages';

describe('the language list', () => {
  it('is the sixteen languages of decision 106', () => {
    expect(LANGS).toEqual([
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
    ]);
  });

  it('uses the codes the core stores in settings.json', () => {
    const codes = [...settingsRs.matchAll(/Lang::\w+ => "([\w-]+)",/g)].map((m) => m[1]);
    expect(new Set(codes)).toEqual(new Set(LANGS));
  });

  it('tells our codes from anything else', () => {
    expect(isLang('zh-Hans')).toBe(true);
    expect(isLang('system')).toBe(false);
    expect(isLang('pt-PT')).toBe(false);
  });
});

describe('negotiating the system language', () => {
  it.each([
    ['ru-RU', 'ru'],
    ['pt-BR', 'pt-BR'],
    // 177: the nearest language we have, rather than English.
    ['pt-PT', 'pt-BR'],
    ['zh-CN', 'zh-Hans'],
    ['zh-TW', 'zh-Hans'],
    ['es-MX', 'es-419'],
    ['es-ES', 'es-ES'],
  ])('%s → %s', (requested, expected) => {
    expect(negotiate([requested])).toBe(expected);
  });

  it.each([
    ['de-AT', 'de'],
    ['en-GB', 'en'],
    ['uk-UA', 'uk'],
    ['zh', 'zh-Hans'],
    ['zh-SG', 'zh-Hans'],
    ['zh-Hans-HK', 'zh-Hans'],
    ['zh-HK', 'zh-Hans'],
    ['zh-Hant', 'zh-Hans'],
    ['pt', 'pt-BR'],
    ['es', 'es-ES'],
    ['es-419', 'es-419'],
    ['es-US', 'es-419'],
    ['es-AR', 'es-419'],
    ['ja-JP', 'ja'],
    ['ko-KR', 'ko'],
    ['id-ID', 'id'],
    ['in', 'id'],
    ['tr-TR', 'tr'],
    ['cs-CZ', 'cs'],
    ['pl-PL', 'pl'],
    ['it-CH', 'it'],
    ['fr-CA', 'fr'],
  ])('%s → %s', (requested, expected) => {
    expect(negotiate([requested])).toBe(expected);
  });

  it('takes the first language of the list that we have', () => {
    expect(negotiate(['nl-NL', 'ru-RU', 'en-US'])).toBe('ru');
    expect(negotiate(['pt-PT', 'ru-RU'])).toBe('pt-BR');
    expect(negotiate(['nl-NL', 'de-DE'])).toBe('de');
  });

  it('falls back to English', () => {
    expect(negotiate([])).toBe('en');
    expect(negotiate(['nl-NL', 'th-TH', 'ar'])).toBe('en');
    expect(negotiate(['', 'not a tag', '*'])).toBe('en');
  });
});

describe('the language setting', () => {
  it('negotiates "system" against the preferred languages', () => {
    expect(resolveLanguage('system', ['pl-PL', 'en-US'])).toBe('pl');
    expect(resolveLanguage('system', ['pt-PT'])).toBe('pt-BR');
  });

  it('lets an explicit language win over the system', () => {
    expect(resolveLanguage('ja', ['ru-RU'])).toBe('ja');
    expect(resolveLanguage('es-419', ['es-ES'])).toBe('es-419');
  });

  it('treats an unknown value like "system", as the core does', () => {
    expect(resolveLanguage('xx', ['uk-UA'])).toBe('uk');
  });
});
