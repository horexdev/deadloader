# The window frame and the sidebar: title bar, navigation, profile, game and network status, Play, D13.
nav-main = Основная навигация
nav-catalog = Каталог
nav-library = Библиотека
nav-profiles = Профили
nav-downloads = Загрузки
nav-activity = Активность
nav-settings = Настройки
nav-pending = Ожидающие изменения
nav-pending-count = Ожидающие изменения: { $count }
nav-downloads-count = Загрузки · { $count }
sidebar-collapse = Свернуть боковую панель
sidebar-expand = Развернуть боковую панель
sidebar-close = Закрыть боковую панель
profile-applied = Применён в игре
profile-not-applied = Выбран · не применён
profile-button = Профиль { $name } · { $state }
game-found = Deadlock найден
game-running = Deadlock запущен
game-missing = Deadlock не найден
game-on-drive = { $status } · { $drive }
game-at-path = { $status } — { $path }
network-online = В сети
network-offline = Нет сети
network-offline-detail = сохранённый каталог от { $time }
network-local = Локальный режим
network-local-detail = без запросов
network-title-online = В сети — открыть «Активность»
network-title-running =
    В сети — { $count ->
        [one] выполняется { $count } запрос
        [few] выполняются { $count } запроса
        [many] выполняется { $count } запросов
       *[other] выполняется { $count } запроса
    } — открыть «Активность»
network-title-offline = Нет сети — показан каталог, сохранённый в { $time } — открыть «Активность»
network-title-local = Локальный режим — без сетевых запросов — открыть «Активность»
play = Играть
play-applying = Применение…
play-running = Deadlock уже запущен
play-missing = Deadlock не найден — сначала выберите папку игры
window-minimize = Свернуть
window-maximize = Развернуть
window-close = Закрыть
play-confirm-title = Применить «{ $profile }» и запустить Deadlock?
play-confirm-off =
    { $count ->
        [one] { $count } мод будет выключен:
        [few] { $count } мода будут выключены:
        [many] { $count } модов будут выключены:
       *[other] { $count } мода будут выключены:
    }
play-confirm-on =
    { $count ->
        [one] { $count } мод будет включён: { $mods }.
        [few] { $count } мода будут включены: { $mods }.
        [many] { $count } модов будут включены: { $mods }.
       *[other] { $count } мода будут включены: { $mods }.
    }
play-confirm-keep = «{ $applied }» останется в ваших профилях — к нему можно вернуться в любой момент.
play-confirm-cancel = Отмена
play-confirm-play = Применить и играть
toast-dismiss = Скрыть
