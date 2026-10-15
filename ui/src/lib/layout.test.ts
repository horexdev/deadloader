import { describe, expect, it } from 'vitest';
import tokens from '../theme/tokens.css?raw';
import { SIDEBAR_AUTO_COLLAPSE_PX, sidebarMode } from './layout';

describe('sidebarMode', () => {
  it('shows the full sidebar from the auto-collapse width up', () => {
    expect(sidebarMode(SIDEBAR_AUTO_COLLAPSE_PX)).toBe('full');
    expect(sidebarMode(1920)).toBe('full');
  });

  it('collapses to the icon rail below the auto-collapse width', () => {
    expect(sidebarMode(SIDEBAR_AUTO_COLLAPSE_PX - 1)).toBe('rail');
    expect(sidebarMode(960)).toBe('rail');
  });

  it('matches the design token, which CSS media queries cannot read', () => {
    const token = /--sidebar-auto-collapse:\s*(\d+)px;/.exec(tokens)?.[1];
    expect(SIDEBAR_AUTO_COLLAPSE_PX).toBe(Number(token));
  });
});
