# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = ¿Abrir este enlace en tu navegador?
catalog-link-body = Sale de deadloader y se abre en tu navegador predeterminado.
catalog-link-always = Abrir siempre los enlaces de inmediato
catalog-link-cancel = Cancelar
catalog-link-open = Abrir

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Ocultos
catalog-nsfw-blurred = Difuminados
catalog-nsfw-shown = Visibles
catalog-age-title = ¿Mostrar mods con contenido sensible?
catalog-age-body = Algunos mods de GameBanana están clasificados por desnudez, temas sexuales o similares. Para verlos, confirma que tienes 18 años o más, o la mayoría de edad donde vives.
catalog-age-choice = Elegiste: { $mode }.
catalog-age-once = deadloader lo pregunta solo una vez. Puedes volver a “Ocultos” cuando quieras.
catalog-age-confirm = Soy mayor de edad
catalog-age-cancel = Cancelar

## S2: the catalog grid
catalog-title = Catálogo
catalog-section = Sección
catalog-section-mods = Mods
catalog-section-sounds = Sonidos
catalog-search = Buscar mods
catalog-search-sounds = Buscar sonidos
catalog-search-offline = Para buscar hace falta conexión
catalog-search-off = Búsqueda no disponible: no se puede leer el catálogo
catalog-search-clear = Borrar búsqueda
catalog-results =
    { $count ->
        [one] { $count } resultado
       *[other] { $count } resultados
    }
catalog-sort = Orden
catalog-sort-newest = Más nuevos
catalog-sort-latest_updated = Actualizados recientemente
catalog-sort-most_downloaded = Más descargados
catalog-sort-most_liked = Más gustados
catalog-sort-most_viewed = Más vistos
catalog-sort-most_commented = Más comentados
catalog-sort-oldest = Más antiguos
catalog-sort-best = Más relevantes
catalog-sort-best-title = Mientras buscas, se ordena por relevancia
catalog-category = Categoría
catalog-category-all = Todas las categorías
catalog-hero = Héroe
catalog-hero-all = Todos los héroes
catalog-hero-filter = Buscar un héroe
catalog-hero-none = Ningún héroe coincide
catalog-filters = Filtros
catalog-filter-nsfw = Mods con clasificación de contenido (NSFW)
catalog-filter-favourites = Solo favoritos
catalog-filter-favourites-short = solo favoritos
catalog-filter-installed = Ocultar instalados
catalog-filter-installed-short = instalados ocultos
catalog-nsfw-caption-hidden = NSFW ocultos
catalog-nsfw-caption-blurred = NSFW difuminados · haz clic en una tarjeta para verla
catalog-nsfw-caption-shown = NSFW visibles
catalog-loading = Cargando mods de GameBanana…
catalog-loading-sounds = Cargando sonidos de GameBanana…
catalog-loading-more = Cargando más…
catalog-more-failed = No se pudo cargar la página siguiente.
catalog-grid = Mods
catalog-grid-sounds = Sonidos
catalog-likes =
    { $count ->
        [one] { $count } me gusta
       *[other] { $count } me gusta
    }
catalog-favourite-add = Agregar a favoritos
catalog-favourite-remove = Quitar de favoritos
catalog-sensitive = Contenido sensible
catalog-click-show = Haz clic para ver
catalog-badge-installed = Instalado
catalog-badge-update = Actualización
catalog-card-installing = Instalando · { $percent } %
catalog-empty = Aún no hay mods aquí
catalog-empty-search = Ningún mod coincide con “{ $text }”
catalog-empty-favourites = Aún no tienes favoritos
catalog-empty-filters = Filtros activos: { $filters }. Quítalos para buscar en todos los mods o prueba una palabra más corta.
catalog-empty-shorter = Prueba una palabra más corta o distinta.
catalog-clear-filters = Quitar filtros
catalog-rate-retry = deadloader lo reintenta automáticamente en { $seconds } s; los mods de abajo siguen disponibles.
catalog-retry-now = Reintentar ahora
catalog-try-again = Reintentar
catalog-check-update = Buscar actualización de deadloader
catalog-details = Detalles · entrada del registro
catalog-copy-id = Copiar el identificador de la entrada
catalog-open-activity = Abrir en Actividad
catalog-offline-empty-title = Sin conexión: no hay nada guardado
catalog-offline-empty-text = deadloader no puede conectar con GameBanana y esta parte del catálogo no se guardó antes.
catalog-error-title = Ahora mismo no se puede mostrar el catálogo.
catalog-local-title = El catálogo está desactivado (modo local)
catalog-local-why = En modo local, deadloader no hace ninguna petición de red. Tu biblioteca, tus perfiles y Jugar funcionan como siempre, y puedes agregar mods desde el disco.
catalog-local-do = Activa el catálogo para explorar e instalar mods de GameBanana. deadloader solo hablará con GameBanana.
catalog-local-enable = Activar catálogo
catalog-local-library = Ir a la biblioteca

## S3: the mod page
catalog-breadcrumb = Ruta de navegación
catalog-back = Volver al catálogo
catalog-page-loading = Cargando la página del mod de GameBanana…
catalog-page-offline = Sin conexión: se muestra la copia guardada el { $date }.
catalog-page-offline-why = Instalar archivos nuevos requiere conexión; cambiar entre variantes descargadas sigue funcionando.
catalog-page-offline-empty-title = Sin conexión: esta página no está guardada
catalog-page-offline-empty-text = deadloader no puede conectar con GameBanana y esta página no se abrió antes.
catalog-page-local-title = Esta página de mod no está guardada
catalog-page-local-text = En modo local, deadloader solo muestra páginas de mods guardadas antes.
catalog-removed-title = Este mod fue retirado de GameBanana
catalog-removed-why = Fue retenido o eliminado, así que deadloader no puede mostrar su página ni descargar sus archivos.
catalog-removed-keep = Tu copia instalada ({ $copy }) se queda en la biblioteca y sigue funcionando; simplemente ya no recibirá actualizaciones.
catalog-show-in-library = Ver en la biblioteca
catalog-installed = Instalado
catalog-by = de { $author }
catalog-version = Versión { $version }
catalog-views =
    { $count ->
        [one] { $count } visita
       *[other] { $count } visitas
    }
catalog-downloads =
    { $count ->
        [one] { $count } descarga
       *[other] { $count } descargas
    }
catalog-added = Agregado { $when }
catalog-updated = Actualizado { $when }
catalog-favourite = Favorito
catalog-open-gamebanana = Abrir en GameBanana
catalog-gallery-alt = { $name }, imagen { $index } de la galería
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Imagen anterior
catalog-gallery-next = Imagen siguiente
catalog-gallery-original = Ver original
catalog-gallery-hidden = Las imágenes de mods con clasificación de contenido están ocultas.
catalog-audio-preview = Este sonido tiene una muestra en GameBanana.
catalog-description = Descripción
catalog-description-more = Ver la descripción completa
catalog-description-less = Ver menos
catalog-description-truncated = Solo se muestra el principio de esta descripción.
catalog-opens-browser = se abre en tu navegador
catalog-opens-here = Se abre en deadloader
catalog-image-from = Imagen de
catalog-image-click = haz clic para cargarla
catalog-image-failed = no se pudo cargar
catalog-image-alt = Imagen de la descripción
catalog-youtube = Video de YouTube
catalog-spoiler = Spoiler: haz clic para verlo
catalog-spoiler-hide = Ocultar spoiler
catalog-files = Archivos
catalog-files-clean =
    { $count ->
        [one] Pasó el antivirus de GameBanana
       *[other] Los { $count } pasaron el antivirus de GameBanana
    }
catalog-files-clean-dated =
    { $count ->
        [one] Pasó el antivirus de GameBanana · subido el { $date }
       *[other] Los { $count } pasaron el antivirus de GameBanana · subidos el { $date }
    }
catalog-files-av = El resultado del antivirus de GameBanana aparece en cada archivo.
catalog-variants-note = Estos archivos cambian los mismos archivos del juego, así que son variantes: solo una está activa.
catalog-no-files = Este mod no tiene archivos para descargar.
catalog-older-files = Archivos anteriores ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } descarga
       *[other] { $count } descargas
    }
catalog-file-active = variante activa
catalog-file-downloaded = descargado
catalog-file-install = Instalar
catalog-file-install-label = Instalar { $file } y hacerla la variante activa
catalog-file-install-title = Descarga este archivo y lo hace la variante activa
catalog-file-install-offline = Instalar requiere conexión
catalog-file-installing = Instalando…
catalog-file-installed = Instalado
catalog-file-switch = Cambiar a
catalog-file-switch-label = Cambiar a { $file }
catalog-file-switch-title = Ya descargado: hacerlo la variante activa
catalog-file-update = Actualizar
catalog-file-update-title = Una versión más nueva sustituye a tu archivo instalado; revísala en la biblioteca
catalog-av-clean = Limpio
catalog-av-clean-title = Antivirus de GameBanana: limpio
catalog-av-flagged = Marcado
catalog-av-flagged-title = Antivirus de GameBanana: { $verdict }
catalog-av-pending = Aún sin analizar
catalog-av-pending-title = GameBanana aún no termina de analizar este archivo
catalog-av-none = Sin análisis
catalog-av-none-title = GameBanana no envió resultado del antivirus para este archivo
catalog-credits = Créditos
catalog-no-ratings = Sin clasificación de contenido
catalog-no-licence = Sin texto de licencia
catalog-no-sources = Sin fuentes alternativas
catalog-rating-sexual_themes = Temas sexuales
catalog-rating-sexual_content = Contenido sexual
catalog-rating-skimpy_attire = Ropa reveladora
catalog-rating-lewd_angles_and_poses = Ángulos y poses sugerentes
catalog-rating-full_nudity = Desnudo integral
catalog-licence = Licencia
catalog-sources = Fuentes alternativas
catalog-videos = Videos

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = ¿Cargar una imagen de un servidor nuevo?
catalog-image-body-before = deadloader descargará esta imagen de
catalog-image-body-after = . Ese servidor no es GameBanana; la petición aparece en Actividad como todas las demás.
catalog-image-note = El tamaño de las imágenes está limitado. Puedes quitar servidores permitidos en Configuración → Catálogo.
catalog-image-always = Permitir siempre imágenes de este servidor
catalog-image-cancel = Cancelar
catalog-image-load = Cargar imagen
