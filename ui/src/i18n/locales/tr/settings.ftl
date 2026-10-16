# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Ayarlar
settings-nav = Ayar bölümleri
settings-reset-nav = Tüm ayarları sıfırla…
settings-default = Varsayılan: { $value }
settings-restore = Varsayılana döndür
settings-on = Açık
settings-off = Kapalı
settings-cancel = İptal
settings-remove = Kaldır
settings-remove-item = { $item } kaldır
settings-add = Ekle
settings-open-folder = Klasörü aç
settings-open-folder-at = Klasörü aç: { $path }
settings-copied = Panoya kopyalandı.
settings-folder = Klasör
settings-example = Örnek:

settings-section-general = Genel
settings-section-general-lead = Dil, başlangıç ve yakınlaştırma.
settings-section-game = Oyun
settings-section-game-lead = Deadlock’un nerede kurulu olduğu ve deadloader’ın orada neleri değiştirebileceği.
settings-section-catalog = Katalog
settings-section-catalog-lead = GameBanana kataloğu, hassas içerik, görseller ve bağlantılar.
settings-section-library = Kitaplık
settings-section-library-lead = Mod güncellemeleri, modları silme ve eski sürümler.
settings-section-downloads = İndirmeler
settings-section-downloads-lead = İndirme kuyruğu, arşivler ve açma.
settings-section-network = Ağ
settings-section-network-lead = deadloader’ın GameBanana ile nasıl konuştuğu.
settings-section-cache = Önbellek
settings-section-cache-lead = Bu bilgisayarda kayıtlı küçük resimler ve katalog sayfaları.
settings-section-logs = Günlükler
settings-section-logs-lead = Etkinlik’in neleri ne kadar süre sakladığı.
settings-section-advanced = Gelişmiş
settings-section-advanced-lead = Ayar dosyaları ve ek izin verilen alan adları.
settings-section-about = deadloader hakkında
settings-section-about-lead = Sürüm, lisans, bu indirmeyi nasıl doğrulayacağınız ve verilerinizin nerede olduğu.

# General
settings-general-group = Dil ve başlangıç
settings-language = Dil
settings-language-help = Sistem dili: deadloader destekliyorsa Windows’un görüntüleme dili, aksi halde İngilizce.
settings-language-system = Sistem dili ({ $language })
settings-startup-profile = Başlangıçtaki profil
settings-startup-profile-help = deadloader başladığında hangi profilin seçili olduğu. Seçmek onu uygulamaz.
settings-startup-last = Son seçilen ({ $name })
settings-startup-last-plain = Son seçilen
settings-app-updates = Başlangıçta deadloader güncellemelerini denetle
settings-app-updates-help = deadloader başlangıçta GitHub’a (api.github.com) bir kez daha yeni bir sürüm olup olmadığını sorar. Güncelleme her zaman elle yapılır. Bunu ilk açılışta seçtiniz.
settings-app-updates-local = Kullanılamıyor: yerel modda hiçbir ağ isteği yapılmaz. Seçenek, kataloğu açtığınızda kullanılabilir olur (Ayarlar → Katalog).
settings-zoom = Yakınlaştırma
settings-zoom-keys-1 = Ctrl + fare tekerleğiyle de,
settings-zoom-keys-2 = ve sıfırlamak için
settings-zoom-keys-3 = kullanılabilir.
settings-ctrl = Ctrl
settings-percent = %{ $value }

# Game
settings-installs-group = Oyun kurulumları
settings-installs-lead = deadloader Deadlock’un Steam kurulumlarını kendisi bulur. Her profil bunlardan birini kullanır — bunu Profiller’de değiştirebilirsiniz.
settings-install-library = Steam kitaplığı { $library }
settings-install-build = derleme { $build }
settings-install-used =
    { $count ->
        [0] hiçbir profil kullanmıyor
       *[other] { $count } profil kullanıyor: { $names }
    }
settings-install-selected = { $name } (seçili)
settings-install-applied = { $name } (burada uygulandı)
settings-install-steam = Steam aracılığıyla bulundu
settings-install-manual = Elle eklendi
settings-install-known = Burada hâlâ deadloader dosyaları var
settings-install-manual-meta = Elle eklendi
settings-gi-ours = gameinfo.gi: Tamam — deadloader’ın satırları yerinde
settings-gi-wait = gameinfo.gi: bir profil kullandığında hazırlanır
settings-gi-rolled = gameinfo.gi: orijinal — Uygula, deadloader’ın satırlarını yeniden ekler
settings-gi-foreign = gameinfo.gi: başka bir program tarafından değiştirilmiş
settings-gi-missing = gameinfo.gi: bulunamadı — klasör taşınmış olabilir
settings-installs-none = Henüz kurulum bulunamadı.
settings-add-manual = Elle ekle…
settings-add-manual-hint = Bulunamayan bir Deadlock klasörü seçin.
settings-add-manual-title = Deadlock klasörünü seçin
settings-install-added = Kurulum eklendi.
settings-install-removed = Kurulum listeden kaldırıldı.
settings-install-removed-text = O klasörde hiçbir şey değiştirilmedi.
settings-gameinfo-group = Oyun güncellemelerinden sonra
settings-autofix = gameinfo.gi’yi otomatik onar
settings-autofix-help = Bir Deadlock güncellemesi, modların yüklenmesini söyleyen oyun dosyasını sıfırlar. deadloader dosyayı yedekler, satırlarını geri koyar ve size haber verir. Kapalıyken önce bir uyarı şeridi sorar.
settings-danger-group = Geri al ve kaldır
settings-rollback = Seçili profilin oyun klasörünü geri al
settings-rollback-help = Seçili profil { $profile } kurulumu için ({ $drive }): deadloader’ın oraya koyduğu { $files } mod dosyasını ve gameinfo.gi’deki satırlarını kaldırır. Kitaplık ve profiller kalır; Uygula her şeyi geri getirir.
settings-rollback-help-others = Diğer oyun klasörlerine dokunulmaz; onları geri almak için onları kullanan bir profil seçin.
settings-rollback-nothing = { $drive } üzerinde geri alınacak bir şey yok — deadloader’ın şu an orada dosyası yok.
settings-rollback-no-game = Deadlock bulunamadı, bu yüzden geri alınacak oyun klasörü yok.
settings-rollback-button = Geri al…
settings-rolled-title = { $drive } üzerindeki oyun klasörü geri alındı.
settings-rolled-text =
    { $foreign ->
        [0] { $drive } üzerinde { $files } mod dosyası ve deadloader’ın gameinfo.gi’deki satırları kaldırıldı. Kitaplık ve profiller korundu — Uygula veya Oyna modları geri getirir.
       *[other] { $drive } üzerinde { $files } mod dosyası, deadloader’ın gameinfo.gi’deki satırları ve { $foreign } yabancı dosya kaldırıldı. Kitaplık ve profiller korundu — Uygula veya Oyna modları geri getirir.
    }
settings-wipe = Tüm deadloader verilerini sil
settings-wipe-help = Kitaplığı, profilleri, günlükleri, önbelleği ve ayarları ({ $size }) siler, ardından deadloader’ı kapatır. deadloader.exe kalır — onu daha sonra kendiniz silin.
settings-wipe-button = Tüm verileri sil…

# Catalog
settings-catalog-group = GameBanana kataloğu
settings-catalog = Kataloğu kullan
settings-catalog-help-on = GameBanana’dan mod arayıp kurun. deadloader yalnızca GameBanana ile konuşur ve her istek Etkinlik’te listelenir. Kapalı, yerel mod demektir.
settings-catalog-help-off = Kapalı — yerel mod: deadloader hiçbir ağ isteği yapmaz. Diskten mod eklemeye devam edebilirsiniz.
settings-consent-title = Önce bir adım.
settings-consent-text = Katalog açılmadan önce deadloader hangi sunuculara bağlanacağını ve GameBanana’nın koşullarını gösterir.
settings-consent-go = İncele ve aç
settings-local-on = Yerel mod açık.
settings-local-on-text = Katalog kapalı ve deadloader ağ isteği yapmıyor.
settings-ttl = Yüklenen katalog sayfalarını şu süre boyunca yeniden kullan
settings-ttl-help = Bu süre içinde bir sayfaya geri dönmek GameBanana’ya yeniden sormaz.
settings-minutes = { $count } dk
settings-hours = { $count } sa
settings-prefetch = Sonraki sayfayı önceden yükle
settings-prefetch-help = Sonuçların yaklaşık %70’ini kaydırdığınızda, ızgara devam etsin diye sonrakiler yüklenir.
settings-nsfw-group = Hassas içerik
settings-nsfw = İçerik derecelendirmeli modlar (NSFW)
settings-nsfw-help = Gizli: hiç gösterilmez. Bulanık: “Hassas içerik” etiketli bulanık bir resim; görmek için tıklayın. Görünür: bulanıklık yok.
settings-nsfw-age-done = Yaş { $date } tarihinde onaylandı.
settings-nsfw-age-ask = “Gizli”den çıkarken yaşınızı bir kez onaylamanız istenir.
settings-nsfw-hidden = Gizli
settings-nsfw-blurred = Bulanık
settings-nsfw-shown = Görünür
settings-images-group = Görseller ve bağlantılar
settings-description-images = Mod açıklamalarındaki görseller
settings-description-images-help = GameBanana dışındaki sunuculardan gelir, ör. i.imgur.com. Tıklayınca: siz tıklayana kadar bir yer tutucu.
settings-images-click = Tıklayınca
settings-images-always = Her zaman
settings-prechecked = “Bu sunucudan gelen görsellere her zaman izin ver”i önceden işaretle
settings-prechecked-help = Bir açıklama görseli ilk kez yeni bir sunucudan geldiğinde gösterilen iletişim kutusunda.
settings-hosts = İzin verilen görsel sunucuları
settings-hosts-help = Bu sunuculardan gelen görseller sormadan yüklenir. Etkinlik’te istekleri “izin verilen görsel sunucusu” olarak işaretlenir.
settings-hosts-built-in = Yerleşik
settings-hosts-none = Eklenmiş sunucu yok — her yeni sunucu önce sorar.
settings-host-removed = { $host } kaldırıldı
settings-host-removed-text = Oradan gelen görseller yüklenmeden önce yeniden sorar.
settings-undo = Geri al
settings-gif = GIF’leri otomatik oynat
settings-gif-help = Kapalı: GIF’ler ilk karelerini bir oynat düğmesiyle gösterir.
settings-links = Dış bağlantılar
settings-links-help = Bağlantılar varsayılan tarayıcınızda açılır. “Önce sor” açmadan önce tam adresi gösterir.
settings-links-ask = Önce sor
settings-links-open = Hemen aç

# Library
settings-updates-group = Mod güncellemeleri
settings-check-updates = Başlangıçta mod güncellemelerini denetle
settings-check-updates-help = GameBanana’dan gelen modlarınızı katalogla karşılaştırır. Yerel modda asla.
settings-auto-download = Güncellemeleri otomatik indir
settings-auto-download-help = Yalnızca indirme — deadloader bir mod yeni sürüme geçmeden önce yine sorar.
settings-disk-group = Silme ve eski sürümler
settings-deletion = Bir modu sildiğinizde
settings-deletion-help = Silme iletişim kutusunda her seferinde farklı seçebilirsiniz.
settings-deletion-permanent = Kalıcı olarak
settings-deletion-bin = Geri dönüşüm kutusuna
settings-old-versions = Eski sürümler
settings-old-versions-help = Geri dönebilmeniz için indirilen her sürüm saklanır. Bir modun eski sürümlerini ayrıntılarından silin.
settings-old-keep-all = Hepsini sakla

# Downloads
settings-queue-group = Kuyruk
settings-parallel = Eş zamanlı indirmeler
settings-parallel-help = Aynı anda kaç dosya indirileceği.
settings-speed = Hız sınırı
settings-speed-help = Yalnızca mod indirmeleri için — katalog ve küçük resimler sınırlanmaz.
settings-speed-none = Sınır yok
settings-speed-value = { $value } MB/sn
settings-keep-archives = Kurulumdan sonra arşivleri sakla
settings-keep-archives-help = Kapalı: .zip, .7z veya .rar dosyası, .vpk dosyaları kitaplığa girer girmez silinir.
settings-unpack-group = Açma
settings-limits = Açma sınırları
settings-limits-help = Devasa bir şeye açılan arşivlere karşı korur. Güvendiğiniz bir arşiv reddedilirse sınırı yükseltin.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = Açılmış boyut
settings-limit-ratio = Sıkıştırma oranı
settings-limit-files = Arşiv başına mod dosyası
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = dosya
settings-unit-s = sn
settings-number-error = { $field }: { $min } ile { $max } arasında bir tam sayı girin. O zamana kadar önceki değer kullanılır.

# Network
settings-network-local = Yerel mod açık.
settings-network-local-text = deadloader ağ isteği yapmaz; bu ayarlar katalog açıldığında geçerli olur.
settings-requests-group = GameBanana’ya istekler
settings-api-parallel = Eş zamanlı katalog istekleri
settings-api-parallel-help = GameBanana API’sine aynı anda yapılan istekler. Görseller ve indirmeler ayrı sayılır.
settings-timeouts = Zaman aşımları
settings-timeouts-help = Bağlantı: bu sürede bağlantı kurulamazsa vazgeç. Yanıt: sunucu göndermeyi bırakınca vazgeç.
settings-timeouts-default = 10 sn · 30 sn
settings-timeout-connect = Bağlantı
settings-timeout-read = Yanıt
settings-retries = Yeniden denemeler
settings-retries-help = Başarısız bir isteğin, deadloader hata göstermeden önce artan aralıklarla kaç kez tekrarlanacağı.
settings-connection-group = Bağlantı
settings-proxy = Proxy
settings-proxy-help = Yok: doğrudan bağlantı. Sistem: Windows’ta ayarlı proxy. Elle: aşağıya girin.
settings-proxy-none = Yok
settings-proxy-system = Sistem
settings-proxy-manual = Elle
settings-proxy-host = Adres
settings-proxy-port = Bağlantı noktası
settings-proxy-hint = HTTP proxy. Yalnızca deadloader’ın kendi istekleri ondan geçer.
settings-proxy-invalid = Kullanıcı adı ve parola olmadan bir sunucu adı veya adres girin, ör. proxy.example.com.

# Cache
settings-thumbs-group = Küçük resimler ve katalog sayfaları
settings-cache-limit = Önbellek sınırı
settings-cache-limit-help = Önbellek dolduğunda, en uzun süredir görmediğiniz küçük resimler önce silinir.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = Geçerli boyut
settings-cache-breakdown = { $thumbnails } küçük resim · { $pages } katalog sayfası
settings-cache-empty = Boş — henüz önbellekte bir şey yok.
settings-cache-used = { $used } / { $limit }
settings-cache-clear = Önbelleği temizle
settings-cache-cleared = Önbellek temizlendi — { $size } boşaldı. Küçük resimler gerektiğinde yeniden yüklenir.
settings-cache-cleared-partly = { $failed } dosya kullanımdaydı ve bir sonraki temizliğe kadar kalır.

# Logs
settings-keep-group = Günlüklerin saklanması
settings-log-days = Günlükleri şu süre sakla
settings-log-days-help = Daha eski kayıtlar deadloader başlarken silinir.
settings-days = { $count ->
       *[other] { $count } gün
    }
settings-year = 1 yıl
settings-log-size = En büyük boyut
settings-log-size-help = Bu boyutun üstünde önce en eski günlük dosyaları silinir.
settings-privacy-group = Gizlilik
settings-mask = Windows kullanıcı adınızı gizle
settings-mask-help = Günlüklerdeki yollar adınız yerine { $masked } gösterir — bir günlüğü paylaşırken daha güvenli.
settings-logs-folder-group = Günlük klasörü
settings-logs-folder = Günlükler nerede
settings-logs-now = { $size } · { $files } dosya · en eskisi { $date }
settings-open-logs = Günlük klasörünü aç

# Advanced
settings-advanced-warning = Deneyimli kullanıcılar için.
settings-advanced-warning-text = Buradaki bir hata deadloader’ın davranışını değiştirebilir. Kaydetmeden önce dosyayı denetler ve önceki sürümün bir yedeğini saklar.
settings-files-group = Ayar dosyalarını düzenle
settings-file-settings = Ayarlar
settings-file-manifest = Oyun klasörü bildirimi
settings-file-label = Düzenlenecek dosya
settings-file-settings-lead = Bu sayfalardaki her şey tek bir dosyada. Değişiklikler kaydettikten sonra sayfalarda görünür.
settings-file-manifest-lead = deadloader’ın oyun klasörüne koyduklarının listesi. Geri alma buna dayanır — yanlış bir kayıt dosya bırakabilir.
settings-json-error = Bu geçerli bir JSON değil, bu yüzden kaydedilmedi.
settings-json-where = Satır { $line }, sütun { $column }: burada veya hemen öncesinde bir şey eksik ya da fazla — çoğu zaman bir virgül veya tırnak işareti.
settings-json-fix = O noktayı düzeltip yeniden kaydedin — o zamana kadar deadloader mevcut dosyayı kullanmaya devam eder.
settings-json-refused = Bu değerler kullanılamıyor, bu yüzden hiçbir şey kaydedilmedi.
settings-json-newer = Bu dosya daha yeni bir deadloader’dan (biçim { $found }; bu sürüm { $supported } sürümüne kadar okur), bu yüzden kaydedilmedi.
settings-json-invalid = deadloader bu dosyayı kullanamıyor, bu yüzden kaydedilmedi.
settings-json-details = Ayrıntılar
settings-problem-unknown-field = { $path }: bu sürümün bir ayarı değil.
settings-problem-invalid-value = { $path }: bu değer burada kullanılamaz.
settings-problem-out-of-range = { $path }: { $found }, { $min }–{ $max } aralığının dışında.
settings-problem-invalid-entry = { $path }: “{ $entry }” burada geçerli değil.
settings-problem-duplicate-entry = { $path }: “{ $entry }” iki kez yazılmış.
settings-problem-needs-age = { $path }: NSFW içerik göstermek için önce yaş onayı gerekir (Ayarlar → Katalog).
settings-problem-local-mode = { $path }: yerel modda kullanılamaz.
settings-json-idle = Kaydederken denetlenir.
settings-json-saved = Geçerli ve kaydedildi. Önceki sürüm { $backup } olarak saklandı.
settings-json-saved-plain = Geçerli ve kaydedildi.
settings-json-save = Denetle ve kaydet
settings-json-discard = Değişikliklerimi at
settings-json-backup = Kaydetmeden önce önceki dosya yedek olarak saklanır; son 10 tanesi kalır.
settings-domains-group = Ek izin verilen alan adları
settings-domains-lead = deadloader yalnızca GameBanana ile konuşur. Bir indirme başka bir sunucuya yönlendirilip engellenirse, o sunucuya buradan izin verebilirsiniz. Etkinlik’te ona yapılan istekler “kullanıcı ekledi” olarak işaretlenir.
settings-domains-built-in = Yerleşik:
settings-domain-user = kullanıcı ekledi
settings-domains-none = Ek alan adı yok.
settings-domain-add = Alan adı ekle
settings-domain-hint = Yalnızca sunucu adı, ör. files.example.com.
settings-domain-empty = Önce bir sunucu adı girin, ör. files.example.com.
settings-domain-not-host = Yalnızca sunucu adını girin — https:// ve yol olmadan, ör. files.example.com.
settings-domain-invalid = “{ $host }” bir sunucu adı değil. Yazımı kontrol edin, ör. files.example.com.
settings-domain-built-in = { $host } yerleşiktir ve her zaman izinlidir.
settings-domain-listed = { $host } zaten listede.

# About
settings-version = sürüm { $version }
settings-commit = Şu commit’ten derlendi
settings-commit-local = Yerel derleme — GitHub Actions tarafından derlenmedi
settings-copy-commit = Commit kimliğini kopyala
settings-release-page = Sürüm sayfası
settings-licence-group = Lisans
settings-licence-help = deadloader özgür bir yazılımdır: GPL-3 kapsamında kullanabilir, inceleyebilir, paylaşabilir ve değiştirebilirsiniz. Hiçbir garanti olmadan sunulur.
settings-read-licence = Lisansı oku
settings-source = Kaynak kodu
settings-verify-group = Bu indirmeyi nasıl doğrularsınız
settings-verify-lead = Bu deadloader.exe’nin GitHub’da yayımlanan dosya olduğuna dair iki bağımsız denetim.
settings-verify-hash = SHA-256 sağlama toplamını karşılaştırın
settings-verify-hash-help = Sürüm sayfasındaki sağlama toplamıyla eşleşmelidir. Değiştirilmiş bir kopya burada sahte bir değer gösterebilir, bu yüzden kendiniz de hesaplayın:
settings-verify-hash-cmd = PowerShell’de, exe’nin klasöründe:
settings-verify-attest = Derleme onayını doğrulayın
settings-verify-attest-help = Bu dosyayı GitHub Actions’ın yayımlanan kaynak koddan derlediğini kanıtlar. GitHub CLI gerekir:
settings-copy-hash = Sağlama toplamını kopyala
settings-copy-command = Komutu kopyala
settings-unsigned = deadloader.exe kod imzalı değil.
settings-unsigned-text = Windows SmartScreen ilk açılışta “Windows bilgisayarınızı korudu” diyebilir. Yukarıdaki iki denetim dosyanın gerçek olduğunu doğrulamanızı sağlar.
settings-webview-group = Pencere motoru
settings-webview-help = deadloader’ın penceresini Windows’un bir parçası olan WebView2 çizer. Windows onu kendisi günceller. deadloader şunları kapatır:
settings-webview-off-1 = DNS-over-HTTPS sorguları
settings-webview-off-2 = Pencere içinde Microsoft hesabıyla oturum açma
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Arka plan ağ trafiği ve bileşen güncellemeleri
settings-webview-off-5 = Form otomatik doldurma sunucu çağrıları
settings-webview-off-6 = Microsoft’a hata raporları (domain reliability)
settings-webview-no-route = Ve pencerenin internete hiçbir yolu yok.
settings-webview-unknown = sürüm bilinmiyor
settings-data-group = Verileriniz
settings-data-folder = Veri klasörü
settings-data-folder-help = deadloader’ın sakladığı her şey, exe’nin yanındaki bu tek klasördedir.
settings-notice = İlk açılış bildirimi
settings-notice-help = Sürüm { $version }, { $date } tarihinde { $language } dilinde kabul edildi. Seçimleriniz: katalog { $catalog }, güncelleme denetimi { $updates }.
settings-notice-none = Henüz kabul edilmedi.
settings-notice-again = Yeniden oku

# D7
settings-d7-title = { $drive } üzerindeki oyun klasörü geri alınsın mı?
settings-d7-from =
    { $others ->
        [0] Seçili profil { $profile } kurulumu. deadloader şuradan kaldırır:
       *[other] Seçili profil { $profile } kurulumu ({ $others } de kullanıyor). deadloader şuradan kaldırır:
    }
settings-d7-files =
    { $count ->
       *[other] Eklediği { $count } mod dosyası, konum:
    }
settings-d7-line = gameinfo.gi’deki satırları
settings-d7-line-why = — dosya Steam’in kurduğu hâline döner.
settings-d7-stays = Olduğu gibi kalır:
settings-d7-library = Kitaplığınız — { $mods } mod, { $size }
settings-d7-profiles =
    { $count ->
        [one] Profiliniz — { $names }
       *[other] { $count } profiliniz — { $names }
    }
settings-d7-other = { $path } üzerindeki kurulum ({ $profile }) — dokunulmaz. Onu geri almak için onu kullanan bir profil seçip yeniden geri alın.
settings-d7-other-plain = { $path } üzerindeki kurulum — dokunulmaz.
settings-d7-back = Uygula veya Oyna her şeyi geri getirir.
settings-d7-foreign =
    { $count ->
       *[other] deadloader’ın oluşturmadığı { $count } dosyayı da kaldır
    }
settings-d7-foreign-why = başka bir programdan veya elle kurulumdan. Kalıcı olarak silinirler; deadloader onları geri getiremez.
settings-d7-confirm = Geri al
settings-d7-confirm-foreign =
    { $count ->
       *[other] Geri al ve { $count } dosyayı sil
    }

# D8
settings-d8-title = Tüm deadloader verileri silinsin mi?
settings-d8-from = deadloader şunun içindeki her şeyi siler:
settings-d8-from-2 = ve sonra kapanır.
settings-d8-what = Ne
settings-d8-contains = İçerik
settings-d8-size = Boyut
settings-d8-total = Toplam
settings-d8-library = Kitaplık
settings-d8-library-has = { $mods } mod, { $files } dosya
settings-d8-profiles = Profiller
settings-d8-logs = Günlükler
settings-d8-logs-has = { $files } dosya, { $date } tarihinden beri
settings-d8-logs-none = günlük dosyası yok
settings-d8-cache = Önbellek
settings-d8-cache-has = küçük resimler ve katalog sayfaları
settings-d8-settings = Ayarlar
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
       *[other] { $backups } yedek dahil
    }
settings-d8-other = Diğer
settings-d8-other-has = yarım kalan indirmeler, gameinfo.gi yedekleri, pencere motorunun verileri
settings-d8-exe = Silinmez: deadloader.exe’nin kendisi — artık gerekmiyorsa kendiniz silin.
settings-d8-rollback = Önce deadloader’ın kullandığı her oyun klasörünü geri al (önerilir)
settings-d8-rollback-why = Silmeden önce deadloader’ın mod dosyalarını ve gameinfo.gi’deki satırlarını her oyun klasöründen kaldırır. Yabancı dosyalar kalır.
settings-d8-rollback-off = İşaretli değil: deadloader’ın mod dosyaları oyun klasörlerinde kalır ve Deadlock onları yüklemeye devam eder — onları kaldıracak bir deadloader kalmadan.
settings-d8-type-1 = Onaylamak için
settings-d8-type-2 = yazın
settings-d8-word = sil
settings-d8-hint = Kelime eşleştiğinde silme düğmesi açılır.
settings-d8-hint-ok = Onaylandı. Bu geri alınamaz.
settings-d8-type-first = Önce “{ $word }” yazın
settings-d8-deleting = Siliniyor…
settings-d8-confirm = Sil ve kapat
settings-d8-confirm-rollback = Geri al, sil ve kapat

# Reset
settings-reset-title = Tüm ayarlar sıfırlansın mı?
settings-reset-body = Her ayar, izin verilen görsel sunucuları ve ek alan adları dahil varsayılanına döner.
settings-reset-keeps = Kitaplığınıza, profillerinize ve günlüklerinize dokunulmaz; oyun klasörü de olduğu gibi kalır.
settings-reset-backup = Geçerli ayarların bir yedeği settings.json.bak-… olarak saklanır.
settings-reset-confirm = Ayarları sıfırla
settings-reset-done = Tüm ayarlar varsayılanlarına sıfırlandı.
settings-reset-done-text = Yedek: { $backup }
