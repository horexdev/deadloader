// Fixed geometry of the virtualized catalog grid and library list, mirrored from the design tokens (the virtualizer
// computes in JS and cannot read CSS custom properties). geometry.test.ts keeps them in sync with tokens.css.

/** `--card-w`: minimum catalog card width; cards stretch to fill the row (149). */
export const CARD_MIN_W = 220;
/** `--card-info-h`: the fixed info block under the thumbnail (151). */
export const CARD_INFO_H = 112;
/** `--space-4`: gap between catalog cards. */
export const GRID_GAP = 16;
/** `--space-1`: gap between library rows (`.dl-list`). */
export const LIST_GAP = 4;
/** `--row-h`: library row height. */
export const ROW_H = 56;
/** `--row-h-compact`: log row height. */
export const ROW_H_COMPACT = 36;

/** Catalog card height for a column width: round(width × 9/16) + info block (CatalogCard.md). */
export function catalogCardHeight(columnWidth: number): number {
  return Math.round((columnWidth * 9) / 16) + CARD_INFO_H;
}

/**
 * Which GameBanana thumbnail to load (152): the 220 px variant while the card needs at most 220 device pixels
 * (card width × display scale, which includes Windows DPI and the app's zoom), the 530 px one above that.
 */
export function thumbVariant(cardWidth: number, displayScale: number): 220 | 530 {
  return cardWidth * displayScale <= 220 ? 220 : 530;
}
