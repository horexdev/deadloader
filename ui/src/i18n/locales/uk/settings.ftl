# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Налаштування
settings-nav = Розділи налаштувань
settings-reset-nav = Скинути всі налаштування…
settings-default = Типово: { $value }
settings-restore = Повернути типове значення
settings-on = Увімк.
settings-off = Вимк.
settings-cancel = Скасувати
settings-remove = Прибрати
settings-remove-item = Прибрати { $item }
settings-add = Додати
settings-open-folder = Відкрити теку
settings-open-folder-at = Відкрити теку: { $path }
settings-copied = Скопійовано в буфер обміну.
settings-folder = Тека
settings-example = Приклад:

settings-section-general = Загальні
settings-section-general-lead = Мова, запуск і масштаб.
settings-section-game = Гра
settings-section-game-lead = Де встановлено Deadlock і що deadloader може там змінювати.
settings-section-catalog = Каталог
settings-section-catalog-lead = Каталог GameBanana, делікатний вміст, зображення й посилання.
settings-section-library = Бібліотека
settings-section-library-lead = Оновлення модів, видалення модів і старі версії.
settings-section-downloads = Завантаження
settings-section-downloads-lead = Черга завантажень, архіви та розпакування.
settings-section-network = Мережа
settings-section-network-lead = Як deadloader спілкується з GameBanana.
settings-section-cache = Кеш
settings-section-cache-lead = Мініатюри та сторінки каталогу, збережені на цьому ПК.
settings-section-logs = Журнали
settings-section-logs-lead = Що зберігає «Активність» і як довго.
settings-section-advanced = Додатково
settings-section-advanced-lead = Файли налаштувань і додаткові дозволені домени.
settings-section-about = Про deadloader
settings-section-about-lead = Версія, ліцензія, перевірка цього завантаження і де лежать ваші дані.

# General
settings-general-group = Мова і запуск
settings-language = Мова
settings-language-help = Мова системи: мова інтерфейсу Windows, якщо вона є в deadloader, інакше англійська.
settings-language-system = Мова системи ({ $language })
settings-startup-profile = Профіль під час запуску
settings-startup-profile-help = Який профіль вибрано під час запуску deadloader. Вибір не застосовує його.
settings-startup-last = Останній вибраний ({ $name })
settings-startup-last-plain = Останній вибраний
settings-app-updates = Перевіряти оновлення deadloader під час запуску
settings-app-updates-help = Під час запуску deadloader один раз запитує GitHub (api.github.com), чи є новіша версія. Оновлення завжди виконується вручну. Ви обрали це під час першого запуску.
settings-app-updates-local = Недоступно: у локальному режимі немає жодних мережевих запитів. Параметр стане доступним, коли ви ввімкнете каталог (Налаштування → Каталог).
settings-zoom = Масштаб
settings-zoom-keys-1 = Також за допомогою Ctrl + коліщатко миші,
settings-zoom-keys-2 = і
settings-zoom-keys-3 = для скидання.
settings-ctrl = Ctrl
settings-percent = { $value } %

# Game
settings-installs-group = Встановлення гри
settings-installs-lead = deadloader сам знаходить встановлення Deadlock у Steam. Кожен профіль використовує одне з них — це змінюється в розділі «Профілі».
settings-install-library = Бібліотека Steam { $library }
settings-install-build = збірка { $build }
settings-install-used =
    { $count ->
        [0] не використовується жодним профілем
        [one] використовується { $count } профілем: { $names }
       *[other] використовується профілями ({ $count }): { $names }
    }
settings-install-selected = { $name } (вибраний)
settings-install-applied = { $name } (застосований тут)
settings-install-steam = Знайдено через Steam
settings-install-manual = Додано вручну
settings-install-known = Тут ще є файли deadloader
settings-install-manual-meta = Додано вручну
settings-gi-ours = gameinfo.gi: гаразд — рядок deadloader на місці
settings-gi-wait = gameinfo.gi: буде налаштовано, коли профіль почне її використовувати
settings-gi-rolled = gameinfo.gi: оригінальний — «Застосувати» знову додасть рядок deadloader
settings-gi-foreign = gameinfo.gi: змінений іншою програмою
settings-gi-missing = gameinfo.gi: не знайдено — можливо, теку перемістили
settings-installs-none = Встановлень поки не знайдено.
settings-add-manual = Додати вручну…
settings-add-manual-hint = Виберіть теку Deadlock, яку не було знайдено.
settings-add-manual-title = Виберіть теку Deadlock
settings-install-added = Встановлення додано.
settings-install-removed = Встановлення прибрано зі списку.
settings-install-removed-text = У тій теці нічого не змінилося.
settings-gameinfo-group = Після оновлень гри
settings-autofix = Виправляти gameinfo.gi автоматично
settings-autofix-help = Оновлення Deadlock скидає файл гри, який наказує завантажувати моди. deadloader робить його резервну копію, повертає свій рядок і повідомляє вас. Якщо вимкнено, спершу запитає банер.
settings-danger-group = Скасування і видалення
settings-rollback = Відкотити теку гри вибраного профілю
settings-rollback-help = Для встановлення вибраного профілю { $profile } ({ $drive }): видаляє файли модів, які туди поклав deadloader ({ $files }), і його рядок у gameinfo.gi. Бібліотека і профілі залишаються; «Застосувати» все поверне.
settings-rollback-help-others = Інші теки гри не зачіпаються; щоб відкотити їх, виберіть профіль, який їх використовує.
settings-rollback-nothing = На { $drive } нічого відкочувати — зараз там немає файлів deadloader.
settings-rollback-no-game = Deadlock не знайдено, тому відкочувати нічого.
settings-rollback-button = Відкотити…
settings-rolled-title = Теку гри на { $drive } відкочено.
settings-rolled-text =
    { $foreign ->
        [0] На { $drive } видалено файли модів ({ $files }) і рядок deadloader у gameinfo.gi. Бібліотеку і профілі збережено — «Застосувати» або «Грати» поверне моди.
       *[other] На { $drive } видалено файли модів ({ $files }), рядок deadloader у gameinfo.gi і сторонні файли ({ $foreign }). Бібліотеку і профілі збережено — «Застосувати» або «Грати» поверне моди.
    }
settings-wipe = Видалити всі дані deadloader
settings-wipe-help = Видаляє бібліотеку, профілі, журнали, кеш і налаштування ({ $size }), потім закриває deadloader. deadloader.exe залишається — видаліть його потім самі.
settings-wipe-button = Видалити всі дані…

# Catalog
settings-catalog-group = Каталог GameBanana
settings-catalog = Використовувати каталог
settings-catalog-help-on = Перегляд і встановлення модів із GameBanana. deadloader спілкується лише з GameBanana, і кожен запит видно в «Активності». Вимкнено — означає локальний режим.
settings-catalog-help-off = Вимкнено — локальний режим: deadloader не робить жодних мережевих запитів. Додавати моди з диска все одно можна.
settings-consent-title = Спершу один крок.
settings-consent-text = Перш ніж увімкнути каталог, deadloader покаже, до яких хостів звертатиметься, і умови GameBanana.
settings-consent-go = Переглянути і ввімкнути
settings-local-on = Локальний режим увімкнено.
settings-local-on-text = Каталог вимкнено, і deadloader не робить мережевих запитів.
settings-ttl = Повторно використовувати завантажені сторінки каталогу протягом
settings-ttl-help = Протягом цього часу повернення до сторінки не звертається до GameBanana знову.
settings-minutes = { $count } хв
settings-hours = { $count } год
settings-prefetch = Завантажувати наступну сторінку заздалегідь
settings-prefetch-help = Коли ви прокрутили близько 70 % результатів, завантажуються наступні, щоб сітка продовжувалася.
settings-nsfw-group = Делікатний вміст
settings-nsfw = Моди з віковим маркуванням (NSFW)
settings-nsfw-help = Приховано: не показуються взагалі. Розмито: розмите зображення з позначкою «Делікатний вміст»; натисніть, щоб показати. Показано: без розмиття.
settings-nsfw-age-done = Вік підтверджено { $date }.
settings-nsfw-age-ask = Під час виходу з режиму «Приховано» вас один раз попросять підтвердити вік.
settings-nsfw-hidden = Приховано
settings-nsfw-blurred = Розмито
settings-nsfw-shown = Показано
settings-images-group = Зображення і посилання
settings-description-images = Зображення в описах модів
settings-description-images-help = Вони надходять із хостів, відмінних від GameBanana, наприклад i.imgur.com. «Після натискання»: заглушка, доки ви на неї не натиснете.
settings-images-click = Після натискання
settings-images-always = Завжди
settings-prechecked = Заздалегідь позначати «Завжди дозволяти зображення з цього хоста»
settings-prechecked-help = У діалозі, що з’являється, коли зображення в описі вперше надходить із нового хоста.
settings-hosts = Дозволені хости зображень
settings-hosts-help = Зображення з цих хостів завантажуються без запитань. В «Активності» їхні запити позначено як «дозволений хост зображень».
settings-hosts-built-in = Вбудований
settings-hosts-none = Хости не додано — кожен новий хост спершу запитує.
settings-host-removed = { $host } прибрано
settings-host-removed-text = Його зображення знову запитуватимуть перед завантаженням.
settings-undo = Скасувати
settings-gif = Відтворювати GIF автоматично
settings-gif-help = Вимкнено: GIF показують перший кадр із кнопкою відтворення.
settings-links = Зовнішні посилання
settings-links-help = Посилання відкриваються в типовому браузері. «Спершу запитати» показує повну адресу перед відкриттям.
settings-links-ask = Спершу запитати
settings-links-open = Відкривати одразу

# Library
settings-updates-group = Оновлення модів
settings-check-updates = Перевіряти оновлення модів під час запуску
settings-check-updates-help = Порівнює ваші моди з GameBanana з каталогом. Ніколи в локальному режимі.
settings-auto-download = Завантажувати оновлення автоматично
settings-auto-download-help = Лише завантаження — deadloader усе одно запитає, перш ніж мод перейде на нову версію.
settings-disk-group = Видалення і старі версії
settings-deletion = Під час видалення мода
settings-deletion-help = У діалозі видалення можна щоразу вибрати інакше.
settings-deletion-permanent = Безповоротно
settings-deletion-bin = До кошика
settings-old-versions = Старі версії
settings-old-versions-help = Кожна завантажена версія зберігається, щоб можна було повернутися. Старі версії мода видаляються в його подробицях.
settings-old-keep-all = Зберігати всі

# Downloads
settings-queue-group = Черга
settings-parallel = Одночасні завантаження
settings-parallel-help = Скільки файлів завантажується одночасно.
settings-speed = Обмеження швидкості
settings-speed-help = Лише для завантаження модів — каталог і мініатюри не обмежуються.
settings-speed-none = Без обмеження
settings-speed-value = { $value } МБ/с
settings-keep-archives = Зберігати архіви після встановлення
settings-keep-archives-help = Вимкнено: файл .zip, .7z або .rar видаляється, щойно його файли .vpk потрапляють до бібліотеки.
settings-unpack-group = Розпакування
settings-limits = Обмеження розпакування
settings-limits-help = Захищають від архівів, які розпаковуються в щось величезне. Якщо архів, якому ви довіряєте, відхилено, збільште обмеження.
settings-limits-default = 2 ГБ · 20× · 256
settings-limit-size = Розмір після розпакування
settings-limit-ratio = Ступінь стиснення
settings-limit-files = Файлів модів в архіві
settings-unit-gb = ГБ
settings-unit-x = ×
settings-unit-files = файлів
settings-unit-s = с
settings-number-error = { $field }: введіть ціле число від { $min } до { $max }. Доти використовується попереднє значення.

# Network
settings-network-local = Локальний режим увімкнено.
settings-network-local-text = deadloader не робить мережевих запитів; ці налаштування почнуть діяти, коли каталог буде ввімкнено.
settings-requests-group = Запити до GameBanana
settings-api-parallel = Одночасні запити до каталогу
settings-api-parallel-help = Одночасні запити до API GameBanana. Зображення і завантаження рахуються окремо.
settings-timeouts = Тайм-аути
settings-timeouts-help = Підключення: здатися, якщо з’єднання не встановлено за цей час. Відповідь: здатися, коли сервер перестає надсилати дані.
settings-timeouts-default = 10 с · 30 с
settings-timeout-connect = Підключення
settings-timeout-read = Відповідь
settings-retries = Повтори
settings-retries-help = Скільки разів невдалий запит повторюється зі зростаючими паузами, перш ніж deadloader покаже помилку.
settings-connection-group = Підключення
settings-proxy = Проксі
settings-proxy-help = Немає: пряме підключення. Системний: проксі, заданий у Windows. Вручну: вкажіть нижче.
settings-proxy-none = Немає
settings-proxy-system = Системний
settings-proxy-manual = Вручну
settings-proxy-host = Адреса
settings-proxy-port = Порт
settings-proxy-hint = HTTP-проксі. Через нього йдуть лише запити deadloader.
settings-proxy-invalid = Введіть ім’я хоста або адресу без імені користувача і пароля, наприклад proxy.example.com.

# Cache
settings-thumbs-group = Мініатюри і сторінки каталогу
settings-cache-limit = Межа кешу
settings-cache-limit-help = Коли кеш заповнено, першими видаляються мініатюри, які ви найдовше не бачили.
settings-size-mb = { $value } МБ
settings-size-gb = { $value } ГБ
settings-cache-now = Поточний розмір
settings-cache-breakdown = Мініатюр: { $thumbnails } · сторінок каталогу: { $pages }
settings-cache-empty = Порожньо — у кеші поки нічого немає.
settings-cache-used = { $used } з { $limit }
settings-cache-clear = Очистити кеш
settings-cache-cleared = Кеш очищено — звільнено { $size }. Мініатюри завантажаться знову, коли знадобляться.
settings-cache-cleared-partly = Файли, які були зайняті ({ $failed }), залишаться до наступного очищення.

# Logs
settings-keep-group = Зберігання журналів
settings-log-days = Зберігати журнали
settings-log-days-help = Старіші записи видаляються під час запуску deadloader.
settings-days = { $count ->
        [one] { $count } день
        [few] { $count } дні
        [many] { $count } днів
       *[other] { $count } дня
    }
settings-year = 1 рік
settings-log-size = Максимальний розмір
settings-log-size-help = Понад цей розмір першими видаляються найстаріші файли журналу.
settings-privacy-group = Конфіденційність
settings-mask = Приховувати ім’я користувача Windows
settings-mask-help = Шляхи в журналах показують { $masked } замість вашого імені — так безпечніше, якщо ви ділитеся журналом.
settings-logs-folder-group = Тека журналів
settings-logs-folder = Де лежать журнали
settings-logs-now = { $size } · файлів: { $files } · найстаріший від { $date }
settings-open-logs = Відкрити теку журналів

# Advanced
settings-advanced-warning = Для досвідчених користувачів.
settings-advanced-warning-text = Помилка тут може змінити поведінку deadloader. Він перевіряє файл перед збереженням і зберігає резервну копію попередньої версії.
settings-files-group = Редагування файлів налаштувань
settings-file-settings = Налаштування
settings-file-manifest = Маніфест теки гри
settings-file-label = Файл для редагування
settings-file-settings-lead = Усе з цих сторінок в одному файлі. Зміни з’являться на сторінках після збереження.
settings-file-manifest-lead = Список того, що deadloader поклав у теку гри. Відкат спирається на нього — хибний запис може залишити файли.
settings-json-error = Це некоректний JSON, тому його не збережено.
settings-json-where = Рядок { $line }, стовпець { $column }: тут або трохи раніше чогось бракує або є зайве — часто кома чи лапки.
settings-json-fix = Виправте це місце і збережіть знову — доти deadloader використовує поточний файл.
settings-json-refused = Ці значення не можна використати, тому нічого не збережено.
settings-json-newer = Файл створено новішою версією deadloader (формат { $found }; ця версія читає до { $supported }), тому його не збережено.
settings-json-invalid = deadloader не може використати цей файл, тому його не збережено.
settings-json-details = Подробиці
settings-problem-unknown-field = { $path }: такого налаштування немає в цій версії.
settings-problem-invalid-value = { $path }: це значення тут неприпустиме.
settings-problem-out-of-range = { $path }: { $found } поза межами { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: «{ $entry }» тут неприпустиме.
settings-problem-duplicate-entry = { $path }: «{ $entry }» вказано двічі.
settings-problem-needs-age = { $path }: для показу NSFW-вмісту спершу потрібно підтвердити вік (Налаштування → Каталог).
settings-problem-local-mode = { $path }: недоступно в локальному режимі.
settings-json-idle = Перевіряється під час збереження.
settings-json-saved = Коректно і збережено. Попередню версію збережено як { $backup }.
settings-json-saved-plain = Коректно і збережено.
settings-json-save = Перевірити і зберегти
settings-json-discard = Скасувати мої правки
settings-json-backup = Перед збереженням попередній файл зберігається як резервна копія; зберігаються останні 10.
settings-domains-group = Додаткові дозволені домени
settings-domains-lead = deadloader спілкується лише з GameBanana. Якщо завантаження перенаправлено на інший хост і його заблоковано, цей хост можна дозволити тут. В «Активності» запити до нього позначено як «додано користувачем».
settings-domains-built-in = Вбудовані:
settings-domain-user = додано користувачем
settings-domains-none = Додаткових доменів немає.
settings-domain-add = Додати домен
settings-domain-hint = Лише ім’я хоста, наприклад files.example.com.
settings-domain-empty = Спершу введіть ім’я хоста, наприклад files.example.com.
settings-domain-not-host = Введіть лише ім’я хоста — без https:// і шляху, наприклад files.example.com.
settings-domain-invalid = «{ $host }» — не ім’я хоста. Перевірте написання, наприклад files.example.com.
settings-domain-built-in = { $host } вбудований і завжди дозволений.
settings-domain-listed = { $host } уже є в списку.

# About
settings-version = версія { $version }
settings-commit = Зібрано з коміту
settings-commit-local = Локальна збірка — не зібрана GitHub Actions
settings-copy-commit = Скопіювати ID коміту
settings-release-page = Сторінка випуску
settings-licence-group = Ліцензія
settings-licence-help = deadloader — вільна програма: ви можете використовувати, вивчати, поширювати і змінювати її на умовах GPL-3. Вона постачається без жодних гарантій.
settings-read-licence = Прочитати ліцензію
settings-source = Вихідний код
settings-verify-group = Як перевірити це завантаження
settings-verify-lead = Дві незалежні перевірки того, що цей deadloader.exe — той, що опубліковано на GitHub.
settings-verify-hash = Порівняти контрольну суму SHA-256
settings-verify-hash-help = Вона має збігатися із сумою на сторінці випуску. Змінена копія могла б показати тут хибне значення, тому обчисліть її і самі:
settings-verify-hash-cmd = У PowerShell, у теці з exe:
settings-verify-attest = Перевірити атестацію збірки
settings-verify-attest-help = Доводить, що GitHub Actions зібрав цей файл з опублікованого вихідного коду. Потрібен GitHub CLI:
settings-copy-hash = Скопіювати суму
settings-copy-command = Скопіювати команду
settings-unsigned = deadloader.exe не підписаний.
settings-unsigned-text = Під час першого запуску Windows SmartScreen може повідомити «Windows захистила ваш комп’ютер». Дві перевірки вище дають змогу переконатися, що файл справжній.
settings-webview-group = Рушій вікна
settings-webview-help = Вікно deadloader малює WebView2 — частина Windows. Windows оновлює його сама. deadloader вимикає:
settings-webview-off-1 = Запити DNS-over-HTTPS
settings-webview-off-2 = Вхід в обліковий запис Microsoft усередині вікна
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Фонові мережеві звернення та оновлення компонентів
settings-webview-off-5 = Звернення до сервера автозаповнення форм
settings-webview-off-6 = Звіти про помилки до Microsoft (domain reliability)
settings-webview-no-route = І вікно взагалі не має виходу в інтернет.
settings-webview-unknown = версія невідома
settings-data-group = Ваші дані
settings-data-folder = Тека даних
settings-data-folder-help = Усе, що зберігає deadloader, лежить в одній теці поруч з exe.
settings-notice = Повідомлення під час першого запуску
settings-notice-help = Версія { $version }, прийнято { $date }, мова: { $language }. Ваш вибір: каталог — { $catalog }, перевірка оновлень — { $updates }.
settings-notice-none = Ще не прийнято.
settings-notice-again = Прочитати знову

# D7
settings-d7-title = Відкотити теку гри на { $drive }?
settings-d7-from =
    { $others ->
        [0] Встановлення вибраного профілю { $profile }. deadloader видалить із
       *[other] Встановлення вибраного профілю { $profile } (її також використовує { $others }). deadloader видалить із
    }
settings-d7-files =
    { $count ->
        [one] { $count } файл мода, який він додав у
        [few] { $count } файли модів, які він додав у
        [many] { $count } файлів модів, які він додав у
       *[other] { $count } файла модів, які він додав у
    }
settings-d7-line = Свій рядок у gameinfo.gi
settings-d7-line-why = — файл стане таким, яким його встановив Steam.
settings-d7-stays = Залишиться як є:
settings-d7-library = Ваша бібліотека — модів: { $mods }, { $size }
settings-d7-profiles =
    { $count ->
        [one] Ваш профіль — { $names }
       *[other] Ваші профілі ({ $count }) — { $names }
    }
settings-d7-other = Встановлення на { $path } ({ $profile }) — не зачіпається. Щоб відкотити його, виберіть профіль, який його використовує, і відкотіть знову.
settings-d7-other-plain = Встановлення на { $path } — не зачіпається.
settings-d7-back = «Застосувати» або «Грати» все поверне.
settings-d7-foreign =
    { $count ->
        [one] Видалити також { $count } файл, створений не deadloader
        [few] Видалити також { $count } файли, створені не deadloader
        [many] Видалити також { $count } файлів, створених не deadloader
       *[other] Видалити також { $count } файла, створених не deadloader
    }
settings-d7-foreign-why = від іншої програми або ручного встановлення. Їх буде видалено безповоротно; deadloader не зможе їх повернути.
settings-d7-confirm = Відкотити
settings-d7-confirm-foreign =
    { $count ->
        [one] Відкотити і видалити { $count } файл
        [few] Відкотити і видалити { $count } файли
        [many] Відкотити і видалити { $count } файлів
       *[other] Відкотити і видалити { $count } файла
    }

# D8
settings-d8-title = Видалити всі дані deadloader?
settings-d8-from = deadloader видалить усе в
settings-d8-from-2 = і потім закриється.
settings-d8-what = Що
settings-d8-contains = Містить
settings-d8-size = Розмір
settings-d8-total = Усього
settings-d8-library = Бібліотека
settings-d8-library-has = модів: { $mods }, файлів: { $files }
settings-d8-profiles = Профілі
settings-d8-logs = Журнали
settings-d8-logs-has = файлів: { $files }, з { $date }
settings-d8-logs-none = немає файлів журналу
settings-d8-cache = Кеш
settings-d8-cache-has = мініатюри і сторінки каталогу
settings-d8-settings = Налаштування
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
       *[other] вкл. резервні копії ({ $backups })
    }
settings-d8-other = Інше
settings-d8-other-has = незавершені завантаження, копії gameinfo.gi, дані рушія вікна
settings-d8-exe = Не видаляється: сам deadloader.exe — видаліть його самі, якщо він більше не потрібен.
settings-d8-rollback = Спершу відкотити всі теки гри, які використовує deadloader (рекомендовано)
settings-d8-rollback-why = Перед видаленням прибирає файли модів deadloader і його рядок у gameinfo.gi з кожної теки гри. Сторонні файли залишаються.
settings-d8-rollback-off = Не позначено: файли модів deadloader залишаться в теках гри, і Deadlock далі їх завантажуватиме — а прибрати їх уже не буде кому.
settings-d8-type-1 = Для підтвердження введіть
settings-d8-type-2 = { "" }
settings-d8-word = видалити
settings-d8-hint = Кнопка видалення стане доступною, коли слово збіжиться.
settings-d8-hint-ok = Підтверджено. Це не можна скасувати.
settings-d8-type-first = Спершу введіть «{ $word }»
settings-d8-deleting = Видалення…
settings-d8-confirm = Видалити і закрити
settings-d8-confirm-rollback = Відкотити, видалити і закрити

# Reset
settings-reset-title = Скинути всі налаштування?
settings-reset-body = Кожне налаштування повернеться до типового значення, включно з дозволеними хостами зображень і додатковими доменами.
settings-reset-keeps = Бібліотека, профілі та журнали не зачіпаються, тека гри залишається як є.
settings-reset-backup = Резервна копія поточних налаштувань зберігається як settings.json.bak-…
settings-reset-confirm = Скинути налаштування
settings-reset-done = Усі налаштування скинуто.
settings-reset-done-text = Резервна копія: { $backup }
