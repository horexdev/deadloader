# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Průběh nastavení
wizard-caption-setup = První nastavení
wizard-caption-changed = Aktualizované upozornění
wizard-caption-catalog = Zapnutí katalogu
wizard-caption-review = Upozornění při prvním spuštění
wizard-language = Jazyk
wizard-step-welcome = Vítejte
wizard-step-risks = Rizika
wizard-step-network = Síť
wizard-step-privacy = Soukromí
wizard-step-updates = Aktualizace
wizard-step-setup = Nastavení
wizard-step-of = Krok { $n } z 6
wizard-changed-of =
    { $count ->
        [one] { $count } změněný krok
        [few] { $count } změněné kroky
       *[other] { $count } změněných kroků
    }
wizard-meta-changed = Změněno
wizard-meta-kept = Beze změny
wizard-back = Zpět
wizard-next = Další
wizard-finish = Dokončit nastavení
wizard-continue = Pokračovat
wizard-close = Zavřít
wizard-cancel = Zrušit
wizard-to-catalog = Potom se otevře katalog.
wizard-to-library = Potom se otevře knihovna (místní režim).

# Step 1
wizard-s1-title = Vítejte v deadloaderu
wizard-s1-lead = deadloader instaluje mody do Deadlocku z GameBanana nebo z vašeho disku a ukazuje vám každou změnu, kterou provede.
wizard-f1a = Přidává soubory modů do složky modů hry a jeden řádek do
wizard-f1b = — souboru hry, který Deadlocku říká, aby načítal mody.
wizard-f2 = Vše jde vrátit: vrácení odstraní ze hry všechno, co tam deadloader přidal.
wizard-f3a = Všechna jeho data zůstávají ve složce
wizard-f3b = vedle
wizard-f3c = .
wizard-f4 = deadloader není spojen s Valve ani s GameBanana.
wizard-f5 = Svobodný software pod licencí GPL-3, bez jakékoli záruky.
wizard-licence = Licence (GPL-3)
wizard-source = Zdrojový kód

# Step 2
wizard-s2-title = Mody na vlastní riziko
wizard-r1 = Valve nemá pro Deadlock žádná oficiální pravidla pro mody. Používat mody je vaše rozhodnutí a vaše riziko.
wizard-r2 = Aktualizace hry mohou mody rozbít. deadloader se po aktualizaci znovu nastaví, ale mod nemusí fungovat, dokud ho autor neaktualizuje.
wizard-r3 = Mody vytvářejí jiní lidé pod vlastními licencemi. deadloader nemůže ručit za jejich obsah; u každého souboru ukazuje výsledek antivirové kontroly GameBanana.
wizard-agree = Rozumím, že deadloader mění soubory hry a že mody používám na vlastní riziko.
wizard-agree-note = Tím potvrzujete kroky 1 a 2. Bez toho nelze pokračovat.
wizard-agree-error = Zaškrtněte toto políčko, abyste mohli pokračovat.
wizard-agree-reason = Nejdřív zaškrtněte políčko.

# Step 3
wizard-s3-title = Síť a GameBanana
wizard-s3-lead = Katalog komunikuje přímo s neoficiálním API GameBanana — a s nikým jiným. Toto jsou jediné servery, které kontaktuje:
wizard-hosts = Servery
wizard-host-api = katalog a stránky modů
wizard-host-images = náhledy a galerie
wizard-host-files = stahování
wizard-host-cache = stahování (files.gamebanana.com sem přesměrovává)
wizard-host-new = Nový
wizard-s3-images = Obrázky z jiných serverů v popisech modů se načtou až poté, co každý server povolíte. Každý požadavek je vidět v sekci
wizard-s3-images-end = .
wizard-activity = Aktivita
wizard-s3-tos = Používat katalog znamená používat GameBanana: platí jejich podmínky použití a GameBanana vyžaduje, aby uživatelům bylo alespoň 13 let.
wizard-tos = Podmínky použití GameBanana
wizard-choice = Jak chcete začít?
wizard-use-catalog = Používat katalog
wizard-use-catalog-text = Procházet a instalovat mody z GameBanana. deadloader se připojuje jen k serverům výše.
wizard-local = Teď ne — místní režim
wizard-local-text = Žádné síťové požadavky; mody jen z disku. Katalog můžete zapnout později v nastavení.
wizard-choice-error = Vyberte jednu ze dvou možností.
wizard-choice-reason = Nejdřív vyberte možnost.
wizard-last-time = Vaše předchozí volba
wizard-changed-title = Co se změnilo ({ $date })
wizard-changed-foot = Ostatní kroky jste potvrdili { $date }; nezměnily se.

# Step 4
wizard-s4-title = Vaše soukromí
wizard-p1 = Žádný účet a žádná telemetrie. deadloader nikam nic neposílá kromě požadavků na GameBanana z předchozího kroku.
wizard-p1-local = Žádný účet a žádná telemetrie. Zvolili jste místní režim, takže deadloader neposílá vůbec nic.
wizard-p2a = Protokoly zůstávají v tomto počítači, ve složce
wizard-p2b = — a skrývají vaše uživatelské jméno ve Windows:
wizard-p3 = Okno deadloaderu kreslí Microsoft WebView2, součást Windows. deadloader vypíná vlastní online funkce WebView2; Windows aktualizuje WebView2 nezávisle na deadloaderu.
wizard-p4 = Co deadloader udělal, můžete kdykoli zjistit v sekci
wizard-p4-end = .

# Step 5
wizard-s5-title = Aktualizace deadloaderu
wizard-u1 = deadloader může při spuštění zkontrolovat na GitHubu, zda existuje jeho novější verze.
wizard-updates-label = Při spuštění hledat na GitHubu nové verze deadloaderu
wizard-updates-on = Zapnuto
wizard-updates-off = Vypnuto (výchozí)
wizard-updates-na = V místním režimu nedostupné
wizard-u2a = Když je zapnuto, deadloader při každém spuštění pošle jeden požadavek na
wizard-u2b = a řekne vám, zda existuje nová verze.
wizard-u3 = Aktualizace je vždy ruční: novou verzi si stáhnete z GitHubu sami. deadloader se nikdy sám nenahradí.
wizard-u4 = Nedostupné: místní režim neposílá vůbec žádné síťové požadavky. Možnost bude dostupná, až v nastavení zapnete katalog.
wizard-u5 = Později to můžete změnit v Nastavení → Obecné.

# Step 6
wizard-s6-title = Nastavení
wizard-s6-lead = deadloader hledal hru a ověřil pár věcí. Zatím se nic nezměnilo a vše můžete později upravit v nastavení.
wizard-checking = Kontrola…
wizard-topic-game = Hra
wizard-topic-exe = Umístění deadloaderu
wizard-topic-disk = Místo na disku
wizard-topic-foreign = Složka modů hry
wizard-topic-gameinfo = Soubor hry pro mody
wizard-game-found = Deadlock nalezen
wizard-game-chosen = Složka vybrána
wizard-change = Změnit…
wizard-game-missing = Deadlock nebyl nalezen
wizard-game-missing-text = Steam nebo hra není nainstalovaná, nebo byla složka hry přesunuta. Vyberte složku hry sami — teď, nebo později v Nastavení → Hra.
wizard-browse = Procházet…
wizard-browse-title = Vyberte složku Deadlocku
wizard-help = Nápověda
wizard-help-a = Ve Steamu: klikněte pravým tlačítkem na Deadlock → Spravovat → Procházet místní soubory. Vyberte tuto složku; obvykle končí na
wizard-help-b = .
wizard-exe-ok = Ve stálé složce
wizard-exe-downloads = deadloader je ve složce Stažené soubory
wizard-exe-desktop = deadloader je na ploše
wizard-exe-warn-a = Jeho data leží vedle exe, takže úklid by smazal vaši knihovnu a profily. Zavřete deadloader, přesuňte celou složku na stálé místo, např.
wizard-exe-warn-b = , a spusťte ho odtud.
wizard-open-folder = Otevřít složku
wizard-opened = Otevřeno v Průzkumníkovi
wizard-linked = Mody se propojují — bez dalšího místa na disku
wizard-linked-text = deadloader i hra jsou na stejném disku ({ $drive }).
wizard-copied = Mody se budou kopírovat, ne propojovat
wizard-copied-text = deadloader je na { $data } a hra na { $game }. Každý zapnutý mod pak zabere místo podruhé; knihovna to ukazuje jako „zkopírováno — +N MB“.
wizard-learn-more = Další informace
wizard-drives-more = Propojení umožní hře použít soubor z knihovny bez druhé kopie. Windows umí propojit jen soubory na stejném disku. Pokud bude deadloader také na { $game }, nic se nekopíruje.
wizard-foreign-none = Žádné cizí soubory modů
wizard-foreign-none-text = Ve složce modů není nic, co by deadloader neznal.
wizard-foreign-some =
    { $count ->
        [one] { $count } soubor z jiného zdroje
        [few] { $count } soubory z jiného zdroje
       *[other] { $count } souborů z jiného zdroje
    }
wizard-foreign-some-text = Nedal je tam deadloader — nejspíš jiný správce modů nebo ruční instalace. Nechá je, jak jsou; prohlédněte si je později v knihovně.
wizard-show-files = Zobrazit soubory
wizard-hide-files = Skrýt soubory
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Bude nastaven
wizard-gi-setup-a = deadloader přidá jeden řádek do
wizard-gi-setup-b = při prvním použití modů. Teď se nic nemění.
wizard-gi-ours = Už nastaven
wizard-gi-ours-text = Řádek deadloaderu už tam je.
wizard-gi-foreign = Změněn jiným programem
wizard-gi-foreign-text = Nevypadá jako originál — často to dělá Deadlock Mod Manager. deadloader nepřepisuje cizí změny a po nastavení vysvětlí, co dělat.
wizard-what-is = Co to je?
wizard-gi-more = je soubor hry, který Deadlocku říká, jaké složky načíst. Obvykle pomůže ověřit soubory hry ve Steamu; potom deadloader přidá svůj řádek.

# E1
wizard-nw-title = deadloader nemůže uložit svá data.
wizard-nw-why-a = Složka vedle
wizard-nw-why-b = je jen pro čtení: Windows nedovolí programům do ní zapisovat (například v Program Files).
wizard-nw-path = Složka deadloaderu
wizard-nw-do = Co dělat
wizard-nw-s1 = Ukončete deadloader.
wizard-nw-s2a = Přesuňte celou složku na místo, kam můžete zapisovat, například
wizard-nw-s2b = . Nepoužívejte Program Files.
wizard-nw-s3a = Spusťte
wizard-nw-s3b = z nového místa.
wizard-nw-why-folder = deadloader má vše vedle exe, aby se nic nerozházelo po počítači: smazáním složky deadloader úplně odstraníte.
wizard-nw-details = Podrobnosti
wizard-nw-copy = Kopírovat podrobnosti
wizard-nw-copied = Zkopírováno
wizard-nw-no-log = Neuloženo do protokolu — deadloader tam nemůže zapisovat.
wizard-exit = Ukončit
