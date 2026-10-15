# The Library (S4): header, states, list, mod details, conflicts (C1), deleting (D6).
library-title = Bibliothek
library-profile = Profil
library-enabled-count = { $enabled } von { $total } aktiviert
library-disk = { $size } auf dem Datenträger
library-not-checked = Noch nicht nach Updates gesucht
library-checked-today = Updates geprüft heute um { $time }
library-checked-on = Updates geprüft am { $date }
library-game-not-found = Spiel nicht gefunden
library-add-disk = Von Festplatte hinzufügen
library-add-title = Mods von der Festplatte hinzufügen
library-add-filter = Mod-Dateien
library-added =
    { $count ->
        [one] { $count } Mod zur Bibliothek hinzugefügt.
       *[other] { $count } Mods zur Bibliothek hinzugefügt.
    }
library-check = Nach Updates suchen
library-checking = Suche…
library-update-all = Alle aktualisieren
library-review = Ansehen
library-undo = Rückgängig
library-cancel = Abbrechen
library-missing-what = Deadlock wurde nicht gefunden.
library-missing-why = Steam oder das Spiel ist nicht installiert, oder der Spielordner wurde verschoben. Deine Bibliothek funktioniert weiter; Anwenden und Spielen sind aus, bis das Spiel gefunden ist.
library-choose-folder = Spielordner wählen
library-search-again = Erneut suchen
library-running-what = Deadlock läuft.
library-running-why = Änderungen werden automatisch übernommen, sobald du das Spiel schließt.
library-pending = Ausstehende Änderungen
library-switch-what = Profil „{ $name }“ ist noch nicht im Spiel.
library-switch-why = Im Spiel ist noch „{ $applied }“. Wende „{ $name }“ jetzt an oder drücke Spielen — deadloader wendet es vor dem Start an.
library-switch-why-first = Wende „{ $name }“ jetzt an oder drücke Spielen — deadloader wendet es vor dem Start an.
library-review-switch = Wechsel prüfen
library-applying-what = Profil „{ $name }“ wird angewendet…
library-applying-why = Du kannst weiter stöbern; Spielen ist danach wieder verfügbar.
library-show = Anzeigen
library-tab-all = Alle
library-tab-enabled = Aktiviert
library-tab-disabled = Deaktiviert
library-tab-updates = Updates
library-tab-conflicts = Konflikte
library-tab-local = Lokal
library-search = Bibliothek durchsuchen
library-search-clear = Suche leeren
library-higher-wins = Weiter oben in der Liste gewinnt, wenn zwei Mods dieselbe Datei ändern.
library-reorder-hint = Umsortieren: Griff ziehen oder fokussieren und Leertaste drücken
library-list-label = Mods im Profil „{ $name }“, nach Priorität
library-toggle = { $name } aktiviert
library-toggle-missing = Deadlock wurde nicht gefunden — wähle den Spielordner, um Änderungen anzuwenden
library-grip = { $name } verschieben, Position { $position } von { $total }
library-more = Weitere Aktionen für { $name }
library-show-variants = Varianten anzeigen
library-hide-variants = Varianten ausblenden
library-show-addons = Add-ons anzeigen
library-hide-addons = Add-ons ausblenden
library-live-picked = { $name } — aufgenommen, Position { $position } von { $total }. ↑ ↓ verschieben, Leertaste ablegen, Esc abbrechen.
library-live-moved = { $name } — Position { $position } von { $total }.
library-live-moved-grabbed = { $name } — Position { $position } von { $total }. Leertaste ablegen, Esc abbrechen.
library-live-dropped = { $name } — abgelegt auf Position { $position } von { $total }.
library-live-back = { $name } — zurück auf Position { $position } von { $total }.
library-live-drag = { $name } — loslassen, um über { $above } abzulegen, auf Position { $position } von { $total }.
library-badge-pending = Ausstehend
library-badge-damaged = Dateien beschädigt
library-badge-update = Update verfügbar
library-badge-wins = Konflikt · gewinnt
library-badge-loses = Konflikt · verliert
library-badge-variants =
    { $count ->
        [one] { $count } Variante
       *[other] { $count } Varianten
    }
library-badge-local = Lokal
library-badge-copied = Kopiert · +{ $size }
library-local = Lokal
library-unknown-author = Unbekannter Autor
library-gamebanana = GameBanana
library-from-disk = von der Festplatte
library-addons-sub =
    { $count ->
        [one] + { $count } Add-on
       *[other] + { $count } Add-ons
    }
library-variants = Varianten
library-variants-downloaded = { $count } heruntergeladen
library-active = Aktiv
library-select-to-switch = Zum Wechseln auswählen
library-parts = Dateien dieses Mods
library-main-files = Hauptdateien
library-from = aus { $archive }
library-on-with-mod = An mit dem Mod · { $size }
library-remove-addon = Add-on entfernen
library-toast-enabled = { $name } aktiviert.
library-toast-disabled = { $name } deaktiviert.
library-toast-variant = { $name } auf { $variant } umgestellt.
library-empty-title = Deine Bibliothek ist leer
library-empty-text = Installiere Mods aus dem Katalog oder füge .vpk-, .zip-, .7z- oder .rar-Dateien von deinem Computer hinzu. Du kannst Dateien auch irgendwo auf dieses Fenster ziehen.
library-empty-local-text = Im lokalen Modus ist der Katalog aus. Füge .vpk-, .zip-, .7z- oder .rar-Dateien von deinem Computer hinzu oder zieh sie irgendwo auf dieses Fenster.
library-browse = Katalog durchsuchen
library-no-match-title = Keine Mods hier
library-no-match-text = Kein Mod passt zu diesem Tab und dieser Suche. Wähle einen anderen Tab oder leere die Suche.

## Mod details
library-details = Mod-Details
library-open-mod-page = Mod-Seite öffnen
library-close-details = Details schließen
library-enabled-in = Aktiviert in „{ $profile }“
library-position = Position { $position } von { $total }
library-version = Version
library-no-version = Keine Version angegeben
library-from-gamebanana = Von GameBanana
library-imported = Von der Festplatte importiert
library-newer-in-library = Eine neuere Version ist in der Bibliothek.
library-switch-to = Wechseln zu
library-delete-old = Alte Versionen löschen
library-no-older = Keine älteren Versionen gespeichert
library-older =
    { $count ->
        [one] { $count } ältere Version gespeichert · { $size }
       *[other] { $count } ältere Versionen gespeichert · { $size }
    }
library-files = Dateien
library-on-disk = { $size } auf dem Datenträger
library-linked = Verknüpft – kein zusätzlicher Speicherplatz
library-copied = Kopiert – { $size } zusätzlicher Speicherplatz
library-not-in-game = Nicht im Spielordner
library-local-mod = Lokaler Mod
library-name = Name in deiner Bibliothek
library-rename = Umbenennen
library-link-label = GameBanana-Seite
library-link-hint = Füge die Adresse des Mods ein, damit er auf Updates geprüft wird.
library-link = Verknüpfen
library-link-done = Verknüpft: { $matched } von { $total } Dateien auf der Seite gefunden.
library-link-not-a-page = Das ist keine Mod-Seite von GameBanana. Füge eine Adresse wie gamebanana.com/mods/123456 ein.
library-link-removed = Dieser Mod ist nicht mehr auf GameBanana.
library-link-failed = Die Seite konnte nicht gelesen werden. Prüfe die Adresse und versuch es noch einmal.
library-imported-from = Importiert aus
library-no-conflicts = Keine Konflikte mit anderen aktivierten Mods
library-conflict-wins =
    { $count ->
        [one] Gewinnt gegen { $name } · { $count } Datei
       *[other] Gewinnt gegen { $name } · { $count } Dateien
    }
library-conflict-loses =
    { $count ->
        [one] Verliert gegen { $name } · { $count } Datei
       *[other] Verliert gegen { $name } · { $count } Dateien
    }
library-show-conflicts = Konflikte anzeigen
library-delete-mod = Mod löschen…
library-open-folder = Ordner öffnen

## Conflicts (C1)
library-conflicts-sub = Zwei aktivierte Mods ändern dieselben Spieldateien. Der weiter oben gewinnt.
library-close-conflicts = Konflikte schließen
library-no-conflicts-title = Keine Konflikte
library-wins = Gewinnt
library-loses = Verliert
library-off = Aus
library-in-use = Wird verwendet
library-conflict-text =
    { $count ->
        [one] Beide ändern { $count } Datei. Im Spiel siehst du die Version von { $winner }; die Kopie von { $loser } wird nicht verwendet.
       *[other] Beide ändern { $count } Dateien. Im Spiel siehst du die Versionen von { $winner }; die Kopien von { $loser } werden nicht verwendet.
    }
library-conflict-off =
    { $count ->
        [one] Beide ändern { $count } Datei. { $off } ist aus, deshalb verwendet das Spiel die Version von { $other }.
       *[other] Beide ändern { $count } Dateien. { $off } ist aus, deshalb verwendet das Spiel die Versionen von { $other }.
    }
library-files-count =
    { $count ->
        [one] { $count } Datei
       *[other] { $count } Dateien
    }
library-show-more = { $count } weitere anzeigen
library-move-above = { $name } nach oben verschieben
library-disable = { $name } deaktivieren
library-pick-variant = Andere Variante wählen
library-no-variants = Keiner der Mods hat Varianten
library-done-wins = Erledigt — { $name } gewinnt jetzt
library-done-off = Erledigt — { $off } ist aus; im Spiel werden die Dateien von { $other } verwendet
library-identical = Identische Dateien (gleiche Prüfsumme) sind keine Konflikte und werden nicht aufgeführt.

## D6 Delete mod
library-d6-title = „{ $name }“ löschen?
library-d6-files-title = Alte Versionen von „{ $name }“ löschen?
library-d6-what =
    { $count ->
        [one] { $count } Datei · { $size }.
       *[other] { $count } Dateien · { $size }.
    }
library-d6-profiles =
    { $count ->
        [one] Verwendet in { $count } Profil: { $names } — er wird daraus und aus dem Spielordner entfernt.
       *[other] Verwendet in { $count } Profilen: { $names } — er wird daraus und aus dem Spielordner entfernt.
    }
library-d6-no-profiles = Kein Profil verwendet ihn.
library-d6-where = Löschen
library-d6-permanently = Endgültig
library-d6-recycle-bin = In den Papierkorb
library-d6-permanent-hint = Die Dateien werden endgültig gelöscht — das lässt sich nicht rückgängig machen.
library-d6-bin-hint = Du kannst ihn aus dem Windows-Papierkorb wiederherstellen.
library-d6-confirm = Mod löschen
library-d6-confirm-files = Dateien löschen
library-deleted = { $name } gelöscht.
library-deleted-files =
    { $count ->
        [one] { $count } Datei von { $name } gelöscht.
       *[other] { $count } Dateien von { $name } gelöscht.
    }
