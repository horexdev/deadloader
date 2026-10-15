# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Активність
activity-sub =
    { $days ->
        [one] Усе, що deadloader зробив на цьому ПК: мережеві запити, зміни файлів і події застосунку. Журнали зберігаються на цьому ПК { $days } день.
        [few] Усе, що deadloader зробив на цьому ПК: мережеві запити, зміни файлів і події застосунку. Журнали зберігаються на цьому ПК { $days } дні.
        [many] Усе, що deadloader зробив на цьому ПК: мережеві запити, зміни файлів і події застосунку. Журнали зберігаються на цьому ПК { $days } днів.
       *[other] Усе, що deadloader зробив на цьому ПК: мережеві запити, зміни файлів і події застосунку. Журнали зберігаються на цьому ПК { $days } дня.
    }
activity-open-logs = Відкрити теку журналів
activity-logs-opened = Теку журналів відкрито у Провіднику.
activity-summary = Сьогодні коротко
activity-net-today = Мережа сьогодні
activity-hosts-label = Хости, до яких зверталися сьогодні, — виберіть, щоб відфільтрувати
activity-hosts-contacted =
    { $count ->
        [one] { $count } хост
        [few] { $count } хости
        [many] { $count } хостів
       *[other] { $count } хости
    }
activity-host-title =
    { $count ->
        [one] { $count } запит · { $source } — виберіть для фільтра
        [few] { $count } запити · { $source } — виберіть для фільтра
        [many] { $count } запитів · { $source } — виберіть для фільтра
       *[other] { $count } запити · { $source } — виберіть для фільтра
    }
activity-net-none-local = немає — локальний режим
activity-net-none-yet = поки немає
activity-nothing-else = Більше нічого.
activity-blocked-lead =
    { $count ->
        [one] { $count } запит до
        [few] { $count } запити до
        [many] { $count } запитів до
       *[other] { $count } запити до
    }
activity-blocked-tail =
    { $count ->
        [one] заблоковано до встановлення з’єднання.
        [few] заблоковано до встановлення з’єднання.
        [many] заблоковано до встановлення з’єднання.
       *[other] заблоковано до встановлення з’єднання.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } запит заблоковано, з’єднання не було.
        [few] { $count } запити заблоковано, з’єднання не було.
        [many] { $count } запитів заблоковано, з’єднання не було.
       *[other] { $count } запити заблоковано, з’єднання не було.
    }
activity-show-entry = Показати запис
activity-game-today = Тека гри сьогодні
activity-show-in-files = Показати у «Файлах»
activity-game-none = без змін
activity-no-game-changes = deadloader поки нічого не змінював у теці гри.
activity-game-changes =
    { $count ->
        [one] { $count } зміна
        [few] { $count } зміни
        [many] { $count } змін
       *[other] { $count } зміни
    }
activity-game-batches =
    { $count ->
        [one] за { $count } раз
        [few] за { $count } рази
        [many] за { $count } разів
       *[other] за { $count } рази
    }
activity-game-failed =
    { $count ->
        [one] { $count } з помилкою
        [few] { $count } з помилкою
        [many] { $count } з помилкою
       *[other] { $count } з помилкою
    }
activity-game-hardlink = пов’язано: { $count }
activity-game-copy = скопійовано: { $count }
activity-game-delete = видалено: { $count }
activity-game-rename = перейменовано: { $count }
activity-game-write = записано: { $count }
activity-game-backup = збережено в резервну копію: { $count }
activity-game-restore = відновлено: { $count }
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi оновлено { $count } раз
        [few] gameinfo.gi оновлено { $count } рази
        [many] gameinfo.gi оновлено { $count } разів
       *[other] gameinfo.gi оновлено { $count } рази
    }
activity-no-requests-today = Сьогодні запитів не було.
activity-no-requests-yet = Запитів поки не було.
activity-local-short = Каталог вимкнено, тому deadloader ні до чого не підключається.
activity-never-contacted = Від встановлення deadloader не звертався до жодного хоста.
activity-tab-network = Мережа
activity-tab-files = Файли
activity-tab-app = Застосунок
activity-tab-off = вимк.
activity-tab-errors =
    { $count ->
        [one] · { $count } помилка
        [few] · { $count } помилки
        [many] · { $count } помилок
       *[other] · { $count } помилки
    }
activity-search = Пошук у журналі
activity-search-clear = Очистити пошук
activity-matches =
    { $count ->
        [one] { $count } збіг
        [few] { $count } збіги
        [many] { $count } збігів
       *[other] { $count } збіги
    }
activity-matches-more = збігів: { $count }+
activity-f-time = Період
activity-f-purpose = Призначення
activity-f-host = Хост
activity-f-op = Операція
activity-f-level = Рівень
activity-f-result = Результат
activity-all = Усі
activity-ok = OK
activity-errors = Помилки
activity-range-hour = Остання година
activity-range-today = Сьогодні
activity-range-7d = Останні 7 днів
activity-range-30d = Останні 30 днів
activity-clear-filters = Скинути фільтри
activity-clear-all = Скинути пошук і фільтри
activity-follow = Стежити наживо
activity-follow-tip = Показувати нові записи вгорі, щойно вони з’являються
activity-follow-on = Нові записи з’являються вгорі
activity-follow-paused = Призупинено о { $time }
activity-show-new =
    { $count ->
        [one] Показати { $count } новий
        [few] Показати { $count } нові
        [many] Показати { $count } нових
       *[other] Показати { $count } нові
    }
activity-net-aria = Мережеві запити, спершу нові
activity-files-aria = Файлові операції, спершу нові
activity-app-aria = Події застосунку, спершу нові
activity-c-redirects = Переспрямування
activity-c-time = Час
activity-c-purpose = Призначення
activity-c-request = Запит
activity-c-status = Статус
activity-c-size = Розмір
activity-c-duration = Тривалість
activity-c-started-by = Ініціатор
activity-c-source = Джерело хоста
activity-c-op = Операція
activity-c-from-to = Звідки → куди
activity-c-result = Результат
activity-c-change = Частина зміни
activity-c-level = Рівень
activity-c-message = Повідомлення
activity-to = до
activity-loading-more = Завантаження…
activity-p-catalog = Каталог
activity-p-thumbnail = Мініатюра
activity-p-description-image = Зображення в описі
activity-p-download = Завантаження
activity-p-update-check = Перевірка оновлень
activity-src-builtin = Вбудований
activity-src-github = GitHub
activity-src-user = Додано вами
activity-src-user-image = Хост зображень
activity-src-blocked = Не дозволено
activity-src-long-builtin = Вбудований — GameBanana, частина deadloader
activity-src-long-github = GitHub — власна перевірка оновлень deadloader, увімкнена в налаштуваннях
activity-src-long-user = Додано вами — у розділі «Налаштування → Додатково»
activity-src-long-user-image = Дозволений хост зображень — ви дозволили його під час завантаження зображення в описі
activity-src-long-blocked = Немає в списку дозволених хостів
activity-by-auto = Авто
activity-by-user = Ви
activity-op-hardlink = Зв’язок
activity-op-copy = Копіювання
activity-op-rename = Перейменування
activity-op-delete = Видалення
activity-op-write = Запис
activity-op-backup = Резервна копія
activity-op-restore = Відновлення
activity-lv-debug = Налагодження
activity-lv-info = Інфо
activity-lv-warn = Попередження
activity-lv-error = Помилка
activity-st-blocked = Заблоковано
activity-st-blocked-long = Заблоковано — з’єднання не було
activity-st-failed = Збій
activity-redirects =
    { $count ->
        [one] { $count } переспрямування
        [few] { $count } переспрямування
        [many] { $count } переспрямувань
       *[other] { $count } переспрямування
    }
activity-retries =
    { $count ->
        [one] { $count } повтор
        [few] { $count } повтори
        [many] { $count } повторів
       *[other] { $count } повтору
    }
activity-show-redirects = Показати переспрямування
activity-hide-redirects = Сховати переспрямування
activity-hop-request = Крок 1 · запит
activity-hop-redirect = Крок { $n } · переспрямування
activity-hop-retry = Крок { $n } · спроба { $try }
activity-ms = { $ms } мс
activity-seconds = { $s } с
activity-no-match = Відповідних записів немає
activity-no-match-text = Виберіть інший період або скиньте фільтри й пошук.
activity-empty-title = Поки нічого
activity-empty-text = Тут deadloader записує кожен мережевий запит, кожну зміну файлів у теці гри та кожну подію застосунку — щойно вони стаються. Поки нічого не сталося.
activity-local-title = Локальний режим — запити не виконуються
activity-local-why = Каталог вимкнено, тому deadloader ні до чого не підключається — ні до GameBanana, ні куди-небудь ще. Зміни файлів і події застосунку й далі записуються на інших вкладках.
activity-local-do = Увімкніть каталог у налаштуваннях, щоб переглядати GameBanana. Тоді кожен запит з’явиться тут.
activity-enable-catalog = Увімкнути каталог
activity-details = Подробиці запису журналу
activity-close-details = Закрити подробиці
activity-close-esc = Закрити (Esc)
activity-why = Чому:
activity-todo = Що робити:
activity-open-advanced = Відкрити «Налаштування → Додатково»
activity-full-record = Повний запис
activity-hops-title = Переспрямування · { $count }
activity-user-hidden = Ваше ім’я користувача Windows у журналах приховано:
activity-settings-logs = Налаштування → Журнали
activity-copy = Копіювати запис
activity-copied = Скопійовано
activity-kind-network = Мережевий запит
activity-kind-files = Файлова операція
activity-kind-app = Подія застосунку
activity-d-time = Час
activity-d-purpose = Призначення
activity-d-started-by = Ініціатор
activity-d-request = Запит
activity-d-source = Джерело хоста
activity-d-status = Статус
activity-d-size = Розмір
activity-d-duration = Тривалість
activity-d-range = Діапазон
activity-d-error = Помилка
activity-d-id = ID запису
activity-d-request-id = ID запиту
activity-d-operation = Операція
activity-d-from = Звідки
activity-d-to = Куди
activity-d-file = Файл
activity-d-result = Результат
activity-d-change = Частина зміни
activity-d-sha256 = SHA-256
activity-d-level = Рівень
activity-d-message = Повідомлення
activity-d-fields = Подробиці
activity-status-after =
    { $count ->
        [one] { $status } — після { $count } переспрямування
        [few] { $status } — після { $count } переспрямувань
        [many] { $status } — після { $count } переспрямувань
       *[other] { $status } — після { $count } переспрямувань
    }
