# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Progreso de la configuración
wizard-caption-setup = Primera configuración
wizard-caption-changed = Aviso actualizado
wizard-caption-catalog = Activar el catálogo
wizard-caption-review = Aviso del primer inicio
wizard-language = Idioma
wizard-step-welcome = Bienvenida
wizard-step-risks = Riesgos
wizard-step-network = Red
wizard-step-privacy = Privacidad
wizard-step-updates = Actualizaciones
wizard-step-setup = Configuración
wizard-step-of = Paso { $n } de 6
wizard-changed-of =
    { $count ->
        [one] { $count } paso modificado
       *[other] { $count } pasos modificados
    }
wizard-meta-changed = Modificado
wizard-meta-kept = Sin cambios
wizard-back = Atrás
wizard-next = Siguiente
wizard-finish = Terminar la configuración
wizard-continue = Continuar
wizard-close = Cerrar
wizard-cancel = Cancelar
wizard-to-catalog = Después se abre el catálogo.
wizard-to-library = Después se abre tu biblioteca (modo local).

# Step 1
wizard-s1-title = Te damos la bienvenida a deadloader
wizard-s1-lead = deadloader instala mods de Deadlock desde GameBanana o desde tu disco y te muestra cada cambio que hace.
wizard-f1a = Agrega archivos de mods a la carpeta de mods del juego y una línea a
wizard-f1b = — un archivo del juego que le dice a Deadlock que cargue mods.
wizard-f2 = Todo se puede deshacer: revertir quita del juego todo lo que puso deadloader.
wizard-f3a = Todos sus datos se quedan en la carpeta
wizard-f3b = junto a
wizard-f3c = .
wizard-f4 = deadloader no está afiliado a Valve ni a GameBanana.
wizard-f5 = Software libre bajo la GPL-3, sin ninguna garantía.
wizard-licence = Licencia (GPL-3)
wizard-source = Código fuente

# Step 2
wizard-s2-title = Los mods son bajo tu propio riesgo
wizard-r1 = Valve no tiene normas oficiales sobre mods para Deadlock. Usar mods es tu decisión y tu riesgo.
wizard-r2 = Las actualizaciones del juego pueden romper mods. deadloader se vuelve a configurar tras una actualización, pero un mod puede no funcionar hasta que su autor lo actualice.
wizard-r3 = Los mods los hacen otras personas bajo sus propias licencias. deadloader no puede responder de su contenido; para cada archivo muestra el resultado del antivirus de GameBanana.
wizard-agree = Entiendo que deadloader cambia archivos del juego y que uso los mods bajo mi propio riesgo.
wizard-agree-note = Esto confirma los pasos 1 y 2. Es necesario para continuar.
wizard-agree-error = Marca esta casilla para continuar.
wizard-agree-reason = Primero marca la casilla.

# Step 3
wizard-s3-title = Red y GameBanana
wizard-s3-lead = El catálogo habla directamente con la API no oficial de GameBanana, y con nadie más. Estos son los únicos hosts a los que se conecta:
wizard-hosts = Hosts
wizard-host-api = catálogo y páginas de mods
wizard-host-images = miniaturas y galería
wizard-host-files = descargas
wizard-host-cache = descargas (files.gamebanana.com redirige ahí)
wizard-host-new = Nuevo
wizard-s3-images = Las imágenes de otros hosts en las descripciones de los mods solo se cargan cuando permites cada host. Cada petición aparece en
wizard-s3-images-end = .
wizard-activity = Actividad
wizard-s3-tos = Usar el catálogo significa usar GameBanana: se aplican sus condiciones de uso, y GameBanana exige que sus usuarios tengan al menos 13 años.
wizard-tos = Condiciones de uso de GameBanana
wizard-choice = ¿Cómo quieres empezar?
wizard-use-catalog = Usar el catálogo
wizard-use-catalog-text = Explora e instala mods de GameBanana. deadloader solo se conecta a los hosts de arriba.
wizard-local = Ahora no: modo local
wizard-local-text = Ninguna petición de red; mods solo desde el disco. Puedes activar el catálogo más tarde en la configuración.
wizard-choice-error = Elige una de las dos opciones.
wizard-choice-reason = Primero elige una opción.
wizard-last-time = Tu elección anterior
wizard-changed-title = Qué cambió ({ $date })
wizard-changed-foot = Confirmaste los demás pasos el { $date }; no cambiaron.

# Step 4
wizard-s4-title = Tu privacidad
wizard-p1 = Sin cuenta y sin telemetría. deadloader no envía nada a ninguna parte, salvo las peticiones a GameBanana del paso anterior.
wizard-p1-local = Sin cuenta y sin telemetría. Elegiste el modo local, así que deadloader no envía absolutamente nada.
wizard-p2a = Los registros se quedan en esta PC, en la carpeta
wizard-p2b = — y ocultan tu nombre de usuario de Windows:
wizard-p3 = La ventana de deadloader la dibuja Microsoft WebView2, parte de Windows. deadloader desactiva las funciones en línea de WebView2; Windows actualiza WebView2 por su cuenta, sin deadloader.
wizard-p4 = Siempre puedes ver lo que hizo deadloader en
wizard-p4-end = .

# Step 5
wizard-s5-title = Actualizaciones de deadloader
wizard-u1 = Al iniciarse, deadloader puede revisar en GitHub si hay una versión más nueva de sí mismo.
wizard-updates-label = Buscar nuevas versiones de deadloader en GitHub al iniciar
wizard-updates-on = Activado
wizard-updates-off = Desactivado (predeterminado)
wizard-updates-na = No disponible en modo local
wizard-u2a = Si está activado, deadloader hace una petición a
wizard-u2b = en cada inicio y te avisa si hay una versión nueva.
wizard-u3 = Actualizar siempre es manual: descargas tú la nueva versión desde GitHub. deadloader nunca se reemplaza a sí mismo.
wizard-u4 = No disponible: el modo local no hace ninguna petición de red. La opción estará disponible cuando actives el catálogo en la configuración.
wizard-u5 = Puedes cambiarlo más tarde en Configuración → General.

# Step 6
wizard-s6-title = Configuración
wizard-s6-lead = deadloader buscó el juego y revisó algunas cosas. Todavía no se cambió nada, y puedes cambiar todo esto más tarde en la configuración.
wizard-checking = Revisando…
wizard-topic-game = Juego
wizard-topic-exe = Ubicación de deadloader
wizard-topic-disk = Espacio en disco
wizard-topic-foreign = Carpeta de mods del juego
wizard-topic-gameinfo = Archivo del juego para mods
wizard-game-found = Deadlock encontrado
wizard-game-chosen = Carpeta elegida
wizard-change = Cambiar…
wizard-game-missing = No se encontró Deadlock
wizard-game-missing-text = Steam o el juego no están instalados, o la carpeta del juego se movió. Elige tú la carpeta del juego, ahora o más tarde en Configuración → Juego.
wizard-browse = Examinar…
wizard-browse-title = Elige la carpeta de Deadlock
wizard-help = Ayuda
wizard-help-a = En Steam: clic derecho en Deadlock → Administrar → Ver archivos locales. Elige esa carpeta; suele terminar en
wizard-help-b = .
wizard-exe-ok = En una carpeta permanente
wizard-exe-downloads = deadloader está en tu carpeta Descargas
wizard-exe-desktop = deadloader está en tu Escritorio
wizard-exe-warn-a = Sus datos están junto al exe, así que una limpieza eliminaría tu biblioteca y tus perfiles. Cierra deadloader, mueve toda la carpeta a un lugar permanente, por ejemplo
wizard-exe-warn-b = , y ábrelo desde ahí.
wizard-open-folder = Abrir carpeta
wizard-opened = Abierta en el Explorador de archivos
wizard-linked = Los mods se enlazan: sin espacio extra en disco
wizard-linked-text = deadloader y el juego están en la misma unidad ({ $drive }).
wizard-copied = Los mods se copiarán, no se enlazarán
wizard-copied-text = deadloader está en { $data } y el juego en { $game }. Cada mod activado ocupa entonces su tamaño una segunda vez; la biblioteca lo muestra como “copiado — +N MB”.
wizard-learn-more = Más información
wizard-drives-more = Un enlace permite al juego usar el archivo de la biblioteca sin una segunda copia. Windows solo puede enlazar archivos de la misma unidad. Si deadloader también está en { $game }, no se copia nada.
wizard-foreign-none = No hay archivos de mods ajenos
wizard-foreign-none-text = La carpeta de mods no contiene nada que deadloader no conozca.
wizard-foreign-some =
    { $count ->
        [one] { $count } archivo de otra fuente
       *[other] { $count } archivos de otra fuente
    }
wizard-foreign-some-text = No los puso deadloader: probablemente otro gestor de mods o una instalación manual. Los deja como están; revísalos más tarde en la biblioteca.
wizard-show-files = Mostrar archivos
wizard-hide-files = Ocultar archivos
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Se configurará
wizard-gi-setup-a = deadloader agrega una línea a
wizard-gi-setup-b = cuando aplicas mods por primera vez. Ahora no se cambia nada.
wizard-gi-ours = Ya configurado
wizard-gi-ours-text = La línea de deadloader ya está ahí.
wizard-gi-foreign = Cambiado por otro programa
wizard-gi-foreign-text = No parece el original; a menudo es cosa de Deadlock Mod Manager. deadloader no sobrescribe cambios ajenos y te explicará qué hacer después de la configuración.
wizard-what-is = ¿Qué es esto?
wizard-gi-more = es un archivo del juego que le dice a Deadlock qué carpetas cargar. Normalmente se arregla verificando los archivos del juego en Steam; después deadloader agrega su línea.

# E1
wizard-nw-title = deadloader no puede guardar sus datos.
wizard-nw-why-a = La carpeta junto a
wizard-nw-why-b = es de solo lectura: Windows no deja que los programas escriban ahí (por ejemplo, dentro de Program Files).
wizard-nw-path = Carpeta de deadloader
wizard-nw-do = Qué hacer
wizard-nw-s1 = Cierra deadloader.
wizard-nw-s2a = Mueve toda la carpeta a un lugar donde puedas escribir, por ejemplo
wizard-nw-s2b = . No uses Program Files.
wizard-nw-s3a = Inicia
wizard-nw-s3b = desde el nuevo lugar.
wizard-nw-why-folder = deadloader guarda todo junto al exe para no repartir nada por tu PC: eliminar la carpeta quita deadloader por completo.
wizard-nw-details = Detalles
wizard-nw-copy = Copiar detalles
wizard-nw-copied = Copiado
wizard-nw-no-log = No se guardó en el registro: deadloader no puede escribir ahí.
wizard-exit = Salir
