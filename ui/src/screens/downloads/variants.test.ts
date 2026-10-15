import { describe, expect, it } from 'vitest';
import { fileName, variantName } from './variants';

describe('the name of a variant', () => {
  it('is the folder it sits in inside the archive', () => {
    expect(variantName('Tracksuit/pak_pocket_tracksuit.vpk')).toBe('Tracksuit');
    expect(variantName('pocket\\Summer dress\\pak01_dir.vpk')).toBe('Summer dress');
  });

  it('shows the file itself without its folders', () => {
    expect(fileName('pocket\\Summer dress\\pak01_dir.vpk')).toBe('pak01_dir.vpk');
    expect(fileName('pak01_dir.vpk')).toBe('pak01_dir.vpk');
  });

  it('is the file name without .vpk when the file sits at the top', () => {
    expect(variantName('pak_mcafe_casual.vpk')).toBe('pak_mcafe_casual');
    expect(variantName('PAK01_DIR.VPK')).toBe('PAK01_DIR');
  });
});
