// The Downloads screen's preview data for the mock core: a queue with a task in every stage (board `Downloads`), and
// the offline, disk-full and variant-choice states. Invented names, like the rest of the mock.

import type { Changed, ModView, TaskSnapshot, TaskState, UiEvent, UserMessage } from '../lib/api';
import { pictureId } from './fixtures';

const MB = 1_000_000;

let nextId = 900;

function task(
  name: string,
  file: string,
  size: number,
  state: TaskState,
  extra: { bytes?: number; speed?: number; meta?: Record<string, string>; picture?: boolean } = {},
): TaskSnapshot {
  const id = (nextId += 1);
  const modId = 700_000 + (id % 100);
  // The mod's picture from its saved mod page; a download started without one shows the placeholder.
  const picture: Record<string, string> = extra.picture === false ? {} : { thumbnail: pictureId(modId, 'small') };
  return {
    id,
    url: `https://gamebanana.com/dl/${id}`,
    file_name: file,
    md5: null,
    size,
    initiator: 'user',
    meta: {
      section: 'mod',
      mod_id: String(modId),
      file_id: String(1_900_000 + id),
      mod_name: name,
      ...picture,
      ...extra.meta,
    },
    state,
    bytes: extra.bytes ?? 0,
    total: size,
    speed: extra.speed ?? 0,
  };
}

const E9: UserMessage = {
  id: 'E9',
  args: { reason: 'too_large', limit: String(2 * 1024 ** 3), found: String(3.8 * 1024 ** 3), detail: 'unpacked size over the limit' },
};
const E11: UserMessage = { id: 'E11', args: { detail: 'tar.exe: unsupported RAR version' } };

/** The board `Downloads`: every stage once, in queue order. */
export function queue(): TaskSnapshot[] {
  nextId = 900;
  return [
    task('Seven — Storm Crown', 'seven_storm_crown_v2.7z', 214.6 * MB, { state: 'downloading' }, {
      bytes: 66.4 * MB,
      speed: 3.1 * MB,
      meta: { replaces: '1880001', version: 'v2' },
    }),
    task('Haze — Midnight Bloom', 'haze_midnight_sky.rar', 3.1 * MB, { state: 'downloading' }, { bytes: 1.9 * MB, speed: 1.2 * MB }),
    task('Abrams — Brass Knight', 'abrams_brass_team.zip', 0.6 * MB, { state: 'verifying' }, { bytes: 0.6 * MB }),
    task('Wraith — Ashen Veil', 'wraith_ashen_veil.7z', 18.2 * MB, { state: 'installing' }, { bytes: 18.2 * MB }),
    task('Haze — Midnight Bloom', 'haze_midnight_grey.rar', 2.8 * MB, { state: 'paused' }, { bytes: 1.2 * MB }),
    task('Haze — Midnight Bloom', 'haze_midnight_cherry.rar', 2.7 * MB, { state: 'downloading' }, { bytes: 2.1 * MB, speed: 0.9 * MB }),
    task('Vindicta — Coral Reef', 'vindicta_coral_reef.zip', 38 * MB, { state: 'downloading' }, { bytes: 9.5 * MB, speed: 2.4 * MB }),
    task('Kelvin — Porcelain', 'kelvin_porcelain.zip', 24.3 * MB, { state: 'queued' }),
    task('Always Show Item Icons', 'always_show_item_icons.zip', 0.4 * MB, { state: 'queued' }, { picture: false }),
    task('Lady Geist — Ember Coat', 'geist_ember_coat.zip', 412 * MB, { state: 'failed', message: E9 }, { bytes: 412 * MB }),
    task('Dynamo — Neon Rider', 'dynamo_neon_rider.rar', 61.3 * MB, { state: 'failed', message: E11 }, { bytes: 61.3 * MB }),
  ];
}

/** Offline: what was downloading waits for the network (E6, resumed automatically). */
export function offline(tasks: TaskSnapshot[]): TaskSnapshot[] {
  return tasks.map((t) =>
    t.state.state === 'downloading'
      ? { ...t, speed: 0, state: { state: 'waiting', message: { id: 'E6', args: { reason: 'connect' } }, retry_in_s: null } }
      : t,
  );
}

/** Disk full: what was downloading or unpacking waits for free space (E12). */
export function diskFull(tasks: TaskSnapshot[]): TaskSnapshot[] {
  const full: UserMessage = {
    id: 'E12',
    args: { needed: String(236 * MB), available: String(96 * MB), drive: 'D:', reason: 'space' },
  };
  return tasks.map((t) =>
    t.state.state === 'downloading' || t.state.state === 'installing'
      ? { ...t, speed: 0, state: { state: 'waiting', message: full, retry_in_s: null } }
      : t,
  );
}

/** A library mod whose archive held four variants, none turned on yet. */
export function variantMod(): ModView {
  const id = 'gb-700042';
  const names = ['Tracksuit', 'Casual', 'Racing silks', 'Summer dress'];
  const files = names.map((name, n) => ({
    id: `${id}-1-${n}`,
    remote_file_id: 1_800_420,
    archive_name: 'pocket_four_outfits.zip',
    inner_path: `${name}/pak_pocket_${name.toLowerCase().replace(' ', '_')}.vpk`,
    version: '1.0',
    len: [12.4, 10.9, 13.1, 9.6][n] ?? 10 * MB,
    damaged: false,
    newer_in_library: null,
  }));
  for (const file of files) {
    file.len = Math.round(file.len * MB);
  }
  return {
    id,
    name: 'Pocket — Four Outfits',
    author: 'velvet.paw',
    source: { kind: 'gamebanana', section: 'mod', mod_id: 700_042 },
    enabled: false,
    files,
    variant_groups: [{ files: files.map((f) => f.id), active: null }],
    add_ons: [],
    bytes: files.reduce((sum, f) => sum + f.len, 0),
    pending: false,
    in_game: null,
  };
}

/**
 * The pushes that finished downloads make: a short history for "Finished this session" (installed, an update, a
 * cancel) and one download waiting for a variant pick (D4). `changed` is the library after an install.
 */
export function finishedEvents(choice: ModView, changed: Changed): UiEvent[] {
  const events: UiEvent[] = [];
  const installed = (t: TaskSnapshot, modId: string, choices: string[][]) => {
    events.push({ event: 'download', data: { event: 'added', task: t } });
    events.push({
      event: 'installed',
      data: {
        task: t.id,
        installed: {
          outcome: { mod_id: modId, new_files: choices.flat(), choices, skipped: [] },
          changed: choices.length === 0 ? changed : null,
        },
      },
    });
    events.push({ event: 'download', data: { event: 'removed', id: t.id } });
  };
  installed(task('Abrams — Coral Reef', 'abrams_coral_reef.zip', 22.7 * MB, { state: 'installing' }), 'gb-700004', []);
  const cancelled = task('Infernus — Porcelain', 'infernus_porcelain.zip', 0.9 * MB, { state: 'queued' });
  events.push({ event: 'download', data: { event: 'added', task: cancelled } });
  events.push({ event: 'download', data: { event: 'removed', id: cancelled.id } });
  const update = task('Item Lock — Quick Save', 'item_lock_v2.zip', 1.8 * MB, { state: 'done' }, {
    meta: { replaces: '1880002', version: 'v2' },
  });
  events.push({ event: 'download', data: { event: 'added', task: update } });
  events.push({ event: 'download', data: { event: 'removed', id: update.id } });
  installed(task('Seven — Ember Coat', 'seven_ember_coat.zip', 12.4 * MB, { state: 'installing' }), 'gb-700001', []);
  installed(
    task(choice.name, 'pocket_four_outfits.zip', 46.2 * MB, { state: 'installing' }, { bytes: 46.2 * MB }),
    choice.id,
    [choice.files.map((f) => f.id)],
  );
  return events;
}
