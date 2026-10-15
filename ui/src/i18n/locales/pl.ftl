## Notifications

n1 = Deadlock został zaktualizowany — mody przywrócone.
    .why = Aktualizacja zresetowała plik gameinfo.gi, więc deadloader ponownie dodał do niego swój wiersz.
n2 = { $name } — zainstalowano i włączono.
n3 =
    { $count ->
        [one] Dostępna jest { $count } aktualizacja moda.
        [few] Dostępne są { $count } aktualizacje modów.
        [many] Dostępnych jest { $count } aktualizacji modów.
       *[other] Dostępne jest { $count } aktualizacji modów.
    }
n4 = Deadlock jest uruchomiony.
    .why = Zmiany zostaną zastosowane automatycznie po zamknięciu gry.
    .pending =
        { $count ->
            [one] { $count } zmiana oczekuje
            [few] { $count } zmiany oczekują
            [many] { $count } zmian oczekuje
           *[other] { $count } zmiany oczekuje
        }
n5 =
    { $reason ->
        [completed] Przerwana zmiana została dokończona.
       *[undone] Przerwana zmiana została cofnięta; folder gry jest taki jak wcześniej.
    }
n6 =
    { $count ->
        [one] W folderze modów gry znaleziono { $count } plik, którego nie utworzył deadloader.
        [few] W folderze modów gry znaleziono { $count } pliki, których nie utworzył deadloader.
        [many] W folderze modów gry znaleziono { $count } plików, których nie utworzył deadloader.
       *[other] W folderze modów gry znaleziono { $count } pliku, których nie utworzył deadloader.
    }
n7 =
    { $when ->
        [today] Brak połączenia — wyświetlany jest katalog zapisany o { $saved }.
       *[earlier] Brak połączenia — wyświetlany jest katalog zapisany { $saved }.
    }
    .why = Pobieranie zostanie wznowione, gdy znów będziesz online.
n8 =
    { $count ->
        [one] Zastosowano profil „{ $name }” — włączono { $count } mod.
        [few] Zastosowano profil „{ $name }” — włączono { $count } mody.
        [many] Zastosowano profil „{ $name }” — włączono { $count } modów.
       *[other] Zastosowano profil „{ $name }” — włączono { $count } moda.
    }
n9 = Dostępna jest nowa wersja programu deadloader: { $version }.
n10 = Tryb lokalny jest włączony.
    .why = Katalog jest wyłączony, a deadloader nie wysyła żadnych żądań sieciowych.
n11 = Mody są kopiowane zamiast łączenia, ponieważ deadloader i gra znajdują się na różnych dyskach.
    .why = Zajmuje to dodatkowo { $size } miejsca na dysku.
n12 =
    { $count ->
        [one] Deadlock został zamknięty — zastosowano { $count } zmianę.
        [few] Deadlock został zamknięty — zastosowano { $count } zmiany.
        [many] Deadlock został zamknięty — zastosowano { $count } zmian.
       *[other] Deadlock został zamknięty — zastosowano { $count } zmiany.
    }

## Errors: what happened, why, what to do

e1 = deadloader nie może zapisać swoich danych.
    .why =
        { $reason ->
            [access_denied] Folder obok deadloader.exe jest tylko do odczytu — na przykład Windows nie pozwala programom zapisywać w folderze Pliki programów (Program Files).
            [in_use] Inny program, często antywirus, otworzył jeden z plików programu deadloader.
           *[other] Windows nie mógł zapisać danych w folderze obok deadloader.exe.
        }
    .todo =
        { $reason ->
            [in_use] Zamknij inne programy i spróbuj ponownie.
           *[other] Przenieś deadloader do folderu, w którym możesz zapisywać, na przykład D:\Games\deadloader.
        }
e2 = Nie znaleziono gry Deadlock.
    .why = Steam lub gra nie są zainstalowane albo folder gry został przeniesiony.
    .todo = Wybierz folder gry ręcznie.
e3 = Plik gry został zmieniony przez inny program.
    .why = Plik gameinfo.gi nie wygląda jak oryginał — często zmienia go Deadlock Mod Manager. deadloader nie nadpisuje cudzych zmian.
    .todo = Zweryfikuj spójność plików gry w Steam; potem deadloader doda swój wiersz.
e4 = Katalog jest niedostępny.
    .why = GameBanana zmieniło swoje API, więc deadloader nie może go odczytać. Biblioteka i profile działają normalnie.
    .todo = Sprawdź, czy jest dostępna aktualizacja programu deadloader, albo spróbuj ponownie później.
e5 = GameBanana jest przeciążone.
    .why =
        { $reason ->
            [rate_limited] Zbyt wiele żądań w krótkim czasie (HTTP 429).
            [server] Serwer GameBanana zgłosił błąd.
           *[other] Zbyt wiele żądań lub błąd serwera.
        }
    .todo =
        { $wait ->
            [seconds]
                { $seconds ->
                    [one] deadloader automatycznie spróbuje ponownie za { $seconds } sekundę.
                    [few] deadloader automatycznie spróbuje ponownie za { $seconds } sekundy.
                    [many] deadloader automatycznie spróbuje ponownie za { $seconds } sekund.
                   *[other] deadloader automatycznie spróbuje ponownie za { $seconds } sekundy.
                }
           *[later] deadloader za chwilę automatycznie spróbuje ponownie.
        }
e6 = Pobieranie nie powiodło się.
    .why =
        { $reason ->
            [connection] Połączenie zostało przerwane.
            [timeout] Serwer zbyt długo nie odpowiadał.
            [server] Serwer GameBanana zgłosił błąd.
           *[other] Pobieranie zostało przerwane.
        }
    .todo = Zostanie wznowione od miejsca, w którym się zatrzymało.
e7 = Pobrany plik jest uszkodzony.
    .why = Jego suma kontrolna nie zgadza się z podaną w GameBanana.
    .todo = Pobierz go ponownie.
e8 = Nie można rozpakować archiwum.
    .why =
        { $reason ->
            [not_an_archive] Plik nie jest archiwum ZIP, 7z ani RAR — być może nie został pobrany w całości.
            [damaged] Archiwum jest uszkodzone.
            [encrypted] Archiwum jest chronione hasłem.
            [unsupported] Używa metody kompresji, której deadloader nie obsługuje.
            [not_a_vpk] { $entry } nie jest prawidłowym plikiem .vpk.
           *[other] Jest uszkodzone lub ma nieobsługiwany format.
        }
    .todo = Spróbuj innego pliku tego moda albo zgłoś problem jego autorowi.
e9 = Archiwum jest zbyt duże, by bezpiecznie je rozpakować.
    .why =
        { $reason ->
            [too_large] Po rozpakowaniu przekracza { $limit }.
            [too_many_files]
                { $found ->
                    [one] Zawiera { $found } plik moda, a limit to { $limit }.
                    [few] Zawiera { $found } pliki modów, a limit to { $limit }.
                    [many] Zawiera { $found } plików modów, a limit to { $limit }.
                   *[other] Zawiera { $found } pliku modów, a limit to { $limit }.
                }
            [ratio] { $entry } jest podejrzanie mocno skompresowany — ponad { $limit }-krotnie.
            [memory] Rozpakowanie wymaga { $needed } pamięci, a limit to { $limit }.
           *[other] Przekracza jeden z limitów bezpieczeństwa dla archiwów.
        }
    .todo =
        { $reason ->
            [memory] Spróbuj innego pliku tego moda.
           *[other] Jeśli mu ufasz, zwiększ limit w sekcji Ustawienia → Pobieranie.
        }
e10 = W tym archiwum nie ma plików modów.
    .why =
        { $listing ->
            [files] Nie zawiera plików .vpk, tylko { $contents }.
           *[empty] Archiwum jest puste.
        }
    .todo = Ten plik nie jest modem do gry Deadlock w formacie, który deadloader potrafi zainstalować.
e11 = Na tym komputerze nie można rozpakować archiwów RAR.
    .why = Wbudowane narzędzie systemu Windows nie potrafi odczytać tego pliku RAR, a nie znaleziono ani 7-Zip, ani WinRAR.
    .todo = Zainstaluj 7-Zip (jest darmowy) i spróbuj ponownie; pobrany plik zostanie zachowany.
e12 = Za mało miejsca na dysku.
    .why =
        { $reason ->
            [space] Potrzeba { $needed }, a wolne jest tylko { $available }.
           *[full] Dysk jest pełny.
        }
    .todo = Zwolnij trochę miejsca albo usuń stare wersje modów w Bibliotece — a potem spróbuj ponownie.
e13 = Nie można zastosować zmian, gdy Deadlock jest uruchomiony.
    .why = Windows blokuje pliki gry, dopóki ona działa.
    .todo = Zamknij grę Deadlock; deadloader zastosuje zmiany zaraz potem.
e14 = W bibliotece brakuje pliku moda lub jest on uszkodzony.
    .why = Został zmieniony lub usunięty poza programem deadloader.
    .todo = Napraw moda, aby pobrać plik ponownie, albo usuń moda.
e15 = Nie udało się otworzyć linku.
    .why = W systemie Windows nie ustawiono domyślnej przeglądarki.
    .todo = Skopiuj link i otwórz go samodzielnie.
e16 = Żądanie zablokowane.
    .why = GameBanana przekierowało do { $host }, a tego hosta nie ma na liście dozwolonych hostów.
    .todo = Nie musisz nic robić — deadloader komunikuje się tylko z GameBanana. Jeśli ufasz temu hostowi, dodaj go w sekcji Ustawienia → Zaawansowane.
e17 = Nie udało się zaimportować profilu.
    .why =
        { $reason ->
            [newer] Został utworzony w nowszej wersji programu deadloader.
            [unreadable] Nie można odczytać pliku — mógł zostać przeniesiony, usunięty lub otwarty przez inny program.
           *[not_a_profile] Ten plik nie jest profilem programu deadloader.
        }
    .todo =
        { $reason ->
            [newer] Zaktualizuj deadloader, a potem ponownie zaimportuj profil.
            [unreadable] Sprawdź, czy plik nadal istnieje i czy żaden inny program go nie otworzył, a potem spróbuj ponownie.
           *[not_a_profile] Sprawdź, czy to właściwy plik.
        }
e18 = Nie udało się zmienić plików w folderze gry.
    .why =
        { $reason ->
            [access_denied] Windows odmówił dostępu do folderu gry.
            [in_use] Plik jest używany przez inny program, na przykład antywirus lub inny menedżer modów.
           *[other] Windows nie mógł zmienić plików.
        }
    .todo = Zamknij inne programy i spróbuj ponownie.
e19 = Nie udało się wczytać obrazu.
    .why =
        { $reason ->
            [no_response] Host nie odpowiedział.
            [not_an_image] Ten plik nie jest obrazem.
            [too_large] Obraz jest zbyt duży.
           *[other] Host nie przesłał prawidłowego obrazu.
        }
    .todo = Symbol zastępczy pozostanie; możesz spróbować ponownie.
e20 = Włączono zbyt wiele modów.
    .why =
        { $free ->
            [one] Gra może wczytać jeszcze { $free } plik moda, a ten profil potrzebuje { $wanted }.
            [few] Gra może wczytać jeszcze { $free } pliki modów, a ten profil potrzebuje { $wanted }.
            [many] Gra może wczytać jeszcze { $free } plików modów, a ten profil potrzebuje { $wanted }.
           *[other] Gra może wczytać jeszcze { $free } pliku modów, a ten profil potrzebuje { $wanted }.
        }
    .todo = Wyłącz część modów lub wybierz mniej dodatków.
e21 = Profil o nazwie „{ $name }” już istnieje.
    .why = Nazwy są porównywane bez rozróżniania wielkości liter, tak jak nazwy plików.
    .todo = Wybierz inną nazwę.
e22 = Nazwy „{ $name }” nie można użyć dla profilu.
    .why = Windows nie zezwala na taką nazwę pliku — na przykład CON, kropkę na końcu lub dowolny ze znaków \ / : * ? " < > |.
    .todo = Wybierz inną nazwę.
e23 = deadloader nie może odczytać swojego pliku { $file }.
    .why =
        { $reason ->
            [damaged_at] Plik jest uszkodzony (wiersz { $line }, kolumna { $column }).
            [damaged] Plik jest uszkodzony.
            [newer] Został utworzony w nowszej wersji programu deadloader.
           *[invalid] Jego zawartość nie jest taka, jakiej oczekuje deadloader.
        }
    .todo =
        { $kind ->
            [settings] Na razie deadloader używa ustawień domyślnych i pozostawia plik bez zmian.
            [manifest] Przywróć plik manifest.json.bak albo zaktualizuj deadloader.
           *[other] Przywróć plik z kopii zapasowej albo zaktualizuj deadloader.
        }
internal = W programie deadloader coś poszło nie tak.
    .why = To błąd programu deadloader, a nie Twój.
    .todo = Spróbuj ponownie. Jeśli to się powtórzy, zgłoś problem razem z wpisem dziennika ze Szczegółów.

## Buttons named in the texts above

action-undo = Cofnij
action-review = Przejrzyj
action-retry = Ponów
action-retry-now = Ponów teraz
action-browse = Przeglądaj…
action-verify-in-steam = Zweryfikuj w Steam
action-show-differences = Pokaż różnice
action-details = Szczegóły
action-repair = Napraw
action-copy-link = Kopiuj link
action-open-folder = Otwórz folder
action-open-release-page = Otwórz stronę wydania
action-enable-catalog = Włącz katalog
action-learn-more = Dowiedz się więcej
action-view-log = Pokaż dziennik

## Shared pieces

log-entry = Wpis dziennika { $id }
list-more = jeszcze { $count }

## Sizes, labelled like Windows Explorer labels them

size-bytes =
    { $count ->
        [one] { $count } bajt
        [few] { $count } bajty
        [many] { $count } bajtów
       *[other] { $count } bajta
    }
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
