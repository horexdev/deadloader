# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = 设置
settings-nav = 设置分区
settings-reset-nav = 重置所有设置…
settings-default = 默认：{ $value }
settings-restore = 恢复默认
settings-on = 开
settings-off = 关
settings-cancel = 取消
settings-remove = 移除
settings-remove-item = 移除 { $item }
settings-add = 添加
settings-open-folder = 打开文件夹
settings-open-folder-at = 打开文件夹：{ $path }
settings-copied = 已复制到剪贴板。
settings-folder = 文件夹
settings-example = 示例：

settings-section-general = 常规
settings-section-general-lead = 语言、启动和缩放。
settings-section-game = 游戏
settings-section-game-lead = Deadlock 安装在哪里，以及 deadloader 可以在那里更改什么。
settings-section-catalog = 目录
settings-section-catalog-lead = GameBanana 目录、敏感内容、图片和链接。
settings-section-library = 库
settings-section-library-lead = 模组更新、删除模组和旧版本。
settings-section-downloads = 下载
settings-section-downloads-lead = 下载队列、压缩包和解压。
settings-section-network = 网络
settings-section-network-lead = deadloader 如何与 GameBanana 通信。
settings-section-cache = 缓存
settings-section-cache-lead = 保存在这台电脑上的缩略图和目录页面。
settings-section-logs = 日志
settings-section-logs-lead = “活动”保留哪些内容，保留多久。
settings-section-advanced = 高级
settings-section-advanced-lead = 设置文件和额外允许的域名。
settings-section-about = 关于 deadloader
settings-section-about-lead = 版本、许可证、如何验证此下载以及你的数据在哪里。

# General
settings-general-group = 语言和启动
settings-language = 语言
settings-language-help = 系统语言：如果 deadloader 支持，则使用 Windows 显示语言，否则使用英语。
settings-language-system = 系统语言（{ $language }）
settings-startup-profile = 启动时的配置
settings-startup-profile-help = deadloader 启动时选中的配置。选中并不会应用它。
settings-startup-last = 上次选中的（{ $name }）
settings-startup-last-plain = 上次选中的
settings-app-updates = 启动时检查 deadloader 更新
settings-app-updates-help = 启动时 deadloader 会向 GitHub（api.github.com）询问一次是否有新版本。更新始终需要手动进行。这是你在首次运行时的选择。
settings-app-updates-local = 不可用：本地模式完全不发送网络请求。开启目录后即可使用（设置 → 目录）。
settings-zoom = 缩放
settings-zoom-keys-1 = 也可用 Ctrl + 鼠标滚轮缩放，
settings-zoom-keys-2 = 缩放，
settings-zoom-keys-3 = 重置。
settings-ctrl = Ctrl
settings-percent = { $value }%

# Game
settings-installs-group = 游戏安装
settings-installs-lead = deadloader 会自动找到 Steam 中的 Deadlock 安装。每个配置使用其中一个——在“配置”中更改。
settings-install-library = Steam 库 { $library }
settings-install-build = 版本号 { $build }
settings-install-used =
    { $count ->
        [0] 没有配置使用
       *[other] { $count } 个配置在使用：{ $names }
    }
settings-install-selected = { $name }（已选中）
settings-install-applied = { $name }（已在此应用）
settings-install-steam = 通过 Steam 找到
settings-install-manual = 手动添加
settings-install-known = 这里还有 deadloader 的文件
settings-install-manual-meta = 手动添加
settings-gi-ours = gameinfo.gi：正常——deadloader 的那一行在
settings-gi-wait = gameinfo.gi：有配置使用时再设置
settings-gi-rolled = gameinfo.gi：原始——“应用”会重新添加 deadloader 的那一行
settings-gi-foreign = gameinfo.gi：已被其他程序修改
settings-gi-missing = gameinfo.gi：未找到——文件夹可能已被移动
settings-installs-none = 尚未找到任何安装。
settings-add-manual = 手动添加…
settings-add-manual-hint = 选择一个未被找到的 Deadlock 文件夹。
settings-add-manual-title = 选择 Deadlock 文件夹
settings-install-added = 已添加安装。
settings-install-removed = 已从列表中移除安装。
settings-install-removed-text = 该文件夹中没有任何更改。
settings-gameinfo-group = 游戏更新后
settings-autofix = 自动修复 gameinfo.gi
settings-autofix-help = Deadlock 更新会重置告诉它加载模组的游戏文件。deadloader 会先备份，再放回自己的那一行并通知你。关闭时，会先用横幅询问你。
settings-danger-group = 撤销和移除
settings-rollback = 回滚所选配置的游戏文件夹
settings-rollback-help = 针对所选配置 { $profile } 的安装（{ $drive }）：移除 deadloader 放在那里的 { $files } 个模组文件，以及它在 gameinfo.gi 中的那一行。库和配置保留；“应用”会恢复一切。
settings-rollback-help-others = 其他游戏文件夹不受影响；要回滚它们，请选择使用它们的配置。
settings-rollback-nothing = { $drive } 上没有可回滚的内容——deadloader 目前在那里没有文件。
settings-rollback-no-game = 未找到 Deadlock，因此没有可回滚的游戏文件夹。
settings-rollback-button = 回滚…
settings-rolled-title = 已回滚 { $drive } 上的游戏文件夹。
settings-rolled-text =
    { $foreign ->
        [0] 已在 { $drive } 上移除 { $files } 个模组文件和 gameinfo.gi 中 deadloader 的那一行。库和配置已保留——“应用”或“开始游戏”会恢复模组。
       *[other] 已在 { $drive } 上移除 { $files } 个模组文件、gameinfo.gi 中 deadloader 的那一行和 { $foreign } 个外部文件。库和配置已保留——“应用”或“开始游戏”会恢复模组。
    }
settings-wipe = 删除所有 deadloader 数据
settings-wipe-help = 删除库、配置、日志、缓存和设置（{ $size }），然后关闭 deadloader。deadloader.exe 会保留——之后请自行删除。
settings-wipe-button = 删除所有数据…

# Catalog
settings-catalog-group = GameBanana 目录
settings-catalog = 使用目录
settings-catalog-help-on = 浏览并安装 GameBanana 上的模组。deadloader 只与 GameBanana 通信，每个请求都会列在“活动”中。关闭即为本地模式。
settings-catalog-help-off = 关闭——本地模式：deadloader 完全不发送网络请求。你仍然可以从磁盘添加模组。
settings-consent-title = 先完成一步。
settings-consent-text = 在开启目录之前，deadloader 会向你展示它将联系的主机以及 GameBanana 的条款。
settings-consent-go = 查看并开启
settings-local-on = 本地模式已开启。
settings-local-on-text = 目录已关闭，deadloader 不发送网络请求。
settings-ttl = 复用已加载目录页面的时长
settings-ttl-help = 在此时间内返回某个页面，不会再次询问 GameBanana。
settings-minutes = { $count } 分钟
settings-hours = { $count } 小时
settings-prefetch = 提前加载下一页
settings-prefetch-help = 当你滚动到约 70% 的结果时，会加载后面的结果，让网格持续延续。
settings-nsfw-group = 敏感内容
settings-nsfw = 带内容分级的模组（NSFW）
settings-nsfw-help = 隐藏：完全不显示。模糊：显示带“敏感内容”标签的模糊图片；点击后显示。显示：不模糊。
settings-nsfw-age-done = 已于 { $date } 确认年龄。
settings-nsfw-age-ask = 离开“隐藏”时会请你确认一次年龄。
settings-nsfw-hidden = 隐藏
settings-nsfw-blurred = 模糊
settings-nsfw-shown = 显示
settings-images-group = 图片和链接
settings-description-images = 模组描述中的图片
settings-description-images-help = 它们来自 GameBanana 以外的主机，例如 i.imgur.com。点击时：在你点击前显示占位符。
settings-images-click = 点击时
settings-images-always = 始终
settings-prechecked = 预先勾选“始终允许此主机的图片”
settings-prechecked-help = 用于描述图片首次来自新主机时显示的对话框。
settings-hosts = 允许的图片主机
settings-hosts-help = 这些主机的图片会直接加载，不再询问。在“活动”中，它们的请求标记为“允许的图片主机”。
settings-hosts-built-in = 内置
settings-hosts-none = 尚未添加主机——每个新主机都会先询问。
settings-host-removed = 已移除 { $host }
settings-host-removed-text = 它的图片在加载前会再次询问。
settings-undo = 撤销
settings-gif = 自动播放 GIF
settings-gif-help = 关闭：GIF 显示第一帧和播放按钮。
settings-links = 外部链接
settings-links-help = 链接会在默认浏览器中打开。“先询问”会在打开前显示完整地址。
settings-links-ask = 先询问
settings-links-open = 直接打开

# Library
settings-updates-group = 模组更新
settings-check-updates = 启动时检查模组更新
settings-check-updates-help = 将你来自 GameBanana 的模组与目录比较。本地模式下从不进行。
settings-auto-download = 自动下载更新
settings-auto-download-help = 仅下载——在模组切换到新版本前，deadloader 仍会询问。
settings-disk-group = 删除和旧版本
settings-deletion = 删除模组时
settings-deletion-help = 每次都可以在删除对话框中另作选择。
settings-deletion-permanent = 永久删除
settings-deletion-bin = 移到回收站
settings-old-versions = 旧版本
settings-old-versions-help = 每个下载过的版本都会保留，方便你切回。在模组详情中删除它的旧版本。
settings-old-keep-all = 全部保留

# Downloads
settings-queue-group = 队列
settings-parallel = 同时下载数
settings-parallel-help = 同时下载多少个文件。
settings-speed = 速度限制
settings-speed-help = 仅限模组下载——目录和缩略图不受限制。
settings-speed-none = 不限制
settings-speed-value = { $value } MB/s
settings-keep-archives = 安装后保留压缩包
settings-keep-archives-help = 关闭：一旦 .vpk 文件进入库，.zip、.7z 或 .rar 文件就会被删除。
settings-unpack-group = 解压
settings-limits = 解压限制
settings-limits-help = 防止压缩包解压成巨大的内容。如果你信任的压缩包被拒绝，请调高限制。
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = 解压后大小
settings-limit-ratio = 压缩比
settings-limit-files = 每个压缩包的模组文件数
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = 个文件
settings-unit-s = 秒
settings-number-error = { $field }：请输入 { $min } 到 { $max } 之间的整数。在此之前继续使用之前的值。

# Network
settings-network-local = 本地模式已开启。
settings-network-local-text = deadloader 不发送网络请求；开启目录后这些设置才会生效。
settings-requests-group = 发往 GameBanana 的请求
settings-api-parallel = 同时目录请求数
settings-api-parallel-help = 同时发往 GameBanana API 的请求数。图片和下载单独计算。
settings-timeouts = 超时
settings-timeouts-help = 连接：在此时间内未能连接则放弃。响应：服务器停止发送时放弃。
settings-timeouts-default = 10 秒 · 30 秒
settings-timeout-connect = 连接
settings-timeout-read = 响应
settings-retries = 重试
settings-retries-help = 失败的请求以逐渐增加的间隔重复多少次，之后 deadloader 才显示错误。
settings-connection-group = 连接
settings-proxy = 代理
settings-proxy-help = 无：直接连接。系统：Windows 中设置的代理。手动：在下方输入。
settings-proxy-none = 无
settings-proxy-system = 系统
settings-proxy-manual = 手动
settings-proxy-host = 地址
settings-proxy-port = 端口
settings-proxy-hint = HTTP 代理。只有 deadloader 自己的请求会经过它。
settings-proxy-invalid = 请输入不含用户名和密码的主机名或地址，例如 proxy.example.com。

# Cache
settings-thumbs-group = 缩略图和目录页面
settings-cache-limit = 缓存上限
settings-cache-limit-help = 缓存满后，最久没看过的缩略图会先被移除。
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = 当前大小
settings-cache-breakdown = { $thumbnails } 张缩略图 · { $pages } 个目录页面
settings-cache-empty = 空——尚未缓存任何内容。
settings-cache-used = { $used } / { $limit }
settings-cache-clear = 清除缓存
settings-cache-cleared = 已清除缓存——释放了 { $size }。缩略图会在需要时重新加载。
settings-cache-cleared-partly = { $failed } 个文件正在使用，会保留到下次清除。

# Logs
settings-keep-group = 日志保留
settings-log-days = 日志保留时长
settings-log-days-help = 更早的记录会在 deadloader 启动时删除。
settings-days = { $count ->
       *[other] { $count } 天
    }
settings-year = 1 年
settings-log-size = 最大大小
settings-log-size-help = 超过此大小时，最早的日志文件会先被删除。
settings-privacy-group = 隐私
settings-mask = 隐藏你的 Windows 用户名
settings-mask-help = 日志中的路径会显示 { $masked } 而不是你的名字——分享日志时更安全。
settings-logs-folder-group = 日志文件夹
settings-logs-folder = 日志位置
settings-logs-now = { $size } · { $files } 个文件 · 最早的来自 { $date }
settings-open-logs = 打开日志文件夹

# Advanced
settings-advanced-warning = 供有经验的用户使用。
settings-advanced-warning-text = 这里的错误可能改变 deadloader 的行为。它会在保存前检查文件，并保留上一版本的备份。
settings-files-group = 编辑设置文件
settings-file-settings = 设置
settings-file-manifest = 游戏文件夹清单
settings-file-label = 要编辑的文件
settings-file-settings-lead = 这些页面上的所有内容都在一个文件里。保存后，更改会显示在页面上。
settings-file-manifest-lead = deadloader 放入游戏文件夹的内容列表。回滚依赖它——错误的条目可能会留下文件。
settings-json-error = 这不是有效的 JSON，因此没有保存。
settings-json-where = 第 { $line } 行，第 { $column } 列：此处或紧挨着的前面缺少或多了些什么——通常是逗号或引号。
settings-json-fix = 修正该处后再次保存——在此之前，deadloader 继续使用当前文件。
settings-json-refused = 这些值无法使用，因此没有保存任何内容。
settings-json-newer = 此文件来自更新版本的 deadloader（格式 { $found }；此版本最多读取 { $supported }），因此没有保存。
settings-json-invalid = deadloader 无法使用此文件，因此没有保存。
settings-json-details = 详细信息
settings-problem-unknown-field = { $path }：不是此版本的设置。
settings-problem-invalid-value = { $path }：此值不能用在这里。
settings-problem-out-of-range = { $path }：{ $found } 不在 { $min }–{ $max } 范围内。
settings-problem-invalid-entry = { $path }：“{ $entry }”在此处无效。
settings-problem-duplicate-entry = { $path }：“{ $entry }”出现了两次。
settings-problem-needs-age = { $path }：显示 NSFW 内容需要先确认年龄（设置 → 目录）。
settings-problem-local-mode = { $path }：本地模式下不可用。
settings-json-idle = 保存时检查。
settings-json-saved = 有效并已保存。上一版本已保留为 { $backup }。
settings-json-saved-plain = 有效并已保存。
settings-json-save = 检查并保存
settings-json-discard = 放弃我的编辑
settings-json-backup = 保存前，上一个文件会保留为备份；保留最近 10 个。
settings-domains-group = 额外允许的域名
settings-domains-lead = deadloader 只与 GameBanana 通信。如果下载被重定向到其他主机并被阻止，你可以在这里允许该主机。在“活动”中，发往它的请求标记为“用户添加”。
settings-domains-built-in = 内置：
settings-domain-user = 用户添加
settings-domains-none = 没有额外的域名。
settings-domain-add = 添加域名
settings-domain-hint = 仅主机名，例如 files.example.com。
settings-domain-empty = 请先输入主机名，例如 files.example.com。
settings-domain-not-host = 只输入主机名——不要 https:// 或路径，例如 files.example.com。
settings-domain-invalid = “{ $host }”不是主机名。请检查拼写，例如 files.example.com。
settings-domain-built-in = { $host } 是内置的，始终允许。
settings-domain-listed = { $host } 已在列表中。

# About
settings-version = 版本 { $version }
settings-commit = 构建自提交
settings-commit-local = 本地构建——非 GitHub Actions 构建
settings-copy-commit = 复制提交 ID
settings-release-page = 发布页面
settings-licence-group = 许可证
settings-licence-help = deadloader 是自由软件：你可以按照 GPL-3 使用、研究、分享和修改它。它不附带任何担保。
settings-read-licence = 阅读许可证
settings-source = 源代码
settings-verify-group = 如何验证此下载
settings-verify-lead = 两项独立检查，确认这个 deadloader.exe 就是 GitHub 上发布的那个。
settings-verify-hash = 比较 SHA-256 校验和
settings-verify-hash-help = 它必须与发布页面上的校验和一致。被篡改的副本可能在这里显示假值，所以也请自己计算：
settings-verify-hash-cmd = 在 exe 所在文件夹的 PowerShell 中：
settings-verify-attest = 检查构建证明
settings-verify-attest-help = 证明此文件是由 GitHub Actions 从已发布的源代码构建的。需要 GitHub CLI：
settings-copy-hash = 复制校验和
settings-copy-command = 复制命令
settings-unsigned = deadloader.exe 没有代码签名。
settings-unsigned-text = 首次运行时，Windows SmartScreen 可能提示“Windows 已保护你的电脑”。上面的两项检查可以确认文件是真实的。
settings-webview-group = 窗口引擎
settings-webview-help = deadloader 的窗口由 Windows 的一部分 WebView2 绘制。Windows 会自行更新它。deadloader 关闭了：
settings-webview-off-1 = DNS-over-HTTPS 查询
settings-webview-off-2 = 窗口内的 Microsoft 帐户登录
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = 后台网络访问和组件更新
settings-webview-off-5 = 表单自动填充的服务器调用
settings-webview-off-6 = 发送给 Microsoft 的错误报告（domain reliability）
settings-webview-no-route = 而且窗口完全没有通往互联网的途径。
settings-webview-unknown = 版本未知
settings-data-group = 你的数据
settings-data-folder = 数据文件夹
settings-data-folder-help = deadloader 保存的一切都在 exe 旁边的这一个文件夹里。
settings-notice = 首次运行说明
settings-notice-help = 版本 { $version }，于 { $date } 以 { $language } 接受。你的选择：目录 { $catalog }，检查更新 { $updates }。
settings-notice-none = 尚未接受。
settings-notice-again = 再读一遍

# D7
settings-d7-title = 回滚 { $drive } 上的游戏文件夹？
settings-d7-from =
    { $others ->
        [0] 所选配置 { $profile } 的安装。deadloader 将从以下位置移除：
       *[other] 所选配置 { $profile } 的安装（{ $others } 也在使用）。deadloader 将从以下位置移除：
    }
settings-d7-files =
    { $count ->
       *[other] 它添加的 { $count } 个模组文件，位于
    }
settings-d7-line = 它在 gameinfo.gi 中的那一行
settings-d7-line-why = ——文件恢复为 Steam 安装时的样子。
settings-d7-stays = 保持不变：
settings-d7-library = 你的库——{ $mods } 个模组，{ $size }
settings-d7-profiles =
    { $count ->
       *[other] 你的 { $count } 个配置——{ $names }
    }
settings-d7-other = { $path } 上的安装（{ $profile }）——不受影响。要回滚它，请选择使用它的配置后再次回滚。
settings-d7-other-plain = { $path } 上的安装——不受影响。
settings-d7-back = “应用”或“开始游戏”会恢复一切。
settings-d7-foreign =
    { $count ->
       *[other] 同时移除 { $count } 个不是 deadloader 创建的文件
    }
settings-d7-foreign-why = 来自其他程序或手动安装。它们会被永久删除；deadloader 无法恢复。
settings-d7-confirm = 回滚
settings-d7-confirm-foreign =
    { $count ->
       *[other] 回滚并删除 { $count } 个文件
    }

# D8
settings-d8-title = 删除所有 deadloader 数据？
settings-d8-from = deadloader 将删除以下文件夹中的所有内容
settings-d8-from-2 = 然后关闭。
settings-d8-what = 项目
settings-d8-contains = 包含
settings-d8-size = 大小
settings-d8-total = 合计
settings-d8-library = 库
settings-d8-library-has = { $mods } 个模组，{ $files } 个文件
settings-d8-profiles = 配置
settings-d8-logs = 日志
settings-d8-logs-has = { $files } 个文件，自 { $date } 起
settings-d8-logs-none = 没有日志文件
settings-d8-cache = 缓存
settings-d8-cache-has = 缩略图和目录页面
settings-d8-settings = 设置
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
       *[other] 含 { $backups } 个备份
    }
settings-d8-other = 其他
settings-d8-other-has = 未完成的下载、gameinfo.gi 备份、窗口引擎数据
settings-d8-exe = 不会删除：deadloader.exe 本身——如果不再需要，请自行删除。
settings-d8-rollback = 先回滚 deadloader 使用的每个游戏文件夹（推荐）
settings-d8-rollback-why = 删除前从每个游戏文件夹中移除 deadloader 的模组文件和它在 gameinfo.gi 中的那一行。外部文件保留。
settings-d8-rollback-off = 未勾选：deadloader 的模组文件会留在游戏文件夹中，Deadlock 会继续加载它们——而再也没有 deadloader 来移除它们。
settings-d8-type-1 = 要确认，请输入
settings-d8-type-2 = { "" }
settings-d8-word = 删除
settings-d8-hint = 输入的词一致后，删除按钮才会解锁。
settings-d8-hint-ok = 已确认。此操作无法撤销。
settings-d8-type-first = 请先输入“{ $word }”
settings-d8-deleting = 正在删除…
settings-d8-confirm = 删除并关闭
settings-d8-confirm-rollback = 回滚、删除并关闭

# Reset
settings-reset-title = 重置所有设置？
settings-reset-body = 所有设置都会恢复默认值，包括允许的图片主机和额外的域名。
settings-reset-keeps = 你的库、配置和日志不受影响，游戏文件夹也保持不变。
settings-reset-backup = 当前设置的备份会保留为 settings.json.bak-…
settings-reset-confirm = 重置设置
settings-reset-done = 所有设置已恢复默认值。
settings-reset-done-text = 备份：{ $backup }
