# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Cerrar
library-apply = Aplicar
library-apply-profile = Aplicar { $name }
library-discard-all = Descartar todo
library-keep = Mantener { $name }
library-keep-applied = Mantener los cambios aplicados
library-undo-this = Deshacer este cambio
library-undo-all = Deshacer todo
library-close-to-apply = Cierra Deadlock: los cambios se aplicarán automáticamente
library-play-applies = Jugar también lo aplica
library-applied-title = Cambios aplicados
library-nothing-pending = Nada pendiente
library-all-in-game = Todo el perfil { $name } ya está en el juego.
library-running-sub = Deadlock se está ejecutando, así que Windows bloquea sus archivos. Estos cambios se aplicarán automáticamente en cuanto cierres el juego.
library-waiting-sub = Estos cambios aún no están en la carpeta del juego. Aplícalos ahora o presiona Jugar.
library-switch-sub = Un cambio de perfil modifica muchos archivos a la vez, así que te espera. Aplícalo aquí o presiona Jugar: deadloader lo aplica antes de iniciar.
library-applied-sub =
    { $count ->
        [one] Deadlock se cerró a las { $time }, así que se aplicó el cambio que hiciste mientras se ejecutaba. Puedes deshacerlo hasta tu próximo cambio.
       *[other] Deadlock se cerró a las { $time }, así que se aplicaron los { $count } cambios que hiciste mientras se ejecutaba. Puedes deshacerlos hasta tu próximo cambio.
    }
library-undoing-sub =
    { $count ->
        [one] Deshiciste el cambio aplicado a las { $time } y Deadlock vuelve a ejecutarse. Se revertirá automáticamente cuando cierres el juego.
       *[other] Deshiciste los { $count } cambios aplicados a las { $time } y Deadlock vuelve a ejecutarse. Se revertirán automáticamente cuando cierres el juego.
    }
library-group-enable = Activar · { $count }
library-group-disable = Desactivar · { $count }
library-group-order = Orden · { $count }
library-group-variant = Variante · { $count }
library-group-version = Versión · { $count }
library-group-switch = Cambio de perfil · { $count }
library-group-undo =
    { $count ->
        [one] Deshacer el cambio aplicado a las { $time }
       *[other] Deshacer los { $count } cambios aplicados a las { $time }
    }
library-change-on = Desactivado → activado
library-change-off = Activado → desactivado
library-change-above = Movido encima de { $name }
library-change-above-wins = Movido encima de { $name }: ahora gana su conflicto
library-change-below = Movido debajo de { $name }
library-switch-what-happens =
    { $on ->
        [one] Se activa 1 mod
       *[other] Se activan { $on } mods
    } · { $off ->
        [one] se desactiva 1
       *[other] se desactivan { $off }
    }
library-what-happens = Qué pasa en la carpeta del juego
library-folder-summary =
    { $added ->
        [one] 1 archivo agregado
       *[other] { $added } archivos agregados
    } · { $removed } quitados · { $renumbered } renumerados

## Foreign files
library-foreign-title = Encontrado en la carpeta de mods del juego
library-foreign-note = deadloader no los puso aquí y no los tocará a menos que tú lo decidas.
library-collapse = Contraer
library-expand = Expandir
library-take = Gestionar con deadloader
library-leave = Dejar como está
library-left = Dejado como está
library-foreign-open = Abrir la carpeta que contiene { $name }
library-foreign-vpk =
    { $count ->
        [one] VPK, { $count } archivo
       *[other] VPK, { $count } archivos
    }
library-foreign-vpk-hero =
    { $count ->
        [one] VPK, { $count } archivo: parece un aspecto para { $hero }
       *[other] VPK, { $count } archivos: parece un aspecto para { $hero }
    }
library-foreign-unreadable = Un archivo de mod que deadloader no puede leer
library-foreign-replaced = Un archivo que ocupó el lugar de uno de deadloader
library-foreign-dmm = Creado por Deadlock Mod Manager: un archivo de ajustes, no un mod
library-foreign-other = Un archivo, no un mod
library-foreign-folder =
    { $count ->
        [one] Carpeta, { $count } archivo: no es un mod
       *[other] Carpeta, { $count } archivos: no es un mod
    }
library-foreign-meta = { $size } · agregado el { $date }
library-taken = { $name } ya está en tu biblioteca.

## D11
library-d11-title = ¿Gestionar { $name } con deadloader?
library-d11-lead = deadloader:
library-d11-step-copy = lo copiará a tu biblioteca como mod local ({ $size }),
library-d11-step-replace = sustituirá el archivo de la carpeta del juego por su propia copia enlazada,
library-d11-step-enable = lo activará en el perfil { $profile }, al final de la lista.
library-d11-name = Nombre en tu biblioteca
library-d11-hint = La biblioteca guarda una copia exacta del archivo original.
library-d11-suggested = { $file } (aspecto de { $hero })

## D10
library-d10-title = Otro programa cambió un archivo del juego
library-d10-what = Otro programa cambió un archivo del juego que controla qué carpetas carga Deadlock; suele ser Deadlock Mod Manager. deadloader no sobrescribe los cambios de otros, así que tus mods no se cargarán hasta que se arregle.
library-d10-do = Deja que Steam restaure el archivo original (Verificar archivos del juego). Después deadloader agrega su propia línea y te avisa.
library-d10-diff = Mostrar diferencias
library-d10-expected = Esperado (original + línea de deadloader)
library-d10-found = Encontrado ahora
library-d10-diff-note = Resaltadas: la línea que agrega deadloader y las líneas que agregó otro programa.
library-d10-help = ¿Qué es este archivo?
library-d10-help-text = gameinfo.gi le dice a Deadlock qué carpetas cargar. Los gestores de mods agregan una línea para que el juego cargue mods de la carpeta addons. Las actualizaciones del juego la restablecen; deadloader lo corrige automáticamente, pero solo si el archivo es el original.
library-d10-wait-title = Steam está verificando los archivos de Deadlock.
library-d10-wait-text = Cuando Steam termine, vuelve a deadloader: comprueba el archivo de nuevo automáticamente. También puedes comprobarlo ahora.
library-d10-fixed = gameinfo.gi está configurado: los mods vuelven a cargarse.
library-d10-fixed-text = Steam restauró el archivo original y deadloader agregó su línea. Detalles en Actividad → Archivos.
library-open-file = Abrir archivo
library-not-now = Ahora no
library-verify-steam = Verificar archivos del juego en Steam
library-check-again = Comprobar de nuevo
library-show-differences = Mostrar diferencias

## D5
library-update = Actualizar
library-update-on-gb = Hay una versión más reciente en GameBanana.
library-later = Más tarde
library-d5-title = ¿Actualizar { $name }?
library-d5-installed = Instalada
library-d5-new = Nueva en GameBanana
library-d5-downloaded = Descargada
library-d5-arrived = En la biblioteca, aún no activa
library-d5-new-meta = Actualizada el { $date } · { $size } de descarga
library-d5-download = { $size } de descarga
library-d5-changed = La nueva versión tiene otro conjunto de archivos. deadloader los emparejó así:
library-d5-same = Estos archivos se sustituyen por sus nuevas versiones:
library-d5-map-text = deadloader no pudo saber qué archivo nuevo sustituye a cada uno. Elige el archivo nuevo para cada uno o mantén el antiguo.
library-d5-map-label = Emparejamiento de archivos
library-d5-col-old = Archivo instalado
library-d5-col-new = Archivo nuevo
library-d5-pick = Archivo nuevo para { $file }
library-d5-choose = Elegir…
library-d5-keep = Mantener el archivo antiguo
library-d5-tag-replaces = Sustituye
library-d5-tag-same = Mismo nombre
library-d5-tag-new = Nuevo · en la página del mod
library-d5-profiles = { $new } se activa en todos los perfiles que usan { $old }. { $old } se queda en la biblioteca, así que puedes volver a ella en los detalles del mod.
library-d5-install = Instalar actualización
library-d5-confirm-map = Usar estos archivos
library-update-downloading = Descargando la actualización de { $name }: se instalará cuando llegue.
library-updated = { $name } actualizado.
library-updated-text = Activo en todos los perfiles que usaban la versión antigua. La versión antigua se conserva: puedes volver a ella en los detalles del mod.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name } activado
library-n12-off = { $name } desactivado
library-n12-above = { $name } movido encima de { $other }
library-n12-below = { $name } movido debajo de { $other }
library-n12-switched = { $name } cambiado a { $to }
library-n12-files = { $name }: archivos cambiados

## S9: the file list of a pending apply
library-show-file-list = Mostrar la lista de archivos
library-hide-file-list = Ocultar la lista de archivos
library-file-list = Archivos en la carpeta del juego
library-files-added = Agregados · { $count }
library-files-removed = Quitados · { $count }
library-files-renumbered = Renumerados · { $count }
