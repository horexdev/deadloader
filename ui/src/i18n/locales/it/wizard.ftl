# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Avanzamento della configurazione
wizard-caption-setup = Prima configurazione
wizard-caption-changed = Avviso aggiornato
wizard-caption-catalog = Attiva il catalogo
wizard-caption-review = Avviso del primo avvio
wizard-language = Lingua
wizard-step-welcome = Benvenuto
wizard-step-risks = Rischi
wizard-step-network = Rete
wizard-step-privacy = Privacy
wizard-step-updates = Aggiornamenti
wizard-step-setup = Configurazione
wizard-step-of = Passo { $n } di 6
wizard-changed-of =
    { $count ->
        [one] { $count } passo modificato
       *[other] { $count } passi modificati
    }
wizard-meta-changed = Modificato
wizard-meta-kept = Invariato
wizard-back = Indietro
wizard-next = Avanti
wizard-finish = Completa la configurazione
wizard-continue = Continua
wizard-close = Chiudi
wizard-cancel = Annulla
wizard-to-catalog = Poi si apre il catalogo.
wizard-to-library = Poi si apre la tua libreria (modalità locale).

# Step 1
wizard-s1-title = Benvenuto in deadloader
wizard-s1-lead = deadloader installa mod per Deadlock da GameBanana o dal tuo disco e ti mostra ogni modifica che fa.
wizard-f1a = Aggiunge file di mod alla cartella delle mod del gioco e tre righe a
wizard-f1b = — un file del gioco che dice a Deadlock di caricare le mod.
wizard-f2 = Tutto si può annullare: il ripristino toglie dal gioco tutto ciò che deadloader vi ha messo.
wizard-f3a = Tutti i suoi dati restano nella cartella
wizard-f3b = accanto a
wizard-f3c = .
wizard-f4 = deadloader non è affiliato a Valve né a GameBanana.
wizard-f5 = Software libero con licenza GPL-3, fornito senza alcuna garanzia.
wizard-licence = Licenza (GPL-3)
wizard-source = Codice sorgente

# Step 2
wizard-s2-title = Le mod sono a tuo rischio
wizard-r1 = Valve non ha regole ufficiali sulle mod per Deadlock. Usare le mod è una tua decisione e un tuo rischio.
wizard-r2 = Gli aggiornamenti del gioco possono rompere le mod. deadloader si riconfigura dopo un aggiornamento, ma una mod potrebbe non funzionare finché il suo autore non la aggiorna.
wizard-r3 = Le mod sono create da altre persone con le loro licenze. deadloader non può garantirne il contenuto; per ogni file mostra il risultato dell’antivirus di GameBanana.
wizard-agree = Capisco che deadloader modifica file del gioco e che uso le mod a mio rischio.
wizard-agree-note = Questo conferma i passi 1 e 2. È necessario per continuare.
wizard-agree-error = Spunta questa casella per continuare.
wizard-agree-reason = Prima spunta la casella.

# Step 3
wizard-s3-title = Rete e GameBanana
wizard-s3-lead = Il catalogo parla direttamente con l’API non ufficiale di GameBanana, e con nessun altro. Questi sono gli unici host che contatta:
wizard-hosts = Host
wizard-host-api = catalogo e pagine delle mod
wizard-host-images = miniature e galleria
wizard-host-files = download
wizard-host-cache = download (files.gamebanana.com reindirizza qui)
wizard-host-new = Nuovo
wizard-s3-images = Le immagini di altri host nelle descrizioni delle mod si caricano solo dopo che hai consentito ciascun host. Ogni richiesta compare in
wizard-s3-images-end = .
wizard-activity = Attività
wizard-s3-tos = Usare il catalogo significa usare GameBanana: valgono i loro termini di servizio, e GameBanana richiede che gli utenti abbiano almeno 13 anni.
wizard-tos = Termini di servizio di GameBanana
wizard-choice = Come vuoi iniziare?
wizard-use-catalog = Usa il catalogo
wizard-use-catalog-text = Sfoglia e installa mod da GameBanana. deadloader si collega solo agli host qui sopra.
wizard-local = Non ora — modalità locale
wizard-local-text = Nessuna richiesta di rete; mod solo dal disco. Puoi attivare il catalogo più tardi nelle impostazioni.
wizard-choice-error = Scegli una delle due opzioni.
wizard-choice-reason = Prima scegli un’opzione.
wizard-last-time = La tua scelta precedente
wizard-changed-title = Cosa è cambiato ({ $date })
wizard-changed-foot = Hai confermato gli altri passi il { $date }; non sono cambiati.

# Step 4
wizard-s4-title = La tua privacy
wizard-p1 = Nessun account e nessuna telemetria. deadloader non invia nulla da nessuna parte, tranne le richieste a GameBanana del passo precedente.
wizard-p1-local = Nessun account e nessuna telemetria. Hai scelto la modalità locale, quindi deadloader non invia proprio nulla.
wizard-p2a = I log restano su questo PC, nella cartella
wizard-p2b = — e nascondono il tuo nome utente di Windows:
wizard-p3 = La finestra di deadloader è disegnata da Microsoft WebView2, una parte di Windows. deadloader disattiva le funzioni online di WebView2; Windows aggiorna WebView2 indipendentemente da deadloader.
wizard-p4 = Puoi sempre vedere cosa ha fatto deadloader in
wizard-p4-end = .

# Step 5
wizard-s5-title = Aggiornamenti di deadloader
wizard-u1 = All’avvio deadloader può controllare su GitHub se esiste una sua versione più recente.
wizard-updates-label = Cerca nuove versioni di deadloader su GitHub all’avvio
wizard-updates-on = Attivo
wizard-updates-off = Disattivo (predefinito)
wizard-updates-na = Non disponibile in modalità locale
wizard-u2a = Se attivo, deadloader fa una richiesta a
wizard-u2b = a ogni avvio e ti dice se esiste una nuova versione.
wizard-u3 = L’aggiornamento è sempre manuale: scarichi tu la nuova versione da GitHub. deadloader non si sostituisce mai da solo.
wizard-u4 = Non disponibile: la modalità locale non fa alcuna richiesta di rete. L’opzione diventa disponibile quando attivi il catalogo nelle impostazioni.
wizard-u5 = Puoi cambiarlo più tardi in Impostazioni → Generale.

# Step 6
wizard-s6-title = Configurazione
wizard-s6-lead = deadloader ha cercato il gioco e controllato alcune cose. Non è ancora stato cambiato nulla, e puoi modificare tutto questo più tardi nelle impostazioni.
wizard-checking = Controllo in corso…
wizard-topic-game = Gioco
wizard-topic-exe = Posizione di deadloader
wizard-topic-disk = Spazio su disco
wizard-topic-foreign = Cartella delle mod del gioco
wizard-topic-gameinfo = File del gioco per le mod
wizard-game-found = Deadlock trovato
wizard-game-chosen = Cartella scelta
wizard-change = Cambia…
wizard-game-missing = Deadlock non trovato
wizard-game-missing-text = Steam o il gioco non sono installati, oppure la cartella del gioco è stata spostata. Scegli tu la cartella del gioco, ora o più tardi in Impostazioni → Gioco.
wizard-browse = Sfoglia…
wizard-browse-title = Scegli la cartella di Deadlock
wizard-help = Aiuto
wizard-help-a = In Steam: clic destro su Deadlock → Gestisci → Sfoglia i file locali. Scegli quella cartella; di solito termina con
wizard-help-b = .
wizard-exe-ok = In una cartella permanente
wizard-exe-downloads = deadloader è nella cartella Download
wizard-exe-desktop = deadloader è sul Desktop
wizard-exe-warn-a = I suoi dati stanno accanto all’exe, quindi fare pulizia cancellerebbe libreria e profili. Chiudi deadloader, sposta l’intera cartella in un posto permanente, ad esempio
wizard-exe-warn-b = , e avvialo da lì.
wizard-open-folder = Apri cartella
wizard-opened = Aperta in Esplora file
wizard-linked = Le mod sono collegate — nessuno spazio in più su disco
wizard-linked-text = deadloader e il gioco sono sulla stessa unità ({ $drive }).
wizard-copied = Le mod verranno copiate, non collegate
wizard-copied-text = deadloader è su { $data } e il gioco su { $game }. Ogni mod attiva occupa allora il suo spazio una seconda volta; la libreria lo indica come «copiata — +N MB».
wizard-learn-more = Scopri di più
wizard-drives-more = Un collegamento permette al gioco di usare il file della libreria senza una seconda copia. Windows può collegare solo file sulla stessa unità. Se anche deadloader è su { $game }, non viene copiato nulla.
wizard-foreign-none = Nessun file di mod estraneo
wizard-foreign-none-text = La cartella delle mod non contiene nulla che deadloader non conosca.
wizard-foreign-some =
    { $count ->
        [one] { $count } file da un’altra fonte
       *[other] { $count } file da un’altra fonte
    }
wizard-foreign-some-text = Non li ha messi deadloader: probabilmente un altro gestore di mod o un’installazione manuale. Li lascia come sono; esaminali più tardi nella libreria.
wizard-show-files = Mostra file
wizard-hide-files = Nascondi file
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Verrà configurato
wizard-gi-setup-a = deadloader aggiunge tre righe a
wizard-gi-setup-b = quando applichi le mod per la prima volta. Ora non viene cambiato nulla.
wizard-gi-ours = Già configurato
wizard-gi-ours-text = Le righe di deadloader ci sono già.
wizard-gi-foreign = Modificato da un altro programma
wizard-gi-foreign-text = Non sembra l’originale: spesso è opera di Deadlock Mod Manager. deadloader non sovrascrive le modifiche altrui e dopo la configurazione ti spiegherà cosa fare.
wizard-what-is = Che cos’è?
wizard-gi-more = è un file del gioco che dice a Deadlock quali cartelle caricare. Di solito si sistema verificando i file del gioco in Steam; poi deadloader aggiunge le sue righe.

# E1
wizard-nw-title = deadloader non può salvare i suoi dati.
wizard-nw-why-a = La cartella accanto a
wizard-nw-why-b = è di sola lettura: Windows non permette ai programmi di scriverci (ad esempio dentro Program Files).
wizard-nw-path = Cartella di deadloader
wizard-nw-do = Cosa fare
wizard-nw-s1 = Chiudi deadloader.
wizard-nw-s2a = Sposta l’intera cartella in un posto in cui puoi scrivere, ad esempio
wizard-nw-s2b = . Non usare Program Files.
wizard-nw-s3a = Avvia
wizard-nw-s3b = dal nuovo posto.
wizard-nw-why-folder = deadloader tiene tutto accanto all’exe per non spargere nulla sul tuo PC: eliminare la cartella rimuove deadloader del tutto.
wizard-nw-details = Dettagli
wizard-nw-copy = Copia dettagli
wizard-nw-copied = Copiato
wizard-nw-no-log = Non salvato nel log: deadloader non può scriverci.
wizard-exit = Esci
