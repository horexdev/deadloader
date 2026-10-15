# Downloads (S6): the queue, its totals, the session’s history, the banners and D4 (choose a variant).
downloads-title = Stahování
downloads-active = Aktivní
downloads-queued = Ve frontě
downloads-speed = Rychlost
downloads-per-second = { $size }/s
downloads-limit-parallel =
    { $count ->
        [one] Jedno stahování najednou
        [few] Až { $count } stahování najednou
        [many] Až { $count } stahování najednou
       *[other] Až { $count } stahování najednou
    }
downloads-limit-none = bez omezení rychlosti
downloads-limit-speed = nejvýše { $speed }/s
downloads-change-settings = Změnit v Nastavení
downloads-pause-all = Pozastavit vše
downloads-resume-all = Pokračovat ve všem
downloads-add-disk = Přidat z disku
downloads-add-title = Přidat módy z disku
downloads-add-filter = Soubory módů a archivy
downloads-queue = Fronta
downloads-queue-empty = Fronta je prázdná. Nainstalujte mód z katalogu nebo přidejte soubory z disku.
downloads-history = Dokončeno v této relaci
downloads-history-note = Vymaže se po zavření deadloaderu. Každé stahování zůstává v Aktivitě.
downloads-history-empty = V této relaci zatím nic nedokončeno.
downloads-clear = Vymazat
downloads-show-in-library = Zobrazit v knihovně
downloads-show-in-library-of = Zobrazit v knihovně: { $name }
downloads-again = Stáhnout znovu
downloads-again-of = Stáhnout znovu: { $file }
downloads-pause = Pozastavit
downloads-pause-of = Pozastavit: { $file }
downloads-resume = Pokračovat
downloads-resume-of = Pokračovat: { $file }
downloads-cancel-of = Zrušit stahování { $file }
downloads-remove-of = Odebrat úlohu: { $file }
downloads-aside-of = Vybrat později: { $file }
downloads-open-mod = Otevřít stránku módu — { $name }
downloads-update = Aktualizace { $version }
downloads-update-plain = Aktualizace
downloads-choose = Vybrat variantu
downloads-retry = Zkusit znovu
downloads-show-details = Zobrazit podrobnosti
downloads-remove-task = Odebrat úlohu
downloads-open-settings = Otevřít Nastavení → Stahování
downloads-open-mod-page = Otevřít stránku módu
downloads-open-library = Otevřít knihovnu
downloads-stage-downloading = Stahuje se
downloads-stage-verifying = Ověřuje se
downloads-stage-extracting = Rozbaluje se
downloads-stage-choice = Čeká na váš výběr
downloads-stage-paused = Pozastaveno
downloads-stage-offline = Pozastaveno — offline
downloads-stage-disk = Pozastaveno — plný disk
downloads-stage-retrying = Čeká na další pokus
downloads-stage-queued = Ve frontě
downloads-stage-failed = Selhalo
downloads-progress = { $done } z { $total } · { $percent } %
downloads-left-seconds = zbývá { $seconds } s
downloads-left-minutes = zbývá { $minutes } min { $seconds } s
downloads-left-hours = zbývá { $hours } h { $minutes } min
downloads-rate = { $speed }/s · { $left }
downloads-verifying-note = Porovnává se kontrolní součet (md5) s GameBananou
downloads-extracting-note = Ponechají se jen soubory .vpk
downloads-choice-note =
    { $count ->
        [one] { $count } varianta uvnitř
        [few] { $count } varianty uvnitř
        [many] { $count } varianty uvnitř
       *[other] { $count } variant uvnitř
    }
downloads-offline-note = Pokračuje samo, až budete znovu online
downloads-disk-note = Čeká na volné místo na { $drive }
downloads-disk-note-plain = Čeká na volné místo
downloads-paused-note = Pokračuje tam, kde přestalo
downloads-held-note = Zadrženo — nespustí se, dokud nebude obnoveno
downloads-queued-next = Další — spustí se, až se uvolní jedno z { $count } míst pro stahování
downloads-queued-ahead = Před ním: { $count }
downloads-queued-offline = Spustí se, až budete znovu online
downloads-retrying-note = Další pokus za { $seconds } s
downloads-downloaded = staženo
downloads-offline-title = Offline — stahování je pozastaveno.
downloads-offline-why = Až budete znovu online, bude samo pokračovat tam, kde přestalo. Import z disku pokračuje.
downloads-try-again = Zkusit znovu
downloads-back-title = Znovu online.
downloads-back-why = Pozastavená stahování pokračují tam, kde přestala.
downloads-empty-title = Žádná stahování
downloads-empty-text = Nainstalujte mód z katalogu nebo přidejte soubory .vpk, .zip, .7z či .rar ze svého počítače. Každý se tu zobrazí, dokud se stahuje a rozbaluje. Soubory můžete také přetáhnout kamkoli do tohoto okna.
downloads-browse = Procházet katalog
downloads-done-installed = Nainstalováno a zapnuto
downloads-done-variant = Nainstalováno · { $variant }
downloads-done-updated = Aktualizováno na { $version }
downloads-done-updated-plain = Aktualizováno
downloads-done-cancelled = Zrušeno
downloads-done-removed = Odebráno — archiv smazán
downloads-done-kept = V knihovně, nezapnuto
downloads-ago-now = právě teď
downloads-ago-minutes = před { $count } min
downloads-ago-hours = před { $count } h
downloads-d4-title = Vyberte variantu
downloads-d4-lead = Tento archiv obsahuje { $count } varianty, které mění stejné soubory hry, takže zapnutá může být vždy jen jedna.
downloads-d4-description = Popis souboru na GameBananě
downloads-d4-list = Varianty v tomto archivu
downloads-d4-list-meta = { $count } · vyberte jednu
downloads-d4-hint = deadloader ponechá všechny ({ $count }) ve vaší knihovně a zapne tu, kterou vyberete, v profilu „{ $profile }“ na začátku pořadí. Později to můžete změnit v knihovně.
downloads-d4-install = Nainstalovat
downloads-d4-installed = Varianta „{ $variant }“ · na začátku profilu „{ $profile }“.
downloads-cancel = Zrušit
downloads-close-esc = Zavřít (Esc)
