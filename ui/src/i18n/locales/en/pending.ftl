# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Close
library-apply = Apply
library-apply-profile = Apply { $name }
library-discard-all = Discard all
library-keep = Keep { $name }
library-keep-applied = Keep the applied changes
library-undo-this = Undo this change
library-undo-all = Undo all
library-close-to-apply = Close Deadlock — changes apply automatically then
library-play-applies = Play applies it too
library-applied-title = Changes applied
library-nothing-pending = Nothing pending
library-all-in-game = Everything in profile { $name } is already in the game.
library-running-sub = Deadlock is running, so Windows keeps its files locked. These changes are applied automatically as soon as you close the game.
library-waiting-sub = These changes aren’t in the game folder yet. Apply them now, or press Play.
library-switch-sub = A profile switch changes many files at once, so it waits for you. Apply it here, or press Play — deadloader applies it before launching.
library-applied-sub =
    { $count ->
        [one] Deadlock was closed at { $time }, so the change you made while it ran was applied. Undo stays available until your next change.
       *[other] Deadlock was closed at { $time }, so the { $count } changes you made while it ran were applied. Undo stays available until your next change.
    }
library-undoing-sub =
    { $count ->
        [one] You undid the change applied at { $time }, and Deadlock is running again. It is reverted automatically when you close the game.
       *[other] You undid the { $count } changes applied at { $time }, and Deadlock is running again. They are reverted automatically when you close the game.
    }
library-group-enable = Enable · { $count }
library-group-disable = Disable · { $count }
library-group-order = Order · { $count }
library-group-variant = Variant · { $count }
library-group-version = Version · { $count }
library-group-switch = Profile switch · { $count }
library-group-undo =
    { $count ->
        [one] Undo of the change applied at { $time }
       *[other] Undo of the { $count } changes applied at { $time }
    }
library-change-on = Off → on
library-change-off = On → off
library-change-above = Moved above { $name }
library-change-above-wins = Moved above { $name } — now wins their conflict
library-change-below = Moved below { $name }
library-switch-what-happens =
    { $on ->
        [one] 1 mod turns on
       *[other] { $on } mods turn on
    } · { $off ->
        [one] 1 turns off
       *[other] { $off } turn off
    }
library-what-happens = What happens in the game folder
library-folder-summary =
    { $added ->
        [one] 1 file added
       *[other] { $added } files added
    } · { $removed } removed · { $renumbered } renumbered

## Foreign files
library-foreign-title = Found in the game’s mod folder
library-foreign-note = deadloader didn’t put these here and won’t touch them unless you choose to.
library-collapse = Collapse
library-expand = Expand
library-take = Take under management
library-leave = Leave as is
library-left = Left as is
library-foreign-open = Open the folder containing { $name }
library-foreign-vpk =
    { $count ->
        [one] VPK, { $count } file
       *[other] VPK, { $count } files
    }
library-foreign-vpk-hero =
    { $count ->
        [one] VPK, { $count } file — looks like a skin for { $hero }
       *[other] VPK, { $count } files — looks like a skin for { $hero }
    }
library-foreign-unreadable = A mod file deadloader can’t read
library-foreign-replaced = A file that took the place of one deadloader put there
library-foreign-dmm = Created by Deadlock Mod Manager — a settings file, not a mod
library-foreign-other = A file, not a mod
library-foreign-folder =
    { $count ->
        [one] Folder, { $count } file — not a mod
       *[other] Folder, { $count } files — not a mod
    }
library-foreign-meta = { $size } · added { $date }
library-taken = { $name } is now in your library.

## D11
library-d11-title = Take { $name } under management?
library-d11-lead = deadloader will:
library-d11-step-copy = copy it into your library as a local mod ({ $size }),
library-d11-step-replace = replace the file in the game folder with its own linked copy,
library-d11-step-enable = enable it in profile { $profile } at the bottom of the list.
library-d11-name = Name in your library
library-d11-hint = The library keeps an exact copy of the original file.
library-d11-suggested = { $file } ({ $hero } skin)

## D10
library-d10-title = A game file was changed by another program
library-d10-what = Another program changed a game file that controls which folders Deadlock loads — often Deadlock Mod Manager. deadloader won’t overwrite someone else’s changes, so your mods won’t load until this is fixed.
library-d10-do = Let Steam restore the original file (Verify game files). deadloader then adds its own line and tells you.
library-d10-diff = Show differences
library-d10-expected = Expected (original + deadloader’s line)
library-d10-found = Found now
library-d10-diff-note = Highlighted: the line deadloader adds, and the lines another program added.
library-d10-help = What is this file?
library-d10-help-text = gameinfo.gi tells Deadlock which folders to load. Mod managers add one line so the game loads mods from the addons folder. Game updates reset it; deadloader fixes that automatically — but only when the file is the original.
library-d10-wait-title = Steam is verifying Deadlock’s files.
library-d10-wait-text = When Steam is done, come back to deadloader — it checks the file again automatically. You can also check now.
library-d10-fixed = gameinfo.gi is set up — mods load again.
library-d10-fixed-text = Steam restored the original file and deadloader added its line. Details in Activity → Files.
library-open-file = Open file
library-not-now = Not now
library-verify-steam = Verify game files in Steam
library-check-again = Check again
library-show-differences = Show differences

## D5
library-update = Update
library-update-on-gb = A newer version is on GameBanana.
library-later = Later
library-d5-title = Update { $name }?
library-d5-installed = Installed
library-d5-new = New on GameBanana
library-d5-downloaded = Downloaded
library-d5-arrived = In the library, not active yet
library-d5-new-meta = Updated { $date } · { $size } to download
library-d5-download = { $size } to download
library-d5-changed = The new version has a different set of files. deadloader matched them like this:
library-d5-same = These files are replaced by their new versions:
library-d5-map-text = deadloader couldn’t tell for every file which new one replaces it. Choose the new file for each, or keep the old one.
library-d5-map-label = File mapping
library-d5-col-old = Installed file
library-d5-col-new = New file
library-d5-pick = New file for { $file }
library-d5-choose = Choose…
library-d5-keep = Keep the old file
library-d5-tag-replaces = Replaces
library-d5-tag-same = Same name
library-d5-tag-new = New · on the mod page
library-d5-profiles = { $new } becomes active in every profile that uses { $old }. { $old } stays in the library, so you can switch back in the mod details.
library-d5-install = Install update
library-d5-confirm-map = Use these files
library-update-downloading = Downloading the update of { $name } — it is installed when it arrives.
library-updated = { $name } updated.
library-updated-text = Active in every profile that used the old version. The old version is kept — you can switch back in the mod details.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name } on
library-n12-off = { $name } off
library-n12-above = { $name } moved above { $other }
library-n12-below = { $name } moved below { $other }
library-n12-switched = { $name } switched to { $to }
library-n12-files = { $name }: files changed

## S9: the file list of a pending apply
library-show-file-list = Show the file list
library-hide-file-list = Hide the file list
library-file-list = Files in the game folder
library-files-added = Added · { $count }
library-files-removed = Removed · { $count }
library-files-renumbered = Renumbered · { $count }
