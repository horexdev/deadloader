# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Setup progress
wizard-caption-setup = First-time setup
wizard-caption-changed = Updated notice
wizard-caption-catalog = Turn on the catalog
wizard-caption-review = First-run notice
wizard-language = Language
wizard-step-welcome = Welcome
wizard-step-risks = Risks
wizard-step-network = Network
wizard-step-privacy = Privacy
wizard-step-updates = Updates
wizard-step-setup = Setup
wizard-step-of = Step { $n } of 6
wizard-changed-of =
    { $count ->
        [one] 1 changed step
       *[other] { $count } changed steps
    }
wizard-meta-changed = Changed
wizard-meta-kept = Unchanged
wizard-back = Back
wizard-next = Next
wizard-finish = Finish setup
wizard-continue = Continue
wizard-close = Close
wizard-cancel = Cancel
wizard-to-catalog = Opens the catalog next.
wizard-to-library = Opens your library next (local mode).

# Step 1
wizard-s1-title = Welcome to deadloader
wizard-s1-lead = deadloader installs Deadlock mods from GameBanana or from your disk and shows you every change it makes.
wizard-f1a = It adds mod files to the game’s mod folder and three lines to
wizard-f1b = — a game file that tells Deadlock to load mods.
wizard-f2 = Everything can be undone: Rollback removes all of it from the game.
wizard-f3a = All its data stays in the
wizard-f3b = folder next to
wizard-f3c = .
wizard-f4 = deadloader is not affiliated with Valve or GameBanana.
wizard-f5 = Free software under the GPL-3, provided without any warranty.
wizard-licence = Licence (GPL-3)
wizard-source = Source code

# Step 2
wizard-s2-title = Mods are at your own risk
wizard-r1 = Valve has no official modding policy for Deadlock. Using mods is your decision and your risk.
wizard-r2 = Game updates can break mods. deadloader sets itself up again after an update, but a mod may not work until its author updates it.
wizard-r3 = Mods are made by other people under their own licences. deadloader can’t vouch for their content; for each file it shows GameBanana’s antivirus result.
wizard-agree = I understand that deadloader changes game files and that I use mods at my own risk.
wizard-agree-note = This confirms steps 1 and 2. It’s required to continue.
wizard-agree-error = Tick this box to continue.
wizard-agree-reason = Tick the box first.

# Step 3
wizard-s3-title = Network and GameBanana
wizard-s3-lead = The catalog talks directly to GameBanana’s unofficial API — and nowhere else. These are the only hosts it contacts:
wizard-hosts = Hosts
wizard-host-api = catalog and mod pages
wizard-host-images = thumbnails and gallery
wizard-host-files = downloads
wizard-host-cache = downloads (files.gamebanana.com redirects there)
wizard-host-new = New
wizard-s3-images = Images in mod descriptions from other hosts load only after you allow each host. Every request is listed in
wizard-s3-images-end = .
wizard-activity = Activity
wizard-s3-tos = Using the catalog means using GameBanana: their Terms of Service apply, and GameBanana requires users to be 13 or older.
wizard-tos = GameBanana Terms of Service
wizard-choice = How do you want to start?
wizard-use-catalog = Use the catalog
wizard-use-catalog-text = Browse and install mods from GameBanana. deadloader connects only to the hosts above.
wizard-local = Not now — local mode
wizard-local-text = No network requests at all; add mods from disk only. You can turn the catalog on later in Settings.
wizard-choice-error = Choose one of the two options.
wizard-choice-reason = Choose an option first.
wizard-last-time = Your previous choice
wizard-changed-title = What changed ({ $date })
wizard-changed-foot = You confirmed the other steps on { $date }; they haven’t changed.

# Step 4
wizard-s4-title = Your privacy
wizard-p1 = No account and no telemetry. deadloader sends nothing anywhere except the GameBanana requests from the previous step.
wizard-p1-local = No account and no telemetry. You chose local mode, so deadloader sends nothing at all.
wizard-p2a = Logs stay on this PC, in the
wizard-p2b = folder. They hide your Windows user name:
wizard-p3 = deadloader’s window is drawn by Microsoft WebView2, a part of Windows. deadloader switches off WebView2’s own online features; Windows updates WebView2 independently of deadloader.
wizard-p4 = You can always see what deadloader did in
wizard-p4-end = .

# Step 5
wizard-s5-title = Updates of deadloader
wizard-u1 = deadloader can check GitHub for a newer version of itself when it starts.
wizard-updates-label = Check GitHub for new deadloader versions at start
wizard-updates-on = On
wizard-updates-off = Off (default)
wizard-updates-na = Unavailable in local mode
wizard-u2a = When on, deadloader makes one request to
wizard-u2b = at each start and tells you if a new version exists.
wizard-u3 = Updating is always manual: you download the new version from GitHub yourself. deadloader never replaces itself.
wizard-u4 = Unavailable: local mode makes no network requests at all. The option becomes available when you turn the catalog on in Settings.
wizard-u5 = You can change this later in Settings → General.

# Step 6
wizard-s6-title = Setup
wizard-s6-lead = deadloader looked for the game and checked a few things. Nothing has been changed yet, and you can change all of this later in Settings.
wizard-checking = Checking…
wizard-topic-game = Game
wizard-topic-exe = deadloader’s location
wizard-topic-disk = Disk space
wizard-topic-foreign = Game’s mod folder
wizard-topic-gameinfo = Game file for mods
wizard-game-found = Deadlock found
wizard-game-chosen = Folder chosen
wizard-change = Change…
wizard-game-missing = Deadlock wasn’t found
wizard-game-missing-text = Steam or the game isn’t installed, or the game folder was moved. Choose the game folder yourself — now or later in Settings → Game.
wizard-browse = Browse…
wizard-browse-title = Choose the Deadlock folder
wizard-help = Help
wizard-help-a = In Steam: right-click Deadlock → Manage → Browse local files. Choose that folder; it usually ends in
wizard-help-b = .
wizard-exe-ok = In a permanent folder
wizard-exe-downloads = deadloader is in your Downloads folder
wizard-exe-desktop = deadloader is on your Desktop
wizard-exe-warn-a = Its data lives next to the exe, so cleaning up would delete your library and profiles. Close deadloader, move the whole folder somewhere permanent, e.g.
wizard-exe-warn-b = , and start it from there.
wizard-open-folder = Open folder
wizard-opened = Opened in Explorer
wizard-linked = Mods are linked — no extra disk space
wizard-linked-text = deadloader and the game are on the same drive ({ $drive }).
wizard-copied = Mods will be copied, not linked
wizard-copied-text = deadloader is on { $data } and the game is on { $game }. Each enabled mod then takes its size a second time; the library shows this as “copied — +N MB”.
wizard-learn-more = Learn more
wizard-drives-more = A link lets the game use the library’s file without a second copy. Windows can only link files on the same drive. If deadloader is also on { $game }, nothing is copied.
wizard-foreign-none = No foreign mod files
wizard-foreign-none-text = The mod folder contains nothing deadloader doesn’t know.
wizard-foreign-some =
    { $count ->
        [one] 1 file from another source
       *[other] { $count } files from another source
    }
wizard-foreign-some-text = deadloader didn’t put them there — probably another mod manager or a manual install. It leaves them as they are; review them later in the Library.
wizard-show-files = Show files
wizard-hide-files = Hide files
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Will be set up
wizard-gi-setup-a = deadloader adds three lines to
wizard-gi-setup-b = when you first apply mods. Nothing is changed now.
wizard-gi-ours = Already set up
wizard-gi-ours-text = deadloader’s lines are already there.
wizard-gi-foreign = Changed by another program
wizard-gi-foreign-text = It doesn’t look like the original — often Deadlock Mod Manager does this. deadloader won’t overwrite someone else’s changes and will explain what to do after setup.
wizard-what-is = What is this?
wizard-gi-more = is a game file that tells Deadlock which folders to load. Usually you fix it by verifying the game files in Steam; deadloader then adds its lines.

# E1
wizard-nw-title = deadloader can’t save its data.
wizard-nw-why-a = The folder next to
wizard-nw-why-b = is read-only: Windows doesn’t let programs write there (for example inside Program Files).
wizard-nw-path = deadloader’s folder
wizard-nw-do = What to do
wizard-nw-s1 = Exit deadloader.
wizard-nw-s2a = Move the whole folder to a place you can write to, for example
wizard-nw-s2b = . Don’t use Program Files.
wizard-nw-s3a = Start
wizard-nw-s3b = from the new place.
wizard-nw-why-folder = deadloader keeps everything next to the exe so nothing is spread across your PC: deleting the folder removes deadloader completely.
wizard-nw-details = Details
wizard-nw-copy = Copy details
wizard-nw-copied = Copied
wizard-nw-no-log = Not saved to the log — deadloader can’t write there.
wizard-exit = Exit
