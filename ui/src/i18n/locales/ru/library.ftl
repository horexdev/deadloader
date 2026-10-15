# The Library (S4): header, states, list, mod details, conflicts (C1), deleting (D6).
library-title = Библиотека
library-profile = Профиль
library-enabled-count = Включено { $enabled } из { $total }
library-disk = { $size } на диске
library-not-checked = Обновления ещё не проверялись
library-checked-today = Обновления проверены сегодня в { $time }
library-checked-on = Обновления проверены { $date }
library-game-not-found = Игра не найдена
library-add-disk = Добавить с диска
library-add-title = Добавить моды с диска
library-add-filter = Файлы модов
library-added =
    { $count ->
        [one] В библиотеку добавлен { $count } мод.
        [few] В библиотеку добавлено { $count } мода.
        [many] В библиотеку добавлено { $count } модов.
       *[other] В библиотеку добавлено { $count } мода.
    }
library-check = Проверить обновления
library-checking = Проверка…
library-update-all = Обновить все
library-review = Посмотреть
library-undo = Отменить
library-cancel = Отмена
library-missing-what = Deadlock не найден.
library-missing-why = Steam или игра не установлены, либо папка игры перемещена. Библиотека работает как обычно; применение изменений и «Играть» недоступны, пока игра не найдена.
library-choose-folder = Выбрать папку игры
library-search-again = Искать снова
library-running-what = Deadlock запущен.
library-running-why = Изменения будут применены автоматически, когда вы закроете игру.
library-pending = Ожидающие изменения
library-switch-what = Профиль «{ $name }» ещё не в игре.
library-switch-why = В игре всё ещё «{ $applied }». Примените «{ $name }» сейчас или нажмите «Играть» — deadloader применит его перед запуском.
library-switch-why-first = Примените «{ $name }» сейчас или нажмите «Играть» — deadloader применит его перед запуском.
library-review-switch = Посмотреть переключение
library-applying-what = Применяется профиль «{ $name }»…
library-applying-why = Можно продолжать работу; «Играть» снова станет доступна, когда это закончится.
library-show = Показать
library-tab-all = Все
library-tab-enabled = Включённые
library-tab-disabled = Выключенные
library-tab-updates = Обновления
library-tab-conflicts = Конфликты
library-tab-local = Локальные
library-search = Поиск в библиотеке
library-search-clear = Очистить поиск
library-higher-wins = Когда два мода меняют один и тот же файл, побеждает тот, что выше в списке.
library-reorder-hint = Порядок: перетащите за маркер или наведите на него фокус и нажмите Пробел
library-list-label = Моды профиля «{ $name }» в порядке приоритета
library-toggle = { $name } включён
library-toggle-missing = Deadlock не найден — выберите папку игры, чтобы применять изменения
library-grip = Переместить { $name }, позиция { $position } из { $total }
library-more = Другие действия для { $name }
library-show-variants = Показать варианты
library-hide-variants = Скрыть варианты
library-show-addons = Показать дополнения
library-hide-addons = Скрыть дополнения
library-live-picked = { $name } — взят, позиция { $position } из { $total }. ↑ ↓ — переместить, Пробел — положить, Esc — отменить.
library-live-moved = { $name } — позиция { $position } из { $total }.
library-live-moved-grabbed = { $name } — позиция { $position } из { $total }. Пробел — положить, Esc — отменить.
library-live-dropped = { $name } — положен на позицию { $position } из { $total }.
library-live-back = { $name } — снова на позиции { $position } из { $total }.
library-live-drag = { $name } — отпустите, чтобы положить над { $above }, на позицию { $position } из { $total }.
library-badge-pending = Ожидает
library-badge-damaged = Файлы повреждены
library-badge-update = Есть обновление
library-badge-wins = Конфликт · побеждает
library-badge-loses = Конфликт · проигрывает
library-badge-variants =
    { $count ->
        [one] { $count } вариант
        [few] { $count } варианта
        [many] { $count } вариантов
       *[other] { $count } варианта
    }
library-badge-local = Локальный
library-badge-copied = Скопирован · +{ $size }
library-local = Локальный
library-unknown-author = Автор неизвестен
library-gamebanana = GameBanana
library-from-disk = с диска
library-addons-sub =
    { $count ->
        [one] + { $count } дополнение
        [few] + { $count } дополнения
        [many] + { $count } дополнений
       *[other] + { $count } дополнения
    }
library-variants = Варианты
library-variants-downloaded = Скачано: { $count }
library-active = Активен
library-select-to-switch = Выберите, чтобы переключить
library-parts = Файлы этого мода
library-main-files = Основные файлы
library-from = из { $archive }
library-on-with-mod = Включается вместе с модом · { $size }
library-remove-addon = Удалить дополнение
library-toast-enabled = { $name } включён.
library-toast-disabled = { $name } выключен.
library-toast-variant = { $name }: вариант переключён на { $variant }.
library-empty-title = Библиотека пуста
library-empty-text = Установите моды из каталога или добавьте файлы .vpk, .zip, .7z или .rar с компьютера. Файлы можно также перетащить в любое место этого окна.
library-empty-local-text = В локальном режиме каталог выключен. Добавьте файлы .vpk, .zip, .7z или .rar с компьютера или перетащите их в любое место этого окна.
library-browse = Открыть каталог
library-no-match-title = Здесь нет модов
library-no-match-text = Ни один мод не подходит под эту вкладку и поиск. Выберите другую вкладку или очистите поиск.

## Mod details
library-details = Сведения о моде
library-open-mod-page = Открыть страницу мода
library-close-details = Закрыть сведения
library-enabled-in = Включён в «{ $profile }»
library-position = Позиция { $position } из { $total }
library-version = Версия
library-no-version = Версия не указана
library-from-gamebanana = С GameBanana
library-imported = Импортирован с диска
library-newer-in-library = В библиотеке есть более новая версия.
library-switch-to = Переключиться
library-delete-old = Удалить старые версии
library-no-older = Старые версии не сохранены
library-older =
    { $count ->
        [one] Сохранена { $count } старая версия · { $size }
        [few] Сохранено { $count } старые версии · { $size }
        [many] Сохранено { $count } старых версий · { $size }
       *[other] Сохранено { $count } старой версии · { $size }
    }
library-files = Файлы
library-on-disk = { $size } на диске
library-linked = Связан — без лишнего места на диске
library-copied = Скопирован — { $size } дополнительного места
library-not-in-game = Не в папке игры
library-local-mod = Локальный мод
library-name = Название в библиотеке
library-rename = Переименовать
library-link-label = Страница на GameBanana
library-link-hint = Вставьте адрес мода, чтобы проверять его обновления.
library-link = Связать
library-link-done = Связано: на странице найдено файлов — { $matched } из { $total }.
library-link-not-a-page = Это не страница мода на GameBanana. Вставьте адрес вида gamebanana.com/mods/123456.
library-link-removed = Этого мода больше нет на GameBanana.
library-link-failed = Не удалось прочитать страницу. Проверьте адрес и попробуйте ещё раз.
library-imported-from = Импортирован из
library-no-conflicts = Нет конфликтов с другими включёнными модами
library-conflict-wins = Побеждает { $name } · файлов: { $count }
library-conflict-loses = Проигрывает { $name } · файлов: { $count }
library-show-conflicts = Показать конфликты
library-delete-mod = Удалить мод…
library-open-folder = Открыть папку

## Conflicts (C1)
library-conflicts-sub = Два включённых мода меняют одни и те же файлы игры. Побеждает тот, что выше в списке.
library-close-conflicts = Закрыть конфликты
library-no-conflicts-title = Конфликтов нет
library-wins = Побеждает
library-loses = Проигрывает
library-off = Выкл.
library-in-use = Используется
library-conflict-text = Оба меняют файлов: { $count }. В игре вы видите версии { $winner }; копии этих файлов из { $loser } не используются.
library-conflict-off = Оба меняют файлов: { $count }. { $off } выключен, поэтому игра использует версии { $other }.
library-files-count =
    { $count ->
        [one] { $count } файл
        [few] { $count } файла
        [many] { $count } файлов
       *[other] { $count } файла
    }
library-show-more = Показать ещё { $count }
library-move-above = Поднять { $name } выше
library-disable = Выключить { $name }
library-pick-variant = Выбрать другой вариант
library-no-variants = Ни у одного мода нет вариантов
library-done-wins = Готово — теперь побеждает { $name }
library-done-off = Готово — { $off } выключен; используются файлы { $other }
library-identical = Одинаковые файлы (с той же контрольной суммой) не считаются конфликтами и не показываются.

## D6 Delete mod
library-d6-title = Удалить «{ $name }»?
library-d6-files-title = Удалить старые версии «{ $name }»?
library-d6-what =
    { $count ->
        [one] { $count } файл · { $size }.
        [few] { $count } файла · { $size }.
        [many] { $count } файлов · { $size }.
       *[other] { $count } файла · { $size }.
    }
library-d6-profiles = Используется в профилях ({ $count }): { $names } — мод будет убран из них и из папки игры.
library-d6-no-profiles = Ни один профиль его не использует.
library-d6-where = Удалить
library-d6-permanently = Навсегда
library-d6-recycle-bin = В корзину
library-d6-permanent-hint = Файлы будут удалены навсегда — это нельзя отменить.
library-d6-bin-hint = Его можно будет восстановить из корзины Windows.
library-d6-confirm = Удалить мод
library-d6-confirm-files = Удалить файлы
library-deleted = { $name } удалён.
library-deleted-files = Удалено файлов { $name }: { $count }.
