# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Aktivita
activity-sub =
    { $days ->
        [one] Vše, co deadloader na tomto počítači udělal: síťové požadavky, změny souborů a události aplikace. Protokoly zůstávají na tomto počítači { $days } den.
        [few] Vše, co deadloader na tomto počítači udělal: síťové požadavky, změny souborů a události aplikace. Protokoly zůstávají na tomto počítači { $days } dny.
        [many] Vše, co deadloader na tomto počítači udělal: síťové požadavky, změny souborů a události aplikace. Protokoly zůstávají na tomto počítači { $days } dne.
       *[other] Vše, co deadloader na tomto počítači udělal: síťové požadavky, změny souborů a události aplikace. Protokoly zůstávají na tomto počítači { $days } dní.
    }
activity-open-logs = Otevřít složku protokolů
activity-logs-opened = Složka protokolů je otevřená v Průzkumníkovi souborů.
activity-summary = Dnešek v kostce
activity-net-today = Síť dnes
activity-hosts-label = Hostitelé kontaktovaní dnes — vyberte jednoho pro filtr
activity-hosts-contacted =
    { $count ->
        [one] { $count } kontaktovaný hostitel
        [few] { $count } kontaktovaní hostitelé
        [many] { $count } kontaktovaného hostitele
       *[other] { $count } kontaktovaných hostitelů
    }
activity-host-title =
    { $count ->
        [one] { $count } požadavek · { $source } — vyberte pro filtr
        [few] { $count } požadavky · { $source } — vyberte pro filtr
        [many] { $count } požadavku · { $source } — vyberte pro filtr
       *[other] { $count } požadavků · { $source } — vyberte pro filtr
    }
activity-net-none-local = žádné — místní režim
activity-net-none-yet = zatím žádné
activity-nothing-else = Nic dalšího.
activity-blocked-lead =
    { $count ->
        [one] { $count } požadavek na
        [few] { $count } požadavky na
        [many] { $count } požadavku na
       *[other] { $count } požadavků na
    }
activity-blocked-tail =
    { $count ->
        [one] byl zablokován dřív, než došlo ke spojení.
        [few] byly zablokovány dřív, než došlo ke spojení.
        [many] bylo zablokováno dřív, než došlo ke spojení.
       *[other] bylo zablokováno dřív, než došlo ke spojení.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } požadavek zablokován, bez spojení.
        [few] { $count } požadavky zablokovány, bez spojení.
        [many] { $count } požadavku zablokováno, bez spojení.
       *[other] { $count } požadavků zablokováno, bez spojení.
    }
activity-show-entry = Zobrazit záznam
activity-game-today = Složka hry dnes
activity-show-in-files = Zobrazit v Souborech
activity-game-none = beze změn
activity-no-game-changes = deadloader ve složce hry zatím nic nezměnil.
activity-game-changes =
    { $count ->
        [one] { $count } změna
        [few] { $count } změny
        [many] { $count } změny
       *[other] { $count } změn
    }
activity-game-batches =
    { $count ->
        [one] v { $count } dávce
        [few] ve { $count } dávkách
        [many] v { $count } dávky
       *[other] v { $count } dávkách
    }
activity-game-failed =
    { $count ->
        [one] { $count } selhala
        [few] { $count } selhaly
        [many] { $count } selhalo
       *[other] { $count } selhalo
    }
activity-game-hardlink = propojeno: { $count }
activity-game-copy = zkopírováno: { $count }
activity-game-delete = odebráno: { $count }
activity-game-rename = přejmenováno: { $count }
activity-game-write = zapsáno: { $count }
activity-game-backup = zálohováno: { $count }
activity-game-restore = obnoveno: { $count }
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi aktualizován { $count }krát
        [few] gameinfo.gi aktualizován { $count }krát
        [many] gameinfo.gi aktualizován { $count }krát
       *[other] gameinfo.gi aktualizován { $count }krát
    }
activity-no-requests-today = Dnes žádné požadavky.
activity-no-requests-yet = Zatím žádné požadavky.
activity-local-short = Katalog je vypnutý, takže se deadloader k ničemu nepřipojuje.
activity-never-contacted = deadloader od instalace nekontaktoval žádného hostitele.
activity-tab-network = Síť
activity-tab-files = Soubory
activity-tab-app = Aplikace
activity-tab-off = vyp.
activity-tab-errors =
    { $count ->
        [one] · { $count } chyba
        [few] · { $count } chyby
        [many] · { $count } chyby
       *[other] · { $count } chyb
    }
activity-search = Hledat v protokolu
activity-search-clear = Vymazat hledání
activity-matches =
    { $count ->
        [one] { $count } výsledek
        [few] { $count } výsledky
        [many] { $count } výsledku
       *[other] { $count } výsledků
    }
activity-matches-more = výsledků: { $count }+
activity-f-time = Období
activity-f-purpose = Účel
activity-f-host = Hostitel
activity-f-op = Operace
activity-f-level = Úroveň
activity-f-result = Výsledek
activity-all = Vše
activity-ok = OK
activity-errors = Chyby
activity-range-hour = Poslední hodina
activity-range-today = Dnes
activity-range-7d = Posledních 7 dní
activity-range-30d = Posledních 30 dní
activity-clear-filters = Zrušit filtry
activity-clear-all = Zrušit hledání a filtry
activity-follow = Sledovat živě
activity-follow-tip = Zobrazovat nové záznamy nahoře hned, jak vzniknou
activity-follow-on = Nové záznamy se zobrazují nahoře
activity-follow-paused = Pozastaveno v { $time }
activity-show-new =
    { $count ->
        [one] Zobrazit { $count } nový
        [few] Zobrazit { $count } nové
        [many] Zobrazit { $count } nového
       *[other] Zobrazit { $count } nových
    }
activity-net-aria = Síťové požadavky, nejnovější nahoře
activity-files-aria = Operace se soubory, nejnovější nahoře
activity-app-aria = Události aplikace, nejnovější nahoře
activity-c-redirects = Přesměrování
activity-c-time = Čas
activity-c-purpose = Účel
activity-c-request = Požadavek
activity-c-status = Stav
activity-c-size = Velikost
activity-c-duration = Trvání
activity-c-started-by = Spustil
activity-c-source = Zdroj hostitele
activity-c-op = Operace
activity-c-from-to = Odkud → kam
activity-c-result = Výsledek
activity-c-change = Součást změny
activity-c-level = Úroveň
activity-c-message = Zpráva
activity-to = do
activity-loading-more = Načítání…
activity-p-catalog = Katalog
activity-p-thumbnail = Náhled
activity-p-description-image = Obrázek v popisu
activity-p-download = Stahování
activity-p-update-check = Kontrola aktualizací
activity-src-builtin = Vestavěný
activity-src-github = GitHub
activity-src-user = Přidaný vámi
activity-src-user-image = Hostitel obrázků
activity-src-blocked = Nepovolený
activity-src-long-builtin = Vestavěný — GameBanana, součást deadloaderu
activity-src-long-github = GitHub — vlastní kontrola aktualizací deadloaderu, zapnutá v nastavení
activity-src-long-user = Přidaný vámi — v části Nastavení → Pokročilé
activity-src-long-user-image = Povolený hostitel obrázků — povolili jste ho při načítání obrázku v popisu
activity-src-long-blocked = Není na seznamu povolených hostitelů
activity-by-auto = Auto
activity-by-user = Vy
activity-op-hardlink = Propojení
activity-op-copy = Kopírování
activity-op-rename = Přejmenování
activity-op-delete = Smazání
activity-op-write = Zápis
activity-op-backup = Záloha
activity-op-restore = Obnovení
activity-lv-debug = Ladění
activity-lv-info = Info
activity-lv-warn = Upozornění
activity-lv-error = Chyba
activity-st-blocked = Zablokováno
activity-st-blocked-long = Zablokováno — bez spojení
activity-st-failed = Selhalo
activity-redirects =
    { $count ->
        [one] { $count } přesměrování
        [few] { $count } přesměrování
        [many] { $count } přesměrování
       *[other] { $count } přesměrování
    }
activity-retries =
    { $count ->
        [one] { $count } opakování
        [few] { $count } opakování
        [many] { $count } opakování
       *[other] { $count } opakování
    }
activity-show-redirects = Zobrazit přesměrování
activity-hide-redirects = Skrýt přesměrování
activity-hop-request = Krok 1 · požadavek
activity-hop-redirect = Krok { $n } · přesměrování
activity-hop-retry = Krok { $n } · pokus { $try }
activity-ms = { $ms } ms
activity-seconds = { $s } s
activity-no-match = Žádné odpovídající záznamy
activity-no-match-text = Zvolte jiné období nebo zrušte filtry a hledání.
activity-empty-title = Zatím nic
activity-empty-text = deadloader sem zapisuje každý síťový požadavek, každou změnu souborů ve složce hry a každou událost aplikace hned, jak k nim dojde. Zatím se nic nestalo.
activity-local-title = Místní režim — žádné požadavky se neodesílají
activity-local-why = Katalog je vypnutý, takže se deadloader k ničemu nepřipojuje — ani ke GameBanana, ani nikam jinam. Změny souborů a události aplikace se dál zapisují na ostatních kartách.
activity-local-do = Zapněte katalog v nastavení, abyste mohli procházet GameBanana. Každý požadavek se pak zobrazí tady.
activity-local-history = Požadavky níže byly odeslány dříve, než byl zapnut místní režim.
activity-enable-catalog = Zapnout katalog
activity-details = Podrobnosti záznamu protokolu
activity-close-details = Zavřít podrobnosti
activity-close-esc = Zavřít (Esc)
activity-why = Proč:
activity-todo = Co dělat:
activity-open-advanced = Otevřít Nastavení → Pokročilé
activity-full-record = Celý záznam
activity-hops-title = Přesměrování · { $count }
activity-user-hidden = Vaše uživatelské jméno ve Windows je v protokolech skryté:
activity-settings-logs = Nastavení → Protokoly
activity-copy = Kopírovat záznam
activity-copied = Zkopírováno
activity-kind-network = Síťový požadavek
activity-kind-files = Operace se souborem
activity-kind-app = Událost aplikace
activity-d-time = Čas
activity-d-purpose = Účel
activity-d-started-by = Spustil
activity-d-request = Požadavek
activity-d-source = Zdroj hostitele
activity-d-status = Stav
activity-d-size = Velikost
activity-d-duration = Trvání
activity-d-range = Rozsah
activity-d-error = Chyba
activity-d-id = ID záznamu
activity-d-request-id = ID požadavku
activity-d-operation = Operace
activity-d-from = Odkud
activity-d-to = Kam
activity-d-file = Soubor
activity-d-result = Výsledek
activity-d-change = Součást změny
activity-change-apply = Profil „{ $profile }“ použit
activity-change-rollback = Vrácení složky hry
activity-change-profile-removed = Profil „{ $profile }“ smazán, jeho mody opustily hru
activity-change-take-foreign = { $file } převzat do správy
activity-change-gameinfo-fixed = Náš řádek přidán do gameinfo.gi
activity-change-recovered = Přerušená změna dokončena při spuštění
activity-d-sha256 = SHA-256
activity-d-level = Úroveň
activity-d-message = Zpráva
activity-d-fields = Podrobnosti
activity-status-after =
    { $count ->
        [one] { $status } — po { $count } přesměrování
        [few] { $status } — po { $count } přesměrováních
        [many] { $status } — po { $count } přesměrování
       *[other] { $status } — po { $count } přesměrováních
    }
