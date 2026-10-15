# The window frame and the sidebar: title bar, navigation, profile, game and network status, Play, D13.
nav-main = Hauptnavigation
nav-catalog = Katalog
nav-library = Bibliothek
nav-profiles = Profile
nav-downloads = Downloads
nav-activity = Aktivität
nav-settings = Einstellungen
nav-pending = Ausstehende Änderungen
nav-pending-count = Ausstehende Änderungen: { $count }
nav-downloads-count = Downloads · { $count }
sidebar-collapse = Seitenleiste einklappen
sidebar-expand = Seitenleiste ausklappen
sidebar-close = Seitenleiste schließen
profile-applied = Im Spiel angewendet
profile-not-applied = Ausgewählt · nicht angewendet
profile-button = Profil { $name } · { $state }
game-found = Deadlock gefunden
game-running = Deadlock läuft
game-missing = Deadlock nicht gefunden
game-on-drive = { $status } · { $drive }
game-at-path = { $status } — { $path }
network-online = Online
network-offline = Offline
network-offline-detail = gespeicherter Katalog von { $time }
network-local = Lokaler Modus
network-local-detail = keine Anfragen
network-title-online = Online — Aktivität öffnen
network-title-running =
    Online — { $count ->
        [one] { $count } Anfrage läuft
       *[other] { $count } Anfragen laufen
    } — Aktivität öffnen
network-title-offline = Offline — gespeicherter Katalog von { $time } — Aktivität öffnen
network-title-local = Lokaler Modus — keine Netzwerkanfragen — Aktivität öffnen
play = Spielen
play-applying = Wird angewendet…
play-running = Deadlock läuft bereits
play-missing = Deadlock wurde nicht gefunden — wähle zuerst den Spielordner
window-minimize = Minimieren
window-maximize = Maximieren
window-close = Schließen
play-confirm-title = { $profile } anwenden und Deadlock starten?
play-confirm-off =
    { $count ->
        [one] 1 Mod wird ausgeschaltet:
       *[other] { $count } Mods werden ausgeschaltet:
    }
play-confirm-on =
    { $count ->
        [one] 1 Mod wird eingeschaltet: { $mods }.
       *[other] { $count } Mods werden eingeschaltet: { $mods }.
    }
play-confirm-keep = { $applied } bleibt in deinen Profilen — du kannst jederzeit zurückwechseln.
play-confirm-cancel = Abbrechen
play-confirm-play = Anwenden und spielen
toast-dismiss = Ausblenden
