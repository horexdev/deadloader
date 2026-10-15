## Notifications

n1 = Deadlock a été mis à jour — mods restaurés.
    .why = La mise à jour a réinitialisé gameinfo.gi ; deadloader y a donc rajouté sa ligne.
n2 = { $name } installé et activé.
n3 =
    { $count ->
        [one] { $count } mise à jour de mod disponible.
       *[other] { $count } mises à jour de mods disponibles.
    }
n4 = Deadlock est en cours d’exécution.
    .why = Les modifications seront appliquées automatiquement à la fermeture du jeu.
    .pending =
        { $count ->
            [one] { $count } modification en attente
           *[other] { $count } modifications en attente
        }
n5 =
    { $reason ->
        [completed] Une modification interrompue a été terminée.
       *[undone] Une modification interrompue a été annulée ; le dossier du jeu est comme avant.
    }
n6 =
    { $count ->
        [one] { $count } fichier que deadloader n’a pas créé a été trouvé dans le dossier des mods du jeu.
       *[other] { $count } fichiers que deadloader n’a pas créés ont été trouvés dans le dossier des mods du jeu.
    }
n7 =
    { $when ->
        [today] Hors ligne — affichage du catalogue enregistré à { $saved }.
       *[earlier] Hors ligne — affichage du catalogue enregistré le { $saved }.
    }
    .why = Les téléchargements reprendront dès que vous serez de nouveau en ligne.
n8 =
    { $count ->
        [one] Profil « { $name } » appliqué — { $count } mod activé.
       *[other] Profil « { $name } » appliqué — { $count } mods activés.
    }
n9 = Une nouvelle version de deadloader est disponible : { $version }.
n10 = Le mode local est activé.
    .why = Le catalogue est désactivé et deadloader n’effectue aucune requête réseau.
n11 = Les mods sont copiés au lieu d’être liés, car deadloader et le jeu se trouvent sur des lecteurs différents.
    .why = Cela occupe { $size } d’espace disque supplémentaire.
n12 =
    { $count ->
        [one] Deadlock a été fermé — { $count } modification appliquée.
       *[other] Deadlock a été fermé — { $count } modifications appliquées.
    }

## Errors: what happened, why, what to do

e1 = deadloader ne peut pas enregistrer ses données.
    .why =
        { $reason ->
            [access_denied] Le dossier de deadloader.exe est en lecture seule — par exemple, Windows n’autorise pas les programmes à écrire dans « Programmes » (Program Files).
            [in_use] Un autre programme, souvent un antivirus, a ouvert l’un des fichiers de deadloader.
           *[other] Windows n’a pas pu écrire dans le dossier de deadloader.exe.
        }
    .todo =
        { $reason ->
            [in_use] Fermez les autres programmes et réessayez.
           *[other] Déplacez deadloader dans un dossier où vous pouvez écrire, par exemple D:\Games\deadloader.
        }
e2 = Deadlock est introuvable.
    .why = Steam ou le jeu n’est pas installé, ou le dossier du jeu a été déplacé.
    .todo = Choisissez le dossier du jeu manuellement.
e3 = Un fichier du jeu a été modifié par un autre programme.
    .why = gameinfo.gi ne ressemble pas à l’original — souvent, c’est Deadlock Mod Manager qui l’a modifié. deadloader n’écrase pas les modifications des autres.
    .todo = Faites vérifier l’intégrité des fichiers du jeu par Steam ; deadloader ajoutera ensuite sa ligne.
e4 = Le catalogue est indisponible.
    .why = GameBanana a modifié son API, deadloader ne peut donc plus la lire. Votre bibliothèque et vos profils fonctionnent comme d’habitude.
    .todo = Vérifiez s’il existe une mise à jour de deadloader, ou réessayez plus tard.
e5 = GameBanana est surchargé.
    .why =
        { $reason ->
            [rate_limited] Trop de requêtes en peu de temps (HTTP 429).
            [server] Le serveur de GameBanana a signalé une erreur.
           *[other] Trop de requêtes, ou une erreur du serveur.
        }
    .todo =
        { $wait ->
            [seconds]
                { $seconds ->
                    [one] deadloader réessaiera automatiquement dans { $seconds } seconde.
                   *[other] deadloader réessaiera automatiquement dans { $seconds } secondes.
                }
           *[later] deadloader réessaiera automatiquement dans un instant.
        }
e6 = Échec du téléchargement.
    .why =
        { $reason ->
            [connection] La connexion a été perdue.
            [timeout] Le serveur a mis trop de temps à répondre.
            [server] Le serveur de GameBanana a signalé une erreur.
           *[other] Le téléchargement a été interrompu.
        }
    .todo = Il reprendra là où il s’est arrêté.
e7 = Le fichier téléchargé est endommagé.
    .why = Sa somme de contrôle ne correspond pas à celle indiquée par GameBanana.
    .todo = Téléchargez-le à nouveau.
e8 = Impossible d’extraire l’archive.
    .why =
        { $reason ->
            [not_an_archive] Le fichier n’est pas une archive ZIP, 7z ou RAR — le téléchargement est peut-être incomplet.
            [damaged] L’archive est endommagée.
            [encrypted] L’archive est protégée par un mot de passe.
            [unsupported] Elle utilise une méthode de compression que deadloader ne prend pas en charge.
            [not_a_vpk] { $entry } n’est pas un fichier .vpk valide.
           *[other] Elle est endommagée ou utilise un format non pris en charge.
        }
    .todo = Essayez un autre fichier de ce mod, ou signalez le problème à son auteur.
e9 = L’archive est trop volumineuse pour être extraite en toute sécurité.
    .why =
        { $reason ->
            [too_large] Une fois extraite, elle dépasse { $limit }.
            [too_many_files]
                { $found ->
                    [one] Elle contient { $found } fichier de mod, et la limite est de { $limit }.
                   *[other] Elle contient { $found } fichiers de mod, et la limite est de { $limit }.
                }
            [ratio] { $entry } est compressé de façon suspecte — plus de { $limit } fois.
            [memory] Son extraction nécessite { $needed } de mémoire, et la limite est de { $limit }.
           *[other] Elle dépasse l’une des limites de sécurité des archives.
        }
    .todo =
        { $reason ->
            [memory] Essayez un autre fichier de ce mod.
           *[other] Si vous lui faites confiance, augmentez la limite dans Paramètres → Téléchargements.
        }
e10 = Aucun fichier de mod dans cette archive.
    .why =
        { $listing ->
            [files] Elle ne contient aucun fichier .vpk, seulement { $contents }.
           *[empty] L’archive est vide.
        }
    .todo = Ce fichier n’est pas un mod Deadlock dans un format que deadloader peut installer.
e11 = Impossible d’extraire les archives RAR sur ce PC.
    .why = L’outil intégré à Windows ne peut pas lire ce RAR, et ni 7-Zip ni WinRAR n’ont été trouvés.
    .todo = Installez 7-Zip (gratuit) et réessayez ; le fichier téléchargé est conservé.
e12 = Espace disque insuffisant.
    .why =
        { $reason ->
            [space] Il faut { $needed }, mais seuls { $available } sont libres.
           *[full] Le disque est plein.
        }
    .todo = Libérez de l’espace ou supprimez d’anciennes versions de mods dans la Bibliothèque, puis réessayez.
e13 = Impossible d’appliquer les modifications tant que Deadlock est en cours d’exécution.
    .why = Windows verrouille les fichiers du jeu tant qu’il tourne.
    .todo = Fermez Deadlock ; deadloader appliquera les modifications juste après.
e14 = Un fichier de mod est manquant ou endommagé dans la bibliothèque.
    .why = Il a été modifié ou supprimé en dehors de deadloader.
    .todo = Réparez le mod pour retélécharger le fichier, ou supprimez le mod.
e15 = Impossible d’ouvrir le lien.
    .why = Aucun navigateur par défaut n’est défini dans Windows.
    .todo = Copiez le lien et ouvrez-le vous-même.
e16 = Requête bloquée.
    .why = GameBanana a redirigé vers { $host }, qui ne figure pas dans la liste des hôtes autorisés.
    .todo = Rien à faire — deadloader ne communique qu’avec GameBanana. Si vous faites confiance à cet hôte, ajoutez-le dans Paramètres → Avancé.
e17 = Impossible d’importer le profil.
    .why =
        { $reason ->
            [newer] Il a été créé par une version plus récente de deadloader.
            [unreadable] Le fichier est illisible — il a peut-être été déplacé, supprimé ou ouvert par un autre programme.
           *[not_a_profile] Ce fichier n’est pas un profil deadloader.
        }
    .todo =
        { $reason ->
            [newer] Mettez à jour deadloader, puis importez à nouveau le profil.
            [unreadable] Vérifiez que le fichier est toujours là et qu’aucun autre programme ne l’a ouvert, puis réessayez.
           *[not_a_profile] Vérifiez que vous avez choisi le bon fichier.
        }
e18 = Impossible de modifier les fichiers du dossier du jeu.
    .why =
        { $reason ->
            [access_denied] Windows a refusé l’accès au dossier du jeu.
            [in_use] Un fichier est utilisé par un autre programme, comme un antivirus ou un autre gestionnaire de mods.
           *[other] Windows n’a pas pu modifier les fichiers.
        }
    .todo = Fermez les autres programmes et réessayez.
e19 = Impossible de charger l’image.
    .why =
        { $reason ->
            [no_response] L’hôte n’a pas répondu.
            [not_an_image] Le fichier n’est pas une image.
            [too_large] L’image est trop volumineuse.
           *[other] L’hôte n’a pas envoyé d’image utilisable.
        }
    .todo = L’espace réservé reste affiché ; vous pouvez réessayer.
e20 = Trop de mods sont activés.
    .why =
        { $free ->
            [one] Le jeu peut encore charger { $free } fichier de mod, et ce profil en demande { $wanted }.
           *[other] Le jeu peut encore charger { $free } fichiers de mod, et ce profil en demande { $wanted }.
        }
    .todo = Désactivez quelques mods ou choisissez moins d’add-ons.
e21 = Un profil nommé « { $name } » existe déjà.
    .why = Les noms sont comparés sans tenir compte des majuscules et des minuscules, comme les noms de fichiers.
    .todo = Choisissez un autre nom.
e22 = « { $name } » ne peut pas servir de nom de profil.
    .why = Windows ne l’accepte pas comme nom de fichier — par exemple CON, un point final ou l’un des caractères \ / : * ? " < > |.
    .todo = Choisissez un autre nom.
e23 = deadloader ne peut pas lire son fichier { $file }.
    .why =
        { $reason ->
            [damaged_at] Le fichier est endommagé (ligne { $line }, colonne { $column }).
            [damaged] Le fichier est endommagé.
            [newer] Il a été créé par une version plus récente de deadloader.
           *[invalid] Son contenu ne correspond pas à ce qu’attend deadloader.
        }
    .todo =
        { $kind ->
            [settings] Pour l’instant, deadloader utilise les paramètres par défaut et laisse le fichier tel quel.
            [manifest] Restaurez manifest.json.bak, ou mettez à jour deadloader.
           *[other] Restaurez le fichier à partir d’une sauvegarde, ou mettez à jour deadloader.
        }
internal = Une erreur s’est produite dans deadloader.
    .why = C’est une erreur de deadloader, pas de votre part.
    .todo = Réessayez. Si cela se reproduit, signalez-le avec l’entrée du journal indiquée dans Détails.

## Buttons named in the texts above

action-undo = Annuler
action-review = Voir
action-retry = Réessayer
action-retry-now = Réessayer maintenant
action-browse = Parcourir…
action-verify-in-steam = Vérifier dans Steam
action-show-differences = Afficher les différences
action-details = Détails
action-repair = Réparer
action-copy-link = Copier le lien
action-open-folder = Ouvrir le dossier
action-open-release-page = Ouvrir la page de la version
action-enable-catalog = Activer le catalogue
action-learn-more = En savoir plus
action-view-log = Voir le journal

## Shared pieces

log-entry = Entrée du journal { $id }
list-more =
    { $count ->
        [one] { $count } autre
       *[other] { $count } autres
    }

## Sizes, labelled like Windows Explorer labels them

size-bytes =
    { $count ->
        [one] { $count } octet
       *[other] { $count } octets
    }
size-kb = { $value } Ko
size-mb = { $value } Mo
size-gb = { $value } Go
size-tb = { $value } To
