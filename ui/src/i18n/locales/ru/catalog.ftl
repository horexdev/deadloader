# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Открыть эту ссылку в браузере?
catalog-link-body = Ссылка откроется не в deadloader, а в вашем браузере по умолчанию.
catalog-link-always = Всегда открывать ссылки сразу
catalog-link-cancel = Отмена
catalog-link-open = Открыть

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Скрыты
catalog-nsfw-blurred = Размыты
catalog-nsfw-shown = Показаны
catalog-age-title = Показывать моды с откровенным содержимым?
catalog-age-body = Некоторые моды на GameBanana отмечены за наготу, сексуальные темы и подобное. Чтобы видеть их, подтвердите, что вам есть 18 лет или вы достигли совершеннолетия по законам вашей страны.
catalog-age-choice = Ваш выбор: { $mode }.
catalog-age-once = deadloader спрашивает об этом один раз. Вернуться к «Скрыты» можно в любой момент.
catalog-age-confirm = Мне есть 18
catalog-age-cancel = Отмена

## S2: the catalog grid
catalog-title = Каталог
catalog-section = Раздел
catalog-section-mods = Моды
catalog-section-sounds = Звуки
catalog-search = Поиск модов
catalog-search-sounds = Поиск звуков
catalog-search-offline = Для поиска нужно подключение
catalog-search-off = Поиск недоступен — каталог не читается
catalog-search-clear = Очистить поиск
catalog-results =
    { $count ->
        [one] { $count } результат
        [few] { $count } результата
       *[other] { $count } результатов
    }
catalog-sort = Сортировка
catalog-sort-newest = Новые
catalog-sort-latest_updated = Недавно обновлённые
catalog-sort-most_downloaded = Больше всего загрузок
catalog-sort-most_liked = Больше всего лайков
catalog-sort-most_viewed = Больше всего просмотров
catalog-sort-most_commented = Больше всего комментариев
catalog-sort-oldest = Старые
catalog-sort-best = Лучшее совпадение
catalog-sort-best-title = Во время поиска результаты сортируются по совпадению
catalog-category = Категория
catalog-category-all = Все категории
catalog-hero = Герой
catalog-hero-all = Все герои
catalog-hero-filter = Найти героя
catalog-hero-none = Такого героя нет
catalog-filters = Фильтры
catalog-filter-nsfw = Моды с возрастными пометками (NSFW)
catalog-filter-favourites = Только избранное
catalog-filter-favourites-short = только избранное
catalog-filter-installed = Скрыть установленные
catalog-filter-installed-short = без установленных
catalog-nsfw-caption-hidden = NSFW скрыты
catalog-nsfw-caption-blurred = NSFW размыты · нажмите на карточку, чтобы показать
catalog-nsfw-caption-shown = NSFW показаны
catalog-loading = Загрузка модов с GameBanana…
catalog-loading-sounds = Загрузка звуков с GameBanana…
catalog-loading-more = Загрузка…
catalog-more-failed = Не удалось загрузить следующую страницу.
catalog-grid = Моды
catalog-grid-sounds = Звуки
catalog-likes =
    { $count ->
        [one] { $count } лайк
        [few] { $count } лайка
       *[other] { $count } лайков
    }
catalog-favourite-add = Добавить в избранное
catalog-favourite-remove = Убрать из избранного
catalog-sensitive = Откровенное содержимое
catalog-click-show = Нажмите, чтобы показать
catalog-badge-installed = Установлен
catalog-badge-update = Обновление
catalog-card-installing = Установка · { $percent } %
catalog-empty = Здесь пока нет модов
catalog-empty-search = Нет модов по запросу «{ $text }»
catalog-empty-favourites = В избранном пока пусто
catalog-empty-filters = Включены фильтры: { $filters }. Сбросьте их, чтобы искать по всем модам, или попробуйте слово короче.
catalog-empty-shorter = Попробуйте более короткое или другое слово.
catalog-clear-filters = Сбросить фильтры
catalog-rate-retry = deadloader повторит попытку через { $seconds } с — моды ниже по-прежнему доступны.
catalog-retry-now = Повторить сейчас
catalog-try-again = Повторить
catalog-check-update = Проверить обновление deadloader
catalog-details = Подробности · запись журнала
catalog-copy-id = Скопировать номер записи
catalog-open-activity = Открыть в «Активности»
catalog-offline-empty-title = Нет подключения — показать нечего
catalog-offline-empty-text = deadloader не может связаться с GameBanana, а эта часть каталога раньше не сохранялась.
catalog-error-title = Каталог сейчас не удаётся показать.
catalog-local-title = Каталог выключен (локальный режим)
catalog-local-why = В локальном режиме deadloader не делает никаких сетевых запросов. Библиотека, профили и запуск игры работают как обычно, а моды можно добавлять с диска.
catalog-local-do = Включите каталог, чтобы искать и устанавливать моды с GameBanana. Тогда deadloader будет обращаться только к GameBanana.
catalog-local-enable = Включить каталог
catalog-local-library = Перейти в библиотеку

## S3: the mod page
catalog-breadcrumb = Навигация
catalog-back = Назад в каталог
catalog-page-loading = Загрузка страницы мода с GameBanana…
catalog-page-offline = Нет подключения — показана копия, сохранённая { $date }.
catalog-page-offline-why = Для установки новых файлов нужно подключение; переключаться между скачанными вариантами можно и сейчас.
catalog-page-offline-empty-title = Нет подключения — эта страница не сохранена
catalog-page-offline-empty-text = deadloader не может связаться с GameBanana, а эту страницу раньше не открывали.
catalog-page-local-title = Эта страница мода не сохранена
catalog-page-local-text = В локальном режиме deadloader показывает только сохранённые ранее страницы модов.
catalog-removed-title = Этот мод удалён с GameBanana
catalog-removed-why = Его скрыли или удалили, поэтому deadloader не может показать его страницу и скачать файлы.
catalog-removed-keep = Установленная копия ({ $copy }) остаётся в библиотеке и продолжает работать — просто больше не будет обновляться.
catalog-show-in-library = Показать в библиотеке
catalog-installed = Установлен
catalog-by = автор: { $author }
catalog-version = Версия { $version }
catalog-views =
    { $count ->
        [one] { $count } просмотр
        [few] { $count } просмотра
       *[other] { $count } просмотров
    }
catalog-downloads =
    { $count ->
        [one] { $count } загрузка
        [few] { $count } загрузки
       *[other] { $count } загрузок
    }
catalog-added = Добавлен { $when }
catalog-updated = Обновлён { $when }
catalog-favourite = В избранное
catalog-open-gamebanana = Открыть на GameBanana
catalog-gallery-alt = { $name }, изображение { $index }
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Предыдущее изображение
catalog-gallery-next = Следующее изображение
catalog-gallery-original = Оригинал
catalog-gallery-hidden = Изображения модов с возрастными пометками скрыты.
catalog-audio-preview = У этого звука есть прослушивание на GameBanana.
catalog-description = Описание
catalog-description-more = Показать описание целиком
catalog-description-less = Свернуть
catalog-description-truncated = Показано только начало описания.
catalog-opens-browser = откроется в браузере
catalog-opens-here = Откроется в deadloader
catalog-image-from = Изображение с
catalog-image-click = нажмите, чтобы загрузить
catalog-image-failed = не удалось загрузить
catalog-image-alt = Изображение из описания
catalog-youtube = Видео на YouTube
catalog-spoiler = Спойлер — нажмите, чтобы показать
catalog-spoiler-hide = Скрыть спойлер
catalog-files = Файлы
catalog-files-clean =
    { $count ->
        [one] Файл прошёл антивирусную проверку GameBanana
       *[other] Все файлы ({ $count }) прошли антивирусную проверку GameBanana
    }
catalog-files-clean-dated =
    { $count ->
        [one] Файл прошёл антивирусную проверку GameBanana · загружен { $date }
       *[other] Все файлы ({ $count }) прошли антивирусную проверку GameBanana · загружены { $date }
    }
catalog-files-av = Результат антивирусной проверки GameBanana указан у каждого файла.
catalog-variants-note = Эти файлы меняют одни и те же файлы игры, поэтому это варианты — активен только один.
catalog-no-files = У этого мода нет файлов для скачивания.
catalog-older-files = Старые файлы ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } загрузка
        [few] { $count } загрузки
       *[other] { $count } загрузок
    }
catalog-file-active = активный вариант
catalog-file-downloaded = скачан
catalog-file-install = Установить
catalog-file-install-label = Установить { $file } и сделать активным вариантом
catalog-file-install-title = Скачивает этот файл и делает его активным вариантом
catalog-file-install-offline = Для установки нужно подключение
catalog-file-installing = Установка…
catalog-file-installed = Установлен
catalog-file-switch = Переключить
catalog-file-switch-label = Переключиться на { $file }
catalog-file-switch-title = Уже скачан — сделать активным вариантом
catalog-file-update = Обновить
catalog-file-update-title = Новая версия заменяет установленный файл — посмотрите её в библиотеке
catalog-av-clean = Чисто
catalog-av-clean-title = Антивирус GameBanana: угроз нет
catalog-av-flagged = Есть замечания
catalog-av-flagged-title = Антивирус GameBanana: { $verdict }
catalog-av-pending = Ещё не проверен
catalog-av-pending-title = GameBanana ещё не закончил проверку этого файла
catalog-av-none = Нет проверки
catalog-av-none-title = GameBanana не прислал результат проверки для этого файла
catalog-credits = Авторы
catalog-no-ratings = Нет возрастных пометок
catalog-no-licence = Нет текста лицензии
catalog-no-sources = Нет других источников
catalog-rating-sexual_themes = Сексуальные темы
catalog-rating-sexual_content = Сексуальное содержимое
catalog-rating-skimpy_attire = Откровенная одежда
catalog-rating-lewd_angles_and_poses = Откровенные ракурсы и позы
catalog-rating-full_nudity = Полная нагота
catalog-licence = Лицензия
catalog-sources = Другие источники
catalog-videos = Видео

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Загрузить изображение с нового сайта?
catalog-image-body-before = deadloader скачает это изображение с
catalog-image-body-after = . Это не GameBanana — запрос появится в «Активности», как и все остальные.
catalog-image-note = Размер изображений ограничен. Разрешённые сайты можно удалить в «Настройки → Каталог».
catalog-image-always = Всегда разрешать изображения с этого сайта
catalog-image-cancel = Отмена
catalog-image-load = Загрузить изображение
