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
export function pictureId(
  seed: number,
  size: 'small' | 'large' | 'gallery' | 'original' | 'gallery-large' | 'gallery-full',
): string {
  const tag = { small: 1, large: 2, gallery: 3, original: 4, 'gallery-large': 5, 'gallery-full': 6 }[size];
  return ((seed * 2654435761 + tag * 97) >>> 0).toString(16).padStart(8, '0').repeat(4);
}

/** The Mods section's root categories (GameBanana's, numbered by us); Skins holds one subcategory per hero. */
export const MOD_ROOTS = [
  { id: 1, name: 'Skins' },
  { id: 2, name: 'HUD' },
  { id: 3, name: 'Model Replacement' },
  { id: 4, name: 'Quality of Life/Fixes' },
  { id: 5, name: 'Other/Misc' },
  { id: 6, name: 'Maps' },
  { id: 7, name: 'Gameplay Modifications' },
  { id: 8, name: 'Animations' },
] as const;

export const SOUND_ROOTS = [
  { id: 50, name: 'Hero voices' },
  { id: 51, name: 'Weapons' },
  { id: 52, name: 'Music' },
] as const;

export const HERO_CATEGORIES = HEROES.map((name, h) => ({ id: 100 + h, name }));

const HUD_NAMES: [string, ...string[]] = ['Compact Top Bar', 'Readable Health Numbers', 'Quiet Kill Feed', 'Minimal Shop'];
const SOUND_NAMES: [string, ...string[]] = ['Soft Footsteps', 'Retro Hit Marker', 'Lo-fi Menu Music', 'Crisp Reload'];

export function card(i: number): Card {
  const hero = pick(HEROES, i);
  const hud = i % 4 === 1;
  return {
    id: 700_000 + i,
    section: 'mod',
    name: hud ? `${pick(HUD_NAMES, Math.floor(i / 4))} ${1 + (i % 3)}` : `${hero} — ${pick(STYLES, i)}`,
    author: pick(AUTHORS, i),
    root_category: hud ? { id: 2, name: 'HUD' } : { id: 1, name: 'Skins' },
    subcategory: hud ? null : { id: 100 + (i % HEROES.length), name: hero },
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

export function soundCard(i: number): Card {
  const root = SOUND_ROOTS[i % SOUND_ROOTS.length] ?? SOUND_ROOTS[0];
  return {
    ...card(i),
    id: 800_000 + i,
    section: 'sound',
    name: `${pick(SOUND_NAMES, i)} ${1 + (i % 5)}`,
    root_category: { id: root.id, name: root.name },
    subcategory: null,
    has_audio_preview: i % 2 === 0,
    nsfw: false,
    thumbnail: { small: pictureId(500 + i, 'small'), large: pictureId(500 + i, 'large') },
  };
}

function file(i: number, n: number): CatalogFile {
  return {
    id: 1_800_000 + i * 10 + n,
    file_name: `${pick(HEROES, i).toLowerCase().replace(' ', '_')}_${pick(['pink', 'blue', 'gold', 'crimson'], n)}.zip`,
    size_bytes: 3_200_000 + n * 750_000,
    date_added: 1_791_500_000 + n * 60,
    download_count: 1200 - n * 300,
    description: pick(['Default colours', 'Blue trims', 'Gold trims', 'Crimson'], n),
    version: '1.2',
    md5: null,
    antivirus: n === 3 ? { unfinished: 'pending' } : 'clean',
  };
}

const EMPTY_DESCRIPTION: Description = { html: '', links: [], images: [], consent: [], truncated: false };

/** A description using every element the sanitiser writes (S3), with its links and pictures. */
function description(i: number): Description {
  const consentImage = pictureId(i * 10 + 7, 'original');
  const clickImage = pictureId(i * 10 + 8, 'original');
  return {
    html:
      '<p>Replaces the default skin with new trims and a soft glow.<br>Pick one colour, or install them all and switch in the library.</p>' +
      '<h2>Colours</h2>' +
      '<ul><li><strong>Pink</strong> — pairs well with <a data-dl-link="0">this HUD</a></li><li><em>Blue</em> trims</li><li><u>Gold</u>, <s>silver</s> (removed)</li></ul>' +
      '<p><span class="dl-green">Compatible with recolours!</span> <span class="dl-red">Not compatible with older versions.</span></p>' +
      '<span class="dl-image" data-dl-image="0"></span>' +
      '<div class="dl-spoiler"><p>The glow is brighter at night.</p></div>' +
      '<span class="dl-youtube" data-dl-link="1"></span>' +
      '<p>Install with <code>deadloader</code> or by hand:</p><pre>citadel/addons/pak01_dir.vpk</pre>' +
      '<hr><table><thead><tr><th>Part</th><th>Size</th></tr></thead><tbody><tr><td>Body</td><td>3 MB</td></tr></tbody></table>' +
      '<span class="dl-image" data-dl-image="1"></span>' +
      '<ol><li>Download</li><li>Play</li></ol>' +
      '<p>Questions? See <a data-dl-link="2">the author page</a>.</p>' +
      '<p>' +
      'More notes about the colours, the glow and how the trims were painted. '.repeat(40) +
      '</p>',
    links: [
      { url: 'https://gamebanana.com/mods/700002', host: 'gamebanana.com', internal_mod_id: 700_002 },
      { url: 'https://www.youtube.com/watch?v=aqz-KE-bpKQ', host: 'www.youtube.com', internal_mod_id: null },
      { url: 'https://example.org/authors/lumen', host: 'example.org', internal_mod_id: null },
    ],
    images: [
      { id: consentImage, host: 'i.imgur.com', alt: 'The skin at night', access: 'consent' },
      { id: clickImage, host: 'images.gamebanana.com', alt: null, access: 'click' },
    ],
    consent: [{ host: 'i.imgur.com', images: [consentImage] }],
    truncated: false,
  };
}

export function modPage(i: number): ModPage {
  const c = card(i);
  return {
    id: c.id,
    section: 'mod',
    name: c.name,
    version: '1.2',
    author: c.author,
    credits: [
      { name: 'Key Authors', authors: [{ name: c.author ?? 'lumen', role: null }] },
      { name: 'Special thanks', authors: [{ name: 'orbit', role: 'Testing' }] },
    ],
    category_path: [c.root_category, c.subcategory].filter((x): x is NonNullable<typeof x> => x !== null),
    gallery: [0, 1, 2].map((g) => ({
      preview: pictureId(i * 10 + g, 'gallery'),
      large: pictureId(i * 10 + g, 'gallery-large'),
      full: pictureId(i * 10 + g, 'gallery-full'),
      original: pictureId(i * 10 + g, 'original'),
      caption: g === 0 ? 'In game' : null,
    })),
    has_audio_preview: false,
    short_description: 'A recolour with new trims and glow.',
    description: i % 5 === 4 ? { ...EMPTY_DESCRIPTION, html: '<p>Just the skin.</p>' } : description(i),
    licence:
      i % 2 === 0 ? { ...EMPTY_DESCRIPTION, html: '<p>Free to use in your own recolours. Credit the author.</p>' } : null,
    likes: c.likes,
    views: c.views,
    downloads: 4_210,
    date_added: c.date_added,
    date_modified: c.date_updated,
    date_updated: c.date_updated,
    nsfw: c.nsfw,
    nsfw_hidden: false,
    content_ratings: c.nsfw ? ['skimpy_attire', 'sexual_themes'] : [],
    obsolete: false,
    embedded_media: i % 2 === 0 ? ['https://www.youtube.com/watch?v=aqz-KE-bpKQ'] : [],
    alternate_sources: i % 3 === 0 ? [{ url: 'https://example.org/mirror/skin', description: 'Mirror' }] : [],
    files: [0, 1, 2, 3].map((n) => file(i, n)),
    archived_files: i % 2 === 0 ? [{ ...file(i, 9), file_name: 'old_release.zip', version: '1.0' }] : [],
    origin: { kind: 'network' },
  };
}

export function soundPage(i: number): ModPage {
  const c = soundCard(i);
  return {
    ...modPage(i),
    id: c.id,
    section: 'sound',
    name: c.name,
    category_path: c.root_category ? [c.root_category] : [],
    has_audio_preview: c.has_audio_preview,
    nsfw: false,
    content_ratings: [],
  };
}

/** A library mod of `files` variants; local mods have no GameBanana source. */
export function libraryMod(i: number, enabled: boolean, local = false): ModView {
  const id = local ? `local-${i}` : `gb-${700_000 + i}`;
  const files = [0, 1].map((n) => ({
    id: `${id}-1-${n}`,
    remote_file_id: local ? null : 1_800_000 + i * 10 + n,
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
