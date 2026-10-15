export type SizeUnit = 'bytes' | 'kb' | 'mb' | 'gb' | 'tb';

export interface SizeParts {
  readonly unit: SizeUnit;
  readonly value: number;
  /** Fraction digits to show, so the value has three significant digits. */
  readonly digits: number;
}

const UNITS: readonly SizeUnit[] = ['bytes', 'kb', 'mb', 'gb', 'tb'];

/**
 * A size as Windows Explorer shows it (`StrFormatByteSize`): binary units, three significant digits, truncated —
 * 1340 bytes is "1.30 KB", 23 506 bytes "22.9 KB". Below 1024 the bytes are counted.
 */
export function sizeParts(bytes: number): SizeParts {
  const b = Number.isFinite(bytes) ? Math.max(0, Math.floor(bytes)) : 0;
  if (b < 1024) {
    return { unit: 'bytes', value: b, digits: 0 };
  }
  let power = 1;
  while (power < UNITS.length - 1 && b / 1024 ** power >= 1000) {
    power += 1;
  }
  const divisor = 1024 ** power;
  const whole = b / divisor;
  const digits = whole < 10 ? 2 : whole < 100 ? 1 : 0;
  const scale = 10 ** digits;
  return { unit: UNITS[power] ?? 'tb', value: Math.floor((b * scale) / divisor) / scale, digits };
}
