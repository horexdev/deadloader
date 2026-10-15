## Notifications

n1 = Deadlock byl aktualizován — mody byly obnoveny.
    .why = Aktualizace vrátila soubor gameinfo.gi do původního stavu, a tak do něj deadloader znovu přidal svůj řádek.
n2 = { $name } — nainstalováno a zapnuto.
n3 =
    { $count ->
        [one] Je k dispozici { $count } aktualizace modu.
        [few] Jsou k dispozici { $count } aktualizace modů.
        [many] Je k dispozici { $count } aktualizace modů.
       *[other] Je k dispozici { $count } aktualizací modů.
    }
n4 = Deadlock je spuštěný.
    .why = Změny se použijí automaticky, jakmile hru zavřete.
    .pending =
        { $count ->
            [one] { $count } změna čeká
            [few] { $count } změny čekají
            [many] { $count } změny čeká
           *[other] { $count } změn čeká
        }
n5 =
    { $reason ->
        [completed] Přerušená změna byla dokončena.
       *[undone] Přerušená změna byla vrácena zpět; složka hry je jako předtím.
    }
n6 =
    { $count ->
        [one] Ve složce modů hry byl nalezen { $count } soubor, který nevytvořil deadloader.
        [few] Ve složce modů hry byly nalezeny { $count } soubory, které nevytvořil deadloader.
        [many] Ve složce modů hry bylo nalezeno { $count } souboru, které nevytvořil deadloader.
       *[other] Ve složce modů hry bylo nalezeno { $count } souborů, které nevytvořil deadloader.
    }
n7 =
    { $when ->
        [today] Offline — zobrazuje se katalog uložený v { $saved }.
       *[earlier] Offline — zobrazuje se katalog uložený { $saved }.
    }
    .why = Stahování bude pokračovat, až budete znovu online.
n8 =
    { $count ->
        [one] Profil „{ $name }“ byl použit — zapnut { $count } mod.
        [few] Profil „{ $name }“ byl použit — zapnuty { $count } mody.
        [many] Profil „{ $name }“ byl použit — zapnuto { $count } modu.
       *[other] Profil „{ $name }“ byl použit — zapnuto { $count } modů.
    }
n9 = Je k dispozici nová verze programu deadloader: { $version }.
n10 = Místní režim je zapnutý.
    .why = Katalog je vypnutý a deadloader neodesílá žádné síťové požadavky.
n11 = Mody se kopírují místo propojení, protože deadloader a hra jsou na různých discích.
    .why = Zabírá to navíc { $size } místa na disku.
n12 =
    { $count ->
        [one] Deadlock byl zavřen — byla použita { $count } změna.
        [few] Deadlock byl zavřen — byly použity { $count } změny.
        [many] Deadlock byl zavřen — bylo použito { $count } změny.
       *[other] Deadlock byl zavřen — bylo použito { $count } změn.
    }

## Errors: what happened, why, what to do

e1 = deadloader nemůže uložit svá data.
    .why =
        { $reason ->
            [access_denied] Složka vedle deadloader.exe je jen pro čtení — Windows například programům nedovolí zapisovat do složky Program Files.
            [in_use] Jiný program, často antivirus, má otevřený jeden ze souborů programu deadloader.
           *[other] Windows nemohl zapisovat do složky vedle deadloader.exe.
        }
    .todo =
        { $reason ->
            [in_use] Zavřete ostatní programy a zkuste to znovu.
           *[other] Přesuňte deadloader do složky, do které můžete zapisovat, například D:\Games\deadloader.
        }
e2 = Deadlock nebyl nalezen.
    .why = Steam nebo hra nejsou nainstalované, nebo byla složka hry přesunuta.
    .todo = Vyberte složku hry ručně.
e3 = Soubor hry změnil jiný program.
    .why = Soubor gameinfo.gi nevypadá jako originál — často ho mění Deadlock Mod Manager. deadloader nepřepisuje cizí změny.
    .todo = Nechte Steam ověřit integritu souborů hry; deadloader pak přidá svůj řádek.
e4 = Katalog není k dispozici.
    .why = GameBanana změnil své API, takže ho deadloader nedokáže přečíst. Knihovna a profily fungují jako obvykle.
    .todo = Zkontrolujte, zda není k dispozici aktualizace programu deadloader, nebo to zkuste později.
e5 = GameBanana je přetížený.
    .why =
        { $reason ->
            [rate_limited] Příliš mnoho požadavků za krátkou dobu (HTTP 429).
            [server] Server GameBanana nahlásil chybu.
           *[other] Příliš mnoho požadavků nebo chyba serveru.
        }
    .todo =
        { $wait ->
            [seconds]
                { $retry_in_s ->
                    [one] deadloader to automaticky zkusí znovu za { $retry_in_s } sekundu.
                    [few] deadloader to automaticky zkusí znovu za { $retry_in_s } sekundy.
                    [many] deadloader to automaticky zkusí znovu za { $retry_in_s } sekundy.
                   *[other] deadloader to automaticky zkusí znovu za { $retry_in_s } sekund.
                }
           *[later] deadloader to za chvíli automaticky zkusí znovu.
        }
e6 = Stahování se nezdařilo.
    .why =
        { $reason ->
            [local_mode] deadloader je v místním režimu, takže neodesílá žádné síťové požadavky.
            [dns] Adresu serveru pro stahování se nepodařilo najít.
            [connect] deadloader se nemohl připojit k serveru pro stahování.
            [tls] Zabezpečené připojení k serveru pro stahování se nepodařilo navázat.
            [timeout] Server příliš dlouho neodpovídal.
            [connection_lost] Připojení bylo přerušeno.
            [http_status] Server odpověděl chybou (HTTP { $status }).
           *[other] Stahování bylo přerušeno.
        }
    .todo =
        { $reason ->
            [local_mode] Zapněte katalog v Nastavení, abyste mohli stahovat mody.
           *[other] Bude pokračovat tam, kde se zastavilo.
        }
e7 = Stažený soubor je poškozený.
    .why =
        { $reason ->
            [size] Jeho velikost neodpovídá té, kterou uvádí GameBanana.
           *[checksum] Jeho kontrolní součet neodpovídá tomu, který uvádí GameBanana.
        }
    .todo = Stáhněte ho znovu.
e8 = Archiv nelze rozbalit.
    .why =
        { $reason ->
            [not_an_archive] Soubor není archiv ZIP, 7z ani RAR — možná se nestáhl celý.
            [damaged] Archiv je poškozený.
            [encrypted] Archiv je chráněný heslem.
            [unsupported] Používá metodu komprese, kterou deadloader nepodporuje.
            [not_a_vpk] { $entry } není platný soubor .vpk.
           *[other] Je poškozený nebo má nepodporovaný formát.
        }
    .todo = Zkuste jiný soubor tohoto modu nebo problém nahlaste jeho autorovi.
e9 = Archiv je příliš velký na bezpečné rozbalení.
    .why =
        { $reason ->
            [too_large]
                { $measured ->
                    [yes] Po rozbalení by měl { $unpacked }, limit je { $limit }.
                   *[no] Po rozbalení má víc než { $limit }.
                }
            [too_many_files]
                { $found ->
                    [one] Obsahuje { $found } soubor modu, limit je { $limit }.
                    [few] Obsahuje { $found } soubory modů, limit je { $limit }.
                    [many] Obsahuje { $found } souboru modů, limit je { $limit }.
                   *[other] Obsahuje { $found } souborů modů, limit je { $limit }.
                }
            [ratio] { $entry } je podezřele silně zkomprimovaný — víc než { $limit }krát. To je typické pro škodlivý archiv.
            [memory] Rozbalení vyžaduje { $needed } paměti, limit je { $limit }.
           *[other] Překračuje jeden z bezpečnostních limitů pro archivy.
        }
    .todo =
        { $reason ->
            [memory] Zkuste jiný soubor tohoto modu.
            [ratio] Instalujte ho jen tehdy, když zdroji důvěřujete; pak můžete limit zvýšit v části Nastavení → Pokročilé.
           *[other] Pokud mu důvěřujete, zvyšte limit v části Nastavení → Stahování.
        }
e10 = V tomto archivu nejsou žádné soubory modů.
    .why =
        { $listing ->
            [files] Neobsahuje žádné soubory .vpk, jen { $contents }.
           *[empty] Archiv je prázdný.
        }
    .todo = Tento soubor není mod pro Deadlock ve formátu, který deadloader umí nainstalovat.
e11 = Na tomto počítači nelze rozbalit archivy RAR.
    .why = Vestavěný nástroj Windows tento RAR nepřečte a 7-Zip ani WinRAR nebyly nalezeny.
    .todo = Nainstalujte 7-Zip (je zdarma) a zkuste to znovu; stažený soubor zůstane zachován.
e12 = Nedostatek místa na disku.
    .why =
        { $reason ->
            [space] Je potřeba { $needed }, ale volno je jen { $available }.
           *[full] Disk je plný.
        }
    .todo = Uvolněte místo nebo v Knihovně smažte staré verze modů — a pak to zkuste znovu.
e13 = Změny nelze použít, dokud je Deadlock spuštěný.
    .why = Windows zamyká soubory hry, dokud běží.
    .todo = Zavřete Deadlock; deadloader změny použije hned potom.
e14 = Soubor modu v knihovně chybí nebo je poškozený.
    .why = Byl změněn nebo smazán mimo deadloader.
    .todo = Opravte mod, aby se soubor stáhl znovu, nebo mod odeberte.
e15 = Odkaz se nepodařilo otevřít.
    .why = Ve Windows není nastavený výchozí prohlížeč.
    .todo = Zkopírujte odkaz a otevřete ho sami.
e16 = Požadavek byl zablokován.
    .why =
        { $reason ->
            [not_https] Server přesměroval na adresu bez HTTPS a taková přesměrování deadloader nikdy nenásleduje.
            [bad_redirect] Server poslal přesměrování, které deadloader nedokáže následovat.
            [too_many_redirects] Server přesměroval příliš mnohokrát.
           *[not_listed] { $host } není na seznamu povolených hostitelů, takže se k němu deadloader nepřipojil.
        }
    .todo =
        { $reason ->
            [not_listed] Nemusíte nic dělat — deadloader komunikuje jen s GameBanana. Pokud tomuto hostiteli důvěřujete, přidejte ho v části Nastavení → Pokročilé.
           *[not_https] Nemusíte nic dělat — deadloader následuje jen bezpečná přesměrování. Zkuste to později.
        }
e17 = Profil se nepodařilo importovat.
    .why =
        { $reason ->
            [newer] Byl vytvořen novější verzí programu deadloader.
            [unreadable] Soubor nelze přečíst — možná byl přesunut, smazán nebo ho má otevřený jiný program.
           *[not_a_profile] Tento soubor není profil programu deadloader.
        }
    .todo =
        { $reason ->
            [newer] Aktualizujte deadloader a potom profil importujte znovu.
            [unreadable] Zkontrolujte, že soubor stále existuje a žádný jiný program ho nemá otevřený, a zkuste to znovu.
           *[not_a_profile] Zkontrolujte, že jste vybrali správný soubor.
        }
e18 = Soubory ve složce hry se nepodařilo změnit.
    .why =
        { $reason ->
            [access_denied] Windows odepřel přístup ke složce hry.
            [in_use] Soubor používá jiný program, například antivirus nebo jiný správce modů.
           *[other] Windows nemohl soubory změnit.
        }
    .todo = Zavřete ostatní programy a zkuste to znovu.
e19 = Obrázek se nepodařilo načíst.
    .why =
        { $reason ->
            [no_response] Hostitel neodpověděl.
            [not_an_image] Soubor není obrázek.
            [too_large] Obrázek je příliš velký.
           *[other] Hostitel neposlal použitelný obrázek.
        }
    .todo = Zástupný obrázek zůstane; můžete to zkusit znovu.
e20 = Je zapnuto příliš mnoho modů.
    .why =
        { $free ->
            [one] Hra může načíst ještě { $free } soubor modu, ale tento profil jich potřebuje { $wanted }.
            [few] Hra může načíst ještě { $free } soubory modů, ale tento profil jich potřebuje { $wanted }.
            [many] Hra může načíst ještě { $free } souboru modů, ale tento profil jich potřebuje { $wanted }.
           *[other] Hra může načíst ještě { $free } souborů modů, ale tento profil jich potřebuje { $wanted }.
        }
    .todo = Vypněte některé mody nebo vyberte méně doplňků.
e21 = Profil s názvem „{ $name }“ už existuje.
    .why = Názvy se porovnávají bez ohledu na velká a malá písmena, stejně jako názvy souborů.
    .todo = Zvolte jiný název.
e22 = Název „{ $name }“ nelze použít pro profil.
    .why = Windows ho nepovoluje jako název souboru — například CON, tečku na konci nebo kterýkoli ze znaků \ / : * ? " < > |.
    .todo = Zvolte jiný název.
e23 = deadloader nemůže přečíst svůj soubor { $file }.
    .why =
        { $reason ->
            [damaged_at] Soubor je poškozený (řádek { $line }, sloupec { $column }).
            [damaged] Soubor je poškozený.
            [newer] Byl vytvořen novější verzí programu deadloader.
           *[invalid] Jeho obsah neodpovídá tomu, co deadloader očekává.
        }
    .todo =
        { $kind ->
            [settings] deadloader zatím používá výchozí nastavení a soubor ponechá beze změny.
            [manifest] Obnovte soubor manifest.json.bak nebo aktualizujte deadloader.
           *[other] Obnovte soubor ze zálohy nebo aktualizujte deadloader.
        }
e24 = Soubor už na GameBanana není.
    .why = Autor ho odstranil nebo nahradil (HTTP { $status }).
    .todo = Otevřete stránku modu a vyberte jiný soubor, nebo zkontrolujte, zda mod nemá aktualizaci.
e25 = GameBanana žádost odmítl.
    .why = Server odpověděl kódem { $status }. Obvykle jde o dočasný problém.
    .todo = Zkuste to později. Pokud se to opakuje, nahlaste to na stránce programu deadloader na GitHubu.
internal = V programu deadloader se něco pokazilo.
    .why = Je to chyba programu deadloader, ne vaše.
    .todo = Zkuste to znovu. Pokud se to bude opakovat, nahlaste to i se záznamem protokolu z Podrobností.

## Buttons named in the texts above

action-undo = Zpět
action-review = Zkontrolovat
action-retry = Zkusit znovu
action-retry-now = Zkusit znovu teď
action-browse = Procházet…
action-verify-in-steam = Ověřit ve Steamu
action-show-differences = Zobrazit rozdíly
action-details = Podrobnosti
action-repair = Opravit
action-copy-link = Kopírovat odkaz
action-open-folder = Otevřít složku
action-open-release-page = Otevřít stránku vydání
action-enable-catalog = Zapnout katalog
action-learn-more = Další informace
action-view-log = Zobrazit protokol

## Shared pieces

log-entry = Záznam protokolu { $id }
list-more =
    { $count ->
        [one] { $count } další
        [few] { $count } další
        [many] { $count } dalšího
       *[other] { $count } dalších
    }

## Sizes, labelled like Windows Explorer labels them

size-bytes =
    { $count ->
        [one] { $count } bajt
        [few] { $count } bajty
        [many] { $count } bajtu
       *[other] { $count } bajtů
    }
size-kb = { $value } kB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
