import { afterEach, describe, expect, it } from 'vitest';
import { NOTICE, noticeNeeded, windowView, type WindowFacts } from './notice.svelte';

const ack = (version: number) => ({ version, at: '2026-09-02T10:00:00Z', language: 'en', catalog: 'catalog' as const });

describe('the first-run notice', () => {
  const original = { ...NOTICE, changes: [...NOTICE.changes] };
  afterEach(() => {
    NOTICE.version = original.version;
    NOTICE.changes = [...original.changes];
  });

  it('is shown in full until it is acknowledged, and not again for the same version', () => {
    expect(noticeNeeded(null)).toEqual({ kind: 'first' });
    expect(noticeNeeded(ack(NOTICE.version))).toBeNull();
  });

  it('shows only the changed steps of a newer version (111)', () => {
    NOTICE.version = 3;
    NOTICE.changes = [
      { version: 2, date: '2026-10-01', steps: ['privacy'], text: () => 'two' },
      { version: 3, date: '2026-10-14', steps: ['network'], text: () => 'three' },
    ];
    const mode = noticeNeeded(ack(2));
    expect(mode?.kind).toBe('changed');
    expect(mode?.kind === 'changed' ? mode.changes.map((c) => c.version) : []).toEqual([3]);
    expect(noticeNeeded(ack(1))?.kind).toBe('changed');
  });

  it('shows everything again when a newer version has no recorded change', () => {
    NOTICE.version = 2;
    NOTICE.changes = [];
    expect(noticeNeeded(ack(1))).toEqual({ kind: 'first' });
  });
});

describe('what the window shows at start', () => {
  const ready = { kind: 'ready' } as const;
  const blocked = { kind: 'no_write', folder: String.raw`C:\Program Files\deadloader`, detail: 'denied' } as const;
  const facts = (over: Partial<WindowFacts> = {}): WindowFacts => ({
    started: ready,
    loaded: true,
    settingsRead: true,
    acknowledged: ack(NOTICE.version),
    request: null,
    ...over,
  });

  it('shows E1 alone when the data folder cannot be written', () => {
    expect(windowView(facts({ started: blocked, loaded: false }))).toEqual({
      kind: 'blocked',
      folder: blocked.folder,
      detail: 'denied',
    });
    expect(windowView(facts({ started: blocked, request: { mode: 'review' } })).kind).toBe('blocked');
  });

  it('waits for the core and the app state before anything else', () => {
    expect(windowView(facts({ started: null, loaded: false }))).toEqual({ kind: 'loading' });
    expect(windowView(facts({ loaded: false }))).toEqual({ kind: 'loading' });
  });

  it('covers the window with the wizard until the notice is acknowledged', () => {
    expect(windowView(facts({ acknowledged: null }))).toEqual({ kind: 'wizard', mode: { kind: 'first' } });
    expect(windowView(facts())).toEqual({ kind: 'shell' });
  });

  it('opens the app when the settings could not be read, rather than asking again', () => {
    expect(windowView(facts({ settingsRead: false, acknowledged: undefined }))).toEqual({ kind: 'shell' });
  });

  it('shows the network step or the whole notice when a screen asks for it (F13, About)', () => {
    expect(windowView(facts({ request: { mode: 'catalog' } }))).toEqual({ kind: 'wizard', mode: { kind: 'catalog' } });
    expect(windowView(facts({ request: { mode: 'review' } }))).toEqual({ kind: 'wizard', mode: { kind: 'review' } });
  });
});
