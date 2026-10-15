# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Otevřít tento odkaz v prohlížeči?
catalog-link-body = Odkaz opustí deadloader a otevře se ve vašem výchozím prohlížeči.
catalog-link-always = Odkazy vždy otevírat hned
catalog-link-cancel = Zrušit
catalog-link-open = Otevřít

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Skryté
catalog-nsfw-blurred = Rozmazané
catalog-nsfw-shown = Zobrazené
catalog-age-title = Zobrazovat mody s citlivým obsahem?
catalog-age-body = Některé mody na GameBanana mají hodnocení kvůli nahotě, sexuálním tématům a podobně. Chcete-li je vidět, potvrďte, že je vám alespoň 18 let nebo že jste plnoletí podle místních zákonů.
catalog-age-choice = Zvolili jste: { $mode }.
catalog-age-once = deadloader se ptá jen jednou. Ke „Skryté“ se můžete kdykoli vrátit.
catalog-age-confirm = Jsem plnoletý/á
catalog-age-cancel = Zrušit

## S2: the catalog grid
catalog-title = Katalog
catalog-section = Sekce
catalog-section-mods = Mody
catalog-section-sounds = Zvuky
catalog-search = Hledat mody
catalog-search-sounds = Hledat zvuky
catalog-search-offline = Hledání vyžaduje připojení
catalog-search-off = Hledání není dostupné — katalog nelze přečíst
catalog-search-clear = Vymazat hledání
catalog-results =
    { $count ->
        [one] { $count } výsledek
        [few] { $count } výsledky
       *[other] { $count } výsledků
    }
catalog-sort = Řazení
catalog-sort-newest = Nejnovější
catalog-sort-latest_updated = Nedávno aktualizované
catalog-sort-most_downloaded = Nejstahovanější
catalog-sort-most_liked = Nejoblíbenější
catalog-sort-most_viewed = Nejzobrazovanější
catalog-sort-most_commented = Nejkomentovanější
catalog-sort-oldest = Nejstarší
catalog-sort-best = Nejlepší shoda
catalog-sort-best-title = Při hledání se řadí podle shody
catalog-category = Kategorie
catalog-category-all = Všechny kategorie
catalog-hero = Hrdina
catalog-hero-all = Všichni hrdinové
catalog-hero-filter = Najít hrdinu
catalog-hero-none = Žádný hrdina neodpovídá
catalog-filters = Filtry
catalog-filter-nsfw = Mody s hodnocením obsahu (NSFW)
catalog-filter-favourites = Jen oblíbené
catalog-filter-favourites-short = jen oblíbené
catalog-filter-installed = Skrýt nainstalované
catalog-filter-installed-short = bez nainstalovaných
catalog-nsfw-caption-hidden = NSFW skryté
catalog-nsfw-caption-blurred = NSFW rozmazané · kartu zobrazíte kliknutím
catalog-nsfw-caption-shown = NSFW zobrazené
catalog-loading = Načítání modů z GameBanana…
catalog-loading-sounds = Načítání zvuků z GameBanana…
catalog-loading-more = Načítání dalších…
catalog-more-failed = Další stránku se nepodařilo načíst.
catalog-grid = Mody
catalog-grid-sounds = Zvuky
catalog-likes =
    { $count ->
        [one] { $count } lajk
        [few] { $count } lajky
       *[other] { $count } lajků
    }
catalog-favourite-add = Přidat do oblíbených
catalog-favourite-remove = Odebrat z oblíbených
catalog-sensitive = Citlivý obsah
catalog-click-show = Zobrazíte kliknutím
catalog-badge-installed = Nainstalováno
catalog-badge-update = Aktualizace
catalog-card-installing = Instalace · { $percent } %
catalog-empty = Zatím tu nejsou žádné mody
catalog-empty-search = Hledání „{ $text }“ neodpovídá žádný mod
catalog-empty-favourites = Zatím žádné oblíbené
catalog-empty-filters = Zapnuté filtry: { $filters }. Zrušte je a hledejte ve všech modech, nebo zkuste kratší slovo.
catalog-empty-shorter = Zkuste kratší nebo jiné slovo.
catalog-clear-filters = Zrušit filtry
catalog-rate-retry = deadloader to automaticky zkusí znovu za { $seconds } s — mody níže zůstávají dostupné.
catalog-retry-now = Zkusit hned
catalog-try-again = Zkusit znovu
catalog-check-update = Zkontrolovat aktualizaci deadloaderu
catalog-details = Podrobnosti · záznam v protokolu
catalog-copy-id = Kopírovat ID záznamu
catalog-open-activity = Otevřít v Aktivitě
catalog-offline-empty-title = Offline — není co zobrazit
catalog-offline-empty-text = deadloader se nemůže spojit s GameBanana a tato část katalogu nebyla dříve uložena.
catalog-error-title = Katalog teď nelze zobrazit.
catalog-local-title = Katalog je vypnutý (místní režim)
catalog-local-why = V místním režimu deadloader neodesílá žádné síťové požadavky. Knihovna, profily i Hrát fungují jako obvykle a mody můžete přidávat z disku.
catalog-local-do = Zapněte katalog, chcete-li procházet a instalovat mody z GameBanana. deadloader pak bude komunikovat jen s GameBanana.
catalog-local-enable = Zapnout katalog
catalog-local-library = Přejít do knihovny

## S3: the mod page
catalog-breadcrumb = Drobečková navigace
catalog-back = Zpět do katalogu
catalog-page-loading = Načítání stránky modu z GameBanana…
catalog-page-offline = Offline — zobrazena kopie uložená { $date }.
catalog-page-offline-why = Instalace nových souborů vyžaduje připojení; přepínání mezi staženými variantami funguje dál.
catalog-page-offline-empty-title = Offline — tato stránka nebyla uložena
catalog-page-offline-empty-text = deadloader se nemůže spojit s GameBanana a tuto stránku jste dříve neotevřeli.
catalog-page-local-title = Tato stránka modu nebyla uložena
catalog-page-local-text = V místním režimu deadloader zobrazuje jen dříve uložené stránky modů.
catalog-removed-title = Tento mod byl z GameBanana odstraněn
catalog-removed-why = Byl zadržen nebo smazán, takže deadloader nemůže zobrazit jeho stránku ani stáhnout soubory.
catalog-removed-keep = Vaše nainstalovaná kopie ({ $copy }) zůstává v knihovně a dál funguje — jen už nebude dostávat aktualizace.
catalog-show-in-library = Zobrazit v knihovně
catalog-installed = Nainstalováno
catalog-by = autor: { $author }
catalog-version = Verze { $version }
catalog-views =
    { $count ->
        [one] { $count } zobrazení
        [few] { $count } zobrazení
       *[other] { $count } zobrazení
    }
catalog-downloads =
    { $count ->
        [one] { $count } stažení
        [few] { $count } stažení
       *[other] { $count } stažení
    }
catalog-added = Přidáno { $when }
catalog-updated = Aktualizováno { $when }
catalog-favourite = Oblíbený
catalog-open-gamebanana = Otevřít na GameBanana
catalog-gallery-alt = { $name }, obrázek { $index } z galerie
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Předchozí obrázek
catalog-gallery-next = Další obrázek
catalog-gallery-original = Zobrazit originál
catalog-gallery-hidden = Obrázky modů s hodnocením obsahu jsou skryté.
catalog-audio-preview = Tento zvuk má ukázku na GameBanana.
catalog-description = Popis
catalog-description-more = Zobrazit celý popis
catalog-description-less = Zobrazit méně
catalog-description-truncated = Zobrazen je jen začátek popisu.
catalog-opens-browser = otevře se v prohlížeči
catalog-opens-here = Otevře se v deadloaderu
catalog-image-from = Obrázek z
catalog-image-click = načtete kliknutím
catalog-image-failed = nepodařilo se načíst
catalog-image-alt = Obrázek z popisu
catalog-youtube = Video na YouTube
catalog-spoiler = Spoiler — zobrazíte kliknutím
catalog-spoiler-hide = Skrýt spoiler
catalog-files = Soubory
catalog-files-clean =
    { $count ->
        [one] Prošel antivirovou kontrolou GameBanana
       *[other] Všechny ({ $count }) prošly antivirovou kontrolou GameBanana
    }
catalog-files-clean-dated =
    { $count ->
        [one] Prošel antivirovou kontrolou GameBanana · nahráno { $date }
       *[other] Všechny ({ $count }) prošly antivirovou kontrolou GameBanana · nahráno { $date }
    }
catalog-files-av = Výsledek antiviru GameBanana je uveden u každého souboru.
catalog-variants-note = Tyto soubory mění stejné soubory hry, takže jde o varianty — aktivní je jen jedna.
catalog-no-files = Tento mod nemá žádné soubory ke stažení.
catalog-older-files = Starší soubory ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } stažení
        [few] { $count } stažení
       *[other] { $count } stažení
    }
catalog-file-active = aktivní varianta
catalog-file-downloaded = staženo
catalog-file-install = Instalovat
catalog-file-install-label = Instalovat { $file } a nastavit jako aktivní variantu
catalog-file-install-title = Stáhne tento soubor a nastaví ho jako aktivní variantu
catalog-file-install-offline = Instalace vyžaduje připojení
catalog-file-installing = Instalace…
catalog-file-installed = Nainstalováno
catalog-file-switch = Přepnout na
catalog-file-switch-label = Přepnout na { $file }
catalog-file-switch-title = Už staženo — nastavit jako aktivní variantu
catalog-file-update = Aktualizovat
catalog-file-update-title = Novější verze nahrazuje váš nainstalovaný soubor — prohlédněte si ji v knihovně
catalog-av-clean = Čistý
catalog-av-clean-title = Antivirus GameBanana: čistý
catalog-av-flagged = Označený
catalog-av-flagged-title = Antivirus GameBanana: { $verdict }
catalog-av-pending = Zatím nezkontrolováno
catalog-av-pending-title = GameBanana ještě nedokončil kontrolu tohoto souboru
catalog-av-none = Bez kontroly
catalog-av-none-title = GameBanana pro tento soubor neposlal výsledek antiviru
catalog-credits = Autoři
catalog-no-ratings = Bez hodnocení obsahu
catalog-no-licence = Bez textu licence
catalog-no-sources = Bez alternativních zdrojů
catalog-rating-sexual_themes = Sexuální témata
catalog-rating-sexual_content = Sexuální obsah
catalog-rating-skimpy_attire = Vyzývavé oblečení
catalog-rating-lewd_angles_and_poses = Vyzývavé záběry a pózy
catalog-rating-full_nudity = Úplná nahota
catalog-licence = Licence
catalog-sources = Alternativní zdroje
catalog-videos = Videa

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Načíst obrázek z nového serveru?
catalog-image-body-before = deadloader stáhne tento obrázek ze serveru
catalog-image-body-after = . Tento server není GameBanana — požadavek se zobrazí v Aktivitě jako všechny ostatní.
catalog-image-note = Velikost obrázků je omezená. Povolené servery můžete odebrat v Nastavení → Katalog.
catalog-image-always = Vždy povolit obrázky z tohoto serveru
catalog-image-cancel = Zrušit
catalog-image-load = Načíst obrázek
