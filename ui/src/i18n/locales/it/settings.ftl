# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Impostazioni
settings-nav = Sezioni delle impostazioni
settings-reset-nav = Ripristina tutte le impostazioni…
settings-default = Predefinito: { $value }
settings-restore = Ripristina il predefinito
settings-on = Attivo
settings-off = Disattivo
settings-cancel = Annulla
settings-remove = Rimuovi
settings-remove-item = Rimuovi { $item }
settings-add = Aggiungi
settings-open-folder = Apri cartella
settings-open-folder-at = Apri cartella: { $path }
settings-copied = Copiato negli appunti.
settings-folder = Cartella
settings-example = Esempio:

settings-section-general = Generale
settings-section-general-lead = Lingua, avvio e zoom.
settings-section-game = Gioco
settings-section-game-lead = Dove è installato Deadlock e cosa può modificare lì deadloader.
settings-section-catalog = Catalogo
settings-section-catalog-lead = Il catalogo di GameBanana, i contenuti sensibili, le immagini e i link.
settings-section-library = Libreria
settings-section-library-lead = Aggiornamenti delle mod, eliminazione delle mod e versioni precedenti.
settings-section-downloads = Download
settings-section-downloads-lead = Coda dei download, archivi ed estrazione.
settings-section-network = Rete
settings-section-network-lead = Come deadloader comunica con GameBanana.
settings-section-cache = Cache
settings-section-cache-lead = Miniature e pagine del catalogo salvate su questo PC.
settings-section-logs = Log
settings-section-logs-lead = Cosa conserva Attività, e per quanto.
settings-section-advanced = Avanzate
settings-section-advanced-lead = File delle impostazioni e domini consentiti aggiuntivi.
settings-section-about = Informazioni su deadloader
settings-section-about-lead = Versione, licenza, come verificare questo download e dove sono i tuoi dati.

# General
settings-general-group = Lingua e avvio
settings-language = Lingua
settings-language-help = Lingua di sistema: la lingua di Windows se deadloader la offre, altrimenti l’inglese.
settings-language-system = Lingua di sistema ({ $language })
settings-startup-profile = Profilo all’avvio
settings-startup-profile-help = Quale profilo è selezionato all’avvio di deadloader. Selezionarlo non lo applica.
settings-startup-last = Ultimo selezionato ({ $name })
settings-startup-last-plain = Ultimo selezionato
settings-app-updates = Cerca aggiornamenti di deadloader all’avvio
settings-app-updates-help = All’avvio deadloader chiede una volta a GitHub (api.github.com) se esiste una versione più recente. L’aggiornamento è sempre manuale. L’hai scelto al primo avvio.
settings-app-updates-local = Non disponibile: la modalità locale non fa alcuna richiesta di rete. L’opzione diventa disponibile quando attivi il catalogo (Impostazioni → Catalogo).
settings-zoom = Zoom
settings-zoom-keys-1 = Anche con Ctrl + rotellina del mouse,
settings-zoom-keys-2 = e
settings-zoom-keys-3 = per ripristinare.
settings-ctrl = Ctrl
settings-percent = { $value }%

# Game
settings-installs-group = Installazioni del gioco
settings-installs-lead = deadloader trova da solo le installazioni di Deadlock su Steam. Ogni profilo ne usa una: cambiala in Profili.
settings-install-library = Libreria di Steam { $library }
settings-install-build = build { $build }
settings-install-used =
    { $count ->
        [0] non usata da nessun profilo
        [one] usata da 1 profilo: { $names }
       *[other] usata da { $count } profili: { $names }
    }
settings-install-selected = { $name } (selezionato)
settings-install-applied = { $name } (applicato qui)
settings-install-steam = Trovata tramite Steam
settings-install-manual = Aggiunta a mano
settings-install-known = Qui ci sono ancora file di deadloader
settings-install-manual-meta = Aggiunta a mano
settings-gi-ours = gameinfo.gi: OK — le righe di deadloader ci sono
settings-gi-wait = gameinfo.gi: configurato quando un profilo lo usa
settings-gi-rolled = gameinfo.gi: originale — Applica aggiunge di nuovo le righe di deadloader
settings-gi-foreign = gameinfo.gi: modificato da un altro programma
settings-gi-missing = gameinfo.gi: non trovato — forse la cartella è stata spostata
settings-installs-none = Nessuna installazione trovata finora.
settings-add-manual = Aggiungi a mano…
settings-add-manual-hint = Scegli una cartella di Deadlock che non è stata trovata.
settings-add-manual-title = Scegli la cartella di Deadlock
settings-install-added = Installazione aggiunta.
settings-install-removed = Installazione rimossa dall’elenco.
settings-install-removed-text = In quella cartella non è stato cambiato nulla.
settings-gameinfo-group = Dopo gli aggiornamenti del gioco
settings-autofix = Correggi gameinfo.gi automaticamente
settings-autofix-help = Un aggiornamento di Deadlock ripristina il file di gioco che gli dice di caricare le mod. deadloader ne fa una copia, rimette le sue righe e ti avvisa. Se disattivo, prima te lo chiede un banner.
settings-danger-group = Annulla e rimuovi
settings-rollback = Ripristina la cartella del gioco del profilo selezionato
settings-rollback-help = Per l’installazione del profilo selezionato { $profile } ({ $drive }): rimuove i { $files } file di mod che deadloader ha messo lì e le sue righe in gameinfo.gi. Libreria e profili restano; Applica riporta tutto.
settings-rollback-help-others = Le altre cartelle del gioco non vengono toccate; seleziona un profilo che le usa per ripristinarle.
settings-rollback-nothing = Niente da ripristinare su { $drive }: ora deadloader non ha file lì.
settings-rollback-no-game = Deadlock non è stato trovato, quindi non c’è nessuna cartella del gioco da ripristinare.
settings-rollback-button = Ripristina…
settings-rolled-title = Cartella del gioco su { $drive } ripristinata.
settings-rolled-text =
    { $foreign ->
        [0] Su { $drive } sono stati rimossi { $files } file di mod e le righe di deadloader in gameinfo.gi. Libreria e profili restano: Applica o Gioca riporta le mod.
       *[other] Su { $drive } sono stati rimossi { $files } file di mod, le righe di deadloader in gameinfo.gi e { $foreign } file estranei. Libreria e profili restano: Applica o Gioca riporta le mod.
    }
settings-wipe = Elimina tutti i dati di deadloader
settings-wipe-help = Elimina libreria, profili, log, cache e impostazioni ({ $size }), poi chiude deadloader. deadloader.exe resta: eliminalo tu dopo.
settings-wipe-button = Elimina tutti i dati…

# Catalog
settings-catalog-group = Catalogo di GameBanana
settings-catalog = Usa il catalogo
settings-catalog-help-on = Sfoglia e installa mod da GameBanana. deadloader parla solo con GameBanana e ogni richiesta compare in Attività. Disattivo significa modalità locale.
settings-catalog-help-off = Disattivo — modalità locale: deadloader non fa alcuna richiesta di rete. Puoi comunque aggiungere mod dal disco.
settings-consent-title = Prima un passaggio.
settings-consent-text = deadloader ti mostra quali host contatterà e i termini di GameBanana prima di attivare il catalogo.
settings-consent-go = Rivedi e attiva
settings-local-on = La modalità locale è attiva.
settings-local-on-text = Il catalogo è disattivo e deadloader non fa richieste di rete.
settings-ttl = Riusa le pagine del catalogo caricate per
settings-ttl-help = Entro questo tempo, tornare a una pagina non richiede di nuovo nulla a GameBanana.
settings-minutes = { $count } min
settings-hours = { $count } h
settings-prefetch = Carica in anticipo la pagina successiva
settings-prefetch-help = Quando hai scorso circa il 70% dei risultati, vengono caricati i successivi così la griglia continua.
settings-nsfw-group = Contenuti sensibili
settings-nsfw = Mod con classificazione dei contenuti (NSFW)
settings-nsfw-help = Nascoste: non vengono mostrate. Sfocate: un’immagine sfocata con l’etichetta «Contenuto sensibile»; clicca per vederla. Visibili: senza sfocatura.
settings-nsfw-age-done = Età confermata il { $date }.
settings-nsfw-age-ask = Lasciare «Nascoste» ti chiede una volta di confermare la tua età.
settings-nsfw-hidden = Nascoste
settings-nsfw-blurred = Sfocate
settings-nsfw-shown = Visibili
settings-images-group = Immagini e link
settings-description-images = Immagini nelle descrizioni delle mod
settings-description-images-help = Arrivano da host diversi da GameBanana, ad es. i.imgur.com. Al clic: un segnaposto finché non ci clicchi.
settings-images-click = Al clic
settings-images-always = Sempre
settings-prechecked = Spunta in anticipo «Consenti sempre le immagini da questo host»
settings-prechecked-help = Nella finestra mostrata la prima volta che un’immagine di una descrizione arriva da un host nuovo.
settings-hosts = Host di immagini consentiti
settings-hosts-help = Le immagini da questi host si caricano senza chiedere. In Attività le loro richieste sono segnate come «host di immagini consentito».
settings-hosts-built-in = Integrato
settings-hosts-none = Nessun host aggiunto: ogni host nuovo chiede prima.
settings-host-removed = { $host } rimosso
settings-host-removed-text = Le sue immagini chiederanno di nuovo prima di caricarsi.
settings-undo = Annulla
settings-gif = Riproduci le GIF automaticamente
settings-gif-help = Disattivo: le GIF mostrano il primo fotogramma con un pulsante di riproduzione.
settings-links = Link esterni
settings-links-help = I link si aprono nel browser predefinito. «Chiedi prima» mostra l’indirizzo completo prima di aprirlo.
settings-links-ask = Chiedi prima
settings-links-open = Apri subito

# Library
settings-updates-group = Aggiornamenti delle mod
settings-check-updates = Cerca aggiornamenti delle mod all’avvio
settings-check-updates-help = Confronta le tue mod di GameBanana con il catalogo. Mai in modalità locale.
settings-auto-download = Scarica gli aggiornamenti automaticamente
settings-auto-download-help = Solo il download: deadloader chiede comunque prima che una mod passi alla nuova versione.
settings-disk-group = Eliminazione e versioni precedenti
settings-deletion = Quando elimini una mod
settings-deletion-help = Puoi scegliere diversamente ogni volta nella finestra di eliminazione.
settings-deletion-permanent = Definitivamente
settings-deletion-bin = Nel cestino
settings-old-versions = Versioni precedenti
settings-old-versions-help = Ogni versione scaricata viene conservata per poter tornare indietro. Elimina le versioni precedenti di una mod nei suoi dettagli.
settings-old-keep-all = Conserva tutte

# Downloads
settings-queue-group = Coda
settings-parallel = Download simultanei
settings-parallel-help = Quanti file vengono scaricati contemporaneamente.
settings-speed = Limite di velocità
settings-speed-help = Solo per i download delle mod: catalogo e miniature non sono limitati.
settings-speed-none = Nessun limite
settings-speed-value = { $value } MB/s
settings-keep-archives = Conserva gli archivi dopo l’installazione
settings-keep-archives-help = Disattivo: il file .zip, .7z o .rar viene eliminato appena i suoi file .vpk sono nella libreria.
settings-unpack-group = Estrazione
settings-limits = Limiti di estrazione
settings-limits-help = Protegge dagli archivi che si estraggono in qualcosa di enorme. Se un archivio di cui ti fidi viene rifiutato, alza il limite.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = Dimensione estratta
settings-limit-ratio = Rapporto di compressione
settings-limit-files = File di mod per archivio
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = file
settings-unit-s = s
settings-number-error = { $field }: inserisci un numero intero da { $min } a { $max }. Fino ad allora resta in uso il valore precedente.

# Network
settings-network-local = La modalità locale è attiva.
settings-network-local-text = deadloader non fa richieste di rete; queste impostazioni valgono quando il catalogo è attivo.
settings-requests-group = Richieste a GameBanana
settings-api-parallel = Richieste simultanee al catalogo
settings-api-parallel-help = Richieste all’API di GameBanana in contemporanea. Immagini e download si contano a parte.
settings-timeouts = Timeout
settings-timeouts-help = Connessione: rinuncia se non si collega entro questo tempo. Risposta: rinuncia quando il server smette di inviare.
settings-timeouts-default = 10 s · 30 s
settings-timeout-connect = Connessione
settings-timeout-read = Risposta
settings-retries = Tentativi
settings-retries-help = Quante volte una richiesta fallita viene ripetuta, con pause crescenti, prima che deadloader mostri un errore.
settings-connection-group = Connessione
settings-proxy = Proxy
settings-proxy-help = Nessuno: connessione diretta. Sistema: il proxy impostato in Windows. Manuale: inseriscilo qui sotto.
settings-proxy-none = Nessuno
settings-proxy-system = Sistema
settings-proxy-manual = Manuale
settings-proxy-host = Indirizzo
settings-proxy-port = Porta
settings-proxy-hint = Proxy HTTP. Ci passano solo le richieste di deadloader.
settings-proxy-invalid = Inserisci un nome host o un indirizzo senza nome utente né password, ad es. proxy.example.com.

# Cache
settings-thumbs-group = Miniature e pagine del catalogo
settings-cache-limit = Limite della cache
settings-cache-limit-help = Quando la cache è piena, se ne vanno prima le miniature che non vedi da più tempo.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = Dimensione attuale
settings-cache-breakdown = { $thumbnails } miniature · { $pages } pagine del catalogo
settings-cache-empty = Vuota: ancora niente in cache.
settings-cache-used = { $used } su { $limit }
settings-cache-clear = Svuota cache
settings-cache-cleared = Cache svuotata: { $size } liberati. Le miniature si ricaricano quando servono.
settings-cache-cleared-partly = { $failed } file erano in uso e restano fino al prossimo svuotamento.

# Logs
settings-keep-group = Conservazione dei log
settings-log-days = Conserva i log per
settings-log-days-help = Le voci più vecchie vengono eliminate all’avvio di deadloader.
settings-days = { $count ->
        [one] { $count } giorno
       *[other] { $count } giorni
    }
settings-year = 1 anno
settings-log-size = Dimensione massima
settings-log-size-help = Oltre questa dimensione si eliminano prima i file di log più vecchi.
settings-privacy-group = Privacy
settings-mask = Nascondi il tuo nome utente di Windows
settings-mask-help = I percorsi nei log mostrano { $masked } invece del tuo nome: più sicuro quando condividi un log.
settings-logs-folder-group = Cartella dei log
settings-logs-folder = Dove sono i log
settings-logs-now = { $size } · { $files } file · il più vecchio del { $date }
settings-open-logs = Apri la cartella dei log

# Advanced
settings-advanced-warning = Per utenti esperti.
settings-advanced-warning-text = Un errore qui può cambiare il comportamento di deadloader. Controlla il file prima di salvarlo e conserva una copia della versione precedente.
settings-files-group = Modifica i file delle impostazioni
settings-file-settings = Impostazioni
settings-file-manifest = Manifest della cartella del gioco
settings-file-label = File da modificare
settings-file-settings-lead = Tutto ciò che è in queste pagine, in un unico file. Le modifiche compaiono nelle pagine dopo il salvataggio.
settings-file-manifest-lead = L’elenco di ciò che deadloader ha messo nella cartella del gioco. Il ripristino si basa su di esso: una voce sbagliata può lasciare file indietro.
settings-json-error = Questo non è JSON valido, quindi non è stato salvato.
settings-json-where = Riga { $line }, colonna { $column }: qui o poco prima manca o avanza qualcosa, spesso una virgola o una virgoletta.
settings-json-fix = Correggi quel punto e salva di nuovo; fino ad allora deadloader continua a usare il file attuale.
settings-json-refused = Questi valori non sono utilizzabili, quindi non è stato salvato nulla.
settings-json-newer = Questo file viene da un deadloader più recente (formato { $found }; questa versione legge fino a { $supported }), quindi non è stato salvato.
settings-json-invalid = deadloader non può usare questo file, quindi non è stato salvato.
settings-json-details = Dettagli
settings-problem-unknown-field = { $path }: non è un’impostazione di questa versione.
settings-problem-invalid-value = { $path }: questo valore non è utilizzabile qui.
settings-problem-out-of-range = { $path }: { $found } è fuori da { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: «{ $entry }» non è valido qui.
settings-problem-duplicate-entry = { $path }: «{ $entry }» compare due volte.
settings-problem-needs-age = { $path }: mostrare contenuti NSFW richiede prima la conferma dell’età (Impostazioni → Catalogo).
settings-problem-local-mode = { $path }: non disponibile in modalità locale.
settings-json-idle = Controllato al salvataggio.
settings-json-saved = Valido e salvato. Versione precedente conservata come { $backup }.
settings-json-saved-plain = Valido e salvato.
settings-json-save = Controlla e salva
settings-json-discard = Annulla le mie modifiche
settings-json-backup = Prima di salvare, il file precedente viene conservato come copia; restano le ultime 10.
settings-domains-group = Domini consentiti aggiuntivi
settings-domains-lead = deadloader parla solo con GameBanana. Se un download viene reindirizzato a un altro host e bloccato, puoi consentire quell’host qui. In Attività le richieste verso di esso sono segnate come «aggiunto dall’utente».
settings-domains-built-in = Integrati:
settings-domain-user = aggiunto dall’utente
settings-domains-none = Nessun dominio aggiuntivo.
settings-domain-add = Aggiungi un dominio
settings-domain-hint = Solo il nome host, ad es. files.example.com.
settings-domain-empty = Inserisci prima un nome host, ad es. files.example.com.
settings-domain-not-host = Inserisci solo il nome host, senza https:// né percorso, ad es. files.example.com.
settings-domain-invalid = «{ $host }» non è un nome host. Controlla come è scritto, ad es. files.example.com.
settings-domain-built-in = { $host } è integrato e sempre consentito.
settings-domain-listed = { $host } è già nell’elenco.

# About
settings-version = versione { $version }
settings-commit = Compilato dal commit
settings-commit-local = Build locale: non compilata da GitHub Actions
settings-copy-commit = Copia l’id del commit
settings-release-page = Pagina della release
settings-licence-group = Licenza
settings-licence-help = deadloader è software libero: puoi usarlo, studiarlo, condividerlo e modificarlo secondo la GPL-3. È fornito senza alcuna garanzia.
settings-read-licence = Leggi la licenza
settings-source = Codice sorgente
settings-verify-group = Come verificare questo download
settings-verify-lead = Due verifiche indipendenti che questo deadloader.exe è quello pubblicato su GitHub.
settings-verify-hash = Confronta il checksum SHA-256
settings-verify-hash-help = Deve corrispondere al checksum della pagina della release. Una copia modificata potrebbe mostrare qui un valore falso, quindi calcolalo anche tu:
settings-verify-hash-cmd = In PowerShell, nella cartella dell’exe:
settings-verify-attest = Verifica l’attestazione della build
settings-verify-attest-help = Dimostra che GitHub Actions ha compilato questo file dal codice sorgente pubblicato. Serve GitHub CLI:
settings-copy-hash = Copia il checksum
settings-copy-command = Copia il comando
settings-unsigned = deadloader.exe non ha la firma del codice.
settings-unsigned-text = Windows SmartScreen potrebbe dire «Windows ha protetto il PC» al primo avvio. Le due verifiche sopra servono a confermare che il file è autentico.
settings-webview-group = Motore della finestra
settings-webview-help = La finestra di deadloader è disegnata da WebView2, una parte di Windows. Windows lo aggiorna da solo. deadloader disattiva:
settings-webview-off-1 = Le query DNS-over-HTTPS
settings-webview-off-2 = L’accesso con l’account Microsoft nella finestra
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Il traffico di rete in background e gli aggiornamenti dei componenti
settings-webview-off-5 = Le chiamate al server della compilazione automatica dei moduli
settings-webview-off-6 = Le segnalazioni di errore a Microsoft (domain reliability)
settings-webview-no-route = E la finestra non ha alcuna via verso Internet.
settings-webview-unknown = versione sconosciuta
settings-data-group = I tuoi dati
settings-data-folder = Cartella dei dati
settings-data-folder-help = Tutto ciò che deadloader conserva è in quest’unica cartella accanto all’exe.
settings-notice = Avviso del primo avvio
settings-notice-help = Versione { $version }, accettata il { $date } in { $language }. Le tue scelte: catalogo { $catalog }, ricerca aggiornamenti { $updates }.
settings-notice-none = Non ancora accettato.
settings-notice-again = Rileggilo

# D7
settings-d7-title = Ripristinare la cartella del gioco su { $drive }?
settings-d7-from =
    { $others ->
        [0] L’installazione del profilo selezionato { $profile }. deadloader rimuove da
       *[other] L’installazione del profilo selezionato { $profile } (usata anche da { $others }). deadloader rimuove da
    }
settings-d7-files =
    { $count ->
        [one] 1 file di mod che ha aggiunto in
       *[other] { $count } file di mod che ha aggiunto in
    }
settings-d7-line = Le sue righe in gameinfo.gi
settings-d7-line-why = — il file torna com’era installato da Steam.
settings-d7-stays = Resta com’è:
settings-d7-library = La tua libreria — { $mods } mod, { $size }
settings-d7-profiles =
    { $count ->
        [one] Il tuo profilo — { $names }
       *[other] I tuoi { $count } profili — { $names }
    }
settings-d7-other = L’installazione su { $path } ({ $profile }): non viene toccata. Per ripristinarla, seleziona un profilo che la usa e ripristina di nuovo.
settings-d7-other-plain = L’installazione su { $path }: non viene toccata.
settings-d7-back = Applica o Gioca riporta tutto.
settings-d7-foreign =
    { $count ->
        [one] Rimuovi anche 1 file che deadloader non ha creato
       *[other] Rimuovi anche { $count } file che deadloader non ha creato
    }
settings-d7-foreign-why = da un altro programma o da un’installazione manuale. Vengono eliminati definitivamente; deadloader non può recuperarli.
settings-d7-confirm = Ripristina
settings-d7-confirm-foreign =
    { $count ->
        [one] Ripristina ed elimina 1 file
       *[other] Ripristina ed elimina { $count } file
    }

# D8
settings-d8-title = Eliminare tutti i dati di deadloader?
settings-d8-from = deadloader elimina tutto in
settings-d8-from-2 = e poi si chiude.
settings-d8-what = Cosa
settings-d8-contains = Contiene
settings-d8-size = Dimensione
settings-d8-total = Totale
settings-d8-library = Libreria
settings-d8-library-has = { $mods } mod, { $files } file
settings-d8-profiles = Profili
settings-d8-logs = Log
settings-d8-logs-has = { $files } file, dal { $date }
settings-d8-logs-none = nessun file di log
settings-d8-cache = Cache
settings-d8-cache-has = miniature e pagine del catalogo
settings-d8-settings = Impostazioni
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
        [one] incl. 1 copia
       *[other] incl. { $backups } copie
    }
settings-d8-other = Altro
settings-d8-other-has = download parziali, copie di gameinfo.gi, dati del motore della finestra
settings-d8-exe = Non eliminato: deadloader.exe stesso; eliminalo tu se non ti serve più.
settings-d8-rollback = Prima ripristina ogni cartella del gioco usata da deadloader (consigliato)
settings-d8-rollback-why = Rimuove i file di mod di deadloader e le sue righe in gameinfo.gi da ogni cartella del gioco prima di eliminare. I file estranei restano.
settings-d8-rollback-off = Non spuntato: i file di mod di deadloader restano nelle cartelle del gioco e Deadlock continua a caricarli, senza più deadloader per rimuoverli.
settings-d8-type-1 = Per confermare, scrivi
settings-d8-type-2 = { "" }
settings-d8-word = elimina
settings-d8-hint = Il pulsante di eliminazione si sblocca quando la parola corrisponde.
settings-d8-hint-ok = Confermato. Non si può annullare.
settings-d8-type-first = Scrivi prima «{ $word }»
settings-d8-deleting = Eliminazione…
settings-d8-confirm = Elimina e chiudi
settings-d8-confirm-rollback = Ripristina, elimina e chiudi

# Reset
settings-reset-title = Ripristinare tutte le impostazioni?
settings-reset-body = Ogni impostazione torna al predefinito, compresi gli host di immagini consentiti e i domini aggiuntivi.
settings-reset-keeps = La tua libreria, i tuoi profili e i log non vengono toccati, e la cartella del gioco resta com’è.
settings-reset-backup = Una copia delle impostazioni attuali viene conservata come settings.json.bak-…
settings-reset-confirm = Ripristina le impostazioni
settings-reset-done = Tutte le impostazioni sono state ripristinate.
settings-reset-done-text = Copia: { $backup }
