# The Library's pending changes (S9), foreign files (F9, D11), gameinfo.gi (D10) and updates (D5).
library-close = Fechar
library-apply = Aplicar
library-apply-profile = Aplicar { $name }
library-discard-all = Descartar tudo
library-keep = Manter { $name }
library-keep-applied = Manter as alterações aplicadas
library-undo-this = Desfazer esta alteração
library-undo-all = Desfazer tudo
library-close-to-apply = Feche o Deadlock — as alterações serão aplicadas automaticamente
library-play-applies = Jogar também aplica
library-applied-title = Alterações aplicadas
library-nothing-pending = Nada pendente
library-all-in-game = Tudo do perfil { $name } já está no jogo.
library-running-sub = O Deadlock está em execução, então o Windows bloqueia os arquivos dele. Estas alterações serão aplicadas automaticamente assim que você fechar o jogo.
library-waiting-sub = Estas alterações ainda não estão na pasta do jogo. Aplique agora ou aperte Jogar.
library-switch-sub = Uma troca de perfil altera muitos arquivos de uma vez, por isso espera por você. Aplique aqui ou aperte Jogar — o deadloader aplica antes de iniciar.
library-applied-sub =
    { $count ->
        [one] O Deadlock foi fechado às { $time }, então a alteração feita enquanto ele rodava foi aplicada. Dá para desfazer até a sua próxima alteração.
       *[other] O Deadlock foi fechado às { $time }, então as { $count } alterações feitas enquanto ele rodava foram aplicadas. Dá para desfazer até a sua próxima alteração.
    }
library-undoing-sub =
    { $count ->
        [one] Você desfez a alteração aplicada às { $time }, e o Deadlock está rodando de novo. Ela será revertida automaticamente quando você fechar o jogo.
       *[other] Você desfez as { $count } alterações aplicadas às { $time }, e o Deadlock está rodando de novo. Elas serão revertidas automaticamente quando você fechar o jogo.
    }
library-group-enable = Ativar · { $count }
library-group-disable = Desativar · { $count }
library-group-order = Ordem · { $count }
library-group-variant = Variante · { $count }
library-group-version = Versão · { $count }
library-group-switch = Troca de perfil · { $count }
library-group-undo =
    { $count ->
        [one] Desfazer a alteração aplicada às { $time }
       *[other] Desfazer as { $count } alterações aplicadas às { $time }
    }
library-change-on = Desativado → ativado
library-change-off = Ativado → desativado
library-change-above = Movido para cima de { $name }
library-change-above-wins = Movido para cima de { $name } — agora vence o conflito entre eles
library-change-below = Movido para baixo de { $name }
library-switch-what-happens =
    { $on ->
        [one] 1 mod é ativado
       *[other] { $on } mods são ativados
    } · { $off ->
        [one] 1 é desativado
       *[other] { $off } são desativados
    }
library-what-happens = O que acontece na pasta do jogo
library-folder-summary =
    { $added ->
        [one] 1 arquivo adicionado
       *[other] { $added } arquivos adicionados
    } · { $removed } removidos · { $renumbered } renumerados

## Foreign files
library-foreign-title = Encontrado na pasta de mods do jogo
library-foreign-note = O deadloader não os colocou aqui e não vai mexer neles a menos que você escolha.
library-collapse = Recolher
library-expand = Expandir
library-take = Passar a gerenciar
library-leave = Deixar como está
library-left = Deixado como está
library-foreign-open = Abrir a pasta que contém { $name }
library-foreign-vpk =
    { $count ->
        [one] VPK, { $count } arquivo
       *[other] VPK, { $count } arquivos
    }
library-foreign-vpk-hero =
    { $count ->
        [one] VPK, { $count } arquivo — parece uma skin para { $hero }
       *[other] VPK, { $count } arquivos — parece uma skin para { $hero }
    }
library-foreign-unreadable = Um arquivo de mod que o deadloader não consegue ler
library-foreign-replaced = Um arquivo que tomou o lugar de um do deadloader
library-foreign-dmm = Criado pelo Deadlock Mod Manager — um arquivo de configurações, não um mod
library-foreign-other = Um arquivo, não um mod
library-foreign-folder =
    { $count ->
        [one] Pasta, { $count } arquivo — não é um mod
       *[other] Pasta, { $count } arquivos — não é um mod
    }
library-foreign-meta = { $size } · adicionado em { $date }
library-taken = { $name } agora está na sua biblioteca.

## D11
library-d11-title = Passar a gerenciar { $name }?
library-d11-lead = O deadloader vai:
library-d11-step-copy = copiá-lo para a sua biblioteca como mod local ({ $size }),
library-d11-step-replace = substituir o arquivo na pasta do jogo pela própria cópia vinculada,
library-d11-step-enable = ativá-lo no perfil { $profile }, no fim da lista.
library-d11-name = Nome na sua biblioteca
library-d11-hint = A biblioteca guarda uma cópia exata do arquivo original.
library-d11-suggested = { $file } (skin de { $hero })

## D10
library-d10-title = Um arquivo do jogo foi alterado por outro programa
library-d10-what = Outro programa alterou um arquivo do jogo que controla quais pastas o Deadlock carrega — muitas vezes o Deadlock Mod Manager. O deadloader não sobrescreve alterações de outros, então seus mods não vão carregar até isso ser resolvido.
library-d10-do = Deixe a Steam restaurar o arquivo original (Verificar arquivos do jogo). Depois o deadloader adiciona as próprias linhas e avisa você.
library-d10-diff = Mostrar diferenças
library-d10-expected = Esperado (original + linhas do deadloader)
library-d10-found = Encontrado agora
library-d10-diff-note = Destacadas: as linhas que o deadloader adiciona e as linhas que outro programa adicionou.
library-d10-help = O que é este arquivo?
library-d10-help-text = O gameinfo.gi diz ao Deadlock quais pastas carregar. Gerenciadores de mods adicionam algumas linhas para o jogo carregar mods da pasta addons. Atualizações do jogo redefinem o arquivo; o deadloader corrige isso automaticamente — mas só quando o arquivo é o original.
library-d10-wait-title = A Steam está verificando os arquivos do Deadlock.
library-d10-wait-text = Quando a Steam terminar, volte ao deadloader — ele verifica o arquivo de novo automaticamente. Você também pode verificar agora.
library-d10-fixed = O gameinfo.gi está configurado — os mods voltam a carregar.
library-d10-fixed-text = A Steam restaurou o arquivo original e o deadloader adicionou as próprias linhas. Detalhes em Atividade → Arquivos.
library-open-file = Abrir arquivo
library-not-now = Agora não
library-verify-steam = Verificar arquivos do jogo na Steam
library-check-again = Verificar de novo
library-show-differences = Mostrar diferenças

## D5
library-update = Atualizar
library-update-on-gb = Há uma versão mais nova no GameBanana.
library-later = Depois
library-d5-title = Atualizar { $name }?
library-d5-installed = Instalada
library-d5-new = Nova no GameBanana
library-d5-downloaded = Baixada
library-d5-arrived = Na biblioteca, ainda não ativa
library-d5-new-meta = Atualizada em { $date } · { $size } para baixar
library-d5-download = { $size } para baixar
library-d5-changed = A nova versão tem outro conjunto de arquivos. O deadloader os associou assim:
library-d5-same = Estes arquivos são substituídos pelas novas versões:
library-d5-map-text = O deadloader não conseguiu saber, para cada arquivo, qual novo o substitui. Escolha o novo arquivo de cada um ou mantenha o antigo.
library-d5-map-label = Associação de arquivos
library-d5-col-old = Arquivo instalado
library-d5-col-new = Novo arquivo
library-d5-pick = Novo arquivo para { $file }
library-d5-choose = Escolher…
library-d5-keep = Manter o arquivo antigo
library-d5-tag-replaces = Substitui
library-d5-tag-same = Mesmo nome
library-d5-tag-new = Novo · na página do mod
library-d5-profiles = { $new } fica ativa em todo perfil que usa { $old }. { $old } continua na biblioteca, então você pode voltar nos detalhes do mod.
library-d5-install = Instalar atualização
library-d5-confirm-map = Usar estes arquivos
library-update-downloading = Baixando a atualização de { $name } — ela será instalada quando chegar.
library-updated = { $name } atualizado.
library-updated-text = Ativo em todo perfil que usava a versão antiga. A versão antiga continua guardada — você pode voltar nos detalhes do mod.

## N12: the changes applied when Deadlock was closed, each in a few words
library-n12-on = { $name } ativado
library-n12-off = { $name } desativado
library-n12-above = { $name } movido para cima de { $other }
library-n12-below = { $name } movido para baixo de { $other }
library-n12-switched = { $name } trocado para { $to }
library-n12-files = { $name }: arquivos alterados

## S9: the file list of a pending apply
library-show-file-list = Mostrar a lista de arquivos
library-hide-file-list = Ocultar a lista de arquivos
library-file-list = Arquivos na pasta do jogo
library-files-added = Adicionados · { $count }
library-files-removed = Removidos · { $count }
library-files-renumbered = Renumerados · { $count }
