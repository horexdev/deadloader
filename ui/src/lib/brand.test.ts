import { describe, expect, it } from 'vitest';
import { brandParts } from './brand';

describe('brandParts', () => {
  it('marks the product name in a text', () => {
    expect(brandParts('Welcome to deadloader')).toEqual([
      { text: 'Welcome to ', brand: false },
      { text: 'deadloader', brand: true },
    ]);
    expect(brandParts('deadloader’s own update check. deadloader.')).toEqual([
      { text: 'deadloader', brand: true },
      { text: '’s own update check. ', brand: false },
      { text: 'deadloader', brand: true },
      { text: '.', brand: false },
    ]);
  });

  it('leaves file names, paths and addresses alone (decision 193)', () => {
    for (const plain of [
      'All its data stays in the deadloader-data folder next to deadloader.exe.',
      'Move it to a folder you can write to, for example D:\\Games\\deadloader',
      'github.com/horexdev/deadloader',
      'deadloaders',
    ]) {
      expect(brandParts(plain), plain).toEqual([{ text: plain, brand: false }]);
    }
  });

  it('finds the name next to letters of scripts without spaces', () => {
    expect(brandParts('deadloaderはデータを保存できません')).toEqual([
      { text: 'deadloader', brand: true },
      { text: 'はデータを保存できません', brand: false },
    ]);
  });

  it('returns nothing for an empty text', () => {
    expect(brandParts('')).toEqual([]);
  });
});
