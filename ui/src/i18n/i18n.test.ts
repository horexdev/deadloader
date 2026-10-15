import { afterEach, describe, expect, it } from 'vitest';
import {
  describe as describeMessage,
  formatDate,
  formatNumber,
  formatSize,
  formatTime,
  language,
  setLanguage,
  t,
} from './i18n';
import { LANGS } from './languages';

const NBSP = ' ';
const MiB = 1024 * 1024;

afterEach(async () => {
  await setLanguage('en', []);
});

describe('choosing the language', () => {
  it('starts in English, which is always loaded', () => {
    expect(language()).toBe('en');
    expect(t('e13')).toBe('Changes can’t be applied while Deadlock is running.');
  });

  it('negotiates "system" and loads the translation', async () => {
    expect(await setLanguage('system', ['ru-RU', 'en-US'])).toBe('ru');
    expect(language()).toBe('ru');
    expect(t('e2')).toBe('Deadlock не найден.');
  });

  it('lets an explicit language win', async () => {
    expect(await setLanguage('de', ['ru-RU'])).toBe('de');
    expect(t('e12')).toBe('Nicht genug Speicherplatz.');
  });

  it('keeps the language of the last call when loads overlap', async () => {
    const first = setLanguage('ja', []);
    const second = setLanguage('ko', []);
    await Promise.all([first, second]);
    expect(language()).toBe('ko');
  });

  it.each(LANGS)('loads %s', async (lang) => {
    await setLanguage(lang, []);
    expect(language()).toBe(lang);
    expect(t('internal')).not.toBe('internal');
  });
});

describe('t', () => {
  it('formats a value or an attribute', () => {
    expect(t('e15.todo')).toBe('Copy the link and open it yourself.');
    expect(t('n2', { name: 'Haze V2' })).toBe('Haze V2 installed and enabled.');
  });

  it('adds no bidi isolation marks around values (no right-to-left languages, decision 106)', () => {
    expect(t('n2', { name: 'Haze V2' })).not.toMatch(/[⁦-⁩]/);
  });

  it('returns the id of a text that does not exist, so it is visible', () => {
    expect(t('no-such-text')).toBe('no-such-text');
    expect(t('e1.nothing')).toBe('e1.nothing');
  });
});

describe('a message from the core', () => {
  it('becomes what happened, why and what to do', () => {
    expect(describeMessage({ id: 'E13', args: {} })).toEqual({
      what: 'Changes can’t be applied while Deadlock is running.',
      why: 'Windows locks the game’s files while it runs.',
      todo: 'Close Deadlock; deadloader applies the changes right after.',
      detail: null,
    });
  });

  it('picks the "why" that matches the reason', () => {
    const inUse = describeMessage({ id: 'E18', args: { reason: 'in_use', detail: 'os error 32' } });
    expect(inUse.why).toMatch(/in use by another program/);
    expect(inUse.detail).toBe('os error 32');
    const denied = describeMessage({ id: 'E18', args: { reason: 'access_denied' } });
    expect(denied.why).not.toBe(inUse.why);
  });

  it('falls back to the general "why" for a missing or unknown reason', () => {
    const general = describeMessage({ id: 'E18', args: { reason: 'other' } }).why;
    expect(describeMessage({ id: 'E18', args: {} }).why).toBe(general);
    expect(describeMessage({ id: 'E18', args: { reason: 'from_the_future' } }).why).toBe(general);
  });

  it('tells an unreadable profile file from one that is not a profile', () => {
    const unreadable = describeMessage({ id: 'E17', args: { reason: 'unreadable', file: 'D:\\x.json' } });
    const notAProfile = describeMessage({ id: 'E17', args: { reason: 'not_a_profile' } });
    expect(unreadable.why).toMatch(/can’t be read/);
    expect(unreadable.why).not.toBe(notAProfile.why);
    expect(unreadable.todo).not.toBe(notAProfile.todo);
  });

  it('reads the HTTP status and the wait of a busy GameBanana', () => {
    const busy = describeMessage({ id: 'E5', args: { status: '429', retry_in_s: '12' } });
    expect(busy.why).toContain('HTTP 429');
    expect(busy.todo).toContain('12 seconds');
    expect(describeMessage({ id: 'E5', args: { status: '503' } }).why).toMatch(/server reported an error/);
    expect(describeMessage({ id: 'E5', args: {} }).todo).toMatch(/in a moment/);
  });

  it('says why a download failed, in the words of the downloads crate', () => {
    expect(describeMessage({ id: 'E6', args: { reason: 'dns', host: 'files.gamebanana.com' } }).why).toMatch(
      /couldn’t be found/,
    );
    const status = describeMessage({ id: 'E6', args: { reason: 'http_status', status: '404' } });
    expect(status.why).toContain('HTTP 404');
    expect(describeMessage({ id: 'E6', args: { reason: 'local_mode' } }).todo).toMatch(/catalog/);
    expect(describeMessage({ id: 'E7', args: { reason: 'size' } }).why).toMatch(/size/);
    expect(describeMessage({ id: 'E7', args: { reason: 'checksum' } }).why).toMatch(/checksum/);
  });

  it('treats a blocked host without a reason as not on the allowlist', () => {
    const plain = describeMessage({ id: 'E16', args: { host: 'x.example' } });
    expect(plain.why).toContain('x.example');
    expect(plain.todo).toMatch(/Settings → Advanced/);
    const insecure = describeMessage({ id: 'E16', args: { reason: 'not_https', from: 'gamebanana.com' } });
    expect(insecure.why).toMatch(/HTTPS/);
    expect(insecure.todo).not.toMatch(/Settings → Advanced/);
  });

  it('names the entry an archive problem is about', () => {
    const m = describeMessage({ id: 'E8', args: { reason: 'not_a_vpk', entry: 'haze.vpk' } });
    expect(m.why).toContain('haze.vpk');
  });

  it('shows disk space in Explorer units when the core knows the sizes', () => {
    const m = describeMessage({ id: 'E12', args: { needed: String(236 * MiB), available: String(96 * MiB) } });
    expect(m.why).toContain(`236${NBSP}MB`);
    expect(m.why).toContain(`96.0${NBSP}MB`);
    expect(describeMessage({ id: 'E12', args: {} }).why).toBe('The disk is full.');
  });

  it('shows the limits of E9 as sizes or counts, depending on the reason', () => {
    const big = describeMessage({ id: 'E9', args: { reason: 'too_large', limit: String(2048 * MiB) } });
    expect(big.why).toContain(`2.00${NBSP}GB`);
    const many = describeMessage({ id: 'E9', args: { reason: 'too_many_files', limit: '256', found: '300' } });
    expect(many.why).toContain('300');
    expect(many.why).toContain('256');
  });

  it('gives the place a damaged file broke at, when it is known', () => {
    const at = describeMessage({
      id: 'E23',
      args: { file: 'settings.json', reason: 'damaged', line: '3', column: '7' },
    });
    expect(at.what).toContain('settings.json');
    expect(at.why).toMatch(/line 3, column 7/);
    expect(at.todo).toMatch(/default settings/);
    const nowhere = describeMessage({ id: 'E23', args: { file: 'settings.json', reason: 'damaged' } });
    expect(nowhere.why).not.toMatch(/line/);
    const manifest = describeMessage({ id: 'E23', args: { file: 'manifest.json', reason: 'newer' } });
    expect(manifest.todo).toContain('manifest.json.bak');
  });

  it('lists what an archive without mod files holds, shortened', () => {
    const two = describeMessage({ id: 'E10', args: { contents: 'README.txt\npreview.png' } });
    expect(two.why).toContain('README.txt and preview.png');
    const lines = ['a.txt', 'b.txt', 'c.txt', 'd.txt', 'e.txt', 'f.txt', 'g.txt'].join('\n');
    expect(describeMessage({ id: 'E10', args: { contents: lines } }).why).toContain(
      'a.txt, b.txt, c.txt, d.txt, e.txt, and 2 more',
    );
    expect(describeMessage({ id: 'E10', args: { contents: '' } }).why).toBe('The archive is empty.');
  });

  it('treats an id this UI does not know as a mistake of deadloader', () => {
    expect(describeMessage({ id: 'E99', args: { detail: 'x' } })).toEqual({
      ...describeMessage({ id: 'Internal', args: {} }),
      detail: 'x',
    });
  });

  it('is translated', async () => {
    await setLanguage('de', []);
    const m = describeMessage({ id: 'E12', args: { needed: String(236 * MiB), available: String(96 * MiB) } });
    expect(m.what).toBe('Nicht genug Speicherplatz.');
    expect(m.why).toContain(`96,0${NBSP}MB`);
  });
});

describe('notifications', () => {
  it('have the same shape, with an optional "why"', () => {
    expect(describeMessage({ id: 'N4', args: {} })).toEqual({
      what: 'Deadlock is running.',
      why: 'Changes will be applied automatically when you close the game.',
      todo: null,
      detail: null,
    });
    expect(describeMessage({ id: 'N9', args: { version: '1.3.0' } }).why).toBeNull();
  });

  it('say when the saved catalog is from', () => {
    const now = new Date(2026, 9, 15, 16, 0);
    const today = describeMessage({ id: 'N7', args: { saved: new Date(2026, 9, 15, 14, 20) } }, now);
    expect(today.what).toBe(`Offline — showing the catalog saved at ${formatTime(new Date(2026, 9, 15, 14, 20))}.`);
    const before = describeMessage({ id: 'N7', args: { saved: new Date(2026, 9, 12, 9, 0) } }, now);
    expect(before.what).toBe(`Offline — showing the catalog saved on ${formatDate(new Date(2026, 9, 12), now)}.`);
  });

  it('use plural forms', () => {
    expect(describeMessage({ id: 'N3', args: { count: 1 } }).what).toBe('1 mod update available.');
    expect(describeMessage({ id: 'N3', args: { count: '3' } }).what).toBe('3 mod updates available.');
  });
});

describe('plural forms of Slavic languages', () => {
  const cases = {
    ru: ['файл', 'файла', 'файлов', 'файл', 'файла'],
    uk: ['файл', 'файли', 'файлів', 'файл', 'файли'],
    pl: ['plik', 'pliki', 'plików', 'plików', 'pliki'],
    cs: ['soubor', 'soubory', 'souborů', 'souborů', 'souborů'],
  } as const;
  const counts = [1, 2, 5, 21, 22] as const;

  it.each(Object.entries(cases))('%s', async (lang, words) => {
    await setLanguage(lang, []);
    counts.forEach((n, i) => {
      const text = describeMessage({ id: 'N6', args: { count: n } }).what;
      expect(text, `${n}`).toMatch(new RegExp(`(^|\\s)${n}\\s${words[i]}(?=[\\s,.])`, 'u'));
    });
  });
});

describe('sizes, in Windows Explorer units', () => {
  it.each([
    [0, '0 bytes'],
    [1, '1 byte'],
    [532, '532 bytes'],
    [1023, '1,023 bytes'],
    [1024, '1.00 KB'],
    [1340, '1.30 KB'],
    [23506, '22.9 KB'],
    [2400016, '2.28 MB'],
    [2400000000, '2.23 GB'],
    [1000 * MiB, '0.97 GB'],
    [5 * 1024 * 1024 * MiB, '5.00 TB'],
  ])('%d → %s', (bytes, expected) => {
    // Truncated to three significant digits, like StrFormatByteSize; the unit never wraps onto its own line.
    expect(formatSize(bytes)).toBe(expected.replaceAll(' ', NBSP));
  });

  it.each([
    ['de', '1,30 KB'],
    ['fr', '1,30 Ko'],
    ['ru', '1,30 КБ'],
    ['uk', '1,30 КБ'],
    ['ja', '1.30 KB'],
  ] as const)('%s labels them like its Explorer', async (lang, expected) => {
    await setLanguage(lang, []);
    expect(formatSize(1340)).toBe(expected.replaceAll(' ', NBSP));
  });
});

describe('numbers and dates', () => {
  it('follow the language', async () => {
    expect(formatNumber(1234567)).toBe('1,234,567');
    await setLanguage('de', []);
    expect(formatNumber(1234567)).toBe('1.234.567');
  });

  it('give the year only when it is not this year', () => {
    const now = new Date(2026, 9, 15);
    expect(formatDate(new Date(2026, 9, 12), now)).not.toContain('2026');
    expect(formatDate(new Date(2025, 9, 12), now)).toContain('2025');
  });
});
