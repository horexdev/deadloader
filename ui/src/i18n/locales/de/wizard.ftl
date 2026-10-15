# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Fortschritt der Einrichtung
wizard-caption-setup = Ersteinrichtung
wizard-caption-changed = Aktualisierter Hinweis
wizard-caption-catalog = Katalog einschalten
wizard-caption-review = Hinweis beim ersten Start
wizard-language = Sprache
wizard-step-welcome = Willkommen
wizard-step-risks = Risiken
wizard-step-network = Netzwerk
wizard-step-privacy = Datenschutz
wizard-step-updates = Updates
wizard-step-setup = Einrichtung
wizard-step-of = Schritt { $n } von 6
wizard-changed-of =
    { $count ->
        [one] 1 geänderter Schritt
       *[other] { $count } geänderte Schritte
    }
wizard-meta-changed = Geändert
wizard-meta-kept = Unverändert
wizard-back = Zurück
wizard-next = Weiter
wizard-finish = Einrichtung abschließen
wizard-continue = Weiter
wizard-close = Schließen
wizard-cancel = Abbrechen
wizard-to-catalog = Danach öffnet sich der Katalog.
wizard-to-library = Danach öffnet sich die Bibliothek (lokaler Modus).

# Step 1
wizard-s1-title = Willkommen bei deadloader
wizard-s1-lead = deadloader installiert Deadlock-Mods von GameBanana oder von deiner Festplatte und zeigt dir jede Änderung, die es vornimmt.
wizard-f1a = Es legt Mod-Dateien in den Mod-Ordner des Spiels und fügt eine Zeile in
wizard-f1b = ein – eine Spieldatei, die Deadlock sagt, dass Mods geladen werden.
wizard-f2 = Alles lässt sich rückgängig machen: Das Zurücksetzen entfernt alles, was deadloader ins Spiel gebracht hat.
wizard-f3a = Alle Daten bleiben im Ordner
wizard-f3b = neben
wizard-f3c = .
wizard-f4 = deadloader steht in keiner Verbindung zu Valve oder GameBanana.
wizard-f5 = Freie Software unter der GPL-3, ohne jegliche Gewährleistung.
wizard-licence = Lizenz (GPL-3)
wizard-source = Quellcode

# Step 2
wizard-s2-title = Mods auf eigenes Risiko
wizard-r1 = Valve hat keine offiziellen Regeln für Mods in Deadlock. Ob du Mods nutzt, entscheidest du selbst – auf eigenes Risiko.
wizard-r2 = Spielupdates können Mods unbrauchbar machen. deadloader richtet sich nach einem Update wieder ein, aber eine Mod funktioniert vielleicht erst wieder, wenn ihr Autor sie aktualisiert.
wizard-r3 = Mods stammen von anderen Menschen und stehen unter deren eigenen Lizenzen. deadloader kann nicht für ihren Inhalt bürgen; zu jeder Datei zeigt es das Virenscan-Ergebnis von GameBanana.
wizard-agree = Ich verstehe, dass deadloader Spieldateien ändert und dass ich Mods auf eigenes Risiko nutze.
wizard-agree-note = Damit bestätigst du die Schritte 1 und 2. Ohne dieses Häkchen geht es nicht weiter.
wizard-agree-error = Setze dieses Häkchen, um fortzufahren.
wizard-agree-reason = Setze zuerst das Häkchen.

# Step 3
wizard-s3-title = Netzwerk und GameBanana
wizard-s3-lead = Der Katalog spricht direkt mit der inoffiziellen Schnittstelle von GameBanana – und mit niemandem sonst. Nur diese Hosts werden kontaktiert:
wizard-hosts = Hosts
wizard-host-api = Katalog und Mod-Seiten
wizard-host-images = Vorschaubilder und Galerie
wizard-host-files = Downloads
wizard-host-cache = Downloads (files.gamebanana.com leitet dorthin weiter)
wizard-host-new = Neu
wizard-s3-images = Bilder anderer Hosts in Mod-Beschreibungen lädt deadloader erst, wenn du den jeweiligen Host erlaubst. Jede Anfrage steht in der
wizard-s3-images-end = .
wizard-activity = Aktivität
wizard-s3-tos = Wer den Katalog nutzt, nutzt GameBanana: Es gelten deren Nutzungsbedingungen, und GameBanana setzt ein Mindestalter von 13 Jahren voraus.
wizard-tos = Nutzungsbedingungen von GameBanana
wizard-choice = Wie möchtest du starten?
wizard-use-catalog = Katalog nutzen
wizard-use-catalog-text = Mods von GameBanana durchsuchen und installieren. deadloader verbindet sich nur mit den Hosts oben.
wizard-local = Jetzt nicht – lokaler Modus
wizard-local-text = Keinerlei Netzwerkanfragen; Mods nur von der Festplatte hinzufügen. Den Katalog kannst du später in den Einstellungen einschalten.
wizard-choice-error = Wähle eine der beiden Optionen.
wizard-choice-reason = Wähle zuerst eine Option.
wizard-last-time = Deine bisherige Wahl
wizard-changed-title = Was sich geändert hat ({ $date })
wizard-changed-foot = Die übrigen Schritte hast du am { $date } bestätigt; sie haben sich nicht geändert.

# Step 4
wizard-s4-title = Deine Privatsphäre
wizard-p1 = Kein Konto und keine Telemetrie. deadloader sendet nichts außer den GameBanana-Anfragen aus dem vorigen Schritt.
wizard-p1-local = Kein Konto und keine Telemetrie. Du hast den lokalen Modus gewählt: deadloader sendet gar nichts.
wizard-p2a = Protokolle bleiben auf diesem PC, im Ordner
wizard-p2b = – sie verbergen deinen Windows-Benutzernamen:
wizard-p3 = Das Fenster von deadloader zeichnet Microsoft WebView2, ein Teil von Windows. deadloader schaltet die eigenen Online-Funktionen von WebView2 ab; Windows aktualisiert WebView2 unabhängig von deadloader.
wizard-p4 = Was deadloader tut, kannst du jederzeit nachlesen in der
wizard-p4-end = .

# Step 5
wizard-s5-title = Updates von deadloader
wizard-u1 = deadloader kann beim Start auf GitHub nachsehen, ob es eine neuere Version von sich selbst gibt.
wizard-updates-label = Beim Start auf GitHub nach neuen deadloader-Versionen suchen
wizard-updates-on = An
wizard-updates-off = Aus (Standard)
wizard-updates-na = Nicht verfügbar im lokalen Modus
wizard-u2a = Wenn an, stellt deadloader bei jedem Start eine Anfrage an
wizard-u2b = und sagt dir, ob es eine neue Version gibt.
wizard-u3 = Aktualisieren geschieht immer von Hand: Du lädst die neue Version selbst von GitHub herunter. deadloader ersetzt sich nie selbst.
wizard-u4 = Nicht verfügbar: Im lokalen Modus stellt deadloader überhaupt keine Netzwerkanfragen. Die Option wird verfügbar, sobald du den Katalog in den Einstellungen einschaltest.
wizard-u5 = Du kannst das später unter Einstellungen → Allgemein ändern.

# Step 6
wizard-s6-title = Einrichtung
wizard-s6-lead = deadloader hat nach dem Spiel gesucht und einiges geprüft. Verändert wurde noch nichts; alles hier kannst du später in den Einstellungen ändern.
wizard-checking = Wird geprüft…
wizard-topic-game = Spiel
wizard-topic-exe = Speicherort von deadloader
wizard-topic-disk = Speicherplatz
wizard-topic-foreign = Mod-Ordner des Spiels
wizard-topic-gameinfo = Spieldatei für Mods
wizard-game-found = Deadlock gefunden
wizard-game-chosen = Ordner gewählt
wizard-change = Ändern…
wizard-game-missing = Deadlock wurde nicht gefunden
wizard-game-missing-text = Steam oder das Spiel ist nicht installiert, oder der Spielordner wurde verschoben. Wähle den Spielordner selbst aus – oder später unter Einstellungen → Spiel.
wizard-browse = Durchsuchen…
wizard-browse-title = Deadlock-Ordner wählen
wizard-help = Hilfe
wizard-help-a = In Steam: Rechtsklick auf Deadlock → Verwalten → Lokale Dateien durchsuchen. Wähle diesen Ordner; er endet meist auf
wizard-help-b = .
wizard-exe-ok = An einem festen Ort
wizard-exe-downloads = deadloader liegt in deinem Download-Ordner
wizard-exe-desktop = deadloader liegt auf deinem Desktop
wizard-exe-warn-a = Die Daten liegen neben der exe – räumst du auf, sind Bibliothek und Profile weg. Schließe deadloader, verschiebe den ganzen Ordner an einen festen Ort, z. B.
wizard-exe-warn-b = , und starte es dort neu.
wizard-open-folder = Ordner öffnen
wizard-opened = Im Explorer geöffnet
wizard-linked = Mods werden verknüpft – kein zusätzlicher Speicherplatz
wizard-linked-text = deadloader und das Spiel liegen auf demselben Laufwerk ({ $drive }).
wizard-copied = Mods werden kopiert, nicht verknüpft
wizard-copied-text = deadloader liegt auf { $data }, das Spiel auf { $game }. Jede aktivierte Mod belegt dann ihren Platz ein zweites Mal; die Bibliothek zeigt das als „kopiert – +N MB“.
wizard-learn-more = Mehr erfahren
wizard-drives-more = Eine Verknüpfung lässt das Spiel die Datei aus der Bibliothek nutzen, ohne zweite Kopie. Windows kann nur Dateien auf demselben Laufwerk verknüpfen. Liegt deadloader ebenfalls auf { $game }, wird nichts kopiert.
wizard-foreign-none = Keine fremden Mod-Dateien
wizard-foreign-none-text = Im Mod-Ordner liegt nichts, was deadloader nicht kennt.
wizard-foreign-some =
    { $count ->
        [one] 1 Datei aus einer anderen Quelle
       *[other] { $count } Dateien aus einer anderen Quelle
    }
wizard-foreign-some-text = Sie stammen nicht von deadloader – vermutlich von einem anderen Mod-Manager oder von Hand. deadloader lässt sie unverändert; sieh sie dir später in der Bibliothek an.
wizard-show-files = Dateien zeigen
wizard-hide-files = Dateien ausblenden
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Wird eingerichtet
wizard-gi-setup-a = deadloader fügt beim ersten Anwenden von Mods eine Zeile in
wizard-gi-setup-b = ein. Jetzt wird nichts verändert.
wizard-gi-ours = Bereits eingerichtet
wizard-gi-ours-text = Die Zeile von deadloader ist schon vorhanden.
wizard-gi-foreign = Von einem anderen Programm geändert
wizard-gi-foreign-text = Die Datei sieht nicht wie das Original aus – oft war das Deadlock Mod Manager. deadloader überschreibt keine fremden Änderungen und erklärt dir nach der Einrichtung, was zu tun ist.
wizard-what-is = Was ist das?
wizard-gi-more = ist eine Spieldatei, die Deadlock sagt, welche Ordner geladen werden. Meist hilft es, die Spieldateien in Steam überprüfen zu lassen; danach fügt deadloader seine Zeile ein.

# E1
wizard-nw-title = deadloader kann seine Daten nicht speichern.
wizard-nw-why-a = Der Ordner neben
wizard-nw-why-b = ist schreibgeschützt: Windows erlaubt Programmen dort nicht zu schreiben (zum Beispiel unter „Programme“).
wizard-nw-path = Ordner von deadloader
wizard-nw-do = Was du tun kannst
wizard-nw-s1 = Beende deadloader.
wizard-nw-s2a = Verschiebe den ganzen Ordner an einen Ort, an dem du schreiben darfst, z. B.
wizard-nw-s2b = . Nicht unter „Programme“.
wizard-nw-s3a = Starte
wizard-nw-s3b = am neuen Ort.
wizard-nw-why-folder = deadloader hält alles neben der exe, damit nichts über deinen PC verstreut wird: Wer den Ordner löscht, entfernt deadloader vollständig.
wizard-nw-details = Details
wizard-nw-copy = Details kopieren
wizard-nw-copied = Kopiert
wizard-nw-no-log = Nicht im Protokoll gespeichert – deadloader kann dort nicht schreiben.
wizard-exit = Beenden
