/** Below this window width the sidebar collapses to the icon rail (design token `--sidebar-auto-collapse`). */
export const SIDEBAR_AUTO_COLLAPSE_PX = 1100;

export type SidebarMode = 'full' | 'rail';

export function sidebarMode(windowWidth: number): SidebarMode {
  return windowWidth < SIDEBAR_AUTO_COLLAPSE_PX ? 'rail' : 'full';
}
