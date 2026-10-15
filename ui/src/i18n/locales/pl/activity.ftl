# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Aktywność
activity-sub =
    { $days ->
        [one] Wszystko, co deadloader zrobił na tym komputerze: żądania sieciowe, zmiany plików i zdarzenia aplikacji. Dzienniki zostają na tym komputerze przez { $days } dzień.
        [few] Wszystko, co deadloader zrobił na tym komputerze: żądania sieciowe, zmiany plików i zdarzenia aplikacji. Dzienniki zostają na tym komputerze przez { $days } dni.
        [many] Wszystko, co deadloader zrobił na tym komputerze: żądania sieciowe, zmiany plików i zdarzenia aplikacji. Dzienniki zostają na tym komputerze przez { $days } dni.
       *[other] Wszystko, co deadloader zrobił na tym komputerze: żądania sieciowe, zmiany plików i zdarzenia aplikacji. Dzienniki zostają na tym komputerze przez { $days } dnia.
    }
activity-open-logs = Otwórz folder dzienników
activity-logs-opened = Folder dzienników otwarty w Eksploratorze plików.
activity-summary = Dziś w skrócie
activity-net-today = Sieć dziś
activity-hosts-label = Hosty, z którymi łączono się dziś — wybierz, aby filtrować
activity-hosts-contacted =
    { $count ->
        [one] { $count } host
        [few] { $count } hosty
        [many] { $count } hostów
       *[other] { $count } hosta
    }
activity-host-title =
    { $count ->
        [one] { $count } żądanie · { $source } — wybierz, aby filtrować
        [few] { $count } żądania · { $source } — wybierz, aby filtrować
        [many] { $count } żądań · { $source } — wybierz, aby filtrować
       *[other] { $count } żądania · { $source } — wybierz, aby filtrować
    }
activity-net-none-local = brak — tryb lokalny
activity-net-none-yet = jeszcze brak
activity-nothing-else = Nic więcej.
activity-blocked-lead =
    { $count ->
        [one] { $count } żądanie do
        [few] { $count } żądania do
        [many] { $count } żądań do
       *[other] { $count } żądania do
    }
activity-blocked-tail =
    { $count ->
        [one] zostało zablokowane przed nawiązaniem połączenia.
        [few] zostały zablokowane przed nawiązaniem połączenia.
        [many] zostało zablokowanych przed nawiązaniem połączenia.
       *[other] zostało zablokowane przed nawiązaniem połączenia.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } żądanie zablokowane, bez połączenia.
        [few] { $count } żądania zablokowane, bez połączenia.
        [many] { $count } żądań zablokowanych, bez połączenia.
       *[other] { $count } żądania zablokowane, bez połączenia.
    }
activity-show-entry = Pokaż wpis
activity-game-today = Folder gry dziś
activity-show-in-files = Pokaż w Plikach
activity-game-none = bez zmian
activity-no-game-changes = deadloader nie zmienił jeszcze niczego w folderze gry.
activity-game-changes =
    { $count ->
        [one] { $count } zmiana
        [few] { $count } zmiany
        [many] { $count } zmian
       *[other] { $count } zmiany
    }
activity-game-batches =
    { $count ->
        [one] w { $count } partii
        [few] w { $count } partiach
        [many] w { $count } partiach
       *[other] w { $count } partii
    }
activity-game-failed =
    { $count ->
        [one] { $count } nieudana
        [few] { $count } nieudane
        [many] { $count } nieudanych
       *[other] { $count } nieudanej
    }
activity-game-hardlink = połączone: { $count }
activity-game-copy = skopiowane: { $count }
activity-game-delete = usunięte: { $count }
activity-game-rename = zmienione nazwy: { $count }
activity-game-write = zapisane: { $count }
activity-game-backup = z kopią zapasową: { $count }
activity-game-restore = przywrócone: { $count }
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi zaktualizowany { $count } raz
        [few] gameinfo.gi zaktualizowany { $count } razy
        [many] gameinfo.gi zaktualizowany { $count } razy
       *[other] gameinfo.gi zaktualizowany { $count } razy
    }
activity-no-requests-today = Dziś nie było żądań.
activity-no-requests-yet = Jeszcze nie było żądań.
activity-local-short = Katalog jest wyłączony, więc deadloader z niczym się nie łączy.
activity-never-contacted = Od instalacji deadloader nie łączył się z żadnym hostem.
activity-tab-network = Sieć
activity-tab-files = Pliki
activity-tab-app = Aplikacja
activity-tab-off = wył.
activity-tab-errors =
    { $count ->
        [one] · { $count } błąd
        [few] · { $count } błędy
        [many] · { $count } błędów
       *[other] · { $count } błędu
    }
activity-search = Szukaj w dzienniku
activity-search-clear = Wyczyść wyszukiwanie
activity-matches =
    { $count ->
        [one] { $count } wynik
        [few] { $count } wyniki
        [many] { $count } wyników
       *[other] { $count } wyniku
    }
activity-matches-more = wyniki: { $count }+
activity-f-time = Okres
activity-f-purpose = Cel
activity-f-host = Host
activity-f-op = Operacja
activity-f-level = Poziom
activity-f-result = Wynik
activity-all = Wszystkie
activity-ok = OK
activity-errors = Błędy
activity-range-hour = Ostatnia godzina
activity-range-today = Dziś
activity-range-7d = Ostatnie 7 dni
activity-range-30d = Ostatnie 30 dni
activity-clear-filters = Wyczyść filtry
activity-clear-all = Wyczyść wyszukiwanie i filtry
activity-follow = Śledź na żywo
activity-follow-tip = Pokazuj nowe wpisy na górze od razu, gdy się pojawiają
activity-follow-on = Nowe wpisy pojawiają się na górze
activity-follow-paused = Wstrzymano o { $time }
activity-show-new =
    { $count ->
        [one] Pokaż { $count } nowy
        [few] Pokaż { $count } nowe
        [many] Pokaż { $count } nowych
       *[other] Pokaż { $count } nowego
    }
activity-net-aria = Żądania sieciowe, od najnowszych
activity-files-aria = Operacje na plikach, od najnowszych
activity-app-aria = Zdarzenia aplikacji, od najnowszych
activity-c-redirects = Przekierowania
activity-c-time = Czas
activity-c-purpose = Cel
activity-c-request = Żądanie
activity-c-status = Stan
activity-c-size = Rozmiar
activity-c-duration = Czas trwania
activity-c-started-by = Inicjator
activity-c-source = Źródło hosta
activity-c-op = Operacja
activity-c-from-to = Skąd → dokąd
activity-c-result = Wynik
activity-c-change = Część zmiany
activity-c-level = Poziom
activity-c-message = Komunikat
activity-to = do
activity-loading-more = Wczytywanie…
activity-p-catalog = Katalog
activity-p-thumbnail = Miniatura
activity-p-description-image = Obraz w opisie
activity-p-download = Pobieranie
activity-p-update-check = Sprawdzanie aktualizacji
activity-src-builtin = Wbudowany
activity-src-github = GitHub
activity-src-user = Dodany przez ciebie
activity-src-user-image = Host obrazów
activity-src-blocked = Niedozwolony
activity-src-long-builtin = Wbudowany — GameBanana, część deadloadera
activity-src-long-github = GitHub — sprawdzanie aktualizacji samego deadloadera, włączone w ustawieniach
activity-src-long-user = Dodany przez ciebie — w Ustawienia → Zaawansowane
activity-src-long-user-image = Dozwolony host obrazów — zezwoliłeś na niego, wczytując obraz w opisie
activity-src-long-blocked = Nie ma go na liście dozwolonych hostów
activity-by-auto = Auto
activity-by-user = Ty
activity-op-hardlink = Połączenie
activity-op-copy = Kopiowanie
activity-op-rename = Zmiana nazwy
activity-op-delete = Usunięcie
activity-op-write = Zapis
activity-op-backup = Kopia zapasowa
activity-op-restore = Przywrócenie
activity-lv-debug = Debug
activity-lv-info = Info
activity-lv-warn = Ostrzeżenie
activity-lv-error = Błąd
activity-st-blocked = Zablokowane
activity-st-blocked-long = Zablokowane — bez połączenia
activity-st-failed = Niepowodzenie
activity-redirects =
    { $count ->
        [one] { $count } przekierowanie
        [few] { $count } przekierowania
        [many] { $count } przekierowań
       *[other] { $count } przekierowania
    }
activity-retries =
    { $count ->
        [one] { $count } ponowienie
        [few] { $count } ponowienia
        [many] { $count } ponowień
       *[other] { $count } ponowienia
    }
activity-show-redirects = Pokaż przekierowania
activity-hide-redirects = Ukryj przekierowania
activity-hop-request = Krok 1 · żądanie
activity-hop-redirect = Krok { $n } · przekierowanie
activity-hop-retry = Krok { $n } · próba { $try }
activity-ms = { $ms } ms
activity-seconds = { $s } s
activity-no-match = Brak pasujących wpisów
activity-no-match-text = Wybierz inny okres albo wyczyść filtry i wyszukiwanie.
activity-empty-title = Jeszcze nic
activity-empty-text = deadloader zapisuje tu każde żądanie sieciowe, każdą zmianę plików w folderze gry i każde zdarzenie aplikacji od razu, gdy się wydarzą. Na razie nic się nie wydarzyło.
activity-local-title = Tryb lokalny — żadne żądania nie są wysyłane
activity-local-why = Katalog jest wyłączony, więc deadloader z niczym się nie łączy — ani z GameBanana, ani z niczym innym. Zmiany plików i zdarzenia aplikacji są nadal zapisywane w pozostałych kartach.
activity-local-do = Włącz katalog w ustawieniach, aby przeglądać GameBanana. Każde żądanie pojawi się wtedy tutaj.
activity-local-history = Poniższe żądania zostały wysłane przed włączeniem trybu lokalnego.
activity-enable-catalog = Włącz katalog
activity-details = Szczegóły wpisu dziennika
activity-close-details = Zamknij szczegóły
activity-close-esc = Zamknij (Esc)
activity-why = Dlaczego:
activity-todo = Co zrobić:
activity-open-advanced = Otwórz Ustawienia → Zaawansowane
activity-full-record = Pełny wpis
activity-hops-title = Przekierowania · { $count }
activity-user-hidden = Twoja nazwa użytkownika Windows jest ukryta w dziennikach:
activity-settings-logs = Ustawienia → Dzienniki
activity-copy = Kopiuj wpis
activity-copied = Skopiowano
activity-kind-network = Żądanie sieciowe
activity-kind-files = Operacja na pliku
activity-kind-app = Zdarzenie aplikacji
activity-d-time = Czas
activity-d-purpose = Cel
activity-d-started-by = Inicjator
activity-d-request = Żądanie
activity-d-source = Źródło hosta
activity-d-status = Stan
activity-d-size = Rozmiar
activity-d-duration = Czas trwania
activity-d-range = Zakres
activity-d-error = Błąd
activity-d-id = ID wpisu
activity-d-request-id = ID żądania
activity-d-operation = Operacja
activity-d-from = Skąd
activity-d-to = Dokąd
activity-d-file = Plik
activity-d-result = Wynik
activity-d-change = Część zmiany
activity-change-apply = Profil „{ $profile }” zastosowany
activity-change-rollback = Przywrócenie folderu gry
activity-change-profile-removed = Profil „{ $profile }” usunięty, jego mody opuściły grę
activity-change-take-foreign = { $file } przejęty do zarządzania
activity-change-gameinfo-fixed = Nasz wiersz dodany do gameinfo.gi
activity-change-recovered = Przerwana zmiana dokończona przy starcie
activity-d-sha256 = SHA-256
activity-d-level = Poziom
activity-d-message = Komunikat
activity-d-fields = Szczegóły
activity-status-after =
    { $count ->
        [one] { $status } — po { $count } przekierowaniu
        [few] { $status } — po { $count } przekierowaniach
        [many] { $status } — po { $count } przekierowaniach
       *[other] { $status } — po { $count } przekierowania
    }
