# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Otworzyć ten link w przeglądarce?
catalog-link-body = Opuszcza deadloader i otwiera się w domyślnej przeglądarce.
catalog-link-always = Zawsze otwieraj linki od razu
catalog-link-cancel = Anuluj
catalog-link-open = Otwórz

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Ukryte
catalog-nsfw-blurred = Rozmyte
catalog-nsfw-shown = Widoczne
catalog-age-title = Pokazywać mody z treściami wrażliwymi?
catalog-age-body = Niektóre mody na GameBanana mają oznaczenia za nagość, tematykę seksualną lub podobne treści. Aby je zobaczyć, potwierdź, że masz co najmniej 18 lat lub jesteś pełnoletni(-a) tam, gdzie mieszkasz.
catalog-age-choice = Twój wybór: { $mode }.
catalog-age-once = deadloader pyta o to tylko raz. W każdej chwili możesz wrócić do „Ukryte”.
catalog-age-confirm = Jestem pełnoletni(-a)
catalog-age-cancel = Anuluj

## S2: the catalog grid
catalog-title = Katalog
catalog-section = Dział
catalog-section-mods = Mody
catalog-section-sounds = Dźwięki
catalog-search = Szukaj modów
catalog-search-sounds = Szukaj dźwięków
catalog-search-offline = Wyszukiwanie wymaga połączenia
catalog-search-off = Wyszukiwanie niedostępne — nie można odczytać katalogu
catalog-search-clear = Wyczyść wyszukiwanie
catalog-results =
    { $count ->
        [one] { $count } wynik
        [few] { $count } wyniki
       *[other] { $count } wyników
    }
catalog-sort = Sortuj
catalog-sort-newest = Najnowsze
catalog-sort-latest_updated = Ostatnio zaktualizowane
catalog-sort-most_downloaded = Najczęściej pobierane
catalog-sort-most_liked = Najbardziej lubiane
catalog-sort-most_viewed = Najczęściej oglądane
catalog-sort-most_commented = Najczęściej komentowane
catalog-sort-oldest = Najstarsze
catalog-sort-best = Najlepiej pasujące
catalog-sort-best-title = Podczas wyszukiwania kolejność wynika z dopasowania
catalog-category = Kategoria
catalog-category-all = Wszystkie kategorie
catalog-hero = Bohater
catalog-hero-all = Wszyscy bohaterowie
catalog-hero-filter = Znajdź bohatera
catalog-hero-none = Brak pasującego bohatera
catalog-filters = Filtry
catalog-filter-nsfw = Mody z oznaczeniami treści (NSFW)
catalog-filter-favourites = Tylko ulubione
catalog-filter-favourites-short = tylko ulubione
catalog-filter-installed = Ukryj zainstalowane
catalog-filter-installed-short = bez zainstalowanych
catalog-nsfw-caption-hidden = NSFW ukryte
catalog-nsfw-caption-blurred = NSFW rozmyte · kliknij kartę, by ją pokazać
catalog-nsfw-caption-shown = NSFW widoczne
catalog-loading = Wczytywanie modów z GameBanana…
catalog-loading-sounds = Wczytywanie dźwięków z GameBanana…
catalog-loading-more = Wczytywanie kolejnych…
catalog-more-failed = Nie udało się wczytać następnej strony.
catalog-grid = Mody
catalog-grid-sounds = Dźwięki
catalog-likes =
    { $count ->
        [one] { $count } polubienie
        [few] { $count } polubienia
       *[other] { $count } polubień
    }
catalog-favourite-add = Dodaj do ulubionych
catalog-favourite-remove = Usuń z ulubionych
catalog-sensitive = Treści wrażliwe
catalog-click-show = Kliknij, by pokazać
catalog-badge-installed = Zainstalowany
catalog-badge-update = Aktualizacja
catalog-card-installing = Instalowanie · { $percent } %
catalog-empty = Na razie nie ma tu modów
catalog-empty-search = Brak modów pasujących do „{ $text }”
catalog-empty-favourites = Na razie brak ulubionych
catalog-empty-filters = Włączone filtry: { $filters }. Wyłącz je, by szukać we wszystkich modach, albo spróbuj krótszego słowa.
catalog-empty-shorter = Spróbuj krótszego lub innego słowa.
catalog-clear-filters = Wyczyść filtry
catalog-rate-retry = deadloader spróbuje ponownie za { $seconds } s — mody poniżej nadal działają.
catalog-retry-now = Ponów teraz
catalog-try-again = Spróbuj ponownie
catalog-check-update = Sprawdź aktualizację deadloadera
catalog-details = Szczegóły · wpis w dzienniku
catalog-copy-id = Kopiuj identyfikator wpisu
catalog-open-activity = Otwórz w Aktywności
catalog-offline-empty-title = Brak połączenia — nic nie zapisano
catalog-offline-empty-text = deadloader nie może połączyć się z GameBanana, a ta część katalogu nie została wcześniej zapisana.
catalog-error-title = Nie można teraz pokazać katalogu.
catalog-local-title = Katalog jest wyłączony (tryb lokalny)
catalog-local-why = W trybie lokalnym deadloader nie wysyła żadnych zapytań sieciowych. Biblioteka, profile i Graj działają jak zwykle, a mody możesz dodawać z dysku.
catalog-local-do = Włącz katalog, by przeglądać i instalować mody z GameBanana. deadloader będzie wtedy łączył się tylko z GameBanana.
catalog-local-enable = Włącz katalog
catalog-local-library = Przejdź do biblioteki

## S3: the mod page
catalog-breadcrumb = Ścieżka nawigacji
catalog-back = Wróć do katalogu
catalog-page-loading = Wczytywanie strony moda z GameBanana…
catalog-page-offline = Brak połączenia — kopia zapisana { $date }.
catalog-page-offline-why = Instalowanie nowych plików wymaga połączenia; przełączanie pobranych wariantów nadal działa.
catalog-page-offline-empty-title = Brak połączenia — ta strona nie została zapisana
catalog-page-offline-empty-text = deadloader nie może połączyć się z GameBanana, a ta strona nie była wcześniej otwierana.
catalog-page-local-title = Ta strona moda nie została zapisana
catalog-page-local-text = W trybie lokalnym deadloader pokazuje tylko wcześniej zapisane strony modów.
catalog-removed-title = Ten mod został usunięty z GameBanana
catalog-removed-why = Został wstrzymany lub usunięty, więc deadloader nie może pokazać jego strony ani pobrać plików.
catalog-removed-keep = Twoja zainstalowana kopia ({ $copy }) zostaje w bibliotece i nadal działa — po prostu nie dostanie aktualizacji.
catalog-show-in-library = Pokaż w bibliotece
catalog-installed = Zainstalowany
catalog-by = autor: { $author }
catalog-version = Wersja { $version }
catalog-views =
    { $count ->
        [one] { $count } wyświetlenie
        [few] { $count } wyświetlenia
       *[other] { $count } wyświetleń
    }
catalog-downloads =
    { $count ->
        [one] { $count } pobranie
        [few] { $count } pobrania
       *[other] { $count } pobrań
    }
catalog-added = Dodano { $when }
catalog-updated = Zaktualizowano { $when }
catalog-favourite = Ulubiony
catalog-open-gamebanana = Otwórz w GameBanana
catalog-gallery-alt = { $name }, obraz { $index } z galerii
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Poprzedni obraz
catalog-gallery-next = Następny obraz
catalog-gallery-original = Zobacz oryginał
catalog-gallery-hidden = Obrazy modów z oznaczeniami treści są ukryte.
catalog-audio-preview = Ten dźwięk ma odsłuch w GameBanana.
catalog-description = Opis
catalog-description-more = Pokaż cały opis
catalog-description-less = Pokaż mniej
catalog-description-truncated = Pokazano tylko początek opisu.
catalog-opens-browser = otwiera się w przeglądarce
catalog-opens-here = Otwiera się w deadloaderze
catalog-image-from = Obraz z
catalog-image-click = kliknij, by wczytać
catalog-image-failed = nie udało się wczytać
catalog-image-alt = Obraz z opisu
catalog-youtube = Film w YouTube
catalog-spoiler = Spoiler — kliknij, by pokazać
catalog-spoiler-hide = Ukryj spoiler
catalog-files = Pliki
catalog-files-clean =
    { $count ->
        [one] Przeszedł skanowanie antywirusowe GameBanana
       *[other] Wszystkie ({ $count }) przeszły skanowanie antywirusowe GameBanana
    }
catalog-files-clean-dated =
    { $count ->
        [one] Przeszedł skanowanie antywirusowe GameBanana · przesłano { $date }
       *[other] Wszystkie ({ $count }) przeszły skanowanie antywirusowe GameBanana · przesłano { $date }
    }
catalog-files-av = Wynik antywirusa GameBanana jest podany przy każdym pliku.
catalog-variants-note = Te pliki zmieniają te same pliki gry, więc są wariantami — aktywny może być tylko jeden.
catalog-no-files = Ten mod nie ma plików do pobrania.
catalog-older-files = Starsze pliki ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } pobranie
        [few] { $count } pobrania
       *[other] { $count } pobrań
    }
catalog-file-active = aktywny wariant
catalog-file-downloaded = pobrany
catalog-file-install = Zainstaluj
catalog-file-install-label = Zainstaluj { $file } i ustaw jako aktywny wariant
catalog-file-install-title = Pobiera ten plik i ustawia go jako aktywny wariant
catalog-file-install-offline = Instalowanie wymaga połączenia
catalog-file-installing = Instalowanie…
catalog-file-installed = Zainstalowany
catalog-file-switch = Przełącz na
catalog-file-switch-label = Przełącz na { $file }
catalog-file-switch-title = Już pobrany — ustaw jako aktywny wariant
catalog-file-update = Aktualizuj
catalog-file-update-title = Nowsza wersja zastępuje zainstalowany plik — sprawdź ją w bibliotece
catalog-av-clean = Czysty
catalog-av-clean-title = Antywirus GameBanana: czysty
catalog-av-flagged = Oznaczony
catalog-av-flagged-title = Antywirus GameBanana: { $verdict }
catalog-av-pending = Jeszcze nie sprawdzony
catalog-av-pending-title = GameBanana jeszcze nie skończył skanowania tego pliku
catalog-av-none = Brak skanu
catalog-av-none-title = GameBanana nie przesłał wyniku antywirusa dla tego pliku
catalog-credits = Autorzy
catalog-no-ratings = Brak oznaczeń treści
catalog-no-licence = Brak tekstu licencji
catalog-no-sources = Brak innych źródeł
catalog-rating-sexual_themes = Tematyka seksualna
catalog-rating-sexual_content = Treści seksualne
catalog-rating-skimpy_attire = Skąpy strój
catalog-rating-lewd_angles_and_poses = Wyzywające ujęcia i pozy
catalog-rating-full_nudity = Pełna nagość
catalog-licence = Licencja
catalog-sources = Inne źródła
catalog-videos = Filmy

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Wczytać obraz z nowego serwera?
catalog-image-body-before = deadloader pobierze ten obraz z
catalog-image-body-after = . To nie jest GameBanana — zapytanie pojawi się w Aktywności jak każde inne.
catalog-image-note = Rozmiar obrazów jest ograniczony. Dozwolone serwery możesz usunąć w Ustawienia → Katalog.
catalog-image-always = Zawsze zezwalaj na obrazy z tego serwera
catalog-image-cancel = Anuluj
catalog-image-load = Wczytaj obraz
