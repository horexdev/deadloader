# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Settings
settings-nav = Settings sections
settings-reset-nav = Reset all settings…
settings-default = Default: { $value }
settings-restore = Restore default
settings-on = On
settings-off = Off
settings-cancel = Cancel
settings-remove = Remove
settings-remove-item = Remove { $item }
settings-add = Add
settings-open-folder = Open folder
settings-open-folder-at = Open folder: { $path }
settings-copied = Copied to the clipboard.
settings-folder = Folder
settings-example = Example:

settings-section-general = General
settings-section-general-lead = Language, start and zoom.
settings-section-game = Game
settings-section-game-lead = Where Deadlock is installed and what deadloader may change there.
settings-section-catalog = Catalog
settings-section-catalog-lead = The GameBanana catalog, sensitive content, images and links.
settings-section-library = Library
settings-section-library-lead = Mod updates, deleting mods and old versions.
settings-section-downloads = Downloads
settings-section-downloads-lead = Download queue, archives and unpacking.
settings-section-network = Network
settings-section-network-lead = How deadloader talks to GameBanana.
settings-section-cache = Cache
settings-section-cache-lead = Thumbnails and catalog pages saved on this PC.
settings-section-logs = Logs
settings-section-logs-lead = What Activity keeps, and for how long.
settings-section-advanced = Advanced
settings-section-advanced-lead = Settings files and extra allowed domains.
settings-section-about = About deadloader
settings-section-about-lead = Version, licence, how to check this download, and where your data is.

# General
settings-general-group = Language and start
settings-language = Language
settings-language-help = System language: the Windows display language if deadloader has it, otherwise English.
settings-language-system = System language ({ $language })
settings-startup-profile = Profile at start
settings-startup-profile-help = Which profile is selected when deadloader starts. Selecting doesn’t apply it.
settings-startup-last = Last selected ({ $name })
settings-startup-last-plain = Last selected
settings-app-updates = Check for deadloader updates at start
settings-app-updates-help = deadloader asks GitHub (api.github.com) once at start whether a newer version exists. Updating is always manual. You chose this at first run.
settings-app-updates-local = Unavailable: local mode makes no network requests at all. It becomes available when you turn the catalog on (Settings → Catalog).
settings-zoom = Zoom
settings-zoom-keys-1 = Also with Ctrl + mouse wheel,
settings-zoom-keys-2 = and
settings-zoom-keys-3 = to reset.
settings-ctrl = Ctrl
settings-percent = { $value } %

# Game
settings-installs-group = Game installations
settings-installs-lead = deadloader finds Steam installations of Deadlock by itself. Each profile uses one of them — change that in Profiles.
settings-install-library = Steam library { $library }
settings-install-build = build { $build }
settings-install-used =
    { $count ->
        [0] not used by any profile
        [one] used by 1 profile: { $names }
       *[other] used by { $count } profiles: { $names }
    }
settings-install-selected = { $name } (selected)
settings-install-applied = { $name } (applied there)
settings-install-steam = Found by Steam detection
settings-install-manual = Added manually
settings-install-known = deadloader’s files are still there
settings-install-manual-meta = Added manually
settings-gi-ours = gameinfo.gi: OK — deadloader’s lines are there
settings-gi-wait = gameinfo.gi: set up when a profile uses it
settings-gi-rolled = gameinfo.gi: original — Apply adds deadloader’s lines again
settings-gi-foreign = gameinfo.gi: changed by another program
settings-gi-missing = gameinfo.gi: not found — the folder may have moved
settings-installs-none = No installation found yet.
settings-add-manual = Add manually…
settings-add-manual-hint = Choose a Deadlock folder that wasn’t found.
settings-add-manual-title = Choose the Deadlock folder
settings-install-added = Installation added.
settings-install-removed = Installation removed from the list.
settings-install-removed-text = Nothing in that folder was changed.
settings-gameinfo-group = After game updates
settings-autofix = Fix gameinfo.gi automatically
settings-autofix-help = A Deadlock update resets the game file that tells it to load mods. deadloader backs it up, puts its lines back and tells you. When off, a banner asks you first.
settings-danger-group = Undo and remove
settings-rollback = Roll back the game folder of the selected profile
settings-rollback-help = For the installation of the selected profile { $profile } ({ $drive }): removes the { $files } mod files deadloader put there and its lines in gameinfo.gi. Library and profiles stay; Apply brings it all back.
settings-rollback-help-others = Other game folders aren’t touched; select a profile that uses them to roll them back.
settings-rollback-nothing = Nothing to roll back on { $drive } — deadloader has no files there now.
settings-rollback-no-game = Deadlock wasn’t found, so there is no game folder to roll back.
settings-rollback-button = Roll back…
settings-rolled-title = Game folder on { $drive } rolled back.
settings-rolled-text =
    { $foreign ->
        [0] On { $drive }, { $files } mod files and deadloader’s lines in gameinfo.gi were removed. Library and profiles are kept — Apply or Play brings the mods back.
       *[other] On { $drive }, { $files } mod files, deadloader’s lines in gameinfo.gi and { $foreign } foreign files were removed. Library and profiles are kept — Apply or Play brings the mods back.
    }
settings-wipe = Delete all deadloader data
settings-wipe-help = Deletes the library, profiles, logs, cache and settings ({ $size }), then closes deadloader. deadloader.exe stays — delete it yourself afterwards.
settings-wipe-button = Delete all data…

# Catalog
settings-catalog-group = GameBanana catalog
settings-catalog = Use the catalog
settings-catalog-help-on = Browse and install mods from GameBanana. deadloader talks to GameBanana only, and every request is listed in Activity. Off means local mode.
settings-catalog-help-off = Off — local mode: deadloader makes no network requests at all. You can still add mods from disk.
settings-consent-title = One step first.
settings-consent-text = deadloader shows which hosts it will contact and GameBanana’s terms before the catalog turns on.
settings-consent-go = Review and turn on
settings-local-on = Local mode is on.
settings-local-on-text = The catalog is off and deadloader makes no network requests.
settings-ttl = Reuse loaded catalog pages for
settings-ttl-help = Within this time, going back to a page doesn’t ask GameBanana again.
settings-minutes = { $count } min
settings-hours = { $count } h
settings-prefetch = Load the next page in advance
settings-prefetch-help = When you have scrolled through about 70 % of the results, the next ones are loaded so the grid keeps going.
settings-nsfw-group = Sensitive content
settings-nsfw = Mods with content ratings (NSFW)
settings-nsfw-help = Hidden: not shown at all. Blurred: a blurred picture with a “Sensitive content” label; click to reveal. Shown: no blur.
settings-nsfw-age-done = Age confirmed on { $date }.
settings-nsfw-age-ask = Leaving Hidden asks you to confirm your age once.
settings-nsfw-hidden = Hidden
settings-nsfw-blurred = Blurred
settings-nsfw-shown = Shown
settings-images-group = Images and links
settings-description-images = Images in mod descriptions
settings-description-images-help = They come from hosts other than GameBanana, e.g. i.imgur.com. On click: a placeholder until you click it.
settings-images-click = On click
settings-images-always = Always
settings-prechecked = Pre-tick “Always allow images from this host”
settings-prechecked-help = In the dialog shown the first time a description image comes from a new host.
settings-hosts = Allowed image hosts
settings-hosts-help = Images from these hosts load without asking. In Activity their requests are marked “allowed image host”.
settings-hosts-built-in = Built-in
settings-hosts-none = No hosts added — every new host asks first.
settings-host-removed = Removed { $host }
settings-host-removed-text = Images from it ask again before loading.
settings-undo = Undo
settings-gif = Play GIFs automatically
settings-gif-help = Off: GIFs show their first frame with a play button.
settings-links = External links
settings-links-help = Links open in your default browser. Ask first shows the full address before it opens.
settings-links-ask = Ask first
settings-links-open = Open immediately

# Library
settings-updates-group = Mod updates
settings-check-updates = Check for mod updates at start
settings-check-updates-help = Compares your mods from GameBanana with the catalog. Never in local mode.
settings-auto-download = Download updates automatically
settings-auto-download-help = Downloads only — deadloader still asks before a mod switches to the new version.
settings-disk-group = Deleting and old versions
settings-deletion = When you delete a mod
settings-deletion-help = You can choose differently in the delete dialog each time.
settings-deletion-permanent = Permanently
settings-deletion-bin = To the recycle bin
settings-old-versions = Old versions
settings-old-versions-help = Every downloaded version is kept so you can go back. Delete old versions of a mod in its details.
settings-old-keep-all = Keep all

# Downloads
settings-queue-group = Queue
settings-parallel = Parallel downloads
settings-parallel-help = How many files download at the same time.
settings-speed = Speed limit
settings-speed-help = For mod downloads only — the catalog and thumbnails aren’t limited.
settings-speed-none = No limit
settings-speed-value = { $value } MB/s
settings-keep-archives = Keep archives after install
settings-keep-archives-help = Off: the .zip, .7z or .rar file is deleted once its .vpk files are in the library.
settings-unpack-group = Unpacking
settings-limits = Limits for unpacking
settings-limits-help = Protects against archives that unpack to something huge. If an archive you trust is refused, raise the limit.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = Unpacked size
settings-limit-ratio = Compression ratio
settings-limit-files = Mod files per archive
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = files
settings-unit-s = s
settings-number-error = { $field }: enter a whole number from { $min } to { $max }. The previous value stays in use until then.

# Network
settings-network-local = Local mode is on.
settings-network-local-text = deadloader makes no network requests; these settings apply once the catalog is on.
settings-requests-group = Requests to GameBanana
settings-api-parallel = Parallel catalog requests
settings-api-parallel-help = Requests to GameBanana’s API at the same time. Images and downloads are counted separately.
settings-timeouts = Timeouts
settings-timeouts-help = Connect: give up when no connection is made in this time. Response: give up when the server stops sending.
settings-timeouts-default = 10 s · 30 s
settings-timeout-connect = Connect
settings-timeout-read = Response
settings-retries = Retries
settings-retries-help = How often a failed request is repeated, with growing pauses, before deadloader shows an error.
settings-connection-group = Connection
settings-proxy = Proxy
settings-proxy-help = None: direct connection. System: the proxy set in Windows. Manual: enter it below.
settings-proxy-none = None
settings-proxy-system = System
settings-proxy-manual = Manual
settings-proxy-host = Address
settings-proxy-port = Port
settings-proxy-hint = HTTP proxy. Only deadloader’s own requests go through it.
settings-proxy-invalid = Enter a host name or address without a user name or password, e.g. proxy.example.com.

# Cache
settings-thumbs-group = Thumbnails and catalog pages
settings-cache-limit = Cache limit
settings-cache-limit-help = When the cache is full, the thumbnails you haven’t seen for the longest time go first.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = Current size
settings-cache-breakdown = { $thumbnails } thumbnails · { $pages } catalog pages
settings-cache-empty = Empty — nothing cached yet.
settings-cache-used = { $used } of { $limit }
settings-cache-clear = Clear cache
settings-cache-cleared = Cache cleared — { $size } freed. Thumbnails load again when you need them.
settings-cache-cleared-partly = { $failed } files were in use and stay until the next clear.

# Logs
settings-keep-group = Keeping logs
settings-log-days = Keep logs for
settings-log-days-help = Older entries are deleted when deadloader starts.
settings-days = { $count ->
        [one] { $count } day
       *[other] { $count } days
    }
settings-year = 1 year
settings-log-size = Maximum size
settings-log-size-help = Above this size the oldest log files are deleted first.
settings-privacy-group = Privacy
settings-mask = Hide your Windows user name
settings-mask-help = Paths in logs show { $masked } instead of your name — safer when you share a log.
settings-logs-folder-group = Logs folder
settings-logs-folder = Where the logs are
settings-logs-now = { $size } · { $files } files · oldest from { $date }
settings-open-logs = Open logs folder

# Advanced
settings-advanced-warning = For experienced users.
settings-advanced-warning-text = A mistake here can change how deadloader behaves. It checks the file before saving and keeps a backup of the previous version.
settings-files-group = Edit the settings files
settings-file-settings = Settings
settings-file-manifest = Game folder manifest
settings-file-label = File to edit
settings-file-settings-lead = Everything on these pages, as one file. Changes here appear on the pages after saving.
settings-file-manifest-lead = The list of what deadloader put into the game folder. Rollback relies on it — a wrong entry can leave files behind.
settings-json-error = This isn’t valid JSON, so it wasn’t saved.
settings-json-where = Line { $line }, column { $column }: something is missing or extra here or just before it — often a comma or a quotation mark.
settings-json-fix = Fix that spot and save again — until then deadloader keeps using the current file.
settings-json-refused = These values can’t be used, so nothing was saved.
settings-json-newer = This file was made by a newer deadloader (format { $found }; this version reads up to { $supported }), so it wasn’t saved.
settings-json-invalid = This isn’t a file deadloader can use, so it wasn’t saved.
settings-json-details = Details
settings-problem-unknown-field = { $path }: not a setting of this version.
settings-problem-invalid-value = { $path }: this value can’t be used here.
settings-problem-out-of-range = { $path }: { $found } is outside { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: “{ $entry }” isn’t valid here.
settings-problem-duplicate-entry = { $path }: “{ $entry }” is listed twice.
settings-problem-needs-age = { $path }: showing NSFW content needs the age confirmation first (Settings → Catalog).
settings-problem-local-mode = { $path }: unavailable in local mode.
settings-json-idle = Checked when you save.
settings-json-saved = Valid and saved. Previous version kept as { $backup }.
settings-json-saved-plain = Valid and saved.
settings-json-save = Check and save
settings-json-discard = Discard my edits
settings-json-backup = Before saving, the previous file is kept as a backup; the last 10 stay.
settings-domains-group = Additional allowed domains
settings-domains-lead = deadloader only talks to GameBanana. If a download is redirected to another host and blocked, you can allow that host here. In Activity, requests to it are marked “user-added”.
settings-domains-built-in = Built-in:
settings-domain-user = user-added
settings-domains-none = No additional domains.
settings-domain-add = Add a domain
settings-domain-hint = Host name only, e.g. files.example.com.
settings-domain-empty = Enter a host name first, e.g. files.example.com.
settings-domain-not-host = Enter the host name only — without https:// or a path, e.g. files.example.com.
settings-domain-invalid = “{ $host }” isn’t a host name. Check the spelling, e.g. files.example.com.
settings-domain-built-in = { $host } is built in and always allowed.
settings-domain-listed = { $host } is already in the list.

# About
settings-version = version { $version }
settings-commit = Build from commit
settings-commit-local = Local build — not built by GitHub Actions
settings-copy-commit = Copy commit id
settings-release-page = Release page
settings-licence-group = Licence
settings-licence-help = deadloader is free software: you may use, study, share and change it under the GPL-3. It comes with no warranty.
settings-read-licence = Read the licence
settings-source = Source code
settings-verify-group = How to verify this download
settings-verify-lead = Two independent checks that this deadloader.exe is the one published on GitHub.
settings-verify-hash = Compare the SHA-256 checksum
settings-verify-hash-help = It must match the checksum on the release page. A modified copy could show a false value here, so also compute it yourself:
settings-verify-hash-cmd = In PowerShell, in the folder of the exe:
settings-verify-attest = Check the build attestation
settings-verify-attest-help = Proves that GitHub Actions built this file from the published source. Needs the GitHub CLI:
settings-copy-hash = Copy checksum
settings-copy-command = Copy command
settings-unsigned = deadloader.exe isn’t code-signed.
settings-unsigned-text = Windows SmartScreen may say “Windows protected your PC” on first start. The two checks above are how you confirm the file is genuine.
settings-webview-group = Window engine
settings-webview-help = deadloader’s window is drawn by WebView2, part of Windows. Windows updates it on its own. deadloader switches off:
settings-webview-off-1 = DNS-over-HTTPS lookups
settings-webview-off-2 = Microsoft account sign-in inside the window
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Background networking and component updates
settings-webview-off-5 = Form-autofill server calls
settings-webview-off-6 = Error reports to Microsoft (domain reliability)
settings-webview-no-route = And the window has no route to the internet at all.
settings-webview-unknown = version unknown
settings-data-group = Your data
settings-data-folder = Data folder
settings-data-folder-help = Everything deadloader keeps is in this one folder next to the exe.
settings-notice = First-run notice
settings-notice-help = Version { $version }, accepted on { $date } in { $language }. Your choices: catalog { $catalog }, update check { $updates }.
settings-notice-none = Not acknowledged yet.
settings-notice-again = Read it again

# D7
settings-d7-title = Roll back the game folder on { $drive }?
settings-d7-from =
    { $others ->
        [0] The installation of the selected profile { $profile }. deadloader removes from
       *[other] The installation of the selected profile { $profile } (also used by { $others }). deadloader removes from
    }
settings-d7-files =
    { $count ->
        [one] 1 mod file it added in
       *[other] { $count } mod files it added in
    }
settings-d7-line = Its lines in gameinfo.gi
settings-d7-line-why = — the file goes back to how Steam installed it.
settings-d7-stays = Stays as it is:
settings-d7-library = Your library — { $mods } mods, { $size }
settings-d7-profiles =
    { $count ->
        [one] Your profile — { $names }
       *[other] Your { $count } profiles — { $names }
    }
settings-d7-other = The installation on { $path } ({ $profile }) — not touched. To roll it back, select a profile that uses it and roll back again.
settings-d7-other-plain = The installation on { $path } — not touched.
settings-d7-back = Apply or Play brings everything back.
settings-d7-foreign =
    { $count ->
        [one] Also remove 1 file that deadloader didn’t create
       *[other] Also remove { $count } files that deadloader didn’t create
    }
settings-d7-foreign-why = from another program or a manual install. They are deleted permanently; deadloader can’t bring them back.
settings-d7-confirm = Roll back
settings-d7-confirm-foreign =
    { $count ->
        [one] Roll back and delete 1 file
       *[other] Roll back and delete { $count } files
    }

# D8
settings-d8-title = Delete all deadloader data?
settings-d8-from = deadloader deletes everything in
settings-d8-from-2 = and then closes.
settings-d8-what = What
settings-d8-contains = Contains
settings-d8-size = Size
settings-d8-total = Total
settings-d8-library = Library
settings-d8-library-has = { $mods } mods, { $files } files
settings-d8-profiles = Profiles
settings-d8-logs = Logs
settings-d8-logs-has = { $files } files, since { $date }
settings-d8-logs-none = no log files
settings-d8-cache = Cache
settings-d8-cache-has = thumbnails and catalog pages
settings-d8-settings = Settings
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
        [one] incl. 1 backup
       *[other] incl. { $backups } backups
    }
settings-d8-other = Other
settings-d8-other-has = partial downloads, gameinfo.gi backups, the window engine’s data
settings-d8-exe = Not deleted: deadloader.exe itself — delete it yourself if you no longer need it.
settings-d8-rollback = First roll back every game folder deadloader uses (recommended)
settings-d8-rollback-why = Removes deadloader’s mod files and its lines in gameinfo.gi from every game folder before deleting. Foreign files stay.
settings-d8-rollback-off = Not ticked: deadloader’s mod files stay in the game folders and Deadlock keeps loading them — with no deadloader left to remove them.
settings-d8-type-1 = To confirm, type
settings-d8-type-2 = { "" }
settings-d8-word = delete
settings-d8-hint = The delete button unlocks when the word matches.
settings-d8-hint-ok = Confirmed. This can’t be undone.
settings-d8-type-first = Type “{ $word }” first
settings-d8-deleting = Deleting…
settings-d8-confirm = Delete and close
settings-d8-confirm-rollback = Roll back, delete and close

# Reset
settings-reset-title = Reset all settings?
settings-reset-body = Every setting goes back to its default, including allowed image hosts and additional domains.
settings-reset-keeps = Your library, profiles and logs aren’t touched, and the game folder stays as it is.
settings-reset-backup = A backup of the current settings is kept as settings.json.bak-…
settings-reset-confirm = Reset settings
settings-reset-done = All settings reset to their defaults.
settings-reset-done-text = Backup: { $backup }
