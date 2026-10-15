# The window frame and the sidebar: title bar, navigation, profile, game and network status, Play, D13.
nav-main = Navigation principale
nav-catalog = Catalogue
nav-library = Bibliothèque
nav-profiles = Profils
nav-downloads = Téléchargements
nav-activity = Activité
nav-settings = Paramètres
nav-pending = Modifications en attente
nav-pending-count = Modifications en attente : { $count }
nav-downloads-count = Téléchargements · { $count }
sidebar-collapse = Réduire la barre latérale
sidebar-expand = Développer la barre latérale
sidebar-close = Fermer la barre latérale
profile-applied = Appliqué au jeu
profile-not-applied = Sélectionné · non appliqué
profile-button = Profil { $name } · { $state }
game-found = Deadlock trouvé
game-running = Deadlock est en cours d’exécution
game-missing = Deadlock introuvable
game-on-drive = { $status } · { $drive }
game-at-path = { $status } — { $path }
network-online = En ligne
network-offline = Hors ligne
network-offline-detail = catalogue enregistré à { $time }
network-local = Mode local
network-local-detail = aucune requête
network-title-online = En ligne — ouvrir l’activité
network-title-running =
    En ligne — { $count ->
        [one] { $count } requête en cours
       *[other] { $count } requêtes en cours
    } — ouvrir l’activité
network-title-offline = Hors ligne — catalogue enregistré à { $time } — ouvrir l’activité
network-title-local = Mode local — aucune requête réseau — ouvrir l’activité
play = Jouer
play-applying = Application…
play-running = Deadlock est déjà lancé
play-missing = Deadlock est introuvable — choisissez d’abord le dossier du jeu
window-minimize = Réduire
window-maximize = Agrandir
window-close = Fermer
play-confirm-title = Appliquer { $profile } et lancer Deadlock ?
play-confirm-off =
    { $count ->
        [one] 1 mod sera désactivé :
       *[other] { $count } mods seront désactivés :
    }
play-confirm-on =
    { $count ->
        [one] 1 mod sera activé : { $mods }.
       *[other] { $count } mods seront activés : { $mods }.
    }
play-confirm-keep = { $applied } reste dans vos profils — vous pouvez y revenir à tout moment.
play-confirm-cancel = Annuler
play-confirm-play = Appliquer et jouer
toast-dismiss = Ignorer
