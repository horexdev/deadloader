# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Pengaturan
settings-nav = Bagian pengaturan
settings-reset-nav = Atur ulang semua pengaturan…
settings-default = Bawaan: { $value }
settings-restore = Kembalikan ke bawaan
settings-on = Aktif
settings-off = Nonaktif
settings-cancel = Batal
settings-remove = Hapus
settings-remove-item = Hapus { $item }
settings-add = Tambah
settings-open-folder = Buka folder
settings-open-folder-at = Buka folder: { $path }
settings-copied = Disalin ke papan klip.
settings-folder = Folder
settings-example = Contoh:

settings-section-general = Umum
settings-section-general-lead = Bahasa, saat mulai, dan zoom.
settings-section-game = Game
settings-section-game-lead = Di mana Deadlock terpasang dan apa yang boleh diubah deadloader di sana.
settings-section-catalog = Katalog
settings-section-catalog-lead = Katalog GameBanana, konten sensitif, gambar, dan tautan.
settings-section-library = Pustaka
settings-section-library-lead = Pembaruan mod, menghapus mod, dan versi lama.
settings-section-downloads = Unduhan
settings-section-downloads-lead = Antrean unduhan, arsip, dan ekstraksi.
settings-section-network = Jaringan
settings-section-network-lead = Cara deadloader berkomunikasi dengan GameBanana.
settings-section-cache = Cache
settings-section-cache-lead = Gambar mini dan halaman katalog yang disimpan di PC ini.
settings-section-logs = Log
settings-section-logs-lead = Apa yang disimpan Aktivitas, dan berapa lama.
settings-section-advanced = Lanjutan
settings-section-advanced-lead = Berkas pengaturan dan domain tambahan yang diizinkan.
settings-section-about = Tentang deadloader
settings-section-about-lead = Versi, lisensi, cara memeriksa unduhan ini, dan di mana data Anda berada.

# General
settings-general-group = Bahasa dan saat mulai
settings-language = Bahasa
settings-language-help = Bahasa sistem: bahasa tampilan Windows jika tersedia di deadloader, jika tidak bahasa Inggris.
settings-language-system = Bahasa sistem ({ $language })
settings-startup-profile = Profil saat mulai
settings-startup-profile-help = Profil yang dipilih saat deadloader dimulai. Memilihnya tidak menerapkannya.
settings-startup-last = Terakhir dipilih ({ $name })
settings-startup-last-plain = Terakhir dipilih
settings-app-updates = Periksa pembaruan deadloader saat mulai
settings-app-updates-help = Saat mulai, deadloader bertanya sekali ke GitHub (api.github.com) apakah ada versi yang lebih baru. Pembaruan selalu manual. Anda memilih ini saat pertama kali menjalankannya.
settings-app-updates-local = Tidak tersedia: mode lokal sama sekali tidak membuat permintaan jaringan. Opsi ini tersedia setelah Anda mengaktifkan katalog (Pengaturan → Katalog).
settings-zoom = Zoom
settings-zoom-keys-1 = Juga dengan Ctrl + roda tetikus,
settings-zoom-keys-2 = dan
settings-zoom-keys-3 = untuk mengatur ulang.
settings-ctrl = Ctrl
settings-percent = { $value }%

# Game
settings-installs-group = Pemasangan game
settings-installs-lead = deadloader menemukan sendiri pemasangan Deadlock di Steam. Setiap profil memakai salah satunya — ubah di Profil.
settings-install-library = Pustaka Steam { $library }
settings-install-build = build { $build }
settings-install-used =
    { $count ->
        [0] tidak dipakai profil mana pun
       *[other] dipakai { $count } profil: { $names }
    }
settings-install-selected = { $name } (dipilih)
settings-install-applied = { $name } (diterapkan di sini)
settings-install-steam = Ditemukan lewat Steam
settings-install-manual = Ditambahkan manual
settings-install-known = Berkas deadloader masih ada di sini
settings-install-manual-meta = Ditambahkan manual
settings-gi-ours = gameinfo.gi: OK — baris deadloader ada
settings-gi-wait = gameinfo.gi: disiapkan saat profil memakainya
settings-gi-rolled = gameinfo.gi: asli — Terapkan menambahkan lagi baris deadloader
settings-gi-foreign = gameinfo.gi: diubah oleh program lain
settings-gi-missing = gameinfo.gi: tidak ditemukan — foldernya mungkin dipindahkan
settings-installs-none = Belum ada pemasangan yang ditemukan.
settings-add-manual = Tambah manual…
settings-add-manual-hint = Pilih folder Deadlock yang tidak ditemukan.
settings-add-manual-title = Pilih folder Deadlock
settings-install-added = Pemasangan ditambahkan.
settings-install-removed = Pemasangan dihapus dari daftar.
settings-install-removed-text = Tidak ada yang diubah di folder itu.
settings-gameinfo-group = Setelah pembaruan game
settings-autofix = Perbaiki gameinfo.gi secara otomatis
settings-autofix-help = Pembaruan Deadlock mengatur ulang berkas game yang memberitahunya untuk memuat mod. deadloader mencadangkannya, memasang kembali barisnya, dan memberi tahu Anda. Jika nonaktif, spanduk akan bertanya dulu.
settings-danger-group = Batalkan dan hapus
settings-rollback = Kembalikan folder game profil yang dipilih
settings-rollback-help = Untuk pemasangan profil yang dipilih { $profile } ({ $drive }): menghapus { $files } berkas mod yang ditaruh deadloader di sana dan barisnya di gameinfo.gi. Pustaka dan profil tetap ada; Terapkan mengembalikan semuanya.
settings-rollback-help-others = Folder game lain tidak disentuh; pilih profil yang memakainya untuk mengembalikannya.
settings-rollback-nothing = Tidak ada yang perlu dikembalikan di { $drive } — deadloader tidak punya berkas di sana sekarang.
settings-rollback-no-game = Deadlock tidak ditemukan, jadi tidak ada folder game untuk dikembalikan.
settings-rollback-button = Kembalikan…
settings-rolled-title = Folder game di { $drive } sudah dikembalikan.
settings-rolled-text =
    { $foreign ->
        [0] Di { $drive }, { $files } berkas mod dan baris deadloader di gameinfo.gi dihapus. Pustaka dan profil disimpan — Terapkan atau Main mengembalikan mod.
       *[other] Di { $drive }, { $files } berkas mod, baris deadloader di gameinfo.gi, dan { $foreign } berkas asing dihapus. Pustaka dan profil disimpan — Terapkan atau Main mengembalikan mod.
    }
settings-wipe = Hapus semua data deadloader
settings-wipe-help = Menghapus pustaka, profil, log, cache, dan pengaturan ({ $size }), lalu menutup deadloader. deadloader.exe tetap ada — hapus sendiri setelahnya.
settings-wipe-button = Hapus semua data…

# Catalog
settings-catalog-group = Katalog GameBanana
settings-catalog = Gunakan katalog
settings-catalog-help-on = Jelajahi dan pasang mod dari GameBanana. deadloader hanya berkomunikasi dengan GameBanana, dan setiap permintaan tercantum di Aktivitas. Nonaktif berarti mode lokal.
settings-catalog-help-off = Nonaktif — mode lokal: deadloader sama sekali tidak membuat permintaan jaringan. Anda tetap bisa menambahkan mod dari disk.
settings-consent-title = Satu langkah dulu.
settings-consent-text = Sebelum katalog aktif, deadloader menunjukkan host mana yang akan dihubungi dan ketentuan GameBanana.
settings-consent-go = Tinjau dan aktifkan
settings-local-on = Mode lokal aktif.
settings-local-on-text = Katalog nonaktif dan deadloader tidak membuat permintaan jaringan.
settings-ttl = Gunakan ulang halaman katalog yang dimuat selama
settings-ttl-help = Dalam waktu ini, kembali ke sebuah halaman tidak bertanya lagi ke GameBanana.
settings-minutes = { $count } mnt
settings-hours = { $count } jam
settings-prefetch = Muat halaman berikutnya lebih awal
settings-prefetch-help = Setelah Anda menggulir sekitar 70% hasil, hasil berikutnya dimuat agar kisi terus berlanjut.
settings-nsfw-group = Konten sensitif
settings-nsfw = Mod dengan peringkat konten (NSFW)
settings-nsfw-help = Disembunyikan: tidak ditampilkan sama sekali. Diburamkan: gambar buram dengan label “Konten sensitif”; klik untuk menampilkan. Ditampilkan: tanpa buram.
settings-nsfw-age-done = Usia dikonfirmasi pada { $date }.
settings-nsfw-age-ask = Keluar dari “Disembunyikan” meminta Anda mengonfirmasi usia sekali.
settings-nsfw-hidden = Disembunyikan
settings-nsfw-blurred = Diburamkan
settings-nsfw-shown = Ditampilkan
settings-images-group = Gambar dan tautan
settings-description-images = Gambar di deskripsi mod
settings-description-images-help = Berasal dari host selain GameBanana, mis. i.imgur.com. Saat diklik: penanda tempat sampai Anda mengkliknya.
settings-images-click = Saat diklik
settings-images-always = Selalu
settings-prechecked = Centang sebelumnya “Selalu izinkan gambar dari host ini”
settings-prechecked-help = Di dialog yang muncul saat gambar deskripsi pertama kali datang dari host baru.
settings-hosts = Host gambar yang diizinkan
settings-hosts-help = Gambar dari host ini dimuat tanpa bertanya. Di Aktivitas, permintaannya ditandai “host gambar yang diizinkan”.
settings-hosts-built-in = Bawaan
settings-hosts-none = Belum ada host yang ditambahkan — setiap host baru bertanya dulu.
settings-host-removed = { $host } dihapus
settings-host-removed-text = Gambar darinya akan bertanya lagi sebelum dimuat.
settings-undo = Urungkan
settings-gif = Putar GIF otomatis
settings-gif-help = Nonaktif: GIF menampilkan bingkai pertamanya dengan tombol putar.
settings-links = Tautan eksternal
settings-links-help = Tautan terbuka di peramban bawaan Anda. “Tanya dulu” menampilkan alamat lengkap sebelum dibuka.
settings-links-ask = Tanya dulu
settings-links-open = Langsung buka

# Library
settings-updates-group = Pembaruan mod
settings-check-updates = Periksa pembaruan mod saat mulai
settings-check-updates-help = Membandingkan mod Anda dari GameBanana dengan katalog. Tidak pernah dalam mode lokal.
settings-auto-download = Unduh pembaruan secara otomatis
settings-auto-download-help = Hanya unduhan — deadloader tetap bertanya sebelum mod beralih ke versi baru.
settings-disk-group = Menghapus dan versi lama
settings-deletion = Saat Anda menghapus mod
settings-deletion-help = Anda bisa memilih lain setiap kali di dialog hapus.
settings-deletion-permanent = Permanen
settings-deletion-bin = Ke Recycle Bin
settings-old-versions = Versi lama
settings-old-versions-help = Setiap versi yang diunduh disimpan agar Anda bisa kembali. Hapus versi lama sebuah mod di detailnya.
settings-old-keep-all = Simpan semua

# Downloads
settings-queue-group = Antrean
settings-parallel = Unduhan bersamaan
settings-parallel-help = Berapa berkas yang diunduh pada saat yang sama.
settings-speed = Batas kecepatan
settings-speed-help = Hanya untuk unduhan mod — katalog dan gambar mini tidak dibatasi.
settings-speed-none = Tanpa batas
settings-speed-value = { $value } MB/dtk
settings-keep-archives = Simpan arsip setelah dipasang
settings-keep-archives-help = Nonaktif: berkas .zip, .7z, atau .rar dihapus setelah berkas .vpk-nya masuk ke pustaka.
settings-unpack-group = Ekstraksi
settings-limits = Batas ekstraksi
settings-limits-help = Melindungi dari arsip yang mengembang menjadi sangat besar. Jika arsip yang Anda percayai ditolak, naikkan batasnya.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = Ukuran hasil ekstraksi
settings-limit-ratio = Rasio kompresi
settings-limit-files = Berkas mod per arsip
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = berkas
settings-unit-s = dtk
settings-number-error = { $field }: masukkan bilangan bulat dari { $min } sampai { $max }. Sampai saat itu nilai sebelumnya tetap dipakai.

# Network
settings-network-local = Mode lokal aktif.
settings-network-local-text = deadloader tidak membuat permintaan jaringan; pengaturan ini berlaku setelah katalog aktif.
settings-requests-group = Permintaan ke GameBanana
settings-api-parallel = Permintaan katalog bersamaan
settings-api-parallel-help = Permintaan ke API GameBanana pada saat yang sama. Gambar dan unduhan dihitung terpisah.
settings-timeouts = Batas waktu
settings-timeouts-help = Sambungan: menyerah jika tidak tersambung dalam waktu ini. Respons: menyerah saat server berhenti mengirim.
settings-timeouts-default = 10 dtk · 30 dtk
settings-timeout-connect = Sambungan
settings-timeout-read = Respons
settings-retries = Coba lagi
settings-retries-help = Berapa kali permintaan yang gagal diulang, dengan jeda yang makin lama, sebelum deadloader menampilkan galat.
settings-connection-group = Sambungan
settings-proxy = Proksi
settings-proxy-help = Tidak ada: sambungan langsung. Sistem: proksi yang diatur di Windows. Manual: masukkan di bawah.
settings-proxy-none = Tidak ada
settings-proxy-system = Sistem
settings-proxy-manual = Manual
settings-proxy-host = Alamat
settings-proxy-port = Port
settings-proxy-hint = Proksi HTTP. Hanya permintaan deadloader sendiri yang melewatinya.
settings-proxy-invalid = Masukkan nama host atau alamat tanpa nama pengguna atau kata sandi, mis. proxy.example.com.

# Cache
settings-thumbs-group = Gambar mini dan halaman katalog
settings-cache-limit = Batas cache
settings-cache-limit-help = Saat cache penuh, gambar mini yang paling lama tidak Anda lihat dihapus lebih dulu.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = Ukuran saat ini
settings-cache-breakdown = { $thumbnails } gambar mini · { $pages } halaman katalog
settings-cache-empty = Kosong — belum ada yang di-cache.
settings-cache-used = { $used } dari { $limit }
settings-cache-clear = Kosongkan cache
settings-cache-cleared = Cache dikosongkan — { $size } dibebaskan. Gambar mini dimuat lagi saat diperlukan.
settings-cache-cleared-partly = { $failed } berkas sedang dipakai dan tetap ada sampai pengosongan berikutnya.

# Logs
settings-keep-group = Menyimpan log
settings-log-days = Simpan log selama
settings-log-days-help = Entri yang lebih lama dihapus saat deadloader dimulai.
settings-days = { $count ->
       *[other] { $count } hari
    }
settings-year = 1 tahun
settings-log-size = Ukuran maksimum
settings-log-size-help = Di atas ukuran ini, berkas log tertua dihapus lebih dulu.
settings-privacy-group = Privasi
settings-mask = Sembunyikan nama pengguna Windows Anda
settings-mask-help = Jalur di log menampilkan { $masked } alih-alih nama Anda — lebih aman saat Anda membagikan log.
settings-logs-folder-group = Folder log
settings-logs-folder = Lokasi log
settings-logs-now = { $size } · { $files } berkas · tertua dari { $date }
settings-open-logs = Buka folder log

# Advanced
settings-advanced-warning = Untuk pengguna berpengalaman.
settings-advanced-warning-text = Kesalahan di sini bisa mengubah perilaku deadloader. Ia memeriksa berkas sebelum menyimpan dan menyimpan cadangan versi sebelumnya.
settings-files-group = Edit berkas pengaturan
settings-file-settings = Pengaturan
settings-file-manifest = Manifes folder game
settings-file-label = Berkas yang diedit
settings-file-settings-lead = Semua isi halaman ini dalam satu berkas. Perubahan muncul di halaman setelah disimpan.
settings-file-manifest-lead = Daftar yang ditaruh deadloader di folder game. Pengembalian bergantung padanya — entri yang salah bisa meninggalkan berkas.
settings-json-error = Ini bukan JSON yang valid, jadi tidak disimpan.
settings-json-where = Baris { $line }, kolom { $column }: ada yang kurang atau berlebih di sini atau tepat sebelumnya — sering kali koma atau tanda kutip.
settings-json-fix = Perbaiki bagian itu lalu simpan lagi — sampai saat itu deadloader tetap memakai berkas yang sekarang.
settings-json-refused = Nilai-nilai ini tidak bisa dipakai, jadi tidak ada yang disimpan.
settings-json-newer = Berkas ini dibuat oleh deadloader yang lebih baru (format { $found }; versi ini membaca sampai { $supported }), jadi tidak disimpan.
settings-json-invalid = deadloader tidak bisa memakai berkas ini, jadi tidak disimpan.
settings-json-details = Detail
settings-problem-unknown-field = { $path }: bukan pengaturan versi ini.
settings-problem-invalid-value = { $path }: nilai ini tidak bisa dipakai di sini.
settings-problem-out-of-range = { $path }: { $found } di luar { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: “{ $entry }” tidak valid di sini.
settings-problem-duplicate-entry = { $path }: “{ $entry }” tercantum dua kali.
settings-problem-needs-age = { $path }: menampilkan konten NSFW memerlukan konfirmasi usia dulu (Pengaturan → Katalog).
settings-problem-local-mode = { $path }: tidak tersedia dalam mode lokal.
settings-json-idle = Diperiksa saat menyimpan.
settings-json-saved = Valid dan tersimpan. Versi sebelumnya disimpan sebagai { $backup }.
settings-json-saved-plain = Valid dan tersimpan.
settings-json-save = Periksa dan simpan
settings-json-discard = Buang suntingan saya
settings-json-backup = Sebelum menyimpan, berkas sebelumnya disimpan sebagai cadangan; 10 terakhir tetap ada.
settings-domains-group = Domain tambahan yang diizinkan
settings-domains-lead = deadloader hanya berkomunikasi dengan GameBanana. Jika unduhan dialihkan ke host lain dan diblokir, Anda bisa mengizinkan host itu di sini. Di Aktivitas, permintaan ke sana ditandai “ditambahkan pengguna”.
settings-domains-built-in = Bawaan:
settings-domain-user = ditambahkan pengguna
settings-domains-none = Tidak ada domain tambahan.
settings-domain-add = Tambah domain
settings-domain-hint = Hanya nama host, mis. files.example.com.
settings-domain-empty = Masukkan nama host dulu, mis. files.example.com.
settings-domain-not-host = Masukkan nama host saja — tanpa https:// atau jalur, mis. files.example.com.
settings-domain-invalid = “{ $host }” bukan nama host. Periksa ejaannya, mis. files.example.com.
settings-domain-built-in = { $host } adalah bawaan dan selalu diizinkan.
settings-domain-listed = { $host } sudah ada di daftar.

# About
settings-version = versi { $version }
settings-commit = Dibangun dari commit
settings-commit-local = Build lokal — tidak dibangun oleh GitHub Actions
settings-copy-commit = Salin id commit
settings-release-page = Halaman rilis
settings-licence-group = Lisensi
settings-licence-help = deadloader adalah perangkat lunak bebas: Anda boleh memakai, mempelajari, membagikan, dan mengubahnya di bawah GPL-3. Tanpa jaminan apa pun.
settings-read-licence = Baca lisensi
settings-source = Kode sumber
settings-verify-group = Cara memeriksa unduhan ini
settings-verify-lead = Dua pemeriksaan independen bahwa deadloader.exe ini adalah yang diterbitkan di GitHub.
settings-verify-hash = Bandingkan checksum SHA-256
settings-verify-hash-help = Harus sama dengan checksum di halaman rilis. Salinan yang diubah bisa menampilkan nilai palsu di sini, jadi hitung juga sendiri:
settings-verify-hash-cmd = Di PowerShell, di folder exe:
settings-verify-attest = Periksa atestasi build
settings-verify-attest-help = Membuktikan bahwa GitHub Actions membangun berkas ini dari kode sumber yang diterbitkan. Memerlukan GitHub CLI:
settings-copy-hash = Salin checksum
settings-copy-command = Salin perintah
settings-unsigned = deadloader.exe tidak ditandatangani kode.
settings-unsigned-text = Windows SmartScreen mungkin menampilkan “Windows melindungi PC Anda” saat pertama kali dijalankan. Dua pemeriksaan di atas memastikan berkas ini asli.
settings-webview-group = Mesin jendela
settings-webview-help = Jendela deadloader digambar oleh WebView2, bagian dari Windows. Windows memperbaruinya sendiri. deadloader mematikan:
settings-webview-off-1 = Pencarian DNS-over-HTTPS
settings-webview-off-2 = Masuk dengan akun Microsoft di dalam jendela
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Lalu lintas jaringan latar belakang dan pembaruan komponen
settings-webview-off-5 = Panggilan server isi otomatis formulir
settings-webview-off-6 = Laporan galat ke Microsoft (domain reliability)
settings-webview-no-route = Dan jendela sama sekali tidak punya jalur ke internet.
settings-webview-unknown = versi tidak diketahui
settings-data-group = Data Anda
settings-data-folder = Folder data
settings-data-folder-help = Semua yang disimpan deadloader ada di satu folder ini di samping exe.
settings-notice = Pemberitahuan saat pertama dijalankan
settings-notice-help = Versi { $version }, diterima pada { $date } dalam { $language }. Pilihan Anda: katalog { $catalog }, pemeriksaan pembaruan { $updates }.
settings-notice-none = Belum diterima.
settings-notice-again = Baca lagi

# D7
settings-d7-title = Kembalikan folder game di { $drive }?
settings-d7-from =
    { $others ->
        [0] Pemasangan profil yang dipilih { $profile }. deadloader menghapus dari
       *[other] Pemasangan profil yang dipilih { $profile } (juga dipakai { $others }). deadloader menghapus dari
    }
settings-d7-files =
    { $count ->
       *[other] { $count } berkas mod yang ditambahkannya di
    }
settings-d7-line = Barisnya di gameinfo.gi
settings-d7-line-why = — berkas kembali seperti saat dipasang Steam.
settings-d7-stays = Tetap seperti semula:
settings-d7-library = Pustaka Anda — { $mods } mod, { $size }
settings-d7-profiles =
    { $count ->
       *[other] { $count } profil Anda — { $names }
    }
settings-d7-other = Pemasangan di { $path } ({ $profile }) — tidak disentuh. Untuk mengembalikannya, pilih profil yang memakainya lalu kembalikan lagi.
settings-d7-other-plain = Pemasangan di { $path } — tidak disentuh.
settings-d7-back = Terapkan atau Main mengembalikan semuanya.
settings-d7-foreign =
    { $count ->
       *[other] Hapus juga { $count } berkas yang tidak dibuat deadloader
    }
settings-d7-foreign-why = dari program lain atau pemasangan manual. Berkas itu dihapus permanen; deadloader tidak bisa mengembalikannya.
settings-d7-confirm = Kembalikan
settings-d7-confirm-foreign =
    { $count ->
       *[other] Kembalikan dan hapus { $count } berkas
    }

# D8
settings-d8-title = Hapus semua data deadloader?
settings-d8-from = deadloader menghapus semua isi
settings-d8-from-2 = lalu menutup.
settings-d8-what = Apa
settings-d8-contains = Isi
settings-d8-size = Ukuran
settings-d8-total = Total
settings-d8-library = Pustaka
settings-d8-library-has = { $mods } mod, { $files } berkas
settings-d8-profiles = Profil
settings-d8-logs = Log
settings-d8-logs-has = { $files } berkas, sejak { $date }
settings-d8-logs-none = tidak ada berkas log
settings-d8-cache = Cache
settings-d8-cache-has = gambar mini dan halaman katalog
settings-d8-settings = Pengaturan
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
       *[other] termasuk { $backups } cadangan
    }
settings-d8-other = Lainnya
settings-d8-other-has = unduhan yang belum selesai, cadangan gameinfo.gi, data mesin jendela
settings-d8-exe = Tidak dihapus: deadloader.exe itu sendiri — hapus sendiri jika tidak diperlukan lagi.
settings-d8-rollback = Kembalikan dulu setiap folder game yang dipakai deadloader (disarankan)
settings-d8-rollback-why = Menghapus berkas mod deadloader dan barisnya di gameinfo.gi dari setiap folder game sebelum menghapus. Berkas asing tetap ada.
settings-d8-rollback-off = Tidak dicentang: berkas mod deadloader tetap di folder game dan Deadlock terus memuatnya — tanpa deadloader untuk menghapusnya.
settings-d8-type-1 = Untuk mengonfirmasi, ketik
settings-d8-type-2 = { "" }
settings-d8-word = hapus
settings-d8-hint = Tombol hapus terbuka saat kata cocok.
settings-d8-hint-ok = Dikonfirmasi. Ini tidak bisa diurungkan.
settings-d8-type-first = Ketik “{ $word }” dulu
settings-d8-deleting = Menghapus…
settings-d8-confirm = Hapus dan tutup
settings-d8-confirm-rollback = Kembalikan, hapus, dan tutup

# Reset
settings-reset-title = Atur ulang semua pengaturan?
settings-reset-body = Setiap pengaturan kembali ke bawaannya, termasuk host gambar yang diizinkan dan domain tambahan.
settings-reset-keeps = Pustaka, profil, dan log Anda tidak disentuh, dan folder game tetap seperti semula.
settings-reset-backup = Cadangan pengaturan saat ini disimpan sebagai settings.json.bak-…
settings-reset-confirm = Atur ulang pengaturan
settings-reset-done = Semua pengaturan dikembalikan ke bawaan.
settings-reset-done-text = Cadangan: { $backup }
