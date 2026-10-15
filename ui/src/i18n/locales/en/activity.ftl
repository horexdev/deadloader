# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Activity
activity-sub =
    { $days ->
        [one] Everything deadloader did on this PC: network requests, file changes and app events. Logs stay on this PC for 1 day.
       *[other] Everything deadloader did on this PC: network requests, file changes and app events. Logs stay on this PC for { $days } days.
    }
activity-open-logs = Open logs folder
activity-logs-opened = Logs folder opened in File Explorer.

## Summary
activity-summary = Today at a glance
activity-net-today = Network today
activity-hosts-label = Hosts contacted today — select one to filter
activity-hosts-contacted =
    { $count ->
        [one] 1 host contacted
       *[other] { $count } hosts contacted
    }
activity-host-title =
    { $count ->
        [one] 1 request · { $source } — select to filter
       *[other] { $count } requests · { $source } — select to filter
    }
activity-net-none-local = none — local mode
activity-net-none-yet = none yet
activity-nothing-else = Nothing else.
activity-blocked-lead =
    { $count ->
        [one] 1 request to
       *[other] { $count } requests to
    }
activity-blocked-tail =
    { $count ->
        [one] was blocked before any connection was made.
       *[other] were blocked before any connection was made.
    }
activity-blocked-short =
    { $count ->
        [one] 1 request blocked, never contacted.
       *[other] { $count } requests blocked, never contacted.
    }
activity-show-entry = Show entry
activity-game-today = Game folder today
activity-show-in-files = Show in Files
activity-game-none = no changes
activity-no-game-changes = deadloader hasn’t changed anything in the game folder yet.
activity-game-changes =
    { $count ->
        [one] 1 change
       *[other] { $count } changes
    }
activity-game-batches =
    { $count ->
        [one] in 1 batch
       *[other] in { $count } batches
    }
activity-game-failed =
    { $count ->
        [one] 1 failed
       *[other] { $count } failed
    }
activity-game-hardlink = { $count } linked
activity-game-copy = { $count } copied
activity-game-delete = { $count } removed
activity-game-rename = { $count } renamed
activity-game-write = { $count } written
activity-game-backup = { $count } backed up
activity-game-restore = { $count } restored
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi updated once
       *[other] gameinfo.gi updated { $count } times
    }
activity-no-requests-today = No requests today.
activity-no-requests-yet = No requests yet.
activity-local-short = The catalog is off, so deadloader doesn’t connect to anything.
activity-never-contacted = deadloader hasn’t contacted any host since it was installed.

## Tabs, filters and search
activity-tab-network = Network
activity-tab-files = Files
activity-tab-app = App
activity-tab-off = off
activity-tab-errors =
    { $count ->
        [one] · 1 error
       *[other] · { $count } errors
    }
activity-search = Search log
activity-search-clear = Clear search
activity-matches =
    { $count ->
        [one] 1 match
       *[other] { $count } matches
    }
activity-matches-more = { $count }+ matches
activity-f-time = Time
activity-f-purpose = Purpose
activity-f-host = Host
activity-f-op = Operation
activity-f-level = Level
activity-f-result = Result
activity-all = All
activity-ok = OK
activity-errors = Errors
activity-range-hour = Last hour
activity-range-today = Today
activity-range-7d = Last 7 days
activity-range-30d = Last 30 days
activity-clear-filters = Clear filters
activity-clear-all = Clear search and filters
activity-follow = Follow live
activity-follow-tip = Show new entries at the top as they happen
activity-follow-on = New entries appear at the top
activity-follow-paused = Paused at { $time }
activity-show-new =
    { $count ->
        [one] Show 1 new
       *[other] Show { $count } new
    }

## Tables
activity-net-aria = Network requests, newest first
activity-files-aria = File operations, newest first
activity-app-aria = App events, newest first
activity-c-redirects = Redirects
activity-c-time = Time
activity-c-purpose = Purpose
activity-c-request = Request
activity-c-status = Status
activity-c-size = Size
activity-c-duration = Duration
activity-c-started-by = Started by
activity-c-source = Host source
activity-c-op = Operation
activity-c-from-to = From → to
activity-c-result = Result
activity-c-change = Part of change
activity-c-level = Level
activity-c-message = Message
activity-to = to
activity-loading-more = Loading more…
activity-p-catalog = Catalog
activity-p-thumbnail = Thumbnail
activity-p-description-image = Description image
activity-p-download = Download
activity-p-update-check = Update check
activity-src-builtin = Built-in
activity-src-github = GitHub
activity-src-user = User-added
activity-src-user-image = Image host
activity-src-blocked = Not allowed
activity-src-long-builtin = Built-in — GameBanana, part of deadloader
activity-src-long-github = GitHub — deadloader’s own update check, on in Settings
activity-src-long-user = User-added — you added it in Settings → Advanced
activity-src-long-user-image = Allowed image host — you allowed it when loading a description image
activity-src-long-blocked = Not on the list of allowed hosts
activity-by-auto = Auto
activity-by-user = You
activity-op-hardlink = Link
activity-op-copy = Copy
activity-op-rename = Rename
activity-op-delete = Delete
activity-op-write = Write
activity-op-backup = Backup
activity-op-restore = Restore
activity-lv-debug = Debug
activity-lv-info = Info
activity-lv-warn = Warning
activity-lv-error = Error
activity-st-blocked = Blocked
activity-st-blocked-long = Blocked — not contacted
activity-st-failed = Failed
activity-redirects =
    { $count ->
        [one] 1 redirect
       *[other] { $count } redirects
    }
activity-retries =
    { $count ->
        [one] 1 retry
       *[other] { $count } retries
    }
activity-show-redirects = Show redirects
activity-hide-redirects = Hide redirects
activity-hop-request = Hop 1 · request
activity-hop-redirect = Hop { $n } · redirect
activity-hop-retry = Hop { $n } · try { $try }
activity-ms = { $ms } ms
activity-seconds = { $s } s

## Empty and local mode
activity-no-match = No entries match
activity-no-match-text = Try another time range or clear the filters and search.
activity-empty-title = Nothing yet
activity-empty-text = deadloader writes every network request, every file change in the game folder and every app event here as it happens. Nothing has happened yet.
activity-local-title = Local mode — no requests are made
activity-local-why = The catalog is off, so deadloader doesn’t connect to anything — not to GameBanana, not anywhere else. File changes and app events are still logged in the other tabs.
activity-local-do = Turn the catalog on in Settings to browse GameBanana. Every request will then show up here.
activity-local-history = The requests below were made before local mode was turned on.
activity-enable-catalog = Enable catalog

## Entry details
activity-details = Log entry details
activity-close-details = Close details
activity-close-esc = Close (Esc)
activity-why = Why:
activity-todo = What to do:
activity-open-advanced = Open Settings → Advanced
activity-full-record = Full record
activity-hops-title = Redirects · { $count }
activity-user-hidden = Your Windows user name is hidden in logs:
activity-settings-logs = Settings → Logs
activity-copy = Copy entry
activity-copied = Copied
activity-kind-network = Network request
activity-kind-files = File operation
activity-kind-app = App event
activity-d-time = Time
activity-d-purpose = Purpose
activity-d-started-by = Started by
activity-d-request = Request
activity-d-source = Host source
activity-d-status = Status
activity-d-size = Size
activity-d-duration = Duration
activity-d-range = Range
activity-d-error = Error
activity-d-id = Entry id
activity-d-request-id = Request id
activity-d-operation = Operation
activity-d-from = From
activity-d-to = To
activity-d-file = File
activity-d-result = Result
activity-d-change = Part of change
activity-d-sha256 = SHA-256
activity-d-level = Level
activity-d-message = Message
activity-d-fields = Details
activity-status-after =
    { $count ->
        [one] { $status } — after 1 redirect
       *[other] { $status } — after { $count } redirects
    }
