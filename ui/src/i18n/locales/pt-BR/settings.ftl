# Settings (S8) and its dialogs: D7 rollback, D8 delete all data, reset all settings.
settings-title = Configurações
settings-nav = Seções das configurações
settings-reset-nav = Redefinir todas as configurações…
settings-default = Padrão: { $value }
settings-restore = Restaurar o padrão
settings-on = Ligado
settings-off = Desligado
settings-cancel = Cancelar
settings-remove = Remover
settings-remove-item = Remover { $item }
settings-add = Adicionar
settings-open-folder = Abrir pasta
settings-open-folder-at = Abrir pasta: { $path }
settings-copied = Copiado para a área de transferência.
settings-folder = Pasta
settings-example = Exemplo:

settings-section-general = Geral
settings-section-general-lead = Idioma, inicialização e zoom.
settings-section-game = Jogo
settings-section-game-lead = Onde o Deadlock está instalado e o que o deadloader pode mudar lá.
settings-section-catalog = Catálogo
settings-section-catalog-lead = O catálogo do GameBanana, conteúdo sensível, imagens e links.
settings-section-library = Biblioteca
settings-section-library-lead = Atualizações de mods, exclusão de mods e versões antigas.
settings-section-downloads = Downloads
settings-section-downloads-lead = Fila de downloads, arquivos compactados e extração.
settings-section-network = Rede
settings-section-network-lead = Como o deadloader conversa com o GameBanana.
settings-section-cache = Cache
settings-section-cache-lead = Miniaturas e páginas do catálogo salvas neste PC.
settings-section-logs = Logs
settings-section-logs-lead = O que a Atividade guarda e por quanto tempo.
settings-section-advanced = Avançado
settings-section-advanced-lead = Arquivos de configuração e domínios permitidos adicionais.
settings-section-about = Sobre o deadloader
settings-section-about-lead = Versão, licença, como verificar este download e onde ficam seus dados.

# General
settings-general-group = Idioma e inicialização
settings-language = Idioma
settings-language-help = Idioma do sistema: o idioma de exibição do Windows, se o deadloader o tiver; senão, inglês.
settings-language-system = Idioma do sistema ({ $language })
settings-startup-profile = Perfil ao iniciar
settings-startup-profile-help = Qual perfil fica selecionado quando o deadloader inicia. Selecionar não o aplica.
settings-startup-last = Último selecionado ({ $name })
settings-startup-last-plain = Último selecionado
settings-app-updates = Procurar atualizações do deadloader ao iniciar
settings-app-updates-help = O deadloader pergunta uma vez ao GitHub (api.github.com) ao iniciar se existe uma versão mais nova. Atualizar é sempre manual. Você escolheu isso na primeira execução.
settings-app-updates-local = Indisponível: o modo local não faz nenhuma requisição de rede. A opção fica disponível quando você liga o catálogo (Configurações → Catálogo).
settings-zoom = Zoom
settings-zoom-keys-1 = Também com Ctrl + roda do mouse,
settings-zoom-keys-2 = e
settings-zoom-keys-3 = para redefinir.
settings-ctrl = Ctrl
settings-percent = { $value }%

# Game
settings-installs-group = Instalações do jogo
settings-installs-lead = O deadloader encontra sozinho as instalações do Deadlock na Steam. Cada perfil usa uma delas — mude isso em Perfis.
settings-install-library = Biblioteca da Steam { $library }
settings-install-build = build { $build }
settings-install-used =
    { $count ->
        [0] não usada por nenhum perfil
        [one] usada por 1 perfil: { $names }
       *[other] usada por { $count } perfis: { $names }
    }
settings-install-selected = { $name } (selecionado)
settings-install-applied = { $name } (aplicado aqui)
settings-install-steam = Encontrada pela Steam
settings-install-manual = Adicionada manualmente
settings-install-known = Ainda há arquivos do deadloader aqui
settings-install-manual-meta = Adicionada manualmente
settings-gi-ours = gameinfo.gi: OK — a linha do deadloader está lá
settings-gi-wait = gameinfo.gi: configurado quando um perfil usar
settings-gi-rolled = gameinfo.gi: original — Aplicar adiciona a linha do deadloader de novo
settings-gi-foreign = gameinfo.gi: alterado por outro programa
settings-gi-missing = gameinfo.gi: não encontrado — a pasta pode ter sido movida
settings-installs-none = Nenhuma instalação encontrada ainda.
settings-add-manual = Adicionar manualmente…
settings-add-manual-hint = Escolha uma pasta do Deadlock que não foi encontrada.
settings-add-manual-title = Escolha a pasta do Deadlock
settings-install-added = Instalação adicionada.
settings-install-removed = Instalação removida da lista.
settings-install-removed-text = Nada foi alterado naquela pasta.
settings-gameinfo-group = Após atualizações do jogo
settings-autofix = Corrigir o gameinfo.gi automaticamente
settings-autofix-help = Uma atualização do Deadlock redefine o arquivo do jogo que manda carregar mods. O deadloader faz um backup, coloca a linha de volta e avisa você. Desligado, um aviso pergunta antes.
settings-danger-group = Desfazer e remover
settings-rollback = Reverter a pasta do jogo do perfil selecionado
settings-rollback-help = Para a instalação do perfil selecionado { $profile } ({ $drive }): remove os { $files } arquivos de mods que o deadloader colocou lá e a linha dele no gameinfo.gi. A biblioteca e os perfis ficam; Aplicar traz tudo de volta.
settings-rollback-help-others = Outras pastas do jogo não são tocadas; selecione um perfil que as use para revertê-las.
settings-rollback-nothing = Nada para reverter em { $drive } — o deadloader não tem arquivos lá agora.
settings-rollback-no-game = O Deadlock não foi encontrado, então não há pasta do jogo para reverter.
settings-rollback-button = Reverter…
settings-rolled-title = Pasta do jogo em { $drive } revertida.
settings-rolled-text =
    { $foreign ->
        [0] Em { $drive }, { $files } arquivos de mods e a linha do deadloader no gameinfo.gi foram removidos. A biblioteca e os perfis foram mantidos — Aplicar ou Jogar traz os mods de volta.
       *[other] Em { $drive }, { $files } arquivos de mods, a linha do deadloader no gameinfo.gi e { $foreign } arquivos de terceiros foram removidos. A biblioteca e os perfis foram mantidos — Aplicar ou Jogar traz os mods de volta.
    }
settings-wipe = Excluir todos os dados do deadloader
settings-wipe-help = Exclui a biblioteca, os perfis, os logs, o cache e as configurações ({ $size }) e fecha o deadloader. O deadloader.exe fica — exclua você mesmo depois.
settings-wipe-button = Excluir todos os dados…

# Catalog
settings-catalog-group = Catálogo do GameBanana
settings-catalog = Usar o catálogo
settings-catalog-help-on = Navegue e instale mods do GameBanana. O deadloader só conversa com o GameBanana, e cada requisição aparece em Atividade. Desligado significa modo local.
settings-catalog-help-off = Desligado — modo local: o deadloader não faz nenhuma requisição de rede. Você ainda pode adicionar mods do disco.
settings-consent-title = Antes, um passo.
settings-consent-text = O deadloader mostra quais hosts vai contatar e os termos do GameBanana antes de ligar o catálogo.
settings-consent-go = Revisar e ligar
settings-local-on = O modo local está ligado.
settings-local-on-text = O catálogo está desligado e o deadloader não faz requisições de rede.
settings-ttl = Reutilizar páginas do catálogo carregadas por
settings-ttl-help = Nesse período, voltar a uma página não pergunta de novo ao GameBanana.
settings-minutes = { $count } min
settings-hours = { $count } h
settings-prefetch = Carregar a próxima página antecipadamente
settings-prefetch-help = Quando você já rolou uns 70% dos resultados, os próximos são carregados para a grade continuar.
settings-nsfw-group = Conteúdo sensível
settings-nsfw = Mods com classificação de conteúdo (NSFW)
settings-nsfw-help = Ocultos: não aparecem. Desfocados: uma imagem borrada com o aviso "Conteúdo sensível"; clique para ver. Exibidos: sem desfoque.
settings-nsfw-age-done = Idade confirmada em { $date }.
settings-nsfw-age-ask = Sair de "Ocultos" pede uma vez a confirmação da sua idade.
settings-nsfw-hidden = Ocultos
settings-nsfw-blurred = Desfocados
settings-nsfw-shown = Exibidos
settings-images-group = Imagens e links
settings-description-images = Imagens nas descrições dos mods
settings-description-images-help = Elas vêm de hosts que não são o GameBanana, por ex. i.imgur.com. Ao clicar: um espaço reservado até você clicar.
settings-images-click = Ao clicar
settings-images-always = Sempre
settings-prechecked = Deixar marcado "Sempre permitir imagens deste host"
settings-prechecked-help = Na caixa de diálogo exibida na primeira vez que uma imagem de descrição vem de um host novo.
settings-hosts = Hosts de imagens permitidos
settings-hosts-help = Imagens desses hosts carregam sem perguntar. Na Atividade, as requisições deles aparecem como "host de imagens permitido".
settings-hosts-built-in = Embutido
settings-hosts-none = Nenhum host adicionado — cada host novo pergunta antes.
settings-host-removed = { $host } removido
settings-host-removed-text = As imagens dele vão perguntar de novo antes de carregar.
settings-undo = Desfazer
settings-gif = Reproduzir GIFs automaticamente
settings-gif-help = Desligado: os GIFs mostram o primeiro quadro com um botão de reproduzir.
settings-links = Links externos
settings-links-help = Os links abrem no seu navegador padrão. "Perguntar antes" mostra o endereço completo antes de abrir.
settings-links-ask = Perguntar antes
settings-links-open = Abrir na hora

# Library
settings-updates-group = Atualizações de mods
settings-check-updates = Procurar atualizações de mods ao iniciar
settings-check-updates-help = Compara seus mods do GameBanana com o catálogo. Nunca no modo local.
settings-auto-download = Baixar atualizações automaticamente
settings-auto-download-help = Só o download — o deadloader ainda pergunta antes de um mod mudar para a nova versão.
settings-disk-group = Exclusão e versões antigas
settings-deletion = Ao excluir um mod
settings-deletion-help = Você pode escolher diferente a cada vez na caixa de diálogo de exclusão.
settings-deletion-permanent = Permanentemente
settings-deletion-bin = Para a lixeira
settings-old-versions = Versões antigas
settings-old-versions-help = Cada versão baixada é mantida para você poder voltar. Exclua versões antigas de um mod nos detalhes dele.
settings-old-keep-all = Manter todas

# Downloads
settings-queue-group = Fila
settings-parallel = Downloads simultâneos
settings-parallel-help = Quantos arquivos baixam ao mesmo tempo.
settings-speed = Limite de velocidade
settings-speed-help = Só para downloads de mods — o catálogo e as miniaturas não são limitados.
settings-speed-none = Sem limite
settings-speed-value = { $value } MB/s
settings-keep-archives = Manter os arquivos compactados após instalar
settings-keep-archives-help = Desligado: o arquivo .zip, .7z ou .rar é excluído assim que os .vpk dele estão na biblioteca.
settings-unpack-group = Extração
settings-limits = Limites de extração
settings-limits-help = Protege contra arquivos compactados que viram algo enorme. Se um arquivo confiável for recusado, aumente o limite.
settings-limits-default = 2 GB · 20× · 256
settings-limit-size = Tamanho extraído
settings-limit-ratio = Taxa de compressão
settings-limit-files = Arquivos de mods por pacote
settings-unit-gb = GB
settings-unit-x = ×
settings-unit-files = arquivos
settings-unit-s = s
settings-number-error = { $field }: digite um número inteiro de { $min } a { $max }. Até lá, o valor anterior continua valendo.

# Network
settings-network-local = O modo local está ligado.
settings-network-local-text = O deadloader não faz requisições de rede; estas configurações valem quando o catálogo estiver ligado.
settings-requests-group = Requisições ao GameBanana
settings-api-parallel = Requisições simultâneas ao catálogo
settings-api-parallel-help = Requisições à API do GameBanana ao mesmo tempo. Imagens e downloads contam separadamente.
settings-timeouts = Tempos limite
settings-timeouts-help = Conexão: desistir se não conectar nesse tempo. Resposta: desistir quando o servidor parar de enviar.
settings-timeouts-default = 10 s · 30 s
settings-timeout-connect = Conexão
settings-timeout-read = Resposta
settings-retries = Novas tentativas
settings-retries-help = Quantas vezes uma requisição que falhou é repetida, com pausas crescentes, antes de o deadloader mostrar um erro.
settings-connection-group = Conexão
settings-proxy = Proxy
settings-proxy-help = Nenhum: conexão direta. Sistema: o proxy definido no Windows. Manual: digite abaixo.
settings-proxy-none = Nenhum
settings-proxy-system = Sistema
settings-proxy-manual = Manual
settings-proxy-host = Endereço
settings-proxy-port = Porta
settings-proxy-hint = Proxy HTTP. Só as requisições do deadloader passam por ele.
settings-proxy-invalid = Digite um nome de host ou endereço sem usuário nem senha, por ex. proxy.example.com.

# Cache
settings-thumbs-group = Miniaturas e páginas do catálogo
settings-cache-limit = Limite do cache
settings-cache-limit-help = Quando o cache enche, saem primeiro as miniaturas que você não vê há mais tempo.
settings-size-mb = { $value } MB
settings-size-gb = { $value } GB
settings-cache-now = Tamanho atual
settings-cache-breakdown = { $thumbnails } miniaturas · { $pages } páginas do catálogo
settings-cache-empty = Vazio — nada em cache ainda.
settings-cache-used = { $used } de { $limit }
settings-cache-clear = Limpar cache
settings-cache-cleared = Cache limpo — { $size } liberados. As miniaturas carregam de novo quando você precisar.
settings-cache-cleared-partly = { $failed } arquivos estavam em uso e ficam até a próxima limpeza.

# Logs
settings-keep-group = Guardar logs
settings-log-days = Guardar logs por
settings-log-days-help = Entradas mais antigas são excluídas quando o deadloader inicia.
settings-days = { $count ->
        [one] { $count } dia
       *[other] { $count } dias
    }
settings-year = 1 ano
settings-log-size = Tamanho máximo
settings-log-size-help = Acima desse tamanho, os arquivos de log mais antigos são excluídos primeiro.
settings-privacy-group = Privacidade
settings-mask = Ocultar seu nome de usuário do Windows
settings-mask-help = Os caminhos nos logs mostram { $masked } em vez do seu nome — mais seguro quando você compartilha um log.
settings-logs-folder-group = Pasta de logs
settings-logs-folder = Onde ficam os logs
settings-logs-now = { $size } · { $files } arquivos · o mais antigo de { $date }
settings-open-logs = Abrir pasta de logs

# Advanced
settings-advanced-warning = Para usuários experientes.
settings-advanced-warning-text = Um erro aqui pode mudar como o deadloader se comporta. Ele verifica o arquivo antes de salvar e guarda um backup da versão anterior.
settings-files-group = Editar os arquivos de configuração
settings-file-settings = Configurações
settings-file-manifest = Manifesto da pasta do jogo
settings-file-label = Arquivo a editar
settings-file-settings-lead = Tudo destas páginas, em um só arquivo. As mudanças aparecem nas páginas depois de salvar.
settings-file-manifest-lead = A lista do que o deadloader colocou na pasta do jogo. A reversão depende dela — uma entrada errada pode deixar arquivos para trás.
settings-json-error = Isto não é JSON válido, então não foi salvo.
settings-json-where = Linha { $line }, coluna { $column }: falta ou sobra algo aqui ou logo antes — muitas vezes uma vírgula ou aspas.
settings-json-fix = Corrija esse ponto e salve de novo — até lá, o deadloader continua usando o arquivo atual.
settings-json-refused = Esses valores não podem ser usados, então nada foi salvo.
settings-json-newer = Este arquivo é de um deadloader mais novo (formato { $found }; esta versão lê até { $supported }), então não foi salvo.
settings-json-invalid = O deadloader não consegue usar este arquivo, então ele não foi salvo.
settings-json-details = Detalhes
settings-problem-unknown-field = { $path }: não é uma configuração desta versão.
settings-problem-invalid-value = { $path }: este valor não pode ser usado aqui.
settings-problem-out-of-range = { $path }: { $found } está fora de { $min }–{ $max }.
settings-problem-invalid-entry = { $path }: "{ $entry }" não é válido aqui.
settings-problem-duplicate-entry = { $path }: "{ $entry }" aparece duas vezes.
settings-problem-needs-age = { $path }: mostrar conteúdo NSFW exige antes a confirmação de idade (Configurações → Catálogo).
settings-problem-local-mode = { $path }: indisponível no modo local.
settings-json-idle = Verificado ao salvar.
settings-json-saved = Válido e salvo. A versão anterior foi guardada como { $backup }.
settings-json-saved-plain = Válido e salvo.
settings-json-save = Verificar e salvar
settings-json-discard = Descartar minhas edições
settings-json-backup = Antes de salvar, o arquivo anterior é guardado como backup; os 10 últimos ficam.
settings-domains-group = Domínios permitidos adicionais
settings-domains-lead = O deadloader só conversa com o GameBanana. Se um download for redirecionado para outro host e bloqueado, você pode permitir esse host aqui. Na Atividade, as requisições a ele aparecem como "adicionado pelo usuário".
settings-domains-built-in = Embutidos:
settings-domain-user = adicionado pelo usuário
settings-domains-none = Nenhum domínio adicional.
settings-domain-add = Adicionar um domínio
settings-domain-hint = Só o nome do host, por ex. files.example.com.
settings-domain-empty = Digite primeiro um nome de host, por ex. files.example.com.
settings-domain-not-host = Digite só o nome do host — sem https:// nem caminho, por ex. files.example.com.
settings-domain-invalid = "{ $host }" não é um nome de host. Confira a grafia, por ex. files.example.com.
settings-domain-built-in = { $host } é embutido e sempre permitido.
settings-domain-listed = { $host } já está na lista.

# About
settings-version = versão { $version }
settings-commit = Build do commit
settings-commit-local = Build local — não feito pelo GitHub Actions
settings-copy-commit = Copiar o id do commit
settings-release-page = Página da versão
settings-licence-group = Licença
settings-licence-help = O deadloader é software livre: você pode usá-lo, estudá-lo, compartilhá-lo e alterá-lo sob a GPL-3. Ele vem sem nenhuma garantia.
settings-read-licence = Ler a licença
settings-source = Código-fonte
settings-verify-group = Como verificar este download
settings-verify-lead = Duas verificações independentes de que este deadloader.exe é o publicado no GitHub.
settings-verify-hash = Comparar o checksum SHA-256
settings-verify-hash-help = Ele precisa bater com o checksum da página da versão. Uma cópia alterada poderia mostrar um valor falso aqui, então calcule você mesmo também:
settings-verify-hash-cmd = No PowerShell, na pasta do exe:
settings-verify-attest = Verificar a atestação do build
settings-verify-attest-help = Prova que o GitHub Actions gerou este arquivo a partir do código-fonte publicado. Precisa do GitHub CLI:
settings-copy-hash = Copiar checksum
settings-copy-command = Copiar comando
settings-unsigned = O deadloader.exe não tem assinatura de código.
settings-unsigned-text = O Windows SmartScreen pode dizer "O Windows protegeu o computador" na primeira execução. As duas verificações acima confirmam que o arquivo é autêntico.
settings-webview-group = Motor da janela
settings-webview-help = A janela do deadloader é desenhada pelo WebView2, parte do Windows. O Windows o atualiza por conta própria. O deadloader desliga:
settings-webview-off-1 = Consultas DNS-over-HTTPS
settings-webview-off-2 = Login com a conta Microsoft dentro da janela
settings-webview-off-3 = SmartScreen
settings-webview-off-4 = Tráfego de rede em segundo plano e atualizações de componentes
settings-webview-off-5 = Chamadas ao servidor do preenchimento automático de formulários
settings-webview-off-6 = Relatórios de erro para a Microsoft (domain reliability)
settings-webview-no-route = E a janela não tem nenhuma rota para a internet.
settings-webview-unknown = versão desconhecida
settings-data-group = Seus dados
settings-data-folder = Pasta de dados
settings-data-folder-help = Tudo o que o deadloader guarda fica nesta única pasta ao lado do exe.
settings-notice = Aviso da primeira execução
settings-notice-help = Versão { $version }, aceita em { $date } em { $language }. Suas escolhas: catálogo { $catalog }, busca de atualizações { $updates }.
settings-notice-none = Ainda não aceito.
settings-notice-again = Ler de novo

# D7
settings-d7-title = Reverter a pasta do jogo em { $drive }?
settings-d7-from =
    { $others ->
        [0] A instalação do perfil selecionado { $profile }. O deadloader remove de
       *[other] A instalação do perfil selecionado { $profile } (também usada por { $others }). O deadloader remove de
    }
settings-d7-files =
    { $count ->
        [one] 1 arquivo de mod que ele adicionou em
       *[other] { $count } arquivos de mods que ele adicionou em
    }
settings-d7-line = A linha dele no gameinfo.gi
settings-d7-line-why = — o arquivo volta a ser como a Steam o instalou.
settings-d7-stays = Fica como está:
settings-d7-library = Sua biblioteca — { $mods } mods, { $size }
settings-d7-profiles =
    { $count ->
        [one] Seu perfil — { $names }
       *[other] Seus { $count } perfis — { $names }
    }
settings-d7-other = A instalação em { $path } ({ $profile }) — não é tocada. Para revertê-la, selecione um perfil que a use e reverta de novo.
settings-d7-other-plain = A instalação em { $path } — não é tocada.
settings-d7-back = Aplicar ou Jogar traz tudo de volta.
settings-d7-foreign =
    { $count ->
        [one] Remover também 1 arquivo que o deadloader não criou
       *[other] Remover também { $count } arquivos que o deadloader não criou
    }
settings-d7-foreign-why = de outro programa ou de uma instalação manual. Eles são excluídos permanentemente; o deadloader não consegue recuperá-los.
settings-d7-confirm = Reverter
settings-d7-confirm-foreign =
    { $count ->
        [one] Reverter e excluir 1 arquivo
       *[other] Reverter e excluir { $count } arquivos
    }

# D8
settings-d8-title = Excluir todos os dados do deadloader?
settings-d8-from = O deadloader exclui tudo em
settings-d8-from-2 = e depois fecha.
settings-d8-what = O quê
settings-d8-contains = Contém
settings-d8-size = Tamanho
settings-d8-total = Total
settings-d8-library = Biblioteca
settings-d8-library-has = { $mods } mods, { $files } arquivos
settings-d8-profiles = Perfis
settings-d8-logs = Logs
settings-d8-logs-has = { $files } arquivos, desde { $date }
settings-d8-logs-none = nenhum arquivo de log
settings-d8-cache = Cache
settings-d8-cache-has = miniaturas e páginas do catálogo
settings-d8-settings = Configurações
settings-d8-settings-has =
    { $backups ->
        [0] settings.json
        [one] incl. 1 backup
       *[other] incl. { $backups } backups
    }
settings-d8-other = Outros
settings-d8-other-has = downloads parciais, backups do gameinfo.gi, dados do motor da janela
settings-d8-exe = Não excluído: o próprio deadloader.exe — exclua você mesmo se não precisar mais.
settings-d8-rollback = Reverter antes cada pasta do jogo que o deadloader usa (recomendado)
settings-d8-rollback-why = Remove os arquivos de mods do deadloader e a linha dele no gameinfo.gi de cada pasta do jogo antes de excluir. Arquivos de terceiros ficam.
settings-d8-rollback-off = Desmarcado: os arquivos de mods do deadloader ficam nas pastas do jogo e o Deadlock continua carregando-os — sem um deadloader para removê-los.
settings-d8-type-1 = Para confirmar, digite
settings-d8-type-2 = { "" }
settings-d8-word = excluir
settings-d8-hint = O botão de excluir é liberado quando a palavra confere.
settings-d8-hint-ok = Confirmado. Não dá para desfazer.
settings-d8-type-first = Digite "{ $word }" primeiro
settings-d8-deleting = Excluindo…
settings-d8-confirm = Excluir e fechar
settings-d8-confirm-rollback = Reverter, excluir e fechar

# Reset
settings-reset-title = Redefinir todas as configurações?
settings-reset-body = Cada configuração volta ao padrão, incluindo os hosts de imagens permitidos e os domínios adicionais.
settings-reset-keeps = Sua biblioteca, seus perfis e seus logs não são tocados, e a pasta do jogo fica como está.
settings-reset-backup = Um backup das configurações atuais é guardado como settings.json.bak-…
settings-reset-confirm = Redefinir configurações
settings-reset-done = Todas as configurações foram redefinidas.
settings-reset-done-text = Backup: { $backup }
