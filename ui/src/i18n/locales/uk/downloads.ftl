# Downloads (S6): the queue, its totals, the session’s history, the banners and D4 (choose a variant).
downloads-title = Завантаження
downloads-active = Активні
downloads-queued = У черзі
downloads-speed = Швидкість
downloads-per-second = { $size }/с
downloads-limit-parallel =
    { $count ->
        [one] До { $count } завантаження одночасно
        [few] До { $count } завантажень одночасно
        [many] До { $count } завантажень одночасно
       *[other] До { $count } завантаження одночасно
    }
downloads-limit-none = без обмеження швидкості
downloads-limit-speed = не більше { $speed }/с
downloads-change-settings = Змінити в налаштуваннях
downloads-pause-all = Призупинити все
downloads-resume-all = Відновити все
downloads-add-disk = Додати з диска
downloads-add-title = Додати моди з диска
downloads-add-filter = Файли модів і архіви
downloads-queue = Черга
downloads-queue-empty = Черга порожня. Установіть мод із каталогу або додайте файли з диска.
downloads-history = Завершено за сеанс
downloads-history-note = Очищується, коли ви закриваєте deadloader. Кожне завантаження лишається в «Активності».
downloads-history-empty = За цей сеанс ще нічого не завершено.
downloads-clear = Очистити
downloads-show-in-library = Показати в бібліотеці
downloads-show-in-library-of = Показати в бібліотеці: { $name }
downloads-again = Завантажити знову
downloads-again-of = Завантажити знову: { $file }
downloads-pause = Призупинити
downloads-pause-of = Призупинити: { $file }
downloads-resume = Відновити
downloads-resume-of = Відновити: { $file }
downloads-cancel-of = Скасувати завантаження { $file }
downloads-remove-of = Видалити завдання: { $file }
downloads-aside-of = Вибрати пізніше: { $file }
downloads-open-mod = Відкрити сторінку мода — { $name }
downloads-update = Оновлення { $version }
downloads-update-plain = Оновлення
downloads-choose = Вибрати варіант
downloads-retry = Повторити
downloads-show-details = Показати подробиці
downloads-remove-task = Видалити завдання
downloads-open-settings = Відкрити Налаштування → Завантаження
downloads-open-mod-page = Відкрити сторінку мода
downloads-open-library = Відкрити бібліотеку
downloads-stage-downloading = Завантажується
downloads-stage-verifying = Перевіряється
downloads-stage-extracting = Розпаковується
downloads-stage-choice = Чекає на ваш вибір
downloads-stage-paused = Призупинено
downloads-stage-offline = Призупинено — немає мережі
downloads-stage-disk = Призупинено — диск заповнений
downloads-stage-retrying = Чекає на повторну спробу
downloads-stage-queued = У черзі
downloads-stage-failed = Помилка
downloads-progress = { $done } з { $total } · { $percent } %
downloads-left-seconds = залишилося { $seconds } с
downloads-left-minutes = залишилося { $minutes } хв { $seconds } с
downloads-left-hours = залишилося { $hours } год { $minutes } хв
downloads-rate = { $speed }/с · { $left }
downloads-verifying-note = Контрольна сума (md5) звіряється з GameBanana
downloads-extracting-note = Зберігаються лише файли .vpk
downloads-choice-note =
    { $count ->
        [one] Усередині { $count } варіант
        [few] Усередині { $count } варіанти
        [many] Усередині { $count } варіантів
       *[other] Усередині { $count } варіанта
    }
downloads-offline-note = Продовжиться саме, коли з’явиться мережа
downloads-disk-note = Чекає на вільне місце на { $drive }
downloads-disk-note-plain = Чекає на вільне місце
downloads-paused-note = Продовжиться з того місця, де зупинилося
downloads-held-note = Відкладено — не почнеться, доки ви не відновите
downloads-queued-next = Наступне — почнеться, коли звільниться одне з місць завантаження ({ $count })
downloads-queued-ahead = Перед ним: { $count }
downloads-queued-offline = Почнеться, коли з’явиться мережа
downloads-retrying-note = Повтор через { $seconds } с
downloads-downloaded = завантажено
downloads-offline-title = Немає мережі — завантаження призупинено.
downloads-offline-why = Вони продовжаться самі з того місця, де зупинилися, коли з’явиться мережа. Імпорт із диска триває.
downloads-try-again = Повторити
downloads-back-title = Мережа знову є.
downloads-back-why = Призупинені завантаження продовжилися з того місця, де зупинилися.
downloads-empty-title = Завантажень немає
downloads-empty-text = Установіть мод із каталогу або додайте файли .vpk, .zip, .7z чи .rar зі свого комп’ютера. Кожен файл буде видно тут, поки він завантажується й розпаковується. Файли також можна перетягнути в будь-яке місце цього вікна.
downloads-browse = Відкрити каталог
downloads-done-installed = Установлено й увімкнено
downloads-done-variant = Установлено · { $variant }
downloads-done-updated = Оновлено до { $version }
downloads-done-updated-plain = Оновлено
downloads-done-cancelled = Скасовано
downloads-done-removed = Видалено — архів стерто
downloads-done-kept = У бібліотеці, не ввімкнено
downloads-ago-now = щойно
downloads-ago-minutes = { $count } хв тому
downloads-ago-hours = { $count } год тому
downloads-d4-title = Виберіть варіант
downloads-d4-lead = У цьому архіві варіантів: { $count }. Вони змінюють ті самі файли гри, тому ввімкненим може бути лише один.
downloads-d4-description = Опис файлу на GameBanana
downloads-d4-list = Варіанти в цьому архіві
downloads-d4-list-meta = { $count } · виберіть один
downloads-d4-hint = deadloader збереже всі варіанти ({ $count }) у вашій бібліотеці й увімкне вибраний у профілі «{ $profile }» — першим у порядку. Пізніше це можна змінити в бібліотеці.
downloads-d4-install = Установити
downloads-d4-installed = Варіант «{ $variant }» · першим у профілі «{ $profile }».
downloads-cancel = Скасувати
downloads-close-esc = Закрити (Esc)
downloads-open-folder = Відкрити теку
downloads-opened = Відкрито у Провіднику
