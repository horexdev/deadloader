// The first-run notice (S1, decisions 107–111) as screens ask for it: Settings → Catalog turning the catalog on shows its
// network step again (F13), Settings → About re-opens the whole notice to read. The wizard covers the window while a
// request is open; it answers by closing it.

export type NoticeRequest =
  /** F13: only the network step, ending in "Use the catalog" or "Not now". */
  | { mode: 'catalog' }
  /** "Read it again": every step, nothing to choose or save. */
  | { mode: 'review' };

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
