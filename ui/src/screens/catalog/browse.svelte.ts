// The catalog grid's state (S2): the query the user built (section, search, sort, category, hero), the pages loaded so
// far and the filters applied on top of them (favourites only, hide installed). It lives as long as the app, so going
// to a mod page or another area and back finds the grid as it was, scroll position included.

import { SvelteSet } from 'svelte/reactivity';
import {
  catalog,
  favourites as favouritesApi,
  isUserMessage,
  type Card,
  type CatalogError,
  type Category,
  type Favourite,
  type ImageId,
  type Origin,
  type Query,
  type Section,
  type Sort,
  type UserMessage,
} from '../../lib/api';
import { app } from '../../lib/app.svelte';
import { libraryModOf, searchable } from './model';

/** Why the grid can't be shown: a catalog answer, or a message when the catalog itself is unavailable. */
export type BrowseError = CatalogError | UserMessage;

/** How far down the loaded cards the next page is fetched into the cache ahead of time (decision 28). */
const PREFETCH_AT = 0.7;
/** Pages listed in a row while the filters on top hide nearly everything, before waiting for the user to scroll. */
const FILL_PAGES = 4;

function toError(error: unknown): BrowseError {
  if (isUserMessage(error)) {
    return error;
  }
  if (typeof error === 'object' && error !== null && typeof (error as { kind?: unknown }).kind === 'string') {
    return error as CatalogError;
  }
  return { id: 'Internal', args: { detail: String(error) } };
}

function sameQuery(a: Query, b: Query): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}

/** A favourite shown as a card; its picture comes from its mod page when that is at hand. */
function favouriteCard(favourite: Favourite, thumb: ImageId | undefined): Card {
  return {
    id: favourite.id,
    section: favourite.section,
    name: favourite.name,
    author: favourite.author,
    root_category: favourite.root_category,
    subcategory: favourite.subcategory,
    likes: 0,
    views: 0,
    date_added: null,
    date_updated: favourite.date_updated,
    thumbnail: thumb ? { small: thumb, large: thumb } : null,
    has_audio_preview: false,
    nsfw: favourite.nsfw,
    obsolete: false,
    featured: false,
    has_files: true,
  };
}

export class CatalogBrowser {
  section: Section = $state('mod');
  /** The search as typed. */
  text = $state('');
  sort: Sort = $state('newest');
  /** A root category id. */
  category: number | null = $state(null);
  /** A hero (a subcategory of Skins). */
  hero: number | null = $state(null);
  favouritesOnly = $state(false);
  hideInstalled = $state(false);

  pages: Card[][] = $state([]);
  hasMore = $state(false);
  /** The first page is on its way. */
  loading = $state(false);
  loadingMore = $state(false);
  /** Why nothing can be shown. */
  error: BrowseError | null = $state(null);
  /** Why the next page didn't come (offline, rate-limited): the loaded cards stay. */
  moreError: BrowseError | null = $state(null);
  /** Where the first page came from: a saved one is shown with its time while offline (N7). */
  origin: Origin | null = $state(null);
  hiddenNsfw = $state(0);

  roots: Category[] = $state([]);
  heroes: Category[] = $state([]);
  favourites: Favourite[] = $state([]);
  /** Pictures of favourites, from their mod pages. */
  favouriteThumbs = $state<Record<string, ImageId>>({});
  /** Blurred cards the user revealed in this session (decision 78). */
  revealed = new SvelteSet<string>();
  /** Where the grid was scrolled, to come back to it. */
  scrollTop = 0;

  // Bookkeeping, never rendered: plain sets.
  #generation = 0;
  // eslint-disable-next-line svelte/prefer-svelte-reactivity
  #prefetched = new Set<number>();
  // eslint-disable-next-line svelte/prefer-svelte-reactivity
  #refetched = new Set<number>();
  #lastEnd = 0;
  #favouritesLoaded = false;
  // eslint-disable-next-line svelte/prefer-svelte-reactivity
  #thumbsAsked = new Set<string>();

  /** Back to a fresh grid: the default query, nothing loaded (tests; a fresh start of the screen). */
  reset(): void {
    this.#generation += 1;
    this.section = 'mod';
    this.text = '';
    this.sort = 'newest';
    this.category = null;
    this.hero = null;
    this.favouritesOnly = false;
    this.hideInstalled = false;
    this.pages = [];
    this.hasMore = false;
    this.loading = false;
    this.loadingMore = false;
    this.error = null;
    this.moreError = null;
    this.origin = null;
    this.roots = [];
    this.heroes = [];
    this.favourites = [];
    this.favouriteThumbs = {};
    this.revealed.clear();
    this.scrollTop = 0;
    this.#favouritesLoaded = false;
    this.#thumbsAsked.clear();
  }

  /** The search text GameBanana takes (2–100 bytes), or null to browse. */
  get searchText(): string | null {
    return searchable(this.text) ? this.text.trim() : null;
  }

  get searching(): boolean {
    return this.searchText !== null;
  }

  /** The cards to show: the loaded pages (or the favourites) with the filters on top. */
  get items(): Card[] {
    const source = this.favouritesOnly
      ? this.favourites
          .filter((f) => f.section === this.section)
          .map((f) => favouriteCard(f, this.favouriteThumbs[`${f.section}-${f.id}`]))
      : this.pages.flat();
    const text = this.favouritesOnly ? this.searchText?.toLocaleLowerCase() : undefined;
    // While searching (or listing favourites) GameBanana doesn't filter by category: it is done here.
    const byCategory = this.favouritesOnly || this.searching;
    const nsfw = app.settings?.catalog?.nsfw ?? 'hidden';
    return source.filter(
      (card) =>
        !(this.hideInstalled && libraryModOf(app.library, card.section, card.id)) &&
        !(text && !card.name.toLocaleLowerCase().includes(text)) &&
        !(byCategory && this.category !== null && card.root_category?.id !== this.category) &&
        !(byCategory && this.hero !== null && card.subcategory?.id !== this.hero) &&
        !(this.favouritesOnly && card.nsfw && nsfw === 'hidden'),
    );
  }

  /** Filters behind the Filters button that are not at their default (its count). */
  get activeFilters(): number {
    const nsfw = app.settings?.catalog?.nsfw ?? 'hidden';
    return Number(nsfw !== 'hidden') + Number(this.favouritesOnly) + Number(this.hideInstalled);
  }

  query(page: number): Query {
    const text = this.searchText;
    return {
      section: this.section,
      filter:
        text !== null
          ? { kind: 'search', text }
          : { kind: 'browse', sort: this.sort, category: this.hero ?? this.category },
      page,
    };
  }

  /** Lists the first page again for the current query. */
  async load(): Promise<void> {
    const generation = ++this.#generation;
    this.pages = [];
    this.hasMore = false;
    this.error = null;
    this.moreError = null;
    this.origin = null;
    this.hiddenNsfw = 0;
    this.scrollTop = 0;
    this.#prefetched.clear();
    this.#refetched.clear();
    this.#lastEnd = 0;
    if (this.favouritesOnly) {
      this.loading = false;
      await this.loadFavourites();
      return;
    }
    this.loading = true;
    try {
      const listing = await catalog.list(this.query(1), true);
      if (generation !== this.#generation) {
        return;
      }
      this.pages = [listing.items];
      this.hasMore = listing.has_more;
      this.origin = listing.origin;
      this.hiddenNsfw = listing.hidden_nsfw;
    } catch (error) {
      if (generation === this.#generation) {
        this.error = toError(error);
      }
    } finally {
      if (generation === this.#generation) {
        this.loading = false;
      }
    }
    if (generation === this.#generation) {
      await this.#fill(generation);
    }
  }

  /** Lists the next page (the user reached the end of the loaded cards). */
  async more(): Promise<void> {
    if (this.loading || this.loadingMore || !this.hasMore || this.favouritesOnly) {
      return;
    }
    const generation = this.#generation;
    const page = this.pages.length + 1;
    this.loadingMore = true;
    try {
      const listing = await catalog.list(this.query(page), true);
      if (generation !== this.#generation) {
        return;
      }
      this.pages = [...this.pages, listing.items];
      this.hasMore = listing.has_more;
      this.hiddenNsfw += listing.hidden_nsfw;
      this.moreError = null;
    } catch (error) {
      if (generation === this.#generation) {
        this.moreError = toError(error);
      }
    } finally {
      if (generation === this.#generation) {
        this.loadingMore = false;
      }
    }
  }

  /**
   * The grid shows cards `start`..`end` (with its overscan): at 70 % of the loaded cards the next page is fetched
   * into the cache, at the end it is listed.
   */
  async rangeChanged(start: number, end: number): Promise<void> {
    this.#lastEnd = end;
    if (this.favouritesOnly) {
      this.#resolveThumbs(start, end);
      return;
    }
    const count = this.items.length;
    if (!this.hasMore || this.loading || this.moreError) {
      return;
    }
    const next = this.pages.length + 1;
    if (count > 0 && end >= count * PREFETCH_AT && !this.#prefetched.has(next) && app.settings?.catalog?.prefetch_next_page !== false) {
      this.#prefetched.add(next);
      void catalog.prefetch(this.query(next)).catch(() => undefined);
    }
    if (end >= count) {
      const generation = this.#generation;
      await this.more();
      await this.#fill(generation);
    }
  }

  /** Lists more pages while the filters hide nearly all of them and the grid still has room. */
  async #fill(generation: number): Promise<void> {
    for (let i = 0; i < FILL_PAGES; i += 1) {
      if (generation !== this.#generation || !this.hasMore || this.moreError || this.#lastEnd < this.items.length) {
        return;
      }
      await this.more();
    }
  }

  /** A card's thumbnail failed: its picture id may have been forgotten by the core, so list its page again, once. */
  async thumbFailed(card: Card): Promise<void> {
    const index = this.pages.findIndex((page) => page.some((c) => c.id === card.id && c.section === card.section));
    if (index < 0 || this.#refetched.has(index)) {
      return;
    }
    this.#refetched.add(index);
    await this.#replacePage(index);
  }

  /** The core refreshed a page in the background (`page_updated`): take it if it is one of ours. */
  async pageUpdated(query: Query): Promise<void> {
    if (this.favouritesOnly) {
      return;
    }
    const index = this.pages.findIndex((_, i) => sameQuery(this.query(i + 1), query));
    if (index >= 0) {
      await this.#replacePage(index);
    }
  }

  async #replacePage(index: number): Promise<void> {
    const generation = this.#generation;
    try {
      const listing = await catalog.list(this.query(index + 1), false);
      if (generation === this.#generation && index < this.pages.length) {
        this.pages = this.pages.map((page, i) => (i === index ? listing.items : page));
        if (index === 0) {
          this.origin = listing.origin;
        }
      }
    } catch {
      // The cards already shown stay; the network state says why.
    }
  }

  async loadCategories(): Promise<void> {
    const section = this.section;
    try {
      const answer = await catalog.categories(section, null);
      if (section === this.section) {
        this.roots = answer.items;
      }
    } catch {
      this.roots = [];
    }
  }

  async #loadHeroes(): Promise<void> {
    const root = this.roots.find((c) => c.id === this.category);
    if (!root || root.subcategory_count === 0) {
      this.heroes = [];
      return;
    }
    try {
      const answer = await catalog.categories(this.section, root.id);
      if (this.category === root.id) {
        this.heroes = answer.items;
      }
    } catch {
      this.heroes = [];
    }
  }

  async loadFavourites(): Promise<void> {
    try {
      this.favourites = await favouritesApi.list();
      this.#favouritesLoaded = true;
    } catch (error) {
      app.fail(error);
    }
  }

  /** Favourites' pictures come from their saved mod pages (or one request each), only for the cards in view. */
  #resolveThumbs(start: number, end: number): void {
    if (app.network.kind === 'local_mode') {
      return;
    }
    for (const card of this.items.slice(start, end)) {
      const key = `${card.section}-${card.id}`;
      if (this.#thumbsAsked.has(key)) {
        continue;
      }
      this.#thumbsAsked.add(key);
      void catalog
        .details(card.section, card.id, card.date_updated)
        .then((page) => {
          const preview = page.gallery[0]?.preview;
          if (preview) {
            this.favouriteThumbs = { ...this.favouriteThumbs, [key]: preview };
          }
        })
        .catch(() => undefined);
    }
  }

  isFavourite(card: Pick<Card, 'id' | 'section'>): boolean {
    return this.favourites.some((f) => f.id === card.id && f.section === card.section);
  }

  /** The heart: marks or unmarks a mod; the card's data is saved with it. */
  async toggleFavourite(card: Card): Promise<void> {
    if (!this.#favouritesLoaded) {
      await this.loadFavourites();
    }
    const on = !this.isFavourite(card);
    // A plain copy: the card may be part of the state.
    const plain = $state.snapshot(card) as Card;
    const favourite: Favourite = {
      section: plain.section,
      id: plain.id,
      name: plain.name,
      author: plain.author,
      root_category: plain.root_category,
      subcategory: plain.subcategory,
      nsfw: plain.nsfw,
      date_updated: plain.date_updated,
    };
    try {
      this.favourites = await favouritesApi.set(favourite, on);
    } catch (error) {
      app.fail(error);
    }
  }

  async setSection(section: Section): Promise<void> {
    if (section === this.section) {
      return;
    }
    this.section = section;
    this.category = null;
    this.hero = null;
    this.heroes = [];
    this.roots = [];
    void this.loadCategories();
    await this.load();
  }

  /** The search after typing stops; only what GameBanana takes is sent. */
  async search(text: string): Promise<void> {
    const before = this.searchText;
    this.text = text;
    if (this.searchText !== before) {
      await this.load();
    }
  }

  async setSort(sort: Sort): Promise<void> {
    this.sort = sort;
    await this.load();
  }

  async setCategory(category: number | null): Promise<void> {
    this.category = category;
    this.hero = null;
    this.heroes = [];
    if (category !== null && this.roots.length === 0) {
      await this.loadCategories();
    }
    const heroes = this.#loadHeroes();
    if (this.searching || this.favouritesOnly) {
      // The results are filtered here; no new request.
      await heroes;
      return;
    }
    await Promise.all([heroes, this.load()]);
  }

  async setHero(hero: number | null): Promise<void> {
    this.hero = hero;
    if (!this.searching && !this.favouritesOnly) {
      await this.load();
    }
  }

  async setFavouritesOnly(on: boolean): Promise<void> {
    this.favouritesOnly = on;
    await this.load();
  }

  setHideInstalled(on: boolean): void {
    this.hideInstalled = on;
  }

  /** "Clear filters" of the no-results state: category, hero, favourites only, hide installed. */
  async clearFilters(): Promise<void> {
    this.category = null;
    this.hero = null;
    this.heroes = [];
    this.favouritesOnly = false;
    this.hideInstalled = false;
    await this.load();
  }

  async clearSearch(): Promise<void> {
    await this.search('');
  }
}

/** The one grid of the app. */
export const browser = new CatalogBrowser();
