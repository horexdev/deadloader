# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Ouvrir ce lien dans votre navigateur ?
catalog-link-body = Il quitte deadloader et s’ouvre dans votre navigateur par défaut.
catalog-link-always = Toujours ouvrir les liens immédiatement
catalog-link-cancel = Annuler
catalog-link-open = Ouvrir

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Masqués
catalog-nsfw-blurred = Floutés
catalog-nsfw-shown = Affichés
catalog-age-title = Afficher les mods au contenu sensible ?
catalog-age-body = Certains mods sur GameBanana sont classés pour nudité, thèmes sexuels ou contenus similaires. Pour les voir, confirmez que vous avez 18 ans ou plus, ou l’âge de la majorité là où vous vivez.
catalog-age-choice = Votre choix : { $mode }.
catalog-age-once = deadloader ne le demande qu’une fois. Vous pouvez revenir à « Masqués » à tout moment.
catalog-age-confirm = Je suis majeur(e)
catalog-age-cancel = Annuler

## S2: the catalog grid
catalog-title = Catalogue
catalog-section = Section
catalog-section-mods = Mods
catalog-section-sounds = Sons
catalog-search = Rechercher des mods
catalog-search-sounds = Rechercher des sons
catalog-search-offline = La recherche nécessite une connexion
catalog-search-off = Recherche indisponible — le catalogue est illisible
catalog-search-clear = Effacer la recherche
catalog-results =
    { $count ->
        [one] { $count } résultat
       *[other] { $count } résultats
    }
catalog-sort = Tri
catalog-sort-newest = Plus récents
catalog-sort-latest_updated = Mis à jour récemment
catalog-sort-most_downloaded = Plus téléchargés
catalog-sort-most_liked = Plus aimés
catalog-sort-most_viewed = Plus vus
catalog-sort-most_commented = Plus commentés
catalog-sort-oldest = Plus anciens
catalog-sort-best = Pertinence
catalog-sort-best-title = Pendant une recherche, le tri se fait par pertinence
catalog-category = Catégorie
catalog-category-all = Toutes les catégories
catalog-hero = Héros
catalog-hero-all = Tous les héros
catalog-hero-filter = Trouver un héros
catalog-hero-none = Aucun héros ne correspond
catalog-filters = Filtres
catalog-filter-nsfw = Mods avec classification de contenu (NSFW)
catalog-filter-favourites = Favoris uniquement
catalog-filter-favourites-short = favoris uniquement
catalog-filter-installed = Masquer les installés
catalog-filter-installed-short = installés masqués
catalog-nsfw-caption-hidden = NSFW masqués
catalog-nsfw-caption-blurred = NSFW floutés · cliquez sur une carte pour l’afficher
catalog-nsfw-caption-shown = NSFW affichés
catalog-loading = Chargement des mods depuis GameBanana…
catalog-loading-sounds = Chargement des sons depuis GameBanana…
catalog-loading-more = Chargement de la suite…
catalog-more-failed = La page suivante n’a pas pu être chargée.
catalog-grid = Mods
catalog-grid-sounds = Sons
catalog-likes =
    { $count ->
        [one] { $count } j’aime
       *[other] { $count } j’aime
    }
catalog-favourite-add = Ajouter aux favoris
catalog-favourite-remove = Retirer des favoris
catalog-sensitive = Contenu sensible
catalog-click-show = Cliquez pour afficher
catalog-badge-installed = Installé
catalog-badge-update = Mise à jour
catalog-card-installing = Installation · { $percent } %
catalog-empty = Aucun mod ici pour l’instant
catalog-empty-search = Aucun mod ne correspond à « { $text } »
catalog-empty-favourites = Aucun favori pour l’instant
catalog-empty-filters = Filtres actifs : { $filters }. Retirez-les pour chercher dans tous les mods, ou essayez un mot plus court.
catalog-empty-shorter = Essayez un mot plus court ou différent.
catalog-clear-filters = Effacer les filtres
catalog-rate-retry = deadloader réessaie automatiquement dans { $seconds } s — les mods ci-dessous restent utilisables.
catalog-retry-now = Réessayer maintenant
catalog-try-again = Réessayer
catalog-check-update = Chercher une mise à jour de deadloader
catalog-details = Détails · entrée du journal
catalog-copy-id = Copier l’identifiant de l’entrée
catalog-open-activity = Ouvrir dans Activité
catalog-offline-empty-title = Hors ligne — rien d’enregistré à afficher
catalog-offline-empty-text = deadloader ne peut pas joindre GameBanana, et cette partie du catalogue n’a pas été enregistrée auparavant.
catalog-error-title = Le catalogue ne peut pas être affiché pour le moment.
catalog-local-title = Le catalogue est désactivé (mode local)
catalog-local-why = En mode local, deadloader n’effectue aucune requête réseau. Votre bibliothèque, vos profils et Jouer fonctionnent normalement, et vous pouvez ajouter des mods depuis le disque.
catalog-local-do = Activez le catalogue pour parcourir et installer des mods de GameBanana. deadloader ne communiquera alors qu’avec GameBanana.
catalog-local-enable = Activer le catalogue
catalog-local-library = Aller à la bibliothèque

## S3: the mod page
catalog-breadcrumb = Fil d’Ariane
catalog-back = Retour au catalogue
catalog-page-loading = Chargement de la page du mod depuis GameBanana…
catalog-page-offline = Hors ligne — copie enregistrée le { $date }.
catalog-page-offline-why = Installer de nouveaux fichiers nécessite une connexion ; passer d’une variante téléchargée à une autre fonctionne toujours.
catalog-page-offline-empty-title = Hors ligne — cette page n’a pas été enregistrée
catalog-page-offline-empty-text = deadloader ne peut pas joindre GameBanana, et cette page n’a jamais été ouverte.
catalog-page-local-title = Cette page de mod n’a pas été enregistrée
catalog-page-local-text = En mode local, deadloader n’affiche que les pages de mods enregistrées auparavant.
catalog-removed-title = Ce mod a été retiré de GameBanana
catalog-removed-why = Il a été retenu ou supprimé : deadloader ne peut ni afficher sa page ni télécharger ses fichiers.
catalog-removed-keep = Votre copie installée ({ $copy }) reste dans la bibliothèque et continue de fonctionner — elle ne recevra simplement plus de mises à jour.
catalog-show-in-library = Afficher dans la bibliothèque
catalog-installed = Installé
catalog-by = par { $author }
catalog-version = Version { $version }
catalog-views =
    { $count ->
        [one] { $count } vue
       *[other] { $count } vues
    }
catalog-downloads =
    { $count ->
        [one] { $count } téléchargement
       *[other] { $count } téléchargements
    }
catalog-added = Ajouté { $when }
catalog-updated = Mis à jour { $when }
catalog-favourite = Favori
catalog-open-gamebanana = Ouvrir sur GameBanana
catalog-gallery-alt = { $name }, image { $index } de la galerie
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Image précédente
catalog-gallery-next = Image suivante
catalog-gallery-original = Voir l’original
catalog-gallery-hidden = Les images des mods avec classification de contenu sont masquées.
catalog-audio-preview = Ce son a un extrait sur GameBanana.
catalog-description = Description
catalog-description-more = Afficher toute la description
catalog-description-less = Afficher moins
catalog-description-truncated = Seul le début de cette description est affiché.
catalog-opens-browser = s’ouvre dans votre navigateur
catalog-opens-here = S’ouvre dans deadloader
catalog-image-from = Image de
catalog-image-click = cliquez pour charger
catalog-image-failed = n’a pas pu être chargée
catalog-image-alt = Image de la description
catalog-youtube = Vidéo YouTube
catalog-spoiler = Spoiler — cliquez pour afficher
catalog-spoiler-hide = Masquer le spoiler
catalog-files = Fichiers
catalog-files-clean =
    { $count ->
        [one] Il a passé l’analyse antivirus de GameBanana
       *[other] Les { $count } ont passé l’analyse antivirus de GameBanana
    }
catalog-files-clean-dated =
    { $count ->
        [one] Il a passé l’analyse antivirus de GameBanana · envoyé le { $date }
       *[other] Les { $count } ont passé l’analyse antivirus de GameBanana · envoyés le { $date }
    }
catalog-files-av = Le résultat de l’antivirus de GameBanana figure sur chaque fichier.
catalog-variants-note = Ces fichiers modifient les mêmes fichiers du jeu : ce sont des variantes, une seule est active à la fois.
catalog-no-files = Ce mod n’a aucun fichier à télécharger.
catalog-older-files = Fichiers plus anciens ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } téléchargement
       *[other] { $count } téléchargements
    }
catalog-file-active = variante active
catalog-file-downloaded = téléchargé
catalog-file-install = Installer
catalog-file-install-label = Installer { $file } et en faire la variante active
catalog-file-install-title = Télécharge ce fichier et en fait la variante active
catalog-file-install-offline = L’installation nécessite une connexion
catalog-file-installing = Installation…
catalog-file-installed = Installé
catalog-file-switch = Passer à
catalog-file-switch-label = Passer à { $file }
catalog-file-switch-title = Déjà téléchargé — en faire la variante active
catalog-file-update = Mettre à jour
catalog-file-update-title = Une version plus récente remplace votre fichier installé — vérifiez-la dans la bibliothèque
catalog-av-clean = Sain
catalog-av-clean-title = Antivirus de GameBanana : sain
catalog-av-flagged = Signalé
catalog-av-flagged-title = Antivirus de GameBanana : { $verdict }
catalog-av-pending = Pas encore analysé
catalog-av-pending-title = GameBanana n’a pas fini d’analyser ce fichier
catalog-av-none = Pas d’analyse
catalog-av-none-title = GameBanana n’a envoyé aucun résultat d’antivirus pour ce fichier
catalog-credits = Crédits
catalog-no-ratings = Aucune classification de contenu
catalog-no-licence = Aucun texte de licence
catalog-no-sources = Aucune autre source
catalog-rating-sexual_themes = Thèmes sexuels
catalog-rating-sexual_content = Contenu sexuel
catalog-rating-skimpy_attire = Tenue légère
catalog-rating-lewd_angles_and_poses = Angles et poses suggestifs
catalog-rating-full_nudity = Nudité intégrale
catalog-licence = Licence
catalog-sources = Autres sources
catalog-videos = Vidéos

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Charger une image depuis un nouvel hôte ?
catalog-image-body-before = deadloader va télécharger cette image depuis
catalog-image-body-after = . Cet hôte n’est pas GameBanana — la requête apparaît dans Activité comme toutes les autres.
catalog-image-note = La taille des images est limitée. Vous pouvez retirer les hôtes autorisés dans Paramètres → Catalogue.
catalog-image-always = Toujours autoriser les images de cet hôte
catalog-image-cancel = Annuler
catalog-image-load = Charger l’image
