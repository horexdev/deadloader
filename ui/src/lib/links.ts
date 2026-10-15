// External links (decision 31): they open only in the user's browser, after D1 shows the full address, unless the user
// chose "always open links immediately" (Settings → Catalog). Every screen opens links through `openLink`.

import { api } from './api';
import { app } from './app.svelte';
import { prompts, saveCatalogSetting } from './prompts.svelte';

/** Shows D1 for `url` (or opens it at once when the setting says so), then hands it to the browser. */
export async function openLink(url: string): Promise<void> {
  if (app.settings?.catalog?.external_links !== 'open_immediately') {
    const answer = await prompts.askLink(url);
    if (!answer.open) {
      return;
    }
    if (answer.always) {
      await saveCatalogSetting({ external_links: 'open_immediately' });
    }
  }
  try {
    await api.openLink(url);
  } catch (error) {
    app.fail(error);
  }
}

/** The address split for D1: the host is highlighted, so a look-alike address stands out. */
export function splitUrl(url: string): { before: string; host: string; after: string } {
  const match = /^([a-z][a-z0-9+.-]*:\/\/(?:[^@/?#]*@)?)([^/?#:]*)(.*)$/i.exec(url);
  if (!match) {
    return { before: url, host: '', after: '' };
  }
  return { before: match[1] ?? '', host: match[2] ?? '', after: match[3] ?? '' };
}
