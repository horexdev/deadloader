# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Fermer
library-apply = Appliquer
library-apply-profile = Appliquer { $name }
library-discard-all = Tout abandonner
library-keep = Garder { $name }
library-keep-applied = Garder les changements appliqués
library-undo-this = Annuler ce changement
library-undo-all = Tout annuler
library-close-to-apply = Fermez Deadlock — les changements s’appliqueront alors automatiquement
library-play-applies = Jouer l’applique aussi
library-applied-title = Changements appliqués
library-nothing-pending = Rien en attente
library-all-in-game = Tout le profil { $name } est déjà dans le jeu.
library-running-sub = Deadlock est en cours d’exécution, donc Windows verrouille ses fichiers. Ces changements seront appliqués automatiquement dès que vous fermerez le jeu.
library-waiting-sub = Ces changements ne sont pas encore dans le dossier du jeu. Appliquez-les maintenant ou appuyez sur Jouer.
library-switch-sub = Un changement de profil modifie beaucoup de fichiers à la fois, il attend donc votre accord. Appliquez-le ici ou appuyez sur Jouer — deadloader l’applique avant le lancement.
library-applied-sub =
    { $count ->
        [one] Deadlock a été fermé à { $time } : le changement fait pendant la partie a été appliqué. L’annulation reste possible jusqu’à votre prochain changement.
       *[other] Deadlock a été fermé à { $time } : les { $count } changements faits pendant la partie ont été appliqués. L’annulation reste possible jusqu’à votre prochain changement.
    }
library-undoing-sub =
    { $count ->
        [one] Vous avez annulé le changement appliqué à { $time }, et Deadlock tourne à nouveau. Il sera annulé automatiquement quand vous fermerez le jeu.
       *[other] Vous avez annulé les { $count } changements appliqués à { $time }, et Deadlock tourne à nouveau. Ils seront annulés automatiquement quand vous fermerez le jeu.
    }
library-group-enable = Activer · { $count }
library-group-disable = Désactiver · { $count }
library-group-order = Ordre · { $count }
library-group-variant = Variante · { $count }
library-group-version = Version · { $count }
library-group-switch = Changement de profil · { $count }
library-group-undo =
    { $count ->
        [one] Annulation du changement appliqué à { $time }
       *[other] Annulation des { $count } changements appliqués à { $time }
    }
library-change-on = Désactivé → activé
library-change-off = Activé → désactivé
library-change-above = Placé au-dessus de { $name }
library-change-above-wins = Placé au-dessus de { $name } — l’emporte maintenant dans leur conflit
library-change-below = Placé sous { $name }
library-switch-what-happens =
    { $on ->
        [one] 1 mod s’active
       *[other] { $on } mods s’activent
    } · { $off ->
        [one] 1 se désactive
       *[other] { $off } se désactivent
    }
library-what-happens = Ce qui se passe dans le dossier du jeu
library-folder-summary =
    { $added ->
        [one] 1 fichier ajouté
       *[other] { $added } fichiers ajoutés
    } · { $removed } retiré(s) · { $renumbered } renuméroté(s)

## Foreign files
library-foreign-title = Trouvé dans le dossier de mods du jeu
library-foreign-note = deadloader ne les a pas mis là et n’y touchera pas sauf si vous le décidez.
library-collapse = Réduire
library-expand = Développer
library-take = Prendre en charge
library-leave = Laisser tel quel
library-left = Laissé tel quel
library-foreign-open = Ouvrir le dossier contenant { $name }
library-foreign-vpk =
    { $count ->
        [one] VPK, { $count } fichier
       *[other] VPK, { $count } fichiers
    }
library-foreign-vpk-hero =
    { $count ->
        [one] VPK, { $count } fichier — ressemble à un skin pour { $hero }
       *[other] VPK, { $count } fichiers — ressemble à un skin pour { $hero }
    }
library-foreign-unreadable = Un fichier de mod que deadloader ne peut pas lire
library-foreign-replaced = Un fichier qui a pris la place d’un fichier de deadloader
library-foreign-dmm = Créé par Deadlock Mod Manager — un fichier de réglages, pas un mod
library-foreign-other = Un fichier, pas un mod
library-foreign-folder =
    { $count ->
        [one] Dossier, { $count } fichier — pas un mod
       *[other] Dossier, { $count } fichiers — pas un mod
    }
library-foreign-meta = { $size } · ajouté le { $date }
library-taken = { $name } est maintenant dans votre bibliothèque.

## D11
library-d11-title = Prendre en charge { $name } ?
library-d11-lead = deadloader va :
library-d11-step-copy = le copier dans votre bibliothèque comme mod local ({ $size }),
library-d11-step-replace = remplacer le fichier du dossier du jeu par sa propre copie liée,
library-d11-step-enable = l’activer dans le profil { $profile }, en bas de la liste.
library-d11-name = Nom dans votre bibliothèque
library-d11-hint = La bibliothèque garde une copie exacte du fichier d’origine.
library-d11-suggested = { $file } (skin { $hero })

## D10
library-d10-title = Un fichier du jeu a été modifié par un autre programme
library-d10-what = Un autre programme a modifié un fichier du jeu qui indique quels dossiers Deadlock charge — souvent Deadlock Mod Manager. deadloader n’écrase pas les modifications des autres, donc vos mods ne se chargeront pas tant que ce n’est pas réglé.
library-d10-do = Laissez Steam restaurer le fichier d’origine (Vérifier les fichiers du jeu). deadloader ajoute ensuite sa propre ligne et vous prévient.
library-d10-diff = Afficher les différences
library-d10-expected = Attendu (original + ligne de deadloader)
library-d10-found = Trouvé maintenant
library-d10-diff-note = En surbrillance : la ligne qu’ajoute deadloader, et les lignes ajoutées par un autre programme.
library-d10-help = Qu’est-ce que ce fichier ?
library-d10-help-text = gameinfo.gi indique à Deadlock quels dossiers charger. Les gestionnaires de mods y ajoutent une ligne pour que le jeu charge les mods du dossier addons. Les mises à jour du jeu la réinitialisent ; deadloader corrige cela automatiquement — mais seulement si le fichier est d’origine.
library-d10-wait-title = Steam vérifie les fichiers de Deadlock.
library-d10-wait-text = Quand Steam a terminé, revenez dans deadloader — il vérifie à nouveau le fichier automatiquement. Vous pouvez aussi vérifier maintenant.
library-d10-fixed = gameinfo.gi est configuré — les mods se chargent à nouveau.
library-d10-fixed-text = Steam a restauré le fichier d’origine et deadloader a ajouté sa ligne. Détails dans Activité → Fichiers.
library-open-file = Ouvrir le fichier
library-not-now = Pas maintenant
library-verify-steam = Vérifier les fichiers du jeu dans Steam
library-check-again = Vérifier à nouveau
library-show-differences = Afficher les différences

## D5
library-update = Mettre à jour
library-update-on-gb = Une version plus récente est sur GameBanana.
library-later = Plus tard
library-d5-title = Mettre à jour { $name } ?
library-d5-installed = Installée
library-d5-new = Nouvelle sur GameBanana
library-d5-downloaded = Téléchargée
library-d5-arrived = Dans la bibliothèque, pas encore active
library-d5-new-meta = Mise à jour le { $date } · { $size } à télécharger
library-d5-download = { $size } à télécharger
library-d5-changed = La nouvelle version a un autre ensemble de fichiers. deadloader les a associés ainsi :
library-d5-same = Ces fichiers sont remplacés par leurs nouvelles versions :
library-d5-map-text = deadloader n’a pas pu déterminer pour chaque fichier lequel le remplace. Choisissez le nouveau fichier pour chacun, ou gardez l’ancien.
library-d5-map-label = Association des fichiers
library-d5-col-old = Fichier installé
library-d5-col-new = Nouveau fichier
library-d5-pick = Nouveau fichier pour { $file }
library-d5-choose = Choisir…
library-d5-keep = Garder l’ancien fichier
library-d5-tag-replaces = Remplace
library-d5-tag-same = Même nom
library-d5-tag-new = Nouveau · sur la page du mod
library-d5-profiles = { $new } devient active dans chaque profil qui utilise { $old }. { $old } reste dans la bibliothèque, vous pouvez y revenir dans les détails du mod.
library-d5-install = Installer la mise à jour
library-d5-confirm-map = Utiliser ces fichiers
library-update-downloading = La mise à jour de { $name } se télécharge — elle sera installée à son arrivée.
library-updated = { $name } mis à jour.
library-updated-text = Active dans chaque profil qui utilisait l’ancienne version. L’ancienne version est conservée — vous pouvez y revenir dans les détails du mod.
