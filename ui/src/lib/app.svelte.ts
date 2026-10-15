// What every screen shares: the library, the settings, the game, the network, running downloads and toasts. Filled
// at start and kept current by the core's pushes (`subscribe`); screens read it and call `api` for their own data.

import type { ToastData } from './components/ToastStack.svelte';
import { router } from './router.svelte';
import Undo2 from '@lucide/svelte/icons/undo-2';
import { describe, t } from '../i18n';
import {
  api,
  catalog,
  downloads,
  isUserMessage,
  library as libraryApi,
  shell,
  subscribe,
  updates as updatesApi,
  type ApplyResult,
  type ImportResult,
  type UpdateReport,
  type UpdateStep,
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
  /** The last mod update check (N3, the library's badges) and when it finished. */
  updates: UpdateReport | null = $state(null);
  updatesAt: number | null = $state(null);
  /** "Check for updates" is running. */
  checking = $state(false);
  /** S9 is open (the sidebar's pending-changes item). */
  pendingOpen = $state(false);
  /** Apply (S9) is putting the selected profile into the game folder. */
  applying = $state(false);
  /** The last apply or Play found gameinfo.gi changed by another program (D10). */
  gameinfoForeign = $state(false);
  /** An update that arrived needs the user to map its files (D5, 169). */
  mapping: Extract<UpdateStep, { kind: 'needs_mapping' }> | null = $state(null);
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
        this.toast({
          kind: 'success',
          title: describe({ id: 'N12', args: { count: Math.max(event.data.changes, 1) } }).what,
          action: { label: t('library-undo'), icon: Undo2, onclick: () => void this.undoApplied() },
        });
        break;
      case 'updates':
        this.updates = event.data;
        this.updatesAt = Date.now();
        break;
      case 'update_step':
        this.updateStep(event.data.step);
        break;
      case 'notice':
        if (event.data.kind === 'foreign') {
          this.toast({
            kind: 'info',
            title: describe({ id: 'N6', args: { count: event.data.files } }).what,
            action: { label: t('library-review'), onclick: () => router.go('library') },
          });
        }
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
        this.imported(event.data.outcomes);
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

  /** Files dropped on the window or added from disk (134): one toast for what joined the library, one per refusal. */
  imported(outcomes: ImportResult[]): void {
    const added = outcomes.filter((o) => 'Ok' in o).length;
    if (added > 0) {
      this.toast({ kind: 'success', title: t('library-added', { count: added }) });
    }
    for (const outcome of outcomes) {
      if ('Err' in outcome) {
        this.fail(outcome.Err);
      }
    }
  }

  /** Apply (S9): the selected profile goes into the game folder. */
  async apply(): Promise<void> {
    if (this.applying || this.playing) {
      return;
    }
    this.applying = true;
    try {
      const result = await api.apply();
      this.applied(result);
      const enabled = result.library.mods.filter((m) => m.enabled).length;
      this.toast({
        kind: 'success',
        title: describe({ id: 'N8', args: { name: result.library.selected_profile, count: enabled } }).what,
      });
    } catch (error) {
      this.fail(error);
    } finally {
      this.applying = false;
    }
  }

  /** After Apply or Play: the library, and D10 when gameinfo.gi is someone else's (E3). */
  private applied(result: ApplyResult): void {
    this.library = result.library;
    if (result.gameinfo_foreign) {
      this.gameinfoForeign = true;
      const text = describe({ id: 'E3', args: {} });
      this.toast({
        kind: 'warning',
        title: text.what,
        text: text.why ?? undefined,
        action: { label: t('library-show-differences'), onclick: () => router.go('library') },
      });
    }
  }

  /** An update the user picked in D5 arrived (169): in the library, or waiting for the user to map files. */
  updateStep(step: UpdateStep): void {
    if (step.kind === 'applied') {
      this.library = step.changed.library;
      const name = step.changed.library.mods.find((m) => m.id === step.mod_id)?.name ?? step.mod_id;
      this.toast({ kind: 'success', title: t('library-updated', { name }), text: t('library-updated-text') });
    } else {
      this.mapping = step;
      router.go('library', step.mod_id);
    }
  }

  /** Looks for the game again ("Search again" when Deadlock wasn't found). */
  async refreshGame(): Promise<void> {
    this.game = await shell.gameStatus().catch(() => this.game);
  }

  /** "Check for updates" (38, F8). */
  async checkUpdates(): Promise<void> {
    if (this.checking) {
      return;
    }
    this.checking = true;
    try {
      this.updates = await updatesApi.check();
      this.updatesAt = Date.now();
      if (this.updates.available > 0) {
        this.toast({ kind: 'info', title: describe({ id: 'N3', args: { count: this.updates.available } }).what });
      }
    } catch (error) {
      this.fail(error);
    } finally {
      this.checking = false;
    }
  }

  /** Undo of what was applied when the game closed (N12, 147). */
  async undoApplied(): Promise<void> {
    try {
      this.library = (await libraryApi.undoApplied()).library;
    } catch (error) {
      this.fail(error);
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
      this.applied(result);
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
}

export const app = new AppState();
