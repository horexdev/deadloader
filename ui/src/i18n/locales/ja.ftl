## Notifications

n1 = Deadlockが更新されました。MODを復元しました。
    .why = アップデートでgameinfo.giがリセットされたため、deadloaderが自分の行を追加し直しました。
n2 = { $name }をインストールして有効にしました。
n3 = { $count }件のMODアップデートがあります。
n4 = Deadlockが実行中です。
    .why = ゲームを終了すると、変更が自動的に適用されます。
    .pending = 保留中の変更: { $count }件
n5 =
    { $reason ->
        [completed] 中断された変更を完了しました。
       *[undone] 中断された変更を元に戻しました。ゲームフォルダーは以前の状態のままです。
    }
n6 = deadloaderが作成していないファイルが、ゲームのMODフォルダーに{ $count }個見つかりました。
n7 =
    { $when ->
        [today] オフライン — { $saved }に保存したカタログを表示しています。
       *[earlier] オフライン — { $saved }に保存したカタログを表示しています。
    }
    .why = オンラインに戻ると、ダウンロードが再開されます。
n8 = プロファイル「{ $name }」を適用しました。{ $count }個のMODが有効です。
n9 = deadloaderの新しいバージョン（{ $version }）を利用できます。
n10 = ローカルモードがオンです。
    .why = カタログはオフで、deadloaderはネットワーク要求を一切行いません。
n11 = deadloaderとゲームが別のドライブにあるため、MODはリンクではなくコピーされます。
    .why = そのため、ディスク容量を{ $size }余分に使用します。
n12 = Deadlockが終了しました。{ $count }件の変更を適用しました。

## Errors: what happened, why, what to do

e1 = deadloaderがデータを保存できません。
    .why =
        { $reason ->
            [access_denied] deadloader.exeのあるフォルダーは読み取り専用です。たとえば、WindowsではProgram Files内へのプログラムによる書き込みが許可されていません。
            [in_use] 別のプログラム（多くの場合はウイルス対策ソフト）が、deadloaderのファイルを開いています。
           *[other] Windowsがdeadloader.exeのあるフォルダーに書き込めませんでした。
        }
    .todo =
        { $reason ->
            [in_use] ほかのプログラムを閉じてから、もう一度お試しください。
           *[other] deadloaderを書き込み可能なフォルダー（例: D:\Games\deadloader）に移動してください。
        }
e2 = Deadlockが見つかりません。
    .why = Steamまたはゲームがインストールされていないか、ゲームフォルダーが移動されています。
    .todo = ゲームフォルダーを手動で選択してください。
e3 = ゲームファイルが別のプログラムによって変更されています。
    .why = gameinfo.giが元の状態と異なります。多くの場合、Deadlock Mod Managerによる変更です。deadloaderはほかのプログラムによる変更を上書きしません。
    .todo = Steamでゲームファイルの整合性を確認してください。その後、deadloaderが自分の行を追加します。
e4 = カタログを利用できません。
    .why = GameBananaのAPIが変更されたため、deadloaderが読み取れません。ライブラリとプロファイルは通常どおり使えます。
    .todo = deadloaderのアップデートを確認するか、しばらくしてからもう一度お試しください。
e5 = GameBananaが混雑しています。
    .why =
        { $reason ->
            [rate_limited] 短時間にリクエストが多すぎます（HTTP 429）。
            [server] GameBananaのサーバーがエラーを返しました。
           *[other] リクエストが多すぎるか、サーバーでエラーが発生しています。
        }
    .todo =
        { $wait ->
            [seconds] { $seconds }秒後にdeadloaderが自動的に再試行します。
           *[later] まもなくdeadloaderが自動的に再試行します。
        }
e6 = ダウンロードに失敗しました。
    .why =
        { $reason ->
            [connection] 接続が切断されました。
            [timeout] サーバーの応答に時間がかかりすぎました。
            [server] GameBananaのサーバーがエラーを返しました。
           *[other] ダウンロードが中断されました。
        }
    .todo = 中断したところから再開されます。
e7 = ダウンロードしたファイルが破損しています。
    .why = チェックサムがGameBananaに記載されている値と一致しません。
    .todo = もう一度ダウンロードしてください。
e8 = アーカイブを展開できません。
    .why =
        { $reason ->
            [not_an_archive] このファイルはZIP、7z、RARのいずれのアーカイブでもありません。ダウンロードが完了していない可能性があります。
            [damaged] アーカイブが破損しています。
            [encrypted] アーカイブがパスワードで保護されています。
            [unsupported] deadloaderが対応していない圧縮方式が使われています。
            [not_a_vpk] { $entry }は有効な.vpkファイルではありません。
           *[other] 破損しているか、対応していない形式です。
        }
    .todo = このMODの別のファイルを試すか、作者に問題を報告してください。
e9 = アーカイブが大きすぎるため、安全に展開できません。
    .why =
        { $reason ->
            [too_large] 展開後のサイズが{ $limit }を超えます。
            [too_many_files] MODファイルが{ $found }個含まれていますが、上限は{ $limit }個です。
            [ratio] { $entry }の圧縮率が不自然に高すぎます（{ $limit }倍超）。
            [memory] 展開に{ $needed }のメモリが必要ですが、上限は{ $limit }です。
           *[other] アーカイブの安全上の上限のいずれかを超えています。
        }
    .todo =
        { $reason ->
            [memory] このMODの別のファイルを試してください。
           *[other] 信頼できるアーカイブであれば、設定 → ダウンロードで上限を引き上げてください。
        }
e10 = このアーカイブにはMODファイルがありません。
    .why =
        { $listing ->
            [files] .vpkファイルは含まれておらず、{ $contents }のみです。
           *[empty] アーカイブが空です。
        }
    .todo = このファイルは、deadloaderでインストールできる形式のDeadlock MODではありません。
e11 = このPCではRARアーカイブを展開できません。
    .why = Windows標準のツールではこのRARを読み取れず、7-ZipもWinRARも見つかりませんでした。
    .todo = 7-Zip（無料）をインストールしてから、もう一度お試しください。ダウンロードしたファイルは保持されます。
e12 = ディスクの空き容量が不足しています。
    .why =
        { $reason ->
            [space] { $needed }必要ですが、空き容量は{ $available }しかありません。
           *[full] ディスクがいっぱいです。
        }
    .todo = 空き容量を増やすか、ライブラリで古いMODバージョンを削除してから、もう一度お試しください。
e13 = Deadlockの実行中は変更を適用できません。
    .why = ゲームの実行中は、Windowsがゲームのファイルをロックします。
    .todo = Deadlockを終了してください。終了後すぐにdeadloaderが変更を適用します。
e14 = ライブラリ内のMODファイルが見つからないか、破損しています。
    .why = deadloaderの外部でファイルが変更または削除されました。
    .todo = MODを修復してファイルを再ダウンロードするか、MODを削除してください。
e15 = リンクを開けませんでした。
    .why = Windowsで既定のブラウザーが設定されていません。
    .todo = リンクをコピーして、ご自身で開いてください。
e16 = リクエストをブロックしました。
    .why = GameBananaが{ $host }にリダイレクトしましたが、このホストは許可されたホストの一覧にありません。
    .todo = 対処は不要です。deadloaderはGameBananaとだけ通信します。このホストを信頼できる場合は、設定 → 詳細設定で追加してください。
e17 = プロファイルをインポートできませんでした。
    .why =
        { $reason ->
            [newer] 新しいバージョンのdeadloaderで作成されたプロファイルです。
            [unreadable] ファイルを読み取れません。移動または削除されたか、別のプログラムで開かれている可能性があります。
           *[not_a_profile] このファイルはdeadloaderのプロファイルではありません。
        }
    .todo =
        { $reason ->
            [newer] deadloaderをアップデートしてから、もう一度インポートしてください。
            [unreadable] ファイルが存在し、ほかのプログラムで開かれていないことを確認してから、もう一度お試しください。
           *[not_a_profile] 正しいファイルを選んだか確認してください。
        }
e18 = ゲームフォルダー内のファイルを変更できませんでした。
    .why =
        { $reason ->
            [access_denied] Windowsがゲームフォルダーへのアクセスを拒否しました。
            [in_use] ウイルス対策ソフトや別のMODマネージャーなど、ほかのプログラムがファイルを使用しています。
           *[other] Windowsがファイルを変更できませんでした。
        }
    .todo = ほかのプログラムを閉じてから、もう一度お試しください。
e19 = 画像を読み込めませんでした。
    .why =
        { $reason ->
            [no_response] ホストが応答しませんでした。
            [not_an_image] このファイルは画像ではありません。
            [too_large] 画像が大きすぎます。
           *[other] ホストから使用できる画像が送られてきませんでした。
        }
    .todo = プレースホルダーはそのまま表示されます。もう一度試すこともできます。
e20 = 有効なMODが多すぎます。
    .why = ゲームが追加で読み込めるMODファイルは{ $free }個ですが、このプロファイルには{ $wanted }個必要です。
    .todo = いくつかのMODを無効にするか、アドオンの数を減らしてください。
e21 = 「{ $name }」という名前のプロファイルはすでにあります。
    .why = ファイル名と同じく、名前は大文字と小文字を区別せずに比較されます。
    .todo = 別の名前を選んでください。
e22 = 「{ $name }」はプロファイル名に使用できません。
    .why = Windowsではファイル名として使用できない名前です（例: CON、末尾のピリオド、\ / : * ? " < > | のいずれかの文字）。
    .todo = 別の名前を選んでください。
e23 = deadloaderが自身のファイル{ $file }を読み取れません。
    .why =
        { $reason ->
            [damaged_at] ファイルが破損しています（{ $line }行目、{ $column }列目）。
            [damaged] ファイルが破損しています。
            [newer] 新しいバージョンのdeadloaderで作成されたファイルです。
           *[invalid] ファイルの内容が、deadloaderが想定しているものと異なります。
        }
    .todo =
        { $kind ->
            [settings] 当面は既定の設定を使用し、ファイルはそのまま残します。
            [manifest] manifest.json.bakを復元するか、deadloaderをアップデートしてください。
           *[other] バックアップからファイルを復元するか、deadloaderをアップデートしてください。
        }
internal = deadloaderの内部で問題が発生しました。
    .why = deadloader側の不具合であり、操作ミスによるものではありません。
    .todo = もう一度お試しください。繰り返し発生する場合は、[詳細]にあるログエントリを添えて報告してください。

## Buttons named in the texts above

action-undo = 元に戻す
action-review = 確認
action-retry = 再試行
action-retry-now = 今すぐ再試行
action-browse = 参照…
action-verify-in-steam = Steamで確認
action-show-differences = 差分を表示
action-details = 詳細
action-repair = 修復
action-copy-link = リンクをコピー
action-open-folder = フォルダーを開く
action-open-release-page = リリースページを開く
action-enable-catalog = カタログを有効にする
action-learn-more = 詳しく見る
action-view-log = ログを表示

## Shared pieces

log-entry = ログエントリ { $id }
list-more = ほか{ $count }件

## Sizes, labelled like Windows Explorer labels them

size-bytes = { $count } バイト
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
