## Notifications

n1 = Deadlock è stato aggiornato: mod ripristinati.
    .why = L’aggiornamento ha reimpostato gameinfo.gi, quindi deadloader ha aggiunto di nuovo le sue righe.
n2 = { $name } installato e attivato.
n3 =
    { $count ->
        [one] { $count } aggiornamento di mod disponibile.
       *[other] { $count } aggiornamenti di mod disponibili.
    }
n4 = Deadlock è in esecuzione.
    .why = Le modifiche verranno applicate automaticamente quando chiudi il gioco.
    .pending =
        { $count ->
            [one] { $count } modifica in sospeso
           *[other] { $count } modifiche in sospeso
        }
n5 =
    { $reason ->
        [completed] Una modifica interrotta è stata completata.
       *[undone] Una modifica interrotta è stata annullata; la cartella del gioco è com’era prima.
    }
n6 =
    { $count ->
        [one] Nella cartella dei mod del gioco è stato trovato { $count } file non creato da deadloader.
       *[other] Nella cartella dei mod del gioco sono stati trovati { $count } file non creati da deadloader.
    }
n7 =
    { $when ->
        [today] Offline: viene mostrato il catalogo salvato alle { $saved }.
       *[earlier] Offline: viene mostrato il catalogo salvato il { $saved }.
    }
    .why = I download riprenderanno quando tornerai online.
n8 =
    { $count ->
        [one] Profilo “{ $name }” applicato: { $count } mod attivato.
       *[other] Profilo “{ $name }” applicato: { $count } mod attivati.
    }
n9 = È disponibile una nuova versione di deadloader: { $version }.
n10 = La modalità locale è attiva.
    .why = Il catalogo è disattivato e deadloader non effettua richieste di rete.
n11 = I mod vengono copiati anziché collegati, perché deadloader e il gioco si trovano su unità diverse.
    .why = Occupano { $size } di spazio su disco in più.
n12 =
    { $count ->
        [one] Deadlock è stato chiuso: { $count } modifica applicata.
       *[other] Deadlock è stato chiuso: { $count } modifiche applicate.
    }

## Errors: what happened, why, what to do

e1 = deadloader non riesce a salvare i propri dati.
    .why =
        { $reason ->
            [access_denied] La cartella di deadloader.exe è di sola lettura: ad esempio, Windows non permette che i programmi scrivano in “Programmi” (Program Files).
            [in_use] Un altro programma, spesso un antivirus, ha aperto uno dei file di deadloader.
           *[other] Windows non è riuscito a scrivere nella cartella di deadloader.exe.
        }
    .todo =
        { $reason ->
            [in_use] Chiudi gli altri programmi e riprova.
           *[other] Sposta deadloader in una cartella in cui puoi scrivere, ad esempio D:\Games\deadloader.
        }
e2 = Deadlock non trovato.
    .why = Steam o il gioco non sono installati, oppure la cartella del gioco è stata spostata.
    .todo = Scegli manualmente la cartella del gioco.
e3 = Un file del gioco è stato modificato da un altro programma.
    .why = gameinfo.gi non sembra l’originale: spesso lo modifica Deadlock Mod Manager. deadloader non sovrascrive le modifiche altrui.
    .todo = Verifica l’integrità dei file di gioco in Steam; poi deadloader aggiungerà le sue righe.
e4 = Il catalogo non è disponibile.
    .why = GameBanana ha cambiato la sua API, quindi deadloader non riesce a leggerla. La libreria e i profili funzionano come sempre.
    .todo = Controlla se è disponibile un aggiornamento di deadloader o riprova più tardi.
e5 = GameBanana è sovraccarico.
    .why =
        { $reason ->
            [rate_limited] Troppe richieste in poco tempo (HTTP 429).
            [server] Il server di GameBanana ha segnalato un errore.
           *[other] Troppe richieste o un errore del server.
        }
    .todo =
        { $wait ->
            [seconds]
                { $retry_in_s ->
                    [one] deadloader riproverà automaticamente tra { $retry_in_s } secondo.
                   *[other] deadloader riproverà automaticamente tra { $retry_in_s } secondi.
                }
           *[later] deadloader riproverà automaticamente tra poco.
        }
e6 = Download non riuscito.
    .why =
        { $reason ->
            [local_mode] deadloader è in modalità locale, quindi non effettua richieste di rete.
            [dns] Impossibile trovare l’indirizzo del server di download.
            [connect] deadloader non è riuscito a connettersi al server di download.
            [tls] Impossibile stabilire una connessione sicura con il server di download.
            [timeout] Il server ha impiegato troppo tempo a rispondere.
            [connection_lost] La connessione si è interrotta.
            [http_status] Il server ha risposto con un errore (HTTP { $status }).
           *[other] Il download è stato interrotto.
        }
    .todo =
        { $reason ->
            [local_mode] Attiva il catalogo nelle Impostazioni per scaricare i mod.
           *[other] Riprenderà da dove si era fermato.
        }
e7 = Il file scaricato è danneggiato.
    .why =
        { $reason ->
            [size] La sua dimensione non corrisponde a quella indicata da GameBanana.
           *[checksum] Il suo checksum non corrisponde a quello indicato da GameBanana.
        }
    .todo = Scaricalo di nuovo.
e8 = Impossibile estrarre l’archivio.
    .why =
        { $reason ->
            [not_an_archive] Il file non è un archivio ZIP, 7z o RAR: forse il download è incompleto.
            [damaged] L’archivio è danneggiato.
            [encrypted] L’archivio è protetto da password.
            [unsupported] Usa un metodo di compressione che deadloader non supporta.
            [not_a_vpk] { $entry } non è un file .vpk valido.
           *[other] È danneggiato o usa un formato non supportato.
        }
    .todo = Prova un altro file di questo mod o segnala il problema al suo autore.
e9 = L’archivio è troppo grande per essere estratto in sicurezza.
    .why =
        { $reason ->
            [too_large]
                { $measured ->
                    [yes] Una volta estratto occuperebbe { $unpacked } e il limite è { $limit }.
                   *[no] Una volta estratto supera { $limit }.
                }
            [too_many_files]
                { $found ->
                    [one] Contiene { $found } file di mod e il limite è { $limit }.
                   *[other] Contiene { $found } file di mod e il limite è { $limit }.
                }
            [ratio] { $entry } è compresso in modo sospetto: oltre { $limit } volte. È tipico di un archivio dannoso.
            [memory] Per estrarlo servono { $needed } di memoria e il limite è { $limit }.
           *[other] Supera uno dei limiti di sicurezza per gli archivi.
        }
    .todo =
        { $reason ->
            [memory] Prova un altro file di questo mod.
            [ratio] Non installarlo se non ti fidi della fonte; solo in quel caso aumenta il limite in Impostazioni → Avanzate.
           *[other] Se ti fidi, aumenta il limite in Impostazioni → Download.
        }
e10 = Nessun file di mod in questo archivio.
    .why =
        { $listing ->
            [files] Non contiene file .vpk, solo { $contents }.
           *[empty] L’archivio è vuoto.
        }
    .todo = Questo file non è un mod di Deadlock in un formato che deadloader può installare.
e11 = Impossibile estrarre archivi RAR su questo PC.
    .why = Lo strumento integrato di Windows non riesce a leggere questo RAR, e non sono stati trovati né 7-Zip né WinRAR.
    .todo = Installa 7-Zip (è gratuito) e riprova; il file scaricato viene conservato.
e12 = Spazio su disco insufficiente.
    .why =
        { $reason ->
            [space] Servono { $needed }, ma ne sono liberi solo { $available }.
           *[full] Il disco è pieno.
        }
    .todo = Libera spazio o elimina le vecchie versioni dei mod nella Libreria, poi riprova.
e13 = Impossibile applicare le modifiche mentre Deadlock è in esecuzione.
    .why = Windows blocca i file del gioco finché è in esecuzione.
    .todo = Chiudi Deadlock; deadloader applicherà le modifiche subito dopo.
e14 = Un file di mod nella libreria manca o è danneggiato.
    .why = È stato modificato o eliminato al di fuori di deadloader.
    .todo = Ripara il mod per scaricare di nuovo il file, oppure rimuovi il mod.
e15 = Impossibile aprire il link.
    .why = In Windows non è impostato un browser predefinito.
    .todo = Copia il link e aprilo tu.
e16 = Richiesta bloccata.
    .why =
        { $reason ->
            [not_https] Il server ha reindirizzato a un indirizzo senza HTTPS, che deadloader non segue mai.
            [bad_redirect] Il server ha inviato un reindirizzamento che deadloader non può seguire.
            [too_many_redirects] Il server ha effettuato troppi reindirizzamenti.
           *[not_listed] { $host } non è nell’elenco degli host consentiti, quindi deadloader non si è connesso.
        }
    .todo =
        { $reason ->
            [not_listed] Non devi fare nulla: deadloader comunica solo con GameBanana. Se ti fidi di questo host, aggiungilo in Impostazioni → Avanzate.
           *[not_https] Non devi fare nulla: deadloader segue solo reindirizzamenti sicuri. Riprova più tardi.
        }
e17 = Impossibile importare il profilo.
    .why =
        { $reason ->
            [newer] È stato creato con una versione più recente di deadloader.
            [unreadable] Il file non è leggibile: potrebbe essere stato spostato, eliminato o aperto da un altro programma.
           *[not_a_profile] Il file non è un profilo di deadloader.
        }
    .todo =
        { $reason ->
            [newer] Aggiorna deadloader, poi importa di nuovo il profilo.
            [unreadable] Controlla che il file ci sia ancora e che nessun altro programma lo tenga aperto, poi riprova.
           *[not_a_profile] Controlla di aver scelto il file giusto.
        }
e18 = Impossibile modificare i file nella cartella del gioco.
    .why =
        { $reason ->
            [access_denied] Windows ha negato l’accesso alla cartella del gioco.
            [in_use] Un file è in uso da parte di un altro programma, ad esempio un antivirus o un altro gestore di mod.
           *[other] Windows non è riuscito a modificare i file.
        }
    .todo = Chiudi gli altri programmi e riprova.
e19 = Impossibile caricare l’immagine.
    .why =
        { $reason ->
            [no_response] L’host non ha risposto.
            [not_an_image] Il file non è un’immagine.
            [too_large] L’immagine è troppo grande.
           *[other] L’host non ha inviato un’immagine utilizzabile.
        }
    .todo = Il segnaposto rimane; puoi riprovare.
e20 = Ci sono troppi mod attivati.
    .why =
        { $free ->
            [one] Il gioco può caricare ancora { $free } file di mod, e questo profilo ne richiede { $wanted }.
           *[other] Il gioco può caricare ancora { $free } file di mod, e questo profilo ne richiede { $wanted }.
        }
    .todo = Disattiva alcuni mod o scegli meno componenti aggiuntivi.
e21 = Esiste già un profilo chiamato “{ $name }”.
    .why = I nomi vengono confrontati senza distinguere maiuscole e minuscole, come i nomi dei file.
    .todo = Scegli un altro nome.
e22 = “{ $name }” non può essere usato come nome di profilo.
    .why = Windows non lo accetta come nome di file: ad esempio CON, un punto finale o uno qualsiasi dei caratteri \ / : * ? " < > |.
    .todo = Scegli un altro nome.
e23 = deadloader non riesce a leggere il proprio file { $file }.
    .why =
        { $reason ->
            [damaged_at] Il file è danneggiato (riga { $line }, colonna { $column }).
            [damaged] Il file è danneggiato.
            [newer] È stato creato con una versione più recente di deadloader.
           *[invalid] Il suo contenuto non è quello che deadloader si aspetta.
        }
    .todo =
        { $kind ->
            [settings] Per ora deadloader usa le impostazioni predefinite e lascia il file così com’è.
            [manifest] Ripristina manifest.json.bak o aggiorna deadloader.
           *[other] Ripristina il file da un backup o aggiorna deadloader.
        }
e24 = Il file non è più su GameBanana.
    .why = L’autore l’ha rimosso o sostituito (HTTP { $status }).
    .todo = Apri la pagina del mod per scegliere un altro file, oppure cerca aggiornamenti del mod.
e25 = GameBanana ha rifiutato la richiesta.
    .why = Il server ha risposto con il codice { $status }. Di solito è temporaneo.
    .todo = Riprova più tardi. Se continua a succedere, segnalalo sulla pagina GitHub di deadloader.
internal = Si è verificato un problema all’interno di deadloader.
    .why = È un errore di deadloader, non qualcosa che hai fatto tu.
    .todo = Riprova. Se succede di nuovo, segnalalo insieme alla voce del registro che trovi in Dettagli.

## Buttons named in the texts above

action-undo = Annulla
action-review = Rivedi
action-retry = Riprova
action-retry-now = Riprova ora
action-browse = Sfoglia…
action-verify-in-steam = Verifica in Steam
action-show-differences = Mostra differenze
action-details = Dettagli
action-repair = Ripara
action-copy-link = Copia link
action-open-folder = Apri cartella
action-open-release-page = Apri la pagina della versione
action-enable-catalog = Attiva catalogo
action-learn-more = Ulteriori informazioni
action-view-log = Visualizza registro

## Shared pieces

log-entry = Voce del registro { $id }
list-more = altri { $count }

## Sizes, labelled like Windows Explorer labels them

size-bytes =
    { $count ->
        [one] { $count } byte
       *[other] { $count } byte
    }
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
