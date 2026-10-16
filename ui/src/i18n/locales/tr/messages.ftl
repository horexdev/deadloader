## Notifications

n1 = Deadlock güncellendi — modlar geri yüklendi.
    .why = Güncelleme gameinfo.gi dosyasını sıfırladı, bu yüzden deadloader kendi satırlarını yeniden ekledi.
n2 = { $name } yüklendi ve etkinleştirildi.
n3 =
    { $count ->
        [one] { $count } mod güncellemesi mevcut.
       *[other] { $count } mod güncellemesi mevcut.
    }
n4 = Deadlock çalışıyor.
    .why = Oyunu kapattığınızda değişiklikler otomatik olarak uygulanacak.
    .pending =
        { $count ->
            [one] { $count } değişiklik bekliyor
           *[other] { $count } değişiklik bekliyor
        }
n5 =
    { $reason ->
        [completed] Yarıda kalan bir değişiklik tamamlandı.
       *[undone] Yarıda kalan bir değişiklik geri alındı; oyun klasörünüz eskisi gibi.
    }
n6 =
    { $count ->
        [one] Oyunun mod klasöründe deadloader’ın oluşturmadığı { $count } dosya bulundu.
       *[other] Oyunun mod klasöründe deadloader’ın oluşturmadığı { $count } dosya bulundu.
    }
n7 =
    { $when ->
        [today] Çevrimdışı — saat { $saved } itibarıyla kaydedilmiş katalog gösteriliyor.
       *[earlier] Çevrimdışı — { $saved } tarihinde kaydedilmiş katalog gösteriliyor.
    }
    .why = Yeniden çevrimiçi olduğunuzda indirmeler devam edecek.
n8 =
    { $count ->
        [one] “{ $name }” profili uygulandı — { $count } mod etkinleştirildi.
       *[other] “{ $name }” profili uygulandı — { $count } mod etkinleştirildi.
    }
n9 = Yeni bir deadloader sürümü mevcut: { $version }.
n10 = Yerel çalışma modu açık.
    .why = Katalog kapalı ve deadloader hiçbir ağ isteği göndermiyor.
n11 = deadloader ve oyun farklı sürücülerde olduğu için modlar bağlanmak yerine kopyalanıyor.
    .why = Bu, diskte fazladan { $size } yer kaplıyor.
n12 =
    { $count ->
        [one] Deadlock kapatıldı — { $count } değişiklik uygulandı.
       *[other] Deadlock kapatıldı — { $count } değişiklik uygulandı.
    }

## Errors: what happened, why, what to do

e1 = deadloader verilerini kaydedemiyor.
    .why =
        { $reason ->
            [access_denied] deadloader.exe’nin bulunduğu klasör salt okunur — örneğin Windows, programların Program Files klasörüne yazmasına izin vermez.
            [in_use] Başka bir program, çoğunlukla bir virüsten koruma yazılımı, deadloader’ın dosyalarından birini açık tutuyor.
           *[other] Windows, deadloader.exe’nin bulunduğu klasöre yazamadı.
        }
    .todo =
        { $reason ->
            [in_use] Diğer programları kapatıp yeniden deneyin.
           *[other] deadloader’ı yazma izniniz olan bir klasöre taşıyın, örneğin D:\Games\deadloader.
        }
e2 = Deadlock bulunamadı.
    .why = Steam veya oyun yüklü değil ya da oyun klasörü taşınmış.
    .todo = Oyun klasörünü elle seçin.
e3 = Bir oyun dosyası başka bir program tarafından değiştirildi.
    .why = gameinfo.gi orijinaline benzemiyor — bunu çoğu zaman Deadlock Mod Manager yapar. deadloader başkalarının yaptığı değişikliklerin üzerine yazmaz.
    .todo = Steam’de oyun dosyalarının bütünlüğünü doğrulayın; ardından deadloader kendi satırlarını ekler.
e4 = Katalog kullanılamıyor.
    .why = GameBanana API’sini değiştirdi, bu yüzden deadloader onu okuyamıyor. Kütüphaneniz ve profilleriniz her zamanki gibi çalışır.
    .todo = deadloader için güncelleme olup olmadığını kontrol edin veya daha sonra yeniden deneyin.
e5 = GameBanana yoğun.
    .why =
        { $reason ->
            [rate_limited] Kısa sürede çok fazla istek gönderildi (HTTP 429).
            [server] GameBanana sunucusu bir hata bildirdi.
           *[other] Çok fazla istek var veya sunucuda bir hata oluştu.
        }
    .todo =
        { $wait ->
            [seconds]
                { $retry_in_s ->
                    [one] deadloader { $retry_in_s } saniye sonra otomatik olarak yeniden deneyecek.
                   *[other] deadloader { $retry_in_s } saniye sonra otomatik olarak yeniden deneyecek.
                }
           *[later] deadloader birazdan otomatik olarak yeniden deneyecek.
        }
e6 = İndirme başarısız oldu.
    .why =
        { $reason ->
            [local_mode] deadloader yerel çalışma modunda olduğu için hiçbir ağ isteği göndermiyor.
            [dns] İndirme sunucusunun adresi bulunamadı.
            [connect] deadloader indirme sunucusuna bağlanamadı.
            [tls] İndirme sunucusuyla güvenli bir bağlantı kurulamadı.
            [timeout] Sunucu çok geç yanıt verdi.
            [connection_lost] Bağlantı kesildi.
            [http_status] Sunucu bir hatayla yanıt verdi (HTTP { $status }).
           *[other] İndirme yarıda kesildi.
        }
    .todo =
        { $reason ->
            [local_mode] Mod indirmek için Ayarlar’dan kataloğu açın.
           *[other] İndirme kaldığı yerden devam edecek.
        }
e7 = İndirilen dosya bozuk.
    .why =
        { $reason ->
            [size] Boyutu, GameBanana’da belirtilenle eşleşmiyor.
           *[checksum] Sağlama toplamı, GameBanana’da belirtilenle eşleşmiyor.
        }
    .todo = Dosyayı yeniden indirin.
e8 = Arşiv ayıklanamıyor.
    .why =
        { $reason ->
            [not_an_archive] Dosya bir ZIP, 7z veya RAR arşivi değil — indirme yarım kalmış olabilir.
            [damaged] Arşiv bozuk.
            [encrypted] Arşiv parolayla korunuyor.
            [unsupported] deadloader’ın desteklemediği bir sıkıştırma yöntemi kullanıyor.
            [not_a_vpk] { $entry } geçerli bir .vpk dosyası değil.
           *[other] Arşiv bozuk veya desteklenmeyen bir biçimde.
        }
    .todo = Bu modun başka bir dosyasını deneyin ya da sorunu modun yapımcısına bildirin.
e9 = Arşiv güvenle ayıklanamayacak kadar büyük.
    .why =
        { $reason ->
            [too_large]
                { $measured ->
                    [yes] Ayıklandığında { $unpacked } olacak, sınır ise { $limit }.
                   *[no] Ayıklandığında { $limit } sınırını aşıyor.
                }
            [too_many_files]
                { $found ->
                    [one] { $found } mod dosyası içeriyor, sınır ise { $limit }.
                   *[other] { $found } mod dosyası içeriyor, sınır ise { $limit }.
                }
            [ratio] { $entry } şüphe uyandıracak kadar çok sıkıştırılmış — { $limit } kattan fazla. Bu, kötü amaçlı bir arşivin tipik özelliğidir.
            [memory] Ayıklamak için { $needed } bellek gerekiyor, sınır ise { $limit }.
           *[other] Arşivler için belirlenen güvenlik sınırlarından birini aşıyor.
        }
    .todo =
        { $reason ->
            [memory] Bu modun başka bir dosyasını deneyin.
            [ratio] Kaynağına güvenmiyorsanız yüklemeyin; yalnızca güveniyorsanız sınırı Ayarlar → Gelişmiş bölümünden yükseltin.
           *[other] Arşive güveniyorsanız Ayarlar → İndirmeler bölümünden sınırı yükseltin.
        }
e10 = Bu arşivde mod dosyası yok.
    .why =
        { $listing ->
            [files] İçinde hiç .vpk dosyası yok, yalnızca şunlar var: { $contents }.
           *[empty] Arşiv boş.
        }
    .todo = Bu dosya, deadloader’ın yükleyebileceği biçimde bir Deadlock modu değil.
e11 = Bu bilgisayarda RAR arşivleri ayıklanamıyor.
    .why = Windows’un yerleşik aracı bu RAR dosyasını okuyamıyor; 7-Zip ve WinRAR da bulunamadı.
    .todo = 7-Zip’i (ücretsiz) yükleyip yeniden deneyin; indirilen dosya saklanıyor.
e12 = Yeterli disk alanı yok.
    .why =
        { $reason ->
            [space] { $needed } gerekiyor, ancak yalnızca { $available } boş.
           *[full] Disk dolu.
        }
    .todo = Biraz yer açın veya Kütüphane’de eski mod sürümlerini silin, sonra yeniden deneyin.
e13 = Deadlock çalışırken değişiklikler uygulanamaz.
    .why = Oyun çalıştığı sürece Windows oyunun dosyalarını kilitler.
    .todo = Deadlock’u kapatın; deadloader değişiklikleri hemen ardından uygular.
e14 = Kütüphanedeki bir mod dosyası eksik veya bozuk.
    .why = Dosya deadloader dışında değiştirilmiş ya da silinmiş.
    .todo = Dosyayı yeniden indirmek için modu onarın veya modu kaldırın.
e15 = Bağlantı açılamadı.
    .why = Windows’ta varsayılan tarayıcı ayarlanmamış.
    .todo = Bağlantıyı kopyalayıp kendiniz açın.
e16 = İstek engellendi.
    .why =
        { $reason ->
            [not_https] Sunucu HTTPS kullanmayan bir adrese yönlendirdi; deadloader bu tür yönlendirmeleri asla izlemez.
            [bad_redirect] Sunucu, deadloader’ın izleyemeyeceği bir yönlendirme gönderdi.
            [too_many_redirects] Sunucu çok fazla kez yönlendirdi.
           *[not_listed] { $host } izin verilen sunucular listesinde yok, bu yüzden deadloader ona bağlanmadı.
        }
    .todo =
        { $reason ->
            [not_listed] Bir şey yapmanız gerekmiyor — deadloader yalnızca GameBanana ile iletişim kurar. Bu sunucuya güveniyorsanız Ayarlar → Gelişmiş bölümünden ekleyin.
           *[not_https] Bir şey yapmanız gerekmiyor — deadloader yalnızca güvenli yönlendirmeleri izler. Daha sonra yeniden deneyin.
        }
e17 = Profil içe aktarılamadı.
    .why =
        { $reason ->
            [newer] Daha yeni bir deadloader sürümüyle oluşturulmuş.
            [unreadable] Dosya okunamıyor — taşınmış, silinmiş ya da başka bir program tarafından açılmış olabilir.
           *[not_a_profile] Bu dosya bir deadloader profili değil.
        }
    .todo =
        { $reason ->
            [newer] deadloader’ı güncelleyip profili yeniden içe aktarın.
            [unreadable] Dosyanın hâlâ yerinde olduğundan ve başka bir programda açık olmadığından emin olup yeniden deneyin.
           *[not_a_profile] Doğru dosyayı seçtiğinizden emin olun.
        }
e18 = Oyun klasöründeki dosyalar değiştirilemedi.
    .why =
        { $reason ->
            [access_denied] Windows oyun klasörüne erişimi reddetti.
            [in_use] Bir dosya başka bir program tarafından kullanılıyor; örneğin bir virüsten koruma yazılımı veya başka bir mod yöneticisi.
           *[other] Windows dosyaları değiştiremedi.
        }
    .todo = Diğer programları kapatıp yeniden deneyin.
e19 = Görsel yüklenemedi.
    .why =
        { $reason ->
            [no_response] Sunucu yanıt vermedi.
            [not_an_image] Dosya bir görsel değil.
            [too_large] Görsel çok büyük.
           *[other] Sunucu kullanılabilir bir görsel göndermedi.
        }
    .todo = Yer tutucu yerinde kalır; yeniden deneyebilirsiniz.
e20 = Çok fazla mod etkin.
    .why =
        { $free ->
            [one] Oyun { $free } mod dosyası daha yükleyebilir, bu profilin ise { $wanted } dosyaya ihtiyacı var.
           *[other] Oyun { $free } mod dosyası daha yükleyebilir, bu profilin ise { $wanted } dosyaya ihtiyacı var.
        }
    .todo = Bazı modları kapatın veya daha az eklenti seçin.
e21 = “{ $name }” adında bir profil zaten var.
    .why = Adlar, dosya adlarında olduğu gibi büyük/küçük harf ayrımı yapılmadan karşılaştırılır.
    .todo = Başka bir ad seçin.
e22 = “{ $name }” profil adı olarak kullanılamaz.
    .why = Windows bunu dosya adı olarak kabul etmez — örneğin CON, sonda nokta veya \ / : * ? " < > | karakterlerinden herhangi biri.
    .todo = Başka bir ad seçin.
e23 = deadloader kendi dosyasını okuyamıyor: { $file }.
    .why =
        { $reason ->
            [damaged_at] Dosya bozuk (satır { $line }, sütun { $column }).
            [damaged] Dosya bozuk.
            [newer] Daha yeni bir deadloader sürümüyle oluşturulmuş.
           *[invalid] İçeriği deadloader’ın beklediği gibi değil.
        }
    .todo =
        { $kind ->
            [settings] deadloader şimdilik varsayılan ayarları kullanıyor ve dosyayı olduğu gibi bırakıyor.
            [manifest] manifest.json.bak dosyasını geri yükleyin veya deadloader’ı güncelleyin.
           *[other] Dosyayı bir yedekten geri yükleyin veya deadloader’ı güncelleyin.
        }
e24 = Dosya artık GameBanana’da yok.
    .why = Yazarı dosyayı kaldırdı veya değiştirdi (HTTP { $status }).
    .todo = Başka bir dosya seçmek için mod sayfasını açın veya modun güncellemesi olup olmadığını kontrol edin.
e25 = GameBanana isteği reddetti.
    .why = Sunucu { $status } koduyla yanıt verdi. Bu genellikle geçicidir.
    .todo = Daha sonra yeniden deneyin. Sorun devam ederse deadloader’ın GitHub sayfasında bildirin.
internal = deadloader’ın içinde bir şeyler ters gitti.
    .why = Bu sizin değil, deadloader’ın hatası.
    .todo = Yeniden deneyin. Tekrar olursa Ayrıntılar’daki günlük kaydıyla birlikte bildirin.

## Buttons named in the texts above

action-undo = Geri al
action-review = İncele
action-retry = Yeniden dene
action-retry-now = Şimdi yeniden dene
action-browse = Gözat…
action-verify-in-steam = Steam’de doğrula
action-show-differences = Farkları göster
action-details = Ayrıntılar
action-repair = Onar
action-copy-link = Bağlantıyı kopyala
action-open-folder = Klasörü aç
action-open-release-page = Sürüm sayfasını aç
action-enable-catalog = Kataloğu aç
action-learn-more = Daha fazla bilgi
action-view-log = Günlüğü görüntüle

## Shared pieces

log-entry = Günlük kaydı { $id }
list-more = { $count } tane daha

## Sizes, labelled like Windows Explorer labels them

size-bytes =
    { $count ->
        [one] { $count } bayt
       *[other] { $count } bayt
    }
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
