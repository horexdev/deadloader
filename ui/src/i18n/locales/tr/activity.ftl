# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Etkinlik
activity-sub =
    { $days ->
        [one] deadloader’ın bu bilgisayarda yaptığı her şey: ağ istekleri, dosya değişiklikleri ve uygulama olayları. Günlükler bu bilgisayarda { $days } gün tutulur.
       *[other] deadloader’ın bu bilgisayarda yaptığı her şey: ağ istekleri, dosya değişiklikleri ve uygulama olayları. Günlükler bu bilgisayarda { $days } gün tutulur.
    }
activity-open-logs = Günlük klasörünü aç
activity-logs-opened = Günlük klasörü Dosya Gezgini’nde açıldı.
activity-summary = Bugün bir bakışta
activity-net-today = Bugün ağ
activity-hosts-label = Bugün bağlanılan sunucular — filtrelemek için birini seçin
activity-hosts-contacted =
    { $count ->
        [one] { $count } sunucuya bağlanıldı
       *[other] { $count } sunucuya bağlanıldı
    }
activity-host-title =
    { $count ->
        [one] { $count } istek · { $source } — filtrelemek için seçin
       *[other] { $count } istek · { $source } — filtrelemek için seçin
    }
activity-net-none-local = yok — yerel mod
activity-net-none-yet = henüz yok
activity-nothing-else = Başka bir şey yok.
activity-blocked-lead =
    { $count ->
        [one] Şu sunucuya { $count } istek:
       *[other] Şu sunucuya { $count } istek:
    }
activity-blocked-tail =
    { $count ->
        [one] bağlantı kurulmadan engellendi.
       *[other] bağlantı kurulmadan engellendi.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } istek engellendi, hiç bağlanılmadı.
       *[other] { $count } istek engellendi, hiç bağlanılmadı.
    }
activity-show-entry = Kaydı göster
activity-game-today = Bugün oyun klasörü
activity-show-in-files = Dosyalar’da göster
activity-game-none = değişiklik yok
activity-no-game-changes = deadloader oyun klasöründe henüz hiçbir şeyi değiştirmedi.
activity-game-changes =
    { $count ->
        [one] { $count } değişiklik
       *[other] { $count } değişiklik
    }
activity-game-batches =
    { $count ->
        [one] { $count } seferde
       *[other] { $count } seferde
    }
activity-game-failed =
    { $count ->
        [one] { $count } başarısız
       *[other] { $count } başarısız
    }
activity-game-hardlink = { $count } bağlandı
activity-game-copy = { $count } kopyalandı
activity-game-delete = { $count } kaldırıldı
activity-game-rename = { $count } yeniden adlandırıldı
activity-game-write = { $count } yazıldı
activity-game-backup = { $count } yedeklendi
activity-game-restore = { $count } geri yüklendi
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi bir kez güncellendi
       *[other] gameinfo.gi { $count } kez güncellendi
    }
activity-no-requests-today = Bugün istek yok.
activity-no-requests-yet = Henüz istek yok.
activity-local-short = Katalog kapalı, bu yüzden deadloader hiçbir yere bağlanmıyor.
activity-never-contacted = deadloader kurulduğundan beri hiçbir sunucuya bağlanmadı.
activity-tab-network = Ağ
activity-tab-files = Dosyalar
activity-tab-app = Uygulama
activity-tab-off = kapalı
activity-tab-errors =
    { $count ->
        [one] · { $count } hata
       *[other] · { $count } hata
    }
activity-search = Günlükte ara
activity-search-clear = Aramayı temizle
activity-matches =
    { $count ->
        [one] { $count } sonuç
       *[other] { $count } sonuç
    }
activity-matches-more = { $count }+ sonuç
activity-f-time = Dönem
activity-f-purpose = Amaç
activity-f-host = Sunucu
activity-f-op = İşlem
activity-f-level = Düzey
activity-f-result = Sonuç
activity-all = Tümü
activity-ok = Tamam
activity-errors = Hatalar
activity-range-hour = Son bir saat
activity-range-today = Bugün
activity-range-7d = Son 7 gün
activity-range-30d = Son 30 gün
activity-clear-filters = Filtreleri temizle
activity-clear-all = Aramayı ve filtreleri temizle
activity-follow = Canlı izle
activity-follow-tip = Yeni kayıtları gerçekleştikleri anda en üstte göster
activity-follow-on = Yeni kayıtlar en üstte görünür
activity-follow-paused = { $time } itibarıyla duraklatıldı
activity-show-new =
    { $count ->
        [one] { $count } yeni kaydı göster
       *[other] { $count } yeni kaydı göster
    }
activity-net-aria = Ağ istekleri, en yenisi önce
activity-files-aria = Dosya işlemleri, en yenisi önce
activity-app-aria = Uygulama olayları, en yenisi önce
activity-c-redirects = Yönlendirmeler
activity-c-time = Saat
activity-c-purpose = Amaç
activity-c-request = İstek
activity-c-status = Durum
activity-c-size = Boyut
activity-c-duration = Süre
activity-c-started-by = Başlatan
activity-c-source = Sunucu kaynağı
activity-c-op = İşlem
activity-c-from-to = Nereden → nereye
activity-c-result = Sonuç
activity-c-change = Ait olduğu değişiklik
activity-c-level = Düzey
activity-c-message = İleti
activity-to = şuraya
activity-loading-more = Yükleniyor…
activity-p-catalog = Katalog
activity-p-thumbnail = Küçük resim
activity-p-description-image = Açıklama görseli
activity-p-download = İndirme
activity-p-update-check = Güncelleme denetimi
activity-src-builtin = Yerleşik
activity-src-github = GitHub
activity-src-user = Sizin eklediğiniz
activity-src-user-image = Görsel sunucusu
activity-src-blocked = İzin verilmiyor
activity-src-long-builtin = Yerleşik — GameBanana, deadloader’ın parçası
activity-src-long-github = GitHub — deadloader’ın kendi güncelleme denetimi, ayarlarda açık
activity-src-long-user = Sizin eklediğiniz — Ayarlar → Gelişmiş bölümünde
activity-src-long-user-image = İzin verilen görsel sunucusu — bir açıklama görselini yüklerken izin verdiniz
activity-src-long-blocked = İzin verilen sunucular listesinde yok
activity-by-auto = Otomatik
activity-by-user = Siz
activity-op-hardlink = Bağlama
activity-op-copy = Kopyalama
activity-op-rename = Yeniden adlandırma
activity-op-delete = Silme
activity-op-write = Yazma
activity-op-backup = Yedekleme
activity-op-restore = Geri yükleme
activity-lv-debug = Hata ayıklama
activity-lv-info = Bilgi
activity-lv-warn = Uyarı
activity-lv-error = Hata
activity-st-blocked = Engellendi
activity-st-blocked-long = Engellendi — bağlanılmadı
activity-st-failed = Başarısız
activity-redirects =
    { $count ->
        [one] { $count } yönlendirme
       *[other] { $count } yönlendirme
    }
activity-retries =
    { $count ->
        [one] { $count } yeniden deneme
       *[other] { $count } yeniden deneme
    }
activity-show-redirects = Yönlendirmeleri göster
activity-hide-redirects = Yönlendirmeleri gizle
activity-hop-request = Adım 1 · istek
activity-hop-redirect = Adım { $n } · yönlendirme
activity-hop-retry = Adım { $n } · deneme { $try }
activity-ms = { $ms } ms
activity-seconds = { $s } sn
activity-no-match = Eşleşen kayıt yok
activity-no-match-text = Başka bir dönem seçin ya da filtreleri ve aramayı temizleyin.
activity-empty-title = Henüz bir şey yok
activity-empty-text = deadloader her ağ isteğini, oyun klasöründeki her dosya değişikliğini ve her uygulama olayını gerçekleştiği anda buraya yazar. Henüz bir şey olmadı.
activity-local-title = Yerel mod — hiçbir istek yapılmıyor
activity-local-why = Katalog kapalı, bu yüzden deadloader hiçbir yere bağlanmıyor — ne GameBanana’ya ne de başka bir yere. Dosya değişiklikleri ve uygulama olayları diğer sekmelerde kaydedilmeye devam eder.
activity-local-do = GameBanana’ya göz atmak için katalogu ayarlardan açın. Her istek o zaman burada görünür.
activity-local-history = Aşağıdaki istekler yerel mod açılmadan önce yapıldı.
activity-enable-catalog = Katalogu aç
activity-details = Günlük kaydının ayrıntıları
activity-close-details = Ayrıntıları kapat
activity-close-esc = Kapat (Esc)
activity-why = Neden:
activity-todo = Ne yapmalı:
activity-open-advanced = Ayarlar → Gelişmiş’i aç
activity-full-record = Tam kayıt
activity-hops-title = Yönlendirmeler · { $count }
activity-user-hidden = Windows kullanıcı adınız günlüklerde gizlenir:
activity-settings-logs = Ayarlar → Günlükler
activity-copy = Kaydı kopyala
activity-copied = Kopyalandı
activity-kind-network = Ağ isteği
activity-kind-files = Dosya işlemi
activity-kind-app = Uygulama olayı
activity-d-time = Saat
activity-d-purpose = Amaç
activity-d-started-by = Başlatan
activity-d-request = İstek
activity-d-source = Sunucu kaynağı
activity-d-status = Durum
activity-d-size = Boyut
activity-d-duration = Süre
activity-d-range = Aralık
activity-d-error = Hata
activity-d-id = Kayıt kimliği
activity-d-request-id = İstek kimliği
activity-d-operation = İşlem
activity-d-from = Nereden
activity-d-to = Nereye
activity-d-file = Dosya
activity-d-result = Sonuç
activity-d-change = Ait olduğu değişiklik
activity-change-apply = “{ $profile }” profili uygulandı
activity-change-rollback = Oyun klasörünü geri alma
activity-change-profile-removed = “{ $profile }” profili silindi, modları oyundan çıkarıldı
activity-change-take-foreign = { $file } yönetime alındı
activity-change-gameinfo-fixed = Satırımız gameinfo.gi dosyasına eklendi
activity-change-recovered = Yarıda kalan değişiklik başlangıçta tamamlandı
activity-d-sha256 = SHA-256
activity-d-level = Düzey
activity-d-message = İleti
activity-d-fields = Ayrıntılar
activity-status-after =
    { $count ->
        [one] { $status } — { $count } yönlendirmeden sonra
       *[other] { $status } — { $count } yönlendirmeden sonra
    }
