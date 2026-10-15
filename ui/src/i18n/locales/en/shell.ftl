# The window frame and the sidebar: title bar, navigation, profile, game and network status, Play, D13.
nav-main = Main
nav-catalog = Catalog
nav-library = Library
nav-profiles = Profiles
nav-downloads = Downloads
nav-activity = Activity
nav-settings = Settings
nav-pending = Pending changes
nav-pending-count = Pending changes: { $count }
nav-downloads-count = Downloads · { $count }
sidebar-collapse = Collapse sidebar
sidebar-expand = Expand sidebar
sidebar-close = Close sidebar
profile-applied = Applied to the game
profile-not-applied = Selected · not applied
profile-button = Profile { $name } · { $state }
game-found = Deadlock found
game-running = Deadlock is running
game-missing = Deadlock not found
game-on-drive = { $status } · { $drive }
game-at-path = { $status } — { $path }
network-online = Online
network-offline = Offline
network-offline-detail = saved catalog from { $time }
network-local = Local mode
network-local-detail = no requests
network-title-online = Online — open Activity
network-title-running =
    Online — { $count ->
        [one] { $count } request running
       *[other] { $count } requests running
    } — open Activity
network-title-offline = Offline — showing saved catalog from { $time } — open Activity
network-title-local = Local mode — no network requests — open Activity
play = Play
play-applying = Applying…
play-running = Deadlock is already running
play-missing = Deadlock wasn’t found — choose the game folder first
window-minimize = Minimise
window-maximize = Maximise
window-close = Close
play-confirm-title = Apply { $profile } and start Deadlock?
play-confirm-off =
    { $count ->
        [one] 1 mod will be turned off:
       *[other] { $count } mods will be turned off:
    }
play-confirm-on =
    { $count ->
        [one] 1 mod will be turned on: { $mods }.
       *[other] { $count } mods will be turned on: { $mods }.
    }
play-confirm-keep = { $applied } stays in your profiles — you can switch back any time.
play-confirm-cancel = Cancel
play-confirm-play = Apply and play
toast-dismiss = Dismiss
