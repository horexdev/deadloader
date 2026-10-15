import { describe, expect, it } from 'vitest';
import { sidebarNetwork } from './network';

describe('the sidebar network status', () => {
  it('shows running requests as a live count while online', () => {
    expect(sidebarNetwork({ kind: 'online' }, 0)).toMatchObject({ state: 'online', active: false, label: 'Online' });
    expect(sidebarNetwork({ kind: 'online' }, 0).count).toBeUndefined();
    const busy = sidebarNetwork({ kind: 'online' }, 3);
    expect(busy).toMatchObject({ state: 'online', active: true, count: '3' });
    expect(busy.title).toBe('Online — 3 requests running — open Activity');
    expect(sidebarNetwork({ kind: 'online' }, 1).title).toBe('Online — 1 request running — open Activity');
  });

  it('says local mode and offline in words, whatever runs', () => {
    expect(sidebarNetwork({ kind: 'local_mode' }, 0)).toMatchObject({ state: 'local', active: false });
    const offline = sidebarNetwork({ kind: 'offline', since_unix_ms: 0 }, 2);
    expect(offline).toMatchObject({ state: 'offline', active: false });
    expect(offline.count).toBeUndefined();
  });
});
