# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Aprire questo link nel browser?
catalog-link-body = Esce da deadloader e si apre nel tuo browser predefinito.
catalog-link-always = Apri sempre i link subito
catalog-link-cancel = Annulla
catalog-link-open = Apri

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Nascosti
catalog-nsfw-blurred = Sfocati
catalog-nsfw-shown = Visibili
catalog-age-title = Mostrare le mod con contenuti sensibili?
catalog-age-body = Alcune mod su GameBanana sono classificate per nudità, temi sessuali o simili. Per vederle, conferma di avere almeno 18 anni o la maggiore età dove vivi.
catalog-age-choice = Hai scelto: { $mode }.
catalog-age-once = deadloader lo chiede una sola volta. Puoi tornare a «Nascosti» in qualsiasi momento.
catalog-age-confirm = Sono maggiorenne
catalog-age-cancel = Annulla

## S2: the catalog grid
catalog-title = Catalogo
catalog-section = Sezione
catalog-section-mods = Mod
catalog-section-sounds = Suoni
catalog-search = Cerca mod
catalog-search-sounds = Cerca suoni
catalog-search-offline = La ricerca richiede una connessione
catalog-search-off = Ricerca non disponibile: il catalogo non è leggibile
catalog-search-clear = Cancella ricerca
catalog-results =
    { $count ->
        [one] { $count } risultato
       *[other] { $count } risultati
    }
catalog-sort = Ordina
catalog-sort-newest = Più recenti
catalog-sort-latest_updated = Aggiornate di recente
catalog-sort-most_downloaded = Più scaricate
catalog-sort-most_liked = Più apprezzate
catalog-sort-most_viewed = Più viste
catalog-sort-most_commented = Più commentate
catalog-sort-oldest = Meno recenti
catalog-sort-best = Più pertinenti
catalog-sort-best-title = Durante la ricerca l’ordine è per pertinenza
catalog-category = Categoria
catalog-category-all = Tutte le categorie
catalog-hero = Eroe
catalog-hero-all = Tutti gli eroi
catalog-hero-filter = Trova un eroe
catalog-hero-none = Nessun eroe corrisponde
catalog-filters = Filtri
catalog-filter-nsfw = Mod con classificazione dei contenuti (NSFW)
catalog-filter-favourites = Solo preferiti
catalog-filter-favourites-short = solo preferiti
catalog-filter-installed = Nascondi installate
catalog-filter-installed-short = installate nascoste
catalog-nsfw-caption-hidden = NSFW nascosti
catalog-nsfw-caption-blurred = NSFW sfocati · fai clic su una scheda per vederla
catalog-nsfw-caption-shown = NSFW visibili
catalog-loading = Caricamento delle mod da GameBanana…
catalog-loading-sounds = Caricamento dei suoni da GameBanana…
catalog-loading-more = Caricamento di altre…
catalog-more-failed = Non è stato possibile caricare la pagina successiva.
catalog-grid = Mod
catalog-grid-sounds = Suoni
catalog-likes =
    { $count ->
        [one] { $count } mi piace
       *[other] { $count } mi piace
    }
catalog-favourite-add = Aggiungi ai preferiti
catalog-favourite-remove = Rimuovi dai preferiti
catalog-sensitive = Contenuto sensibile
catalog-click-show = Fai clic per vedere
catalog-badge-installed = Installata
catalog-badge-update = Aggiornamento
catalog-card-installing = Installazione · { $percent } %
catalog-empty = Ancora nessuna mod qui
catalog-empty-search = Nessuna mod corrisponde a «{ $text }»
catalog-empty-favourites = Ancora nessun preferito
catalog-empty-filters = Filtri attivi: { $filters }. Toglili per cercare in tutte le mod, o prova una parola più corta.
catalog-empty-shorter = Prova una parola più corta o diversa.
catalog-clear-filters = Togli i filtri
catalog-rate-retry = deadloader riprova da solo tra { $seconds } s — le mod qui sotto restano utilizzabili.
catalog-retry-now = Riprova ora
catalog-try-again = Riprova
catalog-check-update = Cerca un aggiornamento di deadloader
catalog-details = Dettagli · voce del registro
catalog-copy-id = Copia l’ID della voce
catalog-open-activity = Apri in Attività
catalog-offline-empty-title = Offline — niente di salvato da mostrare
catalog-offline-empty-text = deadloader non raggiunge GameBanana e questa parte del catalogo non è stata salvata prima.
catalog-error-title = Il catalogo non può essere mostrato ora.
catalog-local-title = Il catalogo è disattivato (modalità locale)
catalog-local-why = In modalità locale deadloader non fa alcuna richiesta di rete. Libreria, profili e Gioca funzionano come sempre, e puoi aggiungere mod dal disco.
catalog-local-do = Attiva il catalogo per sfogliare e installare mod da GameBanana. deadloader parlerà solo con GameBanana.
catalog-local-enable = Attiva il catalogo
catalog-local-library = Vai alla libreria

## S3: the mod page
catalog-breadcrumb = Percorso
catalog-back = Torna al catalogo
catalog-page-loading = Caricamento della pagina della mod da GameBanana…
catalog-page-offline = Offline — copia salvata il { $date }.
catalog-page-offline-why = Installare nuovi file richiede una connessione; passare tra le varianti scaricate funziona ancora.
catalog-page-offline-empty-title = Offline — questa pagina non è stata salvata
catalog-page-offline-empty-text = deadloader non raggiunge GameBanana e questa pagina non è mai stata aperta.
catalog-page-local-title = Questa pagina della mod non è stata salvata
catalog-page-local-text = In modalità locale deadloader mostra solo le pagine delle mod salvate prima.
catalog-removed-title = Questa mod è stata rimossa da GameBanana
catalog-removed-why = È stata trattenuta o eliminata, quindi deadloader non può mostrarne la pagina né scaricarne i file.
catalog-removed-keep = La tua copia installata ({ $copy }) resta nella libreria e continua a funzionare — semplicemente non riceverà aggiornamenti.
catalog-show-in-library = Mostra nella libreria
catalog-installed = Installata
catalog-by = di { $author }
catalog-version = Versione { $version }
catalog-views =
    { $count ->
        [one] { $count } visualizzazione
       *[other] { $count } visualizzazioni
    }
catalog-downloads =
    { $count ->
        [one] { $count } download
       *[other] { $count } download
    }
catalog-added = Aggiunta { $when }
catalog-updated = Aggiornata { $when }
catalog-favourite = Preferita
catalog-open-gamebanana = Apri su GameBanana
catalog-gallery-alt = { $name }, immagine { $index } della galleria
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Immagine precedente
catalog-gallery-next = Immagine successiva
catalog-gallery-original = Vedi originale
catalog-gallery-hidden = Le immagini delle mod con classificazione dei contenuti sono nascoste.
catalog-audio-preview = Questo suono ha un’anteprima su GameBanana.
catalog-description = Descrizione
catalog-description-more = Mostra tutta la descrizione
catalog-description-less = Mostra meno
catalog-description-truncated = Viene mostrato solo l’inizio di questa descrizione.
catalog-opens-browser = si apre nel browser
catalog-opens-here = Si apre in deadloader
catalog-image-from = Immagine da
catalog-image-click = fai clic per caricarla
catalog-image-failed = non è stato possibile caricarla
catalog-image-alt = Immagine dalla descrizione
catalog-youtube = Video di YouTube
catalog-spoiler = Spoiler — fai clic per vedere
catalog-spoiler-hide = Nascondi spoiler
catalog-files = File
catalog-files-clean =
    { $count ->
        [one] Ha superato il controllo antivirus di GameBanana
       *[other] Tutti i { $count } hanno superato il controllo antivirus di GameBanana
    }
catalog-files-clean-dated =
    { $count ->
        [one] Ha superato il controllo antivirus di GameBanana · caricato il { $date }
       *[other] Tutti i { $count } hanno superato il controllo antivirus di GameBanana · caricati il { $date }
    }
catalog-files-av = Il risultato dell’antivirus di GameBanana è indicato su ogni file.
catalog-variants-note = Questi file modificano gli stessi file del gioco, quindi sono varianti: ne è attiva una sola.
catalog-no-files = Questa mod non ha file da scaricare.
catalog-older-files = File precedenti ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } download
       *[other] { $count } download
    }
catalog-file-active = variante attiva
catalog-file-downloaded = scaricato
catalog-file-install = Installa
catalog-file-install-label = Installa { $file } e rendilo la variante attiva
catalog-file-install-title = Scarica questo file e lo rende la variante attiva
catalog-file-install-offline = L’installazione richiede una connessione
catalog-file-installing = Installazione…
catalog-file-installed = Installata
catalog-file-switch = Passa a
catalog-file-switch-label = Passa a { $file }
catalog-file-switch-title = Già scaricato — rendilo la variante attiva
catalog-file-update = Aggiorna
catalog-file-update-title = Una versione più recente sostituisce il file installato — controllala nella libreria
catalog-av-clean = Pulito
catalog-av-clean-title = Antivirus di GameBanana: pulito
catalog-av-flagged = Segnalato
catalog-av-flagged-title = Antivirus di GameBanana: { $verdict }
catalog-av-pending = Non ancora controllato
catalog-av-pending-title = GameBanana non ha ancora finito di controllare questo file
catalog-av-none = Nessun controllo
catalog-av-none-title = GameBanana non ha inviato alcun risultato antivirus per questo file
catalog-credits = Riconoscimenti
catalog-no-ratings = Nessuna classificazione dei contenuti
catalog-no-licence = Nessun testo di licenza
catalog-no-sources = Nessuna fonte alternativa
catalog-rating-sexual_themes = Temi sessuali
catalog-rating-sexual_content = Contenuti sessuali
catalog-rating-skimpy_attire = Abbigliamento succinto
catalog-rating-lewd_angles_and_poses = Inquadrature e pose provocanti
catalog-rating-full_nudity = Nudità integrale
catalog-licence = Licenza
catalog-sources = Fonti alternative
catalog-videos = Video

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Caricare un’immagine da un nuovo host?
catalog-image-body-before = deadloader scaricherà questa immagine da
catalog-image-body-after = . Quell’host non è GameBanana — la richiesta compare in Attività come tutte le altre.
catalog-image-note = Le immagini hanno un limite di dimensione. Puoi togliere gli host consentiti in Impostazioni → Catalogo.
catalog-image-always = Consenti sempre le immagini da questo host
catalog-image-cancel = Annulla
catalog-image-load = Carica immagine
