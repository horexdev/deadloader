# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Paramètres
settings-nav = Sections des paramètres
settings-reset-nav = Réinitialiser tous les paramètres…
settings-default = Par défaut : { $value }
settings-restore = Rétablir la valeur par défaut
settings-on = Activé
settings-off = Désactivé
settings-cancel = Annuler
settings-remove = Retirer
settings-remove-item = Retirer { $item }
settings-add = Ajouter
settings-open-folder = Ouvrir le dossier
settings-open-folder-at = Ouvrir le dossier : { $path }
settings-copied = Copié dans le presse-papiers.
settings-folder = Dossier
settings-example = Exemple :

settings-section-general = Général
settings-section-general-lead = Langue, démarrage et zoom.
settings-section-game = Jeu
settings-section-game-lead = Où Deadlock est installé et ce que deadloader peut y modifier.
settings-section-catalog = Catalogue
settings-section-catalog-lead = Le catalogue GameBanana, le contenu sensible, les images et les liens.
settings-section-library = Bibliothèque
settings-section-library-lead = Mises à jour des mods, suppression des mods et anciennes versions.
settings-section-downloads = Téléchargements
settings-section-downloads-lead = File d’attente, archives et décompression.
settings-section-network = Réseau
settings-section-network-lead = Comment deadloader communique avec GameBanana.
settings-section-cache = Cache
settings-section-cache-lead = Miniatures et pages du catalogue enregistrées sur ce PC.
settings-section-logs = Journaux
settings-section-logs-lead = Ce que l’Activité conserve, et pendant combien de temps.
settings-section-advanced = Avancé
settings-section-advanced-lead = Fichiers de paramètres et domaines autorisés en plus.
settings-section-about = À propos de deadloader
settings-section-about-lead = Version, licence, vérification de ce téléchargement et emplacement de vos données.

# General
settings-general-group = Langue et démarrage
settings-language = Langue
settings-language-help = Langue du système : la langue d’affichage de Windows si deadloader la propose, sinon l’anglais.
settings-language-system = Langue du système ({ $language })
settings-startup-profile = Profil au démarrage
settings-startup-profile-help = Le profil sélectionné au démarrage de deadloader. Le sélectionner ne l’applique pas.
settings-startup-last = Dernier sélectionné ({ $name })
settings-startup-last-plain = Dernier sélectionné
settings-app-updates = Rechercher les mises à jour de deadloader au démarrage
settings-app-updates-help = deadloader demande une fois à GitHub (api.github.com) au démarrage si une version plus récente existe. La mise à jour reste toujours manuelle. Vous l’avez choisi au premier lancement.
settings-app-updates-local = Indisponible : le mode local n’effectue aucune requête réseau. L’option devient disponible quand vous activez le catalogue (Paramètres → Catalogue).
settings-zoom = Zoom
settings-zoom-keys-1 = Aussi avec Ctrl + molette de la souris,
settings-zoom-keys-2 = et
settings-zoom-keys-3 = pour réinitialiser.
settings-ctrl = Ctrl
settings-percent = { $value } %

# Game
settings-installs-group = Installations du jeu
settings-installs-lead = deadloader trouve seul les installations Steam de Deadlock. Chaque profil en utilise une — modifiez-la dans Profils.
settings-install-library = Bibliothèque Steam { $library }
settings-install-build = build { $build }
settings-install-used =
    { $count ->
        [0] utilisée par aucun profil
        [one] utilisée par 1 profil : { $names }
       *[other] utilisée par { $count } profils : { $names }
    }
settings-install-selected = { $name } (sélectionné)
settings-install-applied = { $name } (appliqué ici)
settings-install-steam = Trouvée via Steam
settings-install-manual = Ajoutée manuellement
settings-install-known = Des fichiers de deadloader s’y trouvent encore
settings-install-manual-meta = Ajoutée manuellement
settings-gi-ours = gameinfo.gi : OK — les lignes de deadloader sont présentes
settings-gi-wait = gameinfo.gi : configuré dès qu’un profil l’utilise
settings-gi-rolled = gameinfo.gi : d’origine — Appliquer ajoute de nouveau les lignes de deadloader
settings-gi-foreign = gameinfo.gi : modifié par un autre programme
settings-gi-missing = gameinfo.gi : introuvable — le dossier a peut-être été déplacé
settings-installs-none = Aucune installation trouvée pour l’instant.
settings-add-manual = Ajouter manuellement…
settings-add-manual-hint = Choisissez un dossier Deadlock qui n’a pas été trouvé.
settings-add-manual-title = Choisir le dossier de Deadlock
settings-install-added = Installation ajoutée.
settings-install-removed = Installation retirée de la liste.
settings-install-removed-text = Rien n’a été modifié dans ce dossier.
settings-gameinfo-group = Après les mises à jour du jeu
settings-autofix = Réparer gameinfo.gi automatiquement
settings-autofix-help = Une mise à jour de Deadlock réinitialise le fichier qui lui dit de charger les mods. deadloader le sauvegarde, remet ses lignes et vous prévient. Si désactivé, un bandeau vous demande d’abord.
settings-danger-group = Annuler et supprimer
settings-rollback = Restaurer le dossier du jeu du profil sélectionné
settings-rollback-help = Pour l’installation du profil sélectionné { $profile } ({ $drive }) : retire les { $files } fichiers de mods que deadloader y a placés et ses lignes dans gameinfo.gi. La bibliothèque et les profils restent ; Appliquer rétablit tout.
settings-rollback-help-others = Les autres dossiers du jeu ne sont pas touchés ; sélectionnez un profil qui les utilise pour les restaurer.
settings-rollback-nothing = Rien à restaurer sur { $drive } — deadloader n’y a aucun fichier pour l’instant.
settings-rollback-no-game = Deadlock est introuvable, il n’y a donc aucun dossier du jeu à restaurer.
settings-rollback-button = Restaurer…
settings-rolled-title = Dossier du jeu sur { $drive } restauré.
settings-rolled-text =
    { $foreign ->
        [0] Sur { $drive }, { $files } fichiers de mods et les lignes de deadloader dans gameinfo.gi ont été retirés. La bibliothèque et les profils sont conservés — Appliquer ou Jouer remet les mods.
       *[other] Sur { $drive }, { $files } fichiers de mods, les lignes de deadloader dans gameinfo.gi et { $foreign } fichiers étrangers ont été retirés. La bibliothèque et les profils sont conservés — Appliquer ou Jouer remet les mods.
    }
settings-wipe = Supprimer toutes les données de deadloader
settings-wipe-help = Supprime la bibliothèque, les profils, les journaux, le cache et les paramètres ({ $size }), puis ferme deadloader. deadloader.exe reste — supprimez-le vous-même ensuite.
settings-wipe-button = Supprimer toutes les données…

# Catalog
settings-catalog-group = Catalogue GameBanana
settings-catalog = Utiliser le catalogue
settings-catalog-help-on = Parcourez et installez des mods depuis GameBanana. deadloader ne communique qu’avec GameBanana, et chaque requête figure dans l’Activité. Désactivé signifie mode local.
settings-catalog-help-off = Désactivé — mode local : deadloader n’effectue aucune requête réseau. Vous pouvez toujours ajouter des mods depuis le disque.
settings-consent-title = Une étape d’abord.
settings-consent-text = deadloader vous montre les hôtes qu’il contactera et les conditions de GameBanana avant d’activer le catalogue.
settings-consent-go = Consulter et activer
settings-local-on = Le mode local est activé.
settings-local-on-text = Le catalogue est désactivé et deadloader n’effectue aucune requête réseau.
settings-ttl = Réutiliser les pages du catalogue chargées pendant
settings-ttl-help = Pendant ce délai, revenir sur une page ne redemande rien à GameBanana.
settings-minutes = { $count } min
settings-hours = { $count } h
settings-prefetch = Charger la page suivante à l’avance
settings-prefetch-help = Quand vous avez parcouru environ 70 % des résultats, les suivants sont chargés pour que la grille continue.
settings-nsfw-group = Contenu sensible
settings-nsfw = Mods avec classification du contenu (NSFW)
settings-nsfw-help = Masqués : pas affichés du tout. Floutés : une image floue avec la mention « Contenu sensible » ; cliquez pour l’afficher. Affichés : sans flou.
settings-nsfw-age-done = Âge confirmé le { $date }.
settings-nsfw-age-ask = Quitter « Masqués » vous demande une fois de confirmer votre âge.
settings-nsfw-hidden = Masqués
settings-nsfw-blurred = Floutés
settings-nsfw-shown = Affichés
settings-images-group = Images et liens
settings-description-images = Images des descriptions de mods
settings-description-images-help = Elles viennent d’hôtes autres que GameBanana, p. ex. i.imgur.com. Au clic : un espace réservé jusqu’à ce que vous cliquiez dessus.
settings-images-click = Au clic
settings-images-always = Toujours
settings-prechecked = Pré-cocher « Toujours autoriser les images de cet hôte »
settings-prechecked-help = Dans la boîte de dialogue affichée la première fois qu’une image de description vient d’un nouvel hôte.
settings-hosts = Hôtes d’images autorisés
settings-hosts-help = Les images de ces hôtes se chargent sans demander. Dans l’Activité, leurs requêtes sont marquées « hôte d’images autorisé ».
settings-hosts-built-in = Intégré
settings-hosts-none = Aucun hôte ajouté — chaque nouvel hôte demande d’abord.
settings-host-removed = { $host } retiré
settings-host-removed-text = Ses images demanderont de nouveau avant de se charger.
settings-undo = Annuler
settings-gif = Lire les GIF automatiquement
settings-gif-help = Désactivé : les GIF affichent leur première image avec un bouton de lecture.
settings-links = Liens externes
settings-links-help = Les liens s’ouvrent dans votre navigateur par défaut. « Demander d’abord » affiche l’adresse complète avant l’ouverture.
settings-links-ask = Demander d’abord
settings-links-open = Ouvrir directement

# Library
settings-updates-group = Mises à jour des mods
settings-check-updates = Rechercher les mises à jour des mods au démarrage
settings-check-updates-help = Compare vos mods de GameBanana avec le catalogue. Jamais en mode local.
settings-auto-download = Télécharger les mises à jour automatiquement
settings-auto-download-help = Seulement le téléchargement — deadloader demande toujours avant qu’un mod passe à la nouvelle version.
settings-disk-group = Suppression et anciennes versions
settings-deletion = Quand vous supprimez un mod
settings-deletion-help = Vous pouvez choisir autrement à chaque fois dans la boîte de dialogue de suppression.
settings-deletion-permanent = Définitivement
settings-deletion-bin = Vers la corbeille
settings-old-versions = Anciennes versions
settings-old-versions-help = Chaque version téléchargée est conservée pour que vous puissiez y revenir. Supprimez les anciennes versions d’un mod dans ses détails.
settings-old-keep-all = Tout conserver

# Downloads
settings-queue-group = File d’attente
settings-parallel = Téléchargements simultanés
settings-parallel-help = Combien de fichiers se téléchargent en même temps.
settings-speed = Limite de vitesse
settings-speed-help = Pour les téléchargements de mods uniquement — le catalogue et les miniatures ne sont pas limités.
settings-speed-none = Aucune limite
settings-speed-value = { $value } Mo/s
settings-keep-archives = Conserver les archives après l’installation
settings-keep-archives-help = Désactivé : le fichier .zip, .7z ou .rar est supprimé dès que ses fichiers .vpk sont dans la bibliothèque.
settings-unpack-group = Décompression
settings-limits = Limites de décompression
settings-limits-help = Protège contre les archives qui se décompressent en quelque chose d’énorme. Si une archive de confiance est refusée, augmentez la limite.
settings-limits-default = 2 Go · 20× · 256
settings-limit-size = Taille décompressée
settings-limit-ratio = Taux de compression
settings-limit-files = Fichiers de mods par archive
settings-unit-gb = Go
settings-unit-x = ×
settings-unit-files = fichiers
settings-unit-s = s
settings-number-error = { $field } : saisissez un nombre entier de { $min } à { $max }. La valeur précédente reste utilisée jusque-là.

# Network
settings-network-local = Le mode local est activé.
settings-network-local-text = deadloader n’effectue aucune requête réseau ; ces paramètres s’appliquent dès que le catalogue est activé.
settings-requests-group = Requêtes vers GameBanana
settings-api-parallel = Requêtes simultanées au catalogue
settings-api-parallel-help = Requêtes simultanées à l’API de GameBanana. Les images et les téléchargements sont comptés à part.
settings-timeouts = Délais d’attente
settings-timeouts-help = Connexion : abandonner si aucune connexion n’est établie dans ce délai. Réponse : abandonner quand le serveur n’envoie plus rien.
settings-timeouts-default = 10 s · 30 s
settings-timeout-connect = Connexion
settings-timeout-read = Réponse
settings-retries = Nouvelles tentatives
settings-retries-help = Combien de fois une requête échouée est répétée, avec des pauses croissantes, avant que deadloader affiche une erreur.
settings-connection-group = Connexion
settings-proxy = Proxy
settings-proxy-help = Aucun : connexion directe. Système : le proxy défini dans Windows. Manuel : saisissez-le ci-dessous.
settings-proxy-none = Aucun
settings-proxy-system = Système
settings-proxy-manual = Manuel
settings-proxy-host = Adresse
settings-proxy-port = Port
settings-proxy-hint = Proxy HTTP. Seules les requêtes de deadloader passent par lui.
settings-proxy-invalid = Saisissez un nom d’hôte ou une adresse sans nom d’utilisateur ni mot de passe, p. ex. proxy.example.com.

# Cache
settings-thumbs-group = Miniatures et pages du catalogue
settings-cache-limit = Limite du cache
settings-cache-limit-help = Quand le cache est plein, les miniatures que vous n’avez pas vues depuis le plus longtemps partent en premier.
settings-size-mb = { $value } Mo
settings-size-gb = { $value } Go
settings-cache-now = Taille actuelle
settings-cache-breakdown = { $thumbnails } miniatures · { $pages } pages du catalogue
settings-cache-empty = Vide — rien en cache pour l’instant.
settings-cache-used = { $used } sur { $limit }
settings-cache-clear = Vider le cache
settings-cache-cleared = Cache vidé — { $size } libérés. Les miniatures se rechargent quand vous en avez besoin.
settings-cache-cleared-partly = { $failed } fichiers étaient utilisés et restent jusqu’au prochain vidage.

# Logs
settings-keep-group = Conservation des journaux
settings-log-days = Conserver les journaux pendant
settings-log-days-help = Les entrées plus anciennes sont supprimées au démarrage de deadloader.
settings-days = { $count ->
        [one] { $count } jour
       *[other] { $count } jours
    }
settings-year = 1 an
settings-log-size = Taille maximale
settings-log-size-help = Au-delà de cette taille, les plus anciens fichiers de journal sont supprimés en premier.
settings-privacy-group = Confidentialité
settings-mask = Masquer votre nom d’utilisateur Windows
settings-mask-help = Les chemins des journaux affichent { $masked } au lieu de votre nom — plus sûr quand vous partagez un journal.
settings-logs-folder-group = Dossier des journaux
settings-logs-folder = Où se trouvent les journaux
settings-logs-now = { $size } · { $files } fichiers · le plus ancien du { $date }
settings-open-logs = Ouvrir le dossier des journaux

# Advanced
settings-advanced-warning = Pour les utilisateurs expérimentés.
settings-advanced-warning-text = Une erreur ici peut changer le comportement de deadloader. Il vérifie le fichier avant l’enregistrement et conserve une sauvegarde de la version précédente.
settings-files-group = Modifier les fichiers de paramètres
settings-file-settings = Paramètres
settings-file-manifest = Manifeste du dossier du jeu
settings-file-label = Fichier à modifier
settings-file-settings-lead = Tout ce qui figure sur ces pages, en un seul fichier. Les modifications apparaissent sur les pages après l’enregistrement.
settings-file-manifest-lead = La liste de ce que deadloader a placé dans le dossier du jeu. La restauration s’appuie dessus — une entrée erronée peut laisser des fichiers derrière.
settings-json-error = Ce n’est pas du JSON valide, rien n’a donc été enregistré.
settings-json-where = Ligne { $line }, colonne { $column } : il manque ou il y a en trop quelque chose ici ou juste avant — souvent une virgule ou un guillemet.
settings-json-fix = Corrigez cet endroit et enregistrez de nouveau — d’ici là, deadloader continue d’utiliser le fichier actuel.
settings-json-refused = Ces valeurs ne sont pas utilisables, rien n’a donc été enregistré.
settings-json-newer = Ce fichier vient d’un deadloader plus récent (format { $found } ; cette version lit jusqu’à { $supported }), il n’a donc pas été enregistré.
settings-json-invalid = deadloader ne peut pas utiliser ce fichier, il n’a donc pas été enregistré.
settings-json-details = Détails
settings-problem-unknown-field = { $path } : pas un paramètre de cette version.
settings-problem-invalid-value = { $path } : cette valeur n’est pas utilisable ici.
settings-problem-out-of-range = { $path } : { $found } est en dehors de { $min }–{ $max }.
settings-problem-invalid-entry = { $path } : « { $entry } » n’est pas valide ici.
settings-problem-duplicate-entry = { $path } : « { $entry } » figure deux fois.
settings-problem-needs-age = { $path } : afficher du contenu NSFW demande d’abord la confirmation de l’âge (Paramètres → Catalogue).
settings-problem-local-mode = { $path } : indisponible en mode local.
settings-json-idle = Vérifié lors de l’enregistrement.
settings-json-saved = Valide et enregistré. Version précédente conservée sous { $backup }.
settings-json-saved-plain = Valide et enregistré.
settings-json-save = Vérifier et enregistrer
settings-json-discard = Abandonner mes modifications
settings-json-backup = Avant l’enregistrement, le fichier précédent est conservé en sauvegarde ; les 10 dernières restent.
settings-domains-group = Domaines autorisés en plus
settings-domains-lead = deadloader ne communique qu’avec GameBanana. Si un téléchargement est redirigé vers un autre hôte et bloqué, vous pouvez autoriser cet hôte ici. Dans l’Activité, ses requêtes sont marquées « ajouté par l’utilisateur ».
settings-domains-built-in = Intégrés :
settings-domain-user = ajouté par l’utilisateur
settings-domains-none = Aucun domaine supplémentaire.
settings-domain-add = Ajouter un domaine
settings-domain-hint = Le nom d’hôte seulement, p. ex. files.example.com.
settings-domain-empty = Saisissez d’abord un nom d’hôte, p. ex. files.example.com.
settings-domain-not-host = Saisissez seulement le nom d’hôte — sans https:// ni chemin, p. ex. files.example.com.
settings-domain-invalid = « { $host } » n’est pas un nom d’hôte. Vérifiez l’orthographe, p. ex. files.example.com.
settings-domain-built-in = { $host } est intégré et toujours autorisé.
settings-domain-listed = { $host } figure déjà dans la liste.

# About
settings-version = version { $version }
settings-commit = Construit à partir du commit
settings-commit-local = Build local — pas construit par GitHub Actions
settings-copy-commit = Copier l’identifiant du commit
settings-release-page = Page de la version
settings-licence-group = Licence
settings-licence-help = deadloader est un logiciel libre : vous pouvez l’utiliser, l’étudier, le partager et le modifier selon la GPL-3. Il est fourni sans aucune garantie.
settings-read-licence = Lire la licence
settings-source = Code source
settings-verify-group = Comment vérifier ce téléchargement
settings-verify-lead = Deux vérifications indépendantes que ce deadloader.exe est bien celui publié sur GitHub.
settings-verify-hash = Comparer la somme de contrôle SHA-256
settings-verify-hash-help = Elle doit correspondre à celle de la page de la version. Une copie modifiée pourrait afficher ici une fausse valeur, calculez-la donc aussi vous-même :
settings-verify-hash-cmd = Dans PowerShell, dans le dossier de l’exe :
settings-verify-attest = Vérifier l’attestation de build
settings-verify-attest-help = Prouve que GitHub Actions a construit ce fichier à partir du code source publié. Nécessite GitHub CLI :
settings-copy-hash = Copier la somme de contrôle
settings-copy-command = Copier la commande
settings-unsigned = deadloader.exe n’est pas signé.
settings-unsigned-text = Windows SmartScreen peut afficher « Windows a protégé votre ordinateur » au premier lancement. Les deux vérifications ci-dessus permettent de confirmer que le fichier est authentique.
settings-webview-group = Moteur de la fenêtre
settings-webview-help = La fenêtre de deadloader est dessinée par WebView2, un composant de Windows. Windows le met à jour lui-même. deadloader désactive :
settings-webview-off-1 = Les requêtes DNS-over-HTTPS
settings-webview-off-2 = La connexion au compte Microsoft dans la fenêtre
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Les accès réseau en arrière-plan et les mises à jour de composants
settings-webview-off-5 = Les appels serveur du remplissage automatique de formulaires
settings-webview-off-6 = Les rapports d’erreurs à Microsoft (domain reliability)
settings-webview-no-route = Et la fenêtre n’a aucun accès à Internet.
settings-webview-unknown = version inconnue
settings-data-group = Vos données
settings-data-folder = Dossier des données
settings-data-folder-help = Tout ce que deadloader conserve se trouve dans ce seul dossier, à côté de l’exe.
settings-notice = Avis du premier lancement
settings-notice-help = Version { $version }, acceptée le { $date } en { $language }. Vos choix : catalogue { $catalog }, recherche de mises à jour { $updates }.
settings-notice-none = Pas encore accepté.
settings-notice-again = Le relire

# D7
settings-d7-title = Restaurer le dossier du jeu sur { $drive } ?
settings-d7-from =
    { $others ->
        [0] L’installation du profil sélectionné { $profile }. deadloader retire de
       *[other] L’installation du profil sélectionné { $profile } (utilisée aussi par { $others }). deadloader retire de
    }
settings-d7-files =
    { $count ->
        [one] 1 fichier de mod qu’il a ajouté dans
       *[other] { $count } fichiers de mods qu’il a ajoutés dans
    }
settings-d7-line = Ses lignes dans gameinfo.gi
settings-d7-line-why = — le fichier redevient tel que Steam l’a installé.
settings-d7-stays = Reste tel quel :
settings-d7-library = Votre bibliothèque — { $mods } mods, { $size }
settings-d7-profiles =
    { $count ->
        [one] Votre profil — { $names }
       *[other] Vos { $count } profils — { $names }
    }
settings-d7-other = L’installation sur { $path } ({ $profile }) — pas touchée. Pour la restaurer, sélectionnez un profil qui l’utilise et restaurez de nouveau.
settings-d7-other-plain = L’installation sur { $path } — pas touchée.
settings-d7-back = Appliquer ou Jouer remet tout en place.
settings-d7-foreign =
    { $count ->
        [one] Retirer aussi 1 fichier que deadloader n’a pas créé
       *[other] Retirer aussi { $count } fichiers que deadloader n’a pas créés
    }
settings-d7-foreign-why = d’un autre programme ou d’une installation manuelle. Ils sont supprimés définitivement ; deadloader ne peut pas les récupérer.
settings-d7-confirm = Restaurer
settings-d7-confirm-foreign =
    { $count ->
        [one] Restaurer et supprimer 1 fichier
       *[other] Restaurer et supprimer { $count } fichiers
    }

# D8
settings-d8-title = Supprimer toutes les données de deadloader ?
settings-d8-from = deadloader supprime tout le contenu de
settings-d8-from-2 = puis se ferme.
settings-d8-what = Quoi
settings-d8-contains = Contenu
settings-d8-size = Taille
settings-d8-total = Total
settings-d8-library = Bibliothèque
settings-d8-library-has = { $mods } mods, { $files } fichiers
settings-d8-profiles = Profils
settings-d8-logs = Journaux
settings-d8-logs-has = { $files } fichiers, depuis le { $date }
settings-d8-logs-none = aucun fichier de journal
settings-d8-cache = Cache
settings-d8-cache-has = miniatures et pages du catalogue
settings-d8-settings = Paramètres
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
        [one] dont 1 sauvegarde
       *[other] dont { $backups } sauvegardes
    }
settings-d8-other = Autres
settings-d8-other-has = téléchargements partiels, sauvegardes de gameinfo.gi, données du moteur de la fenêtre
settings-d8-exe = Non supprimé : deadloader.exe lui-même — supprimez-le vous-même si vous n’en avez plus besoin.
settings-d8-rollback = D’abord restaurer chaque dossier du jeu utilisé par deadloader (recommandé)
settings-d8-rollback-why = Retire les fichiers de mods de deadloader et ses lignes dans gameinfo.gi de chaque dossier du jeu avant la suppression. Les fichiers étrangers restent.
settings-d8-rollback-off = Non coché : les fichiers de mods de deadloader restent dans les dossiers du jeu et Deadlock continue de les charger — sans deadloader pour les retirer.
settings-d8-type-1 = Pour confirmer, saisissez
settings-d8-type-2 = { "" }
settings-d8-word = supprimer
settings-d8-hint = Le bouton de suppression se débloque quand le mot correspond.
settings-d8-hint-ok = Confirmé. C’est irréversible.
settings-d8-type-first = Saisissez d’abord « { $word } »
settings-d8-deleting = Suppression…
settings-d8-confirm = Supprimer et fermer
settings-d8-confirm-rollback = Restaurer, supprimer et fermer

# Reset
settings-reset-title = Réinitialiser tous les paramètres ?
settings-reset-body = Chaque paramètre revient à sa valeur par défaut, y compris les hôtes d’images autorisés et les domaines supplémentaires.
settings-reset-keeps = Votre bibliothèque, vos profils et vos journaux ne sont pas touchés, et le dossier du jeu reste tel quel.
settings-reset-backup = Une sauvegarde des paramètres actuels est conservée sous settings.json.bak-…
settings-reset-confirm = Réinitialiser les paramètres
settings-reset-done = Tous les paramètres ont été réinitialisés.
settings-reset-done-text = Sauvegarde : { $backup }
