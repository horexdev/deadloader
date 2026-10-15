# Activity (S7): what deadloader did — network requests, file operations and app events.
activity-title = Atividade
activity-sub =
    { $days ->
        [one] Tudo o que o deadloader fez neste PC: solicitações de rede, alterações de arquivos e eventos do app. Os logs ficam neste PC por { $days } dia.
       *[other] Tudo o que o deadloader fez neste PC: solicitações de rede, alterações de arquivos e eventos do app. Os logs ficam neste PC por { $days } dias.
    }
activity-open-logs = Abrir a pasta de logs
activity-logs-opened = Pasta de logs aberta no Explorador de Arquivos.
activity-summary = Hoje em resumo
activity-net-today = Rede hoje
activity-hosts-label = Hosts contatados hoje — escolha um para filtrar
activity-hosts-contacted =
    { $count ->
        [one] { $count } host contatado
       *[other] { $count } hosts contatados
    }
activity-host-title =
    { $count ->
        [one] { $count } solicitação · { $source } — escolha para filtrar
       *[other] { $count } solicitações · { $source } — escolha para filtrar
    }
activity-net-none-local = nenhuma — modo local
activity-net-none-yet = nenhuma ainda
activity-nothing-else = Nada mais.
activity-blocked-lead =
    { $count ->
        [one] { $count } solicitação para
       *[other] { $count } solicitações para
    }
activity-blocked-tail =
    { $count ->
        [one] foi bloqueada antes de qualquer conexão.
       *[other] foram bloqueadas antes de qualquer conexão.
    }
activity-blocked-short =
    { $count ->
        [one] { $count } solicitação bloqueada, nunca contatada.
       *[other] { $count } solicitações bloqueadas, nunca contatadas.
    }
activity-show-entry = Mostrar entrada
activity-game-today = Pasta do jogo hoje
activity-show-in-files = Mostrar em Arquivos
activity-game-none = nenhuma alteração
activity-no-game-changes = O deadloader ainda não alterou nada na pasta do jogo.
activity-game-changes =
    { $count ->
        [one] { $count } alteração
       *[other] { $count } alterações
    }
activity-game-batches =
    { $count ->
        [one] em { $count } etapa
       *[other] em { $count } etapas
    }
activity-game-failed =
    { $count ->
        [one] { $count } com falha
       *[other] { $count } com falha
    }
activity-game-hardlink = { $count } vinculados
activity-game-copy = { $count } copiados
activity-game-delete = { $count } removidos
activity-game-rename = { $count } renomeados
activity-game-write = { $count } gravados
activity-game-backup = { $count } com backup
activity-game-restore = { $count } restaurados
activity-game-gameinfo =
    { $count ->
        [one] gameinfo.gi atualizado uma vez
       *[other] gameinfo.gi atualizado { $count } vezes
    }
activity-no-requests-today = Nenhuma solicitação hoje.
activity-no-requests-yet = Nenhuma solicitação ainda.
activity-local-short = O catálogo está desativado, então o deadloader não se conecta a nada.
activity-never-contacted = O deadloader não contatou nenhum host desde que foi instalado.
activity-tab-network = Rede
activity-tab-files = Arquivos
activity-tab-app = App
activity-tab-off = desativada
activity-tab-errors =
    { $count ->
        [one] · { $count } erro
       *[other] · { $count } erros
    }
activity-search = Pesquisar no log
activity-search-clear = Limpar pesquisa
activity-matches =
    { $count ->
        [one] { $count } resultado
       *[other] { $count } resultados
    }
activity-matches-more = { $count }+ resultados
activity-f-time = Período
activity-f-purpose = Finalidade
activity-f-host = Host
activity-f-op = Operação
activity-f-level = Nível
activity-f-result = Resultado
activity-all = Todos
activity-ok = OK
activity-errors = Erros
activity-range-hour = Última hora
activity-range-today = Hoje
activity-range-7d = Últimos 7 dias
activity-range-30d = Últimos 30 dias
activity-clear-filters = Limpar filtros
activity-clear-all = Limpar pesquisa e filtros
activity-follow = Acompanhar ao vivo
activity-follow-tip = Mostrar as novas entradas no topo assim que acontecem
activity-follow-on = Novas entradas aparecem no topo
activity-follow-paused = Pausado às { $time }
activity-show-new =
    { $count ->
        [one] Mostrar { $count } nova
       *[other] Mostrar { $count } novas
    }
activity-net-aria = Solicitações de rede, mais recentes primeiro
activity-files-aria = Operações de arquivos, mais recentes primeiro
activity-app-aria = Eventos do app, mais recentes primeiro
activity-c-redirects = Redirecionamentos
activity-c-time = Hora
activity-c-purpose = Finalidade
activity-c-request = Solicitação
activity-c-status = Status
activity-c-size = Tamanho
activity-c-duration = Duração
activity-c-started-by = Iniciada por
activity-c-source = Origem do host
activity-c-op = Operação
activity-c-from-to = De → para
activity-c-result = Resultado
activity-c-change = Parte da alteração
activity-c-level = Nível
activity-c-message = Mensagem
activity-to = para
activity-loading-more = Carregando mais…
activity-p-catalog = Catálogo
activity-p-thumbnail = Miniatura
activity-p-description-image = Imagem da descrição
activity-p-download = Download
activity-p-update-check = Verificação de atualizações
activity-src-builtin = Integrado
activity-src-github = GitHub
activity-src-user = Adicionado por você
activity-src-user-image = Host de imagens
activity-src-blocked = Não permitido
activity-src-long-builtin = Integrado — GameBanana, parte do deadloader
activity-src-long-github = GitHub — a verificação de atualizações do próprio deadloader, ativada nas configurações
activity-src-long-user = Adicionado por você — em Configurações → Avançado
activity-src-long-user-image = Host de imagens permitido — você o permitiu ao carregar uma imagem da descrição
activity-src-long-blocked = Não está na lista de hosts permitidos
activity-by-auto = Auto
activity-by-user = Você
activity-op-hardlink = Vincular
activity-op-copy = Copiar
activity-op-rename = Renomear
activity-op-delete = Excluir
activity-op-write = Gravar
activity-op-backup = Backup
activity-op-restore = Restaurar
activity-lv-debug = Depuração
activity-lv-info = Info
activity-lv-warn = Aviso
activity-lv-error = Erro
activity-st-blocked = Bloqueada
activity-st-blocked-long = Bloqueada — nunca contatada
activity-st-failed = Falhou
activity-redirects =
    { $count ->
        [one] { $count } redirecionamento
       *[other] { $count } redirecionamentos
    }
activity-retries =
    { $count ->
        [one] { $count } nova tentativa
       *[other] { $count } novas tentativas
    }
activity-show-redirects = Mostrar redirecionamentos
activity-hide-redirects = Ocultar redirecionamentos
activity-hop-request = Etapa 1 · solicitação
activity-hop-redirect = Etapa { $n } · redirecionamento
activity-hop-retry = Etapa { $n } · tentativa { $try }
activity-ms = { $ms } ms
activity-seconds = { $s } s
activity-no-match = Nenhuma entrada corresponde
activity-no-match-text = Tente outro período ou limpe os filtros e a pesquisa.
activity-empty-title = Nada ainda
activity-empty-text = O deadloader registra aqui cada solicitação de rede, cada alteração de arquivos na pasta do jogo e cada evento do app assim que acontecem. Nada aconteceu ainda.
activity-local-title = Modo local — nenhuma solicitação é feita
activity-local-why = O catálogo está desativado, então o deadloader não se conecta a nada — nem ao GameBanana, nem a qualquer outro lugar. Alterações de arquivos e eventos do app continuam registrados nas outras abas.
activity-local-do = Ative o catálogo nas Configurações para explorar o GameBanana. Cada solicitação aparecerá aqui.
activity-local-history = As solicitações abaixo foram feitas antes de o modo local ser ativado.
activity-enable-catalog = Ativar catálogo
activity-details = Detalhes da entrada do log
activity-close-details = Fechar detalhes
activity-close-esc = Fechar (Esc)
activity-why = Por quê:
activity-todo = O que fazer:
activity-open-advanced = Abrir Configurações → Avançado
activity-full-record = Entrada completa
activity-hops-title = Redirecionamentos · { $count }
activity-user-hidden = Seu nome de usuário do Windows está oculto nos logs:
activity-settings-logs = Configurações → Logs
activity-copy = Copiar entrada
activity-copied = Copiado
activity-kind-network = Solicitação de rede
activity-kind-files = Operação de arquivo
activity-kind-app = Evento do app
activity-d-time = Hora
activity-d-purpose = Finalidade
activity-d-started-by = Iniciada por
activity-d-request = Solicitação
activity-d-source = Origem do host
activity-d-status = Status
activity-d-size = Tamanho
activity-d-duration = Duração
activity-d-range = Intervalo
activity-d-error = Erro
activity-d-id = ID da entrada
activity-d-request-id = ID da solicitação
activity-d-operation = Operação
activity-d-from = De
activity-d-to = Para
activity-d-file = Arquivo
activity-d-result = Resultado
activity-d-change = Parte da alteração
activity-d-sha256 = SHA-256
activity-d-level = Nível
activity-d-message = Mensagem
activity-d-fields = Detalhes
activity-status-after =
    { $count ->
        [one] { $status } — após { $count } redirecionamento
       *[other] { $status } — após { $count } redirecionamentos
    }
