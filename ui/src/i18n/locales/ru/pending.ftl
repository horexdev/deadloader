# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Закрыть
library-apply = Применить
library-apply-profile = Применить «{ $name }»
library-discard-all = Отменить все
library-keep = Оставить «{ $name }»
library-keep-applied = Оставить применённые изменения
library-undo-this = Отменить это изменение
library-undo-all = Отменить все
library-close-to-apply = Закройте Deadlock — изменения применятся автоматически
library-play-applies = «Играть» тоже применит его
library-applied-title = Изменения применены
library-nothing-pending = Ничего не ожидает
library-all-in-game = Всё из профиля «{ $name }» уже в игре.
library-running-sub = Deadlock запущен, и Windows блокирует его файлы. Эти изменения применятся автоматически, как только вы закроете игру.
library-waiting-sub = Этих изменений ещё нет в папке игры. Примените их сейчас или нажмите «Играть».
library-switch-sub = Переключение профиля меняет сразу много файлов, поэтому оно ждёт вас. Примените его здесь или нажмите «Играть» — deadloader применит его перед запуском.
library-applied-sub = Deadlock был закрыт в { $time }, поэтому изменения, сделанные во время игры ({ $count }), применены. Отменить можно до следующего изменения.
library-undoing-sub = Вы отменили изменения, применённые в { $time } ({ $count }), а Deadlock снова запущен. Они будут отменены автоматически, когда вы закроете игру.
library-group-enable = Включить · { $count }
library-group-disable = Выключить · { $count }
library-group-order = Порядок · { $count }
library-group-variant = Вариант · { $count }
library-group-version = Версия · { $count }
library-group-switch = Смена профиля · { $count }
library-group-undo = Отмена изменений, применённых в { $time } ({ $count })
library-change-on = Выкл. → вкл.
library-change-off = Вкл. → выкл.
library-change-above = Перемещён над { $name }
library-change-above-wins = Перемещён над { $name } — теперь побеждает в их конфликте
library-change-below = Перемещён под { $name }
library-switch-what-happens = Включится модов: { $on } · выключится: { $off }
library-what-happens = Что произойдёт в папке игры
library-folder-summary = Файлов добавится: { $added } · удалится: { $removed } · сменит номер: { $renumbered }

## Foreign files
library-foreign-title = Найдено в папке модов игры
library-foreign-note = Их положил не deadloader, и он их не тронет, пока вы сами этого не решите.
library-collapse = Свернуть
library-expand = Развернуть
library-take = Взять под управление
library-leave = Оставить как есть
library-left = Оставлено как есть
library-foreign-open = Открыть папку с { $name }
library-foreign-vpk = VPK, файлов: { $count }
library-foreign-vpk-hero = VPK, файлов: { $count } — похоже на скин для { $hero }
library-foreign-unreadable = Файл мода, который deadloader не может прочитать
library-foreign-replaced = Файл, занявший место файла deadloader
library-foreign-dmm = Создан Deadlock Mod Manager — файл настроек, не мод
library-foreign-other = Файл, не мод
library-foreign-folder = Папка, файлов: { $count } — не мод
library-foreign-meta = { $size } · добавлен { $date }
library-taken = { $name } теперь в вашей библиотеке.

## D11
library-d11-title = Взять { $name } под управление?
library-d11-lead = deadloader:
library-d11-step-copy = скопирует его в библиотеку как локальный мод ({ $size }),
library-d11-step-replace = заменит файл в папке игры своей связанной копией,
library-d11-step-enable = включит его в профиле «{ $profile }» в конце списка.
library-d11-name = Название в библиотеке
library-d11-hint = В библиотеке хранится точная копия исходного файла.
library-d11-suggested = { $file } (скин { $hero })

## D10
library-d10-title = Файл игры изменён другой программой
library-d10-what = Другая программа изменила файл игры, который определяет, какие папки загружает Deadlock, — чаще всего это Deadlock Mod Manager. deadloader не перезаписывает чужие изменения, поэтому ваши моды не загрузятся, пока это не исправлено.
library-d10-do = Пусть Steam восстановит исходный файл (проверка файлов игры). Затем deadloader добавит свои строки и сообщит вам.
library-d10-diff = Показать различия
library-d10-expected = Ожидается (оригинал + строки deadloader)
library-d10-found = Сейчас в файле
library-d10-diff-note = Выделены строки, которые добавляет deadloader, и строки, добавленные другой программой.
library-d10-help = Что это за файл?
library-d10-help-text = gameinfo.gi сообщает Deadlock, какие папки загружать. Менеджеры модов добавляют несколько строк, чтобы игра загружала моды из папки addons. Обновления игры сбрасывают файл; deadloader исправляет это автоматически — но только когда файл в исходном виде.
library-d10-wait-title = Steam проверяет файлы Deadlock.
library-d10-wait-text = Когда Steam закончит, вернитесь в deadloader — он проверит файл снова автоматически. Можно проверить и сейчас.
library-d10-fixed = gameinfo.gi настроен — моды снова загружаются.
library-d10-fixed-text = Steam восстановил исходный файл, и deadloader добавил свои строки. Подробности — в «Активность → Файлы».
library-open-file = Открыть файл
library-not-now = Не сейчас
library-verify-steam = Проверить файлы игры в Steam
library-check-again = Проверить снова
library-show-differences = Показать различия

## D5
library-update = Обновить
library-update-on-gb = На GameBanana есть более новая версия.
library-later = Позже
library-d5-title = Обновить { $name }?
library-d5-installed = Установлено
library-d5-new = Новое на GameBanana
library-d5-downloaded = Скачано
library-d5-arrived = В библиотеке, ещё не активно
library-d5-new-meta = Обновлено { $date } · скачать { $size }
library-d5-download = Скачать { $size }
library-d5-changed = В новой версии другой набор файлов. deadloader сопоставил их так:
library-d5-same = Эти файлы будут заменены новыми версиями:
library-d5-map-text = deadloader не смог определить для каждого файла, какой новый его заменяет. Выберите новый файл для каждого или оставьте старый.
library-d5-map-label = Сопоставление файлов
library-d5-col-old = Установленный файл
library-d5-col-new = Новый файл
library-d5-pick = Новый файл для { $file }
library-d5-choose = Выберите…
library-d5-keep = Оставить старый файл
library-d5-tag-replaces = Заменяет
library-d5-tag-same = То же имя
library-d5-tag-new = Новый · на странице мода
library-d5-profiles = { $new } станет активной во всех профилях, где используется { $old }. { $old } останется в библиотеке — вернуться можно в сведениях о моде.
library-d5-install = Установить обновление
library-d5-confirm-map = Использовать эти файлы
library-update-downloading = Обновление { $name } скачивается — оно будет установлено, когда загрузится.
library-updated = { $name } обновлён.
library-updated-text = Активен во всех профилях, где была старая версия. Старая версия сохранена — вернуться можно в сведениях о моде.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name }: включён
library-n12-off = { $name }: выключен
library-n12-above = { $name }: перемещён над { $other }
library-n12-below = { $name }: перемещён под { $other }
library-n12-switched = { $name }: переключён на { $to }
library-n12-files = { $name }: файлы изменены

## S9: the file list of a pending apply
library-show-file-list = Показать список файлов
library-hide-file-list = Скрыть список файлов
library-file-list = Файлы в папке игры
library-files-added = Добавятся · { $count }
library-files-removed = Удалятся · { $count }
library-files-renumbered = Сменят номер · { $count }
