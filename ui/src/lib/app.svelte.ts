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
  type CatalogStatus,
  type GameStatus,
  type LibraryView,
  type Settings,
  type TaskSnapshot,
  type UiEvent,
  type UserMessage,
} from './api';

/** D13: what Play would change, and the profiles involved. */
export interface PlayConfirmation extends ApplyPreview {
  profile: string;
  applied: string;
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
        }
        break;
      case 'download':
        this.followDownload(event.data);
        break;
      case 'installed':
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
        this.tasks = this.tasks.filter((t) => t.id !== event.id);
        break;
    }
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

export const app = new AppState();
