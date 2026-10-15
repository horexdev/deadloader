// Invented data for the mock core: no GameBanana names, authors or pictures. Shapes follow `lib/api.ts`.

import type { Card, CatalogFile, Description, ModPage, ModView, Settings } from '../lib/api';

/** An entry of a non-empty list, cycling. */
function pick<T>(list: readonly [T, ...T[]], i: number): T {
  return list[i % list.length] ?? list[0];
}

const HEROES: [string, ...string[]] = ['Haze', 'Infernus', 'Seven', 'Vindicta', 'Lady Geist', 'Abrams', 'Wraith', 'McGinnis', 'Dynamo', 'Kelvin'];
const STYLES: [string, ...string[]] = ['Midnight Bloom', 'Ember Coat', 'Porcelain', 'Neon Rider', 'Ashen Veil', 'Coral Reef', 'Brass Knight'];
const AUTHORS: [string, ...string[]] = ['lumen', 'kestrel_mods', 'velvet.paw', 'nightjar', 'orbit'];

/** A stable picture id for a mod and a size, 32 hex digits like the core's. */
export function pictureId(seed: number, size: 'small' | 'large' | 'gallery' | 'original'): string {
  const tag = { small: 1, large: 2, gallery: 3, original: 4 }[size];
  return ((seed * 2654435761 + tag * 97) >>> 0).toString(16).padStart(8, '0').repeat(4);
}

export function card(i: number): Card {
  const hero = pick(HEROES, i);
  return {
    id: 700_000 + i,
    section: 'mod',
    name: `${hero} — ${pick(STYLES, i)}`,
    author: pick(AUTHORS, i),
    root_category: { id: 1, name: 'Skins' },
    subcategory: { id: 100 + (i % HEROES.length), name: hero },
    likes: (i * 37) % 500,
    views: (i * 1301) % 90_000,
    date_added: 1_791_000_000 - i * 86_400,
    date_updated: i % 3 === 0 ? 1_792_000_000 - i * 3_600 : null,
    thumbnail: { small: pictureId(i, 'small'), large: pictureId(i, 'large') },
    has_audio_preview: false,
    nsfw: i % 11 === 7,
    obsolete: false,
    featured: i % 9 === 0,
    has_files: true,
  };
}

function file(i: number, n: number): CatalogFile {
  return {
    id: 1_800_000 + i * 10 + n,
    file_name: `${pick(HEROES, i).toLowerCase().replace(' ', '_')}_${pick(['pink', 'blue', 'gold'], n)}.zip`,
    size_bytes: 3_200_000 + n * 750_000,
    date_added: 1_791_500_000 + n * 60,
    download_count: 1200 - n * 300,
    description: n === 0 ? 'Default colours' : null,
    version: '1.2',
    md5: null,
    antivirus: 'clean',
  };
}

const EMPTY_DESCRIPTION: Description = { html: '', links: [], images: [], consent: [], truncated: false };

export function modPage(i: number): ModPage {
  const c = card(i);
  const hero = pick(HEROES, i);
  return {
    id: c.id,
    section: 'mod',
    name: c.name,
    version: '1.2',
    author: c.author,
    credits: [],
    category_path: [
      { id: 1, name: 'Skins' },
      { id: 100 + (i % HEROES.length), name: hero },
    ],
    gallery: [0, 1, 2].map((g) => ({
      preview: pictureId(i * 10 + g, 'gallery'),
      original: pictureId(i * 10 + g, 'original'),
      caption: g === 0 ? 'In game' : null,
    })),
    has_audio_preview: false,
    short_description: 'A recolour with new trims and glow.',
    description: {
      ...EMPTY_DESCRIPTION,
      html: '<p>Replaces the default skin. Pick one colour, or install all three and switch in the library.</p>',
    },
    licence: null,
    likes: c.likes,
    views: c.views,
    downloads: 4_210,
    date_added: c.date_added,
    date_modified: c.date_updated,
    date_updated: c.date_updated,
    nsfw: c.nsfw,
    nsfw_hidden: false,
    content_ratings: [],
    obsolete: false,
    embedded_media: [],
    alternate_sources: [],
    files: [0, 1, 2].map((n) => file(i, n)),
    archived_files: [],
    origin: { kind: 'network' },
  };
}

/** A library mod of `files` variants; local mods have no GameBanana source. */
export function libraryMod(i: number, enabled: boolean, local = false): ModView {
  const id = local ? `local-${i}` : `gb-${700_000 + i}`;
  const files = [0, 1].map((n) => ({
    id: `${id}-1-${n}`,
    archive_name: `${pick(HEROES, i).toLowerCase().replace(' ', '_')}.zip`,
    inner_path: `${pick(['pink', 'blue'], n)}/pak01_dir.vpk`,
    version: '1.2',
    len: 4_000_000 + n * 1_000_000,
    damaged: false,
    newer_in_library: null,
  }));
  return {
    id,
    name: local ? `my ${pick(HEROES, i).toLowerCase()} edit` : card(i).name,
    author: local ? null : card(i).author,
    source: local ? { kind: 'local', imported_from: 'D:\\Mods\\edit.vpk' } : { kind: 'gamebanana', section: 'mod', mod_id: 700_000 + i },
    enabled,
    files,
    variant_groups: [{ files: files.map((f) => f.id), active: files[0]?.id ?? null }],
    add_ons: [],
    bytes: files.reduce((sum, f) => sum + f.len, 0),
    pending: false,
    in_game: enabled ? { copied: false, bytes: files[0]?.len ?? 0 } : null,
  };
}

/** What a first start writes (core `Settings::default`), with the catalog on. */
export function settings(): Settings {
  return {
    general: { language: 'system', startup_profile: 'last_selected', check_app_updates: false, zoom_percent: 100 },
    game: { auto_fix_gameinfo: true, manual_installations: [] },
    catalog: {
      enabled: true,
      nsfw: 'hidden',
      description_images: 'on_click',
      always_allow_host_prechecked: true,
      allowed_image_hosts: [],
      gif_autoplay: false,
      external_links: 'ask',
      page_ttl_minutes: 10,
      prefetch_next_page: true,
    },
    library: {
      check_updates_at_start: true,
      download_updates_automatically: false,
      deletion: 'permanent',
      old_versions: 'keep_all',
    },
    downloads: {
      parallel: 4,
      speed_limit_kib_per_s: null,
      keep_archives: false,
      max_extracted_mib: 2048,
      max_extracted_factor: 20,
      max_vpk_files: 256,
      max_entry_ratio: 100,
    },
    network: { parallel_api_requests: 2, connect_timeout_s: 10, read_timeout_s: 30, retries: 5, proxy: { mode: 'none' } },
    cache: { thumbnail_limit_mib: 100 },
    logs: { retention_days: 30, size_limit_mib: 100, mask_user_path: true },
    advanced: { extra_allowed_domains: [] },
    ui: { sidebar_collapsed: false, last_selected_profile: null },
    acknowledgements: { notice: null, age_confirmed_at: null },
  };
}
