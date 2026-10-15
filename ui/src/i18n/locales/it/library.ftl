# The Library (S4): header, states, list, mod details, conflicts (C1), deleting (D6).
library-title = Libreria
library-profile = Profilo
library-enabled-count = { $enabled } di { $total } attivi
library-disk = { $size } su disco
library-not-checked = Aggiornamenti non ancora controllati
library-checked-today = Aggiornamenti controllati oggi alle { $time }
library-checked-on = Aggiornamenti controllati il { $date }
library-game-not-found = Gioco non trovato
library-add-disk = Aggiungi dal disco
library-add-title = Aggiungi mod dal disco
library-add-filter = File di mod
library-added =
    { $count ->
        [one] { $count } mod aggiunta alla libreria.
       *[other] { $count } mod aggiunte alla libreria.
    }
library-check = Controlla aggiornamenti
library-checking = Controllo…
library-update-all = Aggiorna tutto
library-review = Rivedi
library-undo = Annulla
library-cancel = Annulla
library-missing-what = Deadlock non è stato trovato.
library-missing-why = Steam o il gioco non sono installati, oppure la cartella del gioco è stata spostata. La libreria funziona comunque; applicare le modifiche e Gioca restano disattivati finché il gioco non viene trovato.
library-choose-folder = Scegli la cartella del gioco
library-search-again = Cerca di nuovo
library-running-what = Deadlock è in esecuzione.
library-running-why = Le modifiche verranno applicate automaticamente quando chiudi il gioco.
library-pending = Modifiche in sospeso
library-switch-what = Il profilo { $name } non è ancora nel gioco.
library-switch-why = Il gioco ha ancora { $applied }. Applica { $name } ora oppure premi Gioca: deadloader lo applica prima dell’avvio.
library-switch-why-first = Applica { $name } ora oppure premi Gioca: deadloader lo applica prima dell’avvio.
library-review-switch = Rivedi il cambio
library-applying-what = Applicazione del profilo { $name }…
library-applying-why = Puoi continuare a navigare; Gioca torna disponibile al termine.
library-show = Mostra
library-tab-all = Tutte
library-tab-enabled = Attive
library-tab-disabled = Disattivate
library-tab-updates = Aggiornamenti
library-tab-conflicts = Conflitti
library-tab-local = Locali
library-search = Cerca nella libreria
library-search-clear = Cancella ricerca
library-higher-wins = Quando due mod cambiano lo stesso file, vince quella più in alto nell’elenco.
library-reorder-hint = Riordina: trascina la maniglia, oppure selezionala e premi Spazio
library-list-label = Mod del profilo { $name }, in ordine di priorità
library-toggle = { $name } attiva
library-toggle-missing = Deadlock non è stato trovato: scegli la cartella del gioco per applicare le modifiche
library-grip = Sposta { $name }, posizione { $position } di { $total }
library-more = Altre azioni per { $name }
library-show-variants = Mostra varianti
library-hide-variants = Nascondi varianti
library-show-addons = Mostra componenti aggiuntivi
library-hide-addons = Nascondi componenti aggiuntivi
library-live-picked = { $name }: presa, posizione { $position } di { $total }. ↑ ↓ per spostarla, Spazio per rilasciarla, Esc per annullare.
library-live-moved = { $name }: posizione { $position } di { $total }.
library-live-moved-grabbed = { $name }: posizione { $position } di { $total }. Spazio per rilasciarla, Esc per annullare.
library-live-dropped = { $name }: rilasciata in posizione { $position } di { $total }.
library-live-back = { $name }: di nuovo in posizione { $position } di { $total }.
library-live-drag = { $name }: rilascia per metterla sopra { $above }, in posizione { $position } di { $total }.
library-badge-pending = In sospeso
library-badge-damaged = File danneggiati
library-badge-update = Aggiornamento disponibile
library-badge-wins = Conflitto · vince
library-badge-loses = Conflitto · perde
library-badge-variants =
    { $count ->
        [one] { $count } variante
       *[other] { $count } varianti
    }
library-badge-local = Locale
library-badge-copied = Copiata · +{ $size }
library-local = Locale
library-unknown-author = Autore sconosciuto
library-gamebanana = GameBanana
library-from-disk = dal disco
library-addons-sub =
    { $count ->
        [one] + { $count } componente aggiuntivo
       *[other] + { $count } componenti aggiuntivi
    }
library-variants = Varianti
library-variants-downloaded = { $count } scaricate
library-active = Attiva
library-select-to-switch = Seleziona per passare
library-parts = File di questa mod
library-main-files = File principali
library-from = da { $archive }
library-on-with-mod = Attivo con la mod · { $size }
library-remove-addon = Rimuovi componente aggiuntivo
library-toast-enabled = { $name } attivata.
library-toast-disabled = { $name } disattivata.
library-toast-variant = { $name } è passata a { $variant }.
library-empty-title = La tua libreria è vuota
library-empty-text = Installa mod dal catalogo oppure aggiungi file .vpk, .zip, .7z o .rar dal tuo computer. Puoi anche trascinare i file in un punto qualsiasi di questa finestra.
library-empty-local-text = In modalità locale il catalogo è disattivato. Aggiungi file .vpk, .zip, .7z o .rar dal tuo computer, oppure trascinali in un punto qualsiasi di questa finestra.
library-browse = Sfoglia il catalogo
library-no-match-title = Nessuna mod qui
library-no-match-text = Nessuna mod corrisponde a questa scheda e a questa ricerca. Prova un’altra scheda o cancella la ricerca.

## Mod details
library-details = Dettagli della mod
library-open-mod-page = Apri la pagina della mod
library-close-details = Chiudi i dettagli
library-enabled-in = Attiva in { $profile }
library-position = Posizione { $position } di { $total }
library-version = Versione
library-no-version = Nessuna versione indicata
library-from-gamebanana = Da GameBanana
library-imported = Importata dal disco
library-newer-in-library = Nella libreria c’è una versione più recente.
library-switch-to = Passa a
library-delete-old = Elimina versioni precedenti
library-no-older = Nessuna versione precedente conservata
library-older =
    { $count ->
        [one] { $count } versione precedente conservata · { $size }
       *[other] { $count } versioni precedenti conservate · { $size }
    }
library-files = File
library-on-disk = { $size } su disco
library-linked = Collegata: nessuno spazio extra su disco
library-copied = Copiata: { $size } di spazio extra su disco
library-not-in-game = Non nella cartella del gioco
library-local-mod = Mod locale
library-name = Nome nella libreria
library-rename = Rinomina
library-link-label = Pagina GameBanana
library-link-hint = Incolla l’indirizzo della mod per controllarne gli aggiornamenti.
library-link = Collega
library-link-done = Collegata: { $matched } file su { $total } trovati nella pagina.
library-link-not-a-page = Non è una pagina di mod di GameBanana. Incolla un indirizzo come gamebanana.com/mods/123456.
library-link-removed = Questa mod non è più su GameBanana.
library-link-failed = Impossibile leggere la pagina. Controlla l’indirizzo e riprova.
library-imported-from = Importata da
library-no-conflicts = Nessun conflitto con altre mod attive
library-conflict-wins =
    { $count ->
        [one] Vince su { $name } · { $count } file
       *[other] Vince su { $name } · { $count } file
    }
library-conflict-loses =
    { $count ->
        [one] Perde contro { $name } · { $count } file
       *[other] Perde contro { $name } · { $count } file
    }
library-show-conflicts = Mostra conflitti
library-delete-mod = Elimina mod…
library-open-folder = Apri cartella

## Conflicts (C1)
library-conflicts-sub = Due mod attive cambiano gli stessi file del gioco. Vince quella più in alto nell’elenco.
library-close-conflicts = Chiudi conflitti
library-no-conflicts-title = Nessun conflitto
library-wins = Vince
library-loses = Perde
library-off = Disattivata
library-in-use = In uso
library-conflict-text = Entrambe cambiano { $count } file. Nel gioco vedi le versioni di { $winner }; le copie di { $loser } non vengono usate.
library-conflict-off = Entrambe cambiano { $count } file. { $off } è disattivata, quindi il gioco usa le versioni di { $other }.
library-files-count = { $count } file
library-show-more = Mostra altri { $count }
library-move-above = Sposta { $name } sopra
library-disable = Disattiva { $name }
library-pick-variant = Scegli un’altra variante
library-no-variants = Nessuna delle due mod ha varianti
library-done-wins = Fatto: ora vince { $name }
library-done-off = Fatto: { $off } è disattivata; vengono usati i file di { $other }
library-identical = I file identici (stesso checksum) non sono conflitti e non vengono elencati.

## D6 Delete mod
library-d6-title = Eliminare “{ $name }”?
library-d6-files-title = Eliminare le versioni precedenti di “{ $name }”?
library-d6-what = { $count } file · { $size }.
library-d6-profiles =
    { $count ->
        [one] Usata in { $count } profilo: { $names }. Verrà rimossa da esso e dalla cartella del gioco.
       *[other] Usata in { $count } profili: { $names }. Verrà rimossa da essi e dalla cartella del gioco.
    }
library-d6-no-profiles = Nessun profilo la usa.
library-d6-where = Elimina
library-d6-permanently = Definitivamente
library-d6-recycle-bin = Nel Cestino
library-d6-permanent-hint = I file vengono eliminati per sempre: non si può annullare.
library-d6-bin-hint = Potrai ripristinarla dal Cestino di Windows.
library-d6-confirm = Elimina mod
library-d6-confirm-files = Elimina file
library-deleted = { $name } eliminata.
library-deleted-files = { $count } file di { $name } eliminati.
