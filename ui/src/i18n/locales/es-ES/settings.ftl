# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Ajustes
settings-nav = Secciones de los ajustes
settings-reset-nav = Restablecer todos los ajustes…
settings-default = Por defecto: { $value }
settings-restore = Restaurar valor por defecto
settings-on = Activado
settings-off = Desactivado
settings-cancel = Cancelar
settings-remove = Quitar
settings-remove-item = Quitar { $item }
settings-add = Añadir
settings-open-folder = Abrir carpeta
settings-open-folder-at = Abrir carpeta: { $path }
settings-copied = Copiado al portapapeles.
settings-folder = Carpeta
settings-example = Ejemplo:

settings-section-general = General
settings-section-general-lead = Idioma, inicio y zoom.
settings-section-game = Juego
settings-section-game-lead = Dónde está instalado Deadlock y qué puede cambiar deadloader allí.
settings-section-catalog = Catálogo
settings-section-catalog-lead = El catálogo de GameBanana, contenido sensible, imágenes y enlaces.
settings-section-library = Biblioteca
settings-section-library-lead = Actualizaciones de mods, borrado de mods y versiones antiguas.
settings-section-downloads = Descargas
settings-section-downloads-lead = Cola de descargas, archivos comprimidos y descompresión.
settings-section-network = Red
settings-section-network-lead = Cómo se comunica deadloader con GameBanana.
settings-section-cache = Caché
settings-section-cache-lead = Miniaturas y páginas del catálogo guardadas en este PC.
settings-section-logs = Registros
settings-section-logs-lead = Qué guarda Actividad y durante cuánto tiempo.
settings-section-advanced = Avanzado
settings-section-advanced-lead = Archivos de ajustes y dominios permitidos adicionales.
settings-section-about = Acerca de deadloader
settings-section-about-lead = Versión, licencia, cómo comprobar esta descarga y dónde están tus datos.

# General
settings-general-group = Idioma e inicio
settings-language = Idioma
settings-language-help = Idioma del sistema: el idioma de Windows si deadloader lo tiene; si no, inglés.
settings-language-system = Idioma del sistema ({ $language })
settings-startup-profile = Perfil al iniciar
settings-startup-profile-help = Qué perfil está seleccionado al iniciar deadloader. Seleccionarlo no lo aplica.
settings-startup-last = El último seleccionado ({ $name })
settings-startup-last-plain = El último seleccionado
settings-app-updates = Buscar actualizaciones de deadloader al iniciar
settings-app-updates-help = deadloader pregunta una vez a GitHub (api.github.com) al iniciar si hay una versión más nueva. Actualizar siempre es manual. Lo elegiste en el primer inicio.
settings-app-updates-local = No disponible: el modo local no hace ninguna petición de red. La opción estará disponible cuando actives el catálogo (Ajustes → Catálogo).
settings-zoom = Zoom
settings-zoom-keys-1 = También con Ctrl + rueda del ratón,
settings-zoom-keys-2 = y
settings-zoom-keys-3 = para restablecer.
settings-ctrl = Ctrl
settings-percent = { $value } %

# Game
settings-installs-group = Instalaciones del juego
settings-installs-lead = deadloader encuentra solo las instalaciones de Deadlock en Steam. Cada perfil usa una de ellas; cámbialo en Perfiles.
settings-install-library = Biblioteca de Steam { $library }
settings-install-build = compilación { $build }
settings-install-used =
    { $count ->
        [0] no la usa ningún perfil
        [one] la usa 1 perfil: { $names }
       *[other] la usan { $count } perfiles: { $names }
    }
settings-install-selected = { $name } (seleccionado)
settings-install-applied = { $name } (aplicado aquí)
settings-install-steam = Encontrada por Steam
settings-install-manual = Añadida a mano
settings-install-known = Aún hay archivos de deadloader aquí
settings-install-manual-meta = Añadida a mano
settings-gi-ours = gameinfo.gi: OK — la línea de deadloader está presente
settings-gi-wait = gameinfo.gi: se configura cuando un perfil la use
settings-gi-rolled = gameinfo.gi: original — Aplicar vuelve a añadir la línea de deadloader
settings-gi-foreign = gameinfo.gi: modificado por otro programa
settings-gi-missing = gameinfo.gi: no encontrado — quizá la carpeta se movió
settings-installs-none = Todavía no se ha encontrado ninguna instalación.
settings-add-manual = Añadir a mano…
settings-add-manual-hint = Elige una carpeta de Deadlock que no se haya encontrado.
settings-add-manual-title = Elige la carpeta de Deadlock
settings-install-added = Instalación añadida.
settings-install-removed = Instalación quitada de la lista.
settings-install-removed-text = No se ha cambiado nada en esa carpeta.
settings-gameinfo-group = Tras las actualizaciones del juego
settings-autofix = Reparar gameinfo.gi automáticamente
settings-autofix-help = Una actualización de Deadlock restablece el archivo que le indica que cargue mods. deadloader hace una copia, vuelve a poner su línea y te avisa. Si está desactivado, primero te pregunta un aviso.
settings-danger-group = Deshacer y eliminar
settings-rollback = Revertir la carpeta del juego del perfil seleccionado
settings-rollback-help = Para la instalación del perfil seleccionado { $profile } ({ $drive }): quita los { $files } archivos de mods que deadloader puso allí y su línea en gameinfo.gi. La biblioteca y los perfiles se quedan; Aplicar lo devuelve todo.
settings-rollback-help-others = Las demás carpetas del juego no se tocan; selecciona un perfil que las use para revertirlas.
settings-rollback-nothing = Nada que revertir en { $drive }: deadloader no tiene archivos allí ahora.
settings-rollback-no-game = No se ha encontrado Deadlock, así que no hay carpeta del juego que revertir.
settings-rollback-button = Revertir…
settings-rolled-title = Carpeta del juego en { $drive } revertida.
settings-rolled-text =
    { $foreign ->
        [0] En { $drive } se quitaron { $files } archivos de mods y la línea de deadloader en gameinfo.gi. La biblioteca y los perfiles se conservan: Aplicar o Jugar recupera los mods.
       *[other] En { $drive } se quitaron { $files } archivos de mods, la línea de deadloader en gameinfo.gi y { $foreign } archivos ajenos. La biblioteca y los perfiles se conservan: Aplicar o Jugar recupera los mods.
    }
settings-wipe = Borrar todos los datos de deadloader
settings-wipe-help = Borra la biblioteca, los perfiles, los registros, la caché y los ajustes ({ $size }) y cierra deadloader. deadloader.exe se queda; bórralo tú después.
settings-wipe-button = Borrar todos los datos…

# Catalog
settings-catalog-group = Catálogo de GameBanana
settings-catalog = Usar el catálogo
settings-catalog-help-on = Explora e instala mods de GameBanana. deadloader solo habla con GameBanana y cada petición aparece en Actividad. Desactivado significa modo local.
settings-catalog-help-off = Desactivado — modo local: deadloader no hace ninguna petición de red. Puedes seguir añadiendo mods desde el disco.
settings-consent-title = Primero, un paso.
settings-consent-text = deadloader te muestra con qué hosts contactará y las condiciones de GameBanana antes de activar el catálogo.
settings-consent-go = Revisar y activar
settings-local-on = El modo local está activado.
settings-local-on-text = El catálogo está desactivado y deadloader no hace peticiones de red.
settings-ttl = Reutilizar las páginas del catálogo cargadas durante
settings-ttl-help = Dentro de este tiempo, volver a una página no vuelve a preguntar a GameBanana.
settings-minutes = { $count } min
settings-hours = { $count } h
settings-prefetch = Cargar la página siguiente por adelantado
settings-prefetch-help = Cuando has recorrido cerca del 70 % de los resultados, se cargan los siguientes para que la cuadrícula siga.
settings-nsfw-group = Contenido sensible
settings-nsfw = Mods con clasificación de contenido (NSFW)
settings-nsfw-help = Ocultos: no se muestran en absoluto. Difuminados: una imagen borrosa con la etiqueta «Contenido sensible»; haz clic para verla. Visibles: sin difuminar.
settings-nsfw-age-done = Edad confirmada el { $date }.
settings-nsfw-age-ask = Al salir de «Ocultos» se te pide una vez que confirmes tu edad.
settings-nsfw-hidden = Ocultos
settings-nsfw-blurred = Difuminados
settings-nsfw-shown = Visibles
settings-images-group = Imágenes y enlaces
settings-description-images = Imágenes en las descripciones de mods
settings-description-images-help = Vienen de hosts distintos de GameBanana, p. ej., i.imgur.com. Al hacer clic: un marcador hasta que hagas clic.
settings-images-click = Al hacer clic
settings-images-always = Siempre
settings-prechecked = Marcar de antemano «Permitir siempre imágenes de este host»
settings-prechecked-help = En el diálogo que aparece la primera vez que una imagen de una descripción viene de un host nuevo.
settings-hosts = Hosts de imágenes permitidos
settings-hosts-help = Las imágenes de estos hosts se cargan sin preguntar. En Actividad sus peticiones se marcan como «host de imágenes permitido».
settings-hosts-built-in = Integrado
settings-hosts-none = No hay hosts añadidos: cada host nuevo pregunta primero.
settings-host-removed = { $host } quitado
settings-host-removed-text = Sus imágenes volverán a preguntar antes de cargarse.
settings-undo = Deshacer
settings-gif = Reproducir GIF automáticamente
settings-gif-help = Desactivado: los GIF muestran su primer fotograma con un botón de reproducir.
settings-links = Enlaces externos
settings-links-help = Los enlaces se abren en tu navegador predeterminado. «Preguntar antes» muestra la dirección completa antes de abrirla.
settings-links-ask = Preguntar antes
settings-links-open = Abrir directamente

# Library
settings-updates-group = Actualizaciones de mods
settings-check-updates = Buscar actualizaciones de mods al iniciar
settings-check-updates-help = Compara tus mods de GameBanana con el catálogo. Nunca en modo local.
settings-auto-download = Descargar actualizaciones automáticamente
settings-auto-download-help = Solo la descarga: deadloader sigue preguntando antes de que un mod cambie a la nueva versión.
settings-disk-group = Borrado y versiones antiguas
settings-deletion = Al borrar un mod
settings-deletion-help = Puedes elegir otra cosa cada vez en el diálogo de borrado.
settings-deletion-permanent = Definitivamente
settings-deletion-bin = A la papelera de reciclaje
settings-old-versions = Versiones antiguas
settings-old-versions-help = Se conserva cada versión descargada para que puedas volver atrás. Borra las versiones antiguas de un mod en sus detalles.
settings-old-keep-all = Conservar todas

# Downloads
settings-queue-group = Cola
settings-parallel = Descargas simultáneas
settings-parallel-help = Cuántos archivos se descargan a la vez.
settings-speed = Límite de velocidad
settings-speed-help = Solo para descargas de mods; el catálogo y las miniaturas no tienen límite.
settings-speed-none = Sin límite
settings-speed-value = { $value } MB/s
settings-keep-archives = Conservar los archivos comprimidos tras instalar
settings-keep-archives-help = Desactivado: el archivo .zip, .7z o .rar se borra en cuanto sus archivos .vpk están en la biblioteca.
settings-unpack-group = Descompresión
settings-limits = Límites al descomprimir
settings-limits-help = Protege frente a archivos que se descomprimen en algo enorme. Si se rechaza un archivo en el que confías, sube el límite.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = Tamaño descomprimido
settings-limit-ratio = Tasa de compresión
settings-limit-files = Archivos de mods por archivo comprimido
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = archivos
settings-unit-s = s
settings-number-error = { $field }: escribe un número entero de { $min } a { $max }. Hasta entonces se sigue usando el valor anterior.

# Network
settings-network-local = El modo local está activado.
settings-network-local-text = deadloader no hace peticiones de red; estos ajustes se aplican cuando el catálogo esté activado.
settings-requests-group = Peticiones a GameBanana
settings-api-parallel = Peticiones simultáneas al catálogo
settings-api-parallel-help = Peticiones a la API de GameBanana al mismo tiempo. Las imágenes y las descargas se cuentan aparte.
settings-timeouts = Tiempos de espera
settings-timeouts-help = Conexión: desistir si no se conecta en este tiempo. Respuesta: desistir cuando el servidor deja de enviar.
settings-timeouts-default = 10 s · 30 s
settings-timeout-connect = Conexión
settings-timeout-read = Respuesta
settings-retries = Reintentos
settings-retries-help = Cuántas veces se repite una petición fallida, con pausas crecientes, antes de que deadloader muestre un error.
settings-connection-group = Conexión
settings-proxy = Proxy
settings-proxy-help = Ninguno: conexión directa. Sistema: el proxy configurado en Windows. Manual: escríbelo abajo.
settings-proxy-none = Ninguno
settings-proxy-system = Sistema
settings-proxy-manual = Manual
settings-proxy-host = Dirección
settings-proxy-port = Puerto
settings-proxy-hint = Proxy HTTP. Solo pasan por él las peticiones de deadloader.
settings-proxy-invalid = Escribe un nombre de host o una dirección sin usuario ni contraseña, p. ej., proxy.example.com.

# Cache
settings-thumbs-group = Miniaturas y páginas del catálogo
settings-cache-limit = Límite de la caché
settings-cache-limit-help = Cuando la caché está llena, se van primero las miniaturas que llevas más tiempo sin ver.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = Tamaño actual
settings-cache-breakdown = { $thumbnails } miniaturas · { $pages } páginas del catálogo
settings-cache-empty = Vacía: todavía no hay nada en caché.
settings-cache-used = { $used } de { $limit }
settings-cache-clear = Vaciar caché
settings-cache-cleared = Caché vaciada: { $size } liberados. Las miniaturas se vuelven a cargar cuando las necesites.
settings-cache-cleared-partly = { $failed } archivos estaban en uso y se quedan hasta el próximo vaciado.

# Logs
settings-keep-group = Conservación de registros
settings-log-days = Conservar registros durante
settings-log-days-help = Las entradas más antiguas se borran al iniciar deadloader.
settings-days = { $count ->
        [one] { $count } día
       *[other] { $count } días
    }
settings-year = 1 año
settings-log-size = Tamaño máximo
settings-log-size-help = Por encima de este tamaño se borran primero los archivos de registro más antiguos.
settings-privacy-group = Privacidad
settings-mask = Ocultar tu nombre de usuario de Windows
settings-mask-help = Las rutas de los registros muestran { $masked } en lugar de tu nombre: más seguro si compartes un registro.
settings-logs-folder-group = Carpeta de registros
settings-logs-folder = Dónde están los registros
settings-logs-now = { $size } · { $files } archivos · el más antiguo del { $date }
settings-open-logs = Abrir carpeta de registros

# Advanced
settings-advanced-warning = Para usuarios con experiencia.
settings-advanced-warning-text = Un error aquí puede cambiar cómo se comporta deadloader. Comprueba el archivo antes de guardarlo y conserva una copia de la versión anterior.
settings-files-group = Editar los archivos de ajustes
settings-file-settings = Ajustes
settings-file-manifest = Manifiesto de la carpeta del juego
settings-file-label = Archivo que editar
settings-file-settings-lead = Todo lo de estas páginas, en un solo archivo. Los cambios aparecen en las páginas tras guardar.
settings-file-manifest-lead = La lista de lo que deadloader puso en la carpeta del juego. Revertir depende de ella: una entrada errónea puede dejar archivos atrás.
settings-json-error = Esto no es JSON válido, así que no se ha guardado.
settings-json-where = Línea { $line }, columna { $column }: falta o sobra algo aquí o justo antes; a menudo una coma o unas comillas.
settings-json-fix = Corrige ese punto y vuelve a guardar; mientras tanto deadloader sigue usando el archivo actual.
settings-json-refused = Estos valores no se pueden usar, así que no se ha guardado nada.
settings-json-newer = Este archivo es de un deadloader más reciente (formato { $found }; esta versión lee hasta { $supported }), así que no se ha guardado.
settings-json-invalid = deadloader no puede usar este archivo, así que no se ha guardado.
settings-json-details = Detalles
settings-problem-unknown-field = { $path }: no es un ajuste de esta versión.
settings-problem-invalid-value = { $path }: este valor no se puede usar aquí.
settings-problem-out-of-range = { $path }: { $found } está fuera de { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: «{ $entry }» no es válido aquí.
settings-problem-duplicate-entry = { $path }: «{ $entry }» aparece dos veces.
settings-problem-needs-age = { $path }: mostrar contenido NSFW requiere antes confirmar la edad (Ajustes → Catálogo).
settings-problem-local-mode = { $path }: no disponible en modo local.
settings-json-idle = Se comprueba al guardar.
settings-json-saved = Válido y guardado. La versión anterior se conserva como { $backup }.
settings-json-saved-plain = Válido y guardado.
settings-json-save = Comprobar y guardar
settings-json-discard = Descartar mis cambios
settings-json-backup = Antes de guardar se conserva una copia del archivo anterior; se mantienen las 10 últimas.
settings-domains-group = Dominios permitidos adicionales
settings-domains-lead = deadloader solo habla con GameBanana. Si una descarga se redirige a otro host y se bloquea, puedes permitir ese host aquí. En Actividad, sus peticiones se marcan como «añadido por el usuario».
settings-domains-built-in = Integrados:
settings-domain-user = añadido por el usuario
settings-domains-none = No hay dominios adicionales.
settings-domain-add = Añadir un dominio
settings-domain-hint = Solo el nombre de host, p. ej., files.example.com.
settings-domain-empty = Escribe primero un nombre de host, p. ej., files.example.com.
settings-domain-not-host = Escribe solo el nombre de host, sin https:// ni ruta, p. ej., files.example.com.
settings-domain-invalid = «{ $host }» no es un nombre de host. Revisa cómo está escrito, p. ej., files.example.com.
settings-domain-built-in = { $host } está integrado y siempre permitido.
settings-domain-listed = { $host } ya está en la lista.

# About
settings-version = versión { $version }
settings-commit = Compilado desde el commit
settings-commit-local = Compilación local: no la compiló GitHub Actions
settings-copy-commit = Copiar el id del commit
settings-release-page = Página de la versión
settings-licence-group = Licencia
settings-licence-help = deadloader es software libre: puedes usarlo, estudiarlo, compartirlo y modificarlo según la GPL-3. Se ofrece sin ninguna garantía.
settings-read-licence = Leer la licencia
settings-source = Código fuente
settings-verify-group = Cómo comprobar esta descarga
settings-verify-lead = Dos comprobaciones independientes de que este deadloader.exe es el publicado en GitHub.
settings-verify-hash = Comparar la suma SHA-256
settings-verify-hash-help = Debe coincidir con la suma de la página de la versión. Una copia modificada podría mostrar aquí un valor falso, así que calcúlala tú también:
settings-verify-hash-cmd = En PowerShell, en la carpeta del exe:
settings-verify-attest = Comprobar la atestación de compilación
settings-verify-attest-help = Demuestra que GitHub Actions compiló este archivo a partir del código fuente publicado. Necesita GitHub CLI:
settings-copy-hash = Copiar la suma
settings-copy-command = Copiar el comando
settings-unsigned = deadloader.exe no está firmado.
settings-unsigned-text = Windows SmartScreen puede decir «Windows protegió su PC» en el primer inicio. Las dos comprobaciones de arriba sirven para confirmar que el archivo es auténtico.
settings-webview-group = Motor de la ventana
settings-webview-help = La ventana de deadloader la dibuja WebView2, una parte de Windows. Windows lo actualiza por su cuenta. deadloader desactiva:
settings-webview-off-1 = Consultas DNS-over-HTTPS
settings-webview-off-2 = El inicio de sesión con la cuenta de Microsoft dentro de la ventana
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = El tráfico de red en segundo plano y las actualizaciones de componentes
settings-webview-off-5 = Las llamadas al servidor del autorrelleno de formularios
settings-webview-off-6 = Los informes de errores a Microsoft (domain reliability)
settings-webview-no-route = Y la ventana no tiene ninguna salida a Internet.
settings-webview-unknown = versión desconocida
settings-data-group = Tus datos
settings-data-folder = Carpeta de datos
settings-data-folder-help = Todo lo que guarda deadloader está en esta única carpeta junto al exe.
settings-notice = Aviso del primer inicio
settings-notice-help = Versión { $version }, aceptada el { $date } en { $language }. Tus elecciones: catálogo { $catalog }, búsqueda de actualizaciones { $updates }.
settings-notice-none = Aún no aceptado.
settings-notice-again = Volver a leerlo

# D7
settings-d7-title = ¿Revertir la carpeta del juego en { $drive }?
settings-d7-from =
    { $others ->
        [0] La instalación del perfil seleccionado { $profile }. deadloader quita de
       *[other] La instalación del perfil seleccionado { $profile } (también la usa { $others }). deadloader quita de
    }
settings-d7-files =
    { $count ->
        [one] 1 archivo de mod que añadió en
       *[other] { $count } archivos de mods que añadió en
    }
settings-d7-line = Su línea en gameinfo.gi
settings-d7-line-why = — el archivo vuelve a estar como lo instaló Steam.
settings-d7-stays = Se queda como está:
settings-d7-library = Tu biblioteca: { $mods } mods, { $size }
settings-d7-profiles =
    { $count ->
        [one] Tu perfil: { $names }
       *[other] Tus { $count } perfiles: { $names }
    }
settings-d7-other = La instalación en { $path } ({ $profile }): no se toca. Para revertirla, selecciona un perfil que la use y vuelve a revertir.
settings-d7-other-plain = La instalación en { $path }: no se toca.
settings-d7-back = Aplicar o Jugar lo devuelve todo.
settings-d7-foreign =
    { $count ->
        [one] Quitar también 1 archivo que no creó deadloader
       *[other] Quitar también { $count } archivos que no creó deadloader
    }
settings-d7-foreign-why = de otro programa o de una instalación a mano. Se borran definitivamente; deadloader no puede recuperarlos.
settings-d7-confirm = Revertir
settings-d7-confirm-foreign =
    { $count ->
        [one] Revertir y borrar 1 archivo
       *[other] Revertir y borrar { $count } archivos
    }

# D8
settings-d8-title = ¿Borrar todos los datos de deadloader?
settings-d8-from = deadloader borra todo lo que hay en
settings-d8-from-2 = y luego se cierra.
settings-d8-what = Qué
settings-d8-contains = Contiene
settings-d8-size = Tamaño
settings-d8-total = Total
settings-d8-library = Biblioteca
settings-d8-library-has = { $mods } mods, { $files } archivos
settings-d8-profiles = Perfiles
settings-d8-logs = Registros
settings-d8-logs-has = { $files } archivos, desde el { $date }
settings-d8-logs-none = sin archivos de registro
settings-d8-cache = Caché
settings-d8-cache-has = miniaturas y páginas del catálogo
settings-d8-settings = Ajustes
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
        [one] incl. 1 copia
       *[other] incl. { $backups } copias
    }
settings-d8-other = Otros
settings-d8-other-has = descargas a medias, copias de gameinfo.gi, datos del motor de la ventana
settings-d8-exe = No se borra: el propio deadloader.exe; bórralo tú si ya no lo necesitas.
settings-d8-rollback = Revertir primero cada carpeta del juego que usa deadloader (recomendado)
settings-d8-rollback-why = Quita los archivos de mods de deadloader y su línea en gameinfo.gi de cada carpeta del juego antes de borrar. Los archivos ajenos se quedan.
settings-d8-rollback-off = Sin marcar: los archivos de mods de deadloader se quedan en las carpetas del juego y Deadlock los sigue cargando, sin deadloader para quitarlos.
settings-d8-type-1 = Para confirmar, escribe
settings-d8-type-2 = { "" }
settings-d8-word = borrar
settings-d8-hint = El botón de borrar se desbloquea cuando la palabra coincide.
settings-d8-hint-ok = Confirmado. No se puede deshacer.
settings-d8-type-first = Escribe primero «{ $word }»
settings-d8-deleting = Borrando…
settings-d8-confirm = Borrar y cerrar
settings-d8-confirm-rollback = Revertir, borrar y cerrar

# Reset
settings-reset-title = ¿Restablecer todos los ajustes?
settings-reset-body = Cada ajuste vuelve a su valor por defecto, incluidos los hosts de imágenes permitidos y los dominios adicionales.
settings-reset-keeps = Tu biblioteca, tus perfiles y tus registros no se tocan, y la carpeta del juego se queda como está.
settings-reset-backup = Se conserva una copia de los ajustes actuales como settings.json.bak-…
settings-reset-confirm = Restablecer ajustes
settings-reset-done = Todos los ajustes se han restablecido.
settings-reset-done-text = Copia: { $backup }
