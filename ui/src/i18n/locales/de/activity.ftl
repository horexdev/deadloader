# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Aktivität
activity-sub =
    { $days ->
        [one] Alles, was deadloader auf diesem PC getan hat: Netzwerkanfragen, Dateiänderungen und App-Ereignisse. Protokolle bleiben { $days } Tag auf diesem PC.
       *[other] Alles, was deadloader auf diesem PC getan hat: Netzwerkanfragen, Dateiänderungen und App-Ereignisse. Protokolle bleiben { $days } Tage auf diesem PC.
    }
activity-open-logs = Protokollordner öffnen
activity-logs-opened = Protokollordner im Explorer geöffnet.
activity-summary = Heute im Überblick
activity-net-today = Netzwerk heute
activity-hosts-label = Heute kontaktierte Hosts — zum Filtern auswählen
activity-hosts-contacted =
    { $count ->
        [one] { $count } Host kontaktiert
       *[other] { $count } Hosts kontaktiert
    }
activity-host-title =
    { $count ->
        [one] { $count } Anfrage · { $source } — zum Filtern auswählen
       *[other] { $count } Anfragen · { $source } — zum Filtern auswählen
    }
activity-net-none-local = keine — lokaler Modus
activity-net-none-yet = noch keine
activity-nothing-else = Sonst nichts.
activity-blocked-lead =
    { $count ->
        [one] { $count } Anfrage an
       *[other] { $count } Anfragen an
    }
activity-blocked-tail =
    { $count ->
        [one] wurde blockiert, bevor eine Verbindung aufgebaut wurde.
       *[other] wurden blockiert, bevor eine Verbindung aufgebaut wurde.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } Anfrage blockiert, nie kontaktiert.
       *[other] { $count } Anfragen blockiert, nie kontaktiert.
    }
activity-show-entry = Eintrag zeigen
activity-game-today = Spielordner heute
activity-show-in-files = In „Dateien“ zeigen
activity-game-none = keine Änderungen
activity-no-game-changes = deadloader hat im Spielordner noch nichts geändert.
activity-game-changes =
    { $count ->
        [one] { $count } Änderung
       *[other] { $count } Änderungen
    }
activity-game-batches =
    { $count ->
        [one] in { $count } Durchgang
       *[other] in { $count } Durchgängen
    }
activity-game-failed =
    { $count ->
        [one] { $count } fehlgeschlagen
       *[other] { $count } fehlgeschlagen
    }
activity-game-hardlink = { $count } verknüpft
activity-game-copy = { $count } kopiert
activity-game-delete = { $count } entfernt
activity-game-rename = { $count } umbenannt
activity-game-write = { $count } geschrieben
activity-game-backup = { $count } gesichert
activity-game-restore = { $count } wiederhergestellt
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi einmal aktualisiert
       *[other] gameinfo.gi { $count }-mal aktualisiert
    }
activity-no-requests-today = Heute keine Anfragen.
activity-no-requests-yet = Noch keine Anfragen.
activity-local-short = Der Katalog ist aus, deshalb verbindet sich deadloader mit nichts.
activity-never-contacted = deadloader hat seit der Installation keinen Host kontaktiert.
activity-tab-network = Netzwerk
activity-tab-files = Dateien
activity-tab-app = App
activity-tab-off = aus
activity-tab-errors =
    { $count ->
        [one] · { $count } Fehler
       *[other] · { $count } Fehler
    }
activity-search = Protokoll durchsuchen
activity-search-clear = Suche löschen
activity-matches =
    { $count ->
        [one] { $count } Treffer
       *[other] { $count } Treffer
    }
activity-matches-more = { $count }+ Treffer
activity-f-time = Zeitraum
activity-f-purpose = Zweck
activity-f-host = Host
activity-f-op = Vorgang
activity-f-level = Stufe
activity-f-result = Ergebnis
activity-all = Alle
activity-ok = OK
activity-errors = Fehler
activity-range-hour = Letzte Stunde
activity-range-today = Heute
activity-range-7d = Letzte 7 Tage
activity-range-30d = Letzte 30 Tage
activity-clear-filters = Filter zurücksetzen
activity-clear-all = Suche und Filter zurücksetzen
activity-follow = Live folgen
activity-follow-tip = Neue Einträge sofort oben anzeigen
activity-follow-on = Neue Einträge erscheinen oben
activity-follow-paused = Angehalten um { $time }
activity-show-new =
    { $count ->
        [one] { $count } neuen anzeigen
       *[other] { $count } neue anzeigen
    }
activity-net-aria = Netzwerkanfragen, neueste zuerst
activity-files-aria = Dateivorgänge, neueste zuerst
activity-app-aria = App-Ereignisse, neueste zuerst
activity-c-redirects = Weiterleitungen
activity-c-time = Zeit
activity-c-purpose = Zweck
activity-c-request = Anfrage
activity-c-status = Status
activity-c-size = Größe
activity-c-duration = Dauer
activity-c-started-by = Auslöser
activity-c-source = Host-Quelle
activity-c-op = Vorgang
activity-c-from-to = Von → nach
activity-c-result = Ergebnis
activity-c-change = Teil der Änderung
activity-c-level = Stufe
activity-c-message = Meldung
activity-to = nach
activity-loading-more = Weitere werden geladen…
activity-p-catalog = Katalog
activity-p-thumbnail = Vorschaubild
activity-p-description-image = Beschreibungsbild
activity-p-download = Download
activity-p-update-check = Update-Prüfung
activity-src-builtin = Integriert
activity-src-github = GitHub
activity-src-user = Hinzugefügt
activity-src-user-image = Bildhost
activity-src-blocked = Nicht erlaubt
activity-src-long-builtin = Integriert — GameBanana, Teil von deadloader
activity-src-long-github = GitHub — deadloaders eigene Update-Prüfung, in den Einstellungen an
activity-src-long-user = Hinzugefügt — von dir unter Einstellungen → Erweitert
activity-src-long-user-image = Erlaubter Bildhost — von dir beim Laden eines Beschreibungsbilds erlaubt
activity-src-long-blocked = Nicht auf der Liste der erlaubten Hosts
activity-by-auto = Auto
activity-by-user = Du
activity-op-hardlink = Verknüpfen
activity-op-copy = Kopieren
activity-op-rename = Umbenennen
activity-op-delete = Löschen
activity-op-write = Schreiben
activity-op-backup = Sichern
activity-op-restore = Wiederherstellen
activity-lv-debug = Debug
activity-lv-info = Info
activity-lv-warn = Warnung
activity-lv-error = Fehler
activity-st-blocked = Blockiert
activity-st-blocked-long = Blockiert — nicht kontaktiert
activity-st-failed = Fehlgeschlagen
activity-redirects =
    { $count ->
        [one] { $count } Weiterleitung
       *[other] { $count } Weiterleitungen
    }
activity-retries =
    { $count ->
        [one] { $count } Wiederholung
       *[other] { $count } Wiederholungen
    }
activity-show-redirects = Weiterleitungen anzeigen
activity-hide-redirects = Weiterleitungen ausblenden
activity-hop-request = Schritt 1 · Anfrage
activity-hop-redirect = Schritt { $n } · Umleitung
activity-hop-retry = Schritt { $n } · Versuch { $try }
activity-ms = { $ms } ms
activity-seconds = { $s } s
activity-no-match = Keine passenden Einträge
activity-no-match-text = Wähle einen anderen Zeitraum oder setze Filter und Suche zurück.
activity-empty-title = Noch nichts
activity-empty-text = deadloader schreibt hier jede Netzwerkanfrage, jede Dateiänderung im Spielordner und jedes App-Ereignis auf, sobald es passiert. Bisher ist nichts passiert.
activity-local-title = Lokaler Modus — es werden keine Anfragen gestellt
activity-local-why = Der Katalog ist aus, deshalb verbindet sich deadloader mit nichts — nicht mit GameBanana und auch sonst nirgendwohin. Dateiänderungen und App-Ereignisse werden weiter in den anderen Tabs protokolliert.
activity-local-do = Schalte den Katalog in den Einstellungen ein, um GameBanana zu durchsuchen. Jede Anfrage erscheint dann hier.
activity-enable-catalog = Katalog aktivieren
activity-details = Details zum Protokolleintrag
activity-close-details = Details schließen
activity-close-esc = Schließen (Esc)
activity-why = Warum:
activity-todo = Was tun:
activity-open-advanced = Einstellungen → Erweitert öffnen
activity-full-record = Vollständiger Eintrag
activity-hops-title = Weiterleitungen · { $count }
activity-user-hidden = Dein Windows-Benutzername ist in Protokollen ausgeblendet:
activity-settings-logs = Einstellungen → Protokolle
activity-copy = Eintrag kopieren
activity-copied = Kopiert
activity-kind-network = Netzwerkanfrage
activity-kind-files = Dateivorgang
activity-kind-app = App-Ereignis
activity-d-time = Zeit
activity-d-purpose = Zweck
activity-d-started-by = Auslöser
activity-d-request = Anfrage
activity-d-source = Host-Quelle
activity-d-status = Status
activity-d-size = Größe
activity-d-duration = Dauer
activity-d-range = Bereich
activity-d-error = Fehler
activity-d-id = Eintrags-ID
activity-d-request-id = Anfrage-ID
activity-d-operation = Vorgang
activity-d-from = Von
activity-d-to = Nach
activity-d-file = Datei
activity-d-result = Ergebnis
activity-d-change = Teil der Änderung
activity-change-apply = Profil „{ $profile }“ angewendet
activity-change-rollback = Zurücksetzen des Spielordners
activity-change-profile-removed = Profil „{ $profile }“ gelöscht, seine Mods haben das Spiel verlassen
activity-change-take-foreign = { $file } zur Verwaltung übernommen
activity-change-gameinfo-fixed = Unsere Zeile in gameinfo.gi eingefügt
activity-change-recovered = Unterbrochene Änderung beim Start abgeschlossen
activity-d-sha256 = SHA-256
activity-d-level = Stufe
activity-d-message = Meldung
activity-d-fields = Details
activity-status-after =
    { $count ->
        [one] { $status } — nach { $count } Weiterleitung
       *[other] { $status } — nach { $count } Weiterleitungen
    }
