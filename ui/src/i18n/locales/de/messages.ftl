## Notifications

n1 = Deadlock wurde aktualisiert — Mods wiederhergestellt.
    .why = Das Update hat gameinfo.gi zurückgesetzt, deshalb hat deadloader seine Zeilen wieder eingefügt.
n2 = { $name } installiert und aktiviert.
n3 =
    { $count ->
        [one] { $count } Mod-Update verfügbar.
       *[other] { $count } Mod-Updates verfügbar.
    }
n4 = Deadlock läuft.
    .why = Änderungen werden automatisch übernommen, sobald du das Spiel schließt.
    .pending =
        { $count ->
            [one] { $count } Änderung ausstehend
           *[other] { $count } Änderungen ausstehend
        }
n5 =
    { $reason ->
        [completed] Eine unterbrochene Änderung wurde abgeschlossen.
       *[undone] Eine unterbrochene Änderung wurde rückgängig gemacht; dein Spielordner ist wie vorher.
    }
n6 =
    { $count ->
        [one] Im Mod-Ordner des Spiels wurde { $count } Datei gefunden, die nicht von deadloader stammt.
       *[other] Im Mod-Ordner des Spiels wurden { $count } Dateien gefunden, die nicht von deadloader stammen.
    }
n7 =
    { $when ->
        [today] Offline — du siehst den Katalog, der um { $saved } gespeichert wurde.
       *[earlier] Offline — du siehst den Katalog, der am { $saved } gespeichert wurde.
    }
    .why = Downloads werden fortgesetzt, sobald du wieder online bist.
n8 =
    { $count ->
        [one] Profil „{ $name }“ angewendet — { $count } Mod aktiviert.
       *[other] Profil „{ $name }“ angewendet — { $count } Mods aktiviert.
    }
n9 = Eine neue deadloader-Version ist verfügbar: { $version }.
n10 = Der lokale Modus ist an.
    .why = Der Katalog ist aus, und deadloader stellt keine Netzwerkanfragen.
n11 = Mods werden kopiert statt verknüpft, weil deadloader und das Spiel auf verschiedenen Laufwerken liegen.
    .why = Das belegt zusätzlich { $size } Speicherplatz.
n12 =
    { $count ->
        [one] Deadlock wurde geschlossen — { $count } Änderung übernommen.
       *[other] Deadlock wurde geschlossen — { $count } Änderungen übernommen.
    }

## Errors: what happened, why, what to do

e1 = deadloader kann seine Daten nicht speichern.
    .why =
        { $reason ->
            [access_denied] Der Ordner neben deadloader.exe ist schreibgeschützt — Windows erlaubt Programmen zum Beispiel nicht, in „Programme“ (Program Files) zu schreiben.
            [in_use] Ein anderes Programm, oft ein Virenschutz, hat eine Datei von deadloader geöffnet.
           *[other] Windows konnte nicht in den Ordner neben deadloader.exe schreiben.
        }
    .todo =
        { $reason ->
            [in_use] Schließe andere Programme und versuche es erneut.
           *[other] Verschiebe deadloader in einen Ordner, in den du schreiben darfst, z. B. D:\Games\deadloader.
        }
e2 = Deadlock wurde nicht gefunden.
    .why = Steam oder das Spiel ist nicht installiert, oder der Spielordner wurde verschoben.
    .todo = Wähle den Spielordner manuell aus.
e3 = Eine Spieldatei wurde von einem anderen Programm geändert.
    .why = gameinfo.gi sieht nicht wie das Original aus — oft war das Deadlock Mod Manager. deadloader überschreibt keine fremden Änderungen.
    .todo = Lass Steam die Spieldateien auf Fehler überprüfen; danach ergänzt deadloader seine Zeilen.
e4 = Der Katalog ist nicht verfügbar.
    .why = GameBanana hat seine Schnittstelle geändert, deshalb kann deadloader sie nicht lesen. Deine Bibliothek und deine Profile funktionieren wie gewohnt.
    .todo = Suche nach einem deadloader-Update oder versuche es später erneut.
e5 = GameBanana ist ausgelastet.
    .why =
        { $reason ->
            [rate_limited] Zu viele Anfragen in kurzer Zeit (HTTP 429).
            [server] Der Server von GameBanana hat einen Fehler gemeldet.
           *[other] Zu viele Anfragen oder ein Serverfehler.
        }
    .todo =
        { $wait ->
            [seconds]
                { $retry_in_s ->
                    [one] deadloader versucht es in { $retry_in_s } Sekunde automatisch erneut.
                   *[other] deadloader versucht es in { $retry_in_s } Sekunden automatisch erneut.
                }
           *[later] deadloader versucht es gleich automatisch erneut.
        }
e6 = Download fehlgeschlagen.
    .why =
        { $reason ->
            [local_mode] deadloader ist im lokalen Modus und stellt keine Netzwerkanfragen.
            [dns] Die Adresse des Download-Servers wurde nicht gefunden.
            [connect] deadloader konnte keine Verbindung zum Download-Server herstellen.
            [tls] Eine sichere Verbindung zum Download-Server kam nicht zustande.
            [timeout] Der Server hat zu lange nicht geantwortet.
            [connection_lost] Die Verbindung wurde unterbrochen.
            [http_status] Der Server hat mit einem Fehler geantwortet (HTTP { $status }).
           *[other] Der Download wurde unterbrochen.
        }
    .todo =
        { $reason ->
            [local_mode] Schalte den Katalog in den Einstellungen ein, um Mods herunterzuladen.
           *[other] Er wird dort fortgesetzt, wo er angehalten hat.
        }
e7 = Die heruntergeladene Datei ist beschädigt.
    .why =
        { $reason ->
            [size] Ihre Größe stimmt nicht mit der von GameBanana überein.
           *[checksum] Ihre Prüfsumme stimmt nicht mit der von GameBanana überein.
        }
    .todo = Lade sie erneut herunter.
e8 = Das Archiv kann nicht entpackt werden.
    .why =
        { $reason ->
            [not_an_archive] Die Datei ist kein ZIP-, 7z- oder RAR-Archiv — vielleicht ist der Download unvollständig.
            [damaged] Das Archiv ist beschädigt.
            [encrypted] Das Archiv ist mit einem Passwort geschützt.
            [unsupported] Es verwendet eine Kompressionsmethode, die deadloader nicht unterstützt.
            [not_a_vpk] { $entry } ist keine gültige .vpk-Datei.
           *[other] Es ist beschädigt oder hat ein nicht unterstütztes Format.
        }
    .todo = Versuche eine andere Datei dieses Mods oder melde das Problem dem Autor.
e9 = Das Archiv ist zu groß, um es sicher zu entpacken.
    .why =
        { $reason ->
            [too_large]
                { $measured ->
                    [yes] Entpackt wäre es { $unpacked } groß; die Grenze liegt bei { $limit }.
                   *[no] Entpackt ist es größer als { $limit }.
                }
            [too_many_files]
                { $found ->
                    [one] Es enthält { $found } Mod-Datei, die Grenze liegt bei { $limit }.
                   *[other] Es enthält { $found } Mod-Dateien, die Grenze liegt bei { $limit }.
                }
            [ratio] { $entry } ist verdächtig stark komprimiert — mehr als { $limit }-fach. Das ist typisch für ein schädliches Archiv.
            [memory] Zum Entpacken braucht es { $needed } Arbeitsspeicher, die Grenze liegt bei { $limit }.
           *[other] Es überschreitet eine der Sicherheitsgrenzen für Archive.
        }
    .todo =
        { $reason ->
            [memory] Versuche eine andere Datei dieses Mods.
            [ratio] Installiere es nur, wenn du der Quelle vertraust; nur dann erhöhe die Grenze unter Einstellungen → Erweitert.
           *[other] Wenn du ihm vertraust, erhöhe die Grenze unter Einstellungen → Downloads.
        }
e10 = Keine Mod-Dateien in diesem Archiv.
    .why =
        { $listing ->
            [files] Es enthält keine .vpk-Dateien, nur { $contents }.
           *[empty] Das Archiv ist leer.
        }
    .todo = Diese Datei ist kein Deadlock-Mod in einem Format, das deadloader installieren kann.
e11 = RAR-Archive lassen sich auf diesem PC nicht entpacken.
    .why = Das in Windows eingebaute Werkzeug kann dieses RAR nicht lesen, und weder 7-Zip noch WinRAR wurde gefunden.
    .todo = Installiere 7-Zip (kostenlos) und versuche es erneut; die heruntergeladene Datei bleibt erhalten.
e12 = Nicht genug Speicherplatz.
    .why =
        { $reason ->
            [space] Benötigt werden { $needed }, frei sind nur { $available }.
           *[full] Der Datenträger ist voll.
        }
    .todo = Gib Speicherplatz frei oder lösche alte Mod-Versionen in der Bibliothek — und versuche es dann erneut.
e13 = Änderungen können nicht übernommen werden, solange Deadlock läuft.
    .why = Windows sperrt die Dateien des Spiels, während es läuft.
    .todo = Schließe Deadlock; deadloader übernimmt die Änderungen direkt danach.
e14 = Eine Mod-Datei in der Bibliothek fehlt oder ist beschädigt.
    .why = Sie wurde außerhalb von deadloader geändert oder gelöscht.
    .todo = Repariere den Mod, um die Datei erneut herunterzuladen, oder entferne den Mod.
e15 = Der Link konnte nicht geöffnet werden.
    .why = In Windows ist kein Standardbrowser festgelegt.
    .todo = Kopiere den Link und öffne ihn selbst.
e16 = Anfrage blockiert.
    .why =
        { $reason ->
            [not_https] Der Server hat auf eine Adresse ohne HTTPS umgeleitet; solchen Umleitungen folgt deadloader nie.
            [bad_redirect] Der Server hat eine Umleitung gesendet, der deadloader nicht folgen kann.
            [too_many_redirects] Der Server hat zu oft umgeleitet.
           *[not_listed] { $host } steht nicht auf der Liste der erlaubten Hosts, deshalb hat deadloader keine Verbindung dorthin aufgebaut.
        }
    .todo =
        { $reason ->
            [not_listed] Du musst nichts tun — deadloader spricht nur mit GameBanana. Wenn du diesem Host vertraust, füge ihn unter Einstellungen → Erweitert hinzu.
           *[not_https] Du musst nichts tun — deadloader folgt nur sicheren Umleitungen. Versuche es später erneut.
        }
e17 = Das Profil konnte nicht importiert werden.
    .why =
        { $reason ->
            [newer] Es wurde mit einer neueren deadloader-Version erstellt.
            [unreadable] Die Datei kann nicht gelesen werden — vielleicht wurde sie verschoben, gelöscht oder von einem anderen Programm geöffnet.
           *[not_a_profile] Die Datei ist kein deadloader-Profil.
        }
    .todo =
        { $reason ->
            [newer] Aktualisiere deadloader und importiere das Profil dann erneut.
            [unreadable] Prüfe, ob die Datei noch da ist und kein anderes Programm sie geöffnet hat, und versuche es dann erneut.
           *[not_a_profile] Prüfe, ob du die richtige Datei gewählt hast.
        }
e18 = Dateien im Spielordner konnten nicht geändert werden.
    .why =
        { $reason ->
            [access_denied] Windows hat den Zugriff auf den Spielordner verweigert.
            [in_use] Eine Datei wird von einem anderen Programm verwendet, etwa von einem Virenschutz oder einem anderen Mod-Manager.
           *[other] Windows konnte die Dateien nicht ändern.
        }
    .todo = Schließe andere Programme und versuche es erneut.
e19 = Das Bild konnte nicht geladen werden.
    .why =
        { $reason ->
            [no_response] Der Host hat nicht geantwortet.
            [not_an_image] Die Datei ist kein Bild.
            [too_large] Das Bild ist zu groß.
           *[other] Der Host hat kein verwendbares Bild gesendet.
        }
    .todo = Der Platzhalter bleibt; du kannst es erneut versuchen.
e20 = Zu viele Mods sind aktiviert.
    .why =
        { $free ->
            [one] Das Spiel kann noch { $free } Mod-Datei laden, dieses Profil braucht { $wanted }.
           *[other] Das Spiel kann noch { $free } Mod-Dateien laden, dieses Profil braucht { $wanted }.
        }
    .todo = Schalte einige Mods aus oder wähle weniger Add-ons.
e21 = Ein Profil namens „{ $name }“ gibt es schon.
    .why = Namen werden ohne Unterscheidung von Groß- und Kleinschreibung verglichen, wie Dateinamen.
    .todo = Wähle einen anderen Namen.
e22 = „{ $name }“ kann nicht als Profilname verwendet werden.
    .why = Windows erlaubt ihn nicht als Dateinamen — zum Beispiel CON, einen Punkt am Ende oder eines der Zeichen \ / : * ? " < > |.
    .todo = Wähle einen anderen Namen.
e23 = deadloader kann seine Datei { $file } nicht lesen.
    .why =
        { $reason ->
            [damaged_at] Die Datei ist beschädigt (Zeile { $line }, Spalte { $column }).
            [damaged] Die Datei ist beschädigt.
            [newer] Sie wurde mit einer neueren deadloader-Version erstellt.
           *[invalid] Ihr Inhalt entspricht nicht dem, was deadloader erwartet.
        }
    .todo =
        { $kind ->
            [settings] deadloader verwendet vorerst die Standardeinstellungen und lässt die Datei unverändert.
            [manifest] Stelle manifest.json.bak wieder her oder aktualisiere deadloader.
           *[other] Stelle die Datei aus einer Sicherung wieder her oder aktualisiere deadloader.
        }
e24 = Die Datei ist nicht mehr auf GameBanana.
    .why = Der Autor hat sie entfernt oder ersetzt (HTTP { $status }).
    .todo = Öffne die Mod-Seite und wähle eine andere Datei, oder suche nach Updates für die Mod.
e25 = GameBanana hat die Anfrage abgelehnt.
    .why = Der Server hat mit Code { $status } geantwortet. Das ist meist nur vorübergehend.
    .todo = Versuche es später erneut. Wenn es weiter passiert, melde es auf der GitHub-Seite von deadloader.
internal = In deadloader ist etwas schiefgelaufen.
    .why = Das ist ein Fehler von deadloader, nicht von dir.
    .todo = Versuche es erneut. Wenn es wieder passiert, melde es mit dem Protokolleintrag aus den Details.

## Buttons named in the texts above

action-undo = Rückgängig
action-review = Ansehen
action-retry = Erneut versuchen
action-retry-now = Jetzt erneut versuchen
action-browse = Durchsuchen…
action-verify-in-steam = In Steam überprüfen
action-show-differences = Unterschiede anzeigen
action-details = Details
action-repair = Reparieren
action-copy-link = Link kopieren
action-open-folder = Ordner öffnen
action-open-release-page = Release-Seite öffnen
action-enable-catalog = Katalog einschalten
action-learn-more = Mehr erfahren
action-view-log = Protokoll anzeigen

## Shared pieces

log-entry = Protokolleintrag { $id }
list-more = { $count } weitere

## Sizes, labelled like Windows Explorer labels them

size-bytes =
    { $count ->
        [one] { $count } Byte
       *[other] { $count } Bytes
    }
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
