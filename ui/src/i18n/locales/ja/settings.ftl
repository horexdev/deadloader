# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = 設定
settings-nav = 設定のセクション
settings-reset-nav = すべての設定をリセット…
settings-default = 既定: { $value }
settings-restore = 既定に戻す
settings-on = オン
settings-off = オフ
settings-cancel = キャンセル
settings-remove = 削除
settings-remove-item = { $item } を削除
settings-add = 追加
settings-open-folder = フォルダーを開く
settings-open-folder-at = フォルダーを開く: { $path }
settings-copied = クリップボードにコピーしました。
settings-folder = フォルダー
settings-example = 例:

settings-section-general = 一般
settings-section-general-lead = 言語、起動、ズーム。
settings-section-game = ゲーム
settings-section-game-lead = Deadlock のインストール場所と、deadloader がそこで変更してよいもの。
settings-section-catalog = カタログ
settings-section-catalog-lead = GameBanana のカタログ、センシティブなコンテンツ、画像とリンク。
settings-section-library = ライブラリ
settings-section-library-lead = Mod の更新、Mod の削除、古いバージョン。
settings-section-downloads = ダウンロード
settings-section-downloads-lead = ダウンロードキュー、アーカイブ、展開。
settings-section-network = ネットワーク
settings-section-network-lead = deadloader と GameBanana の通信方法。
settings-section-cache = キャッシュ
settings-section-cache-lead = この PC に保存されたサムネイルとカタログのページ。
settings-section-logs = ログ
settings-section-logs-lead = アクティビティが何をどれだけの期間保持するか。
settings-section-advanced = 詳細
settings-section-advanced-lead = 設定ファイルと追加の許可ドメイン。
settings-section-about = deadloader について
settings-section-about-lead = バージョン、ライセンス、このダウンロードの確認方法、データの保存場所。

# General
settings-general-group = 言語と起動
settings-language = 言語
settings-language-help = システムの言語: deadloader が対応していれば Windows の表示言語、なければ英語です。
settings-language-system = システムの言語 ({ $language })
settings-startup-profile = 起動時のプロファイル
settings-startup-profile-help = deadloader の起動時に選択されるプロファイルです。選択しても適用はされません。
settings-startup-last = 最後に選択したもの ({ $name })
settings-startup-last-plain = 最後に選択したもの
settings-app-updates = 起動時に deadloader の更新を確認
settings-app-updates-help = 起動時に GitHub (api.github.com) へ新しいバージョンがあるかを 1 回だけ問い合わせます。更新は常に手動です。初回起動時に選択した設定です。
settings-app-updates-local = 利用できません: ローカルモードではネットワークリクエストを一切行いません。カタログをオンにすると利用できます (設定 → カタログ)。
settings-zoom = ズーム
settings-zoom-keys-1 = Ctrl + マウスホイールでも変更でき、
settings-zoom-keys-2 = で拡大・縮小、
settings-zoom-keys-3 = でリセットします。
settings-ctrl = Ctrl
settings-percent = { $value }%

# Game
settings-installs-group = ゲームのインストール
settings-installs-lead = deadloader は Steam にある Deadlock のインストールを自動で見つけます。各プロファイルはそのうち 1 つを使います。変更はプロファイルで行います。
settings-install-library = Steam ライブラリ { $library }
settings-install-build = ビルド { $build }
settings-install-used =
    { $count ->
        [0] どのプロファイルも使用していません
       *[other] { $count } 個のプロファイルが使用中: { $names }
    }
settings-install-selected = { $name } (選択中)
settings-install-applied = { $name } (ここに適用済み)
settings-install-steam = Steam で検出
settings-install-manual = 手動で追加
settings-install-known = deadloader のファイルがまだ残っています
settings-install-manual-meta = 手動で追加
settings-gi-ours = gameinfo.gi: OK — deadloader の行があります
settings-gi-wait = gameinfo.gi: プロファイルで使用するときに設定されます
settings-gi-rolled = gameinfo.gi: 元のまま — 適用すると deadloader の行が再び追加されます
settings-gi-foreign = gameinfo.gi: 別のプログラムが変更しています
settings-gi-missing = gameinfo.gi: 見つかりません — フォルダーが移動された可能性があります
settings-installs-none = まだインストールが見つかっていません。
settings-add-manual = 手動で追加…
settings-add-manual-hint = 見つからなかった Deadlock のフォルダーを選択します。
settings-add-manual-title = Deadlock のフォルダーを選択
settings-install-added = インストールを追加しました。
settings-install-removed = インストールを一覧から削除しました。
settings-install-removed-text = そのフォルダー内は何も変更していません。
settings-gameinfo-group = ゲームの更新後
settings-autofix = gameinfo.gi を自動で修正
settings-autofix-help = Deadlock を更新すると、Mod を読み込むよう指示するゲームファイルが元に戻ります。deadloader はバックアップを取り、自分の行を戻して通知します。オフの場合は先にバナーで確認します。
settings-danger-group = 元に戻す・削除
settings-rollback = 選択中のプロファイルのゲームフォルダーをロールバック
settings-rollback-help = 選択中のプロファイル { $profile } のインストール ({ $drive }): deadloader が置いた Mod ファイル { $files } 個と gameinfo.gi の行を削除します。ライブラリとプロファイルは残り、適用ですべて元に戻ります。
settings-rollback-help-others = ほかのゲームフォルダーは変更しません。ロールバックするには、それを使うプロファイルを選択してください。
settings-rollback-nothing = { $drive } にはロールバックするものがありません — 現在 deadloader のファイルはありません。
settings-rollback-no-game = Deadlock が見つからないため、ロールバックするゲームフォルダーがありません。
settings-rollback-button = ロールバック…
settings-rolled-title = { $drive } のゲームフォルダーをロールバックしました。
settings-rolled-text =
    { $foreign ->
        [0] { $drive } で Mod ファイル { $files } 個と gameinfo.gi の deadloader の行を削除しました。ライブラリとプロファイルはそのままです — 適用またはプレイで Mod が戻ります。
       *[other] { $drive } で Mod ファイル { $files } 個、gameinfo.gi の deadloader の行、ほかのファイル { $foreign } 個を削除しました。ライブラリとプロファイルはそのままです — 適用またはプレイで Mod が戻ります。
    }
settings-wipe = deadloader のデータをすべて削除
settings-wipe-help = ライブラリ、プロファイル、ログ、キャッシュ、設定 ({ $size }) を削除し、deadloader を閉じます。deadloader.exe は残るので、あとでご自身で削除してください。
settings-wipe-button = すべてのデータを削除…

# Catalog
settings-catalog-group = GameBanana カタログ
settings-catalog = カタログを使う
settings-catalog-help-on = GameBanana の Mod を閲覧してインストールします。deadloader は GameBanana とだけ通信し、すべてのリクエストはアクティビティに表示されます。オフはローカルモードです。
settings-catalog-help-off = オフ — ローカルモード: deadloader はネットワークリクエストを一切行いません。ディスクからの Mod 追加は引き続き可能です。
settings-consent-title = その前に 1 ステップ。
settings-consent-text = カタログをオンにする前に、接続するホストと GameBanana の規約を表示します。
settings-consent-go = 確認してオンにする
settings-local-on = ローカルモードがオンです。
settings-local-on-text = カタログはオフで、deadloader はネットワークリクエストを行いません。
settings-ttl = 読み込んだカタログページを再利用する期間
settings-ttl-help = この期間内にページへ戻っても、GameBanana に再度問い合わせません。
settings-minutes = { $count } 分
settings-hours = { $count } 時間
settings-prefetch = 次のページを先読み
settings-prefetch-help = 結果の約 70% までスクロールすると、グリッドが続くよう次の結果を読み込みます。
settings-nsfw-group = センシティブなコンテンツ
settings-nsfw = コンテンツレーティング付きの Mod (NSFW)
settings-nsfw-help = 非表示: まったく表示しません。ぼかし: 「センシティブなコンテンツ」ラベル付きのぼかした画像。クリックで表示。表示: ぼかしなし。
settings-nsfw-age-done = { $date } に年齢を確認済み。
settings-nsfw-age-ask = 「非表示」から変更すると、一度だけ年齢の確認を求めます。
settings-nsfw-hidden = 非表示
settings-nsfw-blurred = ぼかし
settings-nsfw-shown = 表示
settings-images-group = 画像とリンク
settings-description-images = Mod の説明内の画像
settings-description-images-help = GameBanana 以外のホスト (例: i.imgur.com) から読み込まれます。クリック時: クリックするまでプレースホルダーを表示します。
settings-images-click = クリック時
settings-images-always = 常に
settings-prechecked = 「このホストの画像を常に許可」をあらかじめチェック
settings-prechecked-help = 説明内の画像が新しいホストから初めて読み込まれるときに表示されるダイアログで。
settings-hosts = 許可した画像ホスト
settings-hosts-help = これらのホストの画像は確認なしで読み込まれます。アクティビティではリクエストに「許可した画像ホスト」と表示されます。
settings-hosts-built-in = 組み込み
settings-hosts-none = 追加したホストはありません — 新しいホストは毎回まず確認します。
settings-host-removed = { $host } を削除しました
settings-host-removed-text = このホストの画像は読み込み前に再び確認します。
settings-undo = 元に戻す
settings-gif = GIF を自動再生
settings-gif-help = オフ: GIF は再生ボタン付きで最初のフレームを表示します。
settings-links = 外部リンク
settings-links-help = リンクは既定のブラウザーで開きます。「先に確認」では開く前に完全なアドレスを表示します。
settings-links-ask = 先に確認
settings-links-open = すぐに開く

# Library
settings-updates-group = Mod の更新
settings-check-updates = 起動時に Mod の更新を確認
settings-check-updates-help = GameBanana から入手した Mod をカタログと比較します。ローカルモードでは行いません。
settings-auto-download = 更新を自動でダウンロード
settings-auto-download-help = ダウンロードのみ — Mod を新しいバージョンに切り替える前には必ず確認します。
settings-disk-group = 削除と古いバージョン
settings-deletion = Mod を削除するとき
settings-deletion-help = 削除ダイアログで毎回別の方法を選べます。
settings-deletion-permanent = 完全に削除
settings-deletion-bin = ごみ箱へ
settings-old-versions = 古いバージョン
settings-old-versions-help = ダウンロードしたバージョンはすべて保持され、戻すことができます。古いバージョンは Mod の詳細から削除します。
settings-old-keep-all = すべて保持

# Downloads
settings-queue-group = キュー
settings-parallel = 同時ダウンロード数
settings-parallel-help = 同時にダウンロードするファイルの数。
settings-speed = 速度制限
settings-speed-help = Mod のダウンロードのみ — カタログとサムネイルは制限しません。
settings-speed-none = 制限なし
settings-speed-value = { $value } MB/秒
settings-keep-archives = インストール後もアーカイブを保持
settings-keep-archives-help = オフ: .vpk ファイルがライブラリに入った時点で .zip、.7z、.rar ファイルを削除します。
settings-unpack-group = 展開
settings-limits = 展開の上限
settings-limits-help = 展開すると巨大になるアーカイブから守ります。信頼できるアーカイブが拒否された場合は上限を上げてください。
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = 展開後のサイズ
settings-limit-ratio = 圧縮率
settings-limit-files = アーカイブあたりの Mod ファイル数
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = 個
settings-unit-s = 秒
settings-number-error = { $field }: { $min }～{ $max } の整数を入力してください。それまでは以前の値が使われます。

# Network
settings-network-local = ローカルモードがオンです。
settings-network-local-text = deadloader はネットワークリクエストを行いません。これらの設定はカタログをオンにすると有効になります。
settings-requests-group = GameBanana へのリクエスト
settings-api-parallel = カタログの同時リクエスト数
settings-api-parallel-help = GameBanana の API への同時リクエスト数です。画像とダウンロードは別に数えます。
settings-timeouts = タイムアウト
settings-timeouts-help = 接続: この時間内に接続できなければ中止します。応答: サーバーからの送信が止まったら中止します。
settings-timeouts-default = 10 秒 · 30 秒
settings-timeout-connect = 接続
settings-timeout-read = 応答
settings-retries = 再試行
settings-retries-help = 失敗したリクエストを間隔を延ばしながら繰り返す回数です。その後 deadloader がエラーを表示します。
settings-connection-group = 接続
settings-proxy = プロキシ
settings-proxy-help = なし: 直接接続。システム: Windows で設定したプロキシ。手動: 下に入力します。
settings-proxy-none = なし
settings-proxy-system = システム
settings-proxy-manual = 手動
settings-proxy-host = アドレス
settings-proxy-port = ポート
settings-proxy-hint = HTTP プロキシ。deadloader 自身のリクエストのみが通ります。
settings-proxy-invalid = ユーザー名やパスワードを含まないホスト名またはアドレスを入力してください (例: proxy.example.com)。

# Cache
settings-thumbs-group = サムネイルとカタログページ
settings-cache-limit = キャッシュの上限
settings-cache-limit-help = キャッシュがいっぱいになると、最も長く見ていないサムネイルから削除されます。
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = 現在のサイズ
settings-cache-breakdown = サムネイル { $thumbnails } 個 · カタログページ { $pages } 個
settings-cache-empty = 空 — まだ何もキャッシュされていません。
settings-cache-used = { $used } / { $limit }
settings-cache-clear = キャッシュを消去
settings-cache-cleared = キャッシュを消去しました — { $size } を解放しました。サムネイルは必要なときに再び読み込まれます。
settings-cache-cleared-partly = 使用中だったファイル { $failed } 個は次回の消去まで残ります。

# Logs
settings-keep-group = ログの保持
settings-log-days = ログの保持期間
settings-log-days-help = 古い記録は deadloader の起動時に削除されます。
settings-days = { $count ->
       *[other] { $count } 日
    }
settings-year = 1 年
settings-log-size = 最大サイズ
settings-log-size-help = このサイズを超えると、古いログファイルから削除されます。
settings-privacy-group = プライバシー
settings-mask = Windows のユーザー名を隠す
settings-mask-help = ログ内のパスでは名前の代わりに { $masked } と表示します — ログを共有するときに安全です。
settings-logs-folder-group = ログフォルダー
settings-logs-folder = ログの保存場所
settings-logs-now = { $size } · { $files } 個のファイル · 最も古いのは { $date }
settings-open-logs = ログフォルダーを開く

# Advanced
settings-advanced-warning = 上級者向けです。
settings-advanced-warning-text = ここでの誤りは deadloader の動作を変える可能性があります。保存前にファイルを検査し、以前のバージョンのバックアップを残します。
settings-files-group = 設定ファイルの編集
settings-file-settings = 設定
settings-file-manifest = ゲームフォルダーのマニフェスト
settings-file-label = 編集するファイル
settings-file-settings-lead = これらのページの内容すべてを 1 つのファイルにまとめたものです。保存後に変更がページに反映されます。
settings-file-manifest-lead = deadloader がゲームフォルダーに置いたものの一覧です。ロールバックはこれに依存するため、誤った項目があるとファイルが残る場合があります。
settings-json-error = 有効な JSON ではないため、保存していません。
settings-json-where = { $line } 行目、{ $column } 列目: この位置かその直前に過不足があります — カンマや引用符であることがよくあります。
settings-json-fix = その箇所を直して再度保存してください。それまで deadloader は現在のファイルを使い続けます。
settings-json-refused = これらの値は使用できないため、何も保存していません。
settings-json-newer = このファイルは新しい deadloader で作成されたため (形式 { $found }、このバージョンは { $supported } まで対応)、保存していません。
settings-json-invalid = deadloader はこのファイルを使用できないため、保存していません。
settings-json-details = 詳細
settings-problem-unknown-field = { $path }: このバージョンの設定ではありません。
settings-problem-invalid-value = { $path }: この値はここでは使えません。
settings-problem-out-of-range = { $path }: { $found } は { $min }～{ $max } の範囲外です。
settings-problem-invalid-entry = { $path }: 「{ $entry }」はここでは無効です。
settings-problem-duplicate-entry = { $path }: 「{ $entry }」が 2 回あります。
settings-problem-needs-age = { $path }: NSFW コンテンツの表示には先に年齢の確認が必要です (設定 → カタログ)。
settings-problem-local-mode = { $path }: ローカルモードでは利用できません。
settings-json-idle = 保存時に検査します。
settings-json-saved = 有効なので保存しました。以前のバージョンは { $backup } として残しています。
settings-json-saved-plain = 有効なので保存しました。
settings-json-save = 検査して保存
settings-json-discard = 編集を破棄
settings-json-backup = 保存前に以前のファイルをバックアップします。最新の 10 個が残ります。
settings-domains-group = 追加の許可ドメイン
settings-domains-lead = deadloader は GameBanana とだけ通信します。ダウンロードが別のホストにリダイレクトされてブロックされた場合、ここでそのホストを許可できます。アクティビティでは、そのホストへのリクエストに「ユーザーが追加」と表示されます。
settings-domains-built-in = 組み込み:
settings-domain-user = ユーザーが追加
settings-domains-none = 追加のドメインはありません。
settings-domain-add = ドメインを追加
settings-domain-hint = ホスト名のみ (例: files.example.com)。
settings-domain-empty = まずホスト名を入力してください (例: files.example.com)。
settings-domain-not-host = https:// やパスを付けず、ホスト名のみを入力してください (例: files.example.com)。
settings-domain-invalid = 「{ $host }」はホスト名ではありません。綴りを確認してください (例: files.example.com)。
settings-domain-built-in = { $host } は組み込みで、常に許可されています。
settings-domain-listed = { $host } はすでに一覧にあります。

# About
settings-version = バージョン { $version }
settings-commit = ビルド元のコミット
settings-commit-local = ローカルビルド — GitHub Actions でビルドされていません
settings-copy-commit = コミット ID をコピー
settings-release-page = リリースページ
settings-licence-group = ライセンス
settings-licence-help = deadloader はフリーソフトウェアです。GPL-3 のもとで使用、調査、共有、変更できます。一切の保証はありません。
settings-read-licence = ライセンスを読む
settings-source = ソースコード
settings-verify-group = このダウンロードを確認する方法
settings-verify-lead = この deadloader.exe が GitHub で公開されたものであることを確かめる、2 つの独立した確認方法です。
settings-verify-hash = SHA-256 チェックサムを比較
settings-verify-hash-help = リリースページのチェックサムと一致する必要があります。改変されたコピーはここに偽の値を表示しうるため、ご自身でも計算してください:
settings-verify-hash-cmd = exe のフォルダーで PowerShell を使って:
settings-verify-attest = ビルドの証明を確認
settings-verify-attest-help = このファイルが公開されたソースコードから GitHub Actions でビルドされたことを証明します。GitHub CLI が必要です:
settings-copy-hash = チェックサムをコピー
settings-copy-command = コマンドをコピー
settings-unsigned = deadloader.exe はコード署名されていません。
settings-unsigned-text = 初回起動時に Windows SmartScreen が「Windows によって PC が保護されました」と表示することがあります。上の 2 つの確認で、ファイルが本物であることを確かめられます。
settings-webview-group = ウィンドウエンジン
settings-webview-help = deadloader のウィンドウは Windows の一部である WebView2 が描画します。WebView2 は Windows が自動で更新します。deadloader が無効にしているもの:
settings-webview-off-1 = DNS-over-HTTPS の問い合わせ
settings-webview-off-2 = ウィンドウ内での Microsoft アカウントのサインイン
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = バックグラウンドの通信とコンポーネントの更新
settings-webview-off-5 = フォーム自動入力のサーバー呼び出し
settings-webview-off-6 = Microsoft へのエラー報告 (domain reliability)
settings-webview-no-route = そして、ウィンドウにはインターネットへの経路がまったくありません。
settings-webview-unknown = バージョン不明
settings-data-group = あなたのデータ
settings-data-folder = データフォルダー
settings-data-folder-help = deadloader が保存するものはすべて、exe の隣にあるこのフォルダー 1 つにあります。
settings-notice = 初回起動時のお知らせ
settings-notice-help = バージョン { $version }、{ $date } に { $language } で同意。選択内容: カタログ { $catalog }、更新の確認 { $updates }。
settings-notice-none = まだ同意していません。
settings-notice-again = もう一度読む

# D7
settings-d7-title = { $drive } のゲームフォルダーをロールバックしますか?
settings-d7-from =
    { $others ->
        [0] 選択中のプロファイル { $profile } のインストールです。deadloader が次の場所から削除します:
       *[other] 選択中のプロファイル { $profile } のインストールです ({ $others } も使用)。deadloader が次の場所から削除します:
    }
settings-d7-files =
    { $count ->
       *[other] 追加した Mod ファイル { $count } 個 —
    }
settings-d7-line = gameinfo.gi 内の行
settings-d7-line-why = — Steam がインストールした状態に戻ります。
settings-d7-stays = そのまま残るもの:
settings-d7-library = ライブラリ — Mod { $mods } 個、{ $size }
settings-d7-profiles =
    { $count ->
       *[other] { $count } 個のプロファイル — { $names }
    }
settings-d7-other = { $path } のインストール ({ $profile }) — 変更しません。ロールバックするには、それを使うプロファイルを選択して再度ロールバックしてください。
settings-d7-other-plain = { $path } のインストール — 変更しません。
settings-d7-back = 適用またはプレイですべて元に戻ります。
settings-d7-foreign =
    { $count ->
       *[other] deadloader が作成していないファイル { $count } 個も削除する
    }
settings-d7-foreign-why = 別のプログラムまたは手動インストールによるものです。完全に削除され、deadloader では元に戻せません。
settings-d7-confirm = ロールバック
settings-d7-confirm-foreign =
    { $count ->
       *[other] ロールバックしてファイル { $count } 個を削除
    }

# D8
settings-d8-title = deadloader のデータをすべて削除しますか?
settings-d8-from = deadloader は次のフォルダー内をすべて削除し、
settings-d8-from-2 = その後終了します。
settings-d8-what = 項目
settings-d8-contains = 内容
settings-d8-size = サイズ
settings-d8-total = 合計
settings-d8-library = ライブラリ
settings-d8-library-has = Mod { $mods } 個、ファイル { $files } 個
settings-d8-profiles = プロファイル
settings-d8-logs = ログ
settings-d8-logs-has = ファイル { $files } 個、{ $date } 以降
settings-d8-logs-none = ログファイルなし
settings-d8-cache = キャッシュ
settings-d8-cache-has = サムネイルとカタログページ
settings-d8-settings = 設定
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
       *[other] バックアップ { $backups } 個を含む
    }
settings-d8-other = その他
settings-d8-other-has = 途中のダウンロード、gameinfo.gi のバックアップ、ウィンドウエンジンのデータ
settings-d8-exe = 削除されないもの: deadloader.exe 本体 — 不要になったらご自身で削除してください。
settings-d8-rollback = 先に deadloader が使うすべてのゲームフォルダーをロールバックする (推奨)
settings-d8-rollback-why = 削除の前に、すべてのゲームフォルダーから deadloader の Mod ファイルと gameinfo.gi の行を取り除きます。ほかのファイルは残ります。
settings-d8-rollback-off = チェックなし: deadloader の Mod ファイルはゲームフォルダーに残り、Deadlock は読み込み続けます — それを取り除く deadloader はもうありません。
settings-d8-type-1 = 確認のため
settings-d8-type-2 = と入力してください
settings-d8-word = 削除
settings-d8-hint = 単語が一致すると削除ボタンが有効になります。
settings-d8-hint-ok = 確認しました。元に戻せません。
settings-d8-type-first = 先に「{ $word }」と入力してください
settings-d8-deleting = 削除中…
settings-d8-confirm = 削除して閉じる
settings-d8-confirm-rollback = ロールバック・削除して閉じる

# Reset
settings-reset-title = すべての設定をリセットしますか?
settings-reset-body = 許可した画像ホストや追加のドメインも含め、すべての設定が既定に戻ります。
settings-reset-keeps = ライブラリ、プロファイル、ログは変更されず、ゲームフォルダーもそのままです。
settings-reset-backup = 現在の設定のバックアップを settings.json.bak-… として残します。
settings-reset-confirm = 設定をリセット
settings-reset-done = すべての設定を既定に戻しました。
settings-reset-done-text = バックアップ: { $backup }
