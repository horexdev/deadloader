// What every screen shares: the library, the settings, the game, the network, running downloads and toasts. Filled
// at start and kept current by the core's pushes (`subscribe`); screens read it and call `api` for their own data.

import type { ErrorAction } from './components/ErrorDialog.svelte';
import type { ToastData } from './components/ToastStack.svelte';
import { describe } from '../i18n';
import {
  api,
  catalog,
  downloads,
  isUserMessage,
  shell,
  subscribe,
  type ApplyPreview,
  type CatalogEvent,
  type CatalogStatus,
  type GameStatus,
  type Installed,
  type LibraryView,
  type Section,
  type Settings,
  type TaskSnapshot,
  type UiEvent,
  type UpdateReport,
  type UserMessage,
} from './api';

/** D13: what Play would change, and the profiles involved. */
export interface PlayConfirmation extends ApplyPreview {
  profile: string;
  applied: string;
}

/** A download (or an import from disk) finished in the library and waits for the user to pick variants (D4). */
export interface PendingChoice {
  key: string;
  modId: string;
  /** The variant groups (library file ids), one pick each. */
  choices: string[][];
  name: string;
  file: string;
  size: number | null;
  /** The GameBanana file it came from, for its description in D4. */
  source?: { section: Section; modId: number; fileId: number };
}

/** A line of "Finished this session" on the Downloads screen (S6); kept until deadloader closes. */
export interface FinishedDownload {
  key: string;
  name: string;
  file: string;
  /** `kept`: a choice put aside, so the mod is in the library but not turned on. */
  outcome: 'installed' | 'updated' | 'cancelled' | 'removed' | 'kept';
  variant?: string;
  version?: string;
  /** Unix ms. */
  at: number;
  /** What "Download again" needs. */
  again?: { section: Section; modId: number; fileId: number; author: string | null };
}

/** D12: an error the user asked to see in full, with the actions that fit it. */
export interface ShownError {
  message: UserMessage;
  actions: ErrorAction[];
}

export class AppState {
  library: LibraryView | null = $state(null);
  settings: Settings | null = $state(null);
  game: GameStatus | null = $state(null);
  network: CatalogStatus = $state({ kind: 'local_mode' });
  tasks: TaskSnapshot[] = $state([]);
  /** Play is applying the profile. */
  playing = $state(false);
  confirm: PlayConfirmation | null = $state(null);
  toasts: ToastData[] = $state([]);
  /** D12, shown by the shell. */
  error: ShownError | null = $state(null);
  /** Downloads finished this session, newest first. */
  history: FinishedDownload[] = $state([]);
  /** Downloads waiting for a variant pick (D4). */
  choices: PendingChoice[] = $state([]);
  /** Tasks whose install was reported, so leaving the queue isn't a cancel. */
  // Bookkeeping only, never rendered: no reactivity needed.
  // eslint-disable-next-line svelte/prefer-svelte-reactivity
  #installed = new Set<number>();
  /** The catalog's latest change other than its status (a page or mod page refreshed in the background). */
  catalogEvent: Exclude<CatalogEvent, { event: 'status' }> | null = $state(null);
  /** The latest mod update check (N3): the catalog's badges and the mod page's Update buttons read it. */
  updates: UpdateReport | null = $state(null);
  #toastId = 0;

  /** Running downloads, for the sidebar. */
  get downloads(): number {
    return this.tasks.length;
  }

  async start(): Promise<void> {
    await subscribe((event) => this.handle(event));
    const [library, settings, game, tasks] = await Promise.all([
      api.library().catch(() => null),
      api.settings().catch(() => null),
      shell.gameStatus().catch(() => null),
      downloads.list().catch(() => []),
    ]);
    this.library = library;
    this.settings = settings;
    this.game = game;
    this.tasks = tasks;
    // The catalog opens in the background; its status arrives here or as a push.
    void catalog
      .status()
      .then((status) => {
        this.network = status;
      })
      .catch(() => undefined);
  }

  handle(event: UiEvent): void {
    switch (event.event) {
      case 'game':
        if (this.game) {
          this.game = { ...this.game, running: event.data.running };
        }
        break;
      case 'applied_after_game':
        this.library = event.data.library;
        this.toast({ kind: 'success', title: describe({ id: 'N12', args: { count: Math.max(event.data.changes, 1) } }).what });
        break;
      case 'apply_after_game_failed':
        this.fail(event.data.message);
        break;
      case 'catalog':
        if (event.data.event === 'status') {
          this.network = event.data.data;
        } else {
          this.catalogEvent = event.data;
        }
        break;
      case 'updates':
        this.updates = event.data;
        break;
      case 'download':
        this.followDownload(event.data);
        break;
      case 'installed':
        this.followInstalled(event.data.task, event.data.installed);
        if (event.data.installed.changed) {
          this.library = event.data.installed.changed.library;
        } else {
          void this.reloadLibrary();
        }
        break;
      case 'dropped':
        void this.reloadLibrary();
        break;
      case 'drop_failed':
        this.fail(event.data.message);
        break;
      default:
        break;
    }
  }

  private followDownload(event: Extract<UiEvent, { event: 'download' }>['data']): void {
    switch (event.event) {
      case 'added':
        this.tasks = [...this.tasks.filter((t) => t.id !== event.task.id), event.task];
        break;
      case 'state':
        this.tasks = this.tasks.map((t) => (t.id === event.id ? { ...t, state: event.state } : t));
        break;
      case 'progress':
        this.tasks = this.tasks.map((t) =>
          t.id === event.id ? { ...t, bytes: event.bytes, total: event.total, speed: event.speed } : t,
        );
        break;
      case 'removed':
        this.followRemoved(event.id);
        this.tasks = this.tasks.filter((t) => t.id !== event.id);
        break;
    }
  }

  private followInstalled(id: number, installed: Installed): void {
    const task = this.tasks.find((t) => t.id === id);
    this.#installed.add(id);
    if (!task) {
      return;
    }
    if (installed.outcome.choices.length > 0) {
      const source = againOf(task);
      this.choices = [
        ...this.choices,
        {
          key: `task-${id}`,
          modId: installed.outcome.mod_id,
          choices: installed.outcome.choices,
          name: nameOf(task),
          file: task.file_name,
          size: task.total ?? task.size,
          ...(source ? { source: { section: source.section, modId: source.modId, fileId: source.fileId } } : {}),
        },
      ];
      return;
    }
    this.remember(finished(task, 'installed'));
  }

  private followRemoved(id: number): void {
    const task = this.tasks.find((t) => t.id === id);
    if (!task || this.#installed.delete(id)) {
      return;
    }
    const outcome = task.state.state === 'done' ? 'updated' : task.state.state === 'failed' ? 'removed' : 'cancelled';
    this.remember(finished(task, outcome));
  }

  /** Adds a line to "Finished this session". */
  remember(entry: FinishedDownload): void {
    this.history = [entry, ...this.history];
  }

  clearHistory(): void {
    this.history = [];
  }

  /** A choice that didn't come from a download (an import from disk with variants). */
  addChoice(choice: PendingChoice): void {
    this.choices = [...this.choices.filter((c) => c.key !== choice.key), choice];
  }

  /** D4: turns the picked variants on (136: at the top of the selected profile). False when it failed (a toast says why). */
  async chooseVariant(key: string, files: string[], variant: string): Promise<boolean> {
    const choice = this.choices.find((c) => c.key === key);
    if (!choice) {
      return false;
    }
    try {
      const changed = await api.choose(choice.modId, files);
      this.library = changed.library;
    } catch (error) {
      this.fail(error);
      return false;
    }
    this.choices = this.choices.filter((c) => c.key !== key);
    this.remember({ key, name: choice.name, file: choice.file, outcome: 'installed', variant, at: Date.now() });
    return true;
  }

  /** Cancel on a choice: the mod stays in the library, not turned on. */
  putChoiceAside(key: string): void {
    const choice = this.choices.find((c) => c.key === key);
    if (!choice) {
      return;
    }
    this.choices = this.choices.filter((c) => c.key !== key);
    this.remember({ key, name: choice.name, file: choice.file, outcome: 'kept', at: Date.now() });
  }

  async reloadLibrary(): Promise<void> {
    this.library = await api.library().catch(() => this.library);
  }

  /** Play: D13 first when a profile switch would take mods out of the game (146), otherwise at once. */
  async requestPlay(): Promise<void> {
    if (this.playing || this.game?.running) {
      return;
    }
    const library = this.library;
    const switched = library !== null && library.applied_profile !== library.selected_profile;
    if (switched) {
      try {
        const preview = await shell.applyPreview();
        if (preview.turned_off.length > 0) {
          this.confirm = { ...preview, profile: library.selected_profile, applied: library.applied_profile ?? '' };
          return;
        }
      } catch (error) {
        this.fail(error);
        return;
      }
    }
    await this.play();
  }

  async confirmPlay(): Promise<void> {
    this.confirm = null;
    await this.play();
  }

  async play(): Promise<void> {
    this.playing = true;
    try {
      const result = await shell.play();
      this.library = result.library;
      this.game = this.game ? { ...this.game, running: true } : this.game;
    } catch (error) {
      this.fail(error);
    } finally {
      this.playing = false;
    }
  }

  toast(toast: Omit<ToastData, 'id'>): void {
    this.#toastId += 1;
    this.toasts = [...this.toasts, { ...toast, id: String(this.#toastId) }];
  }

  dismiss(id: string): void {
    this.toasts = this.toasts.filter((t) => t.id !== id);
  }

  /** A core message as an error toast with what happened and what to do. */
  fail(error: unknown): void {
    const message: UserMessage = isUserMessage(error) ? error : { id: 'Internal', args: { detail: String(error) } };
    const text = describe(message);
    this.toast({ kind: 'error', title: text.what, text: [text.why, text.todo].filter(Boolean).join(' ') });
  }

  /** D12: the whole story of an error (what, why, what to do, Details) with its actions. Open it on the user's
   *  request, e.g. "Show details", never by itself while the user browses (155). */
  showError(error: unknown, actions: ErrorAction[] = []): void {
    const message: UserMessage = isUserMessage(error) ? error : { id: 'Internal', args: { detail: String(error) } };
    this.error = { message, actions };
  }

  closeError(): void {
    this.error = null;
  }
}

function nameOf(task: TaskSnapshot): string {
  return task.meta.mod_name || task.file_name;
}

/** The GameBanana file a task downloads, for "Download again". */
function againOf(task: TaskSnapshot): FinishedDownload['again'] {
  const section = task.meta.section;
  const modId = Number(task.meta.mod_id);
  const fileId = Number(task.meta.file_id);
  if ((section !== 'mod' && section !== 'sound') || !Number.isFinite(modId) || !Number.isFinite(fileId)) {
    return undefined;
  }
  return { section, modId, fileId, author: task.meta.author ?? null };
}

function finished(task: TaskSnapshot, outcome: FinishedDownload['outcome']): FinishedDownload {
  const again = againOf(task);
  const version = task.meta.version;
  return {
    key: `task-${task.id}`,
    name: nameOf(task),
    file: task.file_name,
    outcome,
    ...(outcome === 'updated' && version ? { version } : {}),
    at: Date.now(),
    ...(again ? { again } : {}),
  };
}

export const app = new AppState();
