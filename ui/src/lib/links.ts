// Stand-in until the Catalog screen brings D1 (the external-link dialog, decision 31): the address goes to the core,
// which opens only https pages in the system browser. The Catalog owner's version replaces this file.

import { invoke } from '@tauri-apps/api/core';

export async function openLink(url: string): Promise<void> {
  await invoke<null>('open_link', { url });
}
