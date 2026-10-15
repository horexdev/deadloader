import { flushSync } from 'svelte';
import { describe, expect, it, vi } from 'vitest';
import type { FolderFile, ModView } from '../../lib/api';
import { all, one, render } from '../../lib/test/render';
import FolderFiles from './FolderFiles.svelte';

class FakeResizeObserver {
  observe(): void {}
  unobserve(): void {}
  disconnect(): void {}
}
vi.stubGlobal('ResizeObserver', FakeResizeObserver);

const HAZE: ModView = {
  id: 'haze',
  name: 'Haze — Midnight Bloom',
  author: null,
  source: { kind: 'local', imported_from: null },
  enabled: true,
  files: [
    {
      id: 'haze-1',
      remote_file_id: null,
      archive_name: 'midnight_bloom_3f2a1.zip',
      inner_path: 'pak01_dir.vpk',
      version: null,
      len: 1,
      damaged: false,
      newer_in_library: null,
    },
  ],
  variant_groups: [],
  add_ons: [],
  bytes: 1,
  pending: false,
  in_game: null,
};

function file(kind: FolderFile['kind'], from_slot: number | null, to_slot: number | null, over: Partial<FolderFile> = {}): FolderFile {
  return { kind, file: 'haze-1', mod_id: 'haze', name: 'Haze — Midnight Bloom', from_slot, to_slot, ...over };
}

describe('the file list of a pending apply (S9)', () => {
  it('groups the files and shows each one’s slot before and after', () => {
    const { target } = render(FolderFiles, {
      files: [
        file('added', null, 3),
        file('removed', 1, null, { file: 'gone-1', mod_id: null, name: null }),
        file('renumbered', 2, 1),
      ],
      mods: [HAZE],
    });
    const text = (el: Element) => el.textContent?.replace(/\s+/g, ' ').trim();
    expect(all(target, '.label').map(text)).toEqual(['Added · 1', 'Removed · 1', 'Renumbered · 1']);
    const rows = all(target, '.file').map(text);
    expect(rows[0]).toBe('pak03_dir.vpk Haze — Midnight Bloom · midnight_bloom');
    // A file whose mod left the library is named by its id.
    expect(rows[1]).toBe('pak01_dir.vpk gone-1');
    expect(rows[2]).toBe('pak02_dir.vpk → pak01_dir.vpk Haze — Midnight Bloom · midnight_bloom');
    expect(one(target, '[role=list]').getAttribute('aria-label')).toBe('Files in the game folder');
  });

  it('draws only the rows in view of a long list', () => {
    const files = Array.from({ length: 99 }, (_, i) => file('renumbered', i + 1, i + 2));
    const { target } = render(FolderFiles, { files, mods: [HAZE] });
    const viewport = one(target, '.viewport');
    Object.defineProperty(viewport, 'clientHeight', { value: 200, configurable: true });
    viewport.dispatchEvent(new Event('scroll'));
    flushSync();
    const drawn = all(target, '.file').length;
    expect(drawn).toBeGreaterThan(0);
    expect(drawn).toBeLessThan(30);
  });
});
