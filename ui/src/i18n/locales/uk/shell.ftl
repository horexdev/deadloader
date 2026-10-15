# The window frame and the sidebar: title bar, navigation, profile, game and network status, Play, D13.
nav-main = Основна навігація
nav-catalog = Каталог
nav-library = Бібліотека
nav-profiles = Профілі
nav-downloads = Завантаження
nav-activity = Активність
nav-settings = Налаштування
nav-pending = Зміни в очікуванні
nav-pending-count = Зміни в очікуванні: { $count }
nav-downloads-count = Завантаження · { $count }
sidebar-collapse = Згорнути бічну панель
sidebar-expand = Розгорнути бічну панель
sidebar-close = Закрити бічну панель
profile-applied = Застосовано в грі
profile-not-applied = Вибрано · не застосовано
profile-button = Профіль { $name } · { $state }
game-found = Deadlock знайдено
game-running = Deadlock запущено
game-missing = Deadlock не знайдено
game-on-drive = { $status } · { $drive }
game-at-path = { $status } — { $path }
network-online = У мережі
network-offline = Немає мережі
network-offline-detail = збережений каталог від { $time }
network-local = Локальний режим
network-local-detail = без запитів
network-title-online = У мережі — відкрити «Активність»
network-title-offline = Немає мережі — показано каталог, збережений о { $time } — відкрити «Активність»
network-title-local = Локальний режим — без мережевих запитів — відкрити «Активність»
play = Грати
play-applying = Застосування…
play-running = Deadlock уже запущено
play-missing = Deadlock не знайдено — спочатку виберіть папку гри
window-minimize = Згорнути
window-maximize = Розгорнути
window-close = Закрити
play-confirm-title = Застосувати «{ $profile }» і запустити Deadlock?
play-confirm-off =
    { $count ->
        [one] { $count } мод буде вимкнено:
        [few] { $count } моди буде вимкнено:
        [many] { $count } модів буде вимкнено:
       *[other] { $count } мода буде вимкнено:
    }
play-confirm-on =
    { $count ->
        [one] { $count } мод буде увімкнено: { $mods }.
        [few] { $count } моди буде увімкнено: { $mods }.
        [many] { $count } модів буде увімкнено: { $mods }.
       *[other] { $count } мода буде увімкнено: { $mods }.
    }
play-confirm-keep = «{ $applied }» залишиться у ваших профілях — до нього можна повернутися будь-коли.
play-confirm-cancel = Скасувати
play-confirm-play = Застосувати й грати
toast-dismiss = Сховати
