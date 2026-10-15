# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Zamknij
library-apply = Zastosuj
library-apply-profile = Zastosuj { $name }
library-discard-all = Odrzuć wszystkie
library-keep = Zostaw { $name }
library-keep-applied = Zostaw zastosowane zmiany
library-undo-this = Cofnij tę zmianę
library-undo-all = Cofnij wszystkie
library-close-to-apply = Zamknij Deadlocka — zmiany zostaną wtedy zastosowane automatycznie
library-play-applies = Graj też go zastosuje
library-applied-title = Zmiany zastosowane
library-nothing-pending = Nic nie oczekuje
library-all-in-game = Wszystko z profilu { $name } jest już w grze.
library-running-sub = Deadlock jest uruchomiony, więc Windows blokuje jego pliki. Te zmiany zostaną zastosowane automatycznie, gdy tylko zamkniesz grę.
library-waiting-sub = Tych zmian nie ma jeszcze w folderze gry. Zastosuj je teraz albo naciśnij Graj.
library-switch-sub = Zmiana profilu zmienia naraz wiele plików, więc czeka na ciebie. Zastosuj ją tutaj albo naciśnij Graj — deadloader zastosuje ją przed uruchomieniem.
library-applied-sub = Deadlock został zamknięty o { $time }, więc zmiany wprowadzone w trakcie gry ({ $count }) zostały zastosowane. Możesz je cofnąć do następnej zmiany.
library-undoing-sub = Cofnięto zmiany zastosowane o { $time } ({ $count }), a Deadlock znów działa. Zostaną przywrócone automatycznie, gdy zamkniesz grę.
library-group-enable = Włącz · { $count }
library-group-disable = Wyłącz · { $count }
library-group-order = Kolejność · { $count }
library-group-variant = Wariant · { $count }
library-group-version = Wersja · { $count }
library-group-switch = Zmiana profilu · { $count }
library-group-undo = Cofnięcie zmian zastosowanych o { $time } ({ $count })
library-change-on = Wył. → wł.
library-change-off = Wł. → wył.
library-change-above = Przeniesiony nad { $name }
library-change-above-wins = Przeniesiony nad { $name } — teraz wygrywa ich konflikt
library-change-below = Przeniesiony pod { $name }
library-switch-what-happens = Włączy się modów: { $on } · wyłączy się: { $off }
library-what-happens = Co się stanie w folderze gry
library-folder-summary = Dodanych plików: { $added } · usuniętych: { $removed } · przenumerowanych: { $renumbered }

## Foreign files
library-foreign-title = Znalezione w folderze modów gry
library-foreign-note = Nie umieścił ich tu deadloader i nie ruszy ich, chyba że tak zdecydujesz.
library-collapse = Zwiń
library-expand = Rozwiń
library-take = Przejmij zarządzanie
library-leave = Zostaw bez zmian
library-left = Zostawiono bez zmian
library-foreign-open = Otwórz folder z { $name }
library-foreign-vpk = VPK, plików: { $count }
library-foreign-vpk-hero = VPK, plików: { $count } — wygląda na skórkę dla { $hero }
library-foreign-unreadable = Plik moda, którego deadloader nie może odczytać
library-foreign-replaced = Plik, który zajął miejsce pliku deadloadera
library-foreign-dmm = Utworzony przez Deadlock Mod Manager — plik ustawień, nie mod
library-foreign-other = Plik, nie mod
library-foreign-folder = Folder, plików: { $count } — nie mod
library-foreign-meta = { $size } · dodano { $date }
library-taken = { $name } jest teraz w twojej bibliotece.

## D11
library-d11-title = Przejąć zarządzanie { $name }?
library-d11-lead = deadloader:
library-d11-step-copy = skopiuje go do biblioteki jako mod lokalny ({ $size }),
library-d11-step-replace = zastąpi plik w folderze gry własną połączoną kopią,
library-d11-step-enable = włączy go w profilu { $profile } na końcu listy.
library-d11-name = Nazwa w bibliotece
library-d11-hint = Biblioteka przechowuje dokładną kopię oryginalnego pliku.
library-d11-suggested = { $file } (skórka { $hero })

## D10
library-d10-title = Plik gry został zmieniony przez inny program
library-d10-what = Inny program zmienił plik gry, który określa, jakie foldery ładuje Deadlock — często jest to Deadlock Mod Manager. deadloader nie nadpisuje cudzych zmian, więc twoje mody nie załadują się, dopóki to nie zostanie naprawione.
library-d10-do = Pozwól Steam przywrócić oryginalny plik (Weryfikuj pliki gry). Potem deadloader doda swój wiersz i da ci znać.
library-d10-diff = Pokaż różnice
library-d10-expected = Oczekiwane (oryginał + wiersz deadloadera)
library-d10-found = Teraz w pliku
library-d10-diff-note = Wyróżnione: wiersz dodawany przez deadloader i wiersze dodane przez inny program.
library-d10-help = Co to za plik?
library-d10-help-text = gameinfo.gi mówi Deadlockowi, które foldery ładować. Menedżery modów dodają jeden wiersz, aby gra ładowała mody z folderu addons. Aktualizacje gry go resetują; deadloader naprawia to automatycznie — ale tylko gdy plik jest oryginalny.
library-d10-wait-title = Steam weryfikuje pliki Deadlocka.
library-d10-wait-text = Gdy Steam skończy, wróć do deadloadera — sprawdzi plik ponownie automatycznie. Możesz też sprawdzić teraz.
library-d10-fixed = gameinfo.gi jest skonfigurowany — mody znów się ładują.
library-d10-fixed-text = Steam przywrócił oryginalny plik, a deadloader dodał swój wiersz. Szczegóły w Aktywność → Pliki.
library-open-file = Otwórz plik
library-not-now = Nie teraz
library-verify-steam = Zweryfikuj pliki gry w Steam
library-check-again = Sprawdź ponownie
library-show-differences = Pokaż różnice

## D5
library-update = Aktualizuj
library-update-on-gb = Na GameBanana jest nowsza wersja.
library-later = Później
library-d5-title = Zaktualizować { $name }?
library-d5-installed = Zainstalowana
library-d5-new = Nowa na GameBanana
library-d5-downloaded = Pobrana
library-d5-arrived = W bibliotece, jeszcze nieaktywna
library-d5-new-meta = Zaktualizowana { $date } · { $size } do pobrania
library-d5-download = { $size } do pobrania
library-d5-changed = Nowa wersja ma inny zestaw plików. deadloader dopasował je tak:
library-d5-same = Te pliki zostaną zastąpione nowymi wersjami:
library-d5-map-text = deadloader nie mógł ustalić dla każdego pliku, który nowy go zastępuje. Wybierz nowy plik dla każdego albo zostaw stary.
library-d5-map-label = Dopasowanie plików
library-d5-col-old = Zainstalowany plik
library-d5-col-new = Nowy plik
library-d5-pick = Nowy plik dla { $file }
library-d5-choose = Wybierz…
library-d5-keep = Zostaw stary plik
library-d5-tag-replaces = Zastępuje
library-d5-tag-same = Ta sama nazwa
library-d5-tag-new = Nowy · na stronie moda
library-d5-profiles = { $new } stanie się aktywna w każdym profilu, który używa { $old }. { $old } zostaje w bibliotece, więc możesz do niej wrócić w szczegółach moda.
library-d5-install = Zainstaluj aktualizację
library-d5-confirm-map = Użyj tych plików
library-update-downloading = Pobieranie aktualizacji { $name } — zostanie zainstalowana po pobraniu.
library-updated = Zaktualizowano { $name }.
library-updated-text = Aktywny w każdym profilu, który używał starej wersji. Stara wersja zostaje — możesz do niej wrócić w szczegółach moda.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name }: włączono
library-n12-off = { $name }: wyłączono
library-n12-above = { $name }: przeniesiony nad { $other }
library-n12-below = { $name }: przeniesiony pod { $other }
library-n12-switched = { $name }: przełączono na { $to }
library-n12-files = { $name }: zmieniono pliki

## S9: the file list of a pending apply
library-show-file-list = Pokaż listę plików
library-hide-file-list = Ukryj listę plików
library-file-list = Pliki w folderze gry
library-files-added = Dodane · { $count }
library-files-removed = Usunięte · { $count }
library-files-renumbered = Przenumerowane · { $count }
