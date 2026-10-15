# The Library (S4): header, states, list, mod details, conflicts (C1), deleting (D6).
library-title = Biblioteca
library-profile = Perfil
library-enabled-count = { $enabled } de { $total } ativados
library-disk = { $size } no disco
library-not-checked = Atualizações ainda não verificadas
library-checked-today = Atualizações verificadas hoje às { $time }
library-checked-on = Atualizações verificadas em { $date }
library-game-not-found = Jogo não encontrado
library-add-disk = Adicionar do disco
library-add-title = Adicionar mods do disco
library-add-filter = Arquivos de mod
library-added =
    { $count ->
        [one] { $count } mod adicionado à biblioteca.
       *[other] { $count } mods adicionados à biblioteca.
    }
library-check = Procurar atualizações
library-checking = Procurando…
library-update-all = Atualizar tudo
library-review = Ver
library-undo = Desfazer
library-cancel = Cancelar
library-missing-what = O Deadlock não foi encontrado.
library-missing-why = A Steam ou o jogo não está instalado, ou a pasta do jogo foi movida. Sua biblioteca continua funcionando; aplicar alterações e Jogar ficam desativados até o jogo ser encontrado.
library-choose-folder = Escolher a pasta do jogo
library-search-again = Procurar de novo
library-running-what = O Deadlock está em execução.
library-running-why = As alterações serão aplicadas automaticamente quando você fechar o jogo.
library-pending = Alterações pendentes
library-switch-what = O perfil { $name } ainda não está no jogo.
library-switch-why = O jogo ainda tem { $applied }. Aplique { $name } agora ou aperte Jogar — o deadloader aplica antes de iniciar.
library-switch-why-first = Aplique { $name } agora ou aperte Jogar — o deadloader aplica antes de iniciar.
library-review-switch = Ver a troca
library-applying-what = Aplicando o perfil { $name }…
library-applying-why = Você pode continuar navegando; Jogar volta quando terminar.
library-show = Mostrar
library-tab-all = Todos
library-tab-enabled = Ativados
library-tab-disabled = Desativados
library-tab-updates = Atualizações
library-tab-conflicts = Conflitos
library-tab-local = Locais
library-search = Pesquisar na biblioteca
library-search-clear = Limpar pesquisa
library-higher-wins = Quando dois mods alteram o mesmo arquivo, vence o que está mais acima na lista.
library-reorder-hint = Reordenar: arraste a alça, ou foque nela e aperte Espaço
library-list-label = Mods do perfil { $name }, por prioridade
library-toggle = { $name } ativado
library-toggle-missing = O Deadlock não foi encontrado — escolha a pasta do jogo para aplicar alterações
library-grip = Mover { $name }, posição { $position } de { $total }
library-more = Mais ações para { $name }
library-show-variants = Mostrar variantes
library-hide-variants = Ocultar variantes
library-show-addons = Mostrar complementos
library-hide-addons = Ocultar complementos
library-live-picked = { $name } — pego, posição { $position } de { $total }. ↑ ↓ move, Espaço solta, Esc cancela.
library-live-moved = { $name } — posição { $position } de { $total }.
library-live-moved-grabbed = { $name } — posição { $position } de { $total }. Espaço solta, Esc cancela.
library-live-dropped = { $name } — solto na posição { $position } de { $total }.
library-live-back = { $name } — de volta à posição { $position } de { $total }.
library-live-drag = { $name } — solte para colocar acima de { $above }, na posição { $position } de { $total }.
library-badge-pending = Pendente
library-badge-damaged = Arquivos danificados
library-badge-update = Atualização disponível
library-badge-wins = Conflito · vence
library-badge-loses = Conflito · perde
library-badge-variants =
    { $count ->
        [one] { $count } variante
       *[other] { $count } variantes
    }
library-badge-local = Local
library-badge-copied = Copiado · +{ $size }
library-local = Local
library-unknown-author = Autor desconhecido
library-gamebanana = GameBanana
library-from-disk = do disco
library-addons-sub =
    { $count ->
        [one] + { $count } complemento
       *[other] + { $count } complementos
    }
library-variants = Variantes
library-variants-downloaded = { $count } baixadas
library-active = Ativa
library-select-to-switch = Selecione para trocar
library-parts = Arquivos deste mod
library-main-files = Arquivos principais
library-from = de { $archive }
library-on-with-mod = Ativado junto com o mod · { $size }
library-remove-addon = Remover complemento
library-toast-enabled = { $name } ativado.
library-toast-disabled = { $name } desativado.
library-toast-variant = { $name } trocado para { $variant }.
library-empty-title = Sua biblioteca está vazia
library-empty-text = Instale mods do catálogo ou adicione arquivos .vpk, .zip, .7z ou .rar do seu computador. Você também pode soltar arquivos em qualquer lugar desta janela.
library-empty-local-text = No modo local o catálogo fica desativado. Adicione arquivos .vpk, .zip, .7z ou .rar do seu computador, ou solte-os em qualquer lugar desta janela.
library-browse = Explorar o catálogo
library-no-match-title = Nenhum mod aqui
library-no-match-text = Nenhum mod corresponde a esta aba e a esta pesquisa. Tente outra aba ou limpe a pesquisa.

## Mod details
library-details = Detalhes do mod
library-open-mod-page = Abrir a página do mod
library-close-details = Fechar detalhes
library-enabled-in = Ativado em { $profile }
library-position = Posição { $position } de { $total }
library-version = Versão
library-no-version = Nenhuma versão informada
library-from-gamebanana = Do GameBanana
library-imported = Importado do disco
library-newer-in-library = Há uma versão mais nova na biblioteca.
library-switch-to = Trocar para
library-delete-old = Excluir versões antigas
library-no-older = Nenhuma versão antiga guardada
library-older =
    { $count ->
        [one] { $count } versão antiga guardada · { $size }
       *[other] { $count } versões antigas guardadas · { $size }
    }
library-files = Arquivos
library-on-disk = { $size } no disco
library-linked = Vinculado — sem espaço extra em disco
library-copied = Copiado — { $size } de espaço extra em disco
library-not-in-game = Fora da pasta do jogo
library-local-mod = Mod local
library-name = Nome na sua biblioteca
library-rename = Renomear
library-link-label = Página no GameBanana
library-link-hint = Cole o endereço do mod para verificar as atualizações dele.
library-link = Vincular
library-link-done = Vinculado: { $matched } de { $total } arquivos encontrados na página.
library-link-not-a-page = Isto não é uma página de mod do GameBanana. Cole um endereço como gamebanana.com/mods/123456.
library-link-removed = Este mod não está mais no GameBanana.
library-link-failed = Não foi possível ler a página. Confira o endereço e tente de novo.
library-imported-from = Importado de
library-no-conflicts = Sem conflitos com outros mods ativados
library-conflict-wins =
    { $count ->
        [one] Vence { $name } · { $count } arquivo
       *[other] Vence { $name } · { $count } arquivos
    }
library-conflict-loses =
    { $count ->
        [one] Perde para { $name } · { $count } arquivo
       *[other] Perde para { $name } · { $count } arquivos
    }
library-show-conflicts = Mostrar conflitos
library-delete-mod = Excluir mod…
library-open-folder = Abrir pasta

## Conflicts (C1)
library-conflicts-sub = Dois mods ativados alteram os mesmos arquivos do jogo. Vence o que está mais acima na lista.
library-close-conflicts = Fechar conflitos
library-no-conflicts-title = Sem conflitos
library-wins = Vence
library-loses = Perde
library-off = Desativado
library-in-use = Em uso
library-conflict-text =
    { $count ->
        [one] Os dois alteram { $count } arquivo. No jogo você vê a versão de { $winner }; a cópia de { $loser } não é usada.
       *[other] Os dois alteram { $count } arquivos. No jogo você vê as versões de { $winner }; as cópias de { $loser } não são usadas.
    }
library-conflict-off =
    { $count ->
        [one] Os dois alteram { $count } arquivo. { $off } está desativado, então o jogo usa a versão de { $other }.
       *[other] Os dois alteram { $count } arquivos. { $off } está desativado, então o jogo usa as versões de { $other }.
    }
library-files-count =
    { $count ->
        [one] { $count } arquivo
       *[other] { $count } arquivos
    }
library-show-more = Mostrar mais { $count }
library-move-above = Colocar { $name } acima
library-disable = Desativar { $name }
library-pick-variant = Escolher outra variante
library-no-variants = Nenhum dos dois mods tem variantes
library-done-wins = Pronto — agora { $name } vence
library-done-off = Pronto — { $off } está desativado; os arquivos de { $other } são usados
library-identical = Arquivos idênticos (mesma soma de verificação) não são conflitos e não aparecem aqui.

## D6 Delete mod
library-d6-title = Excluir “{ $name }”?
library-d6-files-title = Excluir as versões antigas de “{ $name }”?
library-d6-what =
    { $count ->
        [one] { $count } arquivo · { $size }.
       *[other] { $count } arquivos · { $size }.
    }
library-d6-profiles =
    { $count ->
        [one] Usado em { $count } perfil: { $names } — ele será removido dele e da pasta do jogo.
       *[other] Usado em { $count } perfis: { $names } — ele será removido deles e da pasta do jogo.
    }
library-d6-no-profiles = Nenhum perfil o usa.
library-d6-where = Excluir
library-d6-permanently = Permanentemente
library-d6-recycle-bin = Para a Lixeira
library-d6-permanent-hint = Os arquivos são excluídos para sempre — não dá para desfazer.
library-d6-bin-hint = Você pode restaurá-lo pela Lixeira do Windows.
library-d6-confirm = Excluir mod
library-d6-confirm-files = Excluir arquivos
library-deleted = { $name } excluído.
library-deleted-files =
    { $count ->
        [one] { $count } arquivo de { $name } excluído.
       *[other] { $count } arquivos de { $name } excluídos.
    }
