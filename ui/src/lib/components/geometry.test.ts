import { describe, expect, it } from 'vitest';
import tokens from '../../theme/tokens.css?raw';
import components from '../../theme/components.css?raw';
import {
  CARD_INFO_H,
  CARD_MIN_W,
  GRID_GAP,
  LIST_GAP,
  ROW_H,
  ROW_H_COMPACT,
  catalogCardHeight,
  thumbVariant,
} from './geometry';

function token(name: string): number {
  const value = new RegExp(`--${name}:\\s*(\\d+)px;`).exec(tokens)?.[1];
  return Number(value);
}

describe('geometry constants', () => {
  it('match the design tokens, which the virtualizer cannot read from CSS', () => {
    expect(CARD_MIN_W).toBe(token('card-w'));
    expect(CARD_INFO_H).toBe(token('card-info-h'));
    expect(GRID_GAP).toBe(token('space-4'));
    expect(LIST_GAP).toBe(token('space-1'));
    expect(ROW_H).toBe(token('row-h'));
    expect(ROW_H_COMPACT).toBe(token('row-h-compact'));
  });

  it('match the gaps the design stylesheet uses for the grid and the list', () => {
    expect(components).toMatch(/\.dl-card-grid \{[^}]*gap: var\(--space-4\)/);
    expect(components).toMatch(/\.dl-list \{[^}]*gap: var\(--space-1\)/);
  });
});

describe('catalogCardHeight', () => {
  it('is the 16:9 thumbnail of the column width plus the info block (236 at 220 px)', () => {
    expect(catalogCardHeight(220)).toBe(236);
    expect(catalogCardHeight(298)).toBe(Math.round((298 * 9) / 16) + 112);
  });
});

describe('thumbVariant', () => {
  it('uses the 220 px image while the card needs at most 220 device pixels, else 530 (decision 152)', () => {
    expect(thumbVariant(220, 1)).toBe(220);
    expect(thumbVariant(298, 1)).toBe(530);
    expect(thumbVariant(220, 1.5)).toBe(530);
    expect(thumbVariant(146, 1.5)).toBe(220);
  });
});

describe('reduced motion', () => {
  it('drops every transition and animation to 0 ms under prefers-reduced-motion and data-reduce-motion', () => {
    expect(components).toMatch(/@media \(prefers-reduced-motion: reduce\) \{\s*\.dl-root \*/);
    expect(components).toMatch(/\.dl-root\[data-reduce-motion\] \*/);
  });
});
