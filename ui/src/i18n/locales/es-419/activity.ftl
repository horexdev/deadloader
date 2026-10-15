# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Actividad
activity-sub =
    { $days ->
        [one] Todo lo que deadloader hizo en esta PC: solicitudes de red, cambios de archivos y eventos de la aplicación. Los registros se guardan en esta PC durante { $days } día.
       *[other] Todo lo que deadloader hizo en esta PC: solicitudes de red, cambios de archivos y eventos de la aplicación. Los registros se guardan en esta PC durante { $days } días.
    }
activity-open-logs = Abrir la carpeta de registros
activity-logs-opened = Carpeta de registros abierta en el Explorador de archivos.
activity-summary = Hoy de un vistazo
activity-net-today = Red hoy
activity-hosts-label = Hosts contactados hoy — elige uno para filtrar
activity-hosts-contacted =
    { $count ->
        [one] { $count } host contactado
       *[other] { $count } hosts contactados
    }
activity-host-title =
    { $count ->
        [one] { $count } solicitud · { $source } — elige para filtrar
       *[other] { $count } solicitudes · { $source } — elige para filtrar
    }
activity-net-none-local = ninguna — modo local
activity-net-none-yet = ninguna todavía
activity-nothing-else = Nada más.
activity-blocked-lead =
    { $count ->
        [one] { $count } solicitud a
       *[other] { $count } solicitudes a
    }
activity-blocked-tail =
    { $count ->
        [one] se bloqueó antes de conectar.
       *[other] se bloquearon antes de conectar.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } solicitud bloqueada, nunca contactada.
       *[other] { $count } solicitudes bloqueadas, nunca contactadas.
    }
activity-show-entry = Mostrar entrada
activity-game-today = Carpeta del juego hoy
activity-show-in-files = Mostrar en Archivos
activity-game-none = sin cambios
activity-no-game-changes = deadloader todavía no ha cambiado nada en la carpeta del juego.
activity-game-changes =
    { $count ->
        [one] { $count } cambio
       *[other] { $count } cambios
    }
activity-game-batches =
    { $count ->
        [one] en { $count } tanda
       *[other] en { $count } tandas
    }
activity-game-failed =
    { $count ->
        [one] { $count } con error
       *[other] { $count } con error
    }
activity-game-hardlink = { $count } vinculados
activity-game-copy = { $count } copiados
activity-game-delete = { $count } quitados
activity-game-rename = { $count } renombrados
activity-game-write = { $count } escritos
activity-game-backup = { $count } con copia de seguridad
activity-game-restore = { $count } restaurados
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi actualizado una vez
       *[other] gameinfo.gi actualizado { $count } veces
    }
activity-no-requests-today = Hoy no hubo solicitudes.
activity-no-requests-yet = Todavía no hay solicitudes.
activity-local-short = El catálogo está desactivado, así que deadloader no se conecta a nada.
activity-never-contacted = deadloader no ha contactado con ningún host desde que se instaló.
activity-tab-network = Red
activity-tab-files = Archivos
activity-tab-app = Aplicación
activity-tab-off = desactivada
activity-tab-errors =
    { $count ->
        [one] · { $count } error
       *[other] · { $count } errores
    }
activity-search = Buscar en el registro
activity-search-clear = Borrar búsqueda
activity-matches =
    { $count ->
        [one] { $count } resultado
       *[other] { $count } resultados
    }
activity-matches-more = { $count }+ resultados
activity-f-time = Periodo
activity-f-purpose = Propósito
activity-f-host = Host
activity-f-op = Operación
activity-f-level = Nivel
activity-f-result = Resultado
activity-all = Todos
activity-ok = OK
activity-errors = Errores
activity-range-hour = Última hora
activity-range-today = Hoy
activity-range-7d = Últimos 7 días
activity-range-30d = Últimos 30 días
activity-clear-filters = Borrar filtros
activity-clear-all = Borrar búsqueda y filtros
activity-follow = Seguir en vivo
activity-follow-tip = Mostrar las entradas nuevas arriba en cuanto ocurren
activity-follow-on = Las entradas nuevas aparecen arriba
activity-follow-paused = En pausa desde las { $time }
activity-show-new =
    { $count ->
        [one] Mostrar { $count } nueva
       *[other] Mostrar { $count } nuevas
    }
activity-net-aria = Solicitudes de red, las más recientes primero
activity-files-aria = Operaciones de archivos, las más recientes primero
activity-app-aria = Eventos de la aplicación, los más recientes primero
activity-c-redirects = Redirecciones
activity-c-time = Hora
activity-c-purpose = Propósito
activity-c-request = Solicitud
activity-c-status = Estado
activity-c-size = Tamaño
activity-c-duration = Duración
activity-c-started-by = Iniciada por
activity-c-source = Origen del host
activity-c-op = Operación
activity-c-from-to = De → a
activity-c-result = Resultado
activity-c-change = Parte del cambio
activity-c-level = Nivel
activity-c-message = Mensaje
activity-to = a
activity-loading-more = Cargando más…
activity-p-catalog = Catálogo
activity-p-thumbnail = Miniatura
activity-p-description-image = Imagen de la descripción
activity-p-download = Descarga
activity-p-update-check = Búsqueda de actualizaciones
activity-src-builtin = Integrado
activity-src-github = GitHub
activity-src-user = Agregado por ti
activity-src-user-image = Host de imágenes
activity-src-blocked = No permitido
activity-src-long-builtin = Integrado — GameBanana, parte de deadloader
activity-src-long-github = GitHub — la búsqueda de actualizaciones de deadloader, activada en Configuración
activity-src-long-user = Agregado por ti — en Configuración → Avanzado
activity-src-long-user-image = Host de imágenes permitido — lo permitiste al cargar una imagen de la descripción
activity-src-long-blocked = No está en la lista de hosts permitidos
activity-by-auto = Auto
activity-by-user = Tú
activity-op-hardlink = Vincular
activity-op-copy = Copiar
activity-op-rename = Renombrar
activity-op-delete = Eliminar
activity-op-write = Escribir
activity-op-backup = Copia de seguridad
activity-op-restore = Restaurar
activity-lv-debug = Depuración
activity-lv-info = Info
activity-lv-warn = Advertencia
activity-lv-error = Error
activity-st-blocked = Bloqueada
activity-st-blocked-long = Bloqueada — nunca contactada
activity-st-failed = Falló
activity-redirects =
    { $count ->
        [one] { $count } redirección
       *[other] { $count } redirecciones
    }
activity-retries =
    { $count ->
        [one] { $count } reintento
       *[other] { $count } reintentos
    }
activity-show-redirects = Mostrar redirecciones
activity-hide-redirects = Ocultar redirecciones
activity-hop-request = Paso 1 · solicitud
activity-hop-redirect = Paso { $n } · redirección
activity-hop-retry = Paso { $n } · intento { $try }
activity-ms = { $ms } ms
activity-seconds = { $s } s
activity-no-match = Ninguna entrada coincide
activity-no-match-text = Prueba con otro periodo o borra los filtros y la búsqueda.
activity-empty-title = Nada todavía
activity-empty-text = deadloader anota aquí cada solicitud de red, cada cambio de archivos en la carpeta del juego y cada evento de la aplicación en cuanto ocurren. Todavía no ha pasado nada.
activity-local-title = Modo local — no se hacen solicitudes
activity-local-why = El catálogo está desactivado, así que deadloader no se conecta a nada: ni a GameBanana ni a ningún otro sitio. Los cambios de archivos y los eventos de la aplicación se siguen registrando en las otras pestañas.
activity-local-do = Activa el catálogo en Configuración para explorar GameBanana. Entonces cada solicitud aparecerá aquí.
activity-enable-catalog = Activar catálogo
activity-details = Detalles de la entrada del registro
activity-close-details = Cerrar detalles
activity-close-esc = Cerrar (Esc)
activity-why = Por qué:
activity-todo = Qué hacer:
activity-open-advanced = Abrir Configuración → Avanzado
activity-full-record = Entrada completa
activity-hops-title = Redirecciones · { $count }
activity-user-hidden = Tu nombre de usuario de Windows está oculto en los registros:
activity-settings-logs = Configuración → Registros
activity-copy = Copiar entrada
activity-copied = Copiado
activity-kind-network = Solicitud de red
activity-kind-files = Operación de archivo
activity-kind-app = Evento de la aplicación
activity-d-time = Hora
activity-d-purpose = Propósito
activity-d-started-by = Iniciada por
activity-d-request = Solicitud
activity-d-source = Origen del host
activity-d-status = Estado
activity-d-size = Tamaño
activity-d-duration = Duración
activity-d-range = Rango
activity-d-error = Error
activity-d-id = ID de la entrada
activity-d-request-id = ID de la solicitud
activity-d-operation = Operación
activity-d-from = De
activity-d-to = A
activity-d-file = Archivo
activity-d-result = Resultado
activity-d-change = Parte del cambio
activity-d-sha256 = SHA-256
activity-d-level = Nivel
activity-d-message = Mensaje
activity-d-fields = Detalles
activity-status-after =
    { $count ->
        [one] { $status } — tras { $count } redirección
       *[other] { $status } — tras { $count } redirecciones
    }
