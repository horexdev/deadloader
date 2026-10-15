# Downloads (S6): the queue, its totals, the session’s history, the banners and D4 (choose a variant).
downloads-title = Загрузки
downloads-active = Активно
downloads-queued = В очереди
downloads-speed = Скорость
downloads-per-second = { $size }/с
downloads-limit-parallel =
    { $count ->
        [one] До { $count } загрузки одновременно
        [few] До { $count } загрузок одновременно
        [many] До { $count } загрузок одновременно
       *[other] До { $count } загрузки одновременно
    }
downloads-limit-none = без ограничения скорости
downloads-limit-speed = не более { $speed }/с
downloads-change-settings = Изменить в настройках
downloads-pause-all = Приостановить все
downloads-resume-all = Возобновить все
downloads-add-disk = Добавить с диска
downloads-add-title = Добавить моды с диска
downloads-add-filter = Файлы модов и архивы
downloads-queue = Очередь
downloads-queue-empty = Очередь пуста. Установите мод из каталога или добавьте файлы с диска.
downloads-history = Завершено за сеанс
downloads-history-note = Очищается при закрытии deadloader. Каждая загрузка остаётся в «Активности».
downloads-history-empty = За этот сеанс пока ничего не завершено.
downloads-clear = Очистить
downloads-show-in-library = Показать в библиотеке
downloads-show-in-library-of = Показать в библиотеке: { $name }
downloads-again = Скачать снова
downloads-again-of = Скачать снова: { $file }
downloads-pause = Приостановить
downloads-pause-of = Приостановить: { $file }
downloads-resume = Возобновить
downloads-resume-of = Возобновить: { $file }
downloads-cancel-of = Отменить загрузку { $file }
downloads-remove-of = Удалить задачу: { $file }
downloads-aside-of = Выбрать позже: { $file }
downloads-open-mod = Открыть страницу мода — { $name }
downloads-update = Обновление { $version }
downloads-update-plain = Обновление
downloads-choose = Выбрать вариант
downloads-retry = Повторить
downloads-show-details = Показать подробности
downloads-remove-task = Удалить задачу
downloads-open-settings = Открыть Настройки → Загрузки
downloads-open-mod-page = Открыть страницу мода
downloads-open-library = Открыть библиотеку
downloads-stage-downloading = Загружается
downloads-stage-verifying = Проверяется
downloads-stage-extracting = Распаковывается
downloads-stage-choice = Ждёт вашего выбора
downloads-stage-paused = Приостановлено
downloads-stage-offline = Приостановлено — нет сети
downloads-stage-disk = Приостановлено — диск заполнен
downloads-stage-retrying = Ждёт повторной попытки
downloads-stage-queued = В очереди
downloads-stage-failed = Ошибка
downloads-progress = { $done } из { $total } · { $percent } %
downloads-left-seconds = осталось { $seconds } с
downloads-left-minutes = осталось { $minutes } мин { $seconds } с
downloads-left-hours = осталось { $hours } ч { $minutes } мин
downloads-rate = { $speed }/с · { $left }
downloads-verifying-note = Контрольная сумма (md5) сверяется с GameBanana
downloads-extracting-note = Сохраняются только файлы .vpk
downloads-choice-note =
    { $count ->
        [one] Внутри { $count } вариант
        [few] Внутри { $count } варианта
        [many] Внутри { $count } вариантов
       *[other] Внутри { $count } варианта
    }
downloads-offline-note = Продолжится само, когда появится сеть
downloads-disk-note = Ждёт свободного места на { $drive }
downloads-disk-note-plain = Ждёт свободного места
downloads-paused-note = Продолжится с того места, где остановилось
downloads-held-note = Отложено — не начнётся, пока вы не возобновите
downloads-queued-next = Следующая — начнётся, когда освободится одно из мест загрузки ({ $count })
downloads-queued-ahead = Перед ней: { $count }
downloads-queued-offline = Начнётся, когда появится сеть
downloads-retrying-note = Повтор через { $seconds } с
downloads-downloaded = скачано
downloads-offline-title = Нет сети — загрузки приостановлены.
downloads-offline-why = Они продолжатся сами с того места, где остановились, когда появится сеть. Импорт с диска продолжается.
downloads-try-again = Повторить
downloads-back-title = Сеть снова есть.
downloads-back-why = Приостановленные загрузки продолжились с того места, где остановились.
downloads-empty-title = Загрузок нет
downloads-empty-text = Установите мод из каталога или добавьте файлы .vpk, .zip, .7z или .rar с компьютера. Каждый файл будет виден здесь, пока он скачивается и распаковывается. Файлы можно также перетащить в любое место этого окна.
downloads-browse = Открыть каталог
downloads-done-installed = Установлено и включено
downloads-done-variant = Установлено · { $variant }
downloads-done-updated = Обновлено до { $version }
downloads-done-updated-plain = Обновлено
downloads-done-cancelled = Отменено
downloads-done-removed = Удалено — архив стёрт
downloads-done-kept = В библиотеке, не включено
downloads-ago-now = только что
downloads-ago-minutes = { $count } мин назад
downloads-ago-hours = { $count } ч назад
downloads-d4-title = Выберите вариант
downloads-d4-lead = В этом архиве вариантов: { $count }. Они меняют одни и те же файлы игры, поэтому включён может быть только один.
downloads-d4-description = Описание файла на GameBanana
downloads-d4-list = Варианты в этом архиве
downloads-d4-list-meta = { $count } · выберите один
downloads-d4-hint = deadloader сохранит все варианты ({ $count }) в вашей библиотеке и включит выбранный в профиле «{ $profile }» — первым в порядке. Позже это можно изменить в библиотеке.
downloads-d4-install = Установить
downloads-d4-installed = Вариант «{ $variant }» · первым в профиле «{ $profile }».
downloads-cancel = Отмена
downloads-close-esc = Закрыть (Esc)
downloads-open-folder = Открыть папку
downloads-opened = Открыто в Проводнике
