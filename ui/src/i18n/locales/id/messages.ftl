## Notifications

n1 = Deadlock telah diperbarui — mod dipulihkan.
    .why = Pembaruan itu mengatur ulang gameinfo.gi, jadi deadloader menambahkan barisnya kembali.
n2 = { $name } telah dipasang dan diaktifkan.
n3 = Tersedia { $count } pembaruan mod.
n4 = Deadlock sedang berjalan.
    .why = Perubahan akan diterapkan secara otomatis saat Anda menutup game.
    .pending = { $count } perubahan tertunda
n5 =
    { $reason ->
        [completed] Perubahan yang sempat terputus telah diselesaikan.
       *[undone] Perubahan yang sempat terputus telah dibatalkan; folder game Anda tetap seperti semula.
    }
n6 = Ditemukan { $count } file di folder mod game yang tidak dibuat oleh deadloader.
n7 =
    { $when ->
        [today] Offline — menampilkan katalog yang disimpan pukul { $saved }.
       *[earlier] Offline — menampilkan katalog yang disimpan pada { $saved }.
    }
    .why = Unduhan akan dilanjutkan saat Anda kembali online.
n8 = Profil “{ $name }” diterapkan — { $count } mod diaktifkan.
n9 = Versi baru deadloader tersedia: { $version }.
n10 = Mode lokal aktif.
    .why = Katalog dinonaktifkan dan deadloader tidak membuat permintaan jaringan apa pun.
n11 = Mod disalin, bukan ditautkan, karena deadloader dan game berada di drive yang berbeda.
    .why = Ini memakan ruang disk tambahan sebesar { $size }.
n12 = Deadlock telah ditutup — { $count } perubahan diterapkan.

## Errors: what happened, why, what to do

e1 = deadloader tidak dapat menyimpan datanya.
    .why =
        { $reason ->
            [access_denied] Folder tempat deadloader.exe berada bersifat hanya-baca — misalnya, Windows tidak mengizinkan program menulis di dalam Program Files.
            [in_use] Program lain, sering kali antivirus, sedang membuka salah satu file deadloader.
           *[other] Windows tidak dapat menulis ke folder tempat deadloader.exe berada.
        }
    .todo =
        { $reason ->
            [in_use] Tutup program lain, lalu coba lagi.
           *[other] Pindahkan deadloader ke folder yang bisa Anda tulisi, misalnya D:\Games\deadloader.
        }
e2 = Deadlock tidak ditemukan.
    .why = Steam atau game belum terpasang, atau folder game telah dipindahkan.
    .todo = Pilih folder game secara manual.
e3 = Sebuah file game diubah oleh program lain.
    .why = gameinfo.gi tidak terlihat seperti aslinya — sering kali diubah oleh Deadlock Mod Manager. deadloader tidak menimpa perubahan yang dibuat pihak lain.
    .todo = Verifikasi integritas file game di Steam; setelah itu deadloader akan menambahkan barisnya.
e4 = Katalog tidak tersedia.
    .why = GameBanana mengubah API-nya, sehingga deadloader tidak dapat membacanya. Pustaka dan profil Anda tetap berfungsi seperti biasa.
    .todo = Periksa apakah ada pembaruan deadloader, atau coba lagi nanti.
e5 = GameBanana sedang sibuk.
    .why =
        { $reason ->
            [rate_limited] Terlalu banyak permintaan dalam waktu singkat (HTTP 429).
            [server] Server GameBanana melaporkan kesalahan.
           *[other] Terlalu banyak permintaan, atau terjadi kesalahan server.
        }
    .todo =
        { $wait ->
            [seconds] deadloader akan mencoba lagi secara otomatis dalam { $retry_in_s } detik.
           *[later] deadloader akan segera mencoba lagi secara otomatis.
        }
e6 = Unduhan gagal.
    .why =
        { $reason ->
            [local_mode] deadloader sedang dalam mode lokal, jadi tidak membuat permintaan jaringan.
            [dns] Alamat server unduhan tidak ditemukan.
            [connect] deadloader tidak dapat terhubung ke server unduhan.
            [tls] Koneksi aman ke server unduhan tidak dapat dibuat.
            [timeout] Server terlalu lama merespons.
            [connection_lost] Koneksi terputus.
            [http_status] Server merespons dengan kesalahan (HTTP { $status }).
           *[other] Unduhan terhenti.
        }
    .todo =
        { $reason ->
            [local_mode] Aktifkan katalog di Pengaturan untuk mengunduh mod.
           *[other] Unduhan akan dilanjutkan dari titik terakhir.
        }
e7 = File yang diunduh rusak.
    .why =
        { $reason ->
            [size] Ukurannya tidak cocok dengan yang tercantum di GameBanana.
           *[checksum] Checksum-nya tidak cocok dengan yang tercantum di GameBanana.
        }
    .todo = Unduh ulang file tersebut.
e8 = Arsip tidak dapat diekstrak.
    .why =
        { $reason ->
            [not_an_archive] File ini bukan arsip ZIP, 7z, atau RAR — mungkin unduhannya tidak lengkap.
            [damaged] Arsip rusak.
            [encrypted] Arsip dilindungi kata sandi.
            [unsupported] Arsip ini menggunakan metode kompresi yang tidak didukung deadloader.
            [not_a_vpk] { $entry } bukan file .vpk yang valid.
           *[other] Arsip rusak atau menggunakan format yang tidak didukung.
        }
    .todo = Coba file lain dari mod ini, atau laporkan masalahnya kepada pembuatnya.
e9 = Arsip terlalu besar untuk diekstrak dengan aman.
    .why =
        { $reason ->
            [too_large]
                { $measured ->
                    [yes] Setelah diekstrak, ukurannya akan menjadi { $unpacked }, sedangkan batasnya { $limit }.
                   *[no] Setelah diekstrak, ukurannya melebihi { $limit }.
                }
            [too_many_files] Arsip ini berisi { $found } file mod, sedangkan batasnya { $limit }.
            [ratio] { $entry } dikompresi secara mencurigakan — lebih dari { $limit } kali.
            [memory] Untuk mengekstraknya diperlukan memori { $needed }, sedangkan batasnya { $limit }.
           *[other] Arsip ini melampaui salah satu batas keamanan untuk arsip.
        }
    .todo =
        { $reason ->
            [memory] Coba file lain dari mod ini.
           *[other] Jika Anda memercayainya, naikkan batasnya di Pengaturan → Unduhan.
        }
e10 = Tidak ada file mod di arsip ini.
    .why =
        { $listing ->
            [files] Arsip ini tidak berisi file .vpk, hanya { $contents }.
           *[empty] Arsip ini kosong.
        }
    .todo = File ini bukan mod Deadlock dalam format yang dapat dipasang oleh deadloader.
e11 = Arsip RAR tidak dapat diekstrak di PC ini.
    .why = Alat bawaan Windows tidak dapat membaca RAR ini, dan 7-Zip maupun WinRAR tidak ditemukan.
    .todo = Pasang 7-Zip (gratis), lalu coba lagi; file yang sudah diunduh tetap disimpan.
e12 = Ruang disk tidak cukup.
    .why =
        { $reason ->
            [space] Diperlukan { $needed }, sedangkan ruang kosong hanya { $available }.
           *[full] Disk penuh.
        }
    .todo = Kosongkan sebagian ruang atau hapus versi mod lama di Pustaka — lalu coba lagi.
e13 = Perubahan tidak dapat diterapkan selama Deadlock berjalan.
    .why = Windows mengunci file game selama game berjalan.
    .todo = Tutup Deadlock; deadloader akan langsung menerapkan perubahan setelahnya.
e14 = Sebuah file mod di pustaka hilang atau rusak.
    .why = File tersebut diubah atau dihapus di luar deadloader.
    .todo = Perbaiki mod untuk mengunduh ulang file tersebut, atau hapus mod.
e15 = Tautan tidak dapat dibuka.
    .why = Belum ada browser default yang diatur di Windows.
    .todo = Salin tautan dan buka sendiri.
e16 = Permintaan diblokir.
    .why =
        { $reason ->
            [not_https] Server mengalihkan ke alamat tanpa HTTPS, dan deadloader tidak pernah mengikuti pengalihan seperti itu.
            [bad_redirect] Server mengirim pengalihan yang tidak dapat diikuti deadloader.
            [too_many_redirects] Server melakukan pengalihan terlalu banyak kali.
           *[not_listed] { $host } tidak ada dalam daftar host yang diizinkan, jadi deadloader tidak terhubung ke sana.
        }
    .todo =
        { $reason ->
            [not_listed] Anda tidak perlu melakukan apa pun — deadloader hanya berkomunikasi dengan GameBanana. Jika Anda memercayai host ini, tambahkan di Pengaturan → Lanjutan.
           *[not_https] Anda tidak perlu melakukan apa pun — deadloader hanya mengikuti pengalihan yang aman. Coba lagi nanti.
        }
e17 = Profil tidak dapat diimpor.
    .why =
        { $reason ->
            [newer] Profil ini dibuat oleh versi deadloader yang lebih baru.
            [unreadable] File tidak dapat dibaca — mungkin telah dipindahkan, dihapus, atau sedang dibuka oleh program lain.
           *[not_a_profile] File ini bukan profil deadloader.
        }
    .todo =
        { $reason ->
            [newer] Perbarui deadloader, lalu impor ulang profilnya.
            [unreadable] Pastikan file masih ada dan tidak sedang dibuka oleh program lain, lalu coba lagi.
           *[not_a_profile] Pastikan Anda memilih file yang benar.
        }
e18 = File di folder game tidak dapat diubah.
    .why =
        { $reason ->
            [access_denied] Windows menolak akses ke folder game.
            [in_use] Sebuah file sedang digunakan oleh program lain, seperti antivirus atau pengelola mod lain.
           *[other] Windows tidak dapat mengubah file tersebut.
        }
    .todo = Tutup program lain, lalu coba lagi.
e19 = Gambar tidak dapat dimuat.
    .why =
        { $reason ->
            [no_response] Host tidak merespons.
            [not_an_image] File ini bukan gambar.
            [too_large] Gambar terlalu besar.
           *[other] Host tidak mengirimkan gambar yang dapat digunakan.
        }
    .todo = Placeholder tetap ditampilkan; Anda dapat mencoba lagi.
e20 = Terlalu banyak mod yang diaktifkan.
    .why = Game hanya dapat memuat { $free } file mod lagi, sedangkan profil ini memerlukan { $wanted }.
    .todo = Nonaktifkan beberapa mod, atau pilih lebih sedikit add-on.
e21 = Profil bernama “{ $name }” sudah ada.
    .why = Nama dibandingkan tanpa membedakan huruf besar dan kecil, seperti nama file.
    .todo = Pilih nama lain.
e22 = “{ $name }” tidak dapat digunakan sebagai nama profil.
    .why = Windows tidak mengizinkannya sebagai nama file — misalnya CON, titik di akhir, atau karakter apa pun dari \ / : * ? " < > |.
    .todo = Pilih nama lain.
e23 = deadloader tidak dapat membaca file miliknya, { $file }.
    .why =
        { $reason ->
            [damaged_at] File rusak (baris { $line }, kolom { $column }).
            [damaged] File rusak.
            [newer] File ini dibuat oleh versi deadloader yang lebih baru.
           *[invalid] Isinya tidak sesuai dengan yang diharapkan deadloader.
        }
    .todo =
        { $kind ->
            [settings] Untuk sementara, deadloader memakai pengaturan default dan membiarkan file apa adanya.
            [manifest] Pulihkan manifest.json.bak, atau perbarui deadloader.
           *[other] Pulihkan file dari cadangan, atau perbarui deadloader.
        }
e24 = File ini sudah tidak ada di GameBanana.
    .why = Pembuatnya telah menghapus atau menggantinya (HTTP { $status }).
    .todo = Buka halaman mod untuk memilih file lain, atau periksa apakah ada pembaruan mod.
e25 = GameBanana menolak permintaan.
    .why = Server menjawab dengan kode { $status }. Biasanya ini hanya sementara.
    .todo = Coba lagi nanti. Jika terus terjadi, laporkan di halaman GitHub deadloader.
internal = Terjadi kesalahan di dalam deadloader.
    .why = Ini kesalahan deadloader, bukan kesalahan Anda.
    .todo = Coba lagi. Jika terjadi lagi, laporkan beserta entri log dari Detail.

## Buttons named in the texts above

action-undo = Urungkan
action-review = Tinjau
action-retry = Coba lagi
action-retry-now = Coba lagi sekarang
action-browse = Telusuri…
action-verify-in-steam = Verifikasi di Steam
action-show-differences = Tampilkan perbedaan
action-details = Detail
action-repair = Perbaiki
action-copy-link = Salin tautan
action-open-folder = Buka folder
action-open-release-page = Buka halaman rilis
action-enable-catalog = Aktifkan katalog
action-learn-more = Pelajari lebih lanjut
action-view-log = Lihat log

## Shared pieces

log-entry = Entri log { $id }
list-more = { $count } lainnya

## Sizes, labelled like Windows Explorer labels them

size-bytes = { $count } byte
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
