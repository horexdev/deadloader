# Profiles (S5): the list, its dialogs (new, delete, game installation, D9), the import errors and the sidebar switcher.
profiles-title = Perfis
profiles-count =
    { $count ->
        [one] { $count } perfil
       *[other] { $count } perfis
    }
profiles-in-game = No jogo agora: { $list }
profiles-in-game-item = “{ $name }” em { $drive }
profiles-in-game-none = Nenhum perfil está no jogo agora.
profiles-import = Importar…
profiles-import-title = Importar um perfil de um arquivo .json
profiles-import-dialog = Importar um perfil
profiles-export-dialog = Exportar o perfil
profiles-file-filter = Perfis do deadloader
profiles-new = Novo perfil
profiles-selected = Selecionado
profiles-applied = Aplicado
profiles-not-applied = Ainda não aplicado
profiles-selected-title = O perfil que você edita na biblioteca e com o qual joga
profiles-pending-title = Aplicar ou Jogar coloca ele na pasta do jogo
profiles-applied-title = Os arquivos dele estão agora na pasta do jogo em { $drive }
profiles-select = Selecionar
profiles-select-of = Selecionar “{ $name }”
profiles-more-of = Mais ações para “{ $name }”
profiles-rename = Renomear
profiles-duplicate = Duplicar
profiles-export = Exportar como arquivo…
profiles-change-installation = Mudar a instalação do jogo…
profiles-delete = Excluir
profiles-delete-confirm = Excluir…
profiles-delete-last = O deadloader precisa de pelo menos um perfil.
profiles-name-label = Nome do perfil
profiles-save = Salvar
profiles-cancel = Cancelar
profiles-close = Fechar
profiles-close-esc = Fechar (Esc)
profiles-rename-hint = Enter salva · Esc cancela
profiles-name-empty = Digite um nome para o perfil.
profiles-mods =
    { $count ->
        [one] { $count } mod ativado
       *[other] { $count } mods ativados
    }
profiles-mods-none = Nenhum mod ativado
profiles-applied-when = Aplicado { $when }
profiles-last-applied = Aplicado pela última vez { $when }
profiles-never-applied = Nunca aplicado
profiles-just-now = agora mesmo
profiles-preview-empty = Selecione este perfil e depois ative mods na biblioteca.
profiles-footnote = Um perfil é uma lista de mods ativados com ordem, variantes e versões. Selecionar não baixa nem apaga nada; Aplicar muda só os arquivos que são diferentes.
profiles-copy-name = { $name } (cópia)
profiles-copy-name-n = { $name } (cópia { $number })
profiles-new-title = Novo perfil
profiles-start-with = Começar com
profiles-empty = Vazio
profiles-empty-hint = Nenhum mod ativado — escolha depois na biblioteca.
profiles-copy-of = Cópia de “{ $name }”
profiles-copy-hint = { $mods } — mesma ordem, variantes e versões.
profiles-installation = Instalação do jogo
profiles-installation-hint = A mesma de “{ $name }”. Você pode mudar depois no menu do perfil.
profiles-new-note = Criar um perfil não seleciona nem aplica ele — o jogo não muda.
profiles-create = Criar perfil
profiles-pending-what = O perfil “{ $name }” ainda não está no jogo.
profiles-pending-why-other = A pasta do jogo em { $drive } ainda tem “{ $other }”. Aplique agora ou aperte Jogar — o deadloader aplica antes de iniciar.
profiles-pending-why-empty = A pasta do jogo em { $drive } não tem mods do deadloader agora. Aplique agora ou aperte Jogar — o deadloader aplica antes de iniciar.
profiles-keep = Manter “{ $name }”
profiles-apply = Aplicar “{ $name }”
profiles-other-what = “{ $name }” usa outra instalação do jogo.
profiles-other-why = Aplicar e Jogar usam { $path }. O jogo em { $drive } mantém “{ $other }”.
profiles-other-why-empty = Aplicar e Jogar usam { $path }. O jogo em { $drive } não muda.
profiles-delete-title = Excluir o perfil “{ $name }”?
profiles-delete-applied =
    { $count ->
        [one] “{ $name }” está no jogo agora. O deadloader vai remover o { $count } mod dele da pasta do jogo:
       *[other] “{ $name }” está no jogo agora. O deadloader vai remover os { $count } mods dele da pasta do jogo:
    }
profiles-delete-applied-empty = “{ $name }” está no jogo agora, mas não tem mods, então a pasta do jogo não muda.
profiles-delete-stay = Os mods continuam na sua biblioteca e em outros perfis. Até você aplicar outro perfil ali, o Deadlock inicia sem mods do deadloader.
profiles-delete-next = “{ $name }” passa a ser o perfil selecionado. Ele só é aplicado quando você aperta Aplicar ou Jogar.
profiles-delete-undoable = Isso não pode ser desfeito. Para guardar uma cópia, exporte o perfil antes.
profiles-export-first = Exportar antes
profiles-delete-profile = Excluir perfil
profiles-installation-title = Instalação do jogo para “{ $name }”
profiles-installation-lead = Escolha a pasta do Deadlock em que este perfil é aplicado.
profiles-used-by =
    { $count ->
        [one] usada por { $count } perfil
       *[other] usada por { $count } perfis
    }
profiles-used-by-none = não usada por nenhum perfil
profiles-installation-applied = “{ $name }” está no jogo em { $drive } agora. Os arquivos dele ficam lá até você aplicar outro perfil em { $drive }.
profiles-installation-not-applied = Nenhuma pasta do jogo muda até você aplicar “{ $name }”.
profiles-manage-installations = Gerenciar instalações nas Configurações
profiles-use-installation = Usar esta instalação
profiles-d9-title =
    { $count ->
        [one] { $count } mod de “{ $name }” não está na sua biblioteca
       *[other] { $count } mods de “{ $name }” não estão na sua biblioteca
    }
profiles-d9-file = O arquivo
profiles-d9-lead = lista { $listed } mods. { $present } já estão na sua biblioteca; estes { $count } não:
profiles-d9-list = Mods que faltam
profiles-d9-from-gamebanana = { $count } do GameBanana
profiles-d9-local-count = { $count } local(is)
profiles-gamebanana = GameBanana
profiles-can-download = Pode ser baixado
profiles-local-mod = Mod local
profiles-cant-download = Não pode ser baixado
profiles-local-why = Foi adicionado do disco em outro PC, então não está no GameBanana. Peça o arquivo a quem compartilhou o perfil.
profiles-d9-local-mode = O catálogo está desligado (modo local), então o deadloader não pode baixar mods. Importe sem eles e depois adicione do disco — ou ligue o catálogo nas Configurações.
profiles-d9-after = De qualquer forma, “{ $name }” passa a ser o seu perfil selecionado, mas não é aplicado — a pasta do jogo só muda quando você aperta Aplicar ou Jogar.
profiles-import-without = Importar sem eles
profiles-download-missing = Baixar os que faltam ({ $count })
profiles-turned-off =
    { $count ->
        [one] { $count } mod desativado
       *[other] { $count } mods desativados
    }
profiles-turned-on =
    { $count ->
        [one] { $count } mod ativado
       *[other] { $count } mods ativados
    }
profiles-differ = Só os arquivos diferentes foram alterados.
profiles-same = Os mesmos mods já estavam no jogo; só a ordem e as variantes foram conferidas.
profiles-deleted = Perfil “{ $name }” excluído.
profiles-deleted-not-applied = Ele não estava aplicado, então a pasta do jogo não mudou.
profiles-deleted-applied =
    { $count ->
        [one] { $count } mod foi removido da pasta do jogo. Ele continua na sua biblioteca.
       *[other] { $count } mods foram removidos da pasta do jogo. Eles continuam na sua biblioteca.
    }
profiles-exported = Perfil “{ $name }” exportado.
profiles-imported = Perfil “{ $name }” importado.
profiles-imported-text = Ele está selecionado, mas ainda não aplicado.
profiles-imported-without =
    { $count ->
        [one] Perfil “{ $name }” importado sem { $count } mod.
       *[other] Perfil “{ $name }” importado sem { $count } mods.
    }
profiles-imported-without-text = Ele está selecionado, mas ainda não aplicado. Você pode adicionar os mods que faltam depois.
profiles-imported-downloading =
    { $count ->
        [one] Perfil “{ $name }” importado — { $count } mod está sendo baixado.
       *[other] Perfil “{ $name }” importado — { $count } mods estão sendo baixados.
    }
profiles-imported-downloading-text = Ele está selecionado, mas ainda não aplicado. Aplique quando os downloads terminarem.
profiles-open-downloads = Abrir downloads
profiles-undo = Desfazer
profiles-check-update = Procurar atualização do deadloader
profiles-choose-another = Escolher outro arquivo
profiles-switcher = Gerenciar perfis
profiles-saved-as = Salvo como
profiles-show-folder = Mostrar na pasta
