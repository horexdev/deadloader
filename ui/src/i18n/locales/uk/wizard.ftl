# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Хід налаштування
wizard-caption-setup = Перше налаштування
wizard-caption-changed = Оновлене повідомлення
wizard-caption-catalog = Увімкнення каталогу
wizard-caption-review = Повідомлення першого запуску
wizard-language = Мова
wizard-step-welcome = Вітання
wizard-step-risks = Ризики
wizard-step-network = Мережа
wizard-step-privacy = Приватність
wizard-step-updates = Оновлення
wizard-step-setup = Налаштування
wizard-step-of = Крок { $n } з 6
wizard-changed-of =
    { $count ->
        [one] { $count } змінений крок
        [few] { $count } змінені кроки
       *[other] { $count } змінених кроків
    }
wizard-meta-changed = Змінено
wizard-meta-kept = Без змін
wizard-back = Назад
wizard-next = Далі
wizard-finish = Завершити налаштування
wizard-continue = Продовжити
wizard-close = Закрити
wizard-cancel = Скасувати
wizard-to-catalog = Далі відкриється каталог.
wizard-to-library = Далі відкриється бібліотека (локальний режим).

# Step 1
wizard-s1-title = Вітаємо в deadloader
wizard-s1-lead = deadloader встановлює моди для Deadlock з GameBanana або з вашого диска й показує кожну зміну, яку робить.
wizard-f1a = Він кладе файли модів у папку модів гри й додає один рядок у
wizard-f1b = — файл гри, який каже Deadlock завантажувати моди.
wizard-f2 = Усе можна скасувати: відкат прибирає з гри все, що туди додав deadloader.
wizard-f3a = Усі його дані зберігаються в папці
wizard-f3b = поруч із
wizard-f3c = .
wizard-f4 = deadloader не пов’язаний із Valve чи GameBanana.
wizard-f5 = Вільна програма за ліцензією GPL-3, без жодних гарантій.
wizard-licence = Ліцензія (GPL-3)
wizard-source = Вихідний код

# Step 2
wizard-s2-title = Моди — на ваш власний ризик
wizard-r1 = У Valve немає офіційних правил щодо модів у Deadlock. Користуватися модами чи ні — ваше рішення і ваш ризик.
wizard-r2 = Оновлення гри можуть ламати моди. deadloader знову налаштовується після оновлення, але мод може не працювати, доки автор його не оновить.
wizard-r3 = Моди створюють інші люди за власними ліцензіями. deadloader не може ручатися за їхній вміст; для кожного файлу він показує результат антивірусної перевірки GameBanana.
wizard-agree = Я розумію, що deadloader змінює файли гри і що я використовую моди на власний ризик.
wizard-agree-note = Так ви підтверджуєте кроки 1 і 2. Без цього продовжити не можна.
wizard-agree-error = Поставте цю позначку, щоб продовжити.
wizard-agree-reason = Спершу поставте позначку.

# Step 3
wizard-s3-title = Мережа і GameBanana
wizard-s3-lead = Каталог звертається безпосередньо до неофіційного API GameBanana — і більше нікуди. Ось усі адреси, до яких він підключається:
wizard-hosts = Адреси
wizard-host-api = каталог і сторінки модів
wizard-host-images = мініатюри й галерея
wizard-host-files = завантаження
wizard-host-cache = завантаження (files.gamebanana.com перенаправляє туди)
wizard-host-new = Новий
wizard-s3-images = Зображення з інших адрес в описах модів завантажуються лише тоді, коли ви дозволите кожну адресу. Кожен запит видно в розділі
wizard-s3-images-end = .
wizard-activity = Активність
wizard-s3-tos = Користуватися каталогом — означає користуватися GameBanana: діють їхні умови використання, а GameBanana вимагає, щоб користувачеві було щонайменше 13 років.
wizard-tos = Умови використання GameBanana
wizard-choice = Як ви хочете почати?
wizard-use-catalog = Використовувати каталог
wizard-use-catalog-text = Шукати й встановлювати моди з GameBanana. deadloader підключається лише до адрес вище.
wizard-local = Не зараз — локальний режим
wizard-local-text = Жодних мережевих запитів; моди лише з диска. Каталог можна ввімкнути пізніше в налаштуваннях.
wizard-choice-error = Виберіть один із двох варіантів.
wizard-choice-reason = Спершу виберіть варіант.
wizard-last-time = Ваш попередній вибір
wizard-changed-title = Що змінилося ({ $date })
wizard-changed-foot = Інші кроки ви підтвердили { $date }; вони не змінилися.

# Step 4
wizard-s4-title = Ваша приватність
wizard-p1 = Ні облікового запису, ні телеметрії. deadloader нічого нікуди не надсилає, крім запитів до GameBanana з попереднього кроку.
wizard-p1-local = Ні облікового запису, ні телеметрії. Ви вибрали локальний режим, тож deadloader не надсилає взагалі нічого.
wizard-p2a = Журнали залишаються на цьому ПК, у папці
wizard-p2b = — і приховують ваше ім’я користувача Windows:
wizard-p3 = Вікно deadloader малює Microsoft WebView2 — частина Windows. deadloader вимикає власні онлайн-функції WebView2; Windows оновлює WebView2 незалежно від deadloader.
wizard-p4 = Що зробив deadloader, завжди можна переглянути в розділі
wizard-p4-end = .

# Step 5
wizard-s5-title = Оновлення deadloader
wizard-u1 = Під час запуску deadloader може перевіряти на GitHub, чи немає його новішої версії.
wizard-updates-label = Перевіряти на GitHub нові версії deadloader під час запуску
wizard-updates-on = Увімк.
wizard-updates-off = Вимк. (типово)
wizard-updates-na = Недоступно в локальному режимі
wizard-u2a = Якщо ввімкнено, deadloader під час кожного запуску робить один запит до
wizard-u2b = і повідомляє, якщо вийшла нова версія.
wizard-u3 = Оновлення завжди ручне: нову версію ви завантажуєте з GitHub самі. deadloader ніколи не замінює себе сам.
wizard-u4 = Недоступно: у локальному режимі немає жодних мережевих запитів. Опція стане доступною, коли ви ввімкнете каталог у налаштуваннях.
wizard-u5 = Це можна змінити пізніше в розділі Налаштування → Загальні.

# Step 6
wizard-s6-title = Налаштування
wizard-s6-lead = deadloader пошукав гру й дещо перевірив. Поки нічого не змінено, і все це можна змінити пізніше в налаштуваннях.
wizard-checking = Перевірка…
wizard-topic-game = Гра
wizard-topic-exe = Де лежить deadloader
wizard-topic-disk = Місце на диску
wizard-topic-foreign = Папка модів гри
wizard-topic-gameinfo = Файл гри для модів
wizard-game-found = Deadlock знайдено
wizard-game-chosen = Папку вибрано
wizard-change = Змінити…
wizard-game-missing = Deadlock не знайдено
wizard-game-missing-text = Steam або гру не встановлено, або папку гри перемістили. Виберіть папку гри самі — зараз або пізніше в розділі Налаштування → Гра.
wizard-browse = Огляд…
wizard-browse-title = Виберіть папку Deadlock
wizard-help = Довідка
wizard-help-a = У Steam: клацніть правою кнопкою на Deadlock → Керувати → Переглянути локальні файли. Виберіть цю папку; зазвичай її шлях закінчується на
wizard-help-b = .
wizard-exe-ok = У постійній папці
wizard-exe-downloads = deadloader лежить у папці «Завантаження»
wizard-exe-desktop = deadloader лежить на робочому столі
wizard-exe-warn-a = Його дані зберігаються поруч з exe, тож під час прибирання зникнуть ваші бібліотека й профілі. Закрийте deadloader, перенесіть усю папку в постійне місце, наприклад
wizard-exe-warn-b = , і запустіть його звідти.
wizard-open-folder = Відкрити папку
wizard-opened = Відкрито в Провіднику
wizard-linked = Моди пов’язуються посиланнями — без зайвого місця на диску
wizard-linked-text = deadloader і гра на одному диску ({ $drive }).
wizard-copied = Моди копіюватимуться, а не пов’язуватимуться
wizard-copied-text = deadloader на диску { $data }, а гра на { $game }. Кожен увімкнений мод займе місце вдруге; бібліотека показує це як «скопійовано — +N МБ».
wizard-learn-more = Докладніше
wizard-drives-more = Посилання дає грі брати файл із бібліотеки без другої копії. Windows може пов’язувати лише файли на одному диску. Якщо deadloader теж буде на { $game }, нічого не копіюватиметься.
wizard-foreign-none = Чужих файлів модів немає
wizard-foreign-none-text = У папці модів немає нічого, чого не знає deadloader.
wizard-foreign-some =
    { $count ->
        [one] { $count } файл з іншого джерела
        [few] { $count } файли з іншого джерела
       *[other] { $count } файлів з іншого джерела
    }
wizard-foreign-some-text = Їх поклав не deadloader — імовірно, інший менеджер модів або хтось уручну. Він залишить їх як є; переглянути їх можна пізніше в бібліотеці.
wizard-show-files = Показати файли
wizard-hide-files = Сховати файли
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Буде налаштовано
wizard-gi-setup-a = deadloader додасть один рядок у
wizard-gi-setup-b = під час першого застосування модів. Зараз нічого не змінюється.
wizard-gi-ours = Уже налаштовано
wizard-gi-ours-text = Рядок deadloader уже на місці.
wizard-gi-foreign = Змінено іншою програмою
wizard-gi-foreign-text = Він не схожий на оригінал — часто так робить Deadlock Mod Manager. deadloader не перезаписує чужі зміни й після налаштування пояснить, що робити.
wizard-what-is = Що це?
wizard-gi-more = — файл гри, який каже Deadlock, які папки завантажувати. Зазвичай допомагає перевірка цілісності файлів гри в Steam; після неї deadloader додасть свій рядок.

# E1
wizard-nw-title = deadloader не може зберегти свої дані.
wizard-nw-why-a = Папка поруч із
wizard-nw-why-b = доступна лише для читання: Windows не дозволяє програмам писати туди (наприклад, у Program Files).
wizard-nw-path = Папка deadloader
wizard-nw-do = Що робити
wizard-nw-s1 = Закрийте deadloader.
wizard-nw-s2a = Перенесіть усю папку туди, де можна писати, наприклад
wizard-nw-s2b = . Не використовуйте Program Files.
wizard-nw-s3a = Запустіть
wizard-nw-s3b = з нового місця.
wizard-nw-why-folder = deadloader зберігає все поруч з exe, щоб нічого не розкидати по ПК: видаливши папку, ви повністю видалите deadloader.
wizard-nw-details = Подробиці
wizard-nw-copy = Скопіювати подробиці
wizard-nw-copied = Скопійовано
wizard-nw-no-log = Не записано в журнал — deadloader не може туди писати.
wizard-exit = Вийти
