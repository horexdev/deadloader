# The Library (S4): header, states, list, mod details, conflicts (C1), deleting (D6).
library-title = Biblioteca
library-profile = Perfil
library-enabled-count = { $enabled } de { $total } activados
library-disk = { $size } en disco
library-not-checked = Aún no se han buscado actualizaciones
library-checked-today = Actualizaciones buscadas hoy a las { $time }
library-checked-on = Actualizaciones buscadas el { $date }
library-game-not-found = Juego no encontrado
library-add-disk = Añadir desde el disco
library-add-title = Añadir mods desde el disco
library-add-filter = Archivos de mods
library-added =
    { $count ->
        [one] { $count } mod añadido a la biblioteca.
       *[other] { $count } mods añadidos a la biblioteca.
    }
library-check = Buscar actualizaciones
library-checking = Buscando…
library-update-all = Actualizar todo
library-review = Revisar
library-undo = Deshacer
library-cancel = Cancelar
library-missing-what = No se ha encontrado Deadlock.
library-missing-why = Steam o el juego no están instalados, o la carpeta del juego se ha movido. Tu biblioteca sigue funcionando; aplicar cambios y Jugar están desactivados hasta encontrar el juego.
library-choose-folder = Elegir la carpeta del juego
library-search-again = Buscar de nuevo
library-running-what = Deadlock se está ejecutando.
library-running-why = Los cambios se aplicarán automáticamente cuando cierres el juego.
library-pending = Cambios pendientes
library-switch-what = El perfil { $name } aún no está en el juego.
library-switch-why = El juego todavía tiene { $applied }. Aplica { $name } ahora o pulsa Jugar: deadloader lo aplica antes de iniciar.
library-switch-why-first = Aplica { $name } ahora o pulsa Jugar: deadloader lo aplica antes de iniciar.
library-review-switch = Revisar el cambio
library-applying-what = Aplicando el perfil { $name }…
library-applying-why = Puedes seguir navegando; Jugar vuelve cuando termine.
library-show = Mostrar
library-tab-all = Todos
library-tab-enabled = Activados
library-tab-disabled = Desactivados
library-tab-updates = Actualizaciones
library-tab-conflicts = Conflictos
library-tab-local = Locales
library-search = Buscar en la biblioteca
library-search-clear = Borrar búsqueda
library-higher-wins = Cuando dos mods cambian el mismo archivo, gana el que está más arriba en la lista.
library-reorder-hint = Reordenar: arrastra el asa, o enfócala y pulsa Espacio
library-list-label = Mods del perfil { $name }, por prioridad
library-toggle = { $name } activado
library-toggle-missing = No se ha encontrado Deadlock: elige la carpeta del juego para aplicar cambios
library-grip = Mover { $name }, posición { $position } de { $total }
library-more = Más acciones para { $name }
library-show-variants = Mostrar variantes
library-hide-variants = Ocultar variantes
library-show-addons = Mostrar complementos
library-hide-addons = Ocultar complementos
library-live-picked = { $name }: recogido, posición { $position } de { $total }. ↑ ↓ para mover, Espacio para soltar, Esc para cancelar.
library-live-moved = { $name }: posición { $position } de { $total }.
library-live-moved-grabbed = { $name }: posición { $position } de { $total }. Espacio para soltar, Esc para cancelar.
library-live-dropped = { $name }: soltado en la posición { $position } de { $total }.
library-live-back = { $name }: de vuelta en la posición { $position } de { $total }.
library-live-drag = { $name }: suelta para colocarlo encima de { $above }, en la posición { $position } de { $total }.
library-badge-pending = Pendiente
library-badge-damaged = Archivos dañados
library-badge-update = Actualización disponible
library-badge-wins = Conflicto · gana
library-badge-loses = Conflicto · pierde
library-badge-variants =
    { $count ->
        [one] { $count } variante
       *[other] { $count } variantes
    }
library-badge-local = Local
library-badge-copied = Copiado · +{ $size }
library-local = Local
library-unknown-author = Autor desconocido
library-gamebanana = GameBanana
library-from-disk = desde el disco
library-addons-sub =
    { $count ->
        [one] + { $count } complemento
       *[other] + { $count } complementos
    }
library-variants = Variantes
library-variants-downloaded = { $count } descargadas
library-active = Activa
library-select-to-switch = Selecciona para cambiar
library-parts = Archivos de este mod
library-main-files = Archivos principales
library-from = de { $archive }
library-on-with-mod = Se activa con el mod · { $size }
library-remove-addon = Quitar complemento
library-toast-enabled = { $name } activado.
library-toast-disabled = { $name } desactivado.
library-toast-variant = { $name } ha cambiado a { $variant }.
library-empty-title = Tu biblioteca está vacía
library-empty-text = Instala mods desde el catálogo o añade archivos .vpk, .zip, .7z o .rar desde tu ordenador. También puedes soltar archivos en cualquier parte de esta ventana.
library-empty-local-text = En el modo local el catálogo está desactivado. Añade archivos .vpk, .zip, .7z o .rar desde tu ordenador, o suéltalos en cualquier parte de esta ventana.
library-browse = Explorar el catálogo
library-no-match-title = No hay mods aquí
library-no-match-text = Ningún mod coincide con esta pestaña y esta búsqueda. Prueba otra pestaña o borra la búsqueda.

## Mod details
library-details = Detalles del mod
library-open-mod-page = Abrir la página del mod
library-close-details = Cerrar detalles
library-enabled-in = Activado en { $profile }
library-position = Posición { $position } de { $total }
library-version = Versión
library-no-version = Sin versión indicada
library-from-gamebanana = De GameBanana
library-imported = Importado desde el disco
library-newer-in-library = Hay una versión más reciente en la biblioteca.
library-switch-to = Cambiar a
library-delete-old = Eliminar versiones antiguas
library-no-older = No se guardan versiones antiguas
library-older =
    { $count ->
        [one] { $count } versión antigua guardada · { $size }
       *[other] { $count } versiones antiguas guardadas · { $size }
    }
library-files = Archivos
library-on-disk = { $size } en disco
library-linked = Enlazado: sin espacio extra en disco
library-copied = Copiado: { $size } de espacio extra en disco
library-not-in-game = No está en la carpeta del juego
library-local-mod = Mod local
library-name = Nombre en tu biblioteca
library-rename = Cambiar nombre
library-link-label = Página de GameBanana
library-link-hint = Pega la dirección del mod para buscar sus actualizaciones.
library-link = Vincular
library-link-done = Vinculado: { $matched } de { $total } archivos encontrados en la página.
library-link-not-a-page = No es una página de mod de GameBanana. Pega una dirección como gamebanana.com/mods/123456.
library-link-removed = Este mod ya no está en GameBanana.
library-link-failed = No se pudo leer la página. Revisa la dirección e inténtalo de nuevo.
library-imported-from = Importado desde
library-no-conflicts = Sin conflictos con otros mods activados
library-conflict-wins =
    { $count ->
        [one] Gana a { $name } · { $count } archivo
       *[other] Gana a { $name } · { $count } archivos
    }
library-conflict-loses =
    { $count ->
        [one] Pierde contra { $name } · { $count } archivo
       *[other] Pierde contra { $name } · { $count } archivos
    }
library-show-conflicts = Mostrar conflictos
library-delete-mod = Eliminar mod…
library-open-folder = Abrir carpeta

## Conflicts (C1)
library-conflicts-sub = Dos mods activados cambian los mismos archivos del juego. Gana el que está más arriba en la lista.
library-close-conflicts = Cerrar conflictos
library-no-conflicts-title = Sin conflictos
library-wins = Gana
library-loses = Pierde
library-off = Desactivado
library-in-use = En uso
library-conflict-text =
    { $count ->
        [one] Ambos cambian { $count } archivo. En el juego ves la versión de { $winner }; la copia de { $loser } no se usa.
       *[other] Ambos cambian { $count } archivos. En el juego ves las versiones de { $winner }; las copias de { $loser } no se usan.
    }
library-conflict-off =
    { $count ->
        [one] Ambos cambian { $count } archivo. { $off } está desactivado, así que el juego usa la versión de { $other }.
       *[other] Ambos cambian { $count } archivos. { $off } está desactivado, así que el juego usa las versiones de { $other }.
    }
library-files-count =
    { $count ->
        [one] { $count } archivo
       *[other] { $count } archivos
    }
library-show-more = Mostrar { $count } más
library-move-above = Subir { $name } por encima
library-disable = Desactivar { $name }
library-pick-variant = Elegir otra variante
library-no-variants = Ninguno de los dos mods tiene variantes
library-done-wins = Hecho: ahora gana { $name }
library-done-off = Hecho: { $off } está desactivado; se usan los archivos de { $other }
library-identical = Los archivos idénticos (misma suma de comprobación) no son conflictos y no se muestran.

## D6 Delete mod
library-d6-title = ¿Eliminar «{ $name }»?
library-d6-files-title = ¿Eliminar las versiones antiguas de «{ $name }»?
library-d6-what =
    { $count ->
        [one] { $count } archivo · { $size }.
       *[other] { $count } archivos · { $size }.
    }
library-d6-profiles =
    { $count ->
        [one] Se usa en { $count } perfil: { $names }. Se quitará de él y de la carpeta del juego.
       *[other] Se usa en { $count } perfiles: { $names }. Se quitará de ellos y de la carpeta del juego.
    }
library-d6-no-profiles = Ningún perfil lo usa.
library-d6-where = Eliminar
library-d6-permanently = Definitivamente
library-d6-recycle-bin = A la papelera de reciclaje
library-d6-permanent-hint = Los archivos se eliminan para siempre: no se puede deshacer.
library-d6-bin-hint = Podrás recuperarlo desde la papelera de reciclaje de Windows.
library-d6-confirm = Eliminar mod
library-d6-confirm-files = Eliminar archivos
library-deleted = { $name } eliminado.
library-deleted-files =
    { $count ->
        [one] { $count } archivo de { $name } eliminado.
       *[other] { $count } archivos de { $name } eliminados.
    }
