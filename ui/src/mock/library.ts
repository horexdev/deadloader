// The Library's part of the mock core: a library with conflicts, variants, add-ons and a local copied mod, pending
// changes while the game runs, foreign files, gameinfo.gi and mod updates. Invented data only: no GameBanana names,
// authors or pictures. Scenarios (`?scenario=library-…`) set up each state of the Library boards: `empty`, `local`,
// `game-missing`, `running` (four pending changes), `applied` (the game closes once the UI listens: N12 and S9's
// result), `applying` and `checking` (slow answers), `foreign`, `gameinfo-foreign` (D10), `updates` (D5 with a file
// mapping). The shared `switched` scenario shows the profile switch.

import type {
  Changed,
  ConflictView,
  FileView,
  ForeignReport,
  GameinfoStatus,
  LibraryView,
  ModView,
  Pending,
  PendingChange,
  RemovalPlan,
  UpdateReport,
  UserMessage,
} from '../lib/api';

/** What the library mock needs from the core mock. */
export interface Host {
  library: LibraryView;
  gameRunning: boolean;
  gameRoot: string | null;
  catalogEnabled: () => boolean;
  push: (event: import('../lib/api').UiEvent) => void;
}

type Args = Record<string, unknown>;

const MB = 1_048_576;

function vpkFile(id: string, archive: string, inner: string, version: string, len: number): FileView {
  return {
    id,
    remote_file_id: null,
    archive_name: archive,
    inner_path: inner,
    version,
    len,
    damaged: false,
    newer_in_library: null,
  };
}

function gb(n: number, name: string, author: string, version: string, mb: number, enabled = true): ModView {
  const id = `gb-${710_000 + n}`;
  const f = vpkFile(`${id}-1`, `${name.toLowerCase().replace(/\W+/g, '_')}.zip`, 'pak01_dir.vpk', version, mb * MB);
  f.remote_file_id = 1_900_000 + n;
  return {
    id,
    name,
    author,
    source: { kind: 'gamebanana', section: 'mod', mod_id: 710_000 + n },
    enabled,
    files: [f],
    variant_groups: [],
    add_ons: [[f.id, true]],
    bytes: f.len,
    pending: false,
    in_game: null,
  };
}

/** The library of the boards, with invented names. */
export function libraryMods(): ModView[] {
  const celeste = gb(3, 'Celeste — Coral Reef', 'velvet.paw', '1', 8);
  celeste.files = [
    vpkFile('gb-710003-pink', 'celeste_coral_pink_f5f81.rar', 'pak01_dir.vpk', '1', 8 * MB),
    vpkFile('gb-710003-crimson', 'celeste_coral_crimson_5027e.rar', 'pak01_dir.vpk', '1', 2.2 * MB),
  ];
  celeste.variant_groups = [{ files: ['gb-710003-pink', 'gb-710003-crimson'], active: 'gb-710003-pink' }];
  celeste.add_ons = [];
  celeste.bytes = 10.2 * MB;
  const mcginnis = gb(4, 'McGinnis — Brass Knight', 'orbit', '1', 4.1);
  const team = vpkFile('gb-710004-team', 'brass_knight_team.zip', 'pak02_dir.vpk', '1', 0.6 * MB);
  mcginnis.files.push(team);
  mcginnis.add_ons.push([team.id, true]);
  mcginnis.bytes += team.len;
  const local = gb(9, 'haze_nightshift', '', '1', 38, false);
  local.id = 'local-1';
  local.author = null;
  local.source = { kind: 'local', imported_from: String.raw`D:\Mods\haze_nightshift.vpk` };
  local.files = [vpkFile('local-1-1', 'haze_nightshift.vpk', 'haze_nightshift.vpk', '', 38 * MB)];
  local.add_ons = [['local-1-1', true]];
  local.in_game = null;
  return [
    gb(1, 'Haze — Midnight Bloom', 'lumen', '2', 120),
    gb(2, 'Yamato — Ember Coat', 'kestrel_mods', '1', 34.2),
    celeste,
    mcginnis,
    gb(5, 'Yamato — Porcelain', 'nightjar', '1', 61),
    gb(6, 'Ivy — Neon Rider', 'velvet.paw', '1', 22),
    gb(7, 'Abrams — Ashen Veil', 'lumen', '1', 48),
    gb(8, 'Wraith — Formal', 'orbit', '1', 19, false),
    local,
    gb(10, 'Lash — Velvet Dusk', 'kestrel_mods', '1', 27),
    gb(11, 'Seven — Storm Glass', 'nightjar', '2', 75),
  ];
}

export function libraryConflicts(): ConflictView[] {
  const files = ['body', 'head', 'sword', 'scabbard', 'hair', 'eyes', 'arms', 'legs', 'cloth'];
  return [
    {
      winner_mod: 'gb-710002',
      winner_file: 'gb-710002-1',
      loser_mod: 'gb-710005',
      loser_file: 'gb-710005-1',
      paths: [
        ...files.map((f) => `models/heroes/yamato/yamato_${f}.vmdl_c`),
        ...['body', 'head', 'sword', 'hair', 'cloth'].map((f) => `materials/models/heroes/yamato/${f}.vmat_c`),
      ],
    },
  ];
}

export function updateReport(changedFiles: boolean): UpdateReport {
  return {
    checks: [
      {
        mod_id: 'gb-710002',
        status: {
          kind: 'update_available',
          version: '2',
          date_updated: Date.now() - 3 * 86_400_000,
          replaced: [
            {
              remote_file_id: 1_900_002,
              archive_name: 'yamato_ember_coat.zip',
              files: ['gb-710002-1'],
              candidate: {
                file: {
                  id: 1_900_102,
                  file_name: changedFiles ? 'yamato_ember_coat_v2.zip' : 'yamato_ember_coat.zip',
                  size_bytes: 36.8 * MB,
                  date_added: Date.now() - 3 * 86_400_000,
                  version: '2',
                  description: null,
                },
                matched_by: 'same',
                in_library: [],
              },
            },
          ],
          kept: [],
          others: changedFiles
            ? [
                {
                  id: 1_900_103,
                  file_name: 'yamato_colors_alt.zip',
                  size_bytes: 3.1 * MB,
                  date_added: Date.now() - 3 * 86_400_000,
                  version: '2',
                  description: 'Alternative colours',
                },
              ]
            : [],
          posts: [],
        },
      },
      ...libraryMods()
        .filter((m) => m.source.kind === 'gamebanana' && m.id !== 'gb-710002')
        .map((m) => ({ mod_id: m.id, status: { kind: 'up_to_date' as const } })),
    ],
    available: 1,
    stopped: null,
  };
}

const FOREIGN: ForeignReport['items'] = [
  {
    name: '.dmm.json',
    kind: 'other_file',
    slot: null,
    bytes: 2_048,
    files: null,
    modified_unix_ms: Date.now() - 12 * 86_400_000,
    origin: 'deadlock_mod_manager',
    vpk: null,
    takeable: false,
    left: false,
  },
  {
    name: 'pak01_dir.vpk',
    kind: 'mod_file',
    slot: 1,
    bytes: 48 * MB,
    files: null,
    modified_unix_ms: Date.now() - 12 * 86_400_000,
    origin: null,
    vpk: { entries: 26, hero: 'haze' },
    takeable: true,
    left: false,
  },
  {
    name: 'replays',
    kind: 'folder',
    slot: null,
    bytes: 1.1 * MB,
    files: 3,
    modified_unix_ms: Date.now() - 6 * 86_400_000,
    origin: null,
    vpk: null,
    takeable: false,
    left: false,
  },
];

const GAMEINFO_FOREIGN: GameinfoStatus = {
  state: 'foreign',
  path: String.raw`D:\SteamLibrary\steamapps\common\Deadlock\game\citadel\gameinfo.gi`,
  diff: {
    expected: [
      { text: '\t\tSearchPaths', marked: false },
      { text: '\t\t{', marked: false },
      { text: '\t\t\tGame\t\t\t\tcitadel/addons', marked: true },
      { text: '\t\t\tMod\t\t\t\tcitadel', marked: false },
      { text: '\t\t\tWrite\t\t\t\tcitadel', marked: false },
      { text: '\t\t\tGame\t\t\t\tcitadel', marked: false },
      { text: '\t\t}', marked: false },
    ],
    found: [
      { text: '\t\tSearchPaths', marked: false },
      { text: '\t\t{', marked: false },
      { text: '\t\t\tGame\t\t\t\tcitadel/addons/dmm', marked: true },
      { text: '\t\t\tGame\t\t\t\tcitadel/addons', marked: true },
      { text: '\t\t\tMod\t\t\t\tcitadel', marked: false },
      { text: '\t\t\tWrite\t\t\t\tcitadel', marked: false },
      { text: '\t\t\tGame\t\t\t\tcitadel', marked: false },
      { text: '\t\t}', marked: false },
    ],
  },
};

/** The game folder as last applied: per mod whether it is in, its files there, in priority order. */
interface Snapshot {
  order: string[];
  files: Record<string, string[]>;
}

function activeFiles(m: ModView): string[] {
  return [
    ...m.variant_groups.map((g) => g.active ?? g.files[0] ?? '').filter(Boolean),
    ...m.add_ons.filter(([, on]) => on).map(([f]) => f),
  ];
}

function snapshotOf(mods: ModView[]): Snapshot {
  const on = mods.filter((m) => m.enabled);
  return { order: on.map((m) => m.id), files: Object.fromEntries(on.map((m) => [m.id, activeFiles(m)])) };
}

/** Indexes of one longest increasing run of `values` (the mods whose relative order is unchanged). */
function longestIncreasing(values: number[]): Set<number> {
  const len = values.map(() => 1);
  const prev: (number | null)[] = values.map(() => null);
  values.forEach((v, i) => {
    for (let j = 0; j < i; j += 1) {
      if ((values[j] ?? 0) < v && (len[j] ?? 0) + 1 > (len[i] ?? 0)) {
        len[i] = (len[j] ?? 0) + 1;
        prev[i] = j;
      }
    }
  });
  let at: number | null = len.length ? len.indexOf(Math.max(...len)) : null;
  const kept = new Set<number>();
  while (at !== null) {
    kept.add(at);
    at = prev[at] ?? null;
  }
  return kept;
}

/** Pending changes between the library and the snapshot, like `core::pending::changes`. */
export function changesOf(mods: ModView[], game: Snapshot): PendingChange[] {
  const out: PendingChange[] = [];
  const wanted = mods.filter((m) => m.enabled);
  for (const m of wanted) {
    const inGame = game.files[m.id];
    if (!inGame) {
      out.push({ mod_id: m.id, name: m.name, kind: 'enable' });
      continue;
    }
    const now = activeFiles(m);
    const from = inGame.filter((f) => !now.includes(f));
    const to = now.filter((f) => !inGame.includes(f));
    if (from.length > 0 || to.length > 0) {
      out.push({ mod_id: m.id, name: m.name, kind: 'variant', from, to });
    }
  }
  for (const id of game.order) {
    if (!wanted.some((m) => m.id === id)) {
      out.push({ mod_id: id, name: mods.find((m) => m.id === id)?.name ?? id, kind: 'disable' });
    }
  }
  const common = wanted.map((m) => m.id).filter((id) => game.order.includes(id));
  const kept = longestIncreasing(common.map((id) => game.order.indexOf(id)));
  common.forEach((id, i) => {
    if (!kept.has(i)) {
      out.push({
        mod_id: id,
        name: mods.find((m) => m.id === id)?.name ?? id,
        kind: 'order',
        above: common[i + 1] ?? null,
        below: common[i - 1] ?? null,
      });
    }
  });
  return out;
}

export class LibraryMock {
  game: Snapshot;
  updates: UpdateReport;
  foreign: ForeignReport['items'] = [];
  gameinfo: GameinfoStatus = { state: 'ours', path: null, diff: null };
  applying = false;
  slow = false;
  auto: { changes: PendingChange[]; at: number; restore: ModView[] } | null = null;
  undoing: number | null = null;
  /** The game closes when the UI subscribes (`library-applied`). */
  closeOnSubscribe = false;

  /** Pairs of mods that change the same files; who wins follows the order, like the core's overlap check. */
  private readonly overlaps: ConflictView[];

  constructor(private readonly host: Host) {
    this.overlaps = structuredClone(host.library.conflicts);
    this.game = snapshotOf(host.library.mods);
    this.updates = updateReport(false);
    this.refresh();
  }

  /** The UI subscribed: the update check at start reports, unless in local mode (157). */
  subscribed(): void {
    if (this.closeOnSubscribe) {
      // `library-applied`: the game closes once the UI listens, so N12 and S9's result show (143).
      this.closeOnSubscribe = false;
      this.host.gameRunning = false;
      this.host.push({ event: 'game', data: { running: false } });
      this.gameClosed();
    }
    if (this.host.catalogEnabled() && this.host.library.mods.length > 0) {
      this.host.push({ event: 'updates', data: structuredClone(this.updates) });
    }
  }

  /** Sets up a `library-…` scenario. */
  scenario(name: string): void {
    const lib = this.host.library;
    switch (name) {
      case 'library-empty':
      case 'library-local':
        lib.mods = [];
        lib.conflicts = [];
        this.updates = { checks: [], available: 0, stopped: null };
        break;
      case 'library-game-missing':
        this.host.gameRoot = null;
        this.gameinfo = { state: 'missing', path: null, diff: null };
        break;
      case 'library-applied':
      case 'library-running': {
        this.host.gameRunning = true;
        this.closeOnSubscribe = name === 'library-applied';
        const mods = lib.mods;
        const byId = (id: string) => mods.find((m) => m.id === id) as ModView;
        byId('gb-710008').enabled = true;
        byId('gb-710007').enabled = false;
        const celeste = byId('gb-710003');
        const group = celeste.variant_groups[0];
        if (group) {
          group.active = 'gb-710003-crimson';
        }
        const porcelain = byId('gb-710005');
        lib.mods = [mods[0] as ModView, porcelain, ...mods.slice(1).filter((m) => m !== porcelain)];
        break;
      }
      case 'library-applying':
      case 'library-checking':
        this.slow = true;
        break;
      case 'library-foreign':
        this.foreign = structuredClone(FOREIGN);
        break;
      case 'library-gameinfo-foreign':
        this.gameinfo = structuredClone(GAMEINFO_FOREIGN);
        break;
      case 'library-updates':
        this.updates = updateReport(true);
        break;
    }
    this.refresh();
  }

  /** The core's derived fields: pending flags and count, what is in the game. */
  refresh(): void {
    const lib = this.host.library;
    const at = (id: string) => lib.mods.findIndex((m) => m.id === id);
    const on = (id: string) => lib.mods.find((m) => m.id === id)?.enabled === true;
    lib.conflicts = this.overlaps
      .filter((c) => on(c.winner_mod) && on(c.loser_mod))
      .map((c) =>
        at(c.winner_mod) <= at(c.loser_mod)
          ? c
          : { winner_mod: c.loser_mod, winner_file: c.loser_file, loser_mod: c.winner_mod, loser_file: c.winner_file, paths: c.paths },
      );
    const changes = this.changes();
    const switched = lib.applied_profile !== lib.selected_profile;
    lib.pending = switched || changes.length > 0;
    lib.pending_count = switched ? 1 : changes.length;
    for (const m of lib.mods) {
      m.pending = !switched && changes.some((c) => c.mod_id === m.id);
      const files = this.game.files[m.id];
      m.in_game = files
        ? { copied: m.source.kind === 'local', bytes: m.files.filter((f) => files.includes(f.id)).reduce((s, f) => s + f.len, 0) }
        : null;
    }
  }

  changes(): PendingChange[] {
    return changesOf(this.host.library.mods, this.game);
  }

  /** An edit: into the game at once unless the game runs (138). */
  changed(): Changed {
    this.auto = null;
    this.undoing = null;
    const lib = this.host.library;
    const live = lib.applied_profile === lib.selected_profile;
    const waiting = live && this.host.gameRunning && this.changes().length > 0;
    if (live && !this.host.gameRunning) {
      this.game = snapshotOf(lib.mods);
    }
    this.refresh();
    return { library: structuredClone(lib), applied: live && !this.host.gameRunning, waiting_for_game: waiting };
  }

  /** The game was closed: what waited goes in, with Undo (143). */
  gameClosed(): void {
    const changes = this.changes();
    if (changes.length === 0) {
      return;
    }
    const restore = structuredClone(this.host.library.mods);
    // The restore point is the library as the game had it.
    for (const m of restore) {
      m.enabled = this.game.order.includes(m.id);
      for (const group of m.variant_groups) {
        group.active = group.files.find((f) => this.game.files[m.id]?.includes(f)) ?? group.active;
      }
    }
    restore.sort((a, b) => {
      const ia = this.game.order.indexOf(a.id);
      const ib = this.game.order.indexOf(b.id);
      return ia < 0 || ib < 0 ? 0 : ia - ib;
    });
    this.game = snapshotOf(this.host.library.mods);
    this.auto = { changes, at: Date.now(), restore };
    this.undoing = null;
    this.refresh();
    this.host.push({ event: 'applied_after_game', data: { changes: changes.length, items: changes, library: structuredClone(this.host.library) } });
  }

  private mod(id: unknown): ModView {
    const found = this.host.library.mods.find((m) => m.id === id);
    if (!found) {
      throw { id: 'Internal', args: { detail: `mock: no mod ${String(id)}` } } satisfies UserMessage;
    }
    return found;
  }

  private wait<T>(value: T): Promise<T> | T {
    return this.slow ? new Promise((resolve) => setTimeout(() => resolve(value), 8000)) : value;
  }

  handlers(): Record<string, (args: Args) => unknown> {
    const lib = () => this.host.library;
    return {
      library: () => {
        this.refresh();
        return structuredClone(lib());
      },
      set_enabled: (args) => {
        this.mod(args.modId).enabled = Boolean(args.enabled);
        return this.changed();
      },
      move_mod: (args) => {
        const m = this.mod(args.modId);
        lib().mods = lib().mods.filter((x) => x !== m);
        lib().mods.splice(Number(args.index), 0, m);
        return this.changed();
      },
      switch_variant: (args) => {
        const m = this.mod(args.modId);
        for (const group of m.variant_groups) {
          if (group.files.includes(String(args.file))) {
            group.active = String(args.file);
          }
        }
        m.add_ons = m.add_ons.map(([f, on]) => [f, f === args.file ? true : on]);
        return this.changed();
      },
      apply: () => {
        if (this.host.gameRunning) {
          throw { id: 'E13', args: {} } satisfies UserMessage;
        }
        lib().applied_profile = lib().selected_profile;
        this.game = snapshotOf(lib().mods);
        this.undoing = null;
        this.refresh();
        return this.wait({ library: structuredClone(lib()), changed: true, gameinfo_foreign: this.gameinfo.state === 'foreign' });
      },
      pending: (): Pending => {
        this.refresh();
        const l = lib();
        const switched = l.applied_profile !== l.selected_profile;
        const changes = switched ? [] : this.changes();
        const on = this.changes().filter((c) => c.kind === 'enable').length;
        const off = this.changes().filter((c) => c.kind === 'disable').length;
        return {
          switch: switched ? { from: l.applied_profile, to: l.selected_profile, turned_on: on, turned_off: off } : null,
          changes,
          folder: {
            added: on,
            removed: off,
            renumbered: changes.filter((c) => c.kind === 'order').length + (switched ? 4 : 0),
            addons: this.host.gameRoot ? `${this.host.gameRoot}\\game\\citadel\\addons` : null,
          },
          applied_after_game: this.auto ? { changes: this.auto.changes, at_unix_ms: this.auto.at } : null,
          undoing: this.undoing,
        };
      },
      pending_revert: (args) => {
        const change = this.changes().find((c) => c.mod_id === args.modId);
        const m = this.mod(args.modId);
        if (change?.kind === 'enable') {
          m.enabled = false;
        } else if (change?.kind === 'disable') {
          m.enabled = true;
        } else if (change?.kind === 'variant') {
          for (const group of m.variant_groups) {
            group.active = group.files.find((f) => this.game.files[m.id]?.includes(f)) ?? group.active;
          }
        } else if (change?.kind === 'order') {
          const target = this.game.order.indexOf(m.id);
          const after = this.game.order[target + 1];
          lib().mods = lib().mods.filter((x) => x !== m);
          const at = after ? lib().mods.findIndex((x) => x.id === after) : lib().mods.length;
          lib().mods.splice(at < 0 ? lib().mods.length : at, 0, m);
        }
        const undoing = this.undoing;
        const result = this.changed();
        this.undoing = undoing;
        return result;
      },
      pending_discard: () => {
        const l = lib();
        if (l.applied_profile !== l.selected_profile && l.applied_profile) {
          l.selected_profile = l.applied_profile;
          return this.changed();
        }
        for (const m of l.mods) {
          m.enabled = this.game.order.includes(m.id);
          for (const group of m.variant_groups) {
            group.active = group.files.find((f) => this.game.files[m.id]?.includes(f)) ?? group.active;
          }
        }
        l.mods.sort((a, b) => {
          const ia = this.game.order.indexOf(a.id);
          const ib = this.game.order.indexOf(b.id);
          return ia < 0 || ib < 0 ? 0 : ia - ib;
        });
        return this.changed();
      },
      pending_undo_applied: () => {
        const auto = this.auto;
        if (!auto) {
          throw { id: 'Internal', args: { detail: 'nothing to undo' } } satisfies UserMessage;
        }
        lib().mods = auto.restore;
        const result = this.changed();
        if (result.waiting_for_game) {
          this.undoing = auto.at;
        }
        return result;
      },
      foreign_files: (): ForeignReport => ({ root: this.host.gameRoot, items: structuredClone(this.foreign) }),
      foreign_leave: (args): ForeignReport => {
        this.foreign = this.foreign.map((f) => (f.name === args.name ? { ...f, left: true } : f));
        return { root: this.host.gameRoot, items: structuredClone(this.foreign) };
      },
      foreign_take: (args) => {
        if (this.host.gameRunning) {
          throw { id: 'E13', args: {} } satisfies UserMessage;
        }
        const item = this.foreign.find((f) => f.name === args.name);
        this.foreign = this.foreign.filter((f) => f.name !== args.name);
        const m = gb(90, String(args.modName || args.name), '', '', (item?.bytes ?? 0) / MB);
        m.id = `local-${lib().mods.length + 2}`;
        m.author = null;
        m.source = { kind: 'local', imported_from: `${this.host.gameRoot ?? ''}\\game\\citadel\\addons\\${String(args.name)}` };
        m.files[0] = vpkFile(`${m.id}-1`, String(args.name), String(args.name), '', item?.bytes ?? 0);
        m.add_ons = [[`${m.id}-1`, true]];
        lib().mods.push(m);
        return { mod_id: m.id, changed: this.changed() };
      },
      gameinfo_status: (args): GameinfoStatus => {
        if (args.fix && this.gameinfo.state !== 'ours' && this.gameinfo.state !== 'missing' && this.gameinfoRestored) {
          this.gameinfo = { state: 'ours', path: this.gameinfo.path, diff: null };
        }
        return structuredClone(this.gameinfo);
      },
      verify_game_files: () => {
        // Steam restores the original file; "Check again" then finds it.
        this.gameinfoRestored = true;
        return null;
      },
      show_place: () => null,
      removal_plan: (args): RemovalPlan => {
        const m = this.mod(args.modId);
        return {
          mod_id: m.id,
          files: m.files.map((f) => f.id),
          bytes: m.bytes,
          removes_mod: true,
          profiles: m.enabled ? [lib().selected_profile] : [],
          applied_in: [],
        };
      },
      remove: (args) => {
        const m = this.mod(args.modId);
        lib().mods = lib().mods.filter((x) => x !== m);
        lib().conflicts = lib().conflicts.filter((c) => c.winner_mod !== m.id && c.loser_mod !== m.id);
        this.changed();
        return { files: m.files.map((f) => f.id), leftovers: [] };
      },
      mod_rename: (args) => {
        this.mod(args.modId).name = String(args.name);
        return structuredClone(lib());
      },
      mod_link: (args) => {
        const url = String(args.url);
        if (!/gamebanana\.com\/(mods|sounds)\/\d+/.test(url)) {
          throw { kind: 'not_a_page' };
        }
        const m = this.mod(args.modId);
        m.source = { kind: 'gamebanana', section: 'mod', mod_id: Number(/\d+/.exec(url.split('/').pop() ?? '')?.[0] ?? 0) };
        return {
          mod_id: m.id,
          archives: m.files.map((f) => ({ archive_name: f.archive_name, files: [f.id], remote_file_id: 1_950_001, matched_by: 'md5' })),
        };
      },
      mod_link_files: () => null,
      updates_check: () => {
        if (!this.host.catalogEnabled()) {
          throw { kind: 'local_mode' };
        }
        return this.wait(structuredClone(this.updates));
      },
      updates_dismiss: () => null,
      update_install: (args) => {
        const changed = this.updates.checks.some((c) => c.mod_id === args.modId && c.status.kind === 'update_available' && c.status.others.length > 0);
        const m = this.mod(args.modId);
        const v2 = vpkFile(`${m.id}-2`, 'yamato_ember_coat_v2.zip', 'pak01_dir.vpk', '2', 36.8 * MB);
        m.files.push(v2);
        if (changed) {
          return {
            tasks: [],
            steps: [
              {
                kind: 'needs_mapping',
                mod_id: m.id,
                replaced: 1_900_002,
                new_files: [v2.id],
                proposals: [{ old: `${m.id}-1`, proposed: { kind: 'tie', candidates: [v2.id], shared_paths: 9, identical: 2 } }],
                version: '2',
              },
            ],
          };
        }
        return this.finishUpdate(m, v2);
      },
      update_map: (args) => {
        const m = this.mod(args.modId);
        const v2 = m.files.find((f) => f.version === '2') as FileView;
        return this.finishUpdate(m, v2).steps[0]?.changed;
      },
    };
  }

  gameinfoRestored = false;

  private finishUpdate(m: ModView, v2: FileView) {
    m.add_ons = m.add_ons.map(([f, on]) => [f === `${m.id}-1` ? v2.id : f, on]);
    const old = m.files.find((f) => f.id === `${m.id}-1`);
    if (old) {
      old.newer_in_library = v2.id;
    }
    this.updates = {
      ...this.updates,
      available: 0,
      checks: this.updates.checks.map((c) => (c.mod_id === m.id ? { mod_id: m.id, status: { kind: 'up_to_date' } } : c)),
    };
    const changed = this.changed();
    this.host.push({ event: 'updates', data: structuredClone(this.updates) });
    return { tasks: [], steps: [{ kind: 'applied' as const, mod_id: m.id, changed }] };
  }
}
