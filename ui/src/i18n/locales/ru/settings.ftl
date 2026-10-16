# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Настройки
settings-nav = Разделы настроек
settings-reset-nav = Сбросить все настройки…
settings-default = По умолчанию: { $value }
settings-restore = Вернуть значение по умолчанию
settings-on = Вкл.
settings-off = Выкл.
settings-cancel = Отмена
settings-remove = Убрать
settings-remove-item = Убрать { $item }
settings-add = Добавить
settings-open-folder = Открыть папку
settings-open-folder-at = Открыть папку: { $path }
settings-copied = Скопировано в буфер обмена.
settings-folder = Папка
settings-example = Пример:

settings-section-general = Общие
settings-section-general-lead = Язык, запуск и масштаб.
settings-section-game = Игра
settings-section-game-lead = Где установлен Deadlock и что deadloader может там менять.
settings-section-catalog = Каталог
settings-section-catalog-lead = Каталог GameBanana, деликатный контент, изображения и ссылки.
settings-section-library = Библиотека
settings-section-library-lead = Обновления модов, удаление модов и старые версии.
settings-section-downloads = Загрузки
settings-section-downloads-lead = Очередь загрузок, архивы и распаковка.
settings-section-network = Сеть
settings-section-network-lead = Как deadloader общается с GameBanana.
settings-section-cache = Кэш
settings-section-cache-lead = Миниатюры и страницы каталога, сохранённые на этом ПК.
settings-section-logs = Журналы
settings-section-logs-lead = Что хранит «Активность» и как долго.
settings-section-advanced = Дополнительно
settings-section-advanced-lead = Файлы настроек и дополнительные разрешённые домены.
settings-section-about = О deadloader
settings-section-about-lead = Версия, лицензия, проверка этой загрузки и где лежат ваши данные.

# General
settings-general-group = Язык и запуск
settings-language = Язык
settings-language-help = Язык системы: язык интерфейса Windows, если он есть в deadloader, иначе английский.
settings-language-system = Язык системы ({ $language })
settings-startup-profile = Профиль при запуске
settings-startup-profile-help = Какой профиль выбран при запуске deadloader. Выбор не применяет его.
settings-startup-last = Последний выбранный ({ $name })
settings-startup-last-plain = Последний выбранный
settings-app-updates = Проверять обновления deadloader при запуске
settings-app-updates-help = При запуске deadloader один раз спрашивает GitHub (api.github.com), есть ли более новая версия. Обновление всегда выполняется вручную. Вы выбрали это при первом запуске.
settings-app-updates-local = Недоступно: в локальном режиме нет никаких сетевых запросов. Параметр станет доступен, когда вы включите каталог (Настройки → Каталог).
settings-zoom = Масштаб
settings-zoom-keys-1 = Также с помощью Ctrl + колесо мыши,
settings-zoom-keys-2 = и
settings-zoom-keys-3 = для сброса.
settings-ctrl = Ctrl
settings-percent = { $value } %

# Game
settings-installs-group = Установки игры
settings-installs-lead = deadloader сам находит установки Deadlock в Steam. Каждый профиль использует одну из них — это меняется в разделе «Профили».
settings-install-library = Библиотека Steam { $library }
settings-install-build = сборка { $build }
settings-install-used =
    { $count ->
        [0] не используется ни одним профилем
        [one] используется { $count } профилем: { $names }
       *[other] используется профилями ({ $count }): { $names }
    }
settings-install-selected = { $name } (выбран)
settings-install-applied = { $name } (применён здесь)
settings-install-steam = Найдена через Steam
settings-install-manual = Добавлена вручную
settings-install-known = Здесь ещё есть файлы deadloader
settings-install-manual-meta = Добавлена вручную
settings-gi-ours = gameinfo.gi: в порядке — строки deadloader на месте
settings-gi-wait = gameinfo.gi: будет настроен, когда профиль начнёт её использовать
settings-gi-rolled = gameinfo.gi: исходный — «Применить» снова добавит строки deadloader
settings-gi-foreign = gameinfo.gi: изменён другой программой
settings-gi-missing = gameinfo.gi: не найден — возможно, папку переместили
settings-installs-none = Установки пока не найдены.
settings-add-manual = Добавить вручную…
settings-add-manual-hint = Выберите папку Deadlock, которая не была найдена.
settings-add-manual-title = Выберите папку Deadlock
settings-install-added = Установка добавлена.
settings-install-removed = Установка убрана из списка.
settings-install-removed-text = В той папке ничего не изменилось.
settings-gameinfo-group = После обновлений игры
settings-autofix = Исправлять gameinfo.gi автоматически
settings-autofix-help = Обновление Deadlock сбрасывает файл игры, который велит загружать моды. deadloader делает его резервную копию, возвращает свои строки и сообщает вам. Если выключено, сначала спросит баннер.
settings-danger-group = Отмена и удаление
settings-rollback = Откатить папку игры выбранного профиля
settings-rollback-help = Для установки выбранного профиля { $profile } ({ $drive }): удаляет файлы модов, которые туда положил deadloader ({ $files }), и его строки в gameinfo.gi. Библиотека и профили остаются; «Применить» всё вернёт.
settings-rollback-help-others = Другие папки игры не затрагиваются; чтобы откатить их, выберите профиль, который их использует.
settings-rollback-nothing = На { $drive } нечего откатывать — сейчас там нет файлов deadloader.
settings-rollback-no-game = Deadlock не найден, поэтому откатывать нечего.
settings-rollback-button = Откатить…
settings-rolled-title = Папка игры на { $drive } откачена.
settings-rolled-text =
    { $foreign ->
        [0] На { $drive } удалены файлы модов ({ $files }) и строки deadloader в gameinfo.gi. Библиотека и профили сохранены — «Применить» или «Играть» вернёт моды.
       *[other] На { $drive } удалены файлы модов ({ $files }), строки deadloader в gameinfo.gi и посторонние файлы ({ $foreign }). Библиотека и профили сохранены — «Применить» или «Играть» вернёт моды.
    }
settings-wipe = Удалить все данные deadloader
settings-wipe-help = Удаляет библиотеку, профили, журналы, кэш и настройки ({ $size }), затем закрывает deadloader. deadloader.exe остаётся — удалите его потом сами.
settings-wipe-button = Удалить все данные…

# Catalog
settings-catalog-group = Каталог GameBanana
settings-catalog = Использовать каталог
settings-catalog-help-on = Просмотр и установка модов с GameBanana. deadloader общается только с GameBanana, и каждый запрос виден в «Активности». Выключено — значит локальный режим.
settings-catalog-help-off = Выключено — локальный режим: deadloader не делает никаких сетевых запросов. Добавлять моды с диска по-прежнему можно.
settings-consent-title = Сначала один шаг.
settings-consent-text = Прежде чем включить каталог, deadloader покажет, к каким хостам он будет обращаться, и условия GameBanana.
settings-consent-go = Просмотреть и включить
settings-local-on = Локальный режим включён.
settings-local-on-text = Каталог выключен, и deadloader не делает сетевых запросов.
settings-ttl = Повторно использовать загруженные страницы каталога в течение
settings-ttl-help = В течение этого времени возврат к странице не обращается к GameBanana повторно.
settings-minutes = { $count } мин
settings-hours = { $count } ч
settings-prefetch = Загружать следующую страницу заранее
settings-prefetch-help = Когда вы прокрутили около 70 % результатов, загружаются следующие, чтобы сетка продолжалась.
settings-nsfw-group = Деликатный контент
settings-nsfw = Моды с возрастной маркировкой (NSFW)
settings-nsfw-help = Скрыты: не показываются вовсе. Размыты: размытое изображение с пометкой «Деликатный контент»; нажмите, чтобы показать. Показаны: без размытия.
settings-nsfw-age-done = Возраст подтверждён { $date }.
settings-nsfw-age-ask = При выходе из режима «Скрыты» вас один раз попросят подтвердить возраст.
settings-nsfw-hidden = Скрыты
settings-nsfw-blurred = Размыты
settings-nsfw-shown = Показаны
settings-images-group = Изображения и ссылки
settings-description-images = Изображения в описаниях модов
settings-description-images-help = Они приходят с хостов, отличных от GameBanana, например i.imgur.com. «По нажатию»: заглушка, пока вы на неё не нажмёте.
settings-images-click = По нажатию
settings-images-always = Всегда
settings-prechecked = Заранее отмечать «Всегда разрешать изображения с этого хоста»
settings-prechecked-help = В диалоге, который появляется, когда изображение в описании впервые приходит с нового хоста.
settings-hosts = Разрешённые хосты изображений
settings-hosts-help = Изображения с этих хостов загружаются без вопросов. В «Активности» их запросы помечены как «разрешённый хост изображений».
settings-hosts-built-in = Встроенный
settings-hosts-none = Хосты не добавлены — каждый новый хост сначала спрашивает.
settings-host-removed = { $host } убран
settings-host-removed-text = Его изображения снова будут спрашивать перед загрузкой.
settings-undo = Отменить
settings-gif = Воспроизводить GIF автоматически
settings-gif-help = Выключено: GIF показывают первый кадр с кнопкой воспроизведения.
settings-links = Внешние ссылки
settings-links-help = Ссылки открываются в браузере по умолчанию. «Сначала спросить» показывает полный адрес перед открытием.
settings-links-ask = Сначала спросить
settings-links-open = Открывать сразу

# Library
settings-updates-group = Обновления модов
settings-check-updates = Проверять обновления модов при запуске
settings-check-updates-help = Сравнивает ваши моды с GameBanana с каталогом. Никогда в локальном режиме.
settings-auto-download = Скачивать обновления автоматически
settings-auto-download-help = Только скачивание — deadloader всё равно спросит, прежде чем мод перейдёт на новую версию.
settings-disk-group = Удаление и старые версии
settings-deletion = При удалении мода
settings-deletion-help = В диалоге удаления можно каждый раз выбрать иначе.
settings-deletion-permanent = Безвозвратно
settings-deletion-bin = В корзину
settings-old-versions = Старые версии
settings-old-versions-help = Каждая скачанная версия сохраняется, чтобы можно было вернуться. Старые версии мода удаляются в его подробностях.
settings-old-keep-all = Хранить все

# Downloads
settings-queue-group = Очередь
settings-parallel = Одновременные загрузки
settings-parallel-help = Сколько файлов скачивается одновременно.
settings-speed = Ограничение скорости
settings-speed-help = Только для загрузки модов — каталог и миниатюры не ограничиваются.
settings-speed-none = Без ограничения
settings-speed-value = { $value } МБ/с
settings-keep-archives = Сохранять архивы после установки
settings-keep-archives-help = Выключено: файл .zip, .7z или .rar удаляется, как только его файлы .vpk попадают в библиотеку.
settings-unpack-group = Распаковка
settings-limits = Ограничения распаковки
settings-limits-help = Защищают от архивов, которые распаковываются во что-то огромное. Если архив, которому вы доверяете, отклонён, увеличьте ограничение.
settings-limits-default = 2 ГБ · 20× · 256
settings-limit-size = Размер после распаковки
settings-limit-ratio = Степень сжатия
settings-limit-files = Файлов модов в архиве
settings-unit-gb = ГБ
settings-unit-x = ×
settings-unit-files = файлов
settings-unit-s = с
settings-number-error = { $field }: введите целое число от { $min } до { $max }. До тех пор используется прежнее значение.

# Network
settings-network-local = Локальный режим включён.
settings-network-local-text = deadloader не делает сетевых запросов; эти настройки начнут действовать, когда каталог будет включён.
settings-requests-group = Запросы к GameBanana
settings-api-parallel = Одновременные запросы к каталогу
settings-api-parallel-help = Одновременные запросы к API GameBanana. Изображения и загрузки считаются отдельно.
settings-timeouts = Тайм-ауты
settings-timeouts-help = Подключение: сдаться, если соединение не установлено за это время. Ответ: сдаться, когда сервер перестаёт присылать данные.
settings-timeouts-default = 10 с · 30 с
settings-timeout-connect = Подключение
settings-timeout-read = Ответ
settings-retries = Повторы
settings-retries-help = Сколько раз неудавшийся запрос повторяется с растущими паузами, прежде чем deadloader покажет ошибку.
settings-connection-group = Подключение
settings-proxy = Прокси
settings-proxy-help = Нет: прямое подключение. Системный: прокси, заданный в Windows. Вручную: укажите ниже.
settings-proxy-none = Нет
settings-proxy-system = Системный
settings-proxy-manual = Вручную
settings-proxy-host = Адрес
settings-proxy-port = Порт
settings-proxy-hint = HTTP-прокси. Через него идут только запросы deadloader.
settings-proxy-invalid = Введите имя хоста или адрес без имени пользователя и пароля, например proxy.example.com.

# Cache
settings-thumbs-group = Миниатюры и страницы каталога
settings-cache-limit = Предел кэша
settings-cache-limit-help = Когда кэш заполнен, первыми удаляются миниатюры, которые вы дольше всего не видели.
settings-size-mb = { $value } МБ
settings-size-gb = { $value } ГБ
settings-cache-now = Текущий размер
settings-cache-breakdown = Миниатюр: { $thumbnails } · страниц каталога: { $pages }
settings-cache-empty = Пусто — в кэше пока ничего нет.
settings-cache-used = { $used } из { $limit }
settings-cache-clear = Очистить кэш
settings-cache-cleared = Кэш очищен — освобождено { $size }. Миниатюры загрузятся снова, когда понадобятся.
settings-cache-cleared-partly = Файлы, которые были заняты ({ $failed }), останутся до следующей очистки.

# Logs
settings-keep-group = Хранение журналов
settings-log-days = Хранить журналы
settings-log-days-help = Более старые записи удаляются при запуске deadloader.
settings-days = { $count ->
        [one] { $count } день
        [few] { $count } дня
        [many] { $count } дней
       *[other] { $count } дня
    }
settings-year = 1 год
settings-log-size = Максимальный размер
settings-log-size-help = Сверх этого размера первыми удаляются самые старые файлы журнала.
settings-privacy-group = Конфиденциальность
settings-mask = Скрывать имя пользователя Windows
settings-mask-help = Пути в журналах показывают { $masked } вместо вашего имени — так безопаснее, если вы делитесь журналом.
settings-logs-folder-group = Папка журналов
settings-logs-folder = Где лежат журналы
settings-logs-now = { $size } · файлов: { $files } · самый старый от { $date }
settings-open-logs = Открыть папку журналов

# Advanced
settings-advanced-warning = Для опытных пользователей.
settings-advanced-warning-text = Ошибка здесь может изменить поведение deadloader. Он проверяет файл перед сохранением и хранит резервную копию прежней версии.
settings-files-group = Редактирование файлов настроек
settings-file-settings = Настройки
settings-file-manifest = Манифест папки игры
settings-file-label = Файл для редактирования
settings-file-settings-lead = Всё с этих страниц в одном файле. Изменения появятся на страницах после сохранения.
settings-file-manifest-lead = Список того, что deadloader положил в папку игры. Откат опирается на него — неверная запись может оставить файлы.
settings-json-error = Это не корректный JSON, поэтому он не сохранён.
settings-json-where = Строка { $line }, столбец { $column }: здесь или чуть раньше чего-то не хватает или есть лишнее — часто запятая или кавычка.
settings-json-fix = Исправьте это место и сохраните снова — до тех пор deadloader использует текущий файл.
settings-json-refused = Эти значения нельзя использовать, поэтому ничего не сохранено.
settings-json-newer = Файл создан более новой версией deadloader (формат { $found }; эта версия читает до { $supported }), поэтому он не сохранён.
settings-json-invalid = deadloader не может использовать этот файл, поэтому он не сохранён.
settings-json-details = Подробности
settings-problem-unknown-field = { $path }: такой настройки нет в этой версии.
settings-problem-invalid-value = { $path }: это значение здесь недопустимо.
settings-problem-out-of-range = { $path }: { $found } вне диапазона { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: «{ $entry }» здесь недопустимо.
settings-problem-duplicate-entry = { $path }: «{ $entry }» указано дважды.
settings-problem-needs-age = { $path }: для показа NSFW-контента сначала нужно подтвердить возраст (Настройки → Каталог).
settings-problem-local-mode = { $path }: недоступно в локальном режиме.
settings-json-idle = Проверяется при сохранении.
settings-json-saved = Корректно и сохранено. Прежняя версия сохранена как { $backup }.
settings-json-saved-plain = Корректно и сохранено.
settings-json-save = Проверить и сохранить
settings-json-discard = Отменить мои правки
settings-json-backup = Перед сохранением прежний файл сохраняется как резервная копия; хранятся последние 10.
settings-domains-group = Дополнительные разрешённые домены
settings-domains-lead = deadloader общается только с GameBanana. Если загрузку перенаправили на другой хост и она заблокирована, этот хост можно разрешить здесь. В «Активности» запросы к нему помечены как «добавлено пользователем».
settings-domains-built-in = Встроенные:
settings-domain-user = добавлено пользователем
settings-domains-none = Дополнительных доменов нет.
settings-domain-add = Добавить домен
settings-domain-hint = Только имя хоста, например files.example.com.
settings-domain-empty = Сначала введите имя хоста, например files.example.com.
settings-domain-not-host = Введите только имя хоста — без https:// и пути, например files.example.com.
settings-domain-invalid = «{ $host }» — не имя хоста. Проверьте написание, например files.example.com.
settings-domain-built-in = { $host } встроен и всегда разрешён.
settings-domain-listed = { $host } уже в списке.

# About
settings-version = версия { $version }
settings-commit = Собрано из коммита
settings-commit-local = Локальная сборка — не собрана GitHub Actions
settings-copy-commit = Скопировать ID коммита
settings-release-page = Страница выпуска
settings-licence-group = Лицензия
settings-licence-help = deadloader — свободная программа: вы можете использовать, изучать, распространять и изменять её на условиях GPL-3. Она поставляется без каких-либо гарантий.
settings-read-licence = Прочитать лицензию
settings-source = Исходный код
settings-verify-group = Как проверить эту загрузку
settings-verify-lead = Две независимые проверки того, что этот deadloader.exe — тот, что опубликован на GitHub.
settings-verify-hash = Сравнить контрольную сумму SHA-256
settings-verify-hash-help = Она должна совпадать с суммой на странице выпуска. Изменённая копия могла бы показать здесь ложное значение, поэтому вычислите её и сами:
settings-verify-hash-cmd = В PowerShell, в папке с exe:
settings-verify-attest = Проверить аттестацию сборки
settings-verify-attest-help = Доказывает, что GitHub Actions собрал этот файл из опубликованного исходного кода. Нужен GitHub CLI:
settings-copy-hash = Скопировать сумму
settings-copy-command = Скопировать команду
settings-unsigned = deadloader.exe не подписан.
settings-unsigned-text = При первом запуске Windows SmartScreen может сообщить «Система Windows защитила ваш компьютер». Две проверки выше позволяют убедиться, что файл подлинный.
settings-webview-group = Движок окна
settings-webview-help = Окно deadloader рисует WebView2 — часть Windows. Windows обновляет его сама. deadloader отключает:
settings-webview-off-1 = Запросы DNS-over-HTTPS
settings-webview-off-2 = Вход в учётную запись Microsoft внутри окна
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Фоновые сетевые обращения и обновления компонентов
settings-webview-off-5 = Обращения к серверу автозаполнения форм
settings-webview-off-6 = Отчёты об ошибках в Microsoft (domain reliability)
settings-webview-no-route = И у окна вообще нет выхода в интернет.
settings-webview-unknown = версия неизвестна
settings-data-group = Ваши данные
settings-data-folder = Папка данных
settings-data-folder-help = Всё, что хранит deadloader, лежит в этой одной папке рядом с exe.
settings-notice = Уведомление при первом запуске
settings-notice-help = Версия { $version }, принято { $date }, язык: { $language }. Ваш выбор: каталог — { $catalog }, проверка обновлений — { $updates }.
settings-notice-none = Ещё не принято.
settings-notice-again = Прочитать снова

# D7
settings-d7-title = Откатить папку игры на { $drive }?
settings-d7-from =
    { $others ->
        [0] Установка выбранного профиля { $profile }. deadloader удалит из
       *[other] Установка выбранного профиля { $profile } (её также использует { $others }). deadloader удалит из
    }
settings-d7-files =
    { $count ->
        [one] { $count } файл мода, который он добавил в
        [few] { $count } файла модов, которые он добавил в
        [many] { $count } файлов модов, которые он добавил в
       *[other] { $count } файла модов, которые он добавил в
    }
settings-d7-line = Свои строки в gameinfo.gi
settings-d7-line-why = — файл станет таким, каким его установил Steam.
settings-d7-stays = Останется как есть:
settings-d7-library = Ваша библиотека — модов: { $mods }, { $size }
settings-d7-profiles =
    { $count ->
        [one] Ваш профиль — { $names }
       *[other] Ваши профили ({ $count }) — { $names }
    }
settings-d7-other = Установка на { $path } ({ $profile }) — не затрагивается. Чтобы откатить её, выберите профиль, который её использует, и откатите снова.
settings-d7-other-plain = Установка на { $path } — не затрагивается.
settings-d7-back = «Применить» или «Играть» всё вернёт.
settings-d7-foreign =
    { $count ->
        [one] Удалить также { $count } файл, созданный не deadloader
        [few] Удалить также { $count } файла, созданных не deadloader
        [many] Удалить также { $count } файлов, созданных не deadloader
       *[other] Удалить также { $count } файла, созданных не deadloader
    }
settings-d7-foreign-why = от другой программы или ручной установки. Они удаляются безвозвратно; deadloader не сможет их вернуть.
settings-d7-confirm = Откатить
settings-d7-confirm-foreign =
    { $count ->
        [one] Откатить и удалить { $count } файл
        [few] Откатить и удалить { $count } файла
        [many] Откатить и удалить { $count } файлов
       *[other] Откатить и удалить { $count } файла
    }

# D8
settings-d8-title = Удалить все данные deadloader?
settings-d8-from = deadloader удалит всё в
settings-d8-from-2 = и затем закроется.
settings-d8-what = Что
settings-d8-contains = Содержит
settings-d8-size = Размер
settings-d8-total = Всего
settings-d8-library = Библиотека
settings-d8-library-has = модов: { $mods }, файлов: { $files }
settings-d8-profiles = Профили
settings-d8-logs = Журналы
settings-d8-logs-has = файлов: { $files }, с { $date }
settings-d8-logs-none = нет файлов журнала
settings-d8-cache = Кэш
settings-d8-cache-has = миниатюры и страницы каталога
settings-d8-settings = Настройки
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
       *[other] вкл. резервные копии ({ $backups })
    }
settings-d8-other = Прочее
settings-d8-other-has = незавершённые загрузки, копии gameinfo.gi, данные движка окна
settings-d8-exe = Не удаляется: сам deadloader.exe — удалите его сами, если он больше не нужен.
settings-d8-rollback = Сначала откатить все папки игры, которые использует deadloader (рекомендуется)
settings-d8-rollback-why = Перед удалением убирает файлы модов deadloader и его строки в gameinfo.gi из каждой папки игры. Посторонние файлы остаются.
settings-d8-rollback-off = Не отмечено: файлы модов deadloader останутся в папках игры, и Deadlock продолжит их загружать — а убрать их будет уже некому.
settings-d8-type-1 = Для подтверждения введите
settings-d8-type-2 = { "" }
settings-d8-word = удалить
settings-d8-hint = Кнопка удаления станет доступна, когда слово совпадёт.
settings-d8-hint-ok = Подтверждено. Это нельзя отменить.
settings-d8-type-first = Сначала введите «{ $word }»
settings-d8-deleting = Удаление…
settings-d8-confirm = Удалить и закрыть
settings-d8-confirm-rollback = Откатить, удалить и закрыть

# Reset
settings-reset-title = Сбросить все настройки?
settings-reset-body = Каждая настройка вернётся к значению по умолчанию, включая разрешённые хосты изображений и дополнительные домены.
settings-reset-keeps = Библиотека, профили и журналы не затрагиваются, папка игры остаётся как есть.
settings-reset-backup = Резервная копия текущих настроек сохраняется как settings.json.bak-…
settings-reset-confirm = Сбросить настройки
settings-reset-done = Все настройки сброшены.
settings-reset-done-text = Резервная копия: { $backup }
