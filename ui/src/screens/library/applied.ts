// N12's text: the changes that went into the game folder when Deadlock was closed, each in a few words ("Formal Wraith
// on, Yamato remodel moved above Yamato redesign, and 2 more"). The core sends them as data (`PendingChange`).

import type { ModView, PendingChange } from '../../lib/api';
import { formatList, t } from '../../i18n';
import { fileLabel, versionText } from './model';

/** One change in a few words. */
export function changePhrase(change: PendingChange, mods: readonly ModView[]): string {
  const mod = mods.find((m) => m.id === change.mod_id);
  const name = change.name;
  const label = (id: string) => (mod ? fileLabel(mod, id) : id);
  const modName = (id: string | null) => (id ? (mods.find((m) => m.id === id)?.name ?? id) : '');
  switch (change.kind) {
    case 'enable':
      return t('library-n12-on', { name });
    case 'disable':
      return t('library-n12-off', { name });
    case 'order':
      return change.above
        ? t('library-n12-above', { name, other: modName(change.above) })
        : t('library-n12-below', { name, other: modName(change.below) });
    case 'variant':
      return change.to.length > 0
        ? t('library-n12-switched', { name, to: change.to.map(label).join(', ') })
        : t('library-n12-files', { name });
    case 'version': {
      const version = versionText(mod?.files.find((f) => f.id === change.to)?.version) ?? label(change.to);
      return t('library-n12-switched', { name, to: version });
    }
  }
}

/** The changes as one line; past a few, the rest are counted ("and 2 more"). Empty when there are none. */
export function appliedSummary(items: readonly PendingChange[], mods: readonly ModView[]): string {
  return items.length > 0 ? formatList(items.map((c) => changePhrase(c, mods))) : '';
}
