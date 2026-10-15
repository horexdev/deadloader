# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Відкрити це посилання в браузері?
catalog-link-body = Посилання відкриється не в deadloader, а у вашому типовому браузері.
catalog-link-always = Завжди відкривати посилання одразу
catalog-link-cancel = Скасувати
catalog-link-open = Відкрити

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Приховані
catalog-nsfw-blurred = Розмиті
catalog-nsfw-shown = Показані
catalog-age-title = Показувати моди з відвертим вмістом?
catalog-age-body = Деякі моди на GameBanana позначені за оголеність, сексуальні теми тощо. Щоб бачити їх, підтвердьте, що вам виповнилося 18 років або ви досягли повноліття за законами вашої країни.
catalog-age-choice = Ваш вибір: { $mode }.
catalog-age-once = deadloader запитує про це один раз. Повернутися до «Приховані» можна будь-коли.
catalog-age-confirm = Мені виповнилося 18
catalog-age-cancel = Скасувати

## S2: the catalog grid
catalog-title = Каталог
catalog-section = Розділ
catalog-section-mods = Моди
catalog-section-sounds = Звуки
catalog-search = Пошук модів
catalog-search-sounds = Пошук звуків
catalog-search-offline = Для пошуку потрібне з’єднання
catalog-search-off = Пошук недоступний — каталог не читається
catalog-search-clear = Очистити пошук
catalog-results =
    { $count ->
        [one] { $count } результат
        [few] { $count } результати
       *[other] { $count } результатів
    }
catalog-sort = Сортування
catalog-sort-newest = Нові
catalog-sort-latest_updated = Нещодавно оновлені
catalog-sort-most_downloaded = Найбільше завантажень
catalog-sort-most_liked = Найбільше вподобань
catalog-sort-most_viewed = Найбільше переглядів
catalog-sort-most_commented = Найбільше коментарів
catalog-sort-oldest = Старі
catalog-sort-best = Найкращий збіг
catalog-sort-best-title = Під час пошуку результати сортуються за збігом
catalog-category = Категорія
catalog-category-all = Усі категорії
catalog-hero = Герой
catalog-hero-all = Усі герої
catalog-hero-filter = Знайти героя
catalog-hero-none = Такого героя немає
catalog-filters = Фільтри
catalog-filter-nsfw = Моди з віковими позначками (NSFW)
catalog-filter-favourites = Лише обране
catalog-filter-favourites-short = лише обране
catalog-filter-installed = Приховати встановлені
catalog-filter-installed-short = без встановлених
catalog-nsfw-caption-hidden = NSFW приховано
catalog-nsfw-caption-blurred = NSFW розмито · натисніть на картку, щоб показати
catalog-nsfw-caption-shown = NSFW показано
catalog-loading = Завантаження модів з GameBanana…
catalog-loading-sounds = Завантаження звуків з GameBanana…
catalog-loading-more = Завантаження…
catalog-more-failed = Не вдалося завантажити наступну сторінку.
catalog-grid = Моди
catalog-grid-sounds = Звуки
catalog-likes =
    { $count ->
        [one] { $count } вподобання
        [few] { $count } вподобання
       *[other] { $count } вподобань
    }
catalog-favourite-add = Додати до обраного
catalog-favourite-remove = Прибрати з обраного
catalog-sensitive = Відвертий вміст
catalog-click-show = Натисніть, щоб показати
catalog-badge-installed = Встановлено
catalog-badge-update = Оновлення
catalog-card-installing = Встановлення · { $percent } %
catalog-empty = Тут поки немає модів
catalog-empty-search = Немає модів за запитом «{ $text }»
catalog-empty-favourites = В обраному поки порожньо
catalog-empty-filters = Увімкнено фільтри: { $filters }. Скиньте їх, щоб шукати серед усіх модів, або спробуйте коротше слово.
catalog-empty-shorter = Спробуйте коротше або інше слово.
catalog-clear-filters = Скинути фільтри
catalog-rate-retry = deadloader повторить спробу через { $seconds } с — моди нижче й надалі доступні.
catalog-retry-now = Повторити зараз
catalog-try-again = Повторити
catalog-check-update = Перевірити оновлення deadloader
catalog-details = Подробиці · запис журналу
catalog-copy-id = Скопіювати номер запису
catalog-open-activity = Відкрити в «Активності»
catalog-offline-empty-title = Немає з’єднання — нічого показати
catalog-offline-empty-text = deadloader не може зв’язатися з GameBanana, а цю частину каталогу раніше не збережено.
catalog-error-title = Каталог зараз не вдається показати.
catalog-local-title = Каталог вимкнено (локальний режим)
catalog-local-why = У локальному режимі deadloader не робить жодних мережевих запитів. Бібліотека, профілі та запуск гри працюють як завжди, а моди можна додавати з диска.
catalog-local-do = Увімкніть каталог, щоб шукати та встановлювати моди з GameBanana. Тоді deadloader звертатиметься лише до GameBanana.
catalog-local-enable = Увімкнути каталог
catalog-local-library = До бібліотеки

## S3: the mod page
catalog-breadcrumb = Навігація
catalog-back = Назад до каталогу
catalog-page-loading = Завантаження сторінки мода з GameBanana…
catalog-page-offline = Немає з’єднання — показано копію, збережену { $date }.
catalog-page-offline-why = Для встановлення нових файлів потрібне з’єднання; перемикатися між завантаженими варіантами можна й зараз.
catalog-page-offline-empty-title = Немає з’єднання — цю сторінку не збережено
catalog-page-offline-empty-text = deadloader не може зв’язатися з GameBanana, а цю сторінку раніше не відкривали.
catalog-page-local-title = Цю сторінку мода не збережено
catalog-page-local-text = У локальному режимі deadloader показує лише збережені раніше сторінки модів.
catalog-removed-title = Цей мод видалено з GameBanana
catalog-removed-why = Його приховали або видалили, тому deadloader не може показати його сторінку чи завантажити файли.
catalog-removed-keep = Встановлена копія ({ $copy }) лишається в бібліотеці й працює далі — просто більше не оновлюватиметься.
catalog-show-in-library = Показати в бібліотеці
catalog-installed = Встановлено
catalog-by = автор: { $author }
catalog-version = Версія { $version }
catalog-views =
    { $count ->
        [one] { $count } перегляд
        [few] { $count } перегляди
       *[other] { $count } переглядів
    }
catalog-downloads =
    { $count ->
        [one] { $count } завантаження
        [few] { $count } завантаження
       *[other] { $count } завантажень
    }
catalog-added = Додано { $when }
catalog-updated = Оновлено { $when }
catalog-favourite = В обране
catalog-open-gamebanana = Відкрити на GameBanana
catalog-gallery-alt = { $name }, зображення { $index }
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Попереднє зображення
catalog-gallery-next = Наступне зображення
catalog-gallery-original = Оригінал
catalog-gallery-hidden = Зображення модів з віковими позначками приховано.
catalog-audio-preview = Цей звук можна прослухати на GameBanana.
catalog-description = Опис
catalog-description-more = Показати опис повністю
catalog-description-less = Згорнути
catalog-description-truncated = Показано лише початок опису.
catalog-opens-browser = відкриється в браузері
catalog-opens-here = Відкриється в deadloader
catalog-image-from = Зображення з
catalog-image-click = натисніть, щоб завантажити
catalog-image-failed = не вдалося завантажити
catalog-image-alt = Зображення з опису
catalog-youtube = Відео на YouTube
catalog-spoiler = Спойлер — натисніть, щоб показати
catalog-spoiler-hide = Сховати спойлер
catalog-files = Файли
catalog-files-clean =
    { $count ->
        [one] Файл пройшов антивірусну перевірку GameBanana
       *[other] Усі файли ({ $count }) пройшли антивірусну перевірку GameBanana
    }
catalog-files-clean-dated =
    { $count ->
        [one] Файл пройшов антивірусну перевірку GameBanana · завантажено { $date }
       *[other] Усі файли ({ $count }) пройшли антивірусну перевірку GameBanana · завантажено { $date }
    }
catalog-files-av = Результат антивірусної перевірки GameBanana вказано біля кожного файлу.
catalog-variants-note = Ці файли змінюють ті самі файли гри, тож це варіанти — активний лише один.
catalog-no-files = У цього мода немає файлів для завантаження.
catalog-older-files = Старі файли ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } завантаження
        [few] { $count } завантаження
       *[other] { $count } завантажень
    }
catalog-file-active = активний варіант
catalog-file-downloaded = завантажено
catalog-file-install = Встановити
catalog-file-install-label = Встановити { $file } і зробити активним варіантом
catalog-file-install-title = Завантажує цей файл і робить його активним варіантом
catalog-file-install-offline = Для встановлення потрібне з’єднання
catalog-file-installing = Встановлення…
catalog-file-installed = Встановлено
catalog-file-switch = Перемкнути
catalog-file-switch-label = Перемкнутися на { $file }
catalog-file-switch-title = Уже завантажено — зробити активним варіантом
catalog-file-update = Оновити
catalog-file-update-title = Нова версія замінює встановлений файл — перегляньте її в бібліотеці
catalog-av-clean = Чисто
catalog-av-clean-title = Антивірус GameBanana: загроз немає
catalog-av-flagged = Є зауваження
catalog-av-flagged-title = Антивірус GameBanana: { $verdict }
catalog-av-pending = Ще не перевірено
catalog-av-pending-title = GameBanana ще не завершив перевірку цього файлу
catalog-av-none = Без перевірки
catalog-av-none-title = GameBanana не надіслав результат перевірки для цього файлу
catalog-credits = Автори
catalog-no-ratings = Немає вікових позначок
catalog-no-licence = Немає тексту ліцензії
catalog-no-sources = Немає інших джерел
catalog-rating-sexual_themes = Сексуальні теми
catalog-rating-sexual_content = Сексуальний вміст
catalog-rating-skimpy_attire = Відвертий одяг
catalog-rating-lewd_angles_and_poses = Відверті ракурси й пози
catalog-rating-full_nudity = Повна оголеність
catalog-licence = Ліцензія
catalog-sources = Інші джерела
catalog-videos = Відео

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Завантажити зображення з нового сайту?
catalog-image-body-before = deadloader завантажить це зображення з
catalog-image-body-after = . Це не GameBanana — запит з’явиться в «Активності», як і всі інші.
catalog-image-note = Розмір зображень обмежено. Дозволені сайти можна видалити в «Налаштування → Каталог».
catalog-image-always = Завжди дозволяти зображення з цього сайту
catalog-image-cancel = Скасувати
catalog-image-load = Завантажити зображення
