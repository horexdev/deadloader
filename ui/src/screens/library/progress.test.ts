import { describe, expect, it } from 'vitest';
import type { Applying } from '../../lib/api';
import { progressFiles, progressLeft, progressStep, progressValue } from './progress';

function at(over: Partial<Applying>): Applying {
  return {
    done: 0,
    total: 4,
    linked: 0,
    copied: 0,
    bytes_done: 0,
    bytes_total: 0,
    bytes_copied: 0,
    left_ms: null,
    finished: false,
    ...over,
  };
}

describe("the applying banner's progress", () => {
  it('fills by steps while files are linked, by bytes once they are copied', () => {
    expect(progressValue(at({ done: 1, total: 4 }))).toBe(0.25);
    expect(progressValue(at({ done: 1, total: 4, bytes_done: 300, bytes_total: 400, bytes_copied: 300 }))).toBe(0.75);
    expect(progressValue(at({ done: 0, total: 0 }))).toBe(0);
  });

  it('names the step in progress', () => {
    expect(progressStep(at({ done: 0, total: 4 }))).toBe('Step 1 of 4');
    expect(progressStep(at({ done: 3, total: 4 }))).toBe('Step 4 of 4');
    expect(progressStep(at({ done: 4, total: 4 }))).toBe('Step 4 of 4');
  });

  it('counts the files linked and copied, and says nothing before the first', () => {
    expect(progressFiles(at({}))).toBe('');
    expect(progressFiles(at({ linked: 9 }))).toBe('9 files linked');
    expect(progressFiles(at({ copied: 1 }))).toBe('1 file copied');
    expect(progressFiles(at({ linked: 9, copied: 1 }))).toBe('9 files linked, 1 copied');
  });

  it('gives the time left only when the core could estimate it', () => {
    expect(progressLeft(at({}))).toBe('');
    expect(progressLeft(at({ left_ms: 400 }))).toBe('');
    expect(progressLeft(at({ left_ms: 4200 }))).toBe('About 5 s left');
    expect(progressLeft(at({ left_ms: 125_000 }))).toBe('About 2 min 5 s left');
  });
});
