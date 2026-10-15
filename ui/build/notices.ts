// THIRD-PARTY-NOTICES.txt for the release zip (decision 167): the licence texts of everything deadloader.exe carries —
// Rust crates (cargo-about, about.toml), the JavaScript packages bundled into the interface, the fonts and the
// vendored libarchive. The release workflow runs it:
//   node ui/build/notices.ts <crates.txt> <pnpm-licenses.json> <out.txt> <version>

import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

export interface Notice {
  name: string;
  license: string;
  text: string;
}

/** A group of notices: either listed one by one, or a block already written (cargo-about's output). */
export type Group = { title: string; notices: Notice[] } | { title: string; body: string };

/** `pnpm licenses list --prod --json`: packages by licence. */
export type PnpmLicenses = Record<string, { name: string; versions: string[]; paths: string[]; license: string }[]>;

// LICENSE, LICENSE.md, LICENSE-MIT, LICENSE-APACHE-2.0, COPYING…; not LICENSE.spdx (an SPDX document, not a licence).
const LICENCE_FILE = /^(licen[cs]e|copying)(-[\w.-]+?)?(\.(md|txt))?$/i;
const RULE = '-'.repeat(80);
const HEAVY = '='.repeat(80);

/**
 * Every bundled package with the text of its own licence files (all of them for a dual licence), sorted by name. A
 * package that ships none gets `standard(license, dir)`: the licence's standard text; none either is an error.
 */
export function packageNotices(
  licenses: PnpmLicenses,
  list: (dir: string) => string[],
  read: (path: string) => string,
  standard: (license: string, dir: string) => string | null,
): Notice[] {
  const notices: Notice[] = [];
  for (const entries of Object.values(licenses)) {
    for (const pkg of entries) {
      pkg.versions.forEach((version, i) => {
        const dir = pkg.paths[i] ?? pkg.paths[0] ?? '';
        const files = list(dir)
          .filter((name) => LICENCE_FILE.test(name))
          .sort();
        const text =
          files.length > 0
            ? files.map((file) => read(join(dir, file)).trim()).join('\n\n')
            : standard(pkg.license, dir);
        if (text === null) {
          throw new Error(`${pkg.name} ${version} has no licence file in ${dir} and ${pkg.license} has no standard text`);
        }
        notices.push({ name: `${pkg.name} ${version}`, license: pkg.license, text });
      });
    }
  }
  return notices.sort((a, b) => a.name.localeCompare(b.name));
}

/** The whole file: a header, then each group under its heading. */
export function noticesText(version: string, groups: Group[]): string {
  const parts = [
    `deadloader ${version} — third-party notices`,
    '',
    'deadloader is free software under the GNU General Public License v3 (see LICENSE). It includes the',
    'third-party software below; each licence text is reproduced here as that licence requires.',
  ];
  for (const group of groups) {
    parts.push('', HEAVY, group.title, HEAVY);
    if ('body' in group) {
      parts.push(group.body.trim());
      continue;
    }
    for (const notice of group.notices) {
      parts.push(RULE, `${notice.name} (${notice.license})`, '', notice.text.trim(), '');
    }
  }
  return `${parts.join('\n').replace(/\r\n?/g, '\n').trimEnd()}\n`;
}

function main([crates, pnpm, out, version]: string[]): void {
  if (!crates || !pnpm || !out || !version) {
    throw new Error('usage: node ui/build/notices.ts <crates.txt> <pnpm-licenses.json> <out.txt> <version>');
  }
  const root = join(import.meta.dirname, '..', '..');
  const text = (path: string) => readFileSync(path, 'utf8');
  const fonts = join(root, 'ui', 'src', 'theme', 'fonts');
  // A package without a licence file: its declared licence's standard text, with the author package.json names.
  const standard = (license: string, dir: string): string | null => {
    const file = join(import.meta.dirname, 'licences', `${license}.txt`);
    let body: string;
    try {
      body = text(file);
    } catch {
      return null;
    }
    const meta = JSON.parse(text(join(dir, 'package.json'))) as { author?: string | { name?: string } };
    const author = typeof meta.author === 'string' ? meta.author : (meta.author?.name ?? 'its authors');
    return `The package ships no licence file; its package.json declares ${license}, by ${author}. The standard text:\n\n${body}`;
  };
  writeFileSync(
    out,
    noticesText(version, [
      { title: 'Rust crates', body: text(crates) },
      {
        title: 'JavaScript packages (bundled into the interface)',
        notices: packageNotices(JSON.parse(text(pnpm)) as PnpmLicenses, (dir) => readdirSync(dir), text, standard),
      },
      {
        title: 'Fonts',
        notices: [
          { name: 'Golos Text', license: 'OFL-1.1', text: text(join(fonts, 'GolosText-OFL.txt')) },
          { name: 'JetBrains Mono', license: 'OFL-1.1', text: text(join(fonts, 'JetBrainsMono-OFL.txt')) },
        ],
      },
      {
        title: 'Vendored C libraries',
        notices: [
          {
            name: 'libarchive 3.8.9 (RAR reading)',
            license: 'BSD-2-Clause and others, see below',
            text: text(join(root, 'crates', 'archive', 'vendor', 'libarchive', 'COPYING')),
          },
        ],
      },
    ]),
  );
}

if (import.meta.main) {
  main(process.argv.slice(2));
}
