# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Zavřít
library-apply = Použít
library-apply-profile = Použít { $name }
library-discard-all = Zahodit vše
library-keep = Ponechat { $name }
library-keep-applied = Ponechat použité změny
library-undo-this = Vrátit tuto změnu
library-undo-all = Vrátit vše
library-close-to-apply = Zavřete Deadlock — změny se pak použijí automaticky
library-play-applies = Hrát ho použije také
library-applied-title = Změny použity
library-nothing-pending = Nic nečeká
library-all-in-game = Vše z profilu { $name } už je ve hře.
library-running-sub = Deadlock běží, a proto Windows zamyká jeho soubory. Tyto změny se použijí automaticky, jakmile hru zavřete.
library-waiting-sub = Tyto změny ještě nejsou ve složce hry. Použijte je hned, nebo stiskněte Hrát.
library-switch-sub = Přepnutí profilu mění najednou mnoho souborů, a proto čeká na vás. Použijte ho tady, nebo stiskněte Hrát — deadloader ho použije před spuštěním.
library-applied-sub = Deadlock byl zavřen v { $time }, takže změny provedené během hry ({ $count }) byly použity. Vrátit je můžete až do další změny.
library-undoing-sub = Vrátili jste změny použité v { $time } ({ $count }) a Deadlock znovu běží. Vrátí se automaticky, až hru zavřete.
library-group-enable = Zapnout · { $count }
library-group-disable = Vypnout · { $count }
library-group-order = Pořadí · { $count }
library-group-variant = Varianta · { $count }
library-group-version = Verze · { $count }
library-group-switch = Přepnutí profilu · { $count }
library-group-undo = Vrácení změn použitých v { $time } ({ $count })
library-change-on = Vyp. → zap.
library-change-off = Zap. → vyp.
library-change-above = Přesunut nad { $name }
library-change-above-wins = Přesunut nad { $name } — teď vyhrává jejich konflikt
library-change-below = Přesunut pod { $name }
library-switch-what-happens = Zapne se modů: { $on } · vypne se: { $off }
library-what-happens = Co se stane ve složce hry
library-folder-summary = Přidaných souborů: { $added } · odebraných: { $removed } · přečíslovaných: { $renumbered }

## Foreign files
library-foreign-title = Nalezeno ve složce modů hry
library-foreign-note = Tyto položky sem nedal deadloader a nesáhne na ně, pokud se tak nerozhodnete.
library-collapse = Sbalit
library-expand = Rozbalit
library-take = Převzít do správy
library-leave = Ponechat beze změny
library-left = Ponecháno beze změny
library-foreign-open = Otevřít složku s { $name }
library-foreign-vpk = VPK, souborů: { $count }
library-foreign-vpk-hero = VPK, souborů: { $count } — vypadá jako skin pro { $hero }
library-foreign-unreadable = Soubor modu, který deadloader neumí přečíst
library-foreign-replaced = Soubor, který zabral místo souboru deadloaderu
library-foreign-dmm = Vytvořeno Deadlock Mod Managerem — soubor nastavení, ne mod
library-foreign-other = Soubor, ne mod
library-foreign-folder = Složka, souborů: { $count } — ne mod
library-foreign-meta = { $size } · přidáno { $date }
library-taken = { $name } je teď ve vaší knihovně.

## D11
library-d11-title = Převzít { $name } do správy?
library-d11-lead = deadloader:
library-d11-step-copy = ho zkopíruje do knihovny jako místní mod ({ $size }),
library-d11-step-replace = nahradí soubor ve složce hry vlastní propojenou kopií,
library-d11-step-enable = zapne ho v profilu { $profile } na konci seznamu.
library-d11-name = Název v knihovně
library-d11-hint = Knihovna uchovává přesnou kopii původního souboru.
library-d11-suggested = { $file } (skin { $hero })

## D10
library-d10-title = Soubor hry změnil jiný program
library-d10-what = Jiný program změnil soubor hry, který určuje, které složky Deadlock načítá — často Deadlock Mod Manager. deadloader cizí změny nepřepisuje, a proto se vaše mody nenačtou, dokud se to neopraví.
library-d10-do = Nechte Steam obnovit původní soubor (Ověřit soubory hry). Pak deadloader přidá vlastní řádek a dá vám vědět.
library-d10-diff = Zobrazit rozdíly
library-d10-expected = Očekáváno (originál + řádek deadloaderu)
library-d10-found = Nyní v souboru
library-d10-diff-note = Zvýrazněno: řádek, který přidává deadloader, a řádky přidané jiným programem.
library-d10-help = Co je to za soubor?
library-d10-help-text = gameinfo.gi říká Deadlocku, které složky má načítat. Správci modů přidávají jeden řádek, aby hra načítala mody ze složky addons. Aktualizace hry ho resetují; deadloader to opraví automaticky — ale jen když je soubor původní.
library-d10-wait-title = Steam ověřuje soubory Deadlocku.
library-d10-wait-text = Až Steam skončí, vraťte se do deadloaderu — soubor zkontroluje znovu automaticky. Můžete ho zkontrolovat i hned.
library-d10-fixed = gameinfo.gi je nastaven — mody se znovu načítají.
library-d10-fixed-text = Steam obnovil původní soubor a deadloader přidal svůj řádek. Podrobnosti v Aktivita → Soubory.
library-open-file = Otevřít soubor
library-not-now = Teď ne
library-verify-steam = Ověřit soubory hry ve Steamu
library-check-again = Zkontrolovat znovu
library-show-differences = Zobrazit rozdíly

## D5
library-update = Aktualizovat
library-update-on-gb = Na GameBanana je novější verze.
library-later = Později
library-d5-title = Aktualizovat { $name }?
library-d5-installed = Nainstalováno
library-d5-new = Nové na GameBanana
library-d5-downloaded = Staženo
library-d5-arrived = V knihovně, zatím neaktivní
library-d5-new-meta = Aktualizováno { $date } · ke stažení { $size }
library-d5-download = Ke stažení { $size }
library-d5-changed = Nová verze má jinou sadu souborů. deadloader je přiřadil takto:
library-d5-same = Tyto soubory nahradí jejich nové verze:
library-d5-map-text = deadloader nedokázal u každého souboru určit, který nový ho nahrazuje. Vyberte nový soubor ke každému, nebo ponechte starý.
library-d5-map-label = Přiřazení souborů
library-d5-col-old = Nainstalovaný soubor
library-d5-col-new = Nový soubor
library-d5-pick = Nový soubor pro { $file }
library-d5-choose = Vyberte…
library-d5-keep = Ponechat starý soubor
library-d5-tag-replaces = Nahrazuje
library-d5-tag-same = Stejný název
library-d5-tag-new = Nový · na stránce modu
library-d5-profiles = { $new } bude aktivní v každém profilu, který používá { $old }. { $old } zůstane v knihovně, takže se k ní můžete vrátit v podrobnostech modu.
library-d5-install = Nainstalovat aktualizaci
library-d5-confirm-map = Použít tyto soubory
library-update-downloading = Aktualizace { $name } se stahuje — nainstaluje se, jakmile dorazí.
library-updated = { $name } aktualizován.
library-updated-text = Aktivní v každém profilu, který používal starou verzi. Stará verze zůstává — můžete se k ní vrátit v podrobnostech modu.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name }: zapnuto
library-n12-off = { $name }: vypnuto
library-n12-above = { $name }: přesunut nad { $other }
library-n12-below = { $name }: přesunut pod { $other }
library-n12-switched = { $name }: přepnuto na { $to }
library-n12-files = { $name }: soubory změněny

## S9: the file list of a pending apply
library-show-file-list = Zobrazit seznam souborů
library-hide-file-list = Skrýt seznam souborů
library-file-list = Soubory ve složce hry
library-files-added = Přidané · { $count }
library-files-removed = Odebrané · { $count }
library-files-renumbered = Přečíslované · { $count }
