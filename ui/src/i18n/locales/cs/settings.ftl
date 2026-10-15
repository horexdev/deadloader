# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Nastavení
settings-nav = Oddíly nastavení
settings-reset-nav = Obnovit všechna nastavení…
settings-default = Výchozí: { $value }
settings-restore = Obnovit výchozí
settings-on = Zapnuto
settings-off = Vypnuto
settings-cancel = Zrušit
settings-remove = Odebrat
settings-remove-item = Odebrat { $item }
settings-add = Přidat
settings-open-folder = Otevřít složku
settings-open-folder-at = Otevřít složku: { $path }
settings-copied = Zkopírováno do schránky.
settings-folder = Složka
settings-example = Příklad:

settings-section-general = Obecné
settings-section-general-lead = Jazyk, spuštění a přiblížení.
settings-section-game = Hra
settings-section-game-lead = Kde je Deadlock nainstalovaný a co v něm smí deadloader měnit.
settings-section-catalog = Katalog
settings-section-catalog-lead = Katalog GameBanana, citlivý obsah, obrázky a odkazy.
settings-section-library = Knihovna
settings-section-library-lead = Aktualizace modů, mazání modů a starší verze.
settings-section-downloads = Stahování
settings-section-downloads-lead = Fronta stahování, archivy a rozbalování.
settings-section-network = Síť
settings-section-network-lead = Jak deadloader komunikuje s GameBanana.
settings-section-cache = Mezipaměť
settings-section-cache-lead = Náhledy a stránky katalogu uložené v tomto počítači.
settings-section-logs = Protokoly
settings-section-logs-lead = Co uchovává Aktivita a jak dlouho.
settings-section-advanced = Pokročilé
settings-section-advanced-lead = Soubory nastavení a další povolené domény.
settings-section-about = O aplikaci deadloader
settings-section-about-lead = Verze, licence, jak ověřit toto stažení a kde jsou vaše data.

# General
settings-general-group = Jazyk a spuštění
settings-language = Jazyk
settings-language-help = Jazyk systému: jazyk zobrazení Windows, pokud ho deadloader má, jinak angličtina.
settings-language-system = Jazyk systému ({ $language })
settings-startup-profile = Profil při spuštění
settings-startup-profile-help = Který profil je vybraný po spuštění deadloaderu. Výběr ho nepoužije.
settings-startup-last = Naposledy vybraný ({ $name })
settings-startup-last-plain = Naposledy vybraný
settings-app-updates = Při spuštění hledat aktualizace deadloaderu
settings-app-updates-help = Při spuštění se deadloader jednou zeptá GitHubu (api.github.com), zda existuje novější verze. Aktualizace je vždy ruční. Zvolili jste to při prvním spuštění.
settings-app-updates-local = Nedostupné: místní režim nevytváří žádné síťové požadavky. Možnost bude dostupná, až zapnete katalog (Nastavení → Katalog).
settings-zoom = Přiblížení
settings-zoom-keys-1 = Také pomocí Ctrl + kolečko myši,
settings-zoom-keys-2 = a
settings-zoom-keys-3 = pro obnovení.
settings-ctrl = Ctrl
settings-percent = { $value } %

# Game
settings-installs-group = Instalace hry
settings-installs-lead = deadloader sám najde instalace Deadlocku ve Steamu. Každý profil používá jednu z nich — to změníte v Profilech.
settings-install-library = Knihovna Steam { $library }
settings-install-build = sestavení { $build }
settings-install-used =
    { $count ->
        [0] nepoužívá ji žádný profil
        [one] používá ji { $count } profil: { $names }
       *[other] používají ji profily ({ $count }): { $names }
    }
settings-install-selected = { $name } (vybraný)
settings-install-applied = { $name } (použitý zde)
settings-install-steam = Nalezeno přes Steam
settings-install-manual = Přidáno ručně
settings-install-known = Jsou tu ještě soubory deadloaderu
settings-install-manual-meta = Přidáno ručně
settings-gi-ours = gameinfo.gi: v pořádku — řádek deadloaderu je na místě
settings-gi-wait = gameinfo.gi: připraví se, až ji bude používat profil
settings-gi-rolled = gameinfo.gi: původní — Použít znovu přidá řádek deadloaderu
settings-gi-foreign = gameinfo.gi: změněno jiným programem
settings-gi-missing = gameinfo.gi: nenalezeno — složka byla možná přesunuta
settings-installs-none = Zatím nebyla nalezena žádná instalace.
settings-add-manual = Přidat ručně…
settings-add-manual-hint = Vyberte složku Deadlocku, která nebyla nalezena.
settings-add-manual-title = Vyberte složku Deadlocku
settings-install-added = Instalace přidána.
settings-install-removed = Instalace odebrána ze seznamu.
settings-install-removed-text = V té složce se nic nezměnilo.
settings-gameinfo-group = Po aktualizacích hry
settings-autofix = Opravovat gameinfo.gi automaticky
settings-autofix-help = Aktualizace Deadlocku obnoví soubor hry, který říká, že se mají načítat mody. deadloader ho zálohuje, vrátí svůj řádek a dá vám vědět. Když je vypnuto, nejprve se zeptá oznámení.
settings-danger-group = Vrácení a odstranění
settings-rollback = Vrátit složku hry vybraného profilu
settings-rollback-help = Pro instalaci vybraného profilu { $profile } ({ $drive }): odebere soubory modů, které tam deadloader vložil ({ $files }), a jeho řádek v gameinfo.gi. Knihovna a profily zůstanou; Použít vše vrátí.
settings-rollback-help-others = Ostatní složky hry zůstanou beze změny; chcete-li je vrátit, vyberte profil, který je používá.
settings-rollback-nothing = Na { $drive } není co vracet — deadloader tam teď nemá žádné soubory.
settings-rollback-no-game = Deadlock nebyl nalezen, takže není co vracet.
settings-rollback-button = Vrátit…
settings-rolled-title = Složka hry na { $drive } vrácena.
settings-rolled-text =
    { $foreign ->
        [0] Na { $drive } byly odebrány soubory modů ({ $files }) a řádek deadloaderu v gameinfo.gi. Knihovna a profily zůstaly — Použít nebo Hrát mody vrátí.
       *[other] Na { $drive } byly odebrány soubory modů ({ $files }), řádek deadloaderu v gameinfo.gi a cizí soubory ({ $foreign }). Knihovna a profily zůstaly — Použít nebo Hrát mody vrátí.
    }
settings-wipe = Smazat všechna data deadloaderu
settings-wipe-help = Smaže knihovnu, profily, protokoly, mezipaměť a nastavení ({ $size }) a pak deadloader zavře. deadloader.exe zůstane — smažte ho potom sami.
settings-wipe-button = Smazat všechna data…

# Catalog
settings-catalog-group = Katalog GameBanana
settings-catalog = Používat katalog
settings-catalog-help-on = Procházejte a instalujte mody z GameBanana. deadloader komunikuje jen s GameBanana a každý požadavek je vidět v Aktivitě. Vypnuto znamená místní režim.
settings-catalog-help-off = Vypnuto — místní režim: deadloader nevytváří žádné síťové požadavky. Mody z disku můžete přidávat dál.
settings-consent-title = Nejprve jeden krok.
settings-consent-text = Než se katalog zapne, deadloader vám ukáže, které hostitele bude kontaktovat, a podmínky GameBanana.
settings-consent-go = Prohlédnout a zapnout
settings-local-on = Místní režim je zapnutý.
settings-local-on-text = Katalog je vypnutý a deadloader nevytváří síťové požadavky.
settings-ttl = Znovu používat načtené stránky katalogu po dobu
settings-ttl-help = Během této doby se návrat na stránku GameBanana znovu neptá.
settings-minutes = { $count } min
settings-hours = { $count } h
settings-prefetch = Načítat další stránku předem
settings-prefetch-help = Když projdete asi 70 % výsledků, načtou se další, aby mřížka pokračovala.
settings-nsfw-group = Citlivý obsah
settings-nsfw = Mody s hodnocením obsahu (NSFW)
settings-nsfw-help = Skryté: vůbec se nezobrazí. Rozmazané: rozmazaný obrázek s označením „Citlivý obsah“; kliknutím ho zobrazíte. Zobrazené: bez rozmazání.
settings-nsfw-age-done = Věk potvrzen { $date }.
settings-nsfw-age-ask = Při opuštění možnosti „Skryté“ budete jednou požádáni o potvrzení věku.
settings-nsfw-hidden = Skryté
settings-nsfw-blurred = Rozmazané
settings-nsfw-shown = Zobrazené
settings-images-group = Obrázky a odkazy
settings-description-images = Obrázky v popisech modů
settings-description-images-help = Pocházejí z jiných hostitelů než GameBanana, např. i.imgur.com. Po kliknutí: zástupný prvek, dokud na něj neklepnete.
settings-images-click = Po kliknutí
settings-images-always = Vždy
settings-prechecked = Předem zaškrtnout „Vždy povolit obrázky z tohoto hostitele“
settings-prechecked-help = V dialogu, který se zobrazí, když obrázek z popisu poprvé pochází z nového hostitele.
settings-hosts = Povolení hostitelé obrázků
settings-hosts-help = Obrázky z těchto hostitelů se načítají bez ptaní. V Aktivitě jsou jejich požadavky označeny jako „povolený hostitel obrázků“.
settings-hosts-built-in = Vestavěný
settings-hosts-none = Žádní přidaní hostitelé — každý nový hostitel se nejprve zeptá.
settings-host-removed = { $host } odebrán
settings-host-removed-text = Jeho obrázky se před načtením znovu zeptají.
settings-undo = Vrátit zpět
settings-gif = Přehrávat GIFy automaticky
settings-gif-help = Vypnuto: GIFy ukazují první snímek s tlačítkem přehrát.
settings-links = Externí odkazy
settings-links-help = Odkazy se otevírají ve výchozím prohlížeči. „Nejprve se zeptat“ zobrazí před otevřením celou adresu.
settings-links-ask = Nejprve se zeptat
settings-links-open = Otevřít hned

# Library
settings-updates-group = Aktualizace modů
settings-check-updates = Při spuštění hledat aktualizace modů
settings-check-updates-help = Porovná vaše mody z GameBanana s katalogem. Nikdy v místním režimu.
settings-auto-download = Stahovat aktualizace automaticky
settings-auto-download-help = Jen stažení — deadloader se i tak zeptá, než mod přejde na novou verzi.
settings-disk-group = Mazání a starší verze
settings-deletion = Když smažete mod
settings-deletion-help = V dialogu mazání můžete pokaždé zvolit jinak.
settings-deletion-permanent = Trvale
settings-deletion-bin = Do koše
settings-old-versions = Starší verze
settings-old-versions-help = Každá stažená verze se uchová, abyste se k ní mohli vrátit. Starší verze modu smažete v jeho podrobnostech.
settings-old-keep-all = Ponechat všechny

# Downloads
settings-queue-group = Fronta
settings-parallel = Souběžná stahování
settings-parallel-help = Kolik souborů se stahuje současně.
settings-speed = Omezení rychlosti
settings-speed-help = Jen pro stahování modů — katalog a náhledy nejsou omezeny.
settings-speed-none = Bez omezení
settings-speed-value = { $value } MB/s
settings-keep-archives = Ponechat archivy po instalaci
settings-keep-archives-help = Vypnuto: soubor .zip, .7z nebo .rar se smaže, jakmile jsou jeho soubory .vpk v knihovně.
settings-unpack-group = Rozbalování
settings-limits = Limity rozbalování
settings-limits-help = Chrání před archivy, které se rozbalí do něčeho obrovského. Pokud je odmítnut archiv, kterému věříte, zvyšte limit.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = Rozbalená velikost
settings-limit-ratio = Kompresní poměr
settings-limit-files = Souborů modů na archiv
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = souborů
settings-unit-s = s
settings-number-error = { $field }: zadejte celé číslo od { $min } do { $max }. Do té doby platí předchozí hodnota.

# Network
settings-network-local = Místní režim je zapnutý.
settings-network-local-text = deadloader nevytváří síťové požadavky; tato nastavení začnou platit, až bude katalog zapnutý.
settings-requests-group = Požadavky na GameBanana
settings-api-parallel = Souběžné požadavky na katalog
settings-api-parallel-help = Požadavky na API GameBanana současně. Obrázky a stahování se počítají zvlášť.
settings-timeouts = Časové limity
settings-timeouts-help = Připojení: vzdát to, když se během této doby nepřipojí. Odpověď: vzdát to, když server přestane posílat.
settings-timeouts-default = 10 s · 30 s
settings-timeout-connect = Připojení
settings-timeout-read = Odpověď
settings-retries = Opakování
settings-retries-help = Kolikrát se neúspěšný požadavek s rostoucími pauzami zopakuje, než deadloader ukáže chybu.
settings-connection-group = Připojení
settings-proxy = Proxy
settings-proxy-help = Žádný: přímé připojení. Systémový: proxy nastavený ve Windows. Ruční: zadejte ho níže.
settings-proxy-none = Žádný
settings-proxy-system = Systémový
settings-proxy-manual = Ruční
settings-proxy-host = Adresa
settings-proxy-port = Port
settings-proxy-hint = HTTP proxy. Prochází jím jen provoz deadloaderu.
settings-proxy-invalid = Zadejte název hostitele nebo adresu bez uživatelského jména a hesla, např. proxy.example.com.

# Cache
settings-thumbs-group = Náhledy a stránky katalogu
settings-cache-limit = Limit mezipaměti
settings-cache-limit-help = Když je mezipaměť plná, jako první odcházejí náhledy, které jste nejdéle neviděli.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = Aktuální velikost
settings-cache-breakdown = Náhledy: { $thumbnails } · stránky katalogu: { $pages }
settings-cache-empty = Prázdná — zatím nic uloženo.
settings-cache-used = { $used } z { $limit }
settings-cache-clear = Vymazat mezipaměť
settings-cache-cleared = Mezipaměť vymazána — uvolněno { $size }. Náhledy se znovu načtou, až budou potřeba.
settings-cache-cleared-partly = Soubory, které se používaly ({ $failed }), zůstanou do příštího vymazání.

# Logs
settings-keep-group = Uchovávání protokolů
settings-log-days = Uchovávat protokoly po dobu
settings-log-days-help = Starší záznamy se mažou při spuštění deadloaderu.
settings-days = { $count ->
        [one] { $count } den
        [few] { $count } dny
        [many] { $count } dne
       *[other] { $count } dní
    }
settings-year = 1 rok
settings-log-size = Maximální velikost
settings-log-size-help = Nad touto velikostí se jako první mažou nejstarší soubory protokolu.
settings-privacy-group = Soukromí
settings-mask = Skrýt uživatelské jméno Windows
settings-mask-help = Cesty v protokolech ukazují { $masked } místo vašeho jména — bezpečnější, když protokol sdílíte.
settings-logs-folder-group = Složka protokolů
settings-logs-folder = Kde jsou protokoly
settings-logs-now = { $size } · souborů: { $files } · nejstarší z { $date }
settings-open-logs = Otevřít složku protokolů

# Advanced
settings-advanced-warning = Pro zkušené uživatele.
settings-advanced-warning-text = Chyba zde může změnit chování deadloaderu. Před uložením soubor zkontroluje a uchová zálohu předchozí verze.
settings-files-group = Úprava souborů nastavení
settings-file-settings = Nastavení
settings-file-manifest = Manifest složky hry
settings-file-label = Soubor k úpravě
settings-file-settings-lead = Vše z těchto stránek v jednom souboru. Změny se po uložení objeví na stránkách.
settings-file-manifest-lead = Seznam toho, co deadloader vložil do složky hry. Vracení se na něj spoléhá — chybný záznam může zanechat soubory.
settings-json-error = Toto není platný JSON, a proto nebyl uložen.
settings-json-where = Řádek { $line }, sloupec { $column }: tady nebo těsně předtím něco chybí nebo přebývá — často čárka nebo uvozovka.
settings-json-fix = Opravte to místo a uložte znovu — do té doby deadloader používá aktuální soubor.
settings-json-refused = Tyto hodnoty nelze použít, a proto se nic neuložilo.
settings-json-newer = Tento soubor pochází z novějšího deadloaderu (formát { $found }; tato verze čte do { $supported }), a proto nebyl uložen.
settings-json-invalid = deadloader tento soubor nemůže použít, a proto nebyl uložen.
settings-json-details = Podrobnosti
settings-problem-unknown-field = { $path }: toto není nastavení této verze.
settings-problem-invalid-value = { $path }: tuto hodnotu zde nelze použít.
settings-problem-out-of-range = { $path }: { $found } je mimo rozsah { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: „{ $entry }“ zde není platné.
settings-problem-duplicate-entry = { $path }: „{ $entry }“ je uvedeno dvakrát.
settings-problem-needs-age = { $path }: zobrazení obsahu NSFW vyžaduje nejprve potvrzení věku (Nastavení → Katalog).
settings-problem-local-mode = { $path }: v místním režimu nedostupné.
settings-json-idle = Kontroluje se při uložení.
settings-json-saved = Platné a uložené. Předchozí verze uložena jako { $backup }.
settings-json-saved-plain = Platné a uložené.
settings-json-save = Zkontrolovat a uložit
settings-json-discard = Zahodit mé úpravy
settings-json-backup = Před uložením se předchozí soubor uchová jako záloha; zůstává posledních 10.
settings-domains-group = Další povolené domény
settings-domains-lead = deadloader komunikuje jen s GameBanana. Pokud je stahování přesměrováno na jiného hostitele a zablokováno, můžete ho zde povolit. V Aktivitě jsou požadavky na něj označeny jako „přidáno uživatelem“.
settings-domains-built-in = Vestavěné:
settings-domain-user = přidáno uživatelem
settings-domains-none = Žádné další domény.
settings-domain-add = Přidat doménu
settings-domain-hint = Jen název hostitele, např. files.example.com.
settings-domain-empty = Nejprve zadejte název hostitele, např. files.example.com.
settings-domain-not-host = Zadejte jen název hostitele — bez https:// a cesty, např. files.example.com.
settings-domain-invalid = „{ $host }“ není název hostitele. Zkontrolujte zápis, např. files.example.com.
settings-domain-built-in = { $host } je vestavěný a vždy povolený.
settings-domain-listed = { $host } už je v seznamu.

# About
settings-version = verze { $version }
settings-commit = Sestaveno z commitu
settings-commit-local = Místní sestavení — nesestavené GitHub Actions
settings-copy-commit = Kopírovat ID commitu
settings-release-page = Stránka vydání
settings-licence-group = Licence
settings-licence-help = deadloader je svobodný software: můžete ho používat, zkoumat, sdílet a měnit podle GPL-3. Je poskytován bez jakékoli záruky.
settings-read-licence = Přečíst licenci
settings-source = Zdrojový kód
settings-verify-group = Jak ověřit toto stažení
settings-verify-lead = Dvě nezávislé kontroly, že tento deadloader.exe je ten zveřejněný na GitHubu.
settings-verify-hash = Porovnat kontrolní součet SHA-256
settings-verify-hash-help = Musí se shodovat se součtem na stránce vydání. Upravená kopie by zde mohla ukázat falešnou hodnotu, proto si ho spočítejte i sami:
settings-verify-hash-cmd = V PowerShellu ve složce s exe:
settings-verify-attest = Ověřit atestaci sestavení
settings-verify-attest-help = Dokazuje, že GitHub Actions sestavil tento soubor ze zveřejněného zdrojového kódu. Vyžaduje GitHub CLI:
settings-copy-hash = Kopírovat kontrolní součet
settings-copy-command = Kopírovat příkaz
settings-unsigned = deadloader.exe není podepsaný.
settings-unsigned-text = Windows SmartScreen může při prvním spuštění hlásit „Systém Windows ochránil váš počítač“. Dvě kontroly výše potvrdí, že je soubor pravý.
settings-webview-group = Jádro okna
settings-webview-help = Okno deadloaderu vykresluje WebView2, součást Windows. Windows ho aktualizují samy. deadloader vypíná:
settings-webview-off-1 = Dotazy DNS-over-HTTPS
settings-webview-off-2 = Přihlášení účtem Microsoft v okně
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Síťový provoz na pozadí a aktualizace komponent
settings-webview-off-5 = Volání serveru automatického vyplňování formulářů
settings-webview-off-6 = Hlášení chyb společnosti Microsoft (domain reliability)
settings-webview-no-route = A okno nemá vůbec žádnou cestu do internetu.
settings-webview-unknown = verze neznámá
settings-data-group = Vaše data
settings-data-folder = Složka dat
settings-data-folder-help = Vše, co deadloader uchovává, je v této jediné složce vedle exe.
settings-notice = Upozornění při prvním spuštění
settings-notice-help = Verze { $version }, přijato { $date }, jazyk: { $language }. Vaše volby: katalog — { $catalog }, hledání aktualizací — { $updates }.
settings-notice-none = Zatím nepřijato.
settings-notice-again = Přečíst znovu

# D7
settings-d7-title = Vrátit složku hry na { $drive }?
settings-d7-from =
    { $others ->
        [0] Instalace vybraného profilu { $profile }. deadloader odebere z
       *[other] Instalace vybraného profilu { $profile } (používá ji také { $others }). deadloader odebere z
    }
settings-d7-files =
    { $count ->
        [one] { $count } soubor modu, který přidal do
        [few] { $count } soubory modů, které přidal do
        [many] { $count } souboru modů, které přidal do
       *[other] { $count } souborů modů, které přidal do
    }
settings-d7-line = Svůj řádek v gameinfo.gi
settings-d7-line-why = — soubor bude zase takový, jak ho nainstaloval Steam.
settings-d7-stays = Zůstane beze změny:
settings-d7-library = Vaše knihovna — modů: { $mods }, { $size }
settings-d7-profiles =
    { $count ->
        [one] Váš profil — { $names }
       *[other] Vaše profily ({ $count }) — { $names }
    }
settings-d7-other = Instalace na { $path } ({ $profile }) — beze změny. Chcete-li ji vrátit, vyberte profil, který ji používá, a vraťte znovu.
settings-d7-other-plain = Instalace na { $path } — beze změny.
settings-d7-back = Použít nebo Hrát vše vrátí.
settings-d7-foreign =
    { $count ->
        [one] Odebrat také { $count } soubor, který nevytvořil deadloader
        [few] Odebrat také { $count } soubory, které nevytvořil deadloader
        [many] Odebrat také { $count } souboru, které nevytvořil deadloader
       *[other] Odebrat také { $count } souborů, které nevytvořil deadloader
    }
settings-d7-foreign-why = z jiného programu nebo ruční instalace. Smažou se trvale; deadloader je nemůže vrátit.
settings-d7-confirm = Vrátit
settings-d7-confirm-foreign =
    { $count ->
        [one] Vrátit a smazat { $count } soubor
        [few] Vrátit a smazat { $count } soubory
        [many] Vrátit a smazat { $count } souboru
       *[other] Vrátit a smazat { $count } souborů
    }

# D8
settings-d8-title = Smazat všechna data deadloaderu?
settings-d8-from = deadloader smaže vše ve složce
settings-d8-from-2 = a pak se zavře.
settings-d8-what = Co
settings-d8-contains = Obsahuje
settings-d8-size = Velikost
settings-d8-total = Celkem
settings-d8-library = Knihovna
settings-d8-library-has = modů: { $mods }, souborů: { $files }
settings-d8-profiles = Profily
settings-d8-logs = Protokoly
settings-d8-logs-has = souborů: { $files }, od { $date }
settings-d8-logs-none = žádné soubory protokolu
settings-d8-cache = Mezipaměť
settings-d8-cache-has = náhledy a stránky katalogu
settings-d8-settings = Nastavení
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
       *[other] včetně záloh ({ $backups })
    }
settings-d8-other = Ostatní
settings-d8-other-has = nedokončená stahování, zálohy gameinfo.gi, data jádra okna
settings-d8-exe = Nesmaže se: samotný deadloader.exe — smažte ho sami, pokud ho už nepotřebujete.
settings-d8-rollback = Nejprve vrátit každou složku hry, kterou deadloader používá (doporučeno)
settings-d8-rollback-why = Před smazáním odebere soubory modů deadloaderu a jeho řádek v gameinfo.gi z každé složky hry. Cizí soubory zůstanou.
settings-d8-rollback-off = Nezaškrtnuto: soubory modů deadloaderu zůstanou ve složkách hry a Deadlock je bude dál načítat — bez deadloaderu, který by je odebral.
settings-d8-type-1 = Pro potvrzení napište
settings-d8-type-2 = { "" }
settings-d8-word = smazat
settings-d8-hint = Tlačítko pro smazání se odemkne, až slovo bude souhlasit.
settings-d8-hint-ok = Potvrzeno. Tuto akci nelze vrátit.
settings-d8-type-first = Nejprve napište „{ $word }“
settings-d8-deleting = Mazání…
settings-d8-confirm = Smazat a zavřít
settings-d8-confirm-rollback = Vrátit, smazat a zavřít

# Reset
settings-reset-title = Obnovit všechna nastavení?
settings-reset-body = Každé nastavení se vrátí na výchozí hodnotu, včetně povolených hostitelů obrázků a dalších domén.
settings-reset-keeps = Knihovna, profily a protokoly zůstanou beze změny, stejně jako složka hry.
settings-reset-backup = Záloha současného nastavení se uchová jako settings.json.bak-…
settings-reset-confirm = Obnovit nastavení
settings-reset-done = Všechna nastavení byla obnovena na výchozí.
settings-reset-done-text = Záloha: { $backup }
