# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Profily
profiles-count =
    { $count ->
        [one] { $count } profil
        [few] { $count } profily
        [many] { $count } profilu
       *[other] { $count } profilů
    }
profiles-in-game = Teď ve hře: { $list }
profiles-in-game-item = „{ $name }“ na { $drive }
profiles-in-game-none = Teď ve hře není žádný profil.
profiles-import = Importovat…
profiles-import-title = Importovat profil ze souboru .json
profiles-import-dialog = Importovat profil
profiles-export-dialog = Exportovat profil
profiles-file-filter = Profily deadloaderu
profiles-new = Nový profil
profiles-selected = Vybraný
profiles-applied = Použitý
profiles-not-applied = Zatím nepoužitý
profiles-selected-title = Profil, který upravujete v knihovně a se kterým hrajete
profiles-pending-title = Použít nebo Hrát ho vloží do složky hry
profiles-applied-title = Jeho soubory jsou teď ve složce hry na { $drive }
profiles-select = Vybrat
profiles-select-of = Vybrat „{ $name }“
profiles-more-of = Další akce pro „{ $name }“
profiles-rename = Přejmenovat
profiles-duplicate = Duplikovat
profiles-export = Exportovat do souboru…
profiles-change-installation = Změnit instalaci hry…
profiles-delete = Smazat
profiles-delete-confirm = Smazat…
profiles-delete-last = deadloader potřebuje alespoň jeden profil.
profiles-name-label = Název profilu
profiles-save = Uložit
profiles-cancel = Zrušit
profiles-close = Zavřít
profiles-close-esc = Zavřít (Esc)
profiles-rename-hint = Enter uloží · Esc zruší
profiles-name-empty = Zadejte název profilu.
profiles-mods =
    { $count ->
        [one] { $count } zapnutý mód
        [few] { $count } zapnuté módy
        [many] { $count } zapnutého módu
       *[other] { $count } zapnutých módů
    }
profiles-mods-none = Žádný zapnutý mód
profiles-applied-when = Použito { $when }
profiles-last-applied = Naposledy použito { $when }
profiles-never-applied = Nikdy nepoužito
profiles-just-now = právě teď
profiles-preview-empty = Vyberte tento profil a pak zapněte módy v knihovně.
profiles-footnote = Profil je seznam zapnutých módů s jejich pořadím, variantami a verzemi. Výběr nic nestahuje ani nemaže; Použít mění jen soubory, které se liší.
profiles-copy-name = { $name } (kopie)
profiles-copy-name-n = { $name } (kopie { $number })
profiles-new-title = Nový profil
profiles-start-with = Začít s
profiles-empty = Prázdný
profiles-empty-hint = Žádný zapnutý mód — vyberte je potom v knihovně.
profiles-copy-of = Kopie „{ $name }“
profiles-copy-hint = { $mods } — stejné pořadí, varianty a verze.
profiles-installation = Instalace hry
profiles-installation-hint = Stejná jako u „{ $name }“. Později ji můžete změnit v nabídce profilu.
profiles-new-note = Vytvoření profilu ho nevybere ani nepoužije — hra se nezmění.
profiles-create = Vytvořit profil
profiles-pending-what = Profil „{ $name }“ zatím není ve hře.
profiles-pending-why-other = Složka hry na { $drive } stále obsahuje „{ $other }“. Použijte ho teď, nebo stiskněte Hrát — deadloader ho použije před spuštěním.
profiles-pending-why-empty = Složka hry na { $drive } teď nemá žádné módy z deadloaderu. Použijte ho teď, nebo stiskněte Hrát — deadloader ho použije před spuštěním.
profiles-keep = Ponechat „{ $name }“
profiles-apply = Použít „{ $name }“
profiles-other-what = „{ $name }“ používá jinou instalaci hry.
profiles-other-why = Použít a Hrát používají { $path }. Hra na { $drive } si ponechá „{ $other }“.
profiles-other-why-empty = Použít a Hrát používají { $path }. Hra na { $drive } se nezmění.
profiles-delete-title = Smazat profil „{ $name }“?
profiles-delete-applied =
    { $count ->
        [one] „{ $name }“ je teď ve hře. deadloader odebere jeho { $count } mód ze složky hry:
        [few] „{ $name }“ je teď ve hře. deadloader odebere jeho { $count } módy ze složky hry:
        [many] „{ $name }“ je teď ve hře. deadloader odebere jeho { $count } módu ze složky hry:
       *[other] „{ $name }“ je teď ve hře. deadloader odebere jeho { $count } módů ze složky hry:
    }
profiles-delete-applied-empty = „{ $name }“ je teď ve hře, ale nemá žádné módy, takže se složka hry nezmění.
profiles-delete-stay = Módy zůstanou ve vaší knihovně a v ostatních profilech. Dokud tam nepoužijete jiný profil, Deadlock se spouští bez módů z deadloaderu.
profiles-delete-next = „{ $name }“ se stane vybraným profilem. Použije se až po stisknutí Použít nebo Hrát.
profiles-delete-undoable = Tuto akci nelze vrátit. Chcete-li si ponechat kopii, profil nejdřív exportujte.
profiles-export-first = Nejdřív exportovat
profiles-delete-profile = Smazat profil
profiles-installation-title = Instalace hry pro „{ $name }“
profiles-installation-lead = Vyberte složku Deadlocku, na kterou se tento profil používá.
profiles-used-by =
    { $count ->
        [one] používá { $count } profil
        [few] používají { $count } profily
        [many] používá { $count } profilu
       *[other] používá { $count } profilů
    }
profiles-used-by-none = nepoužívá žádný profil
profiles-installation-applied = „{ $name }“ je teď ve hře na { $drive }. Jeho soubory tam zůstanou, dokud na { $drive } nepoužijete jiný profil.
profiles-installation-not-applied = Žádná složka hry se nezmění, dokud nepoužijete „{ $name }“.
profiles-manage-installations = Spravovat instalace v Nastavení
profiles-use-installation = Použít tuto instalaci
profiles-d9-title =
    { $count ->
        [one] { $count } mód z „{ $name }“ není ve vaší knihovně
        [few] { $count } módy z „{ $name }“ nejsou ve vaší knihovně
        [many] { $count } módu z „{ $name }“ není ve vaší knihovně
       *[other] { $count } módů z „{ $name }“ není ve vaší knihovně
    }
profiles-d9-file = Soubor
profiles-d9-lead = obsahuje módů: { $listed }. Ve vaší knihovně jich už je { $present }; těchto { $count } ne:
profiles-d9-list = Chybějící módy
profiles-d9-from-gamebanana = z GameBanany: { $count }
profiles-d9-local-count = místní: { $count }
profiles-gamebanana = GameBanana
profiles-can-download = Lze stáhnout
profiles-local-mod = Místní mód
profiles-cant-download = Nelze stáhnout
profiles-local-why = Přidán z disku na jiném PC, takže není na GameBananě. Požádejte o soubor toho, kdo profil sdílel.
profiles-d9-local-mode = Katalog je vypnutý (místní režim), takže deadloader nemůže stahovat módy. Importujte bez nich a pak je přidejte z disku — nebo zapněte katalog v Nastavení.
profiles-d9-after = V obou případech se „{ $name }“ stane vaším vybraným profilem, ale nepoužije se — složka hry se změní až po stisknutí Použít nebo Hrát.
profiles-import-without = Importovat bez nich
profiles-download-missing = Stáhnout chybějící ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } mód vypnut
        [few] { $count } módy vypnuty
        [many] { $count } módu vypnuto
       *[other] { $count } módů vypnuto
    }
profiles-turned-on =
    { $count ->
        [one] { $count } mód zapnut
        [few] { $count } módy zapnuty
        [many] { $count } módu zapnuto
       *[other] { $count } módů zapnuto
    }
profiles-differ = Změnily se jen soubory, které se liší.
profiles-same = Stejné módy už ve hře byly; zkontrolovalo se jen pořadí a varianty.
profiles-deleted = Profil „{ $name }“ smazán.
profiles-deleted-not-applied = Nebyl použitý, takže se složka hry nezměnila.
profiles-deleted-applied =
    { $count ->
        [one] { $count } mód byl odebrán ze složky hry. Zůstává ve vaší knihovně.
        [few] { $count } módy byly odebrány ze složky hry. Zůstávají ve vaší knihovně.
        [many] { $count } módu bylo odebráno ze složky hry. Zůstávají ve vaší knihovně.
       *[other] { $count } módů bylo odebráno ze složky hry. Zůstávají ve vaší knihovně.
    }
profiles-exported = Profil „{ $name }“ exportován.
profiles-imported = Profil „{ $name }“ importován.
profiles-imported-text = Je vybraný, ale zatím nepoužitý.
profiles-imported-without =
    { $count ->
        [one] Profil „{ $name }“ importován bez { $count } módu.
        [few] Profil „{ $name }“ importován bez { $count } módů.
        [many] Profil „{ $name }“ importován bez { $count } módu.
       *[other] Profil „{ $name }“ importován bez { $count } módů.
    }
profiles-imported-without-text = Je vybraný, ale zatím nepoužitý. Chybějící módy můžete přidat později.
profiles-imported-downloading =
    { $count ->
        [one] Profil „{ $name }“ importován — stahuje se { $count } mód.
        [few] Profil „{ $name }“ importován — stahují se { $count } módy.
        [many] Profil „{ $name }“ importován — stahuje se { $count } módu.
       *[other] Profil „{ $name }“ importován — stahuje se { $count } módů.
    }
profiles-imported-downloading-text = Je vybraný, ale zatím nepoužitý. Použijte ho, až se stahování dokončí.
profiles-open-downloads = Otevřít stahování
profiles-undo = Zpět
profiles-check-update = Zkontrolovat aktualizaci deadloaderu
profiles-choose-another = Vybrat jiný soubor
profiles-switcher = Spravovat profily
profiles-saved-as = Uloženo jako
profiles-show-folder = Zobrazit ve složce
