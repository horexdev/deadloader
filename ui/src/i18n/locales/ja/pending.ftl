# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = 閉じる
library-apply = 適用
library-apply-profile = 「{ $name }」を適用
library-discard-all = すべて破棄
library-keep = 「{ $name }」のままにする
library-keep-applied = 適用した変更を残す
library-undo-this = この変更を元に戻す
library-undo-all = すべて元に戻す
library-close-to-apply = Deadlockを閉じると、変更が自動的に適用されます
library-play-applies = プレイでも適用されます
library-applied-title = 変更を適用しました
library-nothing-pending = 保留中の変更はありません
library-all-in-game = プロファイル「{ $name }」の内容はすべてゲームに適用済みです。
library-running-sub = Deadlockの実行中は、Windowsがゲームのファイルをロックしています。これらの変更は、ゲームを閉じるとすぐに自動的に適用されます。
library-waiting-sub = これらの変更はまだゲームフォルダーに反映されていません。今すぐ適用するか、プレイを押してください。
library-switch-sub = プロファイルの切り替えは多くのファイルを一度に変更するため、あなたの操作を待ちます。ここで適用するか、プレイを押してください。deadloaderが起動前に適用します。
library-applied-sub = { $time }にDeadlockが閉じられたため、実行中に行った{ $count }件の変更を適用しました。次に変更するまで元に戻せます。
library-undoing-sub = { $time }に適用した{ $count }件の変更を元に戻しましたが、Deadlockが再び実行中です。ゲームを閉じると自動的に元に戻されます。
library-group-enable = 有効化 · { $count }
library-group-disable = 無効化 · { $count }
library-group-order = 順序 · { $count }
library-group-variant = バリアント · { $count }
library-group-version = バージョン · { $count }
library-group-switch = プロファイルの切り替え · { $count }
library-group-undo = { $time }に適用した{ $count }件の変更の取り消し
library-change-on = オフ → オン
library-change-off = オン → オフ
library-change-above = { $name }の上に移動
library-change-above-wins = { $name }の上に移動 — 競合で優先されるようになりました
library-change-below = { $name }の下に移動
library-switch-what-happens = { $on }個のMODがオン · { $off }個がオフ
library-what-happens = ゲームフォルダーで起きること
library-folder-summary = { $added }ファイル追加 · { $removed }削除 · { $renumbered }番号変更

## Foreign files
library-foreign-title = ゲームのMODフォルダーで見つかったもの
library-foreign-note = これらはdeadloaderが置いたものではありません。あなたが選ばない限り、deadloaderは触れません。
library-collapse = 折りたたむ
library-expand = 展開
library-take = 管理下に置く
library-leave = そのままにする
library-left = そのままにしています
library-foreign-open = { $name }があるフォルダーを開く
library-foreign-vpk = VPK、{ $count }ファイル
library-foreign-vpk-hero = VPK、{ $count }ファイル — { $hero }のスキンのようです
library-foreign-unreadable = deadloaderが読み取れないMODファイル
library-foreign-replaced = deadloaderが置いたファイルと入れ替わったファイル
library-foreign-dmm = Deadlock Mod Managerが作成 — 設定ファイルで、MODではありません
library-foreign-other = ファイル（MODではありません）
library-foreign-folder = フォルダー、{ $count }ファイル — MODではありません
library-foreign-meta = { $size } · { $date }に追加
library-taken = { $name }をライブラリに追加しました。

## D11
library-d11-title = { $name }を管理下に置きますか？
library-d11-lead = deadloaderは次のことを行います：
library-d11-step-copy = ローカルMODとしてライブラリにコピーします（{ $size }）。
library-d11-step-replace = ゲームフォルダーのファイルを、deadloaderがリンクしたコピーに置き換えます。
library-d11-step-enable = プロファイル「{ $profile }」でリストの最後に追加して有効にします。
library-d11-name = ライブラリでの名前
library-d11-hint = ライブラリには元のファイルの完全なコピーが保存されます。
library-d11-suggested = { $file }（{ $hero }のスキン）

## D10
library-d10-title = ゲームファイルがほかのプログラムによって変更されました
library-d10-what = Deadlockが読み込むフォルダーを決めるゲームファイルを、ほかのプログラムが変更しました。多くの場合、Deadlock Mod Managerです。deadloaderはほかのプログラムによる変更を上書きしないため、これが直るまでMODは読み込まれません。
library-d10-do = Steamで元のファイルを復元してください（ゲームファイルの整合性を確認）。その後、deadloaderが自分の行を追加してお知らせします。
library-d10-diff = 違いを表示
library-d10-expected = 期待される内容（元のファイル + deadloaderの行）
library-d10-found = 現在の内容
library-d10-diff-note = 強調表示：deadloaderが追加する行と、ほかのプログラムが追加した行。
library-d10-help = このファイルは何ですか？
library-d10-help-text = gameinfo.giは、Deadlockが読み込むフォルダーを指定するファイルです。MODマネージャーは数行を追加して、ゲームがaddonsフォルダーからMODを読み込むようにします。ゲームのアップデートでファイルはリセットされますが、deadloaderが自動的に直します。ただし、ファイルが元の状態の場合に限ります。
library-d10-wait-title = SteamがDeadlockのファイルを確認しています。
library-d10-wait-text = Steamの確認が終わったらdeadloaderに戻ってください。ファイルは自動的に再確認されます。今すぐ確認することもできます。
library-d10-fixed = gameinfo.giを設定しました — MODが再び読み込まれます。
library-d10-fixed-text = Steamが元のファイルを復元し、deadloaderが自分の行を追加しました。詳しくはアクティビティ → ファイルで確認できます。
library-open-file = ファイルを開く
library-not-now = 今はしない
library-verify-steam = Steamでゲームファイルを確認
library-check-again = もう一度確認
library-show-differences = 違いを表示

## D5
library-update = 更新
library-update-on-gb = GameBananaに新しいバージョンがあります。
library-later = 後で
library-d5-title = { $name }を更新しますか？
library-d5-installed = インストール済み
library-d5-new = GameBananaの新バージョン
library-d5-downloaded = ダウンロード済み
library-d5-arrived = ライブラリにあり、まだ使用されていません
library-d5-new-meta = { $date }に更新 · ダウンロード { $size }
library-d5-download = ダウンロード { $size }
library-d5-changed = 新しいバージョンではファイル構成が異なります。deadloaderは次のように対応付けました：
library-d5-same = これらのファイルは新しいバージョンに置き換えられます：
library-d5-map-text = deadloaderには、どの新しいファイルが置き換えるのかを判断できないファイルがありました。それぞれ新しいファイルを選ぶか、古いファイルを残してください。
library-d5-map-label = ファイルの対応付け
library-d5-col-old = インストール済みのファイル
library-d5-col-new = 新しいファイル
library-d5-pick = { $file }の新しいファイル
library-d5-choose = 選択…
library-d5-keep = 古いファイルを残す
library-d5-tag-replaces = 置き換え
library-d5-tag-same = 同じ名前
library-d5-tag-new = 新規 · MODページにあり
library-d5-profiles = { $old }を使っているすべてのプロファイルで{ $new }が使われるようになります。{ $old }はライブラリに残るので、MODの詳細から元に戻せます。
library-d5-install = アップデートをインストール
library-d5-confirm-map = これらのファイルを使う
library-update-downloading = { $name }のアップデートをダウンロード中です。届き次第インストールされます。
library-updated = { $name }を更新しました。
library-updated-text = 古いバージョンを使っていたすべてのプロファイルで有効です。古いバージョンは保存されているので、MODの詳細から元に戻せます。

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name }：オン
library-n12-off = { $name }：オフ
library-n12-above = { $name }：{ $other }の上に移動
library-n12-below = { $name }：{ $other }の下に移動
library-n12-switched = { $name }：{ $to }に切り替え
library-n12-files = { $name }：ファイルを変更

## S9: the file list of a pending apply
library-show-file-list = ファイル一覧を表示
library-hide-file-list = ファイル一覧を隠す
library-file-list = ゲームフォルダー内のファイル
library-files-added = 追加 · { $count }
library-files-removed = 削除 · { $count }
library-files-renumbered = 番号変更 · { $count }
