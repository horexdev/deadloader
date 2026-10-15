# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Профілі
profiles-count =
    { $count ->
        [one] { $count } профіль
        [few] { $count } профілі
        [many] { $count } профілів
       *[other] { $count } профілю
    }
profiles-in-game = Зараз у грі: { $list }
profiles-in-game-item = «{ $name }» на { $drive }
profiles-in-game-none = Зараз у грі немає жодного профілю.
profiles-import = Імпорт…
profiles-import-title = Імпортувати профіль із файлу .json
profiles-import-dialog = Імпорт профілю
profiles-export-dialog = Експорт профілю
profiles-file-filter = Профілі deadloader
profiles-new = Новий профіль
profiles-selected = Вибраний
profiles-applied = Застосований
profiles-not-applied = Ще не застосований
profiles-selected-title = Профіль, який ви редагуєте в бібліотеці й з яким граєте
profiles-pending-title = «Застосувати» або «Грати» переносить його в папку гри
profiles-applied-title = Його файли зараз у папці гри на { $drive }
profiles-select = Вибрати
profiles-select-of = Вибрати «{ $name }»
profiles-more-of = Інші дії для «{ $name }»
profiles-rename = Перейменувати
profiles-duplicate = Дублювати
profiles-export = Експортувати у файл…
profiles-change-installation = Змінити встановлення гри…
profiles-delete = Видалити
profiles-delete-confirm = Видалити…
profiles-delete-last = deadloader потрібен хоча б один профіль.
profiles-name-label = Назва профілю
profiles-save = Зберегти
profiles-cancel = Скасувати
profiles-close = Закрити
profiles-close-esc = Закрити (Esc)
profiles-rename-hint = Enter — зберегти · Esc — скасувати
profiles-name-empty = Введіть назву профілю.
profiles-mods =
    { $count ->
        [one] { $count } мод увімкнено
        [few] { $count } моди ввімкнено
        [many] { $count } модів увімкнено
       *[other] { $count } мода ввімкнено
    }
profiles-mods-none = Моди не ввімкнено
profiles-applied-when = Застосовано { $when }
profiles-last-applied = Востаннє застосовано { $when }
profiles-never-applied = Ще жодного разу не застосовано
profiles-just-now = щойно
profiles-preview-empty = Виберіть цей профіль, а потім увімкніть моди в бібліотеці.
profiles-footnote = Профіль — це список увімкнених модів із їхнім порядком, варіантами й версіями. Вибір профілю нічого не завантажує й не видаляє; «Застосувати» змінює лише ті файли, що відрізняються.
profiles-copy-name = { $name } (копія)
profiles-copy-name-n = { $name } (копія { $number })
profiles-new-title = Новий профіль
profiles-start-with = Почати з
profiles-empty = Порожній
profiles-empty-hint = Моди не ввімкнено — виберіть їх потім у бібліотеці.
profiles-copy-of = Копія «{ $name }»
profiles-copy-hint = { $mods } — той самий порядок, варіанти й версії.
profiles-installation = Встановлення гри
profiles-installation-hint = Те саме, що в «{ $name }». Його можна змінити пізніше в меню профілю.
profiles-new-note = Новий профіль не вибирається й не застосовується — гра не змінюється.
profiles-create = Створити профіль
profiles-pending-what = Профіль «{ $name }» ще не в грі.
profiles-pending-why-other = У папці гри на { $drive } досі «{ $other }». Застосуйте його зараз або натисніть «Грати» — deadloader застосує його перед запуском.
profiles-pending-why-empty = У папці гри на { $drive } зараз немає модів deadloader. Застосуйте його зараз або натисніть «Грати» — deadloader застосує його перед запуском.
profiles-keep = Залишити «{ $name }»
profiles-apply = Застосувати «{ $name }»
profiles-other-what = «{ $name }» використовує інше встановлення гри.
profiles-other-why = «Застосувати» й «Грати» використовують { $path }. У грі на { $drive } лишається «{ $other }».
profiles-other-why-empty = «Застосувати» й «Грати» використовують { $path }. Гра на { $drive } не змінюється.
profiles-delete-title = Видалити профіль «{ $name }»?
profiles-delete-applied =
    { $count ->
        [one] «{ $name }» зараз у грі. deadloader прибере його { $count } мод із папки гри:
        [few] «{ $name }» зараз у грі. deadloader прибере його { $count } моди з папки гри:
        [many] «{ $name }» зараз у грі. deadloader прибере його { $count } модів із папки гри:
       *[other] «{ $name }» зараз у грі. deadloader прибере його { $count } мода з папки гри:
    }
profiles-delete-applied-empty = «{ $name }» зараз у грі, але модів у ньому немає, тож папка гри не зміниться.
profiles-delete-stay = Моди лишаться в бібліотеці та в інших профілях. Доки ви не застосуєте там інший профіль, Deadlock запускається без модів deadloader.
profiles-delete-next = Вибраним стане «{ $name }». Його не буде застосовано, доки ви не натиснете «Застосувати» або «Грати».
profiles-delete-undoable = Це не можна скасувати. Щоб зберегти копію, спершу експортуйте профіль.
profiles-export-first = Спершу експортувати
profiles-delete-profile = Видалити профіль
profiles-installation-title = Встановлення гри для «{ $name }»
profiles-installation-lead = Виберіть папку Deadlock, до якої застосовується цей профіль.
profiles-used-by =
    { $count ->
        [one] використовує { $count } профіль
        [few] використовують { $count } профілі
        [many] використовують { $count } профілів
       *[other] використовують { $count } профілю
    }
profiles-used-by-none = не використовує жоден профіль
profiles-installation-applied = «{ $name }» зараз у грі на { $drive }. Його файли лишаться там, доки ви не застосуєте інший профіль на { $drive }.
profiles-installation-not-applied = Жодна папка гри не зміниться, доки ви не застосуєте «{ $name }».
profiles-manage-installations = Керувати встановленнями в налаштуваннях
profiles-use-installation = Використати це встановлення
profiles-d9-title =
    { $count ->
        [one] { $count } мода з «{ $name }» немає у вашій бібліотеці
        [few] { $count } модів з «{ $name }» немає у вашій бібліотеці
        [many] { $count } модів з «{ $name }» немає у вашій бібліотеці
       *[other] { $count } мода з «{ $name }» немає у вашій бібліотеці
    }
profiles-d9-file = У файлі
profiles-d9-lead = модів: { $listed }. У вашій бібліотеці вже є { $present }; цих ({ $count }) немає:
profiles-d9-list = Відсутні моди
profiles-d9-from-gamebanana = з GameBanana: { $count }
profiles-d9-local-count = локальних: { $count }
profiles-gamebanana = GameBanana
profiles-can-download = Можна завантажити
profiles-local-mod = Локальний мод
profiles-cant-download = Не можна завантажити
profiles-local-why = Додано з диска на іншому ПК, тому його немає на GameBanana. Попросіть файл у того, хто поділився профілем.
profiles-d9-local-mode = Каталог вимкнено (локальний режим), тож deadloader не може завантажувати моди. Імпортуйте без них і додайте їх потім із диска — або ввімкніть каталог у налаштуваннях.
profiles-d9-after = У будь-якому разі «{ $name }» стане вибраним профілем, але не застосується — папка гри зміниться, лише коли ви натиснете «Застосувати» або «Грати».
profiles-import-without = Імпортувати без них
profiles-download-missing = Завантажити відсутні ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } мод вимкнено
        [few] { $count } моди вимкнено
        [many] { $count } модів вимкнено
       *[other] { $count } мода вимкнено
    }
profiles-turned-on =
    { $count ->
        [one] { $count } мод увімкнено
        [few] { $count } моди ввімкнено
        [many] { $count } модів увімкнено
       *[other] { $count } мода ввімкнено
    }
profiles-differ = Змінено лише файли, що відрізняються.
profiles-same = Ті самі моди вже були в грі; перевірено лише порядок і варіанти.
profiles-deleted = Профіль «{ $name }» видалено.
profiles-deleted-not-applied = Його не було застосовано, тож папка гри не змінилася.
profiles-deleted-applied =
    { $count ->
        [one] { $count } мод прибрано з папки гри. Він лишився у вашій бібліотеці.
        [few] { $count } моди прибрано з папки гри. Вони лишилися у вашій бібліотеці.
        [many] { $count } модів прибрано з папки гри. Вони лишилися у вашій бібліотеці.
       *[other] { $count } мода прибрано з папки гри. Вони лишилися у вашій бібліотеці.
    }
profiles-exported = Профіль «{ $name }» експортовано.
profiles-imported = Профіль «{ $name }» імпортовано.
profiles-imported-text = Його вибрано, але ще не застосовано.
profiles-imported-without =
    { $count ->
        [one] Профіль «{ $name }» імпортовано без { $count } мода.
        [few] Профіль «{ $name }» імпортовано без { $count } модів.
        [many] Профіль «{ $name }» імпортовано без { $count } модів.
       *[other] Профіль «{ $name }» імпортовано без { $count } мода.
    }
profiles-imported-without-text = Його вибрано, але ще не застосовано. Відсутні моди можна додати пізніше.
profiles-imported-downloading =
    { $count ->
        [one] Профіль «{ $name }» імпортовано — завантажується { $count } мод.
        [few] Профіль «{ $name }» імпортовано — завантажуються { $count } моди.
        [many] Профіль «{ $name }» імпортовано — завантажуються { $count } модів.
       *[other] Профіль «{ $name }» імпортовано — завантажуються { $count } мода.
    }
profiles-imported-downloading-text = Його вибрано, але ще не застосовано. Застосуйте його, коли завантаження завершаться.
profiles-open-downloads = Відкрити завантаження
profiles-undo = Скасувати
profiles-check-update = Перевірити оновлення deadloader
profiles-choose-another = Вибрати інший файл
profiles-switcher = Керувати профілями
