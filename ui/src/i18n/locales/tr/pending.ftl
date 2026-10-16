# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Kapat
library-apply = Uygula
library-apply-profile = { $name } profilini uygula
library-discard-all = Tümünü at
library-keep = { $name } profilini koru
library-keep-applied = Uygulanan değişiklikleri koru
library-undo-this = Bu değişikliği geri al
library-undo-all = Tümünü geri al
library-close-to-apply = Deadlock’u kapatın — değişiklikler o zaman otomatik uygulanır
library-play-applies = Oyna da uygular
library-applied-title = Değişiklikler uygulandı
library-nothing-pending = Bekleyen bir şey yok
library-all-in-game = { $name } profilindeki her şey zaten oyunda.
library-running-sub = Deadlock çalışıyor, bu yüzden Windows dosyalarını kilitliyor. Bu değişiklikler oyunu kapatır kapatmaz otomatik olarak uygulanacak.
library-waiting-sub = Bu değişiklikler henüz oyun klasöründe değil. Şimdi uygulayın veya Oyna’ya basın.
library-switch-sub = Profil geçişi aynı anda birçok dosyayı değiştirir, bu yüzden sizi bekler. Burada uygulayın veya Oyna’ya basın — deadloader başlatmadan önce uygular.
library-applied-sub = Deadlock { $time } saatinde kapatıldı; oyun açıkken yaptığınız { $count } değişiklik uygulandı. Bir sonraki değişikliğinize kadar geri alabilirsiniz.
library-undoing-sub = { $time } saatinde uygulanan { $count } değişikliği geri aldınız ve Deadlock yeniden çalışıyor. Oyunu kapattığınızda otomatik olarak geri alınacaklar.
library-group-enable = Etkinleştir · { $count }
library-group-disable = Devre dışı bırak · { $count }
library-group-order = Sıra · { $count }
library-group-variant = Varyant · { $count }
library-group-version = Sürüm · { $count }
library-group-switch = Profil geçişi · { $count }
library-group-undo = { $time } saatinde uygulanan { $count } değişikliğin geri alınması
library-change-on = Kapalı → açık
library-change-off = Açık → kapalı
library-change-above = { $name } üstüne taşındı
library-change-above-wins = { $name } üstüne taşındı — artık aralarındaki çakışmayı kazanıyor
library-change-below = { $name } altına taşındı
library-switch-what-happens = { $on } mod açılır · { $off } mod kapanır
library-what-happens = Oyun klasöründe ne olacak
library-folder-summary = { $added } dosya eklenir · { $removed } kaldırılır · { $renumbered } yeniden numaralanır

## Foreign files
library-foreign-title = Oyunun mod klasöründe bulundu
library-foreign-note = Bunları deadloader koymadı ve siz istemedikçe dokunmayacak.
library-collapse = Daralt
library-expand = Genişlet
library-take = Yönetime al
library-leave = Olduğu gibi bırak
library-left = Olduğu gibi bırakıldı
library-foreign-open = { $name } içeren klasörü aç
library-foreign-vpk = VPK, { $count } dosya
library-foreign-vpk-hero = VPK, { $count } dosya — { $hero } için bir kaplama gibi görünüyor
library-foreign-unreadable = deadloader’ın okuyamadığı bir mod dosyası
library-foreign-replaced = deadloader’ın bir dosyasının yerini alan bir dosya
library-foreign-dmm = Deadlock Mod Manager tarafından oluşturuldu — bir ayar dosyası, mod değil
library-foreign-other = Bir dosya, mod değil
library-foreign-folder = Klasör, { $count } dosya — mod değil
library-foreign-meta = { $size } · { $date } tarihinde eklendi
library-taken = { $name } artık kitaplığınızda.

## D11
library-d11-title = { $name } yönetime alınsın mı?
library-d11-lead = deadloader şunları yapacak:
library-d11-step-copy = dosyayı kitaplığınıza yerel mod olarak kopyalayacak ({ $size }),
library-d11-step-replace = oyun klasöründeki dosyayı kendi bağlı kopyasıyla değiştirecek,
library-d11-step-enable = { $profile } profilinde listenin sonunda etkinleştirecek.
library-d11-name = Kitaplıktaki adı
library-d11-hint = Kitaplık özgün dosyanın birebir kopyasını saklar.
library-d11-suggested = { $file } ({ $hero } kaplaması)

## D10
library-d10-title = Bir oyun dosyası başka bir program tarafından değiştirildi
library-d10-what = Başka bir program, Deadlock’un hangi klasörleri yükleyeceğini belirleyen bir oyun dosyasını değiştirdi — çoğu zaman Deadlock Mod Manager. deadloader başkalarının değişikliklerinin üzerine yazmaz; bu yüzden düzeltilene kadar modlarınız yüklenmez.
library-d10-do = Steam’in özgün dosyayı geri yüklemesine izin verin (Oyun dosyalarını doğrula). Ardından deadloader kendi satırlarını ekler ve size haber verir.
library-d10-diff = Farkları göster
library-d10-expected = Beklenen (özgün + deadloader satırları)
library-d10-found = Şu an bulunan
library-d10-diff-note = Vurgulananlar: deadloader’ın eklediği satırlar ve başka bir programın eklediği satırlar.
library-d10-help = Bu dosya nedir?
library-d10-help-text = gameinfo.gi, Deadlock’a hangi klasörleri yükleyeceğini söyler. Mod yöneticileri, oyunun addons klasöründen mod yüklemesi için birkaç satır ekler. Oyun güncellemeleri dosyayı sıfırlar; deadloader bunu otomatik düzeltir — ama yalnızca dosya özgün hâlindeyse.
library-d10-wait-title = Steam, Deadlock’un dosyalarını doğruluyor.
library-d10-wait-text = Steam bitirince deadloader’a dönün — dosyayı otomatik olarak yeniden denetler. Şimdi de denetleyebilirsiniz.
library-d10-fixed = gameinfo.gi ayarlandı — modlar yeniden yükleniyor.
library-d10-fixed-text = Steam özgün dosyayı geri yükledi ve deadloader kendi satırlarını ekledi. Ayrıntılar Etkinlik → Dosyalar bölümünde.
library-open-file = Dosyayı aç
library-not-now = Şimdi değil
library-verify-steam = Oyun dosyalarını Steam’de doğrula
library-check-again = Yeniden denetle
library-show-differences = Farkları göster

## D5
library-update = Güncelle
library-update-on-gb = GameBanana’da daha yeni bir sürüm var.
library-later = Sonra
library-d5-title = { $name } güncellensin mi?
library-d5-installed = Yüklü
library-d5-new = GameBanana’da yeni
library-d5-downloaded = İndirildi
library-d5-arrived = Kitaplıkta, henüz etkin değil
library-d5-new-meta = { $date } tarihinde güncellendi · { $size } indirilecek
library-d5-download = { $size } indirilecek
library-d5-changed = Yeni sürümün dosya seti farklı. deadloader bunları şöyle eşleştirdi:
library-d5-same = Bu dosyalar yeni sürümleriyle değiştirilecek:
library-d5-map-text = deadloader her dosya için hangi yeni dosyanın onun yerini aldığını belirleyemedi. Her biri için yeni dosyayı seçin veya eskisini koruyun.
library-d5-map-label = Dosya eşleştirmesi
library-d5-col-old = Yüklü dosya
library-d5-col-new = Yeni dosya
library-d5-pick = { $file } için yeni dosya
library-d5-choose = Seçin…
library-d5-keep = Eski dosyayı koru
library-d5-tag-replaces = Yerini alır
library-d5-tag-same = Aynı ad
library-d5-tag-new = Yeni · mod sayfasında
library-d5-profiles = { $new }, { $old } kullanan her profilde etkin olur. { $old } kitaplıkta kalır; mod ayrıntılarından geri dönebilirsiniz.
library-d5-install = Güncellemeyi yükle
library-d5-confirm-map = Bu dosyaları kullan
library-update-downloading = { $name } güncellemesi indiriliyor — geldiğinde yüklenecek.
library-updated = { $name } güncellendi.
library-updated-text = Eski sürümü kullanan her profilde etkin. Eski sürüm saklanıyor — mod ayrıntılarından ona dönebilirsiniz.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name } açık
library-n12-off = { $name } kapalı
library-n12-above = { $name }, { $other } üstüne taşındı
library-n12-below = { $name }, { $other } altına taşındı
library-n12-switched = { $name }: { $to } seçildi
library-n12-files = { $name }: dosyalar değişti

## S9: the file list of a pending apply
library-show-file-list = Dosya listesini göster
library-hide-file-list = Dosya listesini gizle
library-file-list = Oyun klasöründeki dosyalar
library-files-added = Eklenir · { $count }
library-files-removed = Kaldırılır · { $count }
library-files-renumbered = Yeniden numaralanır · { $count }
