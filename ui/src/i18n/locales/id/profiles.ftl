# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Profil
profiles-count =
    { $count ->
       *[other] { $count } profil
    }
profiles-in-game = Di game sekarang: { $list }
profiles-in-game-item = “{ $name }” di { $drive }
profiles-in-game-none = Tidak ada profil di game saat ini.
profiles-import = Impor…
profiles-import-title = Impor profil dari file .json
profiles-import-dialog = Impor profil
profiles-export-dialog = Ekspor profil
profiles-file-filter = Profil deadloader
profiles-new = Profil baru
profiles-selected = Dipilih
profiles-applied = Diterapkan
profiles-not-applied = Belum diterapkan
profiles-selected-title = Profil yang Anda ubah di pustaka dan mainkan
profiles-pending-title = Terapkan atau Main memasukkannya ke folder game
profiles-applied-title = File-nya sekarang ada di folder game di { $drive }
profiles-select = Pilih
profiles-select-of = Pilih “{ $name }”
profiles-more-of = Tindakan lain untuk “{ $name }”
profiles-rename = Ganti nama
profiles-duplicate = Duplikat
profiles-export = Ekspor sebagai file…
profiles-change-installation = Ubah instalasi game…
profiles-delete = Hapus
profiles-delete-confirm = Hapus…
profiles-delete-last = deadloader membutuhkan setidaknya satu profil.
profiles-name-label = Nama profil
profiles-save = Simpan
profiles-cancel = Batal
profiles-close = Tutup
profiles-close-esc = Tutup (Esc)
profiles-rename-hint = Enter menyimpan · Esc membatalkan
profiles-name-empty = Masukkan nama untuk profil.
profiles-mods =
    { $count ->
       *[other] { $count } mod aktif
    }
profiles-mods-none = Tidak ada mod aktif
profiles-applied-when = Diterapkan { $when }
profiles-last-applied = Terakhir diterapkan { $when }
profiles-never-applied = Belum pernah diterapkan
profiles-just-now = baru saja
profiles-preview-empty = Pilih profil ini, lalu aktifkan mod di pustaka.
profiles-footnote = Profil adalah daftar mod aktif beserta urutan, varian, dan versinya. Memilihnya tidak mengunduh atau menghapus apa pun; Terapkan hanya mengubah file yang berbeda.
profiles-copy-name = { $name } (salinan)
profiles-copy-name-n = { $name } (salinan { $number })
profiles-new-title = Profil baru
profiles-start-with = Mulai dengan
profiles-empty = Kosong
profiles-empty-hint = Tidak ada mod aktif — pilih nanti di pustaka.
profiles-copy-of = Salinan “{ $name }”
profiles-copy-hint = { $mods } — urutan, varian, dan versi yang sama.
profiles-installation = Instalasi game
profiles-installation-hint = Sama seperti “{ $name }”. Anda bisa mengubahnya nanti dari menu profil.
profiles-new-note = Membuat profil tidak memilih atau menerapkannya — game tidak berubah.
profiles-create = Buat profil
profiles-pending-what = Profil “{ $name }” belum ada di game.
profiles-pending-why-other = Folder game di { $drive } masih berisi “{ $other }”. Terapkan sekarang, atau tekan Main — deadloader menerapkannya sebelum memulai.
profiles-pending-why-empty = Folder game di { $drive } saat ini tidak berisi mod dari deadloader. Terapkan sekarang, atau tekan Main — deadloader menerapkannya sebelum memulai.
profiles-keep = Pertahankan “{ $name }”
profiles-apply = Terapkan “{ $name }”
profiles-other-what = “{ $name }” memakai instalasi game lain.
profiles-other-why = Terapkan dan Main memakai { $path }. Game di { $drive } tetap memakai “{ $other }”.
profiles-other-why-empty = Terapkan dan Main memakai { $path }. Game di { $drive } tidak berubah.
profiles-delete-title = Hapus profil “{ $name }”?
profiles-delete-applied =
    { $count ->
       *[other] “{ $name }” sedang ada di game. deadloader akan menghapus { $count } mod-nya dari folder game:
    }
profiles-delete-applied-empty = “{ $name }” sedang ada di game tetapi tidak punya mod, jadi folder game tidak berubah.
profiles-delete-stay = Mod tetap ada di pustaka dan profil lain. Sampai Anda menerapkan profil lain di sana, Deadlock berjalan tanpa mod dari deadloader.
profiles-delete-next = “{ $name }” menjadi profil yang dipilih. Profil itu tidak diterapkan sampai Anda menekan Terapkan atau Main.
profiles-delete-undoable = Ini tidak bisa dibatalkan. Untuk menyimpan salinan, ekspor profil terlebih dahulu.
profiles-export-first = Ekspor dulu
profiles-delete-profile = Hapus profil
profiles-installation-title = Instalasi game untuk “{ $name }”
profiles-installation-lead = Pilih folder Deadlock tempat profil ini diterapkan.
profiles-used-by =
    { $count ->
       *[other] dipakai oleh { $count } profil
    }
profiles-used-by-none = tidak dipakai profil mana pun
profiles-installation-applied = “{ $name }” sedang ada di game di { $drive }. File-nya tetap di sana sampai Anda menerapkan profil lain di { $drive }.
profiles-installation-not-applied = Tidak ada folder game yang berubah sampai Anda menerapkan “{ $name }”.
profiles-manage-installations = Kelola instalasi di Pengaturan
profiles-use-installation = Pakai instalasi ini
profiles-d9-title =
    { $count ->
       *[other] { $count } mod dari “{ $name }” tidak ada di pustaka Anda
    }
profiles-d9-file = File
profiles-d9-lead = mencantumkan { $listed } mod. { $present } sudah ada di pustaka Anda; { $count } ini belum:
profiles-d9-list = Mod yang hilang
profiles-d9-from-gamebanana = { $count } dari GameBanana
profiles-d9-local-count = { $count } lokal
profiles-gamebanana = GameBanana
profiles-can-download = Bisa diunduh
profiles-local-mod = Mod lokal
profiles-cant-download = Tidak bisa diunduh
profiles-local-why = Ditambahkan dari disk di PC lain, jadi tidak ada di GameBanana. Minta file-nya dari orang yang membagikan profil.
profiles-d9-local-mode = Katalog mati (mode lokal), jadi deadloader tidak bisa mengunduh mod. Impor tanpa mod tersebut, lalu tambahkan dari disk — atau nyalakan katalog di Pengaturan.
profiles-d9-after = Apa pun pilihannya, “{ $name }” menjadi profil yang dipilih tetapi tidak diterapkan — folder game baru berubah saat Anda menekan Terapkan atau Main.
profiles-import-without = Impor tanpa mod itu
profiles-download-missing = Unduh yang hilang ({ $count })
profiles-turned-off =
    { $count ->
       *[other] { $count } mod dimatikan
    }
profiles-turned-on =
    { $count ->
       *[other] { $count } mod dinyalakan
    }
profiles-differ = Hanya file yang berbeda yang diubah.
profiles-same = Mod yang sama sudah ada di game; hanya urutan dan varian yang diperiksa.
profiles-deleted = Profil “{ $name }” dihapus.
profiles-deleted-not-applied = Profil itu tidak diterapkan, jadi folder game tidak berubah.
profiles-deleted-applied =
    { $count ->
       *[other] { $count } mod dihapus dari folder game. Mod tersebut masih ada di pustaka Anda.
    }
profiles-exported = Profil “{ $name }” diekspor.
profiles-imported = Profil “{ $name }” diimpor.
profiles-imported-text = Profil dipilih tetapi belum diterapkan.
profiles-imported-without =
    { $count ->
       *[other] Profil “{ $name }” diimpor tanpa { $count } mod.
    }
profiles-imported-without-text = Profil dipilih tetapi belum diterapkan. Anda bisa menambahkan mod yang hilang nanti.
profiles-imported-downloading =
    { $count ->
       *[other] Profil “{ $name }” diimpor — { $count } mod sedang diunduh.
    }
profiles-imported-downloading-text = Profil dipilih tetapi belum diterapkan. Terapkan setelah unduhan selesai.
profiles-open-downloads = Buka unduhan
profiles-undo = Urungkan
profiles-check-update = Periksa pembaruan deadloader
profiles-choose-another = Pilih file lain
profiles-switcher = Kelola profil
