# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Ustawienia
settings-nav = Sekcje ustawień
settings-reset-nav = Przywróć wszystkie ustawienia…
settings-default = Domyślnie: { $value }
settings-restore = Przywróć domyślne
settings-on = Wł.
settings-off = Wył.
settings-cancel = Anuluj
settings-remove = Usuń
settings-remove-item = Usuń { $item }
settings-add = Dodaj
settings-open-folder = Otwórz folder
settings-open-folder-at = Otwórz folder: { $path }
settings-copied = Skopiowano do schowka.
settings-folder = Folder
settings-example = Przykład:

settings-section-general = Ogólne
settings-section-general-lead = Język, uruchamianie i powiększenie.
settings-section-game = Gra
settings-section-game-lead = Gdzie jest zainstalowany Deadlock i co deadloader może tam zmieniać.
settings-section-catalog = Katalog
settings-section-catalog-lead = Katalog GameBanana, treści wrażliwe, obrazy i linki.
settings-section-library = Biblioteka
settings-section-library-lead = Aktualizacje modów, usuwanie modów i stare wersje.
settings-section-downloads = Pobieranie
settings-section-downloads-lead = Kolejka pobierania, archiwa i rozpakowywanie.
settings-section-network = Sieć
settings-section-network-lead = Jak deadloader komunikuje się z GameBanana.
settings-section-cache = Pamięć podręczna
settings-section-cache-lead = Miniatury i strony katalogu zapisane na tym komputerze.
settings-section-logs = Dzienniki
settings-section-logs-lead = Co przechowuje Aktywność i jak długo.
settings-section-advanced = Zaawansowane
settings-section-advanced-lead = Pliki ustawień i dodatkowe dozwolone domeny.
settings-section-about = O deadloader
settings-section-about-lead = Wersja, licencja, jak sprawdzić to pobranie i gdzie są twoje dane.

# General
settings-general-group = Język i uruchamianie
settings-language = Język
settings-language-help = Język systemu: język wyświetlania Windows, jeśli deadloader go ma, w przeciwnym razie angielski.
settings-language-system = Język systemu ({ $language })
settings-startup-profile = Profil przy uruchomieniu
settings-startup-profile-help = Który profil jest wybrany po uruchomieniu deadloader. Wybranie go nie stosuje go.
settings-startup-last = Ostatnio wybrany ({ $name })
settings-startup-last-plain = Ostatnio wybrany
settings-app-updates = Sprawdzaj aktualizacje deadloader przy uruchomieniu
settings-app-updates-help = Przy uruchomieniu deadloader raz pyta GitHub (api.github.com), czy jest nowsza wersja. Aktualizacja zawsze odbywa się ręcznie. Wybrałeś to przy pierwszym uruchomieniu.
settings-app-updates-local = Niedostępne: tryb lokalny nie wysyła żadnych żądań sieciowych. Opcja stanie się dostępna, gdy włączysz katalog (Ustawienia → Katalog).
settings-zoom = Powiększenie
settings-zoom-keys-1 = Także za pomocą Ctrl + kółko myszy,
settings-zoom-keys-2 = oraz
settings-zoom-keys-3 = aby przywrócić.
settings-ctrl = Ctrl
settings-percent = { $value }%

# Game
settings-installs-group = Instalacje gry
settings-installs-lead = deadloader sam znajduje instalacje Deadlock w Steam. Każdy profil używa jednej z nich — zmienisz to w Profilach.
settings-install-library = Biblioteka Steam { $library }
settings-install-build = kompilacja { $build }
settings-install-used =
    { $count ->
        [0] nieużywana przez żaden profil
        [one] używana przez { $count } profil: { $names }
       *[other] używana przez profile ({ $count }): { $names }
    }
settings-install-selected = { $name } (wybrany)
settings-install-applied = { $name } (zastosowany tutaj)
settings-install-steam = Znaleziona przez Steam
settings-install-manual = Dodana ręcznie
settings-install-known = Wciąż są tu pliki deadloader
settings-install-manual-meta = Dodana ręcznie
settings-gi-ours = gameinfo.gi: OK — wiersz deadloader jest na miejscu
settings-gi-wait = gameinfo.gi: zostanie przygotowany, gdy profil zacznie jej używać
settings-gi-rolled = gameinfo.gi: oryginalny — „Zastosuj” ponownie doda wiersz deadloader
settings-gi-foreign = gameinfo.gi: zmieniony przez inny program
settings-gi-missing = gameinfo.gi: nie znaleziono — folder mógł zostać przeniesiony
settings-installs-none = Nie znaleziono jeszcze żadnej instalacji.
settings-add-manual = Dodaj ręcznie…
settings-add-manual-hint = Wybierz folder Deadlock, którego nie znaleziono.
settings-add-manual-title = Wybierz folder Deadlock
settings-install-added = Dodano instalację.
settings-install-removed = Usunięto instalację z listy.
settings-install-removed-text = W tym folderze nic nie zmieniono.
settings-gameinfo-group = Po aktualizacjach gry
settings-autofix = Naprawiaj gameinfo.gi automatycznie
settings-autofix-help = Aktualizacja Deadlock przywraca plik gry, który każe wczytywać mody. deadloader robi jego kopię, przywraca swój wiersz i informuje cię. Gdy wyłączone, najpierw zapyta baner.
settings-danger-group = Cofanie i usuwanie
settings-rollback = Przywróć folder gry wybranego profilu
settings-rollback-help = Dla instalacji wybranego profilu { $profile } ({ $drive }): usuwa pliki modów, które deadloader tam umieścił ({ $files }), i jego wiersz w gameinfo.gi. Biblioteka i profile zostają; „Zastosuj” wszystko przywróci.
settings-rollback-help-others = Inne foldery gry nie są ruszane; aby je przywrócić, wybierz profil, który ich używa.
settings-rollback-nothing = Na { $drive } nie ma nic do przywrócenia — deadloader nie ma tam teraz plików.
settings-rollback-no-game = Nie znaleziono Deadlock, więc nie ma folderu gry do przywrócenia.
settings-rollback-button = Przywróć…
settings-rolled-title = Przywrócono folder gry na { $drive }.
settings-rolled-text =
    { $foreign ->
        [0] Na { $drive } usunięto pliki modów ({ $files }) i wiersz deadloader w gameinfo.gi. Biblioteka i profile zostały — „Zastosuj” lub „Graj” przywróci mody.
       *[other] Na { $drive } usunięto pliki modów ({ $files }), wiersz deadloader w gameinfo.gi i obce pliki ({ $foreign }). Biblioteka i profile zostały — „Zastosuj” lub „Graj” przywróci mody.
    }
settings-wipe = Usuń wszystkie dane deadloader
settings-wipe-help = Usuwa bibliotekę, profile, dzienniki, pamięć podręczną i ustawienia ({ $size }), a potem zamyka deadloader. deadloader.exe zostaje — usuń go potem sam.
settings-wipe-button = Usuń wszystkie dane…

# Catalog
settings-catalog-group = Katalog GameBanana
settings-catalog = Używaj katalogu
settings-catalog-help-on = Przeglądaj i instaluj mody z GameBanana. deadloader rozmawia tylko z GameBanana, a każde żądanie widać w Aktywności. Wyłączone oznacza tryb lokalny.
settings-catalog-help-off = Wyłączone — tryb lokalny: deadloader nie wysyła żadnych żądań sieciowych. Nadal możesz dodawać mody z dysku.
settings-consent-title = Najpierw jeden krok.
settings-consent-text = Zanim katalog się włączy, deadloader pokaże, z jakimi hostami będzie się łączyć, i warunki GameBanana.
settings-consent-go = Przejrzyj i włącz
settings-local-on = Tryb lokalny jest włączony.
settings-local-on-text = Katalog jest wyłączony, a deadloader nie wysyła żądań sieciowych.
settings-ttl = Używaj wczytanych stron katalogu przez
settings-ttl-help = W tym czasie powrót do strony nie pyta ponownie GameBanana.
settings-minutes = { $count } min
settings-hours = { $count } godz.
settings-prefetch = Wczytuj następną stronę z wyprzedzeniem
settings-prefetch-help = Gdy przewiniesz około 70% wyników, wczytywane są kolejne, żeby siatka się nie kończyła.
settings-nsfw-group = Treści wrażliwe
settings-nsfw = Mody z klasyfikacją treści (NSFW)
settings-nsfw-help = Ukryte: w ogóle niewidoczne. Rozmyte: rozmyty obraz z etykietą „Treść wrażliwa”; kliknij, aby pokazać. Widoczne: bez rozmycia.
settings-nsfw-age-done = Wiek potwierdzono { $date }.
settings-nsfw-age-ask = Opuszczenie opcji „Ukryte” raz poprosi o potwierdzenie wieku.
settings-nsfw-hidden = Ukryte
settings-nsfw-blurred = Rozmyte
settings-nsfw-shown = Widoczne
settings-images-group = Obrazy i linki
settings-description-images = Obrazy w opisach modów
settings-description-images-help = Pochodzą z hostów innych niż GameBanana, np. i.imgur.com. Po kliknięciu: zastępnik, dopóki go nie klikniesz.
settings-images-click = Po kliknięciu
settings-images-always = Zawsze
settings-prechecked = Wstępnie zaznaczaj „Zawsze zezwalaj na obrazy z tego hosta”
settings-prechecked-help = W oknie, które pojawia się, gdy obraz z opisu po raz pierwszy pochodzi z nowego hosta.
settings-hosts = Dozwolone hosty obrazów
settings-hosts-help = Obrazy z tych hostów wczytują się bez pytania. W Aktywności ich żądania są oznaczone jako „dozwolony host obrazów”.
settings-hosts-built-in = Wbudowany
settings-hosts-none = Brak dodanych hostów — każdy nowy host najpierw pyta.
settings-host-removed = Usunięto { $host }
settings-host-removed-text = Jego obrazy znów zapytają przed wczytaniem.
settings-undo = Cofnij
settings-gif = Odtwarzaj GIF-y automatycznie
settings-gif-help = Wyłączone: GIF-y pokazują pierwszą klatkę z przyciskiem odtwarzania.
settings-links = Linki zewnętrzne
settings-links-help = Linki otwierają się w domyślnej przeglądarce. „Najpierw pytaj” pokazuje pełny adres przed otwarciem.
settings-links-ask = Najpierw pytaj
settings-links-open = Otwieraj od razu

# Library
settings-updates-group = Aktualizacje modów
settings-check-updates = Sprawdzaj aktualizacje modów przy uruchomieniu
settings-check-updates-help = Porównuje twoje mody z GameBanana z katalogiem. Nigdy w trybie lokalnym.
settings-auto-download = Pobieraj aktualizacje automatycznie
settings-auto-download-help = Tylko pobieranie — deadloader i tak zapyta, zanim mod przejdzie na nową wersję.
settings-disk-group = Usuwanie i stare wersje
settings-deletion = Gdy usuwasz mod
settings-deletion-help = W oknie usuwania możesz za każdym razem wybrać inaczej.
settings-deletion-permanent = Trwale
settings-deletion-bin = Do kosza
settings-old-versions = Stare wersje
settings-old-versions-help = Każda pobrana wersja jest zachowywana, żebyś mógł do niej wrócić. Stare wersje moda usuwasz w jego szczegółach.
settings-old-keep-all = Zachowaj wszystkie

# Downloads
settings-queue-group = Kolejka
settings-parallel = Równoczesne pobierania
settings-parallel-help = Ile plików pobiera się jednocześnie.
settings-speed = Limit prędkości
settings-speed-help = Tylko dla pobierania modów — katalog i miniatury nie są ograniczane.
settings-speed-none = Bez limitu
settings-speed-value = { $value } MB/s
settings-keep-archives = Zachowuj archiwa po instalacji
settings-keep-archives-help = Wyłączone: plik .zip, .7z lub .rar jest usuwany, gdy tylko jego pliki .vpk trafią do biblioteki.
settings-unpack-group = Rozpakowywanie
settings-limits = Limity rozpakowywania
settings-limits-help = Chronią przed archiwami, które rozpakowują się do czegoś ogromnego. Jeśli archiwum, któremu ufasz, zostanie odrzucone, podnieś limit.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = Rozmiar po rozpakowaniu
settings-limit-ratio = Stopień kompresji
settings-limit-files = Pliki modów na archiwum
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = plików
settings-unit-s = s
settings-number-error = { $field }: wpisz liczbę całkowitą od { $min } do { $max }. Do tego czasu obowiązuje poprzednia wartość.

# Network
settings-network-local = Tryb lokalny jest włączony.
settings-network-local-text = deadloader nie wysyła żądań sieciowych; te ustawienia zaczną działać, gdy katalog będzie włączony.
settings-requests-group = Żądania do GameBanana
settings-api-parallel = Równoczesne żądania do katalogu
settings-api-parallel-help = Żądania do API GameBanana w tym samym czasie. Obrazy i pobierania liczą się osobno.
settings-timeouts = Limity czasu
settings-timeouts-help = Połączenie: zrezygnuj, jeśli w tym czasie nie nawiązano połączenia. Odpowiedź: zrezygnuj, gdy serwer przestaje wysyłać.
settings-timeouts-default = 10 s · 30 s
settings-timeout-connect = Połączenie
settings-timeout-read = Odpowiedź
settings-retries = Ponowienia
settings-retries-help = Ile razy nieudane żądanie jest powtarzane, z rosnącymi przerwami, zanim deadloader pokaże błąd.
settings-connection-group = Połączenie
settings-proxy = Proxy
settings-proxy-help = Brak: połączenie bezpośrednie. Systemowe: proxy ustawione w Windows. Ręczne: wpisz je poniżej.
settings-proxy-none = Brak
settings-proxy-system = Systemowe
settings-proxy-manual = Ręczne
settings-proxy-host = Adres
settings-proxy-port = Port
settings-proxy-hint = Proxy HTTP. Przechodzą przez nie tylko żądania deadloader.
settings-proxy-invalid = Wpisz nazwę hosta lub adres bez nazwy użytkownika i hasła, np. proxy.example.com.

# Cache
settings-thumbs-group = Miniatury i strony katalogu
settings-cache-limit = Limit pamięci podręcznej
settings-cache-limit-help = Gdy pamięć podręczna jest pełna, jako pierwsze znikają miniatury, których najdłużej nie oglądałeś.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = Obecny rozmiar
settings-cache-breakdown = Miniatury: { $thumbnails } · strony katalogu: { $pages }
settings-cache-empty = Pusto — nic jeszcze nie zapisano.
settings-cache-used = { $used } z { $limit }
settings-cache-clear = Wyczyść pamięć podręczną
settings-cache-cleared = Pamięć podręczna wyczyszczona — zwolniono { $size }. Miniatury wczytają się ponownie, gdy będą potrzebne.
settings-cache-cleared-partly = Pliki w użyciu ({ $failed }) zostaną do następnego czyszczenia.

# Logs
settings-keep-group = Przechowywanie dzienników
settings-log-days = Przechowuj dzienniki przez
settings-log-days-help = Starsze wpisy są usuwane przy uruchomieniu deadloader.
settings-days = { $count ->
        [one] { $count } dzień
        [few] { $count } dni
        [many] { $count } dni
       *[other] { $count } dnia
    }
settings-year = 1 rok
settings-log-size = Maksymalny rozmiar
settings-log-size-help = Powyżej tego rozmiaru najpierw usuwane są najstarsze pliki dziennika.
settings-privacy-group = Prywatność
settings-mask = Ukrywaj nazwę użytkownika Windows
settings-mask-help = Ścieżki w dziennikach pokazują { $masked } zamiast twojej nazwy — bezpieczniej, gdy udostępniasz dziennik.
settings-logs-folder-group = Folder dzienników
settings-logs-folder = Gdzie są dzienniki
settings-logs-now = { $size } · plików: { $files } · najstarszy z { $date }
settings-open-logs = Otwórz folder dzienników

# Advanced
settings-advanced-warning = Dla doświadczonych użytkowników.
settings-advanced-warning-text = Błąd tutaj może zmienić działanie deadloader. Sprawdza on plik przed zapisaniem i zachowuje kopię poprzedniej wersji.
settings-files-group = Edycja plików ustawień
settings-file-settings = Ustawienia
settings-file-manifest = Manifest folderu gry
settings-file-label = Plik do edycji
settings-file-settings-lead = Wszystko z tych stron w jednym pliku. Zmiany pojawią się na stronach po zapisaniu.
settings-file-manifest-lead = Lista tego, co deadloader umieścił w folderze gry. Przywracanie na niej polega — błędny wpis może pozostawić pliki.
settings-json-error = To nie jest poprawny JSON, więc nie zapisano.
settings-json-where = Wiersz { $line }, kolumna { $column }: tutaj lub tuż przed czegoś brakuje albo jest za dużo — często przecinka lub cudzysłowu.
settings-json-fix = Popraw to miejsce i zapisz ponownie — do tego czasu deadloader używa obecnego pliku.
settings-json-refused = Tych wartości nie można użyć, więc nic nie zapisano.
settings-json-newer = Ten plik pochodzi z nowszego deadloader (format { $found }; ta wersja czyta do { $supported }), więc nie zapisano.
settings-json-invalid = deadloader nie może użyć tego pliku, więc go nie zapisano.
settings-json-details = Szczegóły
settings-problem-unknown-field = { $path }: to nie jest ustawienie tej wersji.
settings-problem-invalid-value = { $path }: tej wartości nie można tu użyć.
settings-problem-out-of-range = { $path }: { $found } jest poza zakresem { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: „{ $entry }” jest tu nieprawidłowe.
settings-problem-duplicate-entry = { $path }: „{ $entry }” występuje dwa razy.
settings-problem-needs-age = { $path }: pokazywanie treści NSFW wymaga najpierw potwierdzenia wieku (Ustawienia → Katalog).
settings-problem-local-mode = { $path }: niedostępne w trybie lokalnym.
settings-json-idle = Sprawdzane przy zapisie.
settings-json-saved = Poprawne i zapisane. Poprzednią wersję zachowano jako { $backup }.
settings-json-saved-plain = Poprawne i zapisane.
settings-json-save = Sprawdź i zapisz
settings-json-discard = Odrzuć moje zmiany
settings-json-backup = Przed zapisaniem poprzedni plik jest zachowywany jako kopia; zostaje 10 ostatnich.
settings-domains-group = Dodatkowe dozwolone domeny
settings-domains-lead = deadloader rozmawia tylko z GameBanana. Jeśli pobieranie zostanie przekierowane na inny host i zablokowane, możesz tu zezwolić na ten host. W Aktywności żądania do niego są oznaczone jako „dodane przez użytkownika”.
settings-domains-built-in = Wbudowane:
settings-domain-user = dodane przez użytkownika
settings-domains-none = Brak dodatkowych domen.
settings-domain-add = Dodaj domenę
settings-domain-hint = Tylko nazwa hosta, np. files.example.com.
settings-domain-empty = Najpierw wpisz nazwę hosta, np. files.example.com.
settings-domain-not-host = Wpisz tylko nazwę hosta — bez https:// i ścieżki, np. files.example.com.
settings-domain-invalid = „{ $host }” to nie jest nazwa hosta. Sprawdź pisownię, np. files.example.com.
settings-domain-built-in = { $host } jest wbudowany i zawsze dozwolony.
settings-domain-listed = { $host } jest już na liście.

# About
settings-version = wersja { $version }
settings-commit = Zbudowano z commita
settings-commit-local = Kompilacja lokalna — niezbudowana przez GitHub Actions
settings-copy-commit = Kopiuj identyfikator commita
settings-release-page = Strona wydania
settings-licence-group = Licencja
settings-licence-help = deadloader to wolne oprogramowanie: możesz go używać, badać, udostępniać i zmieniać na warunkach GPL-3. Jest dostarczany bez żadnej gwarancji.
settings-read-licence = Przeczytaj licencję
settings-source = Kod źródłowy
settings-verify-group = Jak sprawdzić to pobranie
settings-verify-lead = Dwie niezależne kontrole, że ten deadloader.exe jest tym opublikowanym na GitHubie.
settings-verify-hash = Porównaj sumę kontrolną SHA-256
settings-verify-hash-help = Musi zgadzać się z sumą na stronie wydania. Zmieniona kopia mogłaby pokazać tu fałszywą wartość, więc oblicz ją też sam:
settings-verify-hash-cmd = W PowerShell, w folderze z exe:
settings-verify-attest = Sprawdź atestację kompilacji
settings-verify-attest-help = Dowodzi, że GitHub Actions zbudował ten plik z opublikowanego kodu źródłowego. Wymaga GitHub CLI:
settings-copy-hash = Kopiuj sumę kontrolną
settings-copy-command = Kopiuj polecenie
settings-unsigned = deadloader.exe nie ma podpisu kodu.
settings-unsigned-text = Przy pierwszym uruchomieniu Windows SmartScreen może pokazać „System Windows ochronił ten komputer”. Dwie kontrole powyżej pozwalają potwierdzić, że plik jest oryginalny.
settings-webview-group = Silnik okna
settings-webview-help = Okno deadloader rysuje WebView2, część systemu Windows. Windows aktualizuje go sam. deadloader wyłącza:
settings-webview-off-1 = Zapytania DNS-over-HTTPS
settings-webview-off-2 = Logowanie kontem Microsoft w oknie
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Ruch sieciowy w tle i aktualizacje składników
settings-webview-off-5 = Wywołania serwera autouzupełniania formularzy
settings-webview-off-6 = Raporty błędów do Microsoft (domain reliability)
settings-webview-no-route = A okno w ogóle nie ma dostępu do internetu.
settings-webview-unknown = wersja nieznana
settings-data-group = Twoje dane
settings-data-folder = Folder danych
settings-data-folder-help = Wszystko, co przechowuje deadloader, jest w tym jednym folderze obok exe.
settings-notice = Informacja przy pierwszym uruchomieniu
settings-notice-help = Wersja { $version }, zaakceptowana { $date }, język: { $language }. Twoje wybory: katalog — { $catalog }, sprawdzanie aktualizacji — { $updates }.
settings-notice-none = Jeszcze niezaakceptowana.
settings-notice-again = Przeczytaj ponownie

# D7
settings-d7-title = Przywrócić folder gry na { $drive }?
settings-d7-from =
    { $others ->
        [0] Instalacja wybranego profilu { $profile }. deadloader usuwa z
       *[other] Instalacja wybranego profilu { $profile } (używana też przez { $others }). deadloader usuwa z
    }
settings-d7-files =
    { $count ->
        [one] { $count } plik moda, który dodał w
        [few] { $count } pliki modów, które dodał w
        [many] { $count } plików modów, które dodał w
       *[other] { $count } pliku modów, które dodał w
    }
settings-d7-line = Swój wiersz w gameinfo.gi
settings-d7-line-why = — plik wraca do stanu, w jakim zainstalował go Steam.
settings-d7-stays = Zostaje bez zmian:
settings-d7-library = Twoja biblioteka — mody: { $mods }, { $size }
settings-d7-profiles =
    { $count ->
        [one] Twój profil — { $names }
       *[other] Twoje profile ({ $count }) — { $names }
    }
settings-d7-other = Instalacja na { $path } ({ $profile }) — nieruszana. Aby ją przywrócić, wybierz profil, który jej używa, i przywróć ponownie.
settings-d7-other-plain = Instalacja na { $path } — nieruszana.
settings-d7-back = „Zastosuj” lub „Graj” wszystko przywróci.
settings-d7-foreign =
    { $count ->
        [one] Usuń też { $count } plik, którego nie utworzył deadloader
        [few] Usuń też { $count } pliki, których nie utworzył deadloader
        [many] Usuń też { $count } plików, których nie utworzył deadloader
       *[other] Usuń też { $count } pliku, których nie utworzył deadloader
    }
settings-d7-foreign-why = z innego programu lub instalacji ręcznej. Zostaną trwale usunięte; deadloader nie może ich odzyskać.
settings-d7-confirm = Przywróć
settings-d7-confirm-foreign =
    { $count ->
        [one] Przywróć i usuń { $count } plik
        [few] Przywróć i usuń { $count } pliki
        [many] Przywróć i usuń { $count } plików
       *[other] Przywróć i usuń { $count } pliku
    }

# D8
settings-d8-title = Usunąć wszystkie dane deadloader?
settings-d8-from = deadloader usuwa wszystko w
settings-d8-from-2 = a potem się zamyka.
settings-d8-what = Co
settings-d8-contains = Zawiera
settings-d8-size = Rozmiar
settings-d8-total = Razem
settings-d8-library = Biblioteka
settings-d8-library-has = mody: { $mods }, pliki: { $files }
settings-d8-profiles = Profile
settings-d8-logs = Dzienniki
settings-d8-logs-has = pliki: { $files }, od { $date }
settings-d8-logs-none = brak plików dziennika
settings-d8-cache = Pamięć podręczna
settings-d8-cache-has = miniatury i strony katalogu
settings-d8-settings = Ustawienia
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
       *[other] wraz z kopiami ({ $backups })
    }
settings-d8-other = Inne
settings-d8-other-has = niedokończone pobierania, kopie gameinfo.gi, dane silnika okna
settings-d8-exe = Nie usunięto: sam deadloader.exe — usuń go sam, jeśli już go nie potrzebujesz.
settings-d8-rollback = Najpierw przywróć każdy folder gry używany przez deadloader (zalecane)
settings-d8-rollback-why = Przed usunięciem usuwa pliki modów deadloader i jego wiersz w gameinfo.gi z każdego folderu gry. Obce pliki zostają.
settings-d8-rollback-off = Niezaznaczone: pliki modów deadloader zostaną w folderach gry, a Deadlock będzie je dalej wczytywać — bez deadloader, który mógłby je usunąć.
settings-d8-type-1 = Aby potwierdzić, wpisz
settings-d8-type-2 = { "" }
settings-d8-word = usuń
settings-d8-hint = Przycisk usuwania odblokuje się, gdy słowo się zgadza.
settings-d8-hint-ok = Potwierdzono. Tego nie da się cofnąć.
settings-d8-type-first = Najpierw wpisz „{ $word }”
settings-d8-deleting = Usuwanie…
settings-d8-confirm = Usuń i zamknij
settings-d8-confirm-rollback = Przywróć, usuń i zamknij

# Reset
settings-reset-title = Przywrócić wszystkie ustawienia?
settings-reset-body = Każde ustawienie wraca do wartości domyślnej, w tym dozwolone hosty obrazów i dodatkowe domeny.
settings-reset-keeps = Twoja biblioteka, profile i dzienniki pozostają nietknięte, a folder gry bez zmian.
settings-reset-backup = Kopia obecnych ustawień zostaje zachowana jako settings.json.bak-…
settings-reset-confirm = Przywróć ustawienia
settings-reset-done = Wszystkie ustawienia przywrócono do domyślnych.
settings-reset-done-text = Kopia: { $backup }
