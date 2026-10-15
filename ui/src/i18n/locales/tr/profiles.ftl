# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Profiller
profiles-count =
    { $count ->
        [one] { $count } profil
       *[other] { $count } profil
    }
profiles-in-game = Şu an oyunda: { $list }
profiles-in-game-item = { $drive } üzerinde “{ $name }”
profiles-in-game-none = Şu an oyunda hiçbir profil yok.
profiles-import = İçe aktar…
profiles-import-title = .json dosyasından profil içe aktar
profiles-import-dialog = Profil içe aktar
profiles-export-dialog = Profili dışa aktar
profiles-file-filter = deadloader profilleri
profiles-new = Yeni profil
profiles-selected = Seçili
profiles-applied = Uygulandı
profiles-not-applied = Henüz uygulanmadı
profiles-selected-title = Kütüphanede düzenlediğiniz ve oynadığınız profil
profiles-pending-title = Uygula veya Oyna onu oyun klasörüne koyar
profiles-applied-title = Dosyaları şu an { $drive } üzerindeki oyun klasöründe
profiles-select = Seç
profiles-select-of = “{ $name }” profilini seç
profiles-more-of = “{ $name }” için diğer işlemler
profiles-rename = Yeniden adlandır
profiles-duplicate = Çoğalt
profiles-export = Dosya olarak dışa aktar…
profiles-change-installation = Oyun kurulumunu değiştir…
profiles-delete = Sil
profiles-delete-confirm = Sil…
profiles-delete-last = deadloader en az bir profile ihtiyaç duyar.
profiles-name-label = Profil adı
profiles-save = Kaydet
profiles-cancel = İptal
profiles-close = Kapat
profiles-close-esc = Kapat (Esc)
profiles-rename-hint = Enter kaydeder · Esc iptal eder
profiles-name-empty = Profil için bir ad girin.
profiles-mods =
    { $count ->
        [one] { $count } mod etkin
       *[other] { $count } mod etkin
    }
profiles-mods-none = Etkin mod yok
profiles-applied-when = { $when } uygulandı
profiles-last-applied = En son { $when } uygulandı
profiles-never-applied = Hiç uygulanmadı
profiles-just-now = az önce
profiles-preview-empty = Bu profili seçin, ardından kütüphanede modları etkinleştirin.
profiles-footnote = Profil, etkin modların sırası, varyantları ve sürümleriyle birlikte bir listesidir. Seçmek hiçbir şey indirmez veya silmez; Uygula yalnızca farklı olan dosyaları değiştirir.
profiles-copy-name = { $name } (kopya)
profiles-copy-name-n = { $name } (kopya { $number })
profiles-new-title = Yeni profil
profiles-start-with = Başlangıç
profiles-empty = Boş
profiles-empty-hint = Etkin mod yok — onları sonra kütüphanede seçin.
profiles-copy-of = “{ $name }” kopyası
profiles-copy-hint = { $mods } — aynı sıra, varyantlar ve sürümler.
profiles-installation = Oyun kurulumu
profiles-installation-hint = “{ $name }” ile aynı. Daha sonra profilin menüsünden değiştirebilirsiniz.
profiles-new-note = Profil oluşturmak onu seçmez veya uygulamaz — oyun değişmez.
profiles-create = Profil oluştur
profiles-pending-what = “{ $name }” profili henüz oyunda değil.
profiles-pending-why-other = { $drive } üzerindeki oyun klasöründe hâlâ “{ $other }” var. Şimdi uygulayın veya Oyna’ya basın — deadloader başlatmadan önce uygular.
profiles-pending-why-empty = { $drive } üzerindeki oyun klasöründe şu an deadloader modu yok. Şimdi uygulayın veya Oyna’ya basın — deadloader başlatmadan önce uygular.
profiles-keep = “{ $name }” kalsın
profiles-apply = “{ $name }” uygula
profiles-other-what = “{ $name }” başka bir oyun kurulumu kullanıyor.
profiles-other-why = Uygula ve Oyna { $path } kullanır. { $drive } üzerindeki oyun “{ $other }” ile kalır.
profiles-other-why-empty = Uygula ve Oyna { $path } kullanır. { $drive } üzerindeki oyun değişmez.
profiles-delete-title = “{ $name }” profili silinsin mi?
profiles-delete-applied =
    { $count ->
        [one] “{ $name }” şu an oyunda. deadloader { $count } modunu oyun klasöründen kaldıracak:
       *[other] “{ $name }” şu an oyunda. deadloader { $count } modunu oyun klasöründen kaldıracak:
    }
profiles-delete-applied-empty = “{ $name }” şu an oyunda ama modu yok, bu yüzden oyun klasörü değişmez.
profiles-delete-stay = Modlar kütüphanenizde ve diğer profillerde kalır. Orada başka bir profil uygulayana kadar Deadlock, deadloader modları olmadan başlar.
profiles-delete-next = “{ $name }” seçili profil olur. Uygula veya Oyna’ya basana kadar uygulanmaz.
profiles-delete-undoable = Bu geri alınamaz. Bir kopya tutmak için önce profili dışa aktarın.
profiles-export-first = Önce dışa aktar
profiles-delete-profile = Profili sil
profiles-installation-title = “{ $name }” için oyun kurulumu
profiles-installation-lead = Bu profilin uygulandığı Deadlock klasörünü seçin.
profiles-used-by =
    { $count ->
        [one] { $count } profil kullanıyor
       *[other] { $count } profil kullanıyor
    }
profiles-used-by-none = hiçbir profil kullanmıyor
profiles-installation-applied = “{ $name }” şu an { $drive } üzerinde oyunda. Dosyaları, { $drive } üzerinde başka bir profil uygulayana kadar orada kalır.
profiles-installation-not-applied = “{ $name }” uygulanana kadar hiçbir oyun klasörü değişmez.
profiles-manage-installations = Kurulumları Ayarlar’da yönet
profiles-use-installation = Bu kurulumu kullan
profiles-d9-title =
    { $count ->
        [one] “{ $name }” içindeki { $count } mod kütüphanenizde yok
       *[other] “{ $name }” içindeki { $count } mod kütüphanenizde yok
    }
profiles-d9-file = Dosya
profiles-d9-lead = { $listed } mod listeliyor. { $present } tanesi zaten kütüphanenizde; şu { $count } tanesi değil:
profiles-d9-list = Eksik modlar
profiles-d9-from-gamebanana = GameBanana’dan { $count }
profiles-d9-local-count = { $count } yerel
profiles-gamebanana = GameBanana
profiles-can-download = İndirilebilir
profiles-local-mod = Yerel mod
profiles-cant-download = İndirilemez
profiles-local-why = Başka bir bilgisayarda diskten eklenmiş, bu yüzden GameBanana’da yok. Dosyayı profili paylaşan kişiden isteyin.
profiles-d9-local-mode = Katalog kapalı (yerel mod), bu yüzden deadloader mod indiremez. Onlar olmadan içe aktarın ve sonra diskten ekleyin — veya kataloğu Ayarlar’da açın.
profiles-d9-after = Her iki durumda da “{ $name }” seçili profiliniz olur ama uygulanmaz — oyun klasörü ancak Uygula veya Oyna’ya bastığınızda değişir.
profiles-import-without = Onlarsız içe aktar
profiles-download-missing = Eksikleri indir ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } mod kapatıldı
       *[other] { $count } mod kapatıldı
    }
profiles-turned-on =
    { $count ->
        [one] { $count } mod açıldı
       *[other] { $count } mod açıldı
    }
profiles-differ = Yalnızca farklı olan dosyalar değiştirildi.
profiles-same = Aynı modlar zaten oyundaydı; yalnızca sıra ve varyantlar denetlendi.
profiles-deleted = “{ $name }” profili silindi.
profiles-deleted-not-applied = Uygulanmamıştı, bu yüzden oyun klasörü değişmedi.
profiles-deleted-applied =
    { $count ->
        [one] { $count } mod oyun klasöründen kaldırıldı. Hâlâ kütüphanenizde.
       *[other] { $count } mod oyun klasöründen kaldırıldı. Hâlâ kütüphanenizdeler.
    }
profiles-exported = “{ $name }” profili dışa aktarıldı.
profiles-imported = “{ $name }” profili içe aktarıldı.
profiles-imported-text = Seçili ama henüz uygulanmadı.
profiles-imported-without =
    { $count ->
        [one] “{ $name }” profili { $count } mod olmadan içe aktarıldı.
       *[other] “{ $name }” profili { $count } mod olmadan içe aktarıldı.
    }
profiles-imported-without-text = Seçili ama henüz uygulanmadı. Eksik modları daha sonra ekleyebilirsiniz.
profiles-imported-downloading =
    { $count ->
        [one] “{ $name }” profili içe aktarıldı — { $count } mod indiriliyor.
       *[other] “{ $name }” profili içe aktarıldı — { $count } mod indiriliyor.
    }
profiles-imported-downloading-text = Seçili ama henüz uygulanmadı. İndirmeler bitince uygulayın.
profiles-open-downloads = İndirmeleri aç
profiles-undo = Geri al
profiles-check-update = deadloader güncellemesini denetle
profiles-choose-another = Başka dosya seç
profiles-switcher = Profilleri yönet
profiles-saved-as = Kaydedildiği yer:
profiles-show-folder = Klasörde göster
