# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Diesen Link im Browser öffnen?
catalog-link-body = Er verlässt deadloader und öffnet sich in deinem Standardbrowser.
catalog-link-always = Links immer sofort öffnen
catalog-link-cancel = Abbrechen
catalog-link-open = Öffnen

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Ausgeblendet
catalog-nsfw-blurred = Verschwommen
catalog-nsfw-shown = Angezeigt
catalog-age-title = Mods mit sensiblen Inhalten anzeigen?
catalog-age-body = Manche Mods auf GameBanana sind wegen Nacktheit, sexueller Themen o. Ä. eingestuft. Um sie zu sehen, bestätige, dass du mindestens 18 oder dort, wo du lebst, volljährig bist.
catalog-age-choice = Deine Wahl: { $mode }.
catalog-age-once = deadloader fragt das nur einmal. Du kannst jederzeit zu „Ausgeblendet“ zurück.
catalog-age-confirm = Ich bin volljährig
catalog-age-cancel = Abbrechen

## S2: the catalog grid
catalog-title = Katalog
catalog-section = Bereich
catalog-section-mods = Mods
catalog-section-sounds = Sounds
catalog-search = Mods durchsuchen
catalog-search-sounds = Sounds durchsuchen
catalog-search-offline = Suche braucht eine Verbindung
catalog-search-off = Suche nicht verfügbar — Katalog nicht lesbar
catalog-search-clear = Suche leeren
catalog-results =
    { $count ->
        [one] { $count } Ergebnis
       *[other] { $count } Ergebnisse
    }
catalog-sort = Sortierung
catalog-sort-newest = Neueste
catalog-sort-latest_updated = Kürzlich aktualisiert
catalog-sort-most_downloaded = Am häufigsten heruntergeladen
catalog-sort-most_liked = Am beliebtesten
catalog-sort-most_viewed = Am häufigsten angesehen
catalog-sort-most_commented = Am häufigsten kommentiert
catalog-sort-oldest = Älteste
catalog-sort-best = Beste Treffer
catalog-sort-best-title = Während der Suche wird nach bester Übereinstimmung sortiert
catalog-category = Kategorie
catalog-category-all = Alle Kategorien
catalog-hero = Held
catalog-hero-all = Alle Helden
catalog-hero-filter = Held suchen
catalog-hero-none = Kein Held passt
catalog-filters = Filter
catalog-filter-nsfw = Mods mit Inhaltseinstufung (NSFW)
catalog-filter-favourites = Nur Favoriten
catalog-filter-favourites-short = nur Favoriten
catalog-filter-installed = Installierte ausblenden
catalog-filter-installed-short = Installierte ausgeblendet
catalog-nsfw-caption-hidden = NSFW ausgeblendet
catalog-nsfw-caption-blurred = NSFW verschwommen · Karte anklicken zum Anzeigen
catalog-nsfw-caption-shown = NSFW angezeigt
catalog-loading = Mods werden von GameBanana geladen…
catalog-loading-sounds = Sounds werden von GameBanana geladen…
catalog-loading-more = Weitere werden geladen…
catalog-more-failed = Die nächste Seite konnte nicht geladen werden.
catalog-grid = Mods
catalog-grid-sounds = Sounds
catalog-likes =
    { $count ->
        [one] { $count } Like
       *[other] { $count } Likes
    }
catalog-favourite-add = Zu Favoriten hinzufügen
catalog-favourite-remove = Aus Favoriten entfernen
catalog-sensitive = Sensibler Inhalt
catalog-click-show = Zum Anzeigen klicken
catalog-badge-installed = Installiert
catalog-badge-update = Update
catalog-card-installing = Wird installiert · { $percent } %
catalog-empty = Hier gibt es noch keine Mods
catalog-empty-search = Keine Mods passen zu „{ $text }“
catalog-empty-favourites = Noch keine Favoriten
catalog-empty-filters = Aktive Filter: { $filters }. Entferne sie, um alle Mods zu durchsuchen, oder versuche ein kürzeres Wort.
catalog-empty-shorter = Versuche ein kürzeres oder anderes Wort.
catalog-clear-filters = Filter entfernen
catalog-rate-retry = deadloader versucht es in { $seconds } s automatisch erneut — die Mods unten bleiben nutzbar.
catalog-retry-now = Jetzt erneut versuchen
catalog-try-again = Erneut versuchen
catalog-check-update = Nach deadloader-Update suchen
catalog-details = Details · Protokolleintrag
catalog-copy-id = Protokoll-ID kopieren
catalog-open-activity = In Aktivität öffnen
catalog-offline-empty-title = Offline — nichts gespeichert
catalog-offline-empty-text = deadloader erreicht GameBanana nicht, und dieser Teil des Katalogs wurde vorher nicht gespeichert.
catalog-error-title = Der Katalog kann gerade nicht angezeigt werden.
catalog-local-title = Der Katalog ist aus (lokaler Modus)
catalog-local-why = Im lokalen Modus stellt deadloader keinerlei Netzwerkanfragen. Bibliothek, Profile und Spielen funktionieren wie gewohnt, und du kannst Mods von der Festplatte hinzufügen.
catalog-local-do = Schalte den Katalog ein, um Mods von GameBanana zu durchsuchen und zu installieren. deadloader spricht dann nur mit GameBanana.
catalog-local-enable = Katalog aktivieren
catalog-local-library = Zur Bibliothek

## S3: the mod page
catalog-breadcrumb = Navigationspfad
catalog-back = Zurück zum Katalog
catalog-page-loading = Mod-Seite wird von GameBanana geladen…
catalog-page-offline = Offline — gespeicherte Kopie vom { $date }.
catalog-page-offline-why = Neue Dateien installieren braucht eine Verbindung; zwischen heruntergeladenen Varianten wechseln geht weiterhin.
catalog-page-offline-empty-title = Offline — diese Mod-Seite wurde nicht gespeichert
catalog-page-offline-empty-text = deadloader erreicht GameBanana nicht, und diese Seite wurde vorher nie geöffnet.
catalog-page-local-title = Diese Mod-Seite wurde nicht gespeichert
catalog-page-local-text = Im lokalen Modus zeigt deadloader nur vorher gespeicherte Mod-Seiten.
catalog-removed-title = Dieser Mod wurde von GameBanana entfernt
catalog-removed-why = Er wurde zurückgehalten oder gelöscht, deshalb kann deadloader seine Seite nicht zeigen und keine Dateien laden.
catalog-removed-keep = Deine installierte Kopie ({ $copy }) bleibt in der Bibliothek und funktioniert weiter — sie bekommt nur keine Updates mehr.
catalog-show-in-library = In der Bibliothek zeigen
catalog-installed = Installiert
catalog-by = von { $author }
catalog-version = Version { $version }
catalog-views =
    { $count ->
        [one] { $count } Aufruf
       *[other] { $count } Aufrufe
    }
catalog-downloads =
    { $count ->
        [one] { $count } Download
       *[other] { $count } Downloads
    }
catalog-added = Hinzugefügt { $when }
catalog-updated = Aktualisiert { $when }
catalog-favourite = Favorit
catalog-open-gamebanana = Auf GameBanana öffnen
catalog-gallery-alt = { $name }, Galeriebild { $index }
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Vorheriges Bild
catalog-gallery-next = Nächstes Bild
catalog-gallery-original = Original ansehen
catalog-gallery-hidden = Bilder von Mods mit Inhaltseinstufung sind ausgeblendet.
catalog-audio-preview = Dieser Sound hat eine Hörprobe auf GameBanana.
catalog-description = Beschreibung
catalog-description-more = Ganze Beschreibung zeigen
catalog-description-less = Weniger zeigen
catalog-description-truncated = Nur der Anfang dieser Beschreibung wird gezeigt.
catalog-opens-browser = öffnet im Browser
catalog-opens-here = Öffnet in deadloader
catalog-image-from = Bild von
catalog-image-click = zum Laden klicken
catalog-image-failed = konnte nicht geladen werden
catalog-image-alt = Bild aus der Beschreibung
catalog-youtube = YouTube-Video
catalog-spoiler = Spoiler — zum Anzeigen klicken
catalog-spoiler-hide = Spoiler verbergen
catalog-files = Dateien
catalog-files-clean =
    { $count ->
        [one] Sie hat die Virenprüfung von GameBanana bestanden
       *[other] Alle { $count } haben die Virenprüfung von GameBanana bestanden
    }
catalog-files-clean-dated =
    { $count ->
        [one] Sie hat die Virenprüfung von GameBanana bestanden · hochgeladen am { $date }
       *[other] Alle { $count } haben die Virenprüfung von GameBanana bestanden · hochgeladen am { $date }
    }
catalog-files-av = Das Ergebnis der Virenprüfung von GameBanana steht bei jeder Datei.
catalog-variants-note = Diese Dateien ändern dieselben Spieldateien, deshalb sind sie Varianten — nur eine ist aktiv.
catalog-no-files = Dieser Mod hat keine Dateien zum Herunterladen.
catalog-older-files = Ältere Dateien ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } Download
       *[other] { $count } Downloads
    }
catalog-file-active = aktive Variante
catalog-file-downloaded = heruntergeladen
catalog-file-install = Installieren
catalog-file-install-label = { $file } installieren und aktivieren
catalog-file-install-title = Lädt diese Datei herunter und macht sie zur aktiven Variante
catalog-file-install-offline = Installieren braucht eine Verbindung
catalog-file-installing = Wird installiert…
catalog-file-installed = Installiert
catalog-file-switch = Wechseln zu
catalog-file-switch-label = Wechseln zu { $file }
catalog-file-switch-title = Bereits heruntergeladen — zur aktiven Variante machen
catalog-file-update = Update
catalog-file-update-title = Eine neuere Version ersetzt deine installierte Datei — sieh sie dir in der Bibliothek an
catalog-av-clean = Sauber
catalog-av-clean-title = GameBanana-Virenprüfung: sauber
catalog-av-flagged = Beanstandet
catalog-av-flagged-title = GameBanana-Virenprüfung: { $verdict }
catalog-av-pending = Noch nicht geprüft
catalog-av-pending-title = GameBanana hat die Prüfung dieser Datei noch nicht abgeschlossen
catalog-av-none = Keine Prüfung
catalog-av-none-title = GameBanana hat für diese Datei kein Prüfergebnis geschickt
catalog-credits = Mitwirkende
catalog-no-ratings = Keine Inhaltseinstufungen
catalog-no-licence = Kein Lizenztext
catalog-no-sources = Keine alternativen Quellen
catalog-rating-sexual_themes = Sexuelle Themen
catalog-rating-sexual_content = Sexuelle Inhalte
catalog-rating-skimpy_attire = Knappe Kleidung
catalog-rating-lewd_angles_and_poses = Anzügliche Posen und Perspektiven
catalog-rating-full_nudity = Vollständige Nacktheit
catalog-licence = Lizenz
catalog-sources = Alternative Quellen
catalog-videos = Videos

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Bild von einem neuen Host laden?
catalog-image-body-before = deadloader lädt dieses Bild von
catalog-image-body-after = { " " }herunter. Dieser Host ist nicht GameBanana — die Anfrage erscheint wie jede andere in der Aktivität.
catalog-image-note = Bilder sind größenbegrenzt. Erlaubte Hosts kannst du unter Einstellungen → Katalog entfernen.
catalog-image-always = Bilder von diesem Host immer erlauben
catalog-image-cancel = Abbrechen
catalog-image-load = Bild laden
