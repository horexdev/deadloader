# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Profile
profiles-count =
    { $count ->
        [one] { $count } profil
        [few] { $count } profile
        [many] { $count } profili
       *[other] { $count } profilu
    }
profiles-in-game = Teraz w grze: { $list }
profiles-in-game-item = „{ $name }” na { $drive }
profiles-in-game-none = Teraz żaden profil nie jest w grze.
profiles-import = Importuj…
profiles-import-title = Importuj profil z pliku .json
profiles-import-dialog = Importuj profil
profiles-export-dialog = Eksportuj profil
profiles-file-filter = Profile deadloadera
profiles-new = Nowy profil
profiles-selected = Wybrany
profiles-applied = Zastosowany
profiles-not-applied = Jeszcze niezastosowany
profiles-selected-title = Profil, który edytujesz w bibliotece i którym grasz
profiles-pending-title = Zastosuj lub Graj umieszcza go w folderze gry
profiles-applied-title = Jego pliki są teraz w folderze gry na { $drive }
profiles-select = Wybierz
profiles-select-of = Wybierz „{ $name }”
profiles-more-of = Więcej działań dla „{ $name }”
profiles-rename = Zmień nazwę
profiles-duplicate = Duplikuj
profiles-export = Eksportuj do pliku…
profiles-change-installation = Zmień instalację gry…
profiles-delete = Usuń
profiles-delete-confirm = Usuń…
profiles-delete-last = deadloader potrzebuje co najmniej jednego profilu.
profiles-name-label = Nazwa profilu
profiles-save = Zapisz
profiles-cancel = Anuluj
profiles-close = Zamknij
profiles-close-esc = Zamknij (Esc)
profiles-rename-hint = Enter zapisuje · Esc anuluje
profiles-name-empty = Wpisz nazwę profilu.
profiles-mods =
    { $count ->
        [one] { $count } mod włączony
        [few] { $count } mody włączone
        [many] { $count } modów włączonych
       *[other] { $count } moda włączonego
    }
profiles-mods-none = Żaden mod nie jest włączony
profiles-applied-when = Zastosowano { $when }
profiles-last-applied = Ostatnio zastosowano { $when }
profiles-never-applied = Nigdy nie zastosowano
profiles-just-now = przed chwilą
profiles-preview-empty = Wybierz ten profil, a potem włącz mody w bibliotece.
profiles-footnote = Profil to lista włączonych modów z ich kolejnością, wariantami i wersjami. Wybranie go niczego nie pobiera ani nie usuwa; Zastosuj zmienia tylko pliki, które się różnią.
profiles-copy-name = { $name } (kopia)
profiles-copy-name-n = { $name } (kopia { $number })
profiles-new-title = Nowy profil
profiles-start-with = Zacznij od
profiles-empty = Pusty
profiles-empty-hint = Żaden mod nie jest włączony — wybierzesz je potem w bibliotece.
profiles-copy-of = Kopia „{ $name }”
profiles-copy-hint = { $mods } — ta sama kolejność, warianty i wersje.
profiles-installation = Instalacja gry
profiles-installation-hint = Ta sama co w „{ $name }”. Możesz ją później zmienić w menu profilu.
profiles-new-note = Utworzenie profilu go nie wybiera ani nie stosuje — gra się nie zmienia.
profiles-create = Utwórz profil
profiles-pending-what = Profil „{ $name }” nie jest jeszcze w grze.
profiles-pending-why-other = W folderze gry na { $drive } wciąż jest „{ $other }”. Zastosuj go teraz albo naciśnij Graj — deadloader zastosuje go przed uruchomieniem.
profiles-pending-why-empty = W folderze gry na { $drive } nie ma teraz modów z deadloadera. Zastosuj go teraz albo naciśnij Graj — deadloader zastosuje go przed uruchomieniem.
profiles-keep = Zostaw „{ $name }”
profiles-apply = Zastosuj „{ $name }”
profiles-other-what = „{ $name }” używa innej instalacji gry.
profiles-other-why = Zastosuj i Graj używają { $path }. Gra na { $drive } zachowuje „{ $other }”.
profiles-other-why-empty = Zastosuj i Graj używają { $path }. Gra na { $drive } się nie zmienia.
profiles-delete-title = Usunąć profil „{ $name }”?
profiles-delete-applied =
    { $count ->
        [one] „{ $name }” jest teraz w grze. deadloader usunie jego { $count } mod z folderu gry:
        [few] „{ $name }” jest teraz w grze. deadloader usunie jego { $count } mody z folderu gry:
        [many] „{ $name }” jest teraz w grze. deadloader usunie jego { $count } modów z folderu gry:
       *[other] „{ $name }” jest teraz w grze. deadloader usunie jego { $count } moda z folderu gry:
    }
profiles-delete-applied-empty = „{ $name }” jest teraz w grze, ale nie ma modów, więc folder gry się nie zmieni.
profiles-delete-stay = Mody zostają w bibliotece i w innych profilach. Dopóki nie zastosujesz tam innego profilu, Deadlock uruchamia się bez modów z deadloadera.
profiles-delete-next = „{ $name }” staje się wybranym profilem. Nie zostanie zastosowany, dopóki nie naciśniesz Zastosuj lub Graj.
profiles-delete-undoable = Tego nie da się cofnąć. Aby zachować kopię, najpierw wyeksportuj profil.
profiles-export-first = Najpierw eksportuj
profiles-delete-profile = Usuń profil
profiles-installation-title = Instalacja gry dla „{ $name }”
profiles-installation-lead = Wybierz folder Deadlocka, do którego stosowany jest ten profil.
profiles-used-by =
    { $count ->
        [one] używana przez { $count } profil
        [few] używana przez { $count } profile
        [many] używana przez { $count } profili
       *[other] używana przez { $count } profilu
    }
profiles-used-by-none = nieużywana przez żaden profil
profiles-installation-applied = „{ $name }” jest teraz w grze na { $drive }. Jego pliki zostaną tam, dopóki nie zastosujesz innego profilu na { $drive }.
profiles-installation-not-applied = Żaden folder gry się nie zmieni, dopóki nie zastosujesz „{ $name }”.
profiles-manage-installations = Zarządzaj instalacjami w Ustawieniach
profiles-use-installation = Użyj tej instalacji
profiles-d9-title =
    { $count ->
        [one] { $count } moda z „{ $name }” nie ma w twojej bibliotece
        [few] { $count } modów z „{ $name }” nie ma w twojej bibliotece
        [many] { $count } modów z „{ $name }” nie ma w twojej bibliotece
       *[other] { $count } moda z „{ $name }” nie ma w twojej bibliotece
    }
profiles-d9-file = Plik
profiles-d9-lead = wymienia mody: { $listed }. W bibliotece masz już { $present }; tych { $count } nie:
profiles-d9-list = Brakujące mody
profiles-d9-from-gamebanana = z GameBanana: { $count }
profiles-d9-local-count = lokalne: { $count }
profiles-gamebanana = GameBanana
profiles-can-download = Można pobrać
profiles-local-mod = Lokalny mod
profiles-cant-download = Nie można pobrać
profiles-local-why = Dodany z dysku na innym komputerze, więc nie ma go na GameBanana. Poproś o plik osobę, która udostępniła profil.
profiles-d9-local-mode = Katalog jest wyłączony (tryb lokalny), więc deadloader nie może pobierać modów. Zaimportuj bez nich, a potem dodaj je z dysku — albo włącz katalog w Ustawieniach.
profiles-d9-after = Tak czy inaczej „{ $name }” stanie się wybranym profilem, ale nie zostanie zastosowany — folder gry zmieni się dopiero po naciśnięciu Zastosuj lub Graj.
profiles-import-without = Importuj bez nich
profiles-download-missing = Pobierz brakujące ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } mod wyłączony
        [few] { $count } mody wyłączone
        [many] { $count } modów wyłączonych
       *[other] { $count } moda wyłączonego
    }
profiles-turned-on =
    { $count ->
        [one] { $count } mod włączony
        [few] { $count } mody włączone
        [many] { $count } modów włączonych
       *[other] { $count } moda włączonego
    }
profiles-differ = Zmieniono tylko pliki, które się różnią.
profiles-same = Te same mody były już w grze; sprawdzono tylko kolejność i warianty.
profiles-deleted = Usunięto profil „{ $name }”.
profiles-deleted-not-applied = Nie był zastosowany, więc folder gry się nie zmienił.
profiles-deleted-applied =
    { $count ->
        [one] Usunięto { $count } mod z folderu gry. Nadal jest w bibliotece.
        [few] Usunięto { $count } mody z folderu gry. Nadal są w bibliotece.
        [many] Usunięto { $count } modów z folderu gry. Nadal są w bibliotece.
       *[other] Usunięto { $count } moda z folderu gry. Nadal są w bibliotece.
    }
profiles-exported = Wyeksportowano profil „{ $name }”.
profiles-imported = Zaimportowano profil „{ $name }”.
profiles-imported-text = Jest wybrany, ale jeszcze niezastosowany.
profiles-imported-without =
    { $count ->
        [one] Zaimportowano profil „{ $name }” bez { $count } moda.
        [few] Zaimportowano profil „{ $name }” bez { $count } modów.
        [many] Zaimportowano profil „{ $name }” bez { $count } modów.
       *[other] Zaimportowano profil „{ $name }” bez { $count } moda.
    }
profiles-imported-without-text = Jest wybrany, ale jeszcze niezastosowany. Brakujące mody możesz dodać później.
profiles-imported-downloading =
    { $count ->
        [one] Zaimportowano profil „{ $name }” — pobieranie { $count } moda.
        [few] Zaimportowano profil „{ $name }” — pobieranie { $count } modów.
        [many] Zaimportowano profil „{ $name }” — pobieranie { $count } modów.
       *[other] Zaimportowano profil „{ $name }” — pobieranie { $count } moda.
    }
profiles-imported-downloading-text = Jest wybrany, ale jeszcze niezastosowany. Zastosuj go, gdy pobieranie się skończy.
profiles-open-downloads = Otwórz pobieranie
profiles-undo = Cofnij
profiles-check-update = Sprawdź aktualizację deadloadera
profiles-choose-another = Wybierz inny plik
profiles-switcher = Zarządzaj profilami
