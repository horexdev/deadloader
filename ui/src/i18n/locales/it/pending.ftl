# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Chiudi
library-apply = Applica
library-apply-profile = Applica { $name }
library-discard-all = Scarta tutto
library-keep = Mantieni { $name }
library-keep-applied = Mantieni le modifiche applicate
library-undo-this = Annulla questa modifica
library-undo-all = Annulla tutto
library-close-to-apply = Chiudi Deadlock: le modifiche verranno applicate automaticamente
library-play-applies = Anche Gioca lo applica
library-applied-title = Modifiche applicate
library-nothing-pending = Niente in sospeso
library-all-in-game = Tutto il profilo { $name } è già nel gioco.
library-running-sub = Deadlock è in esecuzione, quindi Windows blocca i suoi file. Queste modifiche verranno applicate automaticamente appena chiudi il gioco.
library-waiting-sub = Queste modifiche non sono ancora nella cartella del gioco. Applicale ora oppure premi Gioca.
library-switch-sub = Un cambio di profilo modifica molti file insieme, quindi aspetta te. Applicalo qui oppure premi Gioca: deadloader lo applica prima dell’avvio.
library-applied-sub =
    { $count ->
        [one] Deadlock è stato chiuso alle { $time }, quindi la modifica fatta mentre era aperto è stata applicata. Puoi annullarla fino alla prossima modifica.
       *[other] Deadlock è stato chiuso alle { $time }, quindi le { $count } modifiche fatte mentre era aperto sono state applicate. Puoi annullarle fino alla prossima modifica.
    }
library-undoing-sub =
    { $count ->
        [one] Hai annullato la modifica applicata alle { $time } e Deadlock è di nuovo in esecuzione. Verrà ripristinata automaticamente quando chiudi il gioco.
       *[other] Hai annullato le { $count } modifiche applicate alle { $time } e Deadlock è di nuovo in esecuzione. Verranno ripristinate automaticamente quando chiudi il gioco.
    }
library-group-enable = Attiva · { $count }
library-group-disable = Disattiva · { $count }
library-group-order = Ordine · { $count }
library-group-variant = Variante · { $count }
library-group-version = Versione · { $count }
library-group-switch = Cambio di profilo · { $count }
library-group-undo =
    { $count ->
        [one] Annullamento della modifica applicata alle { $time }
       *[other] Annullamento delle { $count } modifiche applicate alle { $time }
    }
library-change-on = Disattivata → attiva
library-change-off = Attiva → disattivata
library-change-above = Spostata sopra { $name }
library-change-above-wins = Spostata sopra { $name }: ora vince il loro conflitto
library-change-below = Spostata sotto { $name }
library-switch-what-happens =
    { $on ->
        [one] Si attiva 1 mod
       *[other] Si attivano { $on } mod
    } · { $off ->
        [one] se ne disattiva 1
       *[other] se ne disattivano { $off }
    }
library-what-happens = Cosa succede nella cartella del gioco
library-folder-summary = { $added } file aggiunti · { $removed } rimossi · { $renumbered } rinumerati

## Foreign files
library-foreign-title = Trovato nella cartella delle mod del gioco
library-foreign-note = Non li ha messi deadloader e non li toccherà a meno che tu non lo scelga.
library-collapse = Comprimi
library-expand = Espandi
library-take = Prendi in gestione
library-leave = Lascia com’è
library-left = Lasciato com’è
library-foreign-open = Apri la cartella che contiene { $name }
library-foreign-vpk = VPK, { $count } file
library-foreign-vpk-hero = VPK, { $count } file: sembra una skin per { $hero }
library-foreign-unreadable = Un file di mod che deadloader non riesce a leggere
library-foreign-replaced = Un file che ha preso il posto di uno di deadloader
library-foreign-dmm = Creato da Deadlock Mod Manager: un file di impostazioni, non una mod
library-foreign-other = Un file, non una mod
library-foreign-folder = Cartella, { $count } file: non è una mod
library-foreign-meta = { $size } · aggiunto il { $date }
library-taken = { $name } ora è nella tua libreria.

## D11
library-d11-title = Prendere in gestione { $name }?
library-d11-lead = deadloader:
library-d11-step-copy = lo copierà nella libreria come mod locale ({ $size }),
library-d11-step-replace = sostituirà il file nella cartella del gioco con la propria copia collegata,
library-d11-step-enable = lo attiverà nel profilo { $profile } in fondo all’elenco.
library-d11-name = Nome nella libreria
library-d11-hint = La libreria conserva una copia esatta del file originale.
library-d11-suggested = { $file } (skin di { $hero })

## D10
library-d10-title = Un file del gioco è stato modificato da un altro programma
library-d10-what = Un altro programma ha modificato un file del gioco che decide quali cartelle carica Deadlock, spesso Deadlock Mod Manager. deadloader non sovrascrive le modifiche altrui, quindi le tue mod non verranno caricate finché il problema non è risolto.
library-d10-do = Lascia che Steam ripristini il file originale (Verifica i file del gioco). Poi deadloader aggiunge la propria riga e ti avvisa.
library-d10-diff = Mostra differenze
library-d10-expected = Atteso (originale + riga di deadloader)
library-d10-found = Trovato ora
library-d10-diff-note = Evidenziate: la riga che aggiunge deadloader e le righe aggiunte da un altro programma.
library-d10-help = Cos’è questo file?
library-d10-help-text = gameinfo.gi dice a Deadlock quali cartelle caricare. I gestori di mod aggiungono una riga perché il gioco carichi le mod dalla cartella addons. Gli aggiornamenti del gioco la ripristinano; deadloader lo corregge automaticamente, ma solo se il file è quello originale.
library-d10-wait-title = Steam sta verificando i file di Deadlock.
library-d10-wait-text = Quando Steam ha finito, torna su deadloader: controlla di nuovo il file automaticamente. Puoi anche controllare ora.
library-d10-fixed = gameinfo.gi è configurato: le mod vengono di nuovo caricate.
library-d10-fixed-text = Steam ha ripristinato il file originale e deadloader ha aggiunto la propria riga. Dettagli in Attività → File.
library-open-file = Apri file
library-not-now = Non ora
library-verify-steam = Verifica i file del gioco in Steam
library-check-again = Controlla di nuovo
library-show-differences = Mostra differenze

## D5
library-update = Aggiorna
library-update-on-gb = Su GameBanana c’è una versione più recente.
library-later = Più tardi
library-d5-title = Aggiornare { $name }?
library-d5-installed = Installata
library-d5-new = Nuova su GameBanana
library-d5-downloaded = Scaricata
library-d5-arrived = Nella libreria, non ancora attiva
library-d5-new-meta = Aggiornata il { $date } · { $size } da scaricare
library-d5-download = { $size } da scaricare
library-d5-changed = La nuova versione ha un insieme di file diverso. deadloader li ha abbinati così:
library-d5-same = Questi file vengono sostituiti dalle loro nuove versioni:
library-d5-map-text = deadloader non è riuscito a capire per ogni file quale nuovo lo sostituisce. Scegli il nuovo file per ciascuno o mantieni quello vecchio.
library-d5-map-label = Abbinamento dei file
library-d5-col-old = File installato
library-d5-col-new = Nuovo file
library-d5-pick = Nuovo file per { $file }
library-d5-choose = Scegli…
library-d5-keep = Mantieni il vecchio file
library-d5-tag-replaces = Sostituisce
library-d5-tag-same = Stesso nome
library-d5-tag-new = Nuovo · nella pagina della mod
library-d5-profiles = { $new } diventa attiva in ogni profilo che usa { $old }. { $old } resta nella libreria, così puoi tornare indietro dai dettagli della mod.
library-d5-install = Installa aggiornamento
library-d5-confirm-map = Usa questi file
library-update-downloading = L’aggiornamento di { $name } è in download: verrà installato all’arrivo.
library-updated = { $name } aggiornata.
library-updated-text = Attiva in ogni profilo che usava la versione precedente. La versione precedente resta: puoi tornarci dai dettagli della mod.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name } attivata
library-n12-off = { $name } disattivata
library-n12-above = { $name } spostata sopra { $other }
library-n12-below = { $name } spostata sotto { $other }
library-n12-switched = { $name } passata a { $to }
library-n12-files = { $name }: file cambiati

## S9: the file list of a pending apply
library-show-file-list = Mostra l’elenco dei file
library-hide-file-list = Nascondi l’elenco dei file
library-file-list = File nella cartella del gioco
library-files-added = Aggiunti · { $count }
library-files-removed = Rimossi · { $count }
library-files-renumbered = Rinumerati · { $count }
