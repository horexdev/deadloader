# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Open this link in your browser?
catalog-link-body = It leaves deadloader and opens in your default browser.
catalog-link-always = Always open links immediately
catalog-link-cancel = Cancel
catalog-link-open = Open

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Hidden
catalog-nsfw-blurred = Blurred
catalog-nsfw-shown = Shown
catalog-age-title = Show mods with sensitive content?
catalog-age-body = Some mods on GameBanana are rated for nudity, sexual themes or similar. To see them, confirm that you are 18 or older, or of legal age where you live.
catalog-age-choice = You chose: { $mode }.
catalog-age-once = deadloader asks this once. You can go back to Hidden at any time.
catalog-age-confirm = I’m of legal age
catalog-age-cancel = Cancel

## S2: the catalog grid
catalog-title = Catalog
catalog-section = Section
catalog-section-mods = Mods
catalog-section-sounds = Sounds
catalog-search = Search mods
catalog-search-sounds = Search sounds
catalog-search-offline = Search needs a connection
catalog-search-off = Search is unavailable — catalog can’t be read
catalog-search-clear = Clear search
catalog-results =
    { $count ->
        [one] { $count } result
       *[other] { $count } results
    }
catalog-sort = Sort
catalog-sort-newest = Newest
catalog-sort-latest_updated = Recently updated
catalog-sort-most_downloaded = Most downloaded
catalog-sort-most_liked = Most liked
catalog-sort-most_viewed = Most viewed
catalog-sort-most_commented = Most commented
catalog-sort-oldest = Oldest
catalog-sort-best = Best match
catalog-sort-best-title = Sorting is by best match while you search
catalog-category = Category
catalog-category-all = All categories
catalog-hero = Hero
catalog-hero-all = All heroes
catalog-hero-filter = Find a hero
catalog-hero-none = No hero matches
catalog-filters = Filters
catalog-filter-nsfw = Mods with content ratings (NSFW)
catalog-filter-favourites = Favourites only
catalog-filter-favourites-short = favourites only
catalog-filter-installed = Hide installed
catalog-filter-installed-short = hide installed
catalog-nsfw-caption-hidden = NSFW hidden
catalog-nsfw-caption-blurred = NSFW blurred · click a card to show it
catalog-nsfw-caption-shown = NSFW shown
catalog-loading = Loading mods from GameBanana…
catalog-loading-sounds = Loading sounds from GameBanana…
catalog-loading-more = Loading more…
catalog-more-failed = The next page couldn’t be loaded.
catalog-grid = Mods
catalog-grid-sounds = Sounds
catalog-likes =
    { $count ->
        [one] { $count } like
       *[other] { $count } likes
    }
catalog-favourite-add = Add to favourites
catalog-favourite-remove = Remove from favourites
catalog-sensitive = Sensitive content
catalog-click-show = Click to show
catalog-badge-installed = Installed
catalog-badge-update = Update
catalog-card-installing = Installing · { $percent } %
catalog-empty = No mods here yet
catalog-empty-search = No mods match “{ $text }”
catalog-empty-favourites = No favourites yet
catalog-empty-filters = Filters on: { $filters }. Clear them to search all mods, or try a shorter word.
catalog-empty-shorter = Try a shorter or different word.
catalog-clear-filters = Clear filters
catalog-rate-retry = deadloader retries automatically in { $seconds } s — the mods below stay usable.
catalog-retry-now = Retry now
catalog-try-again = Try again
catalog-check-update = Check for deadloader update
catalog-details = Details · log entry
catalog-copy-id = Copy log entry id
catalog-open-activity = Open in Activity
catalog-offline-empty-title = Offline — nothing saved to show
catalog-offline-empty-text = deadloader can’t reach GameBanana, and this part of the catalog wasn’t saved earlier.
catalog-error-title = The catalog can’t be shown right now.
catalog-local-title = The catalog is off (local mode)
catalog-local-why = In local mode deadloader makes no network requests at all. Your library, profiles and Play work as usual, and you can add mods from disk.
catalog-local-do = Turn the catalog on to browse and install mods from GameBanana. deadloader will then talk to GameBanana only.
catalog-local-enable = Enable catalog
catalog-local-library = Go to library

## S3: the mod page
catalog-breadcrumb = Breadcrumb
catalog-back = Back to catalog
catalog-page-loading = Loading mod page from GameBanana…
catalog-page-offline = Offline — showing the copy saved on { $date }.
catalog-page-offline-why = Installing new files needs a connection; switching between downloaded variants still works.
catalog-page-offline-empty-title = Offline — this mod page wasn’t saved
catalog-page-offline-empty-text = deadloader can’t reach GameBanana, and this page wasn’t opened before.
catalog-page-local-title = This mod page wasn’t saved
catalog-page-local-text = In local mode deadloader shows only mod pages saved earlier.
catalog-removed-title = This mod was removed from GameBanana
catalog-removed-why = It was withheld or deleted, so deadloader can’t show its page or download its files.
catalog-removed-keep = Your installed copy ({ $copy }) stays in your library and keeps working — it just won’t get updates.
catalog-show-in-library = Show in library
catalog-installed = Installed
catalog-by = by { $author }
catalog-version = Version { $version }
catalog-views =
    { $count ->
        [one] { $count } view
       *[other] { $count } views
    }
catalog-downloads =
    { $count ->
        [one] { $count } download
       *[other] { $count } downloads
    }
catalog-added = Added { $when }
catalog-updated = Updated { $when }
catalog-favourite = Favourite
catalog-open-gamebanana = Open on GameBanana
catalog-gallery-alt = { $name }, gallery image { $index }
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Previous image
catalog-gallery-next = Next image
catalog-gallery-original = View original
catalog-gallery-hidden = Pictures of mods with content ratings are hidden.
catalog-audio-preview = This sound has a preview on GameBanana.
catalog-description = Description
catalog-description-more = Show the whole description
catalog-description-less = Show less
catalog-description-truncated = Only the beginning of this description is shown.
catalog-opens-browser = opens in your browser
catalog-opens-here = Opens in deadloader
catalog-image-from = Image from
catalog-image-click = click to load
catalog-image-failed = couldn’t be loaded
catalog-image-alt = Image from the description
catalog-youtube = YouTube video
catalog-spoiler = Spoiler — click to show
catalog-spoiler-hide = Hide spoiler
catalog-files = Files
catalog-files-clean =
    { $count ->
        [one] It passed GameBanana’s antivirus check
       *[other] All { $count } passed GameBanana’s antivirus check
    }
catalog-files-clean-dated =
    { $count ->
        [one] It passed GameBanana’s antivirus check · uploaded { $date }
       *[other] All { $count } passed GameBanana’s antivirus check · uploaded { $date }
    }
catalog-files-av = GameBanana’s antivirus result is shown on each file.
catalog-variants-note = These files change the same game files, so they’re variants — one is active at a time.
catalog-no-files = This mod has no files to download.
catalog-older-files = Older files ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } download
       *[other] { $count } downloads
    }
catalog-file-active = active variant
catalog-file-downloaded = downloaded
catalog-file-install = Install
catalog-file-install-label = Install { $file } and make it the active variant
catalog-file-install-title = Downloads this file and makes it the active variant
catalog-file-install-offline = Installing needs a connection
catalog-file-installing = Installing…
catalog-file-installed = Installed
catalog-file-switch = Switch to
catalog-file-switch-label = Switch to { $file }
catalog-file-switch-title = Already downloaded — make it the active variant
catalog-file-update = Update
catalog-file-update-title = A newer version replaces your installed file — review it in the library
catalog-av-clean = Clean
catalog-av-clean-title = GameBanana antivirus: clean
catalog-av-flagged = Flagged
catalog-av-flagged-title = GameBanana antivirus: { $verdict }
catalog-av-pending = Not scanned yet
catalog-av-pending-title = GameBanana hasn’t finished scanning this file
catalog-av-none = No scan
catalog-av-none-title = GameBanana sent no antivirus result for this file
catalog-credits = Credits
catalog-no-ratings = No content ratings
catalog-no-licence = No licence text
catalog-no-sources = No alternate sources
catalog-rating-sexual_themes = Sexual themes
catalog-rating-sexual_content = Sexual content
catalog-rating-skimpy_attire = Skimpy attire
catalog-rating-lewd_angles_and_poses = Lewd angles and poses
catalog-rating-full_nudity = Full nudity
catalog-licence = Licence
catalog-sources = Alternate sources
catalog-videos = Videos

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Load an image from a new host?
catalog-image-body-before = deadloader will download this image from
catalog-image-body-after = . That host is not GameBanana — the request appears in Activity like every other.
catalog-image-note = Images are size-limited. You can remove allowed hosts in Settings → Catalog.
catalog-image-always = Always allow images from this host
catalog-image-cancel = Cancel
catalog-image-load = Load image
