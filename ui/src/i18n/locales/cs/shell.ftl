# The window frame and the sidebar: title bar, navigation, profile, game and network status, Play, D13.
nav-main = Hlavní navigace
nav-catalog = Katalog
nav-library = Knihovna
nav-profiles = Profily
nav-downloads = Stahování
nav-activity = Aktivita
nav-settings = Nastavení
nav-pending = Čekající změny
nav-pending-count = Čekající změny: { $count }
nav-downloads-count = Stahování · { $count }
sidebar-collapse = Sbalit postranní panel
sidebar-expand = Rozbalit postranní panel
sidebar-close = Zavřít postranní panel
profile-applied = Použito ve hře
profile-not-applied = Vybráno · nepoužito
profile-button = Profil { $name } · { $state }
game-found = Deadlock nalezen
game-running = Deadlock běží
game-missing = Deadlock nenalezen
game-on-drive = { $status } · { $drive }
game-at-path = { $status } — { $path }
network-online = Online
network-offline = Offline
network-offline-detail = uložený katalog z { $time }
network-local = Místní režim
network-local-detail = žádné požadavky
network-title-online = Online — otevřít Aktivitu
network-title-running =
    Online — { $count ->
        [one] běží { $count } požadavek
        [few] běží { $count } požadavky
        [many] běží { $count } požadavku
       *[other] běží { $count } požadavků
    } — otevřít Aktivitu
network-title-offline = Offline — uložený katalog z { $time } — otevřít Aktivitu
network-title-local = Místní režim — žádné síťové požadavky — otevřít Aktivitu
play = Hrát
play-applying = Používání…
play-running = Deadlock už běží
play-missing = Deadlock nebyl nalezen — nejprve vyberte složku hry
window-minimize = Minimalizovat
window-maximize = Maximalizovat
window-close = Zavřít
play-confirm-title = Použít { $profile } a spustit Deadlock?
play-confirm-off =
    { $count ->
        [one] 1 mod bude vypnut:
        [few] { $count } mody budou vypnuty:
        [many] { $count } modu bude vypnuto:
       *[other] { $count } modů bude vypnuto:
    }
play-confirm-on =
    { $count ->
        [one] 1 mod bude zapnut: { $mods }.
        [few] { $count } mody budou zapnuty: { $mods }.
        [many] { $count } modu bude zapnuto: { $mods }.
       *[other] { $count } modů bude zapnuto: { $mods }.
    }
play-confirm-keep = { $applied } zůstane ve vašich profilech — kdykoli se k němu můžete vrátit.
play-confirm-cancel = Zrušit
play-confirm-play = Použít a hrát
toast-dismiss = Zavřít
