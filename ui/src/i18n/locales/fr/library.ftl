# The Library (S4): header, states, list, mod details, conflicts (C1), deleting (D6).
library-title = Bibliothèque
library-profile = Profil
library-enabled-count = { $enabled } sur { $total } activés
library-disk = { $size } sur le disque
library-not-checked = Mises à jour pas encore vérifiées
library-checked-today = Mises à jour vérifiées aujourd’hui à { $time }
library-checked-on = Mises à jour vérifiées le { $date }
library-game-not-found = Jeu introuvable
library-add-disk = Ajouter depuis le disque
library-add-title = Ajouter des mods depuis le disque
library-add-filter = Fichiers de mod
library-added =
    { $count ->
        [one] { $count } mod ajouté à la bibliothèque.
       *[other] { $count } mods ajoutés à la bibliothèque.
    }
library-check = Rechercher des mises à jour
library-checking = Recherche…
library-update-all = Tout mettre à jour
library-review = Voir
library-undo = Annuler
library-cancel = Annuler
library-missing-what = Deadlock est introuvable.
library-missing-why = Steam ou le jeu n’est pas installé, ou le dossier du jeu a été déplacé. Votre bibliothèque fonctionne toujours ; appliquer les changements et Jouer restent désactivés tant que le jeu n’est pas trouvé.
library-choose-folder = Choisir le dossier du jeu
library-search-again = Chercher à nouveau
library-running-what = Deadlock est en cours d’exécution.
library-running-why = Les changements seront appliqués automatiquement quand vous fermerez le jeu.
library-pending = Changements en attente
library-switch-what = Le profil { $name } n’est pas encore dans le jeu.
library-switch-why = Le jeu a toujours { $applied }. Appliquez { $name } maintenant ou appuyez sur Jouer — deadloader l’applique avant le lancement.
library-switch-why-first = Appliquez { $name } maintenant ou appuyez sur Jouer — deadloader l’applique avant le lancement.
library-review-switch = Voir le changement
library-applying-what = Application du profil { $name }…
library-applying-why = Vous pouvez continuer à naviguer ; Jouer revient quand c’est terminé.
library-show = Afficher
library-tab-all = Tous
library-tab-enabled = Activés
library-tab-disabled = Désactivés
library-tab-updates = Mises à jour
library-tab-conflicts = Conflits
library-tab-local = Locaux
library-search = Rechercher dans la bibliothèque
library-search-clear = Effacer la recherche
library-higher-wins = Quand deux mods modifient le même fichier, celui placé plus haut dans la liste l’emporte.
library-reorder-hint = Réordonner : faites glisser la poignée, ou donnez-lui le focus et appuyez sur Espace
library-list-label = Mods du profil { $name }, par ordre de priorité
library-toggle = { $name } activé
library-toggle-missing = Deadlock est introuvable — choisissez le dossier du jeu pour appliquer les changements
library-grip = Déplacer { $name }, position { $position } sur { $total }
library-more = Autres actions pour { $name }
library-show-variants = Afficher les variantes
library-hide-variants = Masquer les variantes
library-show-addons = Afficher les modules complémentaires
library-hide-addons = Masquer les modules complémentaires
library-live-picked = { $name } — saisi, position { $position } sur { $total }. ↑ ↓ pour déplacer, Espace pour déposer, Échap pour annuler.
library-live-moved = { $name } — position { $position } sur { $total }.
library-live-moved-grabbed = { $name } — position { $position } sur { $total }. Espace pour déposer, Échap pour annuler.
library-live-dropped = { $name } — déposé en position { $position } sur { $total }.
library-live-back = { $name } — de retour en position { $position } sur { $total }.
library-live-drag = { $name } — relâchez pour déposer au-dessus de { $above }, en position { $position } sur { $total }.
library-badge-pending = En attente
library-badge-damaged = Fichiers endommagés
library-badge-update = Mise à jour disponible
library-badge-wins = Conflit · l’emporte
library-badge-loses = Conflit · perd
library-badge-variants =
    { $count ->
        [one] { $count } variante
       *[other] { $count } variantes
    }
library-badge-local = Local
library-badge-copied = Copié · +{ $size }
library-local = Local
library-unknown-author = Auteur inconnu
library-gamebanana = GameBanana
library-from-disk = depuis le disque
library-addons-sub =
    { $count ->
        [one] + { $count } module
       *[other] + { $count } modules
    }
library-variants = Variantes
library-variants-downloaded = { $count } téléchargée(s)
library-active = Active
library-select-to-switch = Sélectionnez pour basculer
library-parts = Fichiers de ce mod
library-main-files = Fichiers principaux
library-from = de { $archive }
library-on-with-mod = Activé avec le mod · { $size }
library-remove-addon = Retirer le module
library-toast-enabled = { $name } activé.
library-toast-disabled = { $name } désactivé.
library-toast-variant = { $name } est passé à { $variant }.
library-empty-title = Votre bibliothèque est vide
library-empty-text = Installez des mods depuis le catalogue, ou ajoutez des fichiers .vpk, .zip, .7z ou .rar depuis votre ordinateur. Vous pouvez aussi déposer des fichiers n’importe où sur cette fenêtre.
library-empty-local-text = En mode local, le catalogue est désactivé. Ajoutez des fichiers .vpk, .zip, .7z ou .rar depuis votre ordinateur, ou déposez-les n’importe où sur cette fenêtre.
library-browse = Parcourir le catalogue
library-no-match-title = Aucun mod ici
library-no-match-text = Aucun mod ne correspond à cet onglet et à cette recherche. Essayez un autre onglet ou effacez la recherche.

## Mod details
library-details = Détails du mod
library-open-mod-page = Ouvrir la page du mod
library-close-details = Fermer les détails
library-enabled-in = Activé dans { $profile }
library-position = Position { $position } sur { $total }
library-version = Version
library-no-version = Aucune version indiquée
library-from-gamebanana = Depuis GameBanana
library-imported = Importé depuis le disque
library-newer-in-library = Une version plus récente est dans la bibliothèque.
library-switch-to = Passer à
library-delete-old = Supprimer les anciennes versions
library-no-older = Aucune ancienne version conservée
library-older =
    { $count ->
        [one] { $count } ancienne version conservée · { $size }
       *[other] { $count } anciennes versions conservées · { $size }
    }
library-files = Fichiers
library-on-disk = { $size } sur le disque
library-linked = Lié — aucun espace disque en plus
library-copied = Copié — { $size } d’espace disque en plus
library-not-in-game = Pas dans le dossier du jeu
library-local-mod = Mod local
library-name = Nom dans votre bibliothèque
library-rename = Renommer
library-link-label = Page GameBanana
library-link-hint = Collez l’adresse du mod pour vérifier ses mises à jour.
library-link = Lier
library-link-done = Lié : { $matched } fichier(s) sur { $total } trouvé(s) sur la page.
library-link-not-a-page = Ce n’est pas une page de mod GameBanana. Collez une adresse comme gamebanana.com/mods/123456.
library-link-removed = Ce mod n’est plus sur GameBanana.
library-link-failed = La page n’a pas pu être lue. Vérifiez l’adresse et réessayez.
library-imported-from = Importé depuis
library-no-conflicts = Aucun conflit avec les autres mods activés
library-conflict-wins =
    { $count ->
        [one] L’emporte sur { $name } · { $count } fichier
       *[other] L’emporte sur { $name } · { $count } fichiers
    }
library-conflict-loses =
    { $count ->
        [one] Perd face à { $name } · { $count } fichier
       *[other] Perd face à { $name } · { $count } fichiers
    }
library-show-conflicts = Afficher les conflits
library-delete-mod = Supprimer le mod…
library-open-folder = Ouvrir le dossier

## Conflicts (C1)
library-conflicts-sub = Deux mods activés modifient les mêmes fichiers du jeu. Celui placé plus haut dans la liste l’emporte.
library-close-conflicts = Fermer les conflits
library-no-conflicts-title = Aucun conflit
library-wins = L’emporte
library-loses = Perd
library-off = Désactivé
library-in-use = Utilisé
library-conflict-text =
    { $count ->
        [one] Les deux modifient { $count } fichier. En jeu, vous voyez la version de { $winner } ; la copie de { $loser } n’est pas utilisée.
       *[other] Les deux modifient { $count } fichiers. En jeu, vous voyez les versions de { $winner } ; les copies de { $loser } ne sont pas utilisées.
    }
library-conflict-off =
    { $count ->
        [one] Les deux modifient { $count } fichier. { $off } est désactivé, donc le jeu utilise la version de { $other }.
       *[other] Les deux modifient { $count } fichiers. { $off } est désactivé, donc le jeu utilise les versions de { $other }.
    }
library-files-count =
    { $count ->
        [one] { $count } fichier
       *[other] { $count } fichiers
    }
library-show-more = Afficher { $count } de plus
library-move-above = Placer { $name } au-dessus
library-disable = Désactiver { $name }
library-pick-variant = Choisir une autre variante
library-no-variants = Aucun des deux mods n’a de variantes
library-done-wins = C’est fait — { $name } l’emporte maintenant
library-done-off = C’est fait — { $off } est désactivé ; les fichiers de { $other } sont utilisés
library-identical = Les fichiers identiques (même somme de contrôle) ne sont pas des conflits et ne sont pas listés.

## D6 Delete mod
library-d6-title = Supprimer « { $name } » ?
library-d6-files-title = Supprimer les anciennes versions de « { $name } » ?
library-d6-what =
    { $count ->
        [one] { $count } fichier · { $size }.
       *[other] { $count } fichiers · { $size }.
    }
library-d6-profiles =
    { $count ->
        [one] Utilisé dans { $count } profil : { $names } — il en sera retiré, ainsi que du dossier du jeu.
       *[other] Utilisé dans { $count } profils : { $names } — il en sera retiré, ainsi que du dossier du jeu.
    }
library-d6-no-profiles = Aucun profil ne l’utilise.
library-d6-where = Supprimer
library-d6-permanently = Définitivement
library-d6-recycle-bin = Vers la corbeille
library-d6-permanent-hint = Les fichiers sont supprimés définitivement — c’est irréversible.
library-d6-bin-hint = Vous pourrez le restaurer depuis la corbeille de Windows.
library-d6-confirm = Supprimer le mod
library-d6-confirm-files = Supprimer les fichiers
library-deleted = { $name } supprimé.
library-deleted-files =
    { $count ->
        [one] { $count } fichier de { $name } supprimé.
       *[other] { $count } fichiers de { $name } supprimés.
    }
