# Downloads (S6): the queue, its totals, the session’s history, the banners and D4 (choose a variant).
downloads-title = Pobieranie
downloads-active = Aktywne
downloads-queued = W kolejce
downloads-speed = Prędkość
downloads-per-second = { $size }/s
downloads-limit-parallel =
    { $count ->
        [one] Jedno pobieranie naraz
        [few] Do { $count } pobierań naraz
        [many] Do { $count } pobierań naraz
       *[other] Do { $count } pobierań naraz
    }
downloads-limit-none = bez limitu prędkości
downloads-limit-speed = najwyżej { $speed }/s
downloads-change-settings = Zmień w Ustawieniach
downloads-pause-all = Wstrzymaj wszystko
downloads-resume-all = Wznów wszystko
downloads-add-disk = Dodaj z dysku
downloads-add-title = Dodaj mody z dysku
downloads-add-filter = Pliki modów i archiwa
downloads-queue = Kolejka
downloads-queue-empty = Kolejka jest pusta. Zainstaluj moda z katalogu albo dodaj pliki z dysku.
downloads-history = Zakończone w tej sesji
downloads-history-note = Czyszczone po zamknięciu deadloadera. Każde pobieranie zostaje w Aktywności.
downloads-history-empty = W tej sesji nic się jeszcze nie zakończyło.
downloads-clear = Wyczyść
downloads-show-in-library = Pokaż w bibliotece
downloads-show-in-library-of = Pokaż w bibliotece: { $name }
downloads-again = Pobierz ponownie
downloads-again-of = Pobierz ponownie: { $file }
downloads-pause = Wstrzymaj
downloads-pause-of = Wstrzymaj: { $file }
downloads-resume = Wznów
downloads-resume-of = Wznów: { $file }
downloads-cancel-of = Anuluj pobieranie { $file }
downloads-remove-of = Usuń zadanie: { $file }
downloads-aside-of = Wybierz później: { $file }
downloads-open-mod = Otwórz stronę moda — { $name }
downloads-update = Aktualizacja { $version }
downloads-update-plain = Aktualizacja
downloads-choose = Wybierz wariant
downloads-retry = Ponów
downloads-show-details = Pokaż szczegóły
downloads-remove-task = Usuń zadanie
downloads-open-settings = Otwórz Ustawienia → Pobieranie
downloads-open-mod-page = Otwórz stronę moda
downloads-open-library = Otwórz bibliotekę
downloads-stage-downloading = Pobieranie
downloads-stage-verifying = Sprawdzanie
downloads-stage-extracting = Rozpakowywanie
downloads-stage-choice = Czeka na twój wybór
downloads-stage-paused = Wstrzymane
downloads-stage-offline = Wstrzymane — offline
downloads-stage-disk = Wstrzymane — pełny dysk
downloads-stage-retrying = Czeka na ponowną próbę
downloads-stage-queued = W kolejce
downloads-stage-failed = Niepowodzenie
downloads-progress = { $done } z { $total } · { $percent } %
downloads-left-seconds = zostało { $seconds } s
downloads-left-minutes = zostało { $minutes } min { $seconds } s
downloads-left-hours = zostało { $hours } h { $minutes } min
downloads-rate = { $speed }/s · { $left }
downloads-verifying-note = Porównywanie sumy kontrolnej (md5) z GameBanana
downloads-extracting-note = Zachowywane są tylko pliki .vpk
downloads-choice-note =
    { $count ->
        [one] { $count } wariant w środku
        [few] { $count } warianty w środku
        [many] { $count } wariantów w środku
       *[other] { $count } wariantu w środku
    }
downloads-offline-note = Wznowi się samo, gdy wrócisz do sieci
downloads-disk-note = Czeka na wolne miejsce na { $drive }
downloads-disk-note-plain = Czeka na wolne miejsce
downloads-paused-note = Wznowi od miejsca, w którym przerwało
downloads-held-note = Wstrzymane — nie ruszy, dopóki go nie wznowisz
downloads-queued-next = Następne — ruszy, gdy zwolni się jedno z { $count } miejsc pobierania
downloads-queued-ahead = Przed nim: { $count }
downloads-queued-offline = Ruszy, gdy wrócisz do sieci
downloads-retrying-note = Ponowna próba za { $seconds } s
downloads-downloaded = pobrano
downloads-offline-title = Offline — pobieranie jest wstrzymane.
downloads-offline-why = Wznowi się samo od miejsca, w którym przerwało, gdy wrócisz do sieci. Import z dysku działa dalej.
downloads-try-again = Spróbuj ponownie
downloads-back-title = Znów online.
downloads-back-why = Wstrzymane pobierania ruszyły od miejsca, w którym przerwały.
downloads-empty-title = Brak pobierań
downloads-empty-text = Zainstaluj moda z katalogu albo dodaj pliki .vpk, .zip, .7z lub .rar ze swojego komputera. Każdy pojawi się tutaj na czas pobierania i rozpakowywania. Możesz też upuścić pliki w dowolnym miejscu tego okna.
downloads-browse = Przeglądaj katalog
downloads-done-installed = Zainstalowano i włączono
downloads-done-variant = Zainstalowano · { $variant }
downloads-done-updated = Zaktualizowano do { $version }
downloads-done-updated-plain = Zaktualizowano
downloads-done-cancelled = Anulowano
downloads-done-removed = Usunięto — archiwum skasowane
downloads-done-kept = W bibliotece, niewłączony
downloads-ago-now = przed chwilą
downloads-ago-minutes = { $count } min temu
downloads-ago-hours = { $count } godz. temu
downloads-d4-title = Wybierz wariant
downloads-d4-lead = To archiwum zawiera { $count } warianty, które zmieniają te same pliki gry, więc naraz może działać tylko jeden.
downloads-d4-description = Opis pliku na GameBanana
downloads-d4-list = Warianty w tym archiwum
downloads-d4-list-meta = { $count } · wybierz jeden
downloads-d4-hint = deadloader zachowa wszystkie ({ $count }) w twojej bibliotece i włączy wybrany w profilu „{ $profile }”, na górze kolejności. Możesz to później zmienić w bibliotece.
downloads-d4-install = Zainstaluj
downloads-d4-installed = Wariant „{ $variant }” · na górze profilu „{ $profile }”.
downloads-cancel = Anuluj
downloads-close-esc = Zamknij (Esc)
downloads-open-folder = Otwórz folder
downloads-opened = Otwarto w Eksploratorze plików
