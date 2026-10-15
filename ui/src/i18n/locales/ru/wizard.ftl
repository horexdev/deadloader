# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Ход настройки
wizard-caption-setup = Первая настройка
wizard-caption-changed = Обновлённое уведомление
wizard-caption-catalog = Включение каталога
wizard-caption-review = Уведомление первого запуска
wizard-language = Язык
wizard-step-welcome = Приветствие
wizard-step-risks = Риски
wizard-step-network = Сеть
wizard-step-privacy = Приватность
wizard-step-updates = Обновления
wizard-step-setup = Настройка
wizard-step-of = Шаг { $n } из 6
wizard-changed-of =
    { $count ->
        [one] { $count } изменённый шаг
        [few] { $count } изменённых шага
       *[other] { $count } изменённых шагов
    }
wizard-meta-changed = Изменён
wizard-meta-kept = Без изменений
wizard-back = Назад
wizard-next = Далее
wizard-finish = Завершить настройку
wizard-continue = Продолжить
wizard-close = Закрыть
wizard-cancel = Отмена
wizard-to-catalog = Дальше откроется каталог.
wizard-to-library = Дальше откроется библиотека (локальный режим).

# Step 1
wizard-s1-title = Добро пожаловать в deadloader
wizard-s1-lead = deadloader устанавливает моды для Deadlock с GameBanana или с вашего диска и показывает каждое изменение, которое делает.
wizard-f1a = Он кладёт файлы модов в папку модов игры и добавляет одну строку в
wizard-f1b = — файл игры, который велит Deadlock загружать моды.
wizard-f2 = Всё можно отменить: откат убирает из игры всё, что туда добавил deadloader.
wizard-f3a = Все его данные хранятся в папке
wizard-f3b = рядом с
wizard-f3c = .
wizard-f4 = deadloader не связан с Valve или GameBanana.
wizard-f5 = Свободная программа под лицензией GPL-3, без каких-либо гарантий.
wizard-licence = Лицензия (GPL-3)
wizard-source = Исходный код

# Step 2
wizard-s2-title = Моды — на ваш страх и риск
wizard-r1 = У Valve нет официальных правил для модов в Deadlock. Пользоваться ли модами — ваше решение и ваш риск.
wizard-r2 = Обновления игры могут ломать моды. deadloader заново настраивается после обновления, но мод может не работать, пока автор его не обновит.
wizard-r3 = Моды делают другие люди под своими лицензиями. deadloader не может ручаться за их содержимое; для каждого файла он показывает результат антивирусной проверки GameBanana.
wizard-agree = Я понимаю, что deadloader изменяет файлы игры и что я использую моды на свой страх и риск.
wizard-agree-note = Так вы подтверждаете шаги 1 и 2. Без этого продолжить нельзя.
wizard-agree-error = Поставьте эту галочку, чтобы продолжить.
wizard-agree-reason = Сначала поставьте галочку.

# Step 3
wizard-s3-title = Сеть и GameBanana
wizard-s3-lead = Каталог обращается напрямую к неофициальному API GameBanana — и больше никуда. Вот все адреса, к которым он подключается:
wizard-hosts = Адреса
wizard-host-api = каталог и страницы модов
wizard-host-images = миниатюры и галерея
wizard-host-files = загрузки
wizard-host-cache = загрузки (files.gamebanana.com перенаправляет туда)
wizard-host-new = Новый
wizard-s3-images = Картинки с других адресов в описаниях модов загружаются, только когда вы разрешите каждый адрес. Каждый запрос виден в разделе
wizard-s3-images-end = .
wizard-activity = Активность
wizard-s3-tos = Пользоваться каталогом — значит пользоваться GameBanana: действуют их условия использования, а GameBanana требует, чтобы пользователю было не меньше 13 лет.
wizard-tos = Условия использования GameBanana
wizard-choice = Как вы хотите начать?
wizard-use-catalog = Использовать каталог
wizard-use-catalog-text = Искать и устанавливать моды с GameBanana. deadloader подключается только к адресам выше.
wizard-local = Не сейчас — локальный режим
wizard-local-text = Никаких сетевых запросов; моды только с диска. Каталог можно включить позже в настройках.
wizard-choice-error = Выберите один из двух вариантов.
wizard-choice-reason = Сначала выберите вариант.
wizard-last-time = Ваш прошлый выбор
wizard-changed-title = Что изменилось ({ $date })
wizard-changed-foot = Остальные шаги вы подтвердили { $date }; они не изменились.

# Step 4
wizard-s4-title = Ваша приватность
wizard-p1 = Ни учётной записи, ни телеметрии. deadloader ничего никуда не отправляет, кроме запросов к GameBanana из прошлого шага.
wizard-p1-local = Ни учётной записи, ни телеметрии. Вы выбрали локальный режим, поэтому deadloader не отправляет вообще ничего.
wizard-p2a = Журналы остаются на этом ПК, в папке
wizard-p2b = — и скрывают ваше имя пользователя Windows:
wizard-p3 = Окно deadloader рисует Microsoft WebView2 — часть Windows. deadloader отключает собственные онлайн-функции WebView2; Windows обновляет WebView2 независимо от deadloader.
wizard-p4 = Что сделал deadloader, всегда можно посмотреть в разделе
wizard-p4-end = .

# Step 5
wizard-s5-title = Обновления deadloader
wizard-u1 = При запуске deadloader может проверять на GitHub, нет ли его новой версии.
wizard-updates-label = Проверять на GitHub новые версии deadloader при запуске
wizard-updates-on = Вкл.
wizard-updates-off = Выкл. (по умолчанию)
wizard-updates-na = Недоступно в локальном режиме
wizard-u2a = Если включено, deadloader при каждом запуске делает один запрос к
wizard-u2b = и сообщает, если вышла новая версия.
wizard-u3 = Обновление всегда ручное: новую версию вы скачиваете с GitHub сами. deadloader никогда не заменяет себя сам.
wizard-u4 = Недоступно: в локальном режиме нет никаких сетевых запросов. Опция станет доступна, когда вы включите каталог в настройках.
wizard-u5 = Это можно изменить позже в разделе Настройки → Общие.

# Step 6
wizard-s6-title = Настройка
wizard-s6-lead = deadloader поискал игру и кое-что проверил. Пока ничего не изменено, и всё это можно поменять позже в настройках.
wizard-checking = Проверка…
wizard-topic-game = Игра
wizard-topic-exe = Где лежит deadloader
wizard-topic-disk = Место на диске
wizard-topic-foreign = Папка модов игры
wizard-topic-gameinfo = Файл игры для модов
wizard-game-found = Deadlock найден
wizard-game-chosen = Папка выбрана
wizard-change = Изменить…
wizard-game-missing = Deadlock не найден
wizard-game-missing-text = Steam или игра не установлены, или папку игры переместили. Выберите папку игры сами — сейчас или позже в разделе Настройки → Игра.
wizard-browse = Обзор…
wizard-browse-title = Выберите папку Deadlock
wizard-help = Справка
wizard-help-a = В Steam: правый щелчок по Deadlock → Управление → Просмотреть локальные файлы. Выберите эту папку; обычно её путь заканчивается на
wizard-help-b = .
wizard-exe-ok = В постоянной папке
wizard-exe-downloads = deadloader лежит в папке «Загрузки»
wizard-exe-desktop = deadloader лежит на рабочем столе
wizard-exe-warn-a = Его данные хранятся рядом с exe, поэтому при уборке пропадут ваши библиотека и профили. Закройте deadloader, перенесите всю папку в постоянное место, например
wizard-exe-warn-b = , и запустите его оттуда.
wizard-open-folder = Открыть папку
wizard-opened = Открыто в Проводнике
wizard-linked = Моды связываются ссылками — без лишнего места на диске
wizard-linked-text = deadloader и игра на одном диске ({ $drive }).
wizard-copied = Моды будут копироваться, а не связываться
wizard-copied-text = deadloader на диске { $data }, а игра на { $game }. Каждый включённый мод займёт место второй раз; библиотека показывает это как «скопировано — +N МБ».
wizard-learn-more = Подробнее
wizard-drives-more = Ссылка позволяет игре брать файл из библиотеки без второй копии. Windows может связывать только файлы на одном диске. Если deadloader тоже будет на { $game }, ничего копироваться не будет.
wizard-foreign-none = Чужих файлов модов нет
wizard-foreign-none-text = В папке модов нет ничего, чего не знает deadloader.
wizard-foreign-some =
    { $count ->
        [one] { $count } файл из другого источника
        [few] { $count } файла из другого источника
       *[other] { $count } файлов из другого источника
    }
wizard-foreign-some-text = Их положил не deadloader — вероятно, другой менеджер модов или кто-то вручную. Он оставит их как есть; посмотреть их можно позже в библиотеке.
wizard-show-files = Показать файлы
wizard-hide-files = Скрыть файлы
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Будет настроен
wizard-gi-setup-a = deadloader добавит одну строку в
wizard-gi-setup-b = при первом применении модов. Сейчас ничего не меняется.
wizard-gi-ours = Уже настроен
wizard-gi-ours-text = Строка deadloader уже на месте.
wizard-gi-foreign = Изменён другой программой
wizard-gi-foreign-text = Он не похож на оригинал — часто так делает Deadlock Mod Manager. deadloader не перезаписывает чужие изменения и после настройки объяснит, что делать.
wizard-what-is = Что это?
wizard-gi-more = — файл игры, который говорит Deadlock, какие папки загружать. Обычно помогает проверка целостности файлов игры в Steam; после неё deadloader добавит свою строку.

# E1
wizard-nw-title = deadloader не может сохранить свои данные.
wizard-nw-why-a = Папка рядом с
wizard-nw-why-b = доступна только для чтения: Windows не разрешает программам писать туда (например, в Program Files).
wizard-nw-path = Папка deadloader
wizard-nw-do = Что делать
wizard-nw-s1 = Закройте deadloader.
wizard-nw-s2a = Перенесите всю папку туда, где можно писать, например
wizard-nw-s2b = . Не используйте Program Files.
wizard-nw-s3a = Запустите
wizard-nw-s3b = из нового места.
wizard-nw-why-folder = deadloader хранит всё рядом с exe, чтобы ничего не разбрасывать по ПК: удалив папку, вы полностью удалите deadloader.
wizard-nw-details = Подробности
wizard-nw-copy = Скопировать подробности
wizard-nw-copied = Скопировано
wizard-nw-no-log = Не записано в журнал — deadloader не может туда писать.
wizard-exit = Выйти
