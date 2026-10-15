# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Attività
activity-sub =
    { $days ->
        [one] Tutto ciò che deadloader ha fatto su questo PC: richieste di rete, modifiche ai file ed eventi dell’app. I log restano su questo PC per { $days } giorno.
       *[other] Tutto ciò che deadloader ha fatto su questo PC: richieste di rete, modifiche ai file ed eventi dell’app. I log restano su questo PC per { $days } giorni.
    }
activity-open-logs = Apri la cartella dei log
activity-logs-opened = Cartella dei log aperta in Esplora file.
activity-summary = Oggi in breve
activity-net-today = Rete oggi
activity-hosts-label = Host contattati oggi — selezionane uno per filtrare
activity-hosts-contacted =
    { $count ->
        [one] { $count } host contattato
       *[other] { $count } host contattati
    }
activity-host-title =
    { $count ->
        [one] { $count } richiesta · { $source } — seleziona per filtrare
       *[other] { $count } richieste · { $source } — seleziona per filtrare
    }
activity-net-none-local = nessuna — modalità locale
activity-net-none-yet = nessuna finora
activity-nothing-else = Nient’altro.
activity-blocked-lead =
    { $count ->
        [one] { $count } richiesta a
       *[other] { $count } richieste a
    }
activity-blocked-tail =
    { $count ->
        [one] è stata bloccata prima di qualsiasi connessione.
       *[other] sono state bloccate prima di qualsiasi connessione.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } richiesta bloccata, mai contattata.
       *[other] { $count } richieste bloccate, mai contattate.
    }
activity-show-entry = Mostra voce
activity-game-today = Cartella del gioco oggi
activity-show-in-files = Mostra in File
activity-game-none = nessuna modifica
activity-no-game-changes = deadloader non ha ancora modificato nulla nella cartella del gioco.
activity-game-changes =
    { $count ->
        [one] { $count } modifica
       *[other] { $count } modifiche
    }
activity-game-batches =
    { $count ->
        [one] in { $count } passaggio
       *[other] in { $count } passaggi
    }
activity-game-failed =
    { $count ->
        [one] { $count } non riuscita
       *[other] { $count } non riuscite
    }
activity-game-hardlink = { $count } collegati
activity-game-copy = { $count } copiati
activity-game-delete = { $count } rimossi
activity-game-rename = { $count } rinominati
activity-game-write = { $count } scritti
activity-game-backup = { $count } salvati in backup
activity-game-restore = { $count } ripristinati
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi aggiornato una volta
       *[other] gameinfo.gi aggiornato { $count } volte
    }
activity-no-requests-today = Nessuna richiesta oggi.
activity-no-requests-yet = Nessuna richiesta finora.
activity-local-short = Il catalogo è disattivato, quindi deadloader non si connette a niente.
activity-never-contacted = deadloader non ha contattato nessun host da quando è stato installato.
activity-tab-network = Rete
activity-tab-files = File
activity-tab-app = App
activity-tab-off = disattivata
activity-tab-errors =
    { $count ->
        [one] · { $count } errore
       *[other] · { $count } errori
    }
activity-search = Cerca nel log
activity-search-clear = Cancella ricerca
activity-matches =
    { $count ->
        [one] { $count } risultato
       *[other] { $count } risultati
    }
activity-matches-more = { $count }+ risultati
activity-f-time = Periodo
activity-f-purpose = Scopo
activity-f-host = Host
activity-f-op = Operazione
activity-f-level = Livello
activity-f-result = Esito
activity-all = Tutti
activity-ok = OK
activity-errors = Errori
activity-range-hour = Ultima ora
activity-range-today = Oggi
activity-range-7d = Ultimi 7 giorni
activity-range-30d = Ultimi 30 giorni
activity-clear-filters = Azzera filtri
activity-clear-all = Azzera ricerca e filtri
activity-follow = Segui in diretta
activity-follow-tip = Mostra le nuove voci in alto appena arrivano
activity-follow-on = Le nuove voci compaiono in alto
activity-follow-paused = In pausa dalle { $time }
activity-show-new =
    { $count ->
        [one] Mostra { $count } nuova
       *[other] Mostra { $count } nuove
    }
activity-net-aria = Richieste di rete, dalla più recente
activity-files-aria = Operazioni sui file, dalla più recente
activity-app-aria = Eventi dell’app, dal più recente
activity-c-redirects = Reindirizzamenti
activity-c-time = Ora
activity-c-purpose = Scopo
activity-c-request = Richiesta
activity-c-status = Stato
activity-c-size = Dimensione
activity-c-duration = Durata
activity-c-started-by = Avviata da
activity-c-source = Origine host
activity-c-op = Operazione
activity-c-from-to = Da → a
activity-c-result = Esito
activity-c-change = Parte della modifica
activity-c-level = Livello
activity-c-message = Messaggio
activity-to = a
activity-loading-more = Caricamento…
activity-p-catalog = Catalogo
activity-p-thumbnail = Miniatura
activity-p-description-image = Immagine della descrizione
activity-p-download = Download
activity-p-update-check = Controllo aggiornamenti
activity-src-builtin = Integrato
activity-src-github = GitHub
activity-src-user = Aggiunto da te
activity-src-user-image = Host di immagini
activity-src-blocked = Non consentito
activity-src-long-builtin = Integrato — GameBanana, parte di deadloader
activity-src-long-github = GitHub — il controllo aggiornamenti di deadloader, attivo nelle impostazioni
activity-src-long-user = Aggiunto da te — in Impostazioni → Avanzate
activity-src-long-user-image = Host di immagini consentito — l’hai consentito caricando un’immagine della descrizione
activity-src-long-blocked = Non è nell’elenco degli host consentiti
activity-by-auto = Auto
activity-by-user = Tu
activity-op-hardlink = Collegamento
activity-op-copy = Copia
activity-op-rename = Rinomina
activity-op-delete = Eliminazione
activity-op-write = Scrittura
activity-op-backup = Backup
activity-op-restore = Ripristino
activity-lv-debug = Debug
activity-lv-info = Info
activity-lv-warn = Avviso
activity-lv-error = Errore
activity-st-blocked = Bloccata
activity-st-blocked-long = Bloccata — mai contattata
activity-st-failed = Non riuscita
activity-redirects =
    { $count ->
        [one] { $count } reindirizzamento
       *[other] { $count } reindirizzamenti
    }
activity-retries =
    { $count ->
        [one] { $count } nuovo tentativo
       *[other] { $count } nuovi tentativi
    }
activity-show-redirects = Mostra reindirizzamenti
activity-hide-redirects = Nascondi reindirizzamenti
activity-hop-request = Passo 1 · richiesta
activity-hop-redirect = Passo { $n } · reindirizzamento
activity-hop-retry = Passo { $n } · tentativo { $try }
activity-ms = { $ms } ms
activity-seconds = { $s } s
activity-no-match = Nessuna voce corrisponde
activity-no-match-text = Prova un altro periodo o azzera filtri e ricerca.
activity-empty-title = Ancora niente
activity-empty-text = deadloader annota qui ogni richiesta di rete, ogni modifica ai file nella cartella del gioco e ogni evento dell’app appena accadono. Finora non è successo niente.
activity-local-title = Modalità locale — non viene fatta nessuna richiesta
activity-local-why = Il catalogo è disattivato, quindi deadloader non si connette a niente: né a GameBanana né altrove. Le modifiche ai file e gli eventi dell’app vengono comunque registrati nelle altre schede.
activity-local-do = Attiva il catalogo nelle impostazioni per sfogliare GameBanana. Ogni richiesta comparirà poi qui.
activity-local-history = Le richieste qui sotto sono state fatte prima di attivare la modalità locale.
activity-enable-catalog = Attiva catalogo
activity-details = Dettagli della voce di log
activity-close-details = Chiudi dettagli
activity-close-esc = Chiudi (Esc)
activity-why = Perché:
activity-todo = Cosa fare:
activity-open-advanced = Apri Impostazioni → Avanzate
activity-full-record = Voce completa
activity-hops-title = Reindirizzamenti · { $count }
activity-user-hidden = Il tuo nome utente di Windows è nascosto nei log:
activity-settings-logs = Impostazioni → Log
activity-copy = Copia voce
activity-copied = Copiato
activity-kind-network = Richiesta di rete
activity-kind-files = Operazione su file
activity-kind-app = Evento dell’app
activity-d-time = Ora
activity-d-purpose = Scopo
activity-d-started-by = Avviata da
activity-d-request = Richiesta
activity-d-source = Origine host
activity-d-status = Stato
activity-d-size = Dimensione
activity-d-duration = Durata
activity-d-range = Intervallo
activity-d-error = Errore
activity-d-id = ID voce
activity-d-request-id = ID richiesta
activity-d-operation = Operazione
activity-d-from = Da
activity-d-to = A
activity-d-file = File
activity-d-result = Esito
activity-d-change = Parte della modifica
activity-d-sha256 = SHA-256
activity-d-level = Livello
activity-d-message = Messaggio
activity-d-fields = Dettagli
activity-status-after =
    { $count ->
        [one] { $status } — dopo { $count } reindirizzamento
       *[other] { $status } — dopo { $count } reindirizzamenti
    }
