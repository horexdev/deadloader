// The Activity screen's state: the tab, filters and search, the loaded entries and the cursor of the next page, the
// totals and today's summary, the selected entry, open hops, and the live view (follow or paused with new entries
// waiting). The list is read a page at a time from the core; nothing is polled — new lines arrive as pushes.

import { activity, type LogCounts, type LogCursor, type LogEntry, type LogKind, type LogSummary } from '../../lib/api';
import { SvelteSet } from 'svelte/reactivity';
import {
  DEFAULT_FILTERS,
  appendNew,
  countNew,
  nowStamp,
  entryId,
  flatten,
  lineIds,
  mergeHead,
  queryFor,
  tabOf,
  type Filters,
  type Row,
  type Tab,
} from './model';

interface Options {
  /** Shows a failed call to the user (the app's toast). */
  fail: (error: unknown) => void;
  /** Entries kept at most; scrolling further loads nothing more (the 300 MB budget after heavy scrolling). */
  maxLoaded?: number;
}

/** About 10–20 MB of entries in the page; narrow the range or search to see older ones. */
export const MAX_LOADED = 10_000;

export class ActivityView {
  tab: Tab = $state('network');
  filters: Filters = $state({ ...DEFAULT_FILTERS });
  text = $state('');
  entries: LogEntry[] = $state([]);
  cursor: LogCursor | null = $state(null);
  /** The first page is loading. */
  loading = $state(false);
  loadingMore = $state(false);
  counts: LogCounts | null = $state(null);
  summary: LogSummary | null = $state(null);
  follow = $state(true);
  /** Newer entries waiting while following is paused. */
  pending = $state(0);
  /** When following was paused (RFC 3339). */
  pausedAt: string | null = $state(null);
  selected: string | null = $state(null);
  /** An entry opened from elsewhere (a link, a related id) that isn't in the list. */
  #opened: LogEntry | null = $state(null);
  openHops = new SvelteSet<string>();

  #fail: (error: unknown) => void;
  #maxLoaded: number;
  /** Bumped on every new list, so late answers for an old one are dropped. */
  #generation = 0;
  #waiting: { entries: LogEntry[]; cursor: LogCursor | null } = { entries: [], cursor: null };

  constructor(options: Options) {
    this.#fail = options.fail;
    this.#maxLoaded = options.maxLoaded ?? MAX_LOADED;
  }

  /** As many entries are loaded as are kept; older ones need a narrower range or a search. */
  get atLimit(): boolean {
    return this.cursor !== null && this.entries.length >= this.#maxLoaded;
  }

  get rows(): Row[] {
    return flatten(this.entries, this.openHops);
  }

  get selectedEntry(): LogEntry | null {
    const id = this.selected;
    if (id === null) {
      return null;
    }
    return (
      this.entries.find((e) => entryId(e) === id || lineIds(e).includes(id)) ??
      (this.#opened && (entryId(this.#opened) === id || lineIds(this.#opened).includes(id)) ? this.#opened : null)
    );
  }

  /** The first load; `target` is an entry id from the address (`#/activity/<id>`). */
  async open(target?: string): Promise<void> {
    if (target) {
      try {
        const found = await activity.entry(target);
        if (found) {
          this.tab = tabOf(found);
          this.#opened = found;
          this.selected = target;
          if (found.kind === 'network') {
            this.openHops.add(found.req);
          }
        }
      } catch (error) {
        this.#fail(error);
      }
    }
    await Promise.all([this.reload(), this.refreshTotals()]);
  }

  async refreshTotals(): Promise<void> {
    try {
      const [counts, summary] = await Promise.all([activity.counts(this.filters.range), activity.summary()]);
      this.counts = counts;
      this.summary = summary;
    } catch (error) {
      this.#fail(error);
    }
  }

  /** The newest page for the current tab, filters and search. */
  async reload(): Promise<void> {
    const generation = ++this.#generation;
    this.loading = true;
    this.pending = 0;
    this.#waiting = { entries: [], cursor: null };
    try {
      const page = await activity.page(queryFor(this.tab, this.filters, this.text, null));
      if (generation === this.#generation) {
        this.entries = page.entries;
        this.cursor = page.cursor;
      }
    } catch (error) {
      this.#fail(error);
    } finally {
      if (generation === this.#generation) {
        this.loading = false;
      }
    }
  }

  /** The next page, when there is one; asking again while it loads does nothing. */
  async more(): Promise<void> {
    if (!this.cursor || this.loadingMore || this.loading || this.atLimit) {
      return;
    }
    const generation = this.#generation;
    this.loadingMore = true;
    try {
      const page = await activity.page(queryFor(this.tab, this.filters, this.text, this.cursor));
      if (generation === this.#generation) {
        this.entries = appendNew(this.entries, page.entries);
        this.cursor = page.cursor;
      }
    } catch (error) {
      this.#fail(error);
    } finally {
      this.loadingMore = false;
    }
  }

  async setTab(tab: Tab): Promise<void> {
    if (tab === this.tab) {
      return;
    }
    this.tab = tab;
    this.selected = null;
    await this.reload();
  }

  async setFilter<K extends keyof Filters>(key: K, value: Filters[K]): Promise<void> {
    this.filters = { ...this.filters, [key]: value };
    await (key === 'range' ? Promise.all([this.reload(), this.refreshTotals()]) : this.reload());
  }

  async search(text: string): Promise<void> {
    this.text = text;
    await this.reload();
  }

  async clearFilters(): Promise<void> {
    const rangeChanged = this.filters.range !== DEFAULT_FILTERS.range;
    this.filters = { ...DEFAULT_FILTERS };
    await (rangeChanged ? Promise.all([this.reload(), this.refreshTotals()]) : this.reload());
  }

  async clearAll(): Promise<void> {
    this.text = '';
    await this.clearFilters();
  }

  /** Opens the details of an entry (a row, or a line id within a request). */
  select(id: string | null): void {
    this.selected = id;
  }

  /** Opens an entry that may be on another tab or outside the list (a related id, "Show entry"). */
  async jump(id: string): Promise<void> {
    try {
      const found = await activity.entry(id);
      if (!found) {
        return;
      }
      this.#opened = found;
      if (tabOf(found) !== this.tab) {
        this.tab = tabOf(found);
        await this.reload();
      }
      if (found.kind === 'network') {
        this.openHops.add(found.req);
      }
      this.selected = entryId(found);
    } catch (error) {
      this.#fail(error);
    }
  }

  toggleHops(req: string, open?: boolean): void {
    if (open ?? !this.openHops.has(req)) {
      this.openHops.add(req);
    } else {
      this.openHops.delete(req);
    }
  }

  setFollow(on: boolean): void {
    this.follow = on;
    if (on) {
      this.showNew();
    } else {
      this.pausedAt = nowStamp();
    }
  }

  /** Puts the waiting entries on top. */
  showNew(): void {
    if (this.#waiting.entries.length > 0) {
      this.#merge(this.#waiting.entries, this.#waiting.cursor);
    }
    this.#waiting = { entries: [], cursor: null };
    this.pending = 0;
  }

  /** A push: these logs got new lines. Totals always follow; the list only for its own log. */
  async onLog(kinds: LogKind[]): Promise<void> {
    const totals = this.refreshTotals();
    if (!kinds.includes(this.tab) || this.loading) {
      await totals;
      return;
    }
    const generation = this.#generation;
    try {
      const [page] = await Promise.all([activity.page(queryFor(this.tab, this.filters, this.text, null)), totals]);
      if (generation !== this.#generation) {
        return;
      }
      if (this.follow) {
        this.#merge(page.entries, page.cursor);
      } else {
        this.#waiting = page;
        this.pending = countNew(this.entries, page.entries);
      }
    } catch (error) {
      this.#fail(error);
    }
  }

  #merge(head: LogEntry[], cursor: LogCursor | null): void {
    const merged = mergeHead(this.entries, head);
    this.entries = merged.entries;
    if (!merged.joined) {
      // More arrived than a page: the list starts again from the newest page.
      this.cursor = cursor;
    }
  }
}
