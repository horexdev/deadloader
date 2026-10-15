# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Profiles
profiles-count =
    { $count ->
        [one] { $count } profile
       *[other] { $count } profiles
    }
profiles-in-game = In the game now: { $list }
profiles-in-game-item = “{ $name }” on { $drive }
profiles-in-game-none = No profile is in the game right now.
profiles-import = Import…
profiles-import-title = Import a profile from a .json file
profiles-import-dialog = Import a profile
profiles-export-dialog = Export the profile
profiles-file-filter = deadloader profiles
profiles-new = New profile
profiles-selected = Selected
profiles-applied = Applied
profiles-not-applied = Not applied yet
profiles-selected-title = The profile you edit in the library and play
profiles-pending-title = Apply or Play puts it into the game folder
profiles-applied-title = Its files are in the game folder on { $drive } now
profiles-select = Select
profiles-select-of = Select “{ $name }”
profiles-more-of = More actions for “{ $name }”
profiles-rename = Rename
profiles-duplicate = Duplicate
profiles-export = Export as file…
profiles-change-installation = Change game installation…
profiles-delete = Delete
profiles-delete-confirm = Delete…
profiles-delete-last = deadloader needs at least one profile.
profiles-name-label = Profile name
profiles-save = Save
profiles-cancel = Cancel
profiles-close = Close
profiles-close-esc = Close (Esc)
profiles-rename-hint = Enter saves · Esc cancels
profiles-name-empty = Enter a name for the profile.
profiles-mods =
    { $count ->
        [one] { $count } mod enabled
       *[other] { $count } mods enabled
    }
profiles-mods-none = No mods enabled
profiles-applied-when = Applied { $when }
profiles-last-applied = Last applied { $when }
profiles-never-applied = Never applied
profiles-just-now = just now
profiles-preview-empty = Enable mods for this profile in the library after selecting it.
profiles-footnote = A profile is a list of enabled mods with their order, variants and versions. Selecting one doesn’t download or delete anything; Apply changes only the files that differ.
profiles-copy-name = { $name } (copy)
profiles-copy-name-n = { $name } (copy { $number })
profiles-new-title = New profile
profiles-start-with = Start with
profiles-empty = Empty
profiles-empty-hint = No mods enabled — pick them in the library afterwards.
profiles-copy-of = Copy of “{ $name }”
profiles-copy-hint = { $mods } — same order, variants and versions.
profiles-installation = Game installation
profiles-installation-hint = Same as “{ $name }”. You can change it later from the profile’s menu.
profiles-new-note = Creating a profile doesn’t select or apply it — the game doesn’t change.
profiles-create = Create profile
profiles-pending-what = Profile “{ $name }” isn’t in the game yet.
profiles-pending-why-other = The game folder on { $drive } still has “{ $other }”. Apply it now, or press Play — deadloader applies it before launching.
profiles-pending-why-empty = The game folder on { $drive } has no mods from deadloader right now. Apply it now, or press Play — deadloader applies it before launching.
profiles-keep = Keep “{ $name }”
profiles-apply = Apply “{ $name }”
profiles-other-what = “{ $name }” uses another game installation.
profiles-other-why = Apply and Play use { $path }. The game on { $drive } keeps “{ $other }”.
profiles-other-why-empty = Apply and Play use { $path }. The game on { $drive } isn’t changed.
profiles-delete-title = Delete profile “{ $name }”?
profiles-delete-applied =
    { $count ->
        [one] “{ $name }” is in the game now. deadloader will remove its { $count } mod from the game folder:
       *[other] “{ $name }” is in the game now. deadloader will remove its { $count } mods from the game folder:
    }
profiles-delete-applied-empty = “{ $name }” is in the game now but has no mods, so the game folder doesn’t change.
profiles-delete-stay = The mods stay in your library and in other profiles. Until you apply another profile there, Deadlock starts without mods from deadloader.
profiles-delete-next = “{ $name }” becomes the selected profile. It isn’t applied until you press Apply or Play.
profiles-delete-undoable = This can’t be undone. To keep a copy, export the profile first.
profiles-export-first = Export first
profiles-delete-profile = Delete profile
profiles-installation-title = Game installation for “{ $name }”
profiles-installation-lead = Pick the Deadlock folder this profile is applied to.
profiles-used-by =
    { $count ->
        [one] used by { $count } profile
       *[other] used by { $count } profiles
    }
profiles-used-by-none = not used by any profile
profiles-installation-applied = “{ $name }” is in the game on { $drive } now. Its files stay there until you apply another profile on { $drive }.
profiles-installation-not-applied = Neither game folder changes until you apply “{ $name }”.
profiles-manage-installations = Manage installations in Settings
profiles-use-installation = Use this installation
profiles-d9-title =
    { $count ->
        [one] { $count } mod from “{ $name }” isn’t in your library
       *[other] { $count } mods from “{ $name }” aren’t in your library
    }
profiles-d9-file = The file
profiles-d9-lead = lists { $listed } mods. { $present } are already in your library; these { $count } aren’t:
profiles-d9-list = Missing mods
profiles-d9-from-gamebanana = { $count } from GameBanana
profiles-d9-local-count = { $count } local
profiles-gamebanana = GameBanana
profiles-can-download = Can be downloaded
profiles-local-mod = Local mod
profiles-cant-download = Can’t be downloaded
profiles-local-why = Added from disk on another PC, so it isn’t on GameBanana. Ask whoever shared the profile for the file.
profiles-d9-local-mode = The catalog is off (local mode), so deadloader can’t download mods. Import without them, then add them from disk — or turn the catalog on in Settings.
profiles-d9-after = Either way, “{ $name }” becomes your selected profile but isn’t applied — the game folder doesn’t change until you press Apply or Play.
profiles-import-without = Import without them
profiles-download-missing = Download missing ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } mod turned off
       *[other] { $count } mods turned off
    }
profiles-turned-on =
    { $count ->
        [one] { $count } mod turned on
       *[other] { $count } mods turned on
    }
profiles-differ = Only the files that differ were changed.
profiles-same = The same mods were already in the game; only order and variants were checked.
profiles-deleted = Profile “{ $name }” deleted.
profiles-deleted-not-applied = It wasn’t applied, so the game folder didn’t change.
profiles-deleted-applied =
    { $count ->
        [one] { $count } mod was removed from the game folder. It’s still in your library.
       *[other] { $count } mods were removed from the game folder. They’re still in your library.
    }
profiles-exported = Profile “{ $name }” exported.
profiles-imported = Profile “{ $name }” imported.
profiles-imported-text = It’s selected but not applied yet.
profiles-imported-without =
    { $count ->
        [one] Profile “{ $name }” imported without { $count } mod.
       *[other] Profile “{ $name }” imported without { $count } mods.
    }
profiles-imported-without-text = It’s selected but not applied yet. You can add the missing mods later.
profiles-imported-downloading =
    { $count ->
        [one] Profile “{ $name }” imported — { $count } mod is downloading.
       *[other] Profile “{ $name }” imported — { $count } mods are downloading.
    }
profiles-imported-downloading-text = It’s selected but not applied yet. Apply it when the downloads are done.
profiles-open-downloads = Open downloads
profiles-undo = Undo
profiles-check-update = Check for deadloader update
profiles-choose-another = Choose another file
profiles-switcher = Manage profiles
profiles-saved-as = Saved as
profiles-show-folder = Show in folder
