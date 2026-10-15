# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = プロファイル
profiles-count =
    { $count ->
       *[other] { $count } 個のプロファイル
    }
profiles-in-game = 現在ゲーム内: { $list }
profiles-in-game-item = { $drive } の「{ $name }」
profiles-in-game-none = 現在ゲームに入っているプロファイルはありません。
profiles-import = インポート…
profiles-import-title = .json ファイルからプロファイルをインポート
profiles-import-dialog = プロファイルをインポート
profiles-export-dialog = プロファイルをエクスポート
profiles-file-filter = deadloader プロファイル
profiles-new = 新しいプロファイル
profiles-selected = 選択中
profiles-applied = 適用済み
profiles-not-applied = 未適用
profiles-selected-title = ライブラリで編集し、プレイに使うプロファイル
profiles-pending-title = 適用またはプレイでゲームフォルダーに反映されます
profiles-applied-title = このプロファイルのファイルは現在 { $drive } のゲームフォルダーにあります
profiles-select = 選択
profiles-select-of = 「{ $name }」を選択
profiles-more-of = 「{ $name }」のその他の操作
profiles-rename = 名前を変更
profiles-duplicate = 複製
profiles-export = ファイルとしてエクスポート…
profiles-change-installation = ゲームのインストール先を変更…
profiles-delete = 削除
profiles-delete-confirm = 削除…
profiles-delete-last = deadloader には少なくとも 1 つのプロファイルが必要です。
profiles-name-label = プロファイル名
profiles-save = 保存
profiles-cancel = キャンセル
profiles-close = 閉じる
profiles-close-esc = 閉じる (Esc)
profiles-rename-hint = Enter で保存 · Esc でキャンセル
profiles-name-empty = プロファイル名を入力してください。
profiles-mods =
    { $count ->
       *[other] { $count } 個の MOD が有効
    }
profiles-mods-none = 有効な MOD なし
profiles-applied-when = { $when }に適用
profiles-last-applied = 最後に適用: { $when }
profiles-never-applied = 未適用
profiles-just-now = たった今
profiles-preview-empty = このプロファイルを選択してから、ライブラリで MOD を有効にしてください。
profiles-footnote = プロファイルとは、有効な MOD とその順番・バリエーション・バージョンのリストです。選択しても何もダウンロード・削除されません。適用すると違うファイルだけが変更されます。
profiles-copy-name = { $name } (コピー)
profiles-copy-name-n = { $name } (コピー { $number })
profiles-new-title = 新しいプロファイル
profiles-start-with = 開始時の内容
profiles-empty = 空
profiles-empty-hint = 有効な MOD なし — あとでライブラリで選びます。
profiles-copy-of = 「{ $name }」のコピー
profiles-copy-hint = { $mods } — 順番・バリエーション・バージョンも同じです。
profiles-installation = ゲームのインストール先
profiles-installation-hint = 「{ $name }」と同じです。あとでプロファイルのメニューから変更できます。
profiles-new-note = プロファイルを作成しても選択・適用はされません — ゲームは変わりません。
profiles-create = プロファイルを作成
profiles-pending-what = プロファイル「{ $name }」はまだゲームに入っていません。
profiles-pending-why-other = { $drive } のゲームフォルダーにはまだ「{ $other }」が入っています。今すぐ適用するか、プレイを押してください — deadloader が起動前に適用します。
profiles-pending-why-empty = { $drive } のゲームフォルダーには現在 deadloader の MOD がありません。今すぐ適用するか、プレイを押してください — deadloader が起動前に適用します。
profiles-keep = 「{ $name }」のままにする
profiles-apply = 「{ $name }」を適用
profiles-other-what = 「{ $name }」は別のゲームのインストール先を使います。
profiles-other-why = 適用とプレイでは { $path } を使います。{ $drive } のゲームは「{ $other }」のままです。
profiles-other-why-empty = 適用とプレイでは { $path } を使います。{ $drive } のゲームは変わりません。
profiles-delete-title = プロファイル「{ $name }」を削除しますか?
profiles-delete-applied =
    { $count ->
       *[other] 「{ $name }」は現在ゲームに入っています。deadloader はその { $count } 個の MOD をゲームフォルダーから取り除きます:
    }
profiles-delete-applied-empty = 「{ $name }」は現在ゲームに入っていますが MOD がないため、ゲームフォルダーは変わりません。
profiles-delete-stay = MOD はライブラリと他のプロファイルに残ります。そこで別のプロファイルを適用するまで、Deadlock は deadloader の MOD なしで起動します。
profiles-delete-next = 「{ $name }」が選択中のプロファイルになります。適用またはプレイを押すまで適用されません。
profiles-delete-undoable = この操作は元に戻せません。コピーを残すには、先にプロファイルをエクスポートしてください。
profiles-export-first = 先にエクスポート
profiles-delete-profile = プロファイルを削除
profiles-installation-title = 「{ $name }」のゲームのインストール先
profiles-installation-lead = このプロファイルを適用する Deadlock フォルダーを選んでください。
profiles-used-by =
    { $count ->
       *[other] { $count } 個のプロファイルが使用
    }
profiles-used-by-none = 使用しているプロファイルなし
profiles-installation-applied = 「{ $name }」は現在 { $drive } のゲームに入っています。{ $drive } で別のプロファイルを適用するまで、ファイルはそこに残ります。
profiles-installation-not-applied = 「{ $name }」を適用するまで、どちらのゲームフォルダーも変わりません。
profiles-manage-installations = 設定でインストール先を管理
profiles-use-installation = このインストール先を使う
profiles-d9-title =
    { $count ->
       *[other] 「{ $name }」の MOD のうち { $count } 個がライブラリにありません
    }
profiles-d9-file = ファイル
profiles-d9-lead = には { $listed } 個の MOD があります。{ $present } 個はすでにライブラリにありますが、次の { $count } 個はありません:
profiles-d9-list = 足りない MOD
profiles-d9-from-gamebanana = GameBanana から { $count } 個
profiles-d9-local-count = ローカル { $count } 個
profiles-gamebanana = GameBanana
profiles-can-download = ダウンロード可能
profiles-local-mod = ローカル MOD
profiles-cant-download = ダウンロード不可
profiles-local-why = 別の PC でディスクから追加されたため、GameBanana にはありません。プロファイルを共有した人にファイルを頼んでください。
profiles-d9-local-mode = カタログがオフ (ローカルモード) のため、deadloader は MOD をダウンロードできません。それらなしでインポートし、あとでディスクから追加してください — または設定でカタログをオンにしてください。
profiles-d9-after = どちらの場合も「{ $name }」は選択中のプロファイルになりますが、適用はされません — 適用またはプレイを押すまでゲームフォルダーは変わりません。
profiles-import-without = それらなしでインポート
profiles-download-missing = 足りないものをダウンロード ({ $count })
profiles-turned-off =
    { $count ->
       *[other] { $count } 個の MOD をオフ
    }
profiles-turned-on =
    { $count ->
       *[other] { $count } 個の MOD をオン
    }
profiles-differ = 違うファイルだけを変更しました。
profiles-same = 同じ MOD がすでにゲームに入っていました。順番とバリエーションだけを確認しました。
profiles-deleted = プロファイル「{ $name }」を削除しました。
profiles-deleted-not-applied = 適用されていなかったため、ゲームフォルダーは変わっていません。
profiles-deleted-applied =
    { $count ->
       *[other] { $count } 個の MOD をゲームフォルダーから取り除きました。ライブラリには残っています。
    }
profiles-exported = プロファイル「{ $name }」をエクスポートしました。
profiles-imported = プロファイル「{ $name }」をインポートしました。
profiles-imported-text = 選択中ですが、まだ適用されていません。
profiles-imported-without =
    { $count ->
       *[other] プロファイル「{ $name }」を { $count } 個の MOD なしでインポートしました。
    }
profiles-imported-without-text = 選択中ですが、まだ適用されていません。足りない MOD はあとで追加できます。
profiles-imported-downloading =
    { $count ->
       *[other] プロファイル「{ $name }」をインポートしました — { $count } 個の MOD をダウンロード中です。
    }
profiles-imported-downloading-text = 選択中ですが、まだ適用されていません。ダウンロードが終わったら適用してください。
profiles-open-downloads = ダウンロードを開く
profiles-undo = 元に戻す
profiles-check-update = deadloader の更新を確認
profiles-choose-another = 別のファイルを選ぶ
profiles-switcher = プロファイルを管理
profiles-saved-as = 保存先:
profiles-show-folder = フォルダーで表示
