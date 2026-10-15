# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Activité
activity-sub =
    { $days ->
        [one] Tout ce que deadloader a fait sur ce PC : requêtes réseau, modifications de fichiers et événements de l’application. Les journaux restent sur ce PC pendant { $days } jour.
       *[other] Tout ce que deadloader a fait sur ce PC : requêtes réseau, modifications de fichiers et événements de l’application. Les journaux restent sur ce PC pendant { $days } jours.
    }
activity-open-logs = Ouvrir le dossier des journaux
activity-logs-opened = Dossier des journaux ouvert dans l’Explorateur de fichiers.
activity-summary = Aujourd’hui en bref
activity-net-today = Réseau aujourd’hui
activity-hosts-label = Hôtes contactés aujourd’hui — sélectionnez-en un pour filtrer
activity-hosts-contacted =
    { $count ->
        [one] { $count } hôte contacté
       *[other] { $count } hôtes contactés
    }
activity-host-title =
    { $count ->
        [one] { $count } requête · { $source } — sélectionnez pour filtrer
       *[other] { $count } requêtes · { $source } — sélectionnez pour filtrer
    }
activity-net-none-local = aucune — mode local
activity-net-none-yet = aucune pour l’instant
activity-nothing-else = Rien d’autre.
activity-blocked-lead =
    { $count ->
        [one] { $count } requête vers
       *[other] { $count } requêtes vers
    }
activity-blocked-tail =
    { $count ->
        [one] a été bloquée avant toute connexion.
       *[other] ont été bloquées avant toute connexion.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } requête bloquée, jamais contactée.
       *[other] { $count } requêtes bloquées, jamais contactées.
    }
activity-show-entry = Afficher l’entrée
activity-game-today = Dossier du jeu aujourd’hui
activity-show-in-files = Afficher dans Fichiers
activity-game-none = aucune modification
activity-no-game-changes = deadloader n’a encore rien modifié dans le dossier du jeu.
activity-game-changes =
    { $count ->
        [one] { $count } modification
       *[other] { $count } modifications
    }
activity-game-batches =
    { $count ->
        [one] en { $count } fois
       *[other] en { $count } fois
    }
activity-game-failed =
    { $count ->
        [one] { $count } en échec
       *[other] { $count } en échec
    }
activity-game-hardlink = { $count } liés
activity-game-copy = { $count } copiés
activity-game-delete = { $count } supprimés
activity-game-rename = { $count } renommés
activity-game-write = { $count } écrits
activity-game-backup = { $count } sauvegardés
activity-game-restore = { $count } restaurés
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi mis à jour une fois
       *[other] gameinfo.gi mis à jour { $count } fois
    }
activity-no-requests-today = Aucune requête aujourd’hui.
activity-no-requests-yet = Aucune requête pour l’instant.
activity-local-short = Le catalogue est désactivé, donc deadloader ne se connecte à rien.
activity-never-contacted = deadloader n’a contacté aucun hôte depuis son installation.
activity-tab-network = Réseau
activity-tab-files = Fichiers
activity-tab-app = Application
activity-tab-off = désactivé
activity-tab-errors =
    { $count ->
        [one] · { $count } erreur
       *[other] · { $count } erreurs
    }
activity-search = Rechercher dans le journal
activity-search-clear = Effacer la recherche
activity-matches =
    { $count ->
        [one] { $count } résultat
       *[other] { $count } résultats
    }
activity-matches-more = { $count }+ résultats
activity-f-time = Période
activity-f-purpose = Objet
activity-f-host = Hôte
activity-f-op = Opération
activity-f-level = Niveau
activity-f-result = Résultat
activity-all = Tous
activity-ok = OK
activity-errors = Erreurs
activity-range-hour = Dernière heure
activity-range-today = Aujourd’hui
activity-range-7d = 7 derniers jours
activity-range-30d = 30 derniers jours
activity-clear-filters = Effacer les filtres
activity-clear-all = Effacer la recherche et les filtres
activity-follow = Suivre en direct
activity-follow-tip = Afficher les nouvelles entrées en haut dès qu’elles arrivent
activity-follow-on = Les nouvelles entrées apparaissent en haut
activity-follow-paused = En pause depuis { $time }
activity-show-new =
    { $count ->
        [one] Afficher { $count } nouvelle
       *[other] Afficher { $count } nouvelles
    }
activity-net-aria = Requêtes réseau, les plus récentes d’abord
activity-files-aria = Opérations sur les fichiers, les plus récentes d’abord
activity-app-aria = Événements de l’application, les plus récents d’abord
activity-c-redirects = Redirections
activity-c-time = Heure
activity-c-purpose = Objet
activity-c-request = Requête
activity-c-status = Statut
activity-c-size = Taille
activity-c-duration = Durée
activity-c-started-by = Lancée par
activity-c-source = Source de l’hôte
activity-c-op = Opération
activity-c-from-to = De → vers
activity-c-result = Résultat
activity-c-change = Fait partie de
activity-c-level = Niveau
activity-c-message = Message
activity-to = vers
activity-loading-more = Chargement…
activity-p-catalog = Catalogue
activity-p-thumbnail = Miniature
activity-p-description-image = Image de description
activity-p-download = Téléchargement
activity-p-update-check = Vérification des mises à jour
activity-src-builtin = Intégré
activity-src-github = GitHub
activity-src-user = Ajouté par vous
activity-src-user-image = Hôte d’images
activity-src-blocked = Non autorisé
activity-src-long-builtin = Intégré — GameBanana, fait partie de deadloader
activity-src-long-github = GitHub — la vérification des mises à jour de deadloader, activée dans les paramètres
activity-src-long-user = Ajouté par vous — dans Paramètres → Avancé
activity-src-long-user-image = Hôte d’images autorisé — vous l’avez autorisé en chargeant une image de description
activity-src-long-blocked = Absent de la liste des hôtes autorisés
activity-by-auto = Auto
activity-by-user = Vous
activity-op-hardlink = Lien
activity-op-copy = Copie
activity-op-rename = Renommage
activity-op-delete = Suppression
activity-op-write = Écriture
activity-op-backup = Sauvegarde
activity-op-restore = Restauration
activity-lv-debug = Débogage
activity-lv-info = Info
activity-lv-warn = Avertissement
activity-lv-error = Erreur
activity-st-blocked = Bloquée
activity-st-blocked-long = Bloquée — jamais contactée
activity-st-failed = Échec
activity-redirects =
    { $count ->
        [one] { $count } redirection
       *[other] { $count } redirections
    }
activity-retries =
    { $count ->
        [one] { $count } nouvel essai
       *[other] { $count } nouveaux essais
    }
activity-show-redirects = Afficher les redirections
activity-hide-redirects = Masquer les redirections
activity-hop-request = Étape 1 · requête
activity-hop-redirect = Étape { $n } · redirection
activity-hop-retry = Étape { $n } · essai { $try }
activity-ms = { $ms } ms
activity-seconds = { $s } s
activity-no-match = Aucune entrée ne correspond
activity-no-match-text = Choisissez une autre période ou effacez les filtres et la recherche.
activity-empty-title = Rien pour l’instant
activity-empty-text = deadloader inscrit ici chaque requête réseau, chaque modification de fichier dans le dossier du jeu et chaque événement de l’application, au moment où ils se produisent. Rien ne s’est encore produit.
activity-local-title = Mode local — aucune requête n’est faite
activity-local-why = Le catalogue est désactivé, donc deadloader ne se connecte à rien — ni à GameBanana, ni ailleurs. Les modifications de fichiers et les événements de l’application restent journalisés dans les autres onglets.
activity-local-do = Activez le catalogue dans les paramètres pour parcourir GameBanana. Chaque requête apparaîtra alors ici.
activity-enable-catalog = Activer le catalogue
activity-details = Détails de l’entrée du journal
activity-close-details = Fermer les détails
activity-close-esc = Fermer (Échap)
activity-why = Pourquoi :
activity-todo = Que faire :
activity-open-advanced = Ouvrir Paramètres → Avancé
activity-full-record = Entrée complète
activity-hops-title = Redirections · { $count }
activity-user-hidden = Votre nom d’utilisateur Windows est masqué dans les journaux :
activity-settings-logs = Paramètres → Journaux
activity-copy = Copier l’entrée
activity-copied = Copié
activity-kind-network = Requête réseau
activity-kind-files = Opération sur un fichier
activity-kind-app = Événement de l’application
activity-d-time = Heure
activity-d-purpose = Objet
activity-d-started-by = Lancée par
activity-d-request = Requête
activity-d-source = Source de l’hôte
activity-d-status = Statut
activity-d-size = Taille
activity-d-duration = Durée
activity-d-range = Plage
activity-d-error = Erreur
activity-d-id = ID de l’entrée
activity-d-request-id = ID de la requête
activity-d-operation = Opération
activity-d-from = De
activity-d-to = Vers
activity-d-file = Fichier
activity-d-result = Résultat
activity-d-change = Fait partie de
activity-d-sha256 = SHA-256
activity-d-level = Niveau
activity-d-message = Message
activity-d-fields = Détails
activity-status-after =
    { $count ->
        [one] { $status } — après { $count } redirection
       *[other] { $status } — après { $count } redirections
    }
