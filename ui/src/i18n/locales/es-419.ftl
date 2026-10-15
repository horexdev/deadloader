## Notifications

n1 = Deadlock se actualizó y se restauraron los mods.
    .why = La actualización restableció gameinfo.gi, así que deadloader volvió a agregar su línea.
n2 = { $name } instalado y activado.
n3 =
    { $count ->
        [one] Hay { $count } actualización de mod disponible.
       *[other] Hay { $count } actualizaciones de mods disponibles.
    }
n4 = Deadlock se está ejecutando.
    .why = Los cambios se aplicarán automáticamente cuando cierres el juego.
    .pending =
        { $count ->
            [one] { $count } cambio pendiente
           *[other] { $count } cambios pendientes
        }
n5 =
    { $reason ->
        [completed] Se completó un cambio que se había interrumpido.
       *[undone] Se deshizo un cambio interrumpido; la carpeta del juego quedó como antes.
    }
n6 =
    { $count ->
        [one] Se encontró { $count } archivo en la carpeta de mods del juego que deadloader no creó.
       *[other] Se encontraron { $count } archivos en la carpeta de mods del juego que deadloader no creó.
    }
n7 =
    { $when ->
        [today] Sin conexión: se muestra el catálogo guardado a las { $saved }.
       *[earlier] Sin conexión: se muestra el catálogo guardado el { $saved }.
    }
    .why = Las descargas se reanudarán cuando vuelvas a estar en línea.
n8 =
    { $count ->
        [one] Perfil “{ $name }” aplicado: { $count } mod activado.
       *[other] Perfil “{ $name }” aplicado: { $count } mods activados.
    }
n9 = Hay una nueva versión de deadloader disponible: { $version }.
n10 = El modo local está activado.
    .why = El catálogo está desactivado y deadloader no hace ninguna solicitud de red.
n11 = Los mods se copian en lugar de vincularse porque deadloader y el juego están en unidades distintas.
    .why = Esto ocupa { $size } de espacio adicional en el disco.
n12 =
    { $count ->
        [one] Deadlock se cerró: se aplicó { $count } cambio.
       *[other] Deadlock se cerró: se aplicaron { $count } cambios.
    }

## Errors: what happened, why, what to do

e1 = deadloader no puede guardar sus datos.
    .why =
        { $reason ->
            [access_denied] La carpeta de deadloader.exe es de solo lectura; por ejemplo, Windows no permite que los programas escriban en “Archivos de programa” (Program Files).
            [in_use] Otro programa, a menudo un antivirus, tiene abierto uno de los archivos de deadloader.
           *[other] Windows no pudo escribir en la carpeta de deadloader.exe.
        }
    .todo =
        { $reason ->
            [in_use] Cierra los demás programas y vuelve a intentarlo.
           *[other] Mueve deadloader a una carpeta en la que puedas escribir, por ejemplo D:\Games\deadloader.
        }
e2 = No se encontró Deadlock.
    .why = Steam o el juego no están instalados, o la carpeta del juego se movió.
    .todo = Elige la carpeta del juego manualmente.
e3 = Otro programa modificó un archivo del juego.
    .why = gameinfo.gi no parece el original; a menudo lo modifica Deadlock Mod Manager. deadloader no sobrescribe los cambios de otros.
    .todo = Verifica la integridad de los archivos del juego en Steam; después, deadloader agregará su línea.
e4 = El catálogo no está disponible.
    .why = GameBanana cambió su API y deadloader no puede leerla. Tu biblioteca y tus perfiles funcionan con normalidad.
    .todo = Verifica si hay una actualización de deadloader o vuelve a intentarlo más tarde.
e5 = GameBanana está saturado.
    .why =
        { $reason ->
            [rate_limited] Demasiadas solicitudes en poco tiempo (HTTP 429).
            [server] El servidor de GameBanana informó un error.
           *[other] Demasiadas solicitudes o un error del servidor.
        }
    .todo =
        { $wait ->
            [seconds]
                { $retry_in_s ->
                    [one] deadloader volverá a intentarlo automáticamente en { $retry_in_s } segundo.
                   *[other] deadloader volverá a intentarlo automáticamente en { $retry_in_s } segundos.
                }
           *[later] deadloader volverá a intentarlo automáticamente en un momento.
        }
e6 = Falló la descarga.
    .why =
        { $reason ->
            [local_mode] deadloader está en modo local, así que no hace solicitudes de red.
            [dns] No se encontró la dirección del servidor de descarga.
            [connect] deadloader no pudo conectarse al servidor de descarga.
            [tls] No se pudo establecer una conexión segura con el servidor de descarga.
            [timeout] El servidor tardó demasiado en responder.
            [connection_lost] Se perdió la conexión.
            [http_status] El servidor respondió con un error (HTTP { $status }).
           *[other] La descarga se interrumpió.
        }
    .todo =
        { $reason ->
            [local_mode] Activa el catálogo en Configuración para descargar mods.
           *[other] Se reanudará desde donde se detuvo.
        }
e7 = El archivo descargado está dañado.
    .why =
        { $reason ->
            [size] Su tamaño no coincide con el que indica GameBanana.
           *[checksum] Su suma de verificación no coincide con la que indica GameBanana.
        }
    .todo = Descárgalo de nuevo.
e8 = No se puede descomprimir el archivo comprimido.
    .why =
        { $reason ->
            [not_an_archive] No es un archivo comprimido ZIP, 7z ni RAR; es posible que la descarga esté incompleta.
            [damaged] El archivo comprimido está dañado.
            [encrypted] El archivo comprimido está protegido con contraseña.
            [unsupported] Usa un método de compresión que deadloader no admite.
            [not_a_vpk] { $entry } no es un archivo .vpk válido.
           *[other] Está dañado o usa un formato no compatible.
        }
    .todo = Prueba con otro archivo de este mod o repórtale el problema a su autor.
e9 = El archivo comprimido es demasiado grande para descomprimirlo de forma segura.
    .why =
        { $reason ->
            [too_large] Descomprimido ocupa más de { $limit }.
            [too_many_files]
                { $found ->
                    [one] Contiene { $found } archivo de mod y el límite es { $limit }.
                   *[other] Contiene { $found } archivos de mod y el límite es { $limit }.
                }
            [ratio] { $entry } está comprimido de forma sospechosa: más de { $limit } veces.
            [memory] Para descomprimirlo se necesitan { $needed } de memoria y el límite es { $limit }.
           *[other] Supera uno de los límites de seguridad para archivos comprimidos.
        }
    .todo =
        { $reason ->
            [memory] Prueba con otro archivo de este mod.
           *[other] Si confías en él, aumenta el límite en Configuración → Descargas.
        }
e10 = No hay archivos de mod en este archivo comprimido.
    .why =
        { $listing ->
            [files] No contiene archivos .vpk, solo { $contents }.
           *[empty] El archivo comprimido está vacío.
        }
    .todo = Este archivo no es un mod de Deadlock en un formato que deadloader pueda instalar.
e11 = No se pueden descomprimir archivos RAR en esta PC.
    .why = La herramienta integrada de Windows no puede leer este RAR, y no se encontró ni 7-Zip ni WinRAR.
    .todo = Instala 7-Zip (es gratis) y vuelve a intentarlo; el archivo descargado se conserva.
e12 = No hay suficiente espacio en disco.
    .why =
        { $reason ->
            [space] Se necesitan { $needed } y solo hay { $available } libres.
           *[full] El disco está lleno.
        }
    .todo = Libera espacio o elimina versiones anteriores de mods en la Biblioteca y vuelve a intentarlo.
e13 = No se pueden aplicar los cambios mientras Deadlock se está ejecutando.
    .why = Windows bloquea los archivos del juego mientras se ejecuta.
    .todo = Cierra Deadlock; deadloader aplicará los cambios justo después.
e14 = Falta un archivo de mod en la biblioteca o está dañado.
    .why = Se modificó o eliminó fuera de deadloader.
    .todo = Repara el mod para volver a descargar el archivo, o quita el mod.
e15 = No se pudo abrir el enlace.
    .why = No hay ningún navegador predeterminado configurado en Windows.
    .todo = Copia el enlace y ábrelo tú mismo.
e16 = Solicitud bloqueada.
    .why =
        { $reason ->
            [not_https] El servidor redirigió a una dirección sin HTTPS, y deadloader nunca sigue esas redirecciones.
            [bad_redirect] El servidor envió una redirección que deadloader no puede seguir.
            [too_many_redirects] El servidor redirigió demasiadas veces.
           *[not_listed] { $host } no está en la lista de hosts permitidos, así que deadloader no se conectó a él.
        }
    .todo =
        { $reason ->
            [not_listed] No tienes que hacer nada: deadloader solo se comunica con GameBanana. Si confías en este host, agrégalo en Configuración → Avanzado.
           *[not_https] No tienes que hacer nada: deadloader solo sigue redirecciones seguras. Vuelve a intentarlo más tarde.
        }
e17 = No se pudo importar el perfil.
    .why =
        { $reason ->
            [newer] Se creó con una versión más reciente de deadloader.
            [unreadable] No se puede leer el archivo; es posible que se haya movido, eliminado o que otro programa lo tenga abierto.
           *[not_a_profile] El archivo no es un perfil de deadloader.
        }
    .todo =
        { $reason ->
            [newer] Actualiza deadloader y vuelve a importar el perfil.
            [unreadable] Verifica que el archivo siga ahí y que ningún otro programa lo tenga abierto, y vuelve a intentarlo.
           *[not_a_profile] Verifica que elegiste el archivo correcto.
        }
e18 = No se pudieron cambiar los archivos de la carpeta del juego.
    .why =
        { $reason ->
            [access_denied] Windows denegó el acceso a la carpeta del juego.
            [in_use] Otro programa está usando un archivo, como un antivirus u otro administrador de mods.
           *[other] Windows no pudo cambiar los archivos.
        }
    .todo = Cierra los demás programas y vuelve a intentarlo.
e19 = No se pudo cargar la imagen.
    .why =
        { $reason ->
            [no_response] El host no respondió.
            [not_an_image] El archivo no es una imagen.
            [too_large] La imagen es demasiado grande.
           *[other] El host no envió una imagen válida.
        }
    .todo = El marcador de posición se mantiene; puedes volver a intentarlo.
e20 = Hay demasiados mods activados.
    .why =
        { $free ->
            [one] El juego puede cargar { $free } archivo de mod más, y este perfil necesita { $wanted }.
           *[other] El juego puede cargar { $free } archivos de mod más, y este perfil necesita { $wanted }.
        }
    .todo = Desactiva algunos mods o elige menos complementos.
e21 = Ya existe un perfil llamado “{ $name }”.
    .why = Los nombres se comparan sin distinguir mayúsculas de minúsculas, como los nombres de archivo.
    .todo = Elige otro nombre.
e22 = “{ $name }” no se puede usar como nombre de perfil.
    .why = Windows no lo admite como nombre de archivo; por ejemplo, CON, un punto al final o cualquiera de los caracteres \ / : * ? " < > |.
    .todo = Elige otro nombre.
e23 = deadloader no puede leer su archivo { $file }.
    .why =
        { $reason ->
            [damaged_at] El archivo está dañado (línea { $line }, columna { $column }).
            [damaged] El archivo está dañado.
            [newer] Se creó con una versión más reciente de deadloader.
           *[invalid] Su contenido no es el que espera deadloader.
        }
    .todo =
        { $kind ->
            [settings] Por ahora, deadloader usa la configuración predeterminada y deja el archivo como está.
            [manifest] Restaura manifest.json.bak o actualiza deadloader.
           *[other] Restaura el archivo desde una copia de seguridad o actualiza deadloader.
        }
internal = Algo falló dentro de deadloader.
    .why = Es un error de deadloader, no algo que hiciste tú.
    .todo = Vuelve a intentarlo. Si vuelve a ocurrir, repórtalo junto con la entrada del registro de Detalles.

## Buttons named in the texts above

action-undo = Deshacer
action-review = Revisar
action-retry = Reintentar
action-retry-now = Reintentar ahora
action-browse = Examinar…
action-verify-in-steam = Verificar en Steam
action-show-differences = Mostrar diferencias
action-details = Detalles
action-repair = Reparar
action-copy-link = Copiar enlace
action-open-folder = Abrir carpeta
action-open-release-page = Abrir la página de la versión
action-enable-catalog = Activar catálogo
action-learn-more = Más información
action-view-log = Ver registro

## Shared pieces

log-entry = Entrada del registro { $id }
list-more = { $count } más

## Sizes, labelled like Windows Explorer labels them

size-bytes =
    { $count ->
        [one] { $count } byte
       *[other] { $count } bytes
    }
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
