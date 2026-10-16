# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Einstellungen
settings-nav = Bereiche der Einstellungen
settings-reset-nav = Alle Einstellungen zurücksetzen…
settings-default = Standard: { $value }
settings-restore = Standard wiederherstellen
settings-on = An
settings-off = Aus
settings-cancel = Abbrechen
settings-remove = Entfernen
settings-remove-item = Entfernen: { $item }
settings-add = Hinzufügen
settings-open-folder = Ordner öffnen
settings-open-folder-at = Ordner öffnen: { $path }
settings-copied = In die Zwischenablage kopiert.
settings-folder = Ordner
settings-example = Beispiel:

settings-section-general = Allgemein
settings-section-general-lead = Sprache, Start und Zoom.
settings-section-game = Spiel
settings-section-game-lead = Wo Deadlock installiert ist und was deadloader dort ändern darf.
settings-section-catalog = Katalog
settings-section-catalog-lead = Der GameBanana-Katalog, sensible Inhalte, Bilder und Links.
settings-section-library = Bibliothek
settings-section-library-lead = Mod-Updates, Löschen von Mods und alte Versionen.
settings-section-downloads = Downloads
settings-section-downloads-lead = Download-Warteschlange, Archive und Entpacken.
settings-section-network = Netzwerk
settings-section-network-lead = Wie deadloader mit GameBanana spricht.
settings-section-cache = Cache
settings-section-cache-lead = Auf diesem PC gespeicherte Vorschaubilder und Katalogseiten.
settings-section-logs = Protokolle
settings-section-logs-lead = Was die Aktivität aufbewahrt und wie lange.
settings-section-advanced = Erweitert
settings-section-advanced-lead = Einstellungsdateien und zusätzlich erlaubte Domains.
settings-section-about = Über deadloader
settings-section-about-lead = Version, Lizenz, Prüfung dieses Downloads und wo deine Daten liegen.

# General
settings-general-group = Sprache und Start
settings-language = Sprache
settings-language-help = Systemsprache: die Anzeigesprache von Windows, wenn deadloader sie hat, sonst Englisch.
settings-language-system = Systemsprache ({ $language })
settings-startup-profile = Profil beim Start
settings-startup-profile-help = Welches Profil beim Start ausgewählt ist. Auswählen wendet es noch nicht an.
settings-startup-last = Zuletzt ausgewählt ({ $name })
settings-startup-last-plain = Zuletzt ausgewählt
settings-app-updates = Beim Start nach deadloader-Updates suchen
settings-app-updates-help = deadloader fragt beim Start einmal bei GitHub (api.github.com) nach einer neueren Version. Aktualisiert wird immer von Hand. Du hast das beim ersten Start festgelegt.
settings-app-updates-local = Nicht verfügbar: Im lokalen Modus stellt deadloader überhaupt keine Netzwerkanfragen. Die Option wird verfügbar, sobald du den Katalog einschaltest (Einstellungen → Katalog).
settings-zoom = Zoom
settings-zoom-keys-1 = Auch mit Strg + Mausrad,
settings-zoom-keys-2 = und
settings-zoom-keys-3 = zum Zurücksetzen.
settings-ctrl = Strg
settings-percent = { $value } %

# Game
settings-installs-group = Spielinstallationen
settings-installs-lead = deadloader findet Steam-Installationen von Deadlock selbst. Jedes Profil verwendet eine davon — das änderst du unter Profile.
settings-install-library = Steam-Bibliothek { $library }
settings-install-build = Build { $build }
settings-install-used =
    { $count ->
        [0] von keinem Profil verwendet
        [one] von 1 Profil verwendet: { $names }
       *[other] von { $count } Profilen verwendet: { $names }
    }
settings-install-selected = { $name } (ausgewählt)
settings-install-applied = { $name } (dort angewendet)
settings-install-steam = Über Steam gefunden
settings-install-manual = Manuell hinzugefügt
settings-install-known = Dateien von deadloader liegen noch dort
settings-install-manual-meta = Manuell hinzugefügt
settings-gi-ours = gameinfo.gi: OK — die Zeilen von deadloader sind vorhanden
settings-gi-wait = gameinfo.gi: wird eingerichtet, sobald ein Profil sie nutzt
settings-gi-rolled = gameinfo.gi: Original — „Anwenden“ fügt die Zeilen von deadloader wieder ein
settings-gi-foreign = gameinfo.gi: von einem anderen Programm geändert
settings-gi-missing = gameinfo.gi: nicht gefunden — der Ordner wurde vielleicht verschoben
settings-installs-none = Noch keine Installation gefunden.
settings-add-manual = Manuell hinzufügen…
settings-add-manual-hint = Wähle einen Deadlock-Ordner, der nicht gefunden wurde.
settings-add-manual-title = Deadlock-Ordner wählen
settings-install-added = Installation hinzugefügt.
settings-install-removed = Installation aus der Liste entfernt.
settings-install-removed-text = In dem Ordner wurde nichts geändert.
settings-gameinfo-group = Nach Spiel-Updates
settings-autofix = gameinfo.gi automatisch reparieren
settings-autofix-help = Ein Deadlock-Update setzt die Spieldatei zurück, die das Laden von Mods einschaltet. deadloader sichert sie, setzt seine Zeilen wieder ein und sagt dir Bescheid. Wenn aus, fragt dich zuerst ein Hinweisbalken.
settings-danger-group = Rückgängig machen und entfernen
settings-rollback = Spielordner des ausgewählten Profils zurücksetzen
settings-rollback-help = Für die Installation des ausgewählten Profils { $profile } ({ $drive }): entfernt die { $files } Mod-Dateien, die deadloader dort abgelegt hat, und seine Zeilen in gameinfo.gi. Bibliothek und Profile bleiben; „Anwenden“ stellt alles wieder her.
settings-rollback-help-others = Andere Spielordner bleiben unberührt; wähle ein Profil, das sie nutzt, um sie zurückzusetzen.
settings-rollback-nothing = Auf { $drive } nichts zurückzusetzen — deadloader hat dort gerade keine Dateien.
settings-rollback-no-game = Deadlock wurde nicht gefunden, daher gibt es keinen Spielordner zum Zurücksetzen.
settings-rollback-button = Zurücksetzen…
settings-rolled-title = Spielordner auf { $drive } zurückgesetzt.
settings-rolled-text =
    { $foreign ->
        [0] Auf { $drive } wurden { $files } Mod-Dateien und die Zeilen von deadloader in gameinfo.gi entfernt. Bibliothek und Profile bleiben — „Anwenden“ oder „Spielen“ holt die Mods zurück.
       *[other] Auf { $drive } wurden { $files } Mod-Dateien, die Zeilen von deadloader in gameinfo.gi und { $foreign } fremde Dateien entfernt. Bibliothek und Profile bleiben — „Anwenden“ oder „Spielen“ holt die Mods zurück.
    }
settings-wipe = Alle deadloader-Daten löschen
settings-wipe-help = Löscht Bibliothek, Profile, Protokolle, Cache und Einstellungen ({ $size }) und schließt deadloader. deadloader.exe bleibt — die löschst du danach selbst.
settings-wipe-button = Alle Daten löschen…

# Catalog
settings-catalog-group = GameBanana-Katalog
settings-catalog = Katalog verwenden
settings-catalog-help-on = Mods von GameBanana durchsuchen und installieren. deadloader spricht nur mit GameBanana, jede Anfrage steht in der Aktivität. Aus bedeutet lokaler Modus.
settings-catalog-help-off = Aus — lokaler Modus: deadloader stellt überhaupt keine Netzwerkanfragen. Mods von der Festplatte kannst du weiter hinzufügen.
settings-consent-title = Vorher ein Schritt.
settings-consent-text = deadloader zeigt dir, welche Hosts es kontaktiert und welche Bedingungen von GameBanana gelten, bevor der Katalog angeht.
settings-consent-go = Ansehen und einschalten
settings-local-on = Lokaler Modus ist an.
settings-local-on-text = Der Katalog ist aus und deadloader stellt keine Netzwerkanfragen.
settings-ttl = Geladene Katalogseiten wiederverwenden für
settings-ttl-help = Innerhalb dieser Zeit fragt deadloader GameBanana nicht erneut, wenn du zu einer Seite zurückgehst.
settings-minutes = { $count } Min.
settings-hours = { $count } Std.
settings-prefetch = Nächste Seite vorab laden
settings-prefetch-help = Wenn du etwa 70 % der Ergebnisse gesehen hast, werden die nächsten geladen, damit die Liste weiterläuft.
settings-nsfw-group = Sensible Inhalte
settings-nsfw = Mods mit Inhaltseinstufung (NSFW)
settings-nsfw-help = Ausgeblendet: gar nicht angezeigt. Verschwommen: ein unscharfes Bild mit dem Hinweis „Sensibler Inhalt“; zum Anzeigen klicken. Angezeigt: ohne Unschärfe.
settings-nsfw-age-done = Alter bestätigt am { $date }.
settings-nsfw-age-ask = Beim Verlassen von „Ausgeblendet“ wirst du einmal nach deinem Alter gefragt.
settings-nsfw-hidden = Ausgeblendet
settings-nsfw-blurred = Verschwommen
settings-nsfw-shown = Angezeigt
settings-images-group = Bilder und Links
settings-description-images = Bilder in Mod-Beschreibungen
settings-description-images-help = Sie kommen von anderen Hosts als GameBanana, z. B. i.imgur.com. Bei Klick: ein Platzhalter, bis du ihn anklickst.
settings-images-click = Bei Klick
settings-images-always = Immer
settings-prechecked = „Bilder von diesem Host immer zulassen“ vorab ankreuzen
settings-prechecked-help = Im Dialog, der erscheint, wenn ein Beschreibungsbild zum ersten Mal von einem neuen Host kommt.
settings-hosts = Zugelassene Bild-Hosts
settings-hosts-help = Bilder von diesen Hosts laden ohne Nachfrage. In der Aktivität sind ihre Anfragen als „zugelassener Bild-Host“ markiert.
settings-hosts-built-in = Eingebaut
settings-hosts-none = Keine Hosts hinzugefügt — jeder neue Host fragt zuerst.
settings-host-removed = Entfernt: { $host }
settings-host-removed-text = Bilder von dort fragen vor dem Laden wieder nach.
settings-undo = Rückgängig
settings-gif = GIFs automatisch abspielen
settings-gif-help = Aus: GIFs zeigen ihr erstes Bild mit einer Wiedergabetaste.
settings-links = Externe Links
settings-links-help = Links öffnen sich in deinem Standardbrowser. „Erst fragen“ zeigt vorher die vollständige Adresse.
settings-links-ask = Erst fragen
settings-links-open = Sofort öffnen

# Library
settings-updates-group = Mod-Updates
settings-check-updates = Beim Start nach Mod-Updates suchen
settings-check-updates-help = Vergleicht deine Mods von GameBanana mit dem Katalog. Nie im lokalen Modus.
settings-auto-download = Updates automatisch herunterladen
settings-auto-download-help = Nur das Herunterladen — deadloader fragt trotzdem, bevor ein Mod zur neuen Version wechselt.
settings-disk-group = Löschen und alte Versionen
settings-deletion = Wenn du einen Mod löschst
settings-deletion-help = Im Löschdialog kannst du jedes Mal anders entscheiden.
settings-deletion-permanent = Endgültig
settings-deletion-bin = In den Papierkorb
settings-old-versions = Alte Versionen
settings-old-versions-help = Jede heruntergeladene Version bleibt, damit du zurückwechseln kannst. Alte Versionen löschst du in den Details eines Mods.
settings-old-keep-all = Alle behalten

# Downloads
settings-queue-group = Warteschlange
settings-parallel = Gleichzeitige Downloads
settings-parallel-help = Wie viele Dateien gleichzeitig heruntergeladen werden.
settings-speed = Geschwindigkeitslimit
settings-speed-help = Nur für Mod-Downloads — Katalog und Vorschaubilder sind nicht begrenzt.
settings-speed-none = Kein Limit
settings-speed-value = { $value } MB/s
settings-keep-archives = Archive nach der Installation behalten
settings-keep-archives-help = Aus: Die .zip-, .7z- oder .rar-Datei wird gelöscht, sobald ihre .vpk-Dateien in der Bibliothek sind.
settings-unpack-group = Entpacken
settings-limits = Grenzen beim Entpacken
settings-limits-help = Schützt vor Archiven, die riesig entpacken. Wenn ein Archiv, dem du vertraust, abgelehnt wird, erhöhe die Grenze.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = Entpackte Größe
settings-limit-ratio = Kompressionsrate
settings-limit-files = Mod-Dateien je Archiv
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = Dateien
settings-unit-s = s
settings-number-error = { $field }: Gib eine ganze Zahl von { $min } bis { $max } ein. Bis dahin gilt der vorherige Wert.

# Network
settings-network-local = Der lokale Modus ist an.
settings-network-local-text = deadloader stellt keine Netzwerkanfragen; diese Einstellungen gelten, sobald der Katalog an ist.
settings-requests-group = Anfragen an GameBanana
settings-api-parallel = Gleichzeitige Katalog-Anfragen
settings-api-parallel-help = Gleichzeitige Anfragen an die API von GameBanana. Bilder und Downloads zählen getrennt.
settings-timeouts = Zeitlimits
settings-timeouts-help = Verbinden: aufgeben, wenn in dieser Zeit keine Verbindung zustande kommt. Antwort: aufgeben, wenn der Server nichts mehr sendet.
settings-timeouts-default = 10 s · 30 s
settings-timeout-connect = Verbinden
settings-timeout-read = Antwort
settings-retries = Wiederholungen
settings-retries-help = Wie oft eine fehlgeschlagene Anfrage mit wachsenden Pausen wiederholt wird, bevor deadloader einen Fehler zeigt.
settings-connection-group = Verbindung
settings-proxy = Proxy
settings-proxy-help = Keiner: direkte Verbindung. System: der in Windows eingestellte Proxy. Manuell: unten eingeben.
settings-proxy-none = Keiner
settings-proxy-system = System
settings-proxy-manual = Manuell
settings-proxy-host = Adresse
settings-proxy-port = Port
settings-proxy-hint = HTTP-Proxy. Nur die Anfragen von deadloader laufen darüber.
settings-proxy-invalid = Gib einen Hostnamen oder eine Adresse ohne Benutzernamen und Passwort ein, z. B. proxy.example.com.

# Cache
settings-thumbs-group = Vorschaubilder und Katalogseiten
settings-cache-limit = Cache-Grenze
settings-cache-limit-help = Ist der Cache voll, werden zuerst die Vorschaubilder entfernt, die du am längsten nicht gesehen hast.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = Aktuelle Größe
settings-cache-breakdown = { $thumbnails } Vorschaubilder · { $pages } Katalogseiten
settings-cache-empty = Leer — noch nichts zwischengespeichert.
settings-cache-used = { $used } von { $limit }
settings-cache-clear = Cache leeren
settings-cache-cleared = Cache geleert — { $size } frei. Vorschaubilder werden bei Bedarf neu geladen.
settings-cache-cleared-partly = { $failed } Dateien waren in Benutzung und bleiben bis zum nächsten Leeren.

# Logs
settings-keep-group = Protokolle aufbewahren
settings-log-days = Protokolle aufbewahren für
settings-log-days-help = Ältere Einträge werden beim Start von deadloader gelöscht.
settings-days = { $count ->
        [one] { $count } Tag
       *[other] { $count } Tage
    }
settings-year = 1 Jahr
settings-log-size = Maximale Größe
settings-log-size-help = Über dieser Größe werden zuerst die ältesten Protokolldateien gelöscht.
settings-privacy-group = Privatsphäre
settings-mask = Windows-Benutzernamen ausblenden
settings-mask-help = Pfade in Protokollen zeigen { $masked } statt deines Namens — sicherer, wenn du ein Protokoll teilst.
settings-logs-folder-group = Protokollordner
settings-logs-folder = Wo die Protokolle liegen
settings-logs-now = { $size } · { $files } Dateien · die älteste vom { $date }
settings-open-logs = Protokollordner öffnen

# Advanced
settings-advanced-warning = Für erfahrene Nutzer.
settings-advanced-warning-text = Ein Fehler hier kann ändern, wie deadloader arbeitet. Es prüft die Datei vor dem Speichern und behält eine Sicherung der vorherigen Version.
settings-files-group = Einstellungsdateien bearbeiten
settings-file-settings = Einstellungen
settings-file-manifest = Spielordner-Manifest
settings-file-label = Zu bearbeitende Datei
settings-file-settings-lead = Alles von diesen Seiten in einer Datei. Änderungen erscheinen nach dem Speichern auf den Seiten.
settings-file-manifest-lead = Die Liste dessen, was deadloader in den Spielordner gelegt hat. Das Zurücksetzen verlässt sich darauf — ein falscher Eintrag kann Dateien zurücklassen.
settings-json-error = Das ist kein gültiges JSON, daher wurde nichts gespeichert.
settings-json-where = Zeile { $line }, Spalte { $column }: Hier oder kurz davor fehlt etwas oder ist zu viel — oft ein Komma oder ein Anführungszeichen.
settings-json-fix = Korrigiere die Stelle und speichere erneut — bis dahin nutzt deadloader die aktuelle Datei weiter.
settings-json-refused = Diese Werte sind nicht verwendbar, daher wurde nichts gespeichert.
settings-json-newer = Diese Datei stammt von einem neueren deadloader (Format { $found }; diese Version liest bis { $supported }), daher wurde nichts gespeichert.
settings-json-invalid = Mit dieser Datei kann deadloader nichts anfangen, daher wurde nichts gespeichert.
settings-json-details = Details
settings-problem-unknown-field = { $path }: keine Einstellung dieser Version.
settings-problem-invalid-value = { $path }: Dieser Wert ist hier nicht verwendbar.
settings-problem-out-of-range = { $path }: { $found } liegt außerhalb von { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: „{ $entry }“ ist hier nicht gültig.
settings-problem-duplicate-entry = { $path }: „{ $entry }“ steht doppelt in der Liste.
settings-problem-needs-age = { $path }: NSFW-Inhalte anzuzeigen braucht zuerst die Altersbestätigung (Einstellungen → Katalog).
settings-problem-local-mode = { $path }: im lokalen Modus nicht verfügbar.
settings-json-idle = Wird beim Speichern geprüft.
settings-json-saved = Gültig und gespeichert. Vorherige Version gesichert als { $backup }.
settings-json-saved-plain = Gültig und gespeichert.
settings-json-save = Prüfen und speichern
settings-json-discard = Änderungen verwerfen
settings-json-backup = Vor dem Speichern wird die vorherige Datei gesichert; die letzten 10 bleiben erhalten.
settings-domains-group = Zusätzlich erlaubte Domains
settings-domains-lead = deadloader spricht nur mit GameBanana. Wird ein Download auf einen anderen Host umgeleitet und blockiert, kannst du diesen Host hier erlauben. In der Aktivität sind Anfragen dorthin als „vom Nutzer hinzugefügt“ markiert.
settings-domains-built-in = Eingebaut:
settings-domain-user = vom Nutzer hinzugefügt
settings-domains-none = Keine zusätzlichen Domains.
settings-domain-add = Domain hinzufügen
settings-domain-hint = Nur der Hostname, z. B. files.example.com.
settings-domain-empty = Gib zuerst einen Hostnamen ein, z. B. files.example.com.
settings-domain-not-host = Gib nur den Hostnamen ein — ohne https:// und Pfad, z. B. files.example.com.
settings-domain-invalid = „{ $host }“ ist kein Hostname. Prüfe die Schreibweise, z. B. files.example.com.
settings-domain-built-in = { $host } ist eingebaut und immer erlaubt.
settings-domain-listed = { $host } steht bereits in der Liste.

# About
settings-version = Version { $version }
settings-commit = Gebaut aus Commit
settings-commit-local = Lokaler Build — nicht von GitHub Actions gebaut
settings-copy-commit = Commit-ID kopieren
settings-release-page = Release-Seite
settings-licence-group = Lizenz
settings-licence-help = deadloader ist freie Software: Du darfst sie unter der GPL-3 nutzen, untersuchen, weitergeben und ändern. Sie kommt ohne Gewährleistung.
settings-read-licence = Lizenz lesen
settings-source = Quellcode
settings-verify-group = So prüfst du diesen Download
settings-verify-lead = Zwei unabhängige Prüfungen, dass diese deadloader.exe die auf GitHub veröffentlichte ist.
settings-verify-hash = SHA-256-Prüfsumme vergleichen
settings-verify-hash-help = Sie muss mit der Prüfsumme auf der Release-Seite übereinstimmen. Eine veränderte Kopie könnte hier einen falschen Wert zeigen, also berechne sie auch selbst:
settings-verify-hash-cmd = In PowerShell, im Ordner der exe:
settings-verify-attest = Build-Nachweis prüfen
settings-verify-attest-help = Belegt, dass GitHub Actions diese Datei aus dem veröffentlichten Quellcode gebaut hat. Benötigt die GitHub CLI:
settings-copy-hash = Prüfsumme kopieren
settings-copy-command = Befehl kopieren
settings-unsigned = deadloader.exe ist nicht code-signiert.
settings-unsigned-text = Windows SmartScreen meldet beim ersten Start eventuell „Der Computer wurde durch Windows geschützt“. Mit den beiden Prüfungen oben stellst du sicher, dass die Datei echt ist.
settings-webview-group = Fenster-Engine
settings-webview-help = Das Fenster von deadloader zeichnet WebView2, ein Teil von Windows. Windows aktualisiert es selbst. deadloader schaltet ab:
settings-webview-off-1 = DNS-over-HTTPS-Abfragen
settings-webview-off-2 = Anmeldung mit dem Microsoft-Konto im Fenster
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Netzwerkzugriffe im Hintergrund und Komponenten-Updates
settings-webview-off-5 = Serveranfragen der Formular-Autofill-Funktion
settings-webview-off-6 = Fehlerberichte an Microsoft (Domain Reliability)
settings-webview-no-route = Außerdem hat das Fenster überhaupt keinen Weg ins Internet.
settings-webview-unknown = Version unbekannt
settings-data-group = Deine Daten
settings-data-folder = Datenordner
settings-data-folder-help = Alles, was deadloader speichert, liegt in diesem einen Ordner neben der exe.
settings-notice = Hinweis beim ersten Start
settings-notice-help = Version { $version }, bestätigt am { $date } auf { $language }. Deine Wahl: Katalog { $catalog }, Update-Suche { $updates }.
settings-notice-none = Noch nicht bestätigt.
settings-notice-again = Erneut lesen

# D7
settings-d7-title = Spielordner auf { $drive } zurücksetzen?
settings-d7-from =
    { $others ->
        [0] Die Installation des ausgewählten Profils { $profile }. deadloader entfernt aus
       *[other] Die Installation des ausgewählten Profils { $profile } (auch von { $others } genutzt). deadloader entfernt aus
    }
settings-d7-files =
    { $count ->
        [one] 1 Mod-Datei, die es hinzugefügt hat, in
       *[other] { $count } Mod-Dateien, die es hinzugefügt hat, in
    }
settings-d7-line = Seine Zeilen in gameinfo.gi
settings-d7-line-why = — die Datei wird wieder so, wie Steam sie installiert hat.
settings-d7-stays = Bleibt, wie es ist:
settings-d7-library = Deine Bibliothek — { $mods } Mods, { $size }
settings-d7-profiles =
    { $count ->
        [one] Dein Profil — { $names }
       *[other] Deine { $count } Profile — { $names }
    }
settings-d7-other = Die Installation auf { $path } ({ $profile }) — bleibt unberührt. Zum Zurücksetzen wähle ein Profil aus, das sie nutzt, und setze erneut zurück.
settings-d7-other-plain = Die Installation auf { $path } — bleibt unberührt.
settings-d7-back = „Anwenden“ oder „Spielen“ stellt alles wieder her.
settings-d7-foreign =
    { $count ->
        [one] Auch 1 Datei entfernen, die nicht von deadloader stammt
       *[other] Auch { $count } Dateien entfernen, die nicht von deadloader stammen
    }
settings-d7-foreign-why = von einem anderen Programm oder manuell installiert. Sie werden endgültig gelöscht; deadloader kann sie nicht wiederherstellen.
settings-d7-confirm = Zurücksetzen
settings-d7-confirm-foreign =
    { $count ->
        [one] Zurücksetzen und 1 Datei löschen
       *[other] Zurücksetzen und { $count } Dateien löschen
    }

# D8
settings-d8-title = Alle deadloader-Daten löschen?
settings-d8-from = deadloader löscht alles in
settings-d8-from-2 = und schließt sich dann.
settings-d8-what = Was
settings-d8-contains = Enthält
settings-d8-size = Größe
settings-d8-total = Gesamt
settings-d8-library = Bibliothek
settings-d8-library-has = { $mods } Mods, { $files } Dateien
settings-d8-profiles = Profile
settings-d8-logs = Protokolle
settings-d8-logs-has = { $files } Dateien, seit { $date }
settings-d8-logs-none = keine Protokolldateien
settings-d8-cache = Cache
settings-d8-cache-has = Vorschaubilder und Katalogseiten
settings-d8-settings = Einstellungen
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
        [one] inkl. 1 Sicherung
       *[other] inkl. { $backups } Sicherungen
    }
settings-d8-other = Sonstiges
settings-d8-other-has = angefangene Downloads, Sicherungen von gameinfo.gi, Daten der Fenster-Engine
settings-d8-exe = Nicht gelöscht: deadloader.exe selbst — lösche sie selbst, wenn du sie nicht mehr brauchst.
settings-d8-rollback = Zuerst alle Spielordner zurücksetzen, die deadloader nutzt (empfohlen)
settings-d8-rollback-why = Entfernt vor dem Löschen die Mod-Dateien von deadloader und seine Zeilen in gameinfo.gi aus jedem Spielordner. Fremde Dateien bleiben.
settings-d8-rollback-off = Nicht angekreuzt: Die Mod-Dateien von deadloader bleiben in den Spielordnern und Deadlock lädt sie weiter — ohne deadloader, das sie entfernen könnte.
settings-d8-type-1 = Gib zur Bestätigung
settings-d8-type-2 = ein
settings-d8-word = löschen
settings-d8-hint = Die Schaltfläche wird frei, sobald das Wort stimmt.
settings-d8-hint-ok = Bestätigt. Das lässt sich nicht rückgängig machen.
settings-d8-type-first = Zuerst „{ $word }“ eingeben
settings-d8-deleting = Wird gelöscht…
settings-d8-confirm = Löschen und schließen
settings-d8-confirm-rollback = Zurücksetzen, löschen, schließen

# Reset
settings-reset-title = Alle Einstellungen zurücksetzen?
settings-reset-body = Jede Einstellung kehrt zum Standard zurück, auch zugelassene Bild-Hosts und zusätzliche Domains.
settings-reset-keeps = Bibliothek, Profile und Protokolle bleiben unberührt, ebenso der Spielordner.
settings-reset-backup = Eine Sicherung der aktuellen Einstellungen bleibt als settings.json.bak-… erhalten.
settings-reset-confirm = Zurücksetzen
settings-reset-done = Alle Einstellungen auf Standard zurückgesetzt.
settings-reset-done-text = Sicherung: { $backup }
