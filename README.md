# deadloader

A portable, transparent mod manager for [Deadlock](https://store.steampowered.com/app/1422450/) with a built-in
[GameBanana](https://gamebanana.com/games/20948) catalog. Windows 10/11 (x64).

- Browse, search and install mods and sounds from GameBanana inside the app.
- Keep a local library; enable, disable and order mods; switch between profiles.
- Every network request and every file operation is shown in the app and written to a log.
- deadloader talks only to GameBanana. No account, no telemetry, no backend of its own.
- Everything it changes in the game can be undone with one click.

deadloader is not affiliated with Valve or GameBanana.

> **Status:** in early development — there is no release yet.

## Download
Get `deadloader.exe` from [Releases](https://github.com/horexdev/deadloader/releases). Each release lists the SHA-256
of the file and is built by GitHub Actions with an
[artifact attestation](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations):
```
gh attestation verify deadloader.exe --repo horexdev/deadloader
```
Releases are not code-signed yet, so Windows may show "Windows protected your PC" — choose "More info → Run anyway"
after checking the hash.

## Install
deadloader is a single exe and needs no installation.
1. Put `deadloader.exe` in its own folder **you can write to**, ideally on the same drive as Deadlock, e.g.
   `D:\Games\deadloader\`. Not `Program Files` (read-only for normal users), not Downloads or the Desktop.
2. Run it. A short first-run guide explains what it does and lets you choose between the catalog and local mode.

If deadloader says it can't save its data, the folder is read-only: move the folder somewhere else.

## Where your data lives
Everything is in `deadloader-data\` next to the exe: library, profiles, settings, cache, logs and the WebView2
profile. Nothing is written to `%APPDATA%` or the registry.

In the game, deadloader only adds mod files to `game\citadel\addons\` and one line to `game\citadel\gameinfo.gi`.

## Uninstall
Settings → Game → **Rollback** removes everything deadloader put into the game. Then delete the deadloader folder.

## Network
With the catalog on, deadloader connects only to `gamebanana.com`, `images.gamebanana.com`, `files.gamebanana.com`
and `filecache<number>.gamebanana.com` — plus image hosts you allow for mod descriptions, and GitHub if you enable the
deadloader update check. In local mode it makes no network requests at all.
The window is drawn by Microsoft WebView2, part of Windows. deadloader starts it with its online features
(DNS-over-HTTPS, Microsoft account sign-in, SmartScreen, component updates and others) switched off and gives it no
route to the internet; WebView2 itself is updated by Windows.

## Privacy
deadloader collects nothing and sends nothing about you. Logs stay on your PC and hide your Windows user name.
Requests to GameBanana are subject to GameBanana's [Terms of Service](https://gamebanana.com/wikis/334).

## License
[GPL-3.0](LICENSE). Mods are made by their authors and are subject to their own licences; you use them at your own risk.
