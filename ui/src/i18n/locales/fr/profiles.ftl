# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Profils
profiles-count =
    { $count ->
        [one] { $count } profil
       *[other] { $count } profils
    }
profiles-in-game = Dans le jeu en ce moment : { $list }
profiles-in-game-item = « { $name } » sur { $drive }
profiles-in-game-none = Aucun profil n’est dans le jeu en ce moment.
profiles-import = Importer…
profiles-import-title = Importer un profil depuis un fichier .json
profiles-import-dialog = Importer un profil
profiles-export-dialog = Exporter le profil
profiles-file-filter = Profils deadloader
profiles-new = Nouveau profil
profiles-selected = Sélectionné
profiles-applied = Appliqué
profiles-not-applied = Pas encore appliqué
profiles-selected-title = Le profil que vous modifiez dans la bibliothèque et avec lequel vous jouez
profiles-pending-title = Appliquer ou Jouer le place dans le dossier du jeu
profiles-applied-title = Ses fichiers sont actuellement dans le dossier du jeu sur { $drive }
profiles-select = Sélectionner
profiles-select-of = Sélectionner « { $name } »
profiles-more-of = Autres actions pour « { $name } »
profiles-rename = Renommer
profiles-duplicate = Dupliquer
profiles-export = Exporter dans un fichier…
profiles-change-installation = Changer l’installation du jeu…
profiles-delete = Supprimer
profiles-delete-confirm = Supprimer…
profiles-delete-last = deadloader a besoin d’au moins un profil.
profiles-name-label = Nom du profil
profiles-save = Enregistrer
profiles-cancel = Annuler
profiles-close = Fermer
profiles-close-esc = Fermer (Échap)
profiles-rename-hint = Entrée enregistre · Échap annule
profiles-name-empty = Saisissez un nom pour le profil.
profiles-mods =
    { $count ->
        [one] { $count } mod activé
       *[other] { $count } mods activés
    }
profiles-mods-none = Aucun mod activé
profiles-applied-when = Appliqué { $when }
profiles-last-applied = Appliqué pour la dernière fois { $when }
profiles-never-applied = Jamais appliqué
profiles-just-now = à l’instant
profiles-preview-empty = Sélectionnez ce profil, puis activez des mods dans la bibliothèque.
profiles-footnote = Un profil est une liste de mods activés avec leur ordre, leurs variantes et leurs versions. Le sélectionner ne télécharge ni ne supprime rien ; Appliquer ne modifie que les fichiers qui diffèrent.
profiles-copy-name = { $name } (copie)
profiles-copy-name-n = { $name } (copie { $number })
profiles-new-title = Nouveau profil
profiles-start-with = Commencer avec
profiles-empty = Vide
profiles-empty-hint = Aucun mod activé — choisissez-les ensuite dans la bibliothèque.
profiles-copy-of = Copie de « { $name } »
profiles-copy-hint = { $mods } — même ordre, mêmes variantes et versions.
profiles-installation = Installation du jeu
profiles-installation-hint = La même que pour « { $name } ». Vous pourrez la changer plus tard depuis le menu du profil.
profiles-new-note = Créer un profil ne le sélectionne ni ne l’applique — le jeu ne change pas.
profiles-create = Créer le profil
profiles-pending-what = Le profil « { $name } » n’est pas encore dans le jeu.
profiles-pending-why-other = Le dossier du jeu sur { $drive } contient encore « { $other } ». Appliquez-le maintenant ou appuyez sur Jouer — deadloader l’applique avant le lancement.
profiles-pending-why-empty = Le dossier du jeu sur { $drive } ne contient actuellement aucun mod de deadloader. Appliquez-le maintenant ou appuyez sur Jouer — deadloader l’applique avant le lancement.
profiles-keep = Garder « { $name } »
profiles-apply = Appliquer « { $name } »
profiles-other-what = « { $name } » utilise une autre installation du jeu.
profiles-other-why = Appliquer et Jouer utilisent { $path }. Le jeu sur { $drive } garde « { $other } ».
profiles-other-why-empty = Appliquer et Jouer utilisent { $path }. Le jeu sur { $drive } ne change pas.
profiles-delete-title = Supprimer le profil « { $name } » ?
profiles-delete-applied =
    { $count ->
        [one] « { $name } » est actuellement dans le jeu. deadloader retirera son { $count } mod du dossier du jeu :
       *[other] « { $name } » est actuellement dans le jeu. deadloader retirera ses { $count } mods du dossier du jeu :
    }
profiles-delete-applied-empty = « { $name } » est actuellement dans le jeu mais n’a aucun mod : le dossier du jeu ne change pas.
profiles-delete-stay = Les mods restent dans votre bibliothèque et dans les autres profils. Tant que vous n’y appliquez pas un autre profil, Deadlock démarre sans mods de deadloader.
profiles-delete-next = « { $name } » devient le profil sélectionné. Il n’est appliqué que lorsque vous appuyez sur Appliquer ou Jouer.
profiles-delete-undoable = Cette action est irréversible. Pour garder une copie, exportez d’abord le profil.
profiles-export-first = Exporter d’abord
profiles-delete-profile = Supprimer le profil
profiles-installation-title = Installation du jeu pour « { $name } »
profiles-installation-lead = Choisissez le dossier de Deadlock auquel ce profil est appliqué.
profiles-used-by =
    { $count ->
        [one] utilisée par { $count } profil
       *[other] utilisée par { $count } profils
    }
profiles-used-by-none = utilisée par aucun profil
profiles-installation-applied = « { $name } » est actuellement dans le jeu sur { $drive }. Ses fichiers y restent jusqu’à ce que vous appliquiez un autre profil sur { $drive }.
profiles-installation-not-applied = Aucun dossier du jeu ne change tant que vous n’appliquez pas « { $name } ».
profiles-manage-installations = Gérer les installations dans les Paramètres
profiles-use-installation = Utiliser cette installation
profiles-d9-title =
    { $count ->
        [one] { $count } mod de « { $name } » n’est pas dans votre bibliothèque
       *[other] { $count } mods de « { $name } » ne sont pas dans votre bibliothèque
    }
profiles-d9-file = Le fichier
profiles-d9-lead = liste { $listed } mods. { $present } sont déjà dans votre bibliothèque ; ces { $count } n’y sont pas :
profiles-d9-list = Mods manquants
profiles-d9-from-gamebanana = { $count } de GameBanana
profiles-d9-local-count = { $count } local(aux)
profiles-gamebanana = GameBanana
profiles-can-download = Peut être téléchargé
profiles-local-mod = Mod local
profiles-cant-download = Ne peut pas être téléchargé
profiles-local-why = Ajouté depuis le disque sur un autre PC, il n’est donc pas sur GameBanana. Demandez le fichier à la personne qui a partagé le profil.
profiles-d9-local-mode = Le catalogue est désactivé (mode local) : deadloader ne peut pas télécharger de mods. Importez sans eux, puis ajoutez-les depuis le disque — ou activez le catalogue dans les Paramètres.
profiles-d9-after = Dans les deux cas, « { $name } » devient votre profil sélectionné sans être appliqué — le dossier du jeu ne change que lorsque vous appuyez sur Appliquer ou Jouer.
profiles-import-without = Importer sans eux
profiles-download-missing = Télécharger les manquants ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } mod désactivé
       *[other] { $count } mods désactivés
    }
profiles-turned-on =
    { $count ->
        [one] { $count } mod activé
       *[other] { $count } mods activés
    }
profiles-differ = Seuls les fichiers qui diffèrent ont été modifiés.
profiles-same = Les mêmes mods étaient déjà dans le jeu ; seuls l’ordre et les variantes ont été vérifiés.
profiles-deleted = Profil « { $name } » supprimé.
profiles-deleted-not-applied = Il n’était pas appliqué : le dossier du jeu n’a pas changé.
profiles-deleted-applied =
    { $count ->
        [one] { $count } mod a été retiré du dossier du jeu. Il est toujours dans votre bibliothèque.
       *[other] { $count } mods ont été retirés du dossier du jeu. Ils sont toujours dans votre bibliothèque.
    }
profiles-exported = Profil « { $name } » exporté.
profiles-imported = Profil « { $name } » importé.
profiles-imported-text = Il est sélectionné mais pas encore appliqué.
profiles-imported-without =
    { $count ->
        [one] Profil « { $name } » importé sans { $count } mod.
       *[other] Profil « { $name } » importé sans { $count } mods.
    }
profiles-imported-without-text = Il est sélectionné mais pas encore appliqué. Vous pourrez ajouter les mods manquants plus tard.
profiles-imported-downloading =
    { $count ->
        [one] Profil « { $name } » importé — { $count } mod est en téléchargement.
       *[other] Profil « { $name } » importé — { $count } mods sont en téléchargement.
    }
profiles-imported-downloading-text = Il est sélectionné mais pas encore appliqué. Appliquez-le une fois les téléchargements terminés.
profiles-open-downloads = Ouvrir les téléchargements
profiles-undo = Annuler
profiles-check-update = Rechercher une mise à jour de deadloader
profiles-choose-another = Choisir un autre fichier
profiles-switcher = Gérer les profils
