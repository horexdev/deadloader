# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Postęp konfiguracji
wizard-caption-setup = Pierwsza konfiguracja
wizard-caption-changed = Zaktualizowana informacja
wizard-caption-catalog = Włączanie katalogu
wizard-caption-review = Informacja przy pierwszym uruchomieniu
wizard-language = Język
wizard-step-welcome = Powitanie
wizard-step-risks = Ryzyko
wizard-step-network = Sieć
wizard-step-privacy = Prywatność
wizard-step-updates = Aktualizacje
wizard-step-setup = Konfiguracja
wizard-step-of = Krok { $n } z 6
wizard-changed-of =
    { $count ->
        [one] { $count } zmieniony krok
        [few] { $count } zmienione kroki
       *[other] { $count } zmienionych kroków
    }
wizard-meta-changed = Zmieniony
wizard-meta-kept = Bez zmian
wizard-back = Wstecz
wizard-next = Dalej
wizard-finish = Zakończ konfigurację
wizard-continue = Kontynuuj
wizard-close = Zamknij
wizard-cancel = Anuluj
wizard-to-catalog = Następnie otworzy się katalog.
wizard-to-library = Następnie otworzy się biblioteka (tryb lokalny).

# Step 1
wizard-s1-title = Witaj w deadloader
wizard-s1-lead = deadloader instaluje mody do Deadlock z GameBanana lub z twojego dysku i pokazuje każdą zmianę, którą wprowadza.
wizard-f1a = Dodaje pliki modów do folderu modów gry i trzy linie do
wizard-f1b = — pliku gry, który każe Deadlock wczytywać mody.
wizard-f2 = Wszystko da się cofnąć: przywrócenie usuwa z gry wszystko, co dodał deadloader.
wizard-f3a = Wszystkie jego dane zostają w folderze
wizard-f3b = obok
wizard-f3c = .
wizard-f4 = deadloader nie jest powiązany z Valve ani GameBanana.
wizard-f5 = Wolne oprogramowanie na licencji GPL-3, bez jakiejkolwiek gwarancji.
wizard-licence = Licencja (GPL-3)
wizard-source = Kod źródłowy

# Step 2
wizard-s2-title = Mody na własne ryzyko
wizard-r1 = Valve nie ma oficjalnych zasad dotyczących modów w Deadlock. Korzystanie z modów to twoja decyzja i twoje ryzyko.
wizard-r2 = Aktualizacje gry mogą psuć mody. deadloader konfiguruje się ponownie po aktualizacji, ale mod może nie działać, dopóki autor go nie zaktualizuje.
wizard-r3 = Mody tworzą inni ludzie na własnych licencjach. deadloader nie może ręczyć za ich zawartość; przy każdym pliku pokazuje wynik skanowania antywirusowego GameBanana.
wizard-agree = Rozumiem, że deadloader zmienia pliki gry i że używam modów na własne ryzyko.
wizard-agree-note = Tym potwierdzasz kroki 1 i 2. Bez tego nie można kontynuować.
wizard-agree-error = Zaznacz to pole, aby kontynuować.
wizard-agree-reason = Najpierw zaznacz pole.

# Step 3
wizard-s3-title = Sieć i GameBanana
wizard-s3-lead = Katalog łączy się bezpośrednio z nieoficjalnym API GameBanana — i z nikim więcej. To jedyne hosty, z którymi się kontaktuje:
wizard-hosts = Hosty
wizard-host-api = katalog i strony modów
wizard-host-images = miniatury i galeria
wizard-host-files = pobrania
wizard-host-cache = pobrania (files.gamebanana.com przekierowuje tutaj)
wizard-host-new = Nowy
wizard-s3-images = Obrazy z innych hostów w opisach modów wczytują się dopiero, gdy zezwolisz na każdy host. Każde żądanie widać w zakładce
wizard-s3-images-end = .
wizard-activity = Aktywność
wizard-s3-tos = Korzystanie z katalogu oznacza korzystanie z GameBanana: obowiązują ich warunki korzystania, a GameBanana wymaga, aby użytkownicy mieli co najmniej 13 lat.
wizard-tos = Warunki korzystania z GameBanana
wizard-choice = Jak chcesz zacząć?
wizard-use-catalog = Używaj katalogu
wizard-use-catalog-text = Przeglądaj i instaluj mody z GameBanana. deadloader łączy się tylko z hostami powyżej.
wizard-local = Nie teraz — tryb lokalny
wizard-local-text = Żadnych żądań sieciowych; mody tylko z dysku. Katalog możesz włączyć później w ustawieniach.
wizard-choice-error = Wybierz jedną z dwóch opcji.
wizard-choice-reason = Najpierw wybierz opcję.
wizard-last-time = Twój poprzedni wybór
wizard-changed-title = Co się zmieniło ({ $date })
wizard-changed-foot = Pozostałe kroki potwierdzono { $date }; nie zmieniły się.

# Step 4
wizard-s4-title = Twoja prywatność
wizard-p1 = Bez konta i bez telemetrii. deadloader nie wysyła niczego poza żądaniami do GameBanana z poprzedniego kroku.
wizard-p1-local = Bez konta i bez telemetrii. Wybrano tryb lokalny, więc deadloader nie wysyła zupełnie nic.
wizard-p2a = Dzienniki zostają na tym komputerze, w folderze
wizard-p2b = — i ukrywają twoją nazwę użytkownika Windows:
wizard-p3 = Okno deadloader rysuje Microsoft WebView2, część systemu Windows. deadloader wyłącza własne funkcje online WebView2; Windows aktualizuje WebView2 niezależnie od deadloader.
wizard-p4 = Co zrobił deadloader, zawsze sprawdzisz w zakładce
wizard-p4-end = .

# Step 5
wizard-s5-title = Aktualizacje deadloader
wizard-u1 = Przy uruchomieniu deadloader może sprawdzać na GitHubie, czy jest jego nowsza wersja.
wizard-updates-label = Sprawdzaj na GitHubie nowe wersje deadloader przy uruchomieniu
wizard-updates-on = Wł.
wizard-updates-off = Wył. (domyślnie)
wizard-updates-na = Niedostępne w trybie lokalnym
wizard-u2a = Gdy włączone, deadloader przy każdym uruchomieniu wysyła jedno żądanie do
wizard-u2b = i informuje, czy jest nowa wersja.
wizard-u3 = Aktualizacja jest zawsze ręczna: nową wersję pobierasz z GitHuba samodzielnie. deadloader nigdy sam się nie podmienia.
wizard-u4 = Niedostępne: tryb lokalny nie wysyła żadnych żądań sieciowych. Opcja stanie się dostępna po włączeniu katalogu w ustawieniach.
wizard-u5 = Możesz to zmienić później w Ustawienia → Ogólne.

# Step 6
wizard-s6-title = Konfiguracja
wizard-s6-lead = deadloader poszukał gry i sprawdził kilka rzeczy. Nic jeszcze nie zostało zmienione, a wszystko to możesz zmienić później w ustawieniach.
wizard-checking = Sprawdzanie…
wizard-topic-game = Gra
wizard-topic-exe = Położenie deadloader
wizard-topic-disk = Miejsce na dysku
wizard-topic-foreign = Folder modów gry
wizard-topic-gameinfo = Plik gry dla modów
wizard-game-found = Znaleziono Deadlock
wizard-game-chosen = Wybrano folder
wizard-change = Zmień…
wizard-game-missing = Nie znaleziono Deadlock
wizard-game-missing-text = Steam lub gra nie są zainstalowane albo folder gry został przeniesiony. Wybierz folder gry samodzielnie — teraz lub później w Ustawienia → Gra.
wizard-browse = Przeglądaj…
wizard-browse-title = Wybierz folder Deadlock
wizard-help = Pomoc
wizard-help-a = W Steam: kliknij prawym przyciskiem Deadlock → Zarządzaj → Przeglądaj pliki lokalne. Wybierz ten folder; zwykle kończy się na
wizard-help-b = .
wizard-exe-ok = W stałym folderze
wizard-exe-downloads = deadloader jest w folderze Pobrane
wizard-exe-desktop = deadloader jest na pulpicie
wizard-exe-warn-a = Jego dane leżą obok exe, więc sprzątanie usunęłoby twoją bibliotekę i profile. Zamknij deadloader, przenieś cały folder w stałe miejsce, np.
wizard-exe-warn-b = , i uruchom go stamtąd.
wizard-open-folder = Otwórz folder
wizard-opened = Otwarto w Eksploratorze
wizard-linked = Mody są linkowane — bez dodatkowego miejsca na dysku
wizard-linked-text = deadloader i gra są na tym samym dysku ({ $drive }).
wizard-copied = Mody będą kopiowane, nie linkowane
wizard-copied-text = deadloader jest na { $data }, a gra na { $game }. Każdy włączony mod zajmie wtedy miejsce drugi raz; biblioteka pokazuje to jako „skopiowano — +N MB”.
wizard-learn-more = Więcej informacji
wizard-drives-more = Link pozwala grze używać pliku z biblioteki bez drugiej kopii. Windows potrafi linkować tylko pliki na tym samym dysku. Jeśli deadloader też będzie na { $game }, nic nie zostanie skopiowane.
wizard-foreign-none = Brak obcych plików modów
wizard-foreign-none-text = W folderze modów nie ma nic, czego deadloader nie zna.
wizard-foreign-some =
    { $count ->
        [one] { $count } plik z innego źródła
        [few] { $count } pliki z innego źródła
       *[other] { $count } plików z innego źródła
    }
wizard-foreign-some-text = Nie umieścił ich tam deadloader — prawdopodobnie inny menedżer modów lub ręczna instalacja. Zostawi je bez zmian; przejrzyj je później w bibliotece.
wizard-show-files = Pokaż pliki
wizard-hide-files = Ukryj pliki
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Zostanie skonfigurowany
wizard-gi-setup-a = deadloader doda trzy linie do
wizard-gi-setup-b = przy pierwszym zastosowaniu modów. Teraz nic się nie zmienia.
wizard-gi-ours = Już skonfigurowany
wizard-gi-ours-text = Linie deadloader już tam są.
wizard-gi-foreign = Zmieniony przez inny program
wizard-gi-foreign-text = Nie wygląda jak oryginał — często robi to Deadlock Mod Manager. deadloader nie nadpisuje cudzych zmian i po konfiguracji wyjaśni, co zrobić.
wizard-what-is = Co to jest?
wizard-gi-more = to plik gry, który mówi Deadlock, jakie foldery wczytać. Zwykle pomaga weryfikacja plików gry w Steam; potem deadloader doda swoje linie.

# E1
wizard-nw-title = deadloader nie może zapisać swoich danych.
wizard-nw-why-a = Folder obok
wizard-nw-why-b = jest tylko do odczytu: Windows nie pozwala programom tam zapisywać (na przykład w Program Files).
wizard-nw-path = Folder deadloader
wizard-nw-do = Co zrobić
wizard-nw-s1 = Zamknij deadloader.
wizard-nw-s2a = Przenieś cały folder w miejsce, gdzie można zapisywać, na przykład
wizard-nw-s2b = . Nie używaj Program Files.
wizard-nw-s3a = Uruchom
wizard-nw-s3b = z nowego miejsca.
wizard-nw-why-folder = deadloader trzyma wszystko obok exe, żeby nic nie rozchodziło się po komputerze: usunięcie folderu całkowicie usuwa deadloader.
wizard-nw-details = Szczegóły
wizard-nw-copy = Kopiuj szczegóły
wizard-nw-copied = Skopiowano
wizard-nw-no-log = Nie zapisano w dzienniku — deadloader nie może tam zapisywać.
wizard-exit = Zakończ
