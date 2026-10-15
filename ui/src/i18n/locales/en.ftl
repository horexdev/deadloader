## Notifications

n1 = Deadlock was updated — mods restored.
    .why = The update reset gameinfo.gi, so deadloader added its line back.
n2 = { $name } installed and enabled.
n3 =
    { $count ->
        [one] { $count } mod update available.
       *[other] { $count } mod updates available.
    }
n4 = Deadlock is running.
    .why = Changes will be applied automatically when you close the game.
    .pending =
        { $count ->
            [one] { $count } change pending
           *[other] { $count } changes pending
        }
n5 =
    { $reason ->
        [completed] An interrupted change was completed.
       *[undone] An interrupted change was undone; your game folder is as before.
    }
n6 =
    { $count ->
        [one] Found { $count } file in the game’s mod folder that deadloader didn’t create.
       *[other] Found { $count } files in the game’s mod folder that deadloader didn’t create.
    }
n7 =
    { $when ->
        [today] Offline — showing the catalog saved at { $saved }.
       *[earlier] Offline — showing the catalog saved on { $saved }.
    }
    .why = Downloads will resume when you’re back online.
n8 =
    { $count ->
        [one] Profile “{ $name }” applied — { $count } mod enabled.
       *[other] Profile “{ $name }” applied — { $count } mods enabled.
    }
n9 = A new deadloader version is available: { $version }.
n10 = Local mode is on.
    .why = The catalog is off, and deadloader makes no network requests.
n11 = Mods are copied, not linked, because deadloader and the game are on different drives.
    .why = This takes { $size } of extra disk space.
n12 =
    { $count ->
        [one] Deadlock was closed — { $count } change applied.
       *[other] Deadlock was closed — { $count } changes applied.
    }

## Errors: what happened, why, what to do

e1 = deadloader can’t save its data.
    .why =
        { $reason ->
            [access_denied] The folder next to deadloader.exe is read-only — for example, Windows doesn’t let programs write inside Program Files.
            [in_use] Another program, often an antivirus, has one of deadloader’s files open.
           *[other] Windows couldn’t write to the folder next to deadloader.exe.
        }
    .todo =
        { $reason ->
            [in_use] Close other programs and try again.
           *[other] Move deadloader to a folder you can write to, for example D:\Games\deadloader.
        }
e2 = Deadlock wasn’t found.
    .why = Steam or the game isn’t installed, or the game folder was moved.
    .todo = Choose the game folder manually.
e3 = A game file was changed by another program.
    .why = gameinfo.gi doesn’t look like the original — often Deadlock Mod Manager changed it. deadloader won’t overwrite someone else’s changes.
    .todo = Verify the game files in Steam; deadloader then adds its line.
e4 = The catalog is unavailable.
    .why = GameBanana changed its API, so deadloader can’t read it. Your library and profiles work as usual.
    .todo = Check for a deadloader update, or try again later.
e5 = GameBanana is busy.
    .why =
        { $reason ->
            [rate_limited] Too many requests in a short time (HTTP 429).
            [server] GameBanana’s server reported an error.
           *[other] Too many requests, or a server error.
        }
    .todo =
        { $wait ->
            [seconds]
                { $seconds ->
                    [one] deadloader retries automatically in { $seconds } second.
                   *[other] deadloader retries automatically in { $seconds } seconds.
                }
           *[later] deadloader retries automatically in a moment.
        }
e6 = Download failed.
    .why =
        { $reason ->
            [connection] The connection was lost.
            [timeout] The server took too long to respond.
            [server] GameBanana’s server reported an error.
           *[other] The download was interrupted.
        }
    .todo = It will resume from where it stopped.
e7 = The downloaded file is damaged.
    .why = Its checksum doesn’t match the one GameBanana lists.
    .todo = Download it again.
e8 = The archive can’t be unpacked.
    .why =
        { $reason ->
            [not_an_archive] The file isn’t a zip, 7z or RAR archive — the download may be incomplete.
            [damaged] The archive is damaged.
            [encrypted] The archive is protected with a password.
            [unsupported] It uses a compression method deadloader doesn’t support.
            [not_a_vpk] { $entry } isn’t a valid .vpk file.
           *[other] It’s damaged or uses an unsupported format.
        }
    .todo = Try another file of this mod, or report the problem to its author.
e9 = The archive is too large to unpack safely.
    .why =
        { $reason ->
            [too_large] It unpacks to more than { $limit }.
            [too_many_files]
                { $found ->
                    [one] It holds { $found } mod file, and the limit is { $limit }.
                   *[other] It holds { $found } mod files, and the limit is { $limit }.
                }
            [ratio] { $entry } is compressed suspiciously well — over { $limit } times.
            [memory] Unpacking it needs { $needed } of memory, and the limit is { $limit }.
           *[other] It goes over one of the safety limits for archives.
        }
    .todo =
        { $reason ->
            [memory] Try another file of this mod.
           *[other] If you trust it, raise the limit in Settings → Downloads.
        }
e10 = No mod files in this archive.
    .why =
        { $listing ->
            [files] It has no .vpk files, only { $contents }.
           *[empty] The archive is empty.
        }
    .todo = This file isn’t a Deadlock mod in a format deadloader can install.
e11 = Can’t unpack RAR archives on this PC.
    .why = Windows’ built-in tool can’t read this RAR, and neither 7-Zip nor WinRAR was found.
    .todo = Install 7-Zip (it’s free) and try again; the downloaded file is kept.
e12 = Not enough disk space.
    .why =
        { $reason ->
            [space] { $needed } is needed, and only { $available } is free.
           *[full] The disk is full.
        }
    .todo = Free some space, or delete old mod versions in Library — then try again.
e13 = Changes can’t be applied while Deadlock is running.
    .why = Windows locks the game’s files while it runs.
    .todo = Close Deadlock; deadloader applies the changes right after.
e14 = A mod file is missing or damaged in the library.
    .why = It was changed or deleted outside deadloader.
    .todo = Repair the mod to download the file again, or remove the mod.
e15 = Couldn’t open the link.
    .why = No default browser is set in Windows.
    .todo = Copy the link and open it yourself.
e16 = Request blocked.
    .why = GameBanana redirected to { $host }, which isn’t on the list of allowed hosts.
    .todo = Nothing to do — deadloader only talks to GameBanana. If you trust this host, add it in Settings → Advanced.
e17 = Couldn’t import the profile.
    .why =
        { $reason ->
            [newer] It was made by a newer version of deadloader.
            [unreadable] The file can’t be read — it may have been moved, deleted or opened by another program.
           *[not_a_profile] The file isn’t a deadloader profile.
        }
    .todo =
        { $reason ->
            [newer] Update deadloader, then import the profile again.
            [unreadable] Check that the file is still there and no other program has it open, then try again.
           *[not_a_profile] Check that you picked the right file.
        }
e18 = Couldn’t change files in the game folder.
    .why =
        { $reason ->
            [access_denied] Windows denied access to the game folder.
            [in_use] A file is in use by another program, such as an antivirus or another mod manager.
           *[other] Windows couldn’t change the files.
        }
    .todo = Close other programs and try again.
e19 = The image couldn’t be loaded.
    .why =
        { $reason ->
            [no_response] The host didn’t respond.
            [not_an_image] The file isn’t an image.
            [too_large] The image is too large.
           *[other] The host didn’t send a usable image.
        }
    .todo = The placeholder stays; you can try again.
e20 = Too many mods are enabled.
    .why =
        { $free ->
            [one] The game can load { $free } more mod file, and this profile needs { $wanted }.
           *[other] The game can load { $free } more mod files, and this profile needs { $wanted }.
        }
    .todo = Turn some mods off, or pick fewer add-ons.
e21 = A profile named “{ $name }” already exists.
    .why = Names are compared without upper or lower case, like file names.
    .todo = Choose another name.
e22 = “{ $name }” can’t be used as a profile name.
    .why = Windows doesn’t allow it as a file name — for example CON, a trailing dot, or any of \ / : * ? " < > |.
    .todo = Choose another name.
e23 = deadloader can’t read its file { $file }.
    .why =
        { $reason ->
            [damaged_at] The file is damaged (line { $line }, column { $column }).
            [damaged] The file is damaged.
            [newer] It was made by a newer version of deadloader.
           *[invalid] Its contents aren’t what deadloader expects.
        }
    .todo =
        { $kind ->
            [settings] deadloader uses the default settings for now and leaves the file as it is.
            [manifest] Restore manifest.json.bak, or update deadloader.
           *[other] Restore the file from a backup, or update deadloader.
        }
internal = Something went wrong inside deadloader.
    .why = It’s a mistake in deadloader, not something you did.
    .todo = Try again. If it happens again, report it with the log entry from Details.

## Buttons named in the texts above

action-undo = Undo
action-review = Review
action-retry = Retry
action-retry-now = Retry now
action-browse = Browse…
action-verify-in-steam = Verify in Steam
action-show-differences = Show differences
action-details = Details
action-repair = Repair
action-copy-link = Copy link
action-open-folder = Open folder
action-open-release-page = Open release page
action-enable-catalog = Enable catalog
action-learn-more = Learn more
action-view-log = View log

## Shared pieces

log-entry = Log entry { $id }
list-more = { $count } more

## Sizes, labelled like Windows Explorer labels them

size-bytes =
    { $count ->
        [one] { $count } byte
       *[other] { $count } bytes
    }
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
