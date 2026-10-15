# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Schließen
library-apply = Anwenden
library-apply-profile = „{ $name }“ anwenden
library-discard-all = Alle verwerfen
library-keep = „{ $name }“ behalten
library-keep-applied = Übernommene Änderungen behalten
library-undo-this = Diese Änderung rückgängig machen
library-undo-all = Alle rückgängig
library-close-to-apply = Deadlock schließen — Änderungen werden dann automatisch übernommen
library-play-applies = Spielen wendet es auch an
library-applied-title = Änderungen übernommen
library-nothing-pending = Nichts ausstehend
library-all-in-game = Alles aus „{ $name }“ ist bereits im Spiel.
library-running-sub = Deadlock läuft, deshalb sperrt Windows seine Dateien. Diese Änderungen werden automatisch übernommen, sobald du das Spiel schließt.
library-waiting-sub = Diese Änderungen sind noch nicht im Spielordner. Wende sie jetzt an oder drücke Spielen.
library-switch-sub = Ein Profilwechsel ändert viele Dateien auf einmal, deshalb wartet er auf dich. Wende ihn hier an oder drücke Spielen — deadloader wendet ihn vor dem Start an.
library-applied-sub =
    { $count ->
        [one] Deadlock wurde um { $time } geschlossen, deshalb wurde die Änderung übernommen, die du währenddessen gemacht hast. Rückgängig bleibt bis zu deiner nächsten Änderung möglich.
       *[other] Deadlock wurde um { $time } geschlossen, deshalb wurden die { $count } Änderungen übernommen, die du währenddessen gemacht hast. Rückgängig bleibt bis zu deiner nächsten Änderung möglich.
    }
library-undoing-sub =
    { $count ->
        [one] Du hast die um { $time } übernommene Änderung rückgängig gemacht, und Deadlock läuft wieder. Sie wird automatisch zurückgesetzt, sobald du das Spiel schließt.
       *[other] Du hast die { $count } um { $time } übernommenen Änderungen rückgängig gemacht, und Deadlock läuft wieder. Sie werden automatisch zurückgesetzt, sobald du das Spiel schließt.
    }
library-group-enable = Aktivieren · { $count }
library-group-disable = Deaktivieren · { $count }
library-group-order = Reihenfolge · { $count }
library-group-variant = Variante · { $count }
library-group-version = Version · { $count }
library-group-switch = Profilwechsel · { $count }
library-group-undo =
    { $count ->
        [one] Rückgängig: die um { $time } übernommene Änderung
       *[other] Rückgängig: { $count } um { $time } übernommene Änderungen
    }
library-change-on = Aus → an
library-change-off = An → aus
library-change-above = Über { $name } verschoben
library-change-above-wins = Über { $name } verschoben — gewinnt jetzt den Konflikt
library-change-below = Unter { $name } verschoben
library-switch-what-happens =
    { $on ->
        [one] 1 Mod geht an
       *[other] { $on } Mods gehen an
    } · { $off ->
        [one] 1 geht aus
       *[other] { $off } gehen aus
    }
library-what-happens = Was im Spielordner passiert
library-folder-summary =
    { $added ->
        [one] 1 Datei hinzugefügt
       *[other] { $added } Dateien hinzugefügt
    } · { $removed } entfernt · { $renumbered } umnummeriert

## Foreign files
library-foreign-title = Im Mod-Ordner des Spiels gefunden
library-foreign-note = deadloader hat diese Dateien nicht angelegt und rührt sie nur an, wenn du es willst.
library-collapse = Einklappen
library-expand = Ausklappen
library-take = Zur Verwaltung übernehmen
library-leave = So lassen
library-left = So gelassen
library-foreign-open = Ordner mit { $name } öffnen
library-foreign-vpk =
    { $count ->
        [one] VPK, { $count } Datei
       *[other] VPK, { $count } Dateien
    }
library-foreign-vpk-hero =
    { $count ->
        [one] VPK, { $count } Datei — sieht aus wie ein Skin für { $hero }
       *[other] VPK, { $count } Dateien — sieht aus wie ein Skin für { $hero }
    }
library-foreign-unreadable = Eine Mod-Datei, die deadloader nicht lesen kann
library-foreign-replaced = Eine Datei, die eine von deadloader abgelegte ersetzt hat
library-foreign-dmm = Von Deadlock Mod Manager erstellt — eine Einstellungsdatei, kein Mod
library-foreign-other = Eine Datei, kein Mod
library-foreign-folder =
    { $count ->
        [one] Ordner, { $count } Datei — kein Mod
       *[other] Ordner, { $count } Dateien — kein Mod
    }
library-foreign-meta = { $size } · hinzugefügt am { $date }
library-taken = { $name } ist jetzt in deiner Bibliothek.

## D11
library-d11-title = { $name } zur Verwaltung übernehmen?
library-d11-lead = deadloader wird:
library-d11-step-copy = die Datei als lokalen Mod in deine Bibliothek kopieren ({ $size }),
library-d11-step-replace = die Datei im Spielordner durch eine eigene verknüpfte Kopie ersetzen,
library-d11-step-enable = sie in „{ $profile }“ am Ende der Liste aktivieren.
library-d11-name = Name in deiner Bibliothek
library-d11-hint = Die Bibliothek behält eine exakte Kopie der Originaldatei.
library-d11-suggested = { $file } ({ $hero }-Skin)

## D10
library-d10-title = Eine Spieldatei wurde von einem anderen Programm geändert
library-d10-what = Ein anderes Programm hat eine Spieldatei geändert, die festlegt, welche Ordner Deadlock lädt — oft Deadlock Mod Manager. deadloader überschreibt keine fremden Änderungen, deshalb werden deine Mods erst geladen, wenn das behoben ist.
library-d10-do = Lass Steam die Originaldatei wiederherstellen (Spieldateien überprüfen). Danach ergänzt deadloader seine eigene Zeile und gibt dir Bescheid.
library-d10-diff = Unterschiede anzeigen
library-d10-expected = Erwartet (Original + Zeile von deadloader)
library-d10-found = Jetzt gefunden
library-d10-diff-note = Markiert: die Zeile, die deadloader ergänzt, und die Zeilen eines anderen Programms.
library-d10-help = Was ist diese Datei?
library-d10-help-text = gameinfo.gi sagt Deadlock, welche Ordner geladen werden. Mod-Manager ergänzen eine Zeile, damit das Spiel Mods aus dem Ordner addons lädt. Spielupdates setzen sie zurück; deadloader behebt das automatisch — aber nur, wenn die Datei im Originalzustand ist.
library-d10-wait-title = Steam überprüft die Dateien von Deadlock.
library-d10-wait-text = Wenn Steam fertig ist, wechsle zurück zu deadloader — die Datei wird automatisch erneut geprüft. Du kannst auch jetzt prüfen.
library-d10-fixed = gameinfo.gi ist eingerichtet — Mods werden wieder geladen.
library-d10-fixed-text = Steam hat die Originaldatei wiederhergestellt und deadloader hat seine Zeile ergänzt. Details unter Aktivität → Dateien.
library-open-file = Datei öffnen
library-not-now = Nicht jetzt
library-verify-steam = Spieldateien in Steam überprüfen
library-check-again = Erneut prüfen
library-show-differences = Unterschiede anzeigen

## D5
library-update = Aktualisieren
library-update-on-gb = Eine neuere Version ist auf GameBanana.
library-later = Später
library-d5-title = { $name } aktualisieren?
library-d5-installed = Installiert
library-d5-new = Neu auf GameBanana
library-d5-downloaded = Heruntergeladen
library-d5-arrived = In der Bibliothek, noch nicht aktiv
library-d5-new-meta = Aktualisiert am { $date } · { $size } Download
library-d5-download = { $size } Download
library-d5-changed = Die neue Version hat andere Dateien. deadloader ordnet sie so zu:
library-d5-same = Diese Dateien werden durch ihre neuen Versionen ersetzt:
library-d5-map-text = deadloader konnte nicht für jede Datei erkennen, welche neue sie ersetzt. Wähle für jede die neue Datei oder behalte die alte.
library-d5-map-label = Dateizuordnung
library-d5-col-old = Installierte Datei
library-d5-col-new = Neue Datei
library-d5-pick = Neue Datei für { $file }
library-d5-choose = Wählen…
library-d5-keep = Alte Datei behalten
library-d5-tag-replaces = Ersetzt
library-d5-tag-same = Gleicher Name
library-d5-tag-new = Neu · auf der Mod-Seite
library-d5-profiles = { $new } wird in jedem Profil aktiv, das { $old } nutzt. { $old } bleibt in der Bibliothek, du kannst in den Mod-Details zurückwechseln.
library-d5-install = Update installieren
library-d5-confirm-map = Diese Dateien verwenden
library-update-downloading = Das Update von { $name } wird heruntergeladen — es wird installiert, sobald es da ist.
library-updated = { $name } aktualisiert.
library-updated-text = Aktiv in jedem Profil, das die alte Version genutzt hat. Die alte Version bleibt erhalten — du kannst in den Mod-Details zurückwechseln.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name } an
library-n12-off = { $name } aus
library-n12-above = { $name } über { $other } verschoben
library-n12-below = { $name } unter { $other } verschoben
library-n12-switched = { $name } auf { $to }
library-n12-files = { $name }: Dateien geändert

## S9: the file list of a pending apply
library-show-file-list = Dateiliste anzeigen
library-hide-file-list = Dateiliste ausblenden
library-file-list = Dateien im Spielordner
library-files-added = Hinzugefügt · { $count }
library-files-removed = Entfernt · { $count }
library-files-renumbered = Umnummeriert · { $count }
