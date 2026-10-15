import { describe, expect, it } from 'vitest';
import { settings as defaults } from '../../mock/fixtures';
import {
  asData,
  changed,
  checkHost,
  excerpt,
  maskUser,
  proxyAddress,
  proxyParts,
  speedKib,
  speedMb,
  wholeNumber,
  withCurrent,
} from './model';

describe('the settings model', () => {
  it('changes a copy and leaves the original alone', () => {
    const before = defaults();
    const after = changed(before, (s) => {
      s.network.retries = 2;
    });
    expect(asData(after).network.retries).toBe(2);
    expect(asData(before).network.retries).toBe(5);
  });

  it('keeps a hand-set value in a select', () => {
    expect(withCurrent([1, 2, 4], 3)).toEqual([1, 2, 3, 4]);
    expect(withCurrent([1, 2, 4], 2)).toEqual([1, 2, 4]);
    expect(withCurrent([null, 1, 5], 3)).toEqual([null, 1, 3, 5]);
  });

  it('converts the speed limit between MB/s and the stored KiB/s', () => {
    expect(speedKib(5)).toBe(5120);
    expect(speedMb(5120)).toBe(5);
    expect(speedMb(null)).toBeNull();
    expect(speedKib(null)).toBeNull();
  });

  it('accepts only whole numbers in range', () => {
    expect(wholeNumber(' 20 ', { min: 1, max: 1000 })).toEqual({ ok: true, value: 20 });
    expect(wholeNumber('0', { min: 1, max: 10 })).toEqual({ ok: false });
    expect(wholeNumber('2.5', { min: 1, max: 10 })).toEqual({ ok: false });
    expect(wholeNumber('', { min: 1, max: 10 })).toEqual({ ok: false });
  });

  it('builds and reads the manual proxy address', () => {
    expect(proxyAddress('https://proxy.example.com/', 8080)).toBe('http://proxy.example.com:8080');
    expect(proxyParts('http://proxy.example.com:3128')).toEqual({ host: 'proxy.example.com', port: '3128' });
  });

  it('checks an added domain like the core and names the problem', () => {
    expect(checkHost(' Files.Example.com ', [])).toEqual({ ok: true, host: 'files.example.com' });
    expect(checkHost('', [])).toEqual({ ok: false, problem: 'empty' });
    expect(checkHost('https://x.com/a', [])).toEqual({ ok: false, problem: 'not_a_host_only' });
    expect(checkHost('nodot', [])).toEqual({ ok: false, problem: 'invalid' });
    expect(checkHost('filecache12.gamebanana.com', [])).toEqual({ ok: false, problem: 'built_in' });
    expect(checkHost('a.com', ['a.com'])).toEqual({ ok: false, problem: 'listed' });
  });

  it('shows the line of a JSON error and the one before it', () => {
    expect(excerpt('{\n  "a": 1\n  "b": 2\n}', 3)).toEqual([
      { n: 2, text: '  "a": 1', error: false },
      { n: 3, text: '  "b": 2', error: true },
    ]);
  });

  it('hides the Windows user name in paths', () => {
    expect(maskUser(String.raw`C:\Users\Alex\Downloads\haze.zip`)).toBe(String.raw`C:\Users\…\Downloads\haze.zip`);
    expect(maskUser(String.raw`D:\Games`)).toBe(String.raw`D:\Games`);
  });
});
