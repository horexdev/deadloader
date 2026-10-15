# The window frame and the sidebar: title bar, navigation, profile, game and network status, Play, D13.
nav-main = Nawigacja główna
nav-catalog = Katalog
nav-library = Biblioteka
nav-profiles = Profile
nav-downloads = Pobieranie
nav-activity = Aktywność
nav-settings = Ustawienia
nav-pending = Oczekujące zmiany
nav-pending-count = Oczekujące zmiany: { $count }
nav-downloads-count = Pobieranie · { $count }
sidebar-collapse = Zwiń pasek boczny
sidebar-expand = Rozwiń pasek boczny
sidebar-close = Zamknij pasek boczny
profile-applied = Zastosowany w grze
profile-not-applied = Wybrany · niezastosowany
profile-button = Profil { $name } · { $state }
game-found = Znaleziono Deadlock
game-running = Deadlock jest uruchomiony
game-missing = Nie znaleziono Deadlock
game-on-drive = { $status } · { $drive }
game-at-path = { $status } — { $path }
network-online = Online
network-offline = Offline
network-offline-detail = zapisany katalog z { $time }
network-local = Tryb lokalny
network-local-detail = brak żądań
network-title-online = Online — otwórz Aktywność
network-title-offline = Offline — zapisany katalog z { $time } — otwórz Aktywność
network-title-local = Tryb lokalny — brak żądań sieciowych — otwórz Aktywność
play = Graj
play-applying = Stosowanie…
play-running = Deadlock jest już uruchomiony
play-missing = Nie znaleziono Deadlock — najpierw wybierz folder gry
window-minimize = Minimalizuj
window-maximize = Maksymalizuj
window-close = Zamknij
play-confirm-title = Zastosować { $profile } i uruchomić Deadlock?
play-confirm-off =
    { $count ->
        [one] 1 mod zostanie wyłączony:
        [few] { $count } mody zostaną wyłączone:
        [many] { $count } modów zostanie wyłączonych:
       *[other] { $count } modu zostanie wyłączone:
    }
play-confirm-on =
    { $count ->
        [one] 1 mod zostanie włączony: { $mods }.
        [few] { $count } mody zostaną włączone: { $mods }.
        [many] { $count } modów zostanie włączonych: { $mods }.
       *[other] { $count } modu zostanie włączone: { $mods }.
    }
play-confirm-keep = { $applied } zostaje w Twoich profilach — możesz do niego wrócić w każdej chwili.
play-confirm-cancel = Anuluj
play-confirm-play = Zastosuj i graj
toast-dismiss = Odrzuć
