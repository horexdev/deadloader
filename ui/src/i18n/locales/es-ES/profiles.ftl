# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Perfiles
profiles-count =
    { $count ->
        [one] { $count } perfil
       *[other] { $count } perfiles
    }
profiles-in-game = En el juego ahora: { $list }
profiles-in-game-item = «{ $name }» en { $drive }
profiles-in-game-none = Ahora no hay ningún perfil en el juego.
profiles-import = Importar…
profiles-import-title = Importar un perfil desde un archivo .json
profiles-import-dialog = Importar un perfil
profiles-export-dialog = Exportar el perfil
profiles-file-filter = Perfiles de deadloader
profiles-new = Nuevo perfil
profiles-selected = Seleccionado
profiles-applied = Aplicado
profiles-not-applied = Aún no aplicado
profiles-selected-title = El perfil que editas en la biblioteca y con el que juegas
profiles-pending-title = Aplicar o Jugar lo pone en la carpeta del juego
profiles-applied-title = Sus archivos están ahora en la carpeta del juego en { $drive }
profiles-select = Seleccionar
profiles-select-of = Seleccionar «{ $name }»
profiles-more-of = Más acciones para «{ $name }»
profiles-rename = Cambiar nombre
profiles-duplicate = Duplicar
profiles-export = Exportar como archivo…
profiles-change-installation = Cambiar la instalación del juego…
profiles-delete = Eliminar
profiles-delete-confirm = Eliminar…
profiles-delete-last = deadloader necesita al menos un perfil.
profiles-name-label = Nombre del perfil
profiles-save = Guardar
profiles-cancel = Cancelar
profiles-close = Cerrar
profiles-close-esc = Cerrar (Esc)
profiles-rename-hint = Intro guarda · Esc cancela
profiles-name-empty = Escribe un nombre para el perfil.
profiles-mods =
    { $count ->
        [one] { $count } mod activado
       *[other] { $count } mods activados
    }
profiles-mods-none = Ningún mod activado
profiles-applied-when = Aplicado { $when }
profiles-last-applied = Aplicado por última vez { $when }
profiles-never-applied = Nunca aplicado
profiles-just-now = ahora mismo
profiles-preview-empty = Selecciona este perfil y luego activa mods en la biblioteca.
profiles-footnote = Un perfil es una lista de mods activados con su orden, variantes y versiones. Seleccionarlo no descarga ni borra nada; Aplicar solo cambia los archivos que difieren.
profiles-copy-name = { $name } (copia)
profiles-copy-name-n = { $name } (copia { $number })
profiles-new-title = Nuevo perfil
profiles-start-with = Empezar con
profiles-empty = Vacío
profiles-empty-hint = Ningún mod activado: elígelos después en la biblioteca.
profiles-copy-of = Copia de «{ $name }»
profiles-copy-hint = { $mods }: mismo orden, variantes y versiones.
profiles-installation = Instalación del juego
profiles-installation-hint = La misma que «{ $name }». Puedes cambiarla más tarde desde el menú del perfil.
profiles-new-note = Crear un perfil no lo selecciona ni lo aplica: el juego no cambia.
profiles-create = Crear perfil
profiles-pending-what = El perfil «{ $name }» aún no está en el juego.
profiles-pending-why-other = La carpeta del juego en { $drive } todavía tiene «{ $other }». Aplícalo ahora o pulsa Jugar: deadloader lo aplica antes de iniciar.
profiles-pending-why-empty = La carpeta del juego en { $drive } no tiene ahora mods de deadloader. Aplícalo ahora o pulsa Jugar: deadloader lo aplica antes de iniciar.
profiles-keep = Conservar «{ $name }»
profiles-apply = Aplicar «{ $name }»
profiles-other-what = «{ $name }» usa otra instalación del juego.
profiles-other-why = Aplicar y Jugar usan { $path }. El juego en { $drive } conserva «{ $other }».
profiles-other-why-empty = Aplicar y Jugar usan { $path }. El juego en { $drive } no cambia.
profiles-delete-title = ¿Eliminar el perfil «{ $name }»?
profiles-delete-applied =
    { $count ->
        [one] «{ $name }» está ahora en el juego. deadloader quitará su { $count } mod de la carpeta del juego:
       *[other] «{ $name }» está ahora en el juego. deadloader quitará sus { $count } mods de la carpeta del juego:
    }
profiles-delete-applied-empty = «{ $name }» está ahora en el juego pero no tiene mods, así que la carpeta del juego no cambia.
profiles-delete-stay = Los mods se quedan en tu biblioteca y en otros perfiles. Hasta que apliques otro perfil allí, Deadlock se inicia sin mods de deadloader.
profiles-delete-next = «{ $name }» pasa a ser el perfil seleccionado. No se aplica hasta que pulses Aplicar o Jugar.
profiles-delete-undoable = Esto no se puede deshacer. Para conservar una copia, exporta antes el perfil.
profiles-export-first = Exportar primero
profiles-delete-profile = Eliminar perfil
profiles-installation-title = Instalación del juego para «{ $name }»
profiles-installation-lead = Elige la carpeta de Deadlock a la que se aplica este perfil.
profiles-used-by =
    { $count ->
        [one] usada por { $count } perfil
       *[other] usada por { $count } perfiles
    }
profiles-used-by-none = no la usa ningún perfil
profiles-installation-applied = «{ $name }» está ahora en el juego en { $drive }. Sus archivos se quedan allí hasta que apliques otro perfil en { $drive }.
profiles-installation-not-applied = Ninguna carpeta del juego cambia hasta que apliques «{ $name }».
profiles-manage-installations = Gestionar las instalaciones en Configuración
profiles-use-installation = Usar esta instalación
profiles-d9-title =
    { $count ->
        [one] { $count } mod de «{ $name }» no está en tu biblioteca
       *[other] { $count } mods de «{ $name }» no están en tu biblioteca
    }
profiles-d9-file = El archivo
profiles-d9-lead = incluye { $listed } mods. { $present } ya están en tu biblioteca; estos { $count } no:
profiles-d9-list = Mods que faltan
profiles-d9-from-gamebanana = { $count } de GameBanana
profiles-d9-local-count = { $count } local(es)
profiles-gamebanana = GameBanana
profiles-can-download = Se puede descargar
profiles-local-mod = Mod local
profiles-cant-download = No se puede descargar
profiles-local-why = Se añadió desde el disco en otro PC, así que no está en GameBanana. Pide el archivo a quien compartió el perfil.
profiles-d9-local-mode = El catálogo está desactivado (modo local), así que deadloader no puede descargar mods. Importa sin ellos y luego añadirlos desde el disco, o activa el catálogo en Configuración.
profiles-d9-after = En cualquier caso, «{ $name }» pasa a ser tu perfil seleccionado pero no se aplica: la carpeta del juego no cambia hasta que pulses Aplicar o Jugar.
profiles-import-without = Importar sin ellos
profiles-download-missing = Descargar los que faltan ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } mod desactivado
       *[other] { $count } mods desactivados
    }
profiles-turned-on =
    { $count ->
        [one] { $count } mod activado
       *[other] { $count } mods activados
    }
profiles-differ = Solo se cambiaron los archivos que difieren.
profiles-same = Los mismos mods ya estaban en el juego; solo se revisaron el orden y las variantes.
profiles-deleted = Perfil «{ $name }» eliminado.
profiles-deleted-not-applied = No estaba aplicado, así que la carpeta del juego no cambió.
profiles-deleted-applied =
    { $count ->
        [one] Se quitó { $count } mod de la carpeta del juego. Sigue en tu biblioteca.
       *[other] Se quitaron { $count } mods de la carpeta del juego. Siguen en tu biblioteca.
    }
profiles-exported = Perfil «{ $name }» exportado.
profiles-imported = Perfil «{ $name }» importado.
profiles-imported-text = Está seleccionado pero aún no aplicado.
profiles-imported-without =
    { $count ->
        [one] Perfil «{ $name }» importado sin { $count } mod.
       *[other] Perfil «{ $name }» importado sin { $count } mods.
    }
profiles-imported-without-text = Está seleccionado pero aún no aplicado. Puedes añadir los mods que faltan más tarde.
profiles-imported-downloading =
    { $count ->
        [one] Perfil «{ $name }» importado: se está descargando { $count } mod.
       *[other] Perfil «{ $name }» importado: se están descargando { $count } mods.
    }
profiles-imported-downloading-text = Está seleccionado pero aún no aplicado. Aplícalo cuando terminen las descargas.
profiles-open-downloads = Abrir descargas
profiles-undo = Deshacer
profiles-check-update = Buscar una actualización de deadloader
profiles-choose-another = Elegir otro archivo
profiles-switcher = Gestionar perfiles
