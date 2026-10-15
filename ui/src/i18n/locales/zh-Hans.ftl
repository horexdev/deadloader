## Notifications

n1 = Deadlock 已更新，模组已恢复。
    .why = 更新重置了 gameinfo.gi，因此 deadloader 重新添加了它的那一行。
n2 = 已安装并启用 { $name }。
n3 = 有 { $count } 个模组更新可用。
n4 = Deadlock 正在运行。
    .why = 关闭游戏后，更改将自动应用。
    .pending = { $count } 项更改待应用
n5 =
    { $reason ->
        [completed] 一项被中断的更改已完成。
       *[undone] 一项被中断的更改已撤销，你的游戏文件夹保持原样。
    }
n6 = 在游戏的模组文件夹中发现了 { $count } 个不是由 deadloader 创建的文件。
n7 =
    { $when ->
        [today] 离线：正在显示 { $saved } 保存的目录。
       *[earlier] 离线：正在显示 { $saved } 保存的目录。
    }
    .why = 恢复联网后，下载将继续。
n8 = 已应用配置方案“{ $name }”，启用了 { $count } 个模组。
n9 = deadloader 有新版本可用：{ $version }。
n10 = 本地模式已开启。
    .why = 目录已关闭，deadloader 不会发出任何网络请求。
n11 = 由于 deadloader 和游戏位于不同的驱动器上，模组将被复制而不是链接。
    .why = 这会额外占用 { $size } 磁盘空间。
n12 = Deadlock 已关闭，已应用 { $count } 项更改。

## Errors: what happened, why, what to do

e1 = deadloader 无法保存其数据。
    .why =
        { $reason ->
            [access_denied] deadloader.exe 所在的文件夹是只读的——例如，Windows 不允许程序写入 Program Files。
            [in_use] 另一个程序（通常是杀毒软件）打开了 deadloader 的某个文件。
           *[other] Windows 无法写入 deadloader.exe 所在的文件夹。
        }
    .todo =
        { $reason ->
            [in_use] 请关闭其他程序后重试。
           *[other] 请将 deadloader 移动到你有写入权限的文件夹，例如 D:\Games\deadloader。
        }
e2 = 未找到 Deadlock。
    .why = 未安装 Steam 或游戏，或者游戏文件夹已被移动。
    .todo = 请手动选择游戏文件夹。
e3 = 游戏文件已被其他程序修改。
    .why = gameinfo.gi 与原始文件不一致——通常是 Deadlock Mod Manager 修改的。deadloader 不会覆盖其他程序所做的更改。
    .todo = 请在 Steam 中验证游戏文件的完整性，之后 deadloader 会添加它的那一行。
e4 = 目录不可用。
    .why = GameBanana 更改了其 API，deadloader 无法读取。你的库和配置方案仍可正常使用。
    .todo = 请检查 deadloader 是否有更新，或稍后重试。
e5 = GameBanana 繁忙。
    .why =
        { $reason ->
            [rate_limited] 短时间内请求过多（HTTP 429）。
            [server] GameBanana 服务器报告了错误。
           *[other] 请求过多或服务器出错。
        }
    .todo =
        { $wait ->
            [seconds] deadloader 将在 { $retry_in_s } 秒后自动重试。
           *[later] deadloader 稍后会自动重试。
        }
e6 = 下载失败。
    .why =
        { $reason ->
            [local_mode] deadloader 处于本地模式，因此不会发出任何网络请求。
            [dns] 找不到下载服务器的地址。
            [connect] deadloader 无法连接到下载服务器。
            [tls] 无法与下载服务器建立安全连接。
            [timeout] 服务器响应超时。
            [connection_lost] 连接已断开。
            [http_status] 服务器返回了错误（HTTP { $status }）。
           *[other] 下载已中断。
        }
    .todo =
        { $reason ->
            [local_mode] 请在“设置”中启用目录以下载模组。
           *[other] 下载将从中断处继续。
        }
e7 = 下载的文件已损坏。
    .why =
        { $reason ->
            [size] 其大小与 GameBanana 上列出的不一致。
           *[checksum] 其校验和与 GameBanana 上列出的不一致。
        }
    .todo = 请重新下载。
e8 = 无法解压该压缩包。
    .why =
        { $reason ->
            [not_an_archive] 该文件不是 ZIP、7z 或 RAR 压缩包——下载可能不完整。
            [damaged] 压缩包已损坏。
            [encrypted] 压缩包受密码保护。
            [unsupported] 它使用了 deadloader 不支持的压缩方式。
            [not_a_vpk] { $entry } 不是有效的 .vpk 文件。
           *[other] 它已损坏或使用了不支持的格式。
        }
    .todo = 请尝试该模组的其他文件，或向作者报告此问题。
e9 = 压缩包过大，无法安全解压。
    .why =
        { $reason ->
            [too_large] 解压后超过 { $limit }。
            [too_many_files] 其中包含 { $found } 个模组文件，而上限为 { $limit } 个。
            [ratio] { $entry } 的压缩比高得可疑——超过 { $limit } 倍。
            [memory] 解压需要 { $needed } 内存，而上限为 { $limit }。
           *[other] 它超出了压缩包的某项安全限制。
        }
    .todo =
        { $reason ->
            [memory] 请尝试该模组的其他文件。
           *[other] 如果你信任它，请在“设置 → 下载”中提高上限。
        }
e10 = 此压缩包中没有模组文件。
    .why =
        { $listing ->
            [files] 其中没有 .vpk 文件，只有 { $contents }。
           *[empty] 压缩包是空的。
        }
    .todo = 此文件不是 deadloader 可以安装的格式的 Deadlock 模组。
e11 = 无法在此电脑上解压 RAR 压缩包。
    .why = Windows 自带的工具无法读取此 RAR，也未找到 7-Zip 或 WinRAR。
    .todo = 请安装 7-Zip（免费）后重试；已下载的文件会保留。
e12 = 磁盘空间不足。
    .why =
        { $reason ->
            [space] 需要 { $needed }，但只有 { $available } 可用。
           *[full] 磁盘已满。
        }
    .todo = 请释放一些空间，或在“库”中删除旧版本的模组，然后重试。
e13 = Deadlock 正在运行，无法应用更改。
    .why = 游戏运行时，Windows 会锁定游戏文件。
    .todo = 请关闭 Deadlock；deadloader 会在关闭后立即应用更改。
e14 = 库中的某个模组文件丢失或已损坏。
    .why = 它在 deadloader 之外被修改或删除。
    .todo = 请修复该模组以重新下载文件，或移除该模组。
e15 = 无法打开链接。
    .why = Windows 中未设置默认浏览器。
    .todo = 请复制链接并自行打开。
e16 = 请求已被阻止。
    .why =
        { $reason ->
            [not_https] 服务器重定向到了不使用 HTTPS 的地址，deadloader 从不跟随此类重定向。
            [bad_redirect] 服务器发送了 deadloader 无法跟随的重定向。
            [too_many_redirects] 服务器重定向次数过多。
           *[not_listed] { $host } 不在允许的主机列表中，因此 deadloader 没有连接到它。
        }
    .todo =
        { $reason ->
            [not_listed] 无需任何操作——deadloader 只与 GameBanana 通信。如果你信任此主机，请在“设置 → 高级”中添加它。
           *[not_https] 无需任何操作——deadloader 只跟随安全的重定向。请稍后重试。
        }
e17 = 无法导入配置方案。
    .why =
        { $reason ->
            [newer] 它由更新版本的 deadloader 创建。
            [unreadable] 无法读取该文件——它可能已被移动、删除，或正被其他程序打开。
           *[not_a_profile] 此文件不是 deadloader 配置方案。
        }
    .todo =
        { $reason ->
            [newer] 请更新 deadloader，然后重新导入配置方案。
            [unreadable] 请确认该文件仍然存在且未被其他程序打开，然后重试。
           *[not_a_profile] 请确认你选择的文件是否正确。
        }
e18 = 无法更改游戏文件夹中的文件。
    .why =
        { $reason ->
            [access_denied] Windows 拒绝访问游戏文件夹。
            [in_use] 某个文件正被其他程序使用，例如杀毒软件或其他模组管理器。
           *[other] Windows 无法更改这些文件。
        }
    .todo = 请关闭其他程序后重试。
e19 = 无法加载图片。
    .why =
        { $reason ->
            [no_response] 主机没有响应。
            [not_an_image] 该文件不是图片。
            [too_large] 图片过大。
           *[other] 主机没有发送可用的图片。
        }
    .todo = 占位图会保留；你可以重试。
e20 = 启用的模组过多。
    .why = 游戏还能加载 { $free } 个模组文件，而此配置方案需要 { $wanted } 个。
    .todo = 请关闭部分模组，或少选一些附加内容。
e21 = 名为“{ $name }”的配置方案已存在。
    .why = 与文件名一样，比较名称时不区分大小写。
    .todo = 请换一个名称。
e22 = “{ $name }”不能用作配置方案名称。
    .why = Windows 不允许将其用作文件名——例如 CON、以句点结尾，或包含 \ / : * ? " < > | 中的任一字符。
    .todo = 请换一个名称。
e23 = deadloader 无法读取其文件 { $file }。
    .why =
        { $reason ->
            [damaged_at] 文件已损坏（第 { $line } 行，第 { $column } 列）。
            [damaged] 文件已损坏。
            [newer] 它由更新版本的 deadloader 创建。
           *[invalid] 其内容与 deadloader 预期的不符。
        }
    .todo =
        { $kind ->
            [settings] deadloader 暂时使用默认设置，并保留该文件不做改动。
            [manifest] 请恢复 manifest.json.bak，或更新 deadloader。
           *[other] 请从备份中恢复该文件，或更新 deadloader。
        }
internal = deadloader 内部出现了问题。
    .why = 这是 deadloader 的错误，并非你的操作导致。
    .todo = 请重试。如果问题再次出现，请附上“详细信息”中的日志条目进行报告。

## Buttons named in the texts above

action-undo = 撤销
action-review = 查看
action-retry = 重试
action-retry-now = 立即重试
action-browse = 浏览…
action-verify-in-steam = 在 Steam 中验证
action-show-differences = 显示差异
action-details = 详细信息
action-repair = 修复
action-copy-link = 复制链接
action-open-folder = 打开文件夹
action-open-release-page = 打开发布页面
action-enable-catalog = 启用目录
action-learn-more = 了解详情
action-view-log = 查看日志

## Shared pieces

log-entry = 日志条目 { $id }
list-more = 另外 { $count } 个

## Sizes, labelled like Windows Explorer labels them

size-bytes = { $count } 字节
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
