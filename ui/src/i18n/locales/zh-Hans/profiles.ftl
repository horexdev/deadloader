# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = 配置方案
profiles-count =
    { $count ->
       *[other] { $count } 个配置方案
    }
profiles-in-game = 当前在游戏中：{ $list }
profiles-in-game-item = { $drive } 上的“{ $name }”
profiles-in-game-none = 当前没有配置方案在游戏中。
profiles-import = 导入…
profiles-import-title = 从 .json 文件导入配置方案
profiles-import-dialog = 导入配置方案
profiles-export-dialog = 导出配置方案
profiles-file-filter = deadloader 配置方案
profiles-new = 新建配置方案
profiles-selected = 已选择
profiles-applied = 已应用
profiles-not-applied = 尚未应用
profiles-selected-title = 你在库中编辑并用来游玩的配置方案
profiles-pending-title = 点击“应用”或“开始游戏”会将它放入游戏文件夹
profiles-applied-title = 它的文件现在位于 { $drive } 上的游戏文件夹中
profiles-select = 选择
profiles-select-of = 选择“{ $name }”
profiles-more-of = “{ $name }”的更多操作
profiles-rename = 重命名
profiles-duplicate = 复制
profiles-export = 导出为文件…
profiles-change-installation = 更改游戏安装位置…
profiles-delete = 删除
profiles-delete-confirm = 删除…
profiles-delete-last = deadloader 至少需要一个配置方案。
profiles-name-label = 配置方案名称
profiles-save = 保存
profiles-cancel = 取消
profiles-close = 关闭
profiles-close-esc = 关闭 (Esc)
profiles-rename-hint = 按 Enter 保存 · 按 Esc 取消
profiles-name-empty = 请输入配置方案名称。
profiles-mods =
    { $count ->
       *[other] 已启用 { $count } 个模组
    }
profiles-mods-none = 未启用模组
profiles-applied-when = { $when }应用
profiles-last-applied = 上次应用：{ $when }
profiles-never-applied = 从未应用
profiles-just-now = 刚刚
profiles-preview-empty = 选择此配置方案后，在库中启用模组。
profiles-footnote = 配置方案是已启用模组及其顺序、变体和版本的列表。选择它不会下载或删除任何内容；“应用”只会更改不同的文件。
profiles-copy-name = { $name }（副本）
profiles-copy-name-n = { $name }（副本 { $number }）
profiles-new-title = 新建配置方案
profiles-start-with = 起始内容
profiles-empty = 空白
profiles-empty-hint = 未启用模组 — 之后在库中挑选。
profiles-copy-of = “{ $name }”的副本
profiles-copy-hint = { $mods } — 顺序、变体和版本都相同。
profiles-installation = 游戏安装位置
profiles-installation-hint = 与“{ $name }”相同。之后可以在配置方案的菜单中更改。
profiles-new-note = 新建配置方案不会选择或应用它 — 游戏不会改变。
profiles-create = 创建配置方案
profiles-pending-what = 配置方案“{ $name }”尚未进入游戏。
profiles-pending-why-other = { $drive } 上的游戏文件夹中仍是“{ $other }”。现在应用，或点击“开始游戏” — deadloader 会在启动前应用。
profiles-pending-why-empty = { $drive } 上的游戏文件夹中目前没有来自 deadloader 的模组。现在应用，或点击“开始游戏” — deadloader 会在启动前应用。
profiles-keep = 保留“{ $name }”
profiles-apply = 应用“{ $name }”
profiles-other-what = “{ $name }”使用另一个游戏安装位置。
profiles-other-why = “应用”和“开始游戏”使用 { $path }。{ $drive } 上的游戏保留“{ $other }”。
profiles-other-why-empty = “应用”和“开始游戏”使用 { $path }。{ $drive } 上的游戏不会改变。
profiles-delete-title = 删除配置方案“{ $name }”？
profiles-delete-applied =
    { $count ->
       *[other] “{ $name }”当前在游戏中。deadloader 会从游戏文件夹中移除它的 { $count } 个模组：
    }
profiles-delete-applied-empty = “{ $name }”当前在游戏中，但没有模组，因此游戏文件夹不会改变。
profiles-delete-stay = 模组会保留在你的库和其他配置方案中。在你于该处应用其他配置方案之前，Deadlock 启动时不会带有 deadloader 的模组。
profiles-delete-next = “{ $name }”将成为已选择的配置方案。在你点击“应用”或“开始游戏”之前不会应用。
profiles-delete-undoable = 此操作无法撤销。如需保留副本，请先导出配置方案。
profiles-export-first = 先导出
profiles-delete-profile = 删除配置方案
profiles-installation-title = “{ $name }”的游戏安装位置
profiles-installation-lead = 选择此配置方案应用到的 Deadlock 文件夹。
profiles-used-by =
    { $count ->
       *[other] 被 { $count } 个配置方案使用
    }
profiles-used-by-none = 未被任何配置方案使用
profiles-installation-applied = “{ $name }”当前在 { $drive } 上的游戏中。在你于 { $drive } 上应用其他配置方案之前，它的文件会一直留在那里。
profiles-installation-not-applied = 在你应用“{ $name }”之前，两个游戏文件夹都不会改变。
profiles-manage-installations = 在“设置”中管理安装位置
profiles-use-installation = 使用此安装位置
profiles-d9-title =
    { $count ->
       *[other] “{ $name }”中有 { $count } 个模组不在你的库中
    }
profiles-d9-file = 文件
profiles-d9-lead = 列出了 { $listed } 个模组。其中 { $present } 个已在你的库中；以下 { $count } 个不在：
profiles-d9-list = 缺少的模组
profiles-d9-from-gamebanana = 来自 GameBanana 的 { $count } 个
profiles-d9-local-count = 本地 { $count } 个
profiles-gamebanana = GameBanana
profiles-can-download = 可以下载
profiles-local-mod = 本地模组
profiles-cant-download = 无法下载
profiles-local-why = 它是在另一台电脑上从磁盘添加的，因此不在 GameBanana 上。请向分享配置方案的人索要文件。
profiles-d9-local-mode = 目录已关闭（本地模式），因此 deadloader 无法下载模组。请先不带它们导入，之后从磁盘添加 — 或在“设置”中打开目录。
profiles-d9-after = 无论哪种方式，“{ $name }”都会成为已选择的配置方案，但不会被应用 — 在你点击“应用”或“开始游戏”之前，游戏文件夹不会改变。
profiles-import-without = 不带它们导入
profiles-download-missing = 下载缺少的模组 ({ $count })
profiles-turned-off =
    { $count ->
       *[other] 已关闭 { $count } 个模组
    }
profiles-turned-on =
    { $count ->
       *[other] 已开启 { $count } 个模组
    }
profiles-differ = 只更改了不同的文件。
profiles-same = 相同的模组已在游戏中；只检查了顺序和变体。
profiles-deleted = 已删除配置方案“{ $name }”。
profiles-deleted-not-applied = 它未被应用，因此游戏文件夹没有改变。
profiles-deleted-applied =
    { $count ->
       *[other] 已从游戏文件夹移除 { $count } 个模组。它们仍在你的库中。
    }
profiles-exported = 已导出配置方案“{ $name }”。
profiles-imported = 已导入配置方案“{ $name }”。
profiles-imported-text = 它已被选择，但尚未应用。
profiles-imported-without =
    { $count ->
       *[other] 已导入配置方案“{ $name }”，缺少 { $count } 个模组。
    }
profiles-imported-without-text = 它已被选择，但尚未应用。你可以之后再添加缺少的模组。
profiles-imported-downloading =
    { $count ->
       *[other] 已导入配置方案“{ $name }” — 正在下载 { $count } 个模组。
    }
profiles-imported-downloading-text = 它已被选择，但尚未应用。下载完成后再应用。
profiles-open-downloads = 打开下载
profiles-undo = 撤销
profiles-check-update = 检查 deadloader 更新
profiles-choose-another = 选择其他文件
profiles-switcher = 管理配置方案
profiles-saved-as = 已保存为
profiles-show-folder = 在文件夹中显示
