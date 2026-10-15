import { describe, expect, it } from 'vitest';
import { edgeEnabled, stepEnabled, typeahead } from './roving';

const items = [{ disabled: false }, { disabled: true }, { disabled: false }, { disabled: false }, { disabled: true }];

describe('stepEnabled', () => {
  it('moves to the next enabled item, skipping disabled ones', () => {
    expect(stepEnabled(items, 0, 1)).toBe(2);
    expect(stepEnabled(items, 2, -1)).toBe(0);
  });

  it('wraps around both ends when asked', () => {
    expect(stepEnabled(items, 3, 1, true)).toBe(0);
    expect(stepEnabled(items, 0, -1, true)).toBe(3);
  });

  it('stays put at the ends without wrapping', () => {
    expect(stepEnabled(items, 3, 1, false)).toBe(3);
    expect(stepEnabled(items, 0, -1, false)).toBe(0);
  });

  it('starts from the edge when nothing is active', () => {
    expect(stepEnabled(items, -1, 1)).toBe(0);
    expect(stepEnabled(items, -1, -1)).toBe(3);
  });

  it('returns -1 when every item is disabled', () => {
    expect(stepEnabled([{ disabled: true }], 0, 1, true)).toBe(-1);
    expect(stepEnabled([], -1, 1, true)).toBe(-1);
  });
});

describe('edgeEnabled', () => {
  it('finds the first and last enabled items', () => {
    expect(edgeEnabled(items, 'first')).toBe(0);
    expect(edgeEnabled(items, 'last')).toBe(3);
    expect(edgeEnabled([{ disabled: true }], 'first')).toBe(-1);
  });
});

describe('typeahead', () => {
  const labels = [
    { label: 'Most liked', disabled: false },
    { label: 'Most downloaded', disabled: false },
    { label: 'Newest', disabled: false },
    { label: 'Mina', disabled: true },
    { label: 'Ändern', disabled: false },
  ];

  it('finds the next item after the active one that starts with the typed text', () => {
    expect(typeahead(labels, 0, 'm')).toBe(1);
    expect(typeahead(labels, 1, 'm')).toBe(0);
    expect(typeahead(labels, 0, 'most d')).toBe(1);
  });

  it('keeps the active item while it still matches a longer prefix', () => {
    expect(typeahead(labels, 0, 'most l')).toBe(0);
  });

  it('ignores case and skips disabled items', () => {
    expect(typeahead(labels, 2, 'MI')).toBe(-1);
    expect(typeahead(labels, 0, 'ä')).toBe(4);
  });
});
