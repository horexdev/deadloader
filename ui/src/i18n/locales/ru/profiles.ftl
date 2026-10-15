# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Профили
profiles-count =
    { $count ->
        [one] { $count } профиль
        [few] { $count } профиля
        [many] { $count } профилей
       *[other] { $count } профиля
    }
profiles-in-game = Сейчас в игре: { $list }
profiles-in-game-item = «{ $name }» на { $drive }
profiles-in-game-none = Сейчас в игре нет ни одного профиля.
profiles-import = Импорт…
profiles-import-title = Импортировать профиль из файла .json
profiles-import-dialog = Импорт профиля
profiles-export-dialog = Экспорт профиля
profiles-file-filter = Профили deadloader
profiles-new = Новый профиль
profiles-selected = Выбран
profiles-applied = Применён
profiles-not-applied = Ещё не применён
profiles-selected-title = Профиль, который вы правите в библиотеке и с которым играете
profiles-pending-title = «Применить» или «Играть» переносит его в папку игры
profiles-applied-title = Его файлы сейчас в папке игры на { $drive }
profiles-select = Выбрать
profiles-select-of = Выбрать «{ $name }»
profiles-more-of = Другие действия для «{ $name }»
profiles-rename = Переименовать
profiles-duplicate = Дублировать
profiles-export = Экспортировать в файл…
profiles-change-installation = Сменить установку игры…
profiles-delete = Удалить
profiles-delete-confirm = Удалить…
profiles-delete-last = deadloader нужен хотя бы один профиль.
profiles-name-label = Название профиля
profiles-save = Сохранить
profiles-cancel = Отмена
profiles-close = Закрыть
profiles-close-esc = Закрыть (Esc)
profiles-rename-hint = Enter — сохранить · Esc — отменить
profiles-name-empty = Введите название профиля.
profiles-mods =
    { $count ->
        [one] { $count } мод включён
        [few] { $count } мода включено
        [many] { $count } модов включено
       *[other] { $count } мода включено
    }
profiles-mods-none = Моды не включены
profiles-applied-when = Применён { $when }
profiles-last-applied = Последний раз применён { $when }
profiles-never-applied = Ни разу не применялся
profiles-just-now = только что
profiles-preview-empty = Выберите этот профиль, затем включите моды в библиотеке.
profiles-footnote = Профиль — это список включённых модов с их порядком, вариантами и версиями. Выбор профиля ничего не скачивает и не удаляет; «Применить» меняет только те файлы, которые отличаются.
profiles-copy-name = { $name } (копия)
profiles-copy-name-n = { $name } (копия { $number })
profiles-new-title = Новый профиль
profiles-start-with = Начать с
profiles-empty = Пустой
profiles-empty-hint = Моды не включены — выберите их потом в библиотеке.
profiles-copy-of = Копия «{ $name }»
profiles-copy-hint = { $mods } — тот же порядок, варианты и версии.
profiles-installation = Установка игры
profiles-installation-hint = Та же, что у «{ $name }». Её можно сменить позже в меню профиля.
profiles-new-note = Новый профиль не выбирается и не применяется — игра не меняется.
profiles-create = Создать профиль
profiles-pending-what = Профиль «{ $name }» ещё не в игре.
profiles-pending-why-other = В папке игры на { $drive } всё ещё «{ $other }». Примените его сейчас или нажмите «Играть» — deadloader применит его перед запуском.
profiles-pending-why-empty = В папке игры на { $drive } сейчас нет модов deadloader. Примените его сейчас или нажмите «Играть» — deadloader применит его перед запуском.
profiles-keep = Оставить «{ $name }»
profiles-apply = Применить «{ $name }»
profiles-other-what = «{ $name }» использует другую установку игры.
profiles-other-why = «Применить» и «Играть» используют { $path }. В игре на { $drive } остаётся «{ $other }».
profiles-other-why-empty = «Применить» и «Играть» используют { $path }. Игра на { $drive } не меняется.
profiles-delete-title = Удалить профиль «{ $name }»?
profiles-delete-applied =
    { $count ->
        [one] «{ $name }» сейчас в игре. deadloader уберёт его { $count } мод из папки игры:
        [few] «{ $name }» сейчас в игре. deadloader уберёт его { $count } мода из папки игры:
        [many] «{ $name }» сейчас в игре. deadloader уберёт его { $count } модов из папки игры:
       *[other] «{ $name }» сейчас в игре. deadloader уберёт его { $count } мода из папки игры:
    }
profiles-delete-applied-empty = «{ $name }» сейчас в игре, но модов в нём нет, поэтому папка игры не изменится.
profiles-delete-stay = Моды останутся в библиотеке и в других профилях. Пока вы не примените там другой профиль, Deadlock запускается без модов deadloader.
profiles-delete-next = Выбранным станет «{ $name }». Он не применится, пока вы не нажмёте «Применить» или «Играть».
profiles-delete-undoable = Это нельзя отменить. Чтобы сохранить копию, сначала экспортируйте профиль.
profiles-export-first = Сначала экспортировать
profiles-delete-profile = Удалить профиль
profiles-installation-title = Установка игры для «{ $name }»
profiles-installation-lead = Выберите папку Deadlock, к которой применяется этот профиль.
profiles-used-by =
    { $count ->
        [one] используется { $count } профилем
        [few] используется { $count } профилями
        [many] используется { $count } профилями
       *[other] используется { $count } профиля
    }
profiles-used-by-none = не используется ни одним профилем
profiles-installation-applied = «{ $name }» сейчас в игре на { $drive }. Его файлы останутся там, пока вы не примените другой профиль на { $drive }.
profiles-installation-not-applied = Ни одна папка игры не изменится, пока вы не примените «{ $name }».
profiles-manage-installations = Управлять установками в настройках
profiles-use-installation = Использовать эту установку
profiles-d9-title =
    { $count ->
        [one] { $count } мода из «{ $name }» нет в вашей библиотеке
        [few] { $count } модов из «{ $name }» нет в вашей библиотеке
        [many] { $count } модов из «{ $name }» нет в вашей библиотеке
       *[other] { $count } мода из «{ $name }» нет в вашей библиотеке
    }
profiles-d9-file = В файле
profiles-d9-lead = модов: { $listed }. В вашей библиотеке уже есть { $present }; этих ({ $count }) нет:
profiles-d9-list = Недостающие моды
profiles-d9-from-gamebanana = с GameBanana: { $count }
profiles-d9-local-count = локальных: { $count }
profiles-gamebanana = GameBanana
profiles-can-download = Можно скачать
profiles-local-mod = Локальный мод
profiles-cant-download = Нельзя скачать
profiles-local-why = Добавлен с диска на другом ПК, поэтому его нет на GameBanana. Попросите файл у того, кто поделился профилем.
profiles-d9-local-mode = Каталог выключен (локальный режим), поэтому deadloader не может скачивать моды. Импортируйте без них и добавьте их потом с диска — или включите каталог в настройках.
profiles-d9-after = В любом случае «{ $name }» станет выбранным профилем, но не применится — папка игры изменится, только когда вы нажмёте «Применить» или «Играть».
profiles-import-without = Импортировать без них
profiles-download-missing = Скачать недостающие ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } мод выключен
        [few] { $count } мода выключено
        [many] { $count } модов выключено
       *[other] { $count } мода выключено
    }
profiles-turned-on =
    { $count ->
        [one] { $count } мод включён
        [few] { $count } мода включено
        [many] { $count } модов включено
       *[other] { $count } мода включено
    }
profiles-differ = Изменены только отличающиеся файлы.
profiles-same = Те же моды уже были в игре; проверены только порядок и варианты.
profiles-deleted = Профиль «{ $name }» удалён.
profiles-deleted-not-applied = Он не был применён, поэтому папка игры не изменилась.
profiles-deleted-applied =
    { $count ->
        [one] { $count } мод убран из папки игры. Он остался в вашей библиотеке.
        [few] { $count } мода убрано из папки игры. Они остались в вашей библиотеке.
        [many] { $count } модов убрано из папки игры. Они остались в вашей библиотеке.
       *[other] { $count } мода убрано из папки игры. Они остались в вашей библиотеке.
    }
profiles-exported = Профиль «{ $name }» экспортирован.
profiles-imported = Профиль «{ $name }» импортирован.
profiles-imported-text = Он выбран, но ещё не применён.
profiles-imported-without =
    { $count ->
        [one] Профиль «{ $name }» импортирован без { $count } мода.
        [few] Профиль «{ $name }» импортирован без { $count } модов.
        [many] Профиль «{ $name }» импортирован без { $count } модов.
       *[other] Профиль «{ $name }» импортирован без { $count } мода.
    }
profiles-imported-without-text = Он выбран, но ещё не применён. Недостающие моды можно добавить позже.
profiles-imported-downloading =
    { $count ->
        [one] Профиль «{ $name }» импортирован — скачивается { $count } мод.
        [few] Профиль «{ $name }» импортирован — скачиваются { $count } мода.
        [many] Профиль «{ $name }» импортирован — скачиваются { $count } модов.
       *[other] Профиль «{ $name }» импортирован — скачиваются { $count } мода.
    }
profiles-imported-downloading-text = Он выбран, но ещё не применён. Примените его, когда загрузки закончатся.
profiles-open-downloads = Открыть загрузки
profiles-undo = Отменить
profiles-check-update = Проверить обновление deadloader
profiles-choose-another = Выбрать другой файл
profiles-switcher = Управлять профилями
profiles-saved-as = Сохранено как
profiles-show-folder = Показать в папке
