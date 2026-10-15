# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Profili
profiles-count =
    { $count ->
        [one] { $count } profilo
       *[other] { $count } profili
    }
profiles-in-game = Nel gioco ora: { $list }
profiles-in-game-item = «{ $name }» su { $drive }
profiles-in-game-none = Nessun profilo è nel gioco in questo momento.
profiles-import = Importa…
profiles-import-title = Importa un profilo da un file .json
profiles-import-dialog = Importa un profilo
profiles-export-dialog = Esporta il profilo
profiles-file-filter = Profili di deadloader
profiles-new = Nuovo profilo
profiles-selected = Selezionato
profiles-applied = Applicato
profiles-not-applied = Non ancora applicato
profiles-selected-title = Il profilo che modifichi nella libreria e con cui giochi
profiles-pending-title = Applica o Gioca lo mette nella cartella del gioco
profiles-applied-title = I suoi file ora sono nella cartella del gioco su { $drive }
profiles-select = Seleziona
profiles-select-of = Seleziona «{ $name }»
profiles-more-of = Altre azioni per «{ $name }»
profiles-rename = Rinomina
profiles-duplicate = Duplica
profiles-export = Esporta come file…
profiles-change-installation = Cambia installazione del gioco…
profiles-delete = Elimina
profiles-delete-confirm = Elimina…
profiles-delete-last = deadloader ha bisogno di almeno un profilo.
profiles-name-label = Nome del profilo
profiles-save = Salva
profiles-cancel = Annulla
profiles-close = Chiudi
profiles-close-esc = Chiudi (Esc)
profiles-rename-hint = Invio salva · Esc annulla
profiles-name-empty = Inserisci un nome per il profilo.
profiles-mods =
    { $count ->
        [one] { $count } mod attivata
       *[other] { $count } mod attivate
    }
profiles-mods-none = Nessuna mod attivata
profiles-applied-when = Applicato { $when }
profiles-last-applied = Ultima applicazione { $when }
profiles-never-applied = Mai applicato
profiles-just-now = proprio ora
profiles-preview-empty = Seleziona questo profilo, poi attiva le mod nella libreria.
profiles-footnote = Un profilo è un elenco di mod attivate con il loro ordine, varianti e versioni. Selezionarlo non scarica né elimina nulla; Applica cambia solo i file diversi.
profiles-copy-name = { $name } (copia)
profiles-copy-name-n = { $name } (copia { $number })
profiles-new-title = Nuovo profilo
profiles-start-with = Inizia con
profiles-empty = Vuoto
profiles-empty-hint = Nessuna mod attivata: sceglile poi nella libreria.
profiles-copy-of = Copia di «{ $name }»
profiles-copy-hint = { $mods }: stesso ordine, varianti e versioni.
profiles-installation = Installazione del gioco
profiles-installation-hint = La stessa di «{ $name }». Puoi cambiarla più tardi dal menu del profilo.
profiles-new-note = Creare un profilo non lo seleziona né lo applica: il gioco non cambia.
profiles-create = Crea profilo
profiles-pending-what = Il profilo «{ $name }» non è ancora nel gioco.
profiles-pending-why-other = La cartella del gioco su { $drive } contiene ancora «{ $other }». Applicalo ora o premi Gioca: deadloader lo applica prima dell’avvio.
profiles-pending-why-empty = La cartella del gioco su { $drive } ora non ha mod di deadloader. Applicalo ora o premi Gioca: deadloader lo applica prima dell’avvio.
profiles-keep = Tieni «{ $name }»
profiles-apply = Applica «{ $name }»
profiles-other-what = «{ $name }» usa un’altra installazione del gioco.
profiles-other-why = Applica e Gioca usano { $path }. Il gioco su { $drive } tiene «{ $other }».
profiles-other-why-empty = Applica e Gioca usano { $path }. Il gioco su { $drive } non cambia.
profiles-delete-title = Eliminare il profilo «{ $name }»?
profiles-delete-applied =
    { $count ->
        [one] «{ $name }» è ora nel gioco. deadloader toglierà la sua { $count } mod dalla cartella del gioco:
       *[other] «{ $name }» è ora nel gioco. deadloader toglierà le sue { $count } mod dalla cartella del gioco:
    }
profiles-delete-applied-empty = «{ $name }» è ora nel gioco ma non ha mod, quindi la cartella del gioco non cambia.
profiles-delete-stay = Le mod restano nella tua libreria e negli altri profili. Finché non applichi lì un altro profilo, Deadlock parte senza mod di deadloader.
profiles-delete-next = «{ $name }» diventa il profilo selezionato. Non viene applicato finché non premi Applica o Gioca.
profiles-delete-undoable = Non si può annullare. Per tenerne una copia, esporta prima il profilo.
profiles-export-first = Esporta prima
profiles-delete-profile = Elimina profilo
profiles-installation-title = Installazione del gioco per «{ $name }»
profiles-installation-lead = Scegli la cartella di Deadlock a cui viene applicato questo profilo.
profiles-used-by =
    { $count ->
        [one] usata da { $count } profilo
       *[other] usata da { $count } profili
    }
profiles-used-by-none = non usata da alcun profilo
profiles-installation-applied = «{ $name }» è ora nel gioco su { $drive }. I suoi file restano lì finché non applichi un altro profilo su { $drive }.
profiles-installation-not-applied = Nessuna cartella del gioco cambia finché non applichi «{ $name }».
profiles-manage-installations = Gestisci le installazioni nelle Impostazioni
profiles-use-installation = Usa questa installazione
profiles-d9-title =
    { $count ->
        [one] { $count } mod di «{ $name }» non è nella tua libreria
       *[other] { $count } mod di «{ $name }» non sono nella tua libreria
    }
profiles-d9-file = Il file
profiles-d9-lead = elenca { $listed } mod. { $present } sono già nella tua libreria; queste { $count } no:
profiles-d9-list = Mod mancanti
profiles-d9-from-gamebanana = { $count } da GameBanana
profiles-d9-local-count = { $count } locali
profiles-gamebanana = GameBanana
profiles-can-download = Si può scaricare
profiles-local-mod = Mod locale
profiles-cant-download = Non si può scaricare
profiles-local-why = Aggiunta dal disco su un altro PC, quindi non è su GameBanana. Chiedi il file a chi ha condiviso il profilo.
profiles-d9-local-mode = Il catalogo è spento (modalità locale), quindi deadloader non può scaricare mod. Importa senza di esse e poi aggiungile dal disco, oppure accendi il catalogo nelle Impostazioni.
profiles-d9-after = In ogni caso «{ $name }» diventa il profilo selezionato ma non viene applicato: la cartella del gioco cambia solo quando premi Applica o Gioca.
profiles-import-without = Importa senza
profiles-download-missing = Scarica le mancanti ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } mod disattivata
       *[other] { $count } mod disattivate
    }
profiles-turned-on =
    { $count ->
        [one] { $count } mod attivata
       *[other] { $count } mod attivate
    }
profiles-differ = Sono stati cambiati solo i file diversi.
profiles-same = Le stesse mod erano già nel gioco; sono stati controllati solo ordine e varianti.
profiles-deleted = Profilo «{ $name }» eliminato.
profiles-deleted-not-applied = Non era applicato, quindi la cartella del gioco non è cambiata.
profiles-deleted-applied =
    { $count ->
        [one] { $count } mod è stata tolta dalla cartella del gioco. È ancora nella tua libreria.
       *[other] { $count } mod sono state tolte dalla cartella del gioco. Sono ancora nella tua libreria.
    }
profiles-exported = Profilo «{ $name }» esportato.
profiles-imported = Profilo «{ $name }» importato.
profiles-imported-text = È selezionato ma non ancora applicato.
profiles-imported-without =
    { $count ->
        [one] Profilo «{ $name }» importato senza { $count } mod.
       *[other] Profilo «{ $name }» importato senza { $count } mod.
    }
profiles-imported-without-text = È selezionato ma non ancora applicato. Puoi aggiungere le mod mancanti più tardi.
profiles-imported-downloading =
    { $count ->
        [one] Profilo «{ $name }» importato: { $count } mod in download.
       *[other] Profilo «{ $name }» importato: { $count } mod in download.
    }
profiles-imported-downloading-text = È selezionato ma non ancora applicato. Applicalo quando i download sono finiti.
profiles-open-downloads = Apri download
profiles-undo = Annulla
profiles-check-update = Cerca aggiornamenti di deadloader
profiles-choose-another = Scegli un altro file
profiles-switcher = Gestisci profili
profiles-saved-as = Salvato come
profiles-show-folder = Mostra nella cartella
