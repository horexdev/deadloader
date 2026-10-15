# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Активность
activity-sub =
    { $days ->
        [one] Всё, что deadloader сделал на этом ПК: сетевые запросы, изменения файлов и события приложения. Журналы хранятся на этом ПК { $days } день.
        [few] Всё, что deadloader сделал на этом ПК: сетевые запросы, изменения файлов и события приложения. Журналы хранятся на этом ПК { $days } дня.
        [many] Всё, что deadloader сделал на этом ПК: сетевые запросы, изменения файлов и события приложения. Журналы хранятся на этом ПК { $days } дней.
       *[other] Всё, что deadloader сделал на этом ПК: сетевые запросы, изменения файлов и события приложения. Журналы хранятся на этом ПК { $days } дня.
    }
activity-open-logs = Открыть папку журналов
activity-logs-opened = Папка журналов открыта в Проводнике.
activity-summary = Сегодня вкратце
activity-net-today = Сеть сегодня
activity-hosts-label = Хосты, к которым обращались сегодня, — выберите, чтобы отфильтровать
activity-hosts-contacted =
    { $count ->
        [one] { $count } хост
        [few] { $count } хоста
        [many] { $count } хостов
       *[other] { $count } хоста
    }
activity-host-title =
    { $count ->
        [one] { $count } запрос · { $source } — выберите для фильтра
        [few] { $count } запроса · { $source } — выберите для фильтра
        [many] { $count } запросов · { $source } — выберите для фильтра
       *[other] { $count } запроса · { $source } — выберите для фильтра
    }
activity-net-none-local = нет — локальный режим
activity-net-none-yet = пока нет
activity-nothing-else = Больше ничего.
activity-blocked-lead =
    { $count ->
        [one] { $count } запрос к
        [few] { $count } запроса к
        [many] { $count } запросов к
       *[other] { $count } запроса к
    }
activity-blocked-tail =
    { $count ->
        [one] заблокирован до установки соединения.
        [few] заблокированы до установки соединения.
        [many] заблокированы до установки соединения.
       *[other] заблокированы до установки соединения.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } запрос заблокирован, соединения не было.
        [few] { $count } запроса заблокированы, соединения не было.
        [many] { $count } запросов заблокированы, соединения не было.
       *[other] { $count } запроса заблокированы, соединения не было.
    }
activity-show-entry = Показать запись
activity-game-today = Папка игры сегодня
activity-show-in-files = Показать в «Файлах»
activity-game-none = без изменений
activity-no-game-changes = deadloader пока ничего не менял в папке игры.
activity-game-changes =
    { $count ->
        [one] { $count } изменение
        [few] { $count } изменения
        [many] { $count } изменений
       *[other] { $count } изменения
    }
activity-game-batches =
    { $count ->
        [one] за { $count } раз
        [few] за { $count } раза
        [many] за { $count } раз
       *[other] за { $count } раза
    }
activity-game-failed =
    { $count ->
        [one] { $count } с ошибкой
        [few] { $count } с ошибкой
        [many] { $count } с ошибкой
       *[other] { $count } с ошибкой
    }
activity-game-hardlink = связано: { $count }
activity-game-copy = скопировано: { $count }
activity-game-delete = удалено: { $count }
activity-game-rename = переименовано: { $count }
activity-game-write = записано: { $count }
activity-game-backup = сохранено в резервную копию: { $count }
activity-game-restore = восстановлено: { $count }
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi обновлён { $count } раз
        [few] gameinfo.gi обновлён { $count } раза
        [many] gameinfo.gi обновлён { $count } раз
       *[other] gameinfo.gi обновлён { $count } раза
    }
activity-no-requests-today = Сегодня запросов не было.
activity-no-requests-yet = Запросов пока не было.
activity-local-short = Каталог выключен, поэтому deadloader ни к чему не подключается.
activity-never-contacted = С момента установки deadloader не обращался ни к одному хосту.
activity-tab-network = Сеть
activity-tab-files = Файлы
activity-tab-app = Приложение
activity-tab-off = выкл.
activity-tab-errors =
    { $count ->
        [one] · { $count } ошибка
        [few] · { $count } ошибки
        [many] · { $count } ошибок
       *[other] · { $count } ошибки
    }
activity-search = Поиск по журналу
activity-search-clear = Очистить поиск
activity-matches =
    { $count ->
        [one] { $count } совпадение
        [few] { $count } совпадения
        [many] { $count } совпадений
       *[other] { $count } совпадения
    }
activity-matches-more = совпадений: { $count }+
activity-f-time = Период
activity-f-purpose = Назначение
activity-f-host = Хост
activity-f-op = Операция
activity-f-level = Уровень
activity-f-result = Результат
activity-all = Все
activity-ok = OK
activity-errors = Ошибки
activity-range-hour = Последний час
activity-range-today = Сегодня
activity-range-7d = Последние 7 дней
activity-range-30d = Последние 30 дней
activity-clear-filters = Сбросить фильтры
activity-clear-all = Сбросить поиск и фильтры
activity-follow = Следить в реальном времени
activity-follow-tip = Показывать новые записи сверху сразу, как они появляются
activity-follow-on = Новые записи появляются сверху
activity-follow-paused = Приостановлено в { $time }
activity-show-new =
    { $count ->
        [one] Показать { $count } новую
        [few] Показать { $count } новые
        [many] Показать { $count } новых
       *[other] Показать { $count } новые
    }
activity-net-aria = Сетевые запросы, сначала новые
activity-files-aria = Файловые операции, сначала новые
activity-app-aria = События приложения, сначала новые
activity-c-redirects = Перенаправления
activity-c-time = Время
activity-c-purpose = Назначение
activity-c-request = Запрос
activity-c-status = Статус
activity-c-size = Размер
activity-c-duration = Длительность
activity-c-started-by = Инициатор
activity-c-source = Источник хоста
activity-c-op = Операция
activity-c-from-to = Откуда → куда
activity-c-result = Результат
activity-c-change = Часть изменения
activity-c-level = Уровень
activity-c-message = Сообщение
activity-to = в
activity-loading-more = Загрузка…
activity-p-catalog = Каталог
activity-p-thumbnail = Миниатюра
activity-p-description-image = Изображение в описании
activity-p-download = Загрузка
activity-p-update-check = Проверка обновлений
activity-src-builtin = Встроенный
activity-src-github = GitHub
activity-src-user = Добавлен вами
activity-src-user-image = Хост изображений
activity-src-blocked = Не разрешён
activity-src-long-builtin = Встроенный — GameBanana, часть deadloader
activity-src-long-github = GitHub — собственная проверка обновлений deadloader, включена в настройках
activity-src-long-user = Добавлен вами — в разделе «Настройки → Дополнительно»
activity-src-long-user-image = Разрешённый хост изображений — вы разрешили его при загрузке изображения в описании
activity-src-long-blocked = Нет в списке разрешённых хостов
activity-by-auto = Авто
activity-by-user = Вы
activity-op-hardlink = Связь
activity-op-copy = Копирование
activity-op-rename = Переименование
activity-op-delete = Удаление
activity-op-write = Запись
activity-op-backup = Резервная копия
activity-op-restore = Восстановление
activity-lv-debug = Отладка
activity-lv-info = Инфо
activity-lv-warn = Предупреждение
activity-lv-error = Ошибка
activity-st-blocked = Заблокирован
activity-st-blocked-long = Заблокирован — соединения не было
activity-st-failed = Сбой
activity-redirects =
    { $count ->
        [one] { $count } перенаправление
        [few] { $count } перенаправления
        [many] { $count } перенаправлений
       *[other] { $count } перенаправления
    }
activity-retries =
    { $count ->
        [one] { $count } повтор
        [few] { $count } повтора
        [many] { $count } повторов
       *[other] { $count } повтора
    }
activity-show-redirects = Показать перенаправления
activity-hide-redirects = Скрыть перенаправления
activity-hop-request = Шаг 1 · запрос
activity-hop-redirect = Шаг { $n } · перенаправление
activity-hop-retry = Шаг { $n } · попытка { $try }
activity-ms = { $ms } мс
activity-seconds = { $s } с
activity-no-match = Подходящих записей нет
activity-no-match-text = Выберите другой период или сбросьте фильтры и поиск.
activity-empty-title = Пока ничего
activity-empty-text = Здесь deadloader записывает каждый сетевой запрос, каждое изменение файлов в папке игры и каждое событие приложения — сразу, как они происходят. Пока ничего не произошло.
activity-local-title = Локальный режим — запросы не выполняются
activity-local-why = Каталог выключен, поэтому deadloader ни к чему не подключается — ни к GameBanana, ни куда-либо ещё. Изменения файлов и события приложения по-прежнему записываются на других вкладках.
activity-local-do = Включите каталог в настройках, чтобы просматривать GameBanana. Тогда каждый запрос появится здесь.
activity-enable-catalog = Включить каталог
activity-details = Подробности записи журнала
activity-close-details = Закрыть подробности
activity-close-esc = Закрыть (Esc)
activity-why = Почему:
activity-todo = Что делать:
activity-open-advanced = Открыть «Настройки → Дополнительно»
activity-full-record = Полная запись
activity-hops-title = Перенаправления · { $count }
activity-user-hidden = Ваше имя пользователя Windows в журналах скрыто:
activity-settings-logs = Настройки → Журналы
activity-copy = Копировать запись
activity-copied = Скопировано
activity-kind-network = Сетевой запрос
activity-kind-files = Файловая операция
activity-kind-app = Событие приложения
activity-d-time = Время
activity-d-purpose = Назначение
activity-d-started-by = Инициатор
activity-d-request = Запрос
activity-d-source = Источник хоста
activity-d-status = Статус
activity-d-size = Размер
activity-d-duration = Длительность
activity-d-range = Диапазон
activity-d-error = Ошибка
activity-d-id = ID записи
activity-d-request-id = ID запроса
activity-d-operation = Операция
activity-d-from = Откуда
activity-d-to = Куда
activity-d-file = Файл
activity-d-result = Результат
activity-d-change = Часть изменения
activity-d-sha256 = SHA-256
activity-d-level = Уровень
activity-d-message = Сообщение
activity-d-fields = Подробности
activity-status-after =
    { $count ->
        [one] { $status } — после { $count } перенаправления
        [few] { $status } — после { $count } перенаправлений
        [many] { $status } — после { $count } перенаправлений
       *[other] { $status } — после { $count } перенаправлений
    }
