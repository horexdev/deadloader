// The first-run notice (S1, decisions 107–111): its version, what changed between versions, and screens asking for it.
// Settings → Catalog turning the catalog on shows its network step again (F13); Settings → About re-opens the whole
// notice to read. The wizard covers the window while it is needed or asked for.

import type { Startup } from './api';

/** The steps of the notice, in order. */
export const STEPS = ['welcome', 'risks', 'network', 'privacy', 'updates', 'setup'] as const;
export type Step = (typeof STEPS)[number];

/** A material change of the notice (111): which steps changed, when, and the "what changed" text. */
export interface NoticeChange {
  version: number;
  /** `YYYY-MM-DD`. */
  date: string;
  steps: Step[];
  /** Hosts this version adds to the network step's list, marked "New" there. */
  hosts?: string[];
  /** The text in the UI language. */
  text: () => string;
}

/** The notice as this build ships it. Bump `version` and add a change when a step changes materially. */
export const NOTICE: { version: number; changes: NoticeChange[] } = {
  version: 1,
  changes: [],
};

export interface Acknowledged {
  version: number;
  at: string;
  language: string;
  catalog: 'catalog' | 'local_mode';
}

/** What the wizard shows. */
export type NoticeMode =
  /** Never acknowledged: all six steps. */
  | { kind: 'first' }
  /** An older version was acknowledged: only the changed steps, with "what changed" (111). */
  | { kind: 'changed'; changes: NoticeChange[]; previous: Acknowledged }
  /** F13: only the network step. */
  | { kind: 'catalog' }
  /** "Read it again": every step, nothing to choose or save. */
  | { kind: 'review' };

/** What the notice needs at start for the acknowledgement on record; `null` when it is current. */
export function noticeNeeded(acknowledged: Acknowledged | null | undefined): NoticeMode | null {
  if (!acknowledged) {
    return { kind: 'first' };
  }
  if (acknowledged.version >= NOTICE.version) {
    return null;
  }
  const changes = NOTICE.changes.filter((c) => c.version > acknowledged.version && c.version <= NOTICE.version);
  // A newer version with no recorded change shows everything again rather than nothing.
  return changes.length > 0 ? { kind: 'changed', changes, previous: acknowledged } : { kind: 'first' };
}

export type NoticeRequest = { mode: 'catalog' } | { mode: 'review' };

/** What the window shows: E1 alone, nothing while the core and the app state load, the wizard, or the app. */
export type WindowView =
  | { kind: 'blocked'; folder: string; detail: string }
  | { kind: 'loading' }
  | { kind: 'wizard'; mode: NoticeMode }
  | { kind: 'shell' };

/** What decides the view: how the core started, whether the app state loaded, the settings' acknowledgement. */
export interface WindowFacts {
  started: Startup | null;
  loaded: boolean;
  /** `false` when the settings couldn't be read: the app opens (its toast says why) rather than asking again. */
  settingsRead: boolean;
  acknowledged: Acknowledged | null | undefined;
  request: NoticeRequest | null;
}

/**
 * E1 blocks everything (57); otherwise the wizard covers the window while the notice on record is missing or older,
 * or while a screen asks for it (F13, "Read it again").
 */
export function windowView(facts: WindowFacts): WindowView {
  const { started, loaded, settingsRead, acknowledged, request } = facts;
  if (started?.kind === 'no_write') {
    return { kind: 'blocked', folder: started.folder, detail: started.detail };
  }
  if (started === null || !loaded) {
    return { kind: 'loading' };
  }
  if (request) {
    return { kind: 'wizard', mode: request.mode === 'catalog' ? { kind: 'catalog' } : { kind: 'review' } };
  }
  const needed = settingsRead ? noticeNeeded(acknowledged) : null;
  return needed ? { kind: 'wizard', mode: needed } : { kind: 'shell' };
}

class NoticeRequests {
  current: NoticeRequest | null = $state(null);

  open(request: NoticeRequest): void {
    this.current = request;
  }

  close(): void {
    this.current = null;
  }
}

export const notice = new NoticeRequests();
