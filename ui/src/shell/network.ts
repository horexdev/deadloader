// The sidebar's network status (design: Sidebar.md, "Network status"): online with the running requests as a live
// count, offline, or local mode. Nothing animates: running requests only change the glyph's colour and its number.

import type { SidebarNetwork } from '../lib/components/Sidebar.svelte';
import type { CatalogStatus } from '../lib/api';
import { formatNumber, formatTime, t } from '../i18n';

export function sidebarNetwork(status: CatalogStatus, running: number): SidebarNetwork {
  if (status.kind === 'local_mode') {
    return {
      state: 'local',
      active: false,
      label: t('network-local'),
      detail: t('network-local-detail'),
      title: t('network-title-local'),
    };
  }
  if (status.kind === 'offline') {
    const time = formatTime(status.since_unix_ms);
    return {
      state: 'offline',
      active: false,
      label: t('network-offline'),
      detail: t('network-offline-detail', { time }),
      title: t('network-title-offline', { time }),
    };
  }
  // Busy or changed: still online here; the catalog shows the banner (E4, E5).
  if (running > 0) {
    return {
      state: 'online',
      active: true,
      label: t('network-online'),
      count: formatNumber(running),
      title: t('network-title-running', { count: running }),
    };
  }
  return { state: 'online', active: false, label: t('network-online'), title: t('network-title-online') };
}
