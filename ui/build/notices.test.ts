import { describe, expect, it } from 'vitest';
import { noticesText, packageNotices, type PnpmLicenses } from './notices';

const pnpm: PnpmLicenses = {
  MIT: [{ name: 'svelte', versions: ['5.57.1'], paths: ['/m/svelte'], license: 'MIT' }],
  ISC: [{ name: '@lucide/svelte', versions: ['1.49.0'], paths: ['/m/lucide'], license: 'ISC' }],
};

const read = (path: string) => `text of ${path.replaceAll('\\', '/')}`;
const none = () => null;

describe('third-party notices', () => {
  it("take each bundled package's own licence file", () => {
    const files: Record<string, string[]> = { '/m/svelte': ['package.json', 'LICENSE.md'], '/m/lucide': ['LICENSE'] };
    expect(packageNotices(pnpm, (dir) => files[dir] ?? [], read, none)).toEqual([
      { name: '@lucide/svelte 1.49.0', license: 'ISC', text: 'text of /m/lucide/LICENSE' },
      { name: 'svelte 5.57.1', license: 'MIT', text: 'text of /m/svelte/LICENSE.md' },
    ]);
  });

  it('take every licence file of a dual-licensed package, but not an SPDX document', () => {
    const dual: PnpmLicenses = {
      'Apache-2.0 OR MIT': [{ name: '@tauri-apps/api', versions: ['2.12.1'], paths: ['/m/tauri'], license: 'Apache-2.0 OR MIT' }],
    };
    const files = ['LICENSE-APACHE-2.0', 'LICENSE-MIT', 'LICENSE.spdx', 'package.json'];
    expect(packageNotices(dual, () => files, read, none)[0]?.text).toBe(
      'text of /m/tauri/LICENSE-APACHE-2.0\n\ntext of /m/tauri/LICENSE-MIT',
    );
  });

  it("use the licence's standard text for a package that ships no licence file", () => {
    const standard = (license: string, dir: string) => (license === 'MIT' ? `standard MIT for ${dir}` : null);
    const notices = packageNotices(pnpm, (dir) => (dir === '/m/lucide' ? ['LICENSE'] : ['package.json']), read, standard);
    expect(notices[1]).toEqual({ name: 'svelte 5.57.1', license: 'MIT', text: 'standard MIT for /m/svelte' });
  });

  it('refuse a package with neither: the notices must be complete', () => {
    expect(() => packageNotices(pnpm, () => ['package.json'], read, none)).toThrow(/svelte 5\.57\.1|@lucide\/svelte/);
  });

  it('put every group under its heading after a header naming the version and the licence of deadloader', () => {
    const text = noticesText('0.1.0', [
      { title: 'Rust crates', body: 'MIT License (MIT), used by:\n  serde 1.0.0\n\nMIT text' },
      { title: 'Fonts', notices: [{ name: 'Golos Text', license: 'OFL-1.1', text: 'OFL text\r\n' }] },
    ]);
    expect(text.startsWith('deadloader 0.1.0 — third-party notices\n')).toBe(true);
    expect(text).toContain('GNU General Public License v3');
    expect(text.indexOf('Rust crates')).toBeLessThan(text.indexOf('serde 1.0.0'));
    expect(text.indexOf('Fonts')).toBeLessThan(text.indexOf('Golos Text (OFL-1.1)'));
    expect(text).toContain('Golos Text (OFL-1.1)\n\nOFL text\n');
    expect(text.endsWith('\n')).toBe(true);
    expect(text).not.toMatch(/\r/);
  });
});
