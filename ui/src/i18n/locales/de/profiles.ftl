# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Profile
profiles-count =
    { $count ->
        [one] { $count } Profil
       *[other] { $count } Profile
    }
profiles-in-game = Gerade im Spiel: { $list }
profiles-in-game-item = „{ $name }“ auf { $drive }
profiles-in-game-none = Gerade ist kein Profil im Spiel.
profiles-import = Importieren…
profiles-import-title = Profil aus einer .json-Datei importieren
profiles-import-dialog = Profil importieren
profiles-export-dialog = Profil exportieren
profiles-file-filter = deadloader-Profile
profiles-new = Neues Profil
profiles-selected = Ausgewählt
profiles-applied = Angewendet
profiles-not-applied = Noch nicht angewendet
profiles-selected-title = Das Profil, das du in der Bibliothek bearbeitest und spielst
profiles-pending-title = Anwenden oder Spielen bringt es in den Spielordner
profiles-applied-title = Seine Dateien sind gerade im Spielordner auf { $drive }
profiles-select = Auswählen
profiles-select-of = „{ $name }“ auswählen
profiles-more-of = Weitere Aktionen für „{ $name }“
profiles-rename = Umbenennen
profiles-duplicate = Duplizieren
profiles-export = Als Datei exportieren…
profiles-change-installation = Spielinstallation ändern…
profiles-delete = Löschen
profiles-delete-confirm = Löschen…
profiles-delete-last = deadloader braucht mindestens ein Profil.
profiles-name-label = Profilname
profiles-save = Speichern
profiles-cancel = Abbrechen
profiles-close = Schließen
profiles-close-esc = Schließen (Esc)
profiles-rename-hint = Eingabe speichert · Esc bricht ab
profiles-name-empty = Gib einen Namen für das Profil ein.
profiles-mods =
    { $count ->
        [one] { $count } Mod aktiviert
       *[other] { $count } Mods aktiviert
    }
profiles-mods-none = Keine Mods aktiviert
profiles-applied-when = { $when } angewendet
profiles-last-applied = Zuletzt { $when } angewendet
profiles-never-applied = Noch nie angewendet
profiles-just-now = gerade eben
profiles-preview-empty = Wähle das Profil aus und aktiviere dann Mods in der Bibliothek.
profiles-footnote = Ein Profil ist eine Liste aktivierter Mods mit Reihenfolge, Varianten und Versionen. Auswählen lädt nichts herunter und löscht nichts; Anwenden ändert nur die Dateien, die sich unterscheiden.
profiles-copy-name = { $name } (Kopie)
profiles-copy-name-n = { $name } (Kopie { $number })
profiles-new-title = Neues Profil
profiles-start-with = Beginnen mit
profiles-empty = Leer
profiles-empty-hint = Keine Mods aktiviert — wähle sie danach in der Bibliothek aus.
profiles-copy-of = Kopie von „{ $name }“
profiles-copy-hint = { $mods } — gleiche Reihenfolge, Varianten und Versionen.
profiles-installation = Spielinstallation
profiles-installation-hint = Wie bei „{ $name }“. Du kannst sie später im Menü des Profils ändern.
profiles-new-note = Ein neues Profil wird weder ausgewählt noch angewendet — am Spiel ändert sich nichts.
profiles-create = Profil erstellen
profiles-pending-what = Profil „{ $name }“ ist noch nicht im Spiel.
profiles-pending-why-other = Im Spielordner auf { $drive } ist noch „{ $other }“. Wende es jetzt an oder drücke Spielen — deadloader wendet es vor dem Start an.
profiles-pending-why-empty = Im Spielordner auf { $drive } sind gerade keine Mods von deadloader. Wende es jetzt an oder drücke Spielen — deadloader wendet es vor dem Start an.
profiles-keep = „{ $name }“ behalten
profiles-apply = „{ $name }“ anwenden
profiles-other-what = „{ $name }“ verwendet eine andere Spielinstallation.
profiles-other-why = Anwenden und Spielen nutzen { $path }. Das Spiel auf { $drive } behält „{ $other }“.
profiles-other-why-empty = Anwenden und Spielen nutzen { $path }. Das Spiel auf { $drive } bleibt unverändert.
profiles-delete-title = Profil „{ $name }“ löschen?
profiles-delete-applied =
    { $count ->
        [one] „{ $name }“ ist gerade im Spiel. deadloader entfernt seinen { $count } Mod aus dem Spielordner:
       *[other] „{ $name }“ ist gerade im Spiel. deadloader entfernt seine { $count } Mods aus dem Spielordner:
    }
profiles-delete-applied-empty = „{ $name }“ ist gerade im Spiel, hat aber keine Mods — der Spielordner ändert sich nicht.
profiles-delete-stay = Die Mods bleiben in deiner Bibliothek und in anderen Profilen. Bis du dort ein anderes Profil anwendest, startet Deadlock ohne Mods von deadloader.
profiles-delete-next = „{ $name }“ wird das ausgewählte Profil. Es wird erst angewendet, wenn du Anwenden oder Spielen drückst.
profiles-delete-undoable = Das lässt sich nicht rückgängig machen. Exportiere das Profil vorher, wenn du eine Kopie behalten willst.
profiles-export-first = Zuerst exportieren
profiles-delete-profile = Profil löschen
profiles-installation-title = Spielinstallation für „{ $name }“
profiles-installation-lead = Wähle den Deadlock-Ordner, auf den dieses Profil angewendet wird.
profiles-used-by =
    { $count ->
        [one] von { $count } Profil verwendet
       *[other] von { $count } Profilen verwendet
    }
profiles-used-by-none = von keinem Profil verwendet
profiles-installation-applied = „{ $name }“ ist gerade auf { $drive } im Spiel. Seine Dateien bleiben dort, bis du auf { $drive } ein anderes Profil anwendest.
profiles-installation-not-applied = Keiner der Spielordner ändert sich, bis du „{ $name }“ anwendest.
profiles-manage-installations = Installationen in den Einstellungen verwalten
profiles-use-installation = Diese Installation verwenden
profiles-d9-title =
    { $count ->
        [one] { $count } Mod aus „{ $name }“ fehlt in deiner Bibliothek
       *[other] { $count } Mods aus „{ $name }“ fehlen in deiner Bibliothek
    }
profiles-d9-file = Die Datei
profiles-d9-lead = enthält { $listed } Mods. { $present } sind schon in deiner Bibliothek, diese { $count } nicht:
profiles-d9-list = Fehlende Mods
profiles-d9-from-gamebanana = { $count } von GameBanana
profiles-d9-local-count = { $count } lokal
profiles-gamebanana = GameBanana
profiles-can-download = Kann heruntergeladen werden
profiles-local-mod = Lokaler Mod
profiles-cant-download = Kann nicht heruntergeladen werden
profiles-local-why = Auf einem anderen PC von der Festplatte hinzugefügt, daher nicht auf GameBanana. Bitte die Person, die das Profil geteilt hat, um die Datei.
profiles-d9-local-mode = Der Katalog ist aus (lokaler Modus), daher kann deadloader keine Mods herunterladen. Importiere ohne sie und füge sie später von der Festplatte hinzu — oder schalte den Katalog in den Einstellungen ein.
profiles-d9-after = So oder so wird „{ $name }“ dein ausgewähltes Profil, aber nicht angewendet — der Spielordner ändert sich erst, wenn du Anwenden oder Spielen drückst.
profiles-import-without = Ohne sie importieren
profiles-download-missing = Fehlende herunterladen ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } Mod ausgeschaltet
       *[other] { $count } Mods ausgeschaltet
    }
profiles-turned-on =
    { $count ->
        [one] { $count } Mod eingeschaltet
       *[other] { $count } Mods eingeschaltet
    }
profiles-differ = Nur Dateien, die sich unterscheiden, wurden geändert.
profiles-same = Dieselben Mods waren schon im Spiel; nur Reihenfolge und Varianten wurden geprüft.
profiles-deleted = Profil „{ $name }“ gelöscht.
profiles-deleted-not-applied = Es war nicht angewendet, der Spielordner bleibt unverändert.
profiles-deleted-applied =
    { $count ->
        [one] { $count } Mod wurde aus dem Spielordner entfernt. Er ist weiterhin in deiner Bibliothek.
       *[other] { $count } Mods wurden aus dem Spielordner entfernt. Sie sind weiterhin in deiner Bibliothek.
    }
profiles-exported = Profil „{ $name }“ exportiert.
profiles-imported = Profil „{ $name }“ importiert.
profiles-imported-text = Es ist ausgewählt, aber noch nicht angewendet.
profiles-imported-without =
    { $count ->
        [one] Profil „{ $name }“ ohne { $count } Mod importiert.
       *[other] Profil „{ $name }“ ohne { $count } Mods importiert.
    }
profiles-imported-without-text = Es ist ausgewählt, aber noch nicht angewendet. Die fehlenden Mods kannst du später hinzufügen.
profiles-imported-downloading =
    { $count ->
        [one] Profil „{ $name }“ importiert — { $count } Mod wird heruntergeladen.
       *[other] Profil „{ $name }“ importiert — { $count } Mods werden heruntergeladen.
    }
profiles-imported-downloading-text = Es ist ausgewählt, aber noch nicht angewendet. Wende es an, wenn die Downloads fertig sind.
profiles-open-downloads = Downloads öffnen
profiles-undo = Rückgängig
profiles-check-update = Nach deadloader-Update suchen
profiles-choose-another = Andere Datei wählen
profiles-switcher = Profile verwalten
