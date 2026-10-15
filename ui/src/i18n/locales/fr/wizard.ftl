# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Progression de la configuration
wizard-caption-setup = Première configuration
wizard-caption-changed = Avis mis à jour
wizard-caption-catalog = Activer le catalogue
wizard-caption-review = Avis du premier lancement
wizard-language = Langue
wizard-step-welcome = Bienvenue
wizard-step-risks = Risques
wizard-step-network = Réseau
wizard-step-privacy = Confidentialité
wizard-step-updates = Mises à jour
wizard-step-setup = Configuration
wizard-step-of = Étape { $n } sur 6
wizard-changed-of =
    { $count ->
        [one] { $count } étape modifiée
       *[other] { $count } étapes modifiées
    }
wizard-meta-changed = Modifiée
wizard-meta-kept = Inchangée
wizard-back = Retour
wizard-next = Suivant
wizard-finish = Terminer la configuration
wizard-continue = Continuer
wizard-close = Fermer
wizard-cancel = Annuler
wizard-to-catalog = Le catalogue s’ouvre ensuite.
wizard-to-library = Votre bibliothèque s’ouvre ensuite (mode local).

# Step 1
wizard-s1-title = Bienvenue dans deadloader
wizard-s1-lead = deadloader installe des mods Deadlock depuis GameBanana ou depuis votre disque, et vous montre chaque modification qu’il effectue.
wizard-f1a = Il ajoute des fichiers de mods au dossier de mods du jeu et une ligne à
wizard-f1b = — un fichier du jeu qui indique à Deadlock de charger les mods.
wizard-f2 = Tout est réversible : la restauration retire du jeu tout ce que deadloader y a mis.
wizard-f3a = Toutes ses données restent dans le dossier
wizard-f3b = à côté de
wizard-f3c = .
wizard-f4 = deadloader n’est affilié ni à Valve ni à GameBanana.
wizard-f5 = Logiciel libre sous licence GPL-3, fourni sans aucune garantie.
wizard-licence = Licence (GPL-3)
wizard-source = Code source

# Step 2
wizard-s2-title = Les mods sont à vos risques
wizard-r1 = Valve n’a aucune règle officielle sur les mods pour Deadlock. Utiliser des mods est votre décision et votre risque.
wizard-r2 = Les mises à jour du jeu peuvent casser des mods. deadloader se reconfigure après une mise à jour, mais un mod peut ne plus marcher tant que son auteur ne l’a pas mis à jour.
wizard-r3 = Les mods sont créés par d’autres personnes, sous leurs propres licences. deadloader ne peut pas garantir leur contenu ; pour chaque fichier, il affiche le résultat de l’antivirus de GameBanana.
wizard-agree = Je comprends que deadloader modifie des fichiers du jeu et que j’utilise les mods à mes risques.
wizard-agree-note = Cela confirme les étapes 1 et 2. C’est nécessaire pour continuer.
wizard-agree-error = Cochez cette case pour continuer.
wizard-agree-reason = Cochez d’abord la case.

# Step 3
wizard-s3-title = Réseau et GameBanana
wizard-s3-lead = Le catalogue parle directement à l’API non officielle de GameBanana — et à personne d’autre. Voici les seuls hôtes qu’il contacte :
wizard-hosts = Hôtes
wizard-host-api = catalogue et pages des mods
wizard-host-images = miniatures et galerie
wizard-host-files = téléchargements
wizard-host-cache = téléchargements (files.gamebanana.com y redirige)
wizard-host-new = Nouveau
wizard-s3-images = Les images d’autres hôtes dans les descriptions de mods ne se chargent qu’après votre autorisation, hôte par hôte. Chaque requête figure dans
wizard-s3-images-end = .
wizard-activity = l’Activité
wizard-s3-tos = Utiliser le catalogue, c’est utiliser GameBanana : leurs conditions d’utilisation s’appliquent, et GameBanana exige que ses utilisateurs aient au moins 13 ans.
wizard-tos = Conditions d’utilisation de GameBanana
wizard-choice = Comment voulez-vous commencer ?
wizard-use-catalog = Utiliser le catalogue
wizard-use-catalog-text = Parcourez et installez des mods depuis GameBanana. deadloader se connecte uniquement aux hôtes ci-dessus.
wizard-local = Pas maintenant — mode local
wizard-local-text = Aucune requête réseau ; mods depuis le disque uniquement. Vous pourrez activer le catalogue plus tard dans les paramètres.
wizard-choice-error = Choisissez l’une des deux options.
wizard-choice-reason = Choisissez d’abord une option.
wizard-last-time = Votre choix précédent
wizard-changed-title = Ce qui a changé ({ $date })
wizard-changed-foot = Vous avez confirmé les autres étapes le { $date } ; elles n’ont pas changé.

# Step 4
wizard-s4-title = Votre vie privée
wizard-p1 = Ni compte ni télémétrie. deadloader n’envoie rien nulle part, à part les requêtes GameBanana de l’étape précédente.
wizard-p1-local = Ni compte ni télémétrie. Vous avez choisi le mode local : deadloader n’envoie absolument rien.
wizard-p2a = Les journaux restent sur ce PC, dans le dossier
wizard-p2b = — et masquent votre nom d’utilisateur Windows :
wizard-p3 = La fenêtre de deadloader est dessinée par Microsoft WebView2, un composant de Windows. deadloader désactive les fonctions en ligne de WebView2 ; Windows met WebView2 à jour indépendamment de deadloader.
wizard-p4 = Vous pouvez toujours voir ce que deadloader a fait dans
wizard-p4-end = .

# Step 5
wizard-s5-title = Mises à jour de deadloader
wizard-u1 = deadloader peut vérifier sur GitHub, au démarrage, s’il existe une version plus récente de lui-même.
wizard-updates-label = Rechercher les nouvelles versions de deadloader sur GitHub au démarrage
wizard-updates-on = Activé
wizard-updates-off = Désactivé (par défaut)
wizard-updates-na = Indisponible en mode local
wizard-u2a = Si activé, deadloader envoie une requête à
wizard-u2b = à chaque démarrage et vous dit si une nouvelle version existe.
wizard-u3 = La mise à jour est toujours manuelle : vous téléchargez vous-même la nouvelle version sur GitHub. deadloader ne se remplace jamais lui-même.
wizard-u4 = Indisponible : le mode local n’envoie aucune requête réseau. L’option devient disponible quand vous activez le catalogue dans les paramètres.
wizard-u5 = Vous pourrez modifier cela plus tard dans Paramètres → Général.

# Step 6
wizard-s6-title = Configuration
wizard-s6-lead = deadloader a cherché le jeu et vérifié quelques points. Rien n’a encore été modifié, et vous pourrez tout changer plus tard dans les paramètres.
wizard-checking = Vérification…
wizard-topic-game = Jeu
wizard-topic-exe = Emplacement de deadloader
wizard-topic-disk = Espace disque
wizard-topic-foreign = Dossier de mods du jeu
wizard-topic-gameinfo = Fichier du jeu pour les mods
wizard-game-found = Deadlock trouvé
wizard-game-chosen = Dossier choisi
wizard-change = Modifier…
wizard-game-missing = Deadlock introuvable
wizard-game-missing-text = Steam ou le jeu n’est pas installé, ou le dossier du jeu a été déplacé. Choisissez vous-même le dossier du jeu — maintenant ou plus tard dans Paramètres → Jeu.
wizard-browse = Parcourir…
wizard-browse-title = Choisir le dossier de Deadlock
wizard-help = Aide
wizard-help-a = Dans Steam : clic droit sur Deadlock → Gérer → Parcourir les fichiers locaux. Choisissez ce dossier ; il se termine généralement par
wizard-help-b = .
wizard-exe-ok = Dans un dossier permanent
wizard-exe-downloads = deadloader est dans votre dossier Téléchargements
wizard-exe-desktop = deadloader est sur votre Bureau
wizard-exe-warn-a = Ses données sont à côté de l’exe : un rangement supprimerait votre bibliothèque et vos profils. Fermez deadloader, déplacez tout le dossier vers un emplacement permanent, par exemple
wizard-exe-warn-b = , et relancez-le depuis là.
wizard-open-folder = Ouvrir le dossier
wizard-opened = Ouvert dans l’Explorateur
wizard-linked = Les mods sont liés — aucun espace disque en plus
wizard-linked-text = deadloader et le jeu sont sur le même lecteur ({ $drive }).
wizard-copied = Les mods seront copiés, pas liés
wizard-copied-text = deadloader est sur { $data } et le jeu sur { $game }. Chaque mod activé occupe alors sa taille une deuxième fois ; la bibliothèque l’indique par « copié — +N Mo ».
wizard-learn-more = En savoir plus
wizard-drives-more = Un lien permet au jeu d’utiliser le fichier de la bibliothèque sans seconde copie. Windows ne peut lier que des fichiers sur le même lecteur. Si deadloader est aussi sur { $game }, rien n’est copié.
wizard-foreign-none = Aucun fichier de mod étranger
wizard-foreign-none-text = Le dossier de mods ne contient rien que deadloader ne connaisse.
wizard-foreign-some =
    { $count ->
        [one] { $count } fichier d’une autre source
       *[other] { $count } fichiers d’une autre source
    }
wizard-foreign-some-text = Ce n’est pas deadloader qui les a mis là — sans doute un autre gestionnaire de mods ou une installation manuelle. Il les laisse tels quels ; passez-les en revue plus tard dans la bibliothèque.
wizard-show-files = Afficher les fichiers
wizard-hide-files = Masquer les fichiers
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Sera configuré
wizard-gi-setup-a = deadloader ajoute une ligne à
wizard-gi-setup-b = quand vous appliquez des mods pour la première fois. Rien n’est modifié maintenant.
wizard-gi-ours = Déjà configuré
wizard-gi-ours-text = La ligne de deadloader est déjà là.
wizard-gi-foreign = Modifié par un autre programme
wizard-gi-foreign-text = Il ne ressemble pas à l’original — c’est souvent le fait de Deadlock Mod Manager. deadloader n’écrase pas les modifications des autres et vous expliquera quoi faire après la configuration.
wizard-what-is = Qu’est-ce que c’est ?
wizard-gi-more = est un fichier du jeu qui indique à Deadlock quels dossiers charger. En général, on le répare en vérifiant les fichiers du jeu dans Steam ; deadloader ajoute ensuite sa ligne.

# E1
wizard-nw-title = deadloader ne peut pas enregistrer ses données.
wizard-nw-why-a = Le dossier à côté de
wizard-nw-why-b = est en lecture seule : Windows n’autorise pas les programmes à y écrire (par exemple dans Program Files).
wizard-nw-path = Dossier de deadloader
wizard-nw-do = Que faire
wizard-nw-s1 = Quittez deadloader.
wizard-nw-s2a = Déplacez tout le dossier vers un emplacement où vous pouvez écrire, par exemple
wizard-nw-s2b = . N’utilisez pas Program Files.
wizard-nw-s3a = Lancez
wizard-nw-s3b = depuis le nouvel emplacement.
wizard-nw-why-folder = deadloader garde tout à côté de l’exe pour ne rien disperser sur votre PC : supprimer le dossier supprime entièrement deadloader.
wizard-nw-details = Détails
wizard-nw-copy = Copier les détails
wizard-nw-copied = Copié
wizard-nw-no-log = Non enregistré dans le journal — deadloader ne peut pas y écrire.
wizard-exit = Quitter
