import { afterEach, describe, expect, it } from 'vitest';
import { formatTime, language, setLanguage, setRegion } from '../i18n';
import { applyLocale } from './locale';

afterEach(async () => {
  setRegion(undefined);
  await setLanguage('en', []);
});

const twenty_past_two = new Date(2026, 9, 15, 14, 20);

describe('the language and regional format at start', () => {
  it("use Windows' languages for \"system\" and its region for the clock (178, 179)", async () => {
    const lang = await applyLocale({
      systemLocale: () => Promise.resolve({ languages: ['pt-PT', 'en-US'], region: 'pt-PT' }),
      settings: () => Promise.resolve({ general: { language: 'system' } }),
    });
    expect(lang).toBe('pt-BR');
    expect(language()).toBe('pt-BR');
    expect(formatTime(twenty_past_two)).toBe('14:20');
  });

  it('use a language chosen in the settings as it is', async () => {
    const lang = await applyLocale({
      systemLocale: () => Promise.resolve({ languages: ['ru-RU'], region: 'ru-RU' }),
      settings: () => Promise.resolve({ general: { language: 'ja' } }),
    });
    expect(lang).toBe('ja');
  });

  it('start in a language even when the core cannot tell', async () => {
    const lang = await applyLocale({
      systemLocale: () => Promise.reject(new Error('no core')),
      settings: () => Promise.reject(new Error('no core')),
    });
    expect(lang).toBe(language());
  });
});
