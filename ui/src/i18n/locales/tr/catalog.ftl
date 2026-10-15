# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Bu bağlantı tarayıcınızda açılsın mı?
catalog-link-body = Bağlantı deadloader’dan çıkar ve varsayılan tarayıcınızda açılır.
catalog-link-always = Bağlantıları her zaman hemen aç
catalog-link-cancel = İptal
catalog-link-open = Aç

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Gizli
catalog-nsfw-blurred = Bulanık
catalog-nsfw-shown = Görünür
catalog-age-title = Hassas içerikli modlar gösterilsin mi?
catalog-age-body = GameBanana’daki bazı modlar çıplaklık, cinsel temalar veya benzeri nedenlerle derecelendirilmiştir. Bunları görmek için 18 yaşında veya daha büyük olduğunuzu ya da yaşadığınız yerde reşit olduğunuzu onaylayın.
catalog-age-choice = Seçiminiz: { $mode }.
catalog-age-once = deadloader bunu yalnızca bir kez sorar. İstediğiniz zaman “Gizli” seçeneğine dönebilirsiniz.
catalog-age-confirm = Reşit olduğumu onaylıyorum
catalog-age-cancel = İptal

## S2: the catalog grid
catalog-title = Katalog
catalog-section = Bölüm
catalog-section-mods = Modlar
catalog-section-sounds = Sesler
catalog-search = Mod ara
catalog-search-sounds = Ses ara
catalog-search-offline = Arama için bağlantı gerekir
catalog-search-off = Arama kullanılamıyor — katalog okunamıyor
catalog-search-clear = Aramayı temizle
catalog-results =
    { $count ->
        [one] { $count } sonuç
       *[other] { $count } sonuç
    }
catalog-sort = Sırala
catalog-sort-newest = En yeni
catalog-sort-latest_updated = Son güncellenen
catalog-sort-most_downloaded = En çok indirilen
catalog-sort-most_liked = En çok beğenilen
catalog-sort-most_viewed = En çok görüntülenen
catalog-sort-most_commented = En çok yorum alan
catalog-sort-oldest = En eski
catalog-sort-best = En iyi eşleşme
catalog-sort-best-title = Arama sırasında sonuçlar eşleşmeye göre sıralanır
catalog-category = Kategori
catalog-category-all = Tüm kategoriler
catalog-hero = Kahraman
catalog-hero-all = Tüm kahramanlar
catalog-hero-filter = Kahraman bul
catalog-hero-none = Eşleşen kahraman yok
catalog-filters = Filtreler
catalog-filter-nsfw = İçerik derecelendirmesi olan modlar (NSFW)
catalog-filter-favourites = Yalnızca favoriler
catalog-filter-favourites-short = yalnızca favoriler
catalog-filter-installed = Kurulu olanları gizle
catalog-filter-installed-short = kurulular gizli
catalog-nsfw-caption-hidden = NSFW gizli
catalog-nsfw-caption-blurred = NSFW bulanık · göstermek için karta tıklayın
catalog-nsfw-caption-shown = NSFW görünür
catalog-loading = Modlar GameBanana’dan yükleniyor…
catalog-loading-sounds = Sesler GameBanana’dan yükleniyor…
catalog-loading-more = Daha fazlası yükleniyor…
catalog-more-failed = Sonraki sayfa yüklenemedi.
catalog-grid = Modlar
catalog-grid-sounds = Sesler
catalog-likes =
    { $count ->
        [one] { $count } beğeni
       *[other] { $count } beğeni
    }
catalog-favourite-add = Favorilere ekle
catalog-favourite-remove = Favorilerden çıkar
catalog-sensitive = Hassas içerik
catalog-click-show = Göstermek için tıklayın
catalog-badge-installed = Kurulu
catalog-badge-update = Güncelleme
catalog-card-installing = Kuruluyor · %{ $percent }
catalog-empty = Burada henüz mod yok
catalog-empty-search = “{ $text }” ile eşleşen mod yok
catalog-empty-favourites = Henüz favori yok
catalog-empty-filters = Açık filtreler: { $filters }. Tüm modlarda aramak için bunları kaldırın ya da daha kısa bir kelime deneyin.
catalog-empty-shorter = Daha kısa ya da farklı bir kelime deneyin.
catalog-clear-filters = Filtreleri temizle
catalog-rate-retry = deadloader { $seconds } sn içinde otomatik olarak yeniden dener — aşağıdaki modlar kullanılabilir durumda.
catalog-retry-now = Şimdi yeniden dene
catalog-try-again = Yeniden dene
catalog-check-update = deadloader güncellemesini denetle
catalog-details = Ayrıntılar · günlük kaydı
catalog-copy-id = Kayıt kimliğini kopyala
catalog-open-activity = Etkinlik’te aç
catalog-offline-empty-title = Çevrimdışı — gösterilecek kayıt yok
catalog-offline-empty-text = deadloader GameBanana’ya ulaşamıyor ve kataloğun bu bölümü daha önce kaydedilmedi.
catalog-error-title = Katalog şu anda gösterilemiyor.
catalog-local-title = Katalog kapalı (yerel mod)
catalog-local-why = Yerel modda deadloader hiçbir ağ isteği yapmaz. Kitaplığınız, profilleriniz ve Oyna her zamanki gibi çalışır; diskten mod ekleyebilirsiniz.
catalog-local-do = GameBanana’daki modlara göz atmak ve kurmak için kataloğu açın. deadloader o zaman yalnızca GameBanana ile konuşur.
catalog-local-enable = Kataloğu aç
catalog-local-library = Kitaplığa git

## S3: the mod page
catalog-breadcrumb = Konum
catalog-back = Kataloğa dön
catalog-page-loading = Mod sayfası GameBanana’dan yükleniyor…
catalog-page-offline = Çevrimdışı — { $date } tarihinde kaydedilen kopya gösteriliyor.
catalog-page-offline-why = Yeni dosya kurmak için bağlantı gerekir; indirilmiş varyantlar arasında geçiş yapmak yine çalışır.
catalog-page-offline-empty-title = Çevrimdışı — bu mod sayfası kaydedilmedi
catalog-page-offline-empty-text = deadloader GameBanana’ya ulaşamıyor ve bu sayfa daha önce hiç açılmadı.
catalog-page-local-title = Bu mod sayfası kaydedilmedi
catalog-page-local-text = Yerel modda deadloader yalnızca daha önce kaydedilen mod sayfalarını gösterir.
catalog-removed-title = Bu mod GameBanana’dan kaldırıldı
catalog-removed-why = Yayından alındı ya da silindi; bu yüzden deadloader sayfasını gösteremez ve dosyalarını indiremez.
catalog-removed-keep = Kurulu kopyanız ({ $copy }) kitaplığınızda kalır ve çalışmaya devam eder — yalnızca artık güncelleme almaz.
catalog-show-in-library = Kitaplıkta göster
catalog-installed = Kurulu
catalog-by = yapan: { $author }
catalog-version = Sürüm { $version }
catalog-views =
    { $count ->
        [one] { $count } görüntülenme
       *[other] { $count } görüntülenme
    }
catalog-downloads =
    { $count ->
        [one] { $count } indirme
       *[other] { $count } indirme
    }
catalog-added = Eklendi: { $when }
catalog-updated = Güncellendi: { $when }
catalog-favourite = Favori
catalog-open-gamebanana = GameBanana’da aç
catalog-gallery-alt = { $name }, galeri görseli { $index }
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Önceki görsel
catalog-gallery-next = Sonraki görsel
catalog-gallery-original = Orijinali görüntüle
catalog-gallery-hidden = İçerik derecelendirmesi olan modların görselleri gizli.
catalog-audio-preview = Bu sesin GameBanana’da bir önizlemesi var.
catalog-description = Açıklama
catalog-description-more = Açıklamanın tamamını göster
catalog-description-less = Daha az göster
catalog-description-truncated = Bu açıklamanın yalnızca başı gösteriliyor.
catalog-opens-browser = tarayıcınızda açılır
catalog-opens-here = deadloader’da açılır
catalog-image-from = Görselin kaynağı:
catalog-image-click = yüklemek için tıklayın
catalog-image-failed = yüklenemedi
catalog-image-alt = Açıklamadaki görsel
catalog-youtube = YouTube videosu
catalog-spoiler = Spoiler — göstermek için tıklayın
catalog-spoiler-hide = Spoiler’ı gizle
catalog-files = Dosyalar
catalog-files-clean =
    { $count ->
        [one] GameBanana’nın virüs taramasından geçti
       *[other] { $count } dosyanın tümü GameBanana’nın virüs taramasından geçti
    }
catalog-files-clean-dated =
    { $count ->
        [one] GameBanana’nın virüs taramasından geçti · yükleme: { $date }
       *[other] { $count } dosyanın tümü GameBanana’nın virüs taramasından geçti · yükleme: { $date }
    }
catalog-files-av = GameBanana’nın virüs taraması sonucu her dosyanın yanında gösterilir.
catalog-variants-note = Bu dosyalar aynı oyun dosyalarını değiştirir, yani varyantlardır — aynı anda yalnızca biri etkindir.
catalog-no-files = Bu modun indirilecek dosyası yok.
catalog-older-files = Eski dosyalar ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } indirme
       *[other] { $count } indirme
    }
catalog-file-active = etkin varyant
catalog-file-downloaded = indirildi
catalog-file-install = Kur
catalog-file-install-label = { $file } dosyasını kur ve etkin varyant yap
catalog-file-install-title = Bu dosyayı indirir ve etkin varyant yapar
catalog-file-install-offline = Kurmak için bağlantı gerekir
catalog-file-installing = Kuruluyor…
catalog-file-installed = Kurulu
catalog-file-switch = Geçiş yap
catalog-file-switch-label = { $file } dosyasına geç
catalog-file-switch-title = Zaten indirildi — etkin varyant yap
catalog-file-update = Güncelle
catalog-file-update-title = Daha yeni bir sürüm kurulu dosyanızın yerini alıyor — kitaplıkta inceleyin
catalog-av-clean = Temiz
catalog-av-clean-title = GameBanana virüs taraması: temiz
catalog-av-flagged = İşaretlendi
catalog-av-flagged-title = GameBanana virüs taraması: { $verdict }
catalog-av-pending = Henüz taranmadı
catalog-av-pending-title = GameBanana bu dosyanın taramasını henüz bitirmedi
catalog-av-none = Tarama yok
catalog-av-none-title = GameBanana bu dosya için virüs taraması sonucu göndermedi
catalog-credits = Katkıda bulunanlar
catalog-no-ratings = İçerik derecelendirmesi yok
catalog-no-licence = Lisans metni yok
catalog-no-sources = Alternatif kaynak yok
catalog-rating-sexual_themes = Cinsel temalar
catalog-rating-sexual_content = Cinsel içerik
catalog-rating-skimpy_attire = Açık kıyafet
catalog-rating-lewd_angles_and_poses = Müstehcen açılar ve pozlar
catalog-rating-full_nudity = Tam çıplaklık
catalog-licence = Lisans
catalog-sources = Alternatif kaynaklar
catalog-videos = Videolar

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Yeni bir sunucudan görsel yüklensin mi?
catalog-image-body-before = deadloader bu görseli şuradan indirecek:
catalog-image-body-after = . Bu sunucu GameBanana değil — istek diğerleri gibi Etkinlik’te görünür.
catalog-image-note = Görsellerin boyutu sınırlıdır. İzin verilen sunucuları Ayarlar → Katalog’dan kaldırabilirsiniz.
catalog-image-always = Bu sunucudan görsellere her zaman izin ver
catalog-image-cancel = İptal
catalog-image-load = Görseli yükle
