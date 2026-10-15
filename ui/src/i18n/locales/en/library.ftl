# The Library (S4): header, states, list, mod details, conflicts (C1), deleting (D6).
library-title = Library
library-profile = Profile
library-enabled-count = { $enabled } of { $total } enabled
library-disk = { $size } on disk
library-not-checked = Updates not checked yet
library-checked-today = Updates checked today at { $time }
library-checked-on = Updates checked on { $date }
library-game-not-found = Game not found
library-add-disk = Add from disk
library-add-title = Add mods from disk
library-add-filter = Mod files
library-added =
    { $count ->
        [one] { $count } mod added to the library.
       *[other] { $count } mods added to the library.
    }
library-check = Check for updates
library-checking = Checking…
library-update-all = Update all
library-review = Review
library-undo = Undo
library-cancel = Cancel
library-missing-what = Deadlock wasn’t found.
library-missing-why = Steam or the game isn’t installed, or the game folder was moved. Your library still works; applying changes and Play stay off until the game is found.
library-choose-folder = Choose game folder
library-search-again = Search again
library-running-what = Deadlock is running.
library-running-why = Changes will be applied automatically when you close the game.
library-pending = Pending changes
library-switch-what = Profile { $name } isn’t in the game yet.
library-switch-why = The game still has { $applied }. Apply { $name } now, or press Play — deadloader applies it before launching.
library-switch-why-first = Apply { $name } now, or press Play — deadloader applies it before launching.
library-review-switch = Review switch
library-applying-what = Applying profile { $name }…
library-applying-why = You can keep browsing; Play is back when this finishes.
library-applying-step = Step { $done } of { $total }
library-applying-linked =
    { $count ->
        [one] { $count } file linked
       *[other] { $count } files linked
    }
library-applying-copied =
    { $count ->
        [one] { $count } file copied
       *[other] { $count } files copied
    }
library-applying-both =
    { $linked ->
        [one] { $linked } file linked
       *[other] { $linked } files linked
    }, { $copied } copied
library-applying-left-seconds = About { $seconds } s left
library-applying-left-minutes = About { $minutes } min { $seconds } s left
library-show = Show
library-tab-all = All
library-tab-enabled = Enabled
library-tab-disabled = Disabled
library-tab-updates = Updates
library-tab-conflicts = Conflicts
library-tab-local = Local
library-search = Search library
library-search-clear = Clear search
library-higher-wins = Higher in the list wins when two mods change the same file.
library-reorder-hint = Reorder: drag the handle, or focus it and press Space
library-list-label = Mods in profile { $name }, in priority order
library-toggle = { $name } enabled
library-toggle-missing = Deadlock wasn’t found — choose the game folder to apply changes
library-grip = Reorder { $name }, position { $position } of { $total }
library-more = More actions for { $name }
library-show-variants = Show variants
library-hide-variants = Hide variants
library-show-addons = Show add-ons
library-hide-addons = Hide add-ons
library-live-picked = { $name } — picked up, position { $position } of { $total }. ↑ ↓ move it, Space drops, Esc cancels.
library-live-moved = { $name } — position { $position } of { $total }.
library-live-moved-grabbed = { $name } — position { $position } of { $total }. Space drops, Esc cancels.
library-live-dropped = { $name } — dropped at position { $position } of { $total }.
library-live-back = { $name } — back at position { $position } of { $total }.
library-live-drag = { $name } — release to drop above { $above }, at position { $position } of { $total }.
library-badge-pending = Pending
library-badge-damaged = Files damaged
library-badge-update = Update available
library-badge-wins = Conflict · wins
library-badge-loses = Conflict · loses
library-badge-variants =
    { $count ->
        [one] { $count } variant
       *[other] { $count } variants
    }
library-badge-local = Local
library-badge-copied = Copied · +{ $size }
library-local = Local
library-unknown-author = Unknown author
library-gamebanana = GameBanana
library-from-disk = from disk
library-addons-sub =
    { $count ->
        [one] + { $count } add-on
       *[other] + { $count } add-ons
    }
library-variants = Variants
library-variants-downloaded = { $count } downloaded
library-active = Active
library-select-to-switch = Select to switch
library-parts = Files of this mod
library-main-files = Main files
library-from = from { $archive }
library-on-with-mod = On with the mod · { $size }
library-remove-addon = Remove add-on
library-toast-enabled = { $name } enabled.
library-toast-disabled = { $name } disabled.
library-toast-variant = { $name } switched to { $variant }.
library-empty-title = Your library is empty
library-empty-text = Install mods from the catalog, or add .vpk, .zip, .7z or .rar files from your computer. You can also drop files anywhere on this window.
library-empty-local-text = The catalog is off in local mode. Add .vpk, .zip, .7z or .rar files from your computer, or drop them anywhere on this window.
library-browse = Browse catalog
library-no-match-title = No mods here
library-no-match-text = No mod matches this tab and search. Try another tab or clear the search.

## Mod details
library-details = Mod details
library-open-mod-page = Open mod page
library-close-details = Close details
library-enabled-in = Enabled in { $profile }
library-position = Position { $position } of { $total }
library-version = Version
library-no-version = No version given
library-from-gamebanana = From GameBanana
library-imported = Imported from disk
library-newer-in-library = A newer version is in the library.
library-switch-to = Switch to
library-delete-old = Delete old versions
library-no-older = No older versions kept
library-older =
    { $count ->
        [one] { $count } older version kept · { $size }
       *[other] { $count } older versions kept · { $size }
    }
library-files = Files
library-on-disk = { $size } on disk
library-linked = Linked — no extra disk space
library-copied = Copied — { $size } of extra disk space
library-not-in-game = Not in the game folder
library-local-mod = Local mod
library-name = Name in your library
library-rename = Rename
library-link-label = GameBanana page
library-link-hint = Paste the mod’s address to check it for updates.
library-link = Link
library-link-done = Linked: { $matched } of { $total } files found on the page.
library-link-not-a-page = This isn’t a GameBanana mod page. Paste an address like gamebanana.com/mods/123456.
library-link-removed = This mod is no longer on GameBanana.
library-link-failed = The page couldn’t be read. Check the address and try again.
library-imported-from = Imported from
library-no-conflicts = No conflicts with other enabled mods
library-conflict-wins =
    { $count ->
        [one] Wins over { $name } · { $count } file
       *[other] Wins over { $name } · { $count } files
    }
library-conflict-loses =
    { $count ->
        [one] Loses to { $name } · { $count } file
       *[other] Loses to { $name } · { $count } files
    }
library-show-conflicts = Show conflicts
library-delete-mod = Delete mod…
library-open-folder = Open folder

## Conflicts (C1)
library-conflicts-sub = Two enabled mods change the same game files. The one higher in the list wins.
library-close-conflicts = Close conflicts
library-no-conflicts-title = No conflicts
library-wins = Wins
library-loses = Loses
library-off = Off
library-in-use = In use
library-conflict-text =
    { $count ->
        [one] Both change { $count } file. In the game you see { $winner }’s version; { $loser }’s copy is not used.
       *[other] Both change { $count } files. In the game you see { $winner }’s versions; { $loser }’s copies of these files are not used.
    }
library-conflict-off =
    { $count ->
        [one] Both change { $count } file. { $off } is off, so the game uses { $other }’s version.
       *[other] Both change { $count } files. { $off } is off, so the game uses { $other }’s versions.
    }
library-files-count =
    { $count ->
        [one] { $count } file
       *[other] { $count } files
    }
library-show-more = Show { $count } more
library-move-above = Move { $name } above
library-disable = Disable { $name }
library-pick-variant = Pick another variant
library-no-variants = Neither mod has variants
library-done-wins = Done — { $name } now wins
library-done-off = Done — { $off } is off; { $other }’s files are used
library-identical = Identical files (same checksum) aren’t conflicts and aren’t listed.

## D6 Delete mod
library-d6-title = Delete “{ $name }”?
library-d6-files-title = Delete old versions of “{ $name }”?
library-d6-what =
    { $count ->
        [one] { $count } file · { $size }.
       *[other] { $count } files · { $size }.
    }
library-d6-profiles =
    { $count ->
        [one] Used in { $count } profile: { $names } — it will be removed from it and from the game folder.
       *[other] Used in { $count } profiles: { $names } — it will be removed from them and from the game folder.
    }
library-d6-no-profiles = No profile uses it.
library-d6-where = Delete
library-d6-permanently = Permanently
library-d6-recycle-bin = To the recycle bin
library-d6-permanent-hint = The files are deleted for good — this can’t be undone.
library-d6-bin-hint = You can restore it from the Windows recycle bin.
library-d6-confirm = Delete mod
library-d6-confirm-files = Delete files
library-deleted = { $name } deleted.
library-deleted-files =
    { $count ->
        [one] { $count } file of { $name } deleted.
       *[other] { $count } files of { $name } deleted.
    }
