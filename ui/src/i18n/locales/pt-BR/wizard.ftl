# The first-run wizard (S1), the re-shown notice, the catalog's network step (F13) and E1 "no write permission".
wizard-progress = Progresso da configuração
wizard-caption-setup = Primeira configuração
wizard-caption-changed = Aviso atualizado
wizard-caption-catalog = Ligar o catálogo
wizard-caption-review = Aviso da primeira execução
wizard-language = Idioma
wizard-step-welcome = Boas-vindas
wizard-step-risks = Riscos
wizard-step-network = Rede
wizard-step-privacy = Privacidade
wizard-step-updates = Atualizações
wizard-step-setup = Configuração
wizard-step-of = Etapa { $n } de 6
wizard-changed-of =
    { $count ->
        [one] { $count } etapa alterada
       *[other] { $count } etapas alteradas
    }
wizard-meta-changed = Alterada
wizard-meta-kept = Sem alterações
wizard-back = Voltar
wizard-next = Avançar
wizard-finish = Concluir configuração
wizard-continue = Continuar
wizard-close = Fechar
wizard-cancel = Cancelar
wizard-to-catalog = Em seguida, abre o catálogo.
wizard-to-library = Em seguida, abre sua biblioteca (modo local).

# Step 1
wizard-s1-title = Boas-vindas ao deadloader
wizard-s1-lead = O deadloader instala mods de Deadlock do GameBanana ou do seu disco e mostra cada alteração que faz.
wizard-f1a = Ele adiciona arquivos de mods à pasta de mods do jogo e uma linha ao
wizard-f1b = — um arquivo do jogo que diz ao Deadlock para carregar mods.
wizard-f2 = Tudo pode ser desfeito: reverter remove do jogo tudo o que o deadloader colocou lá.
wizard-f3a = Todos os dados dele ficam na pasta
wizard-f3b = ao lado de
wizard-f3c = .
wizard-f4 = O deadloader não tem vínculo com a Valve nem com o GameBanana.
wizard-f5 = Software livre sob a GPL-3, fornecido sem nenhuma garantia.
wizard-licence = Licença (GPL-3)
wizard-source = Código-fonte

# Step 2
wizard-s2-title = Mods por sua conta e risco
wizard-r1 = A Valve não tem regras oficiais sobre mods para Deadlock. Usar mods é decisão sua e risco seu.
wizard-r2 = Atualizações do jogo podem quebrar mods. O deadloader se reconfigura após uma atualização, mas um mod pode não funcionar até o autor atualizá-lo.
wizard-r3 = Mods são feitos por outras pessoas, sob as próprias licenças. O deadloader não pode garantir o conteúdo deles; para cada arquivo, mostra o resultado do antivírus do GameBanana.
wizard-agree = Entendo que o deadloader altera arquivos do jogo e que uso mods por minha conta e risco.
wizard-agree-note = Isso confirma as etapas 1 e 2. É necessário para continuar.
wizard-agree-error = Marque esta caixa para continuar.
wizard-agree-reason = Marque a caixa primeiro.

# Step 3
wizard-s3-title = Rede e GameBanana
wizard-s3-lead = O catálogo conversa diretamente com a API não oficial do GameBanana — e com mais ninguém. Estes são os únicos hosts que ele contata:
wizard-hosts = Hosts
wizard-host-api = catálogo e páginas de mods
wizard-host-images = miniaturas e galeria
wizard-host-files = downloads
wizard-host-cache = downloads (files.gamebanana.com redireciona para lá)
wizard-host-new = Novo
wizard-s3-images = Imagens de outros hosts nas descrições de mods só carregam depois que você permite cada host. Cada requisição aparece em
wizard-s3-images-end = .
wizard-activity = Atividade
wizard-s3-tos = Usar o catálogo significa usar o GameBanana: os termos de serviço deles se aplicam, e o GameBanana exige que os usuários tenham 13 anos ou mais.
wizard-tos = Termos de serviço do GameBanana
wizard-choice = Como você quer começar?
wizard-use-catalog = Usar o catálogo
wizard-use-catalog-text = Navegue e instale mods do GameBanana. O deadloader se conecta só aos hosts acima.
wizard-local = Agora não — modo local
wizard-local-text = Nenhuma requisição de rede; mods só do disco. Você pode ligar o catálogo depois nas configurações.
wizard-choice-error = Escolha uma das duas opções.
wizard-choice-reason = Escolha uma opção primeiro.
wizard-last-time = Sua escolha anterior
wizard-changed-title = O que mudou ({ $date })
wizard-changed-foot = Você confirmou as outras etapas em { $date }; elas não mudaram.

# Step 4
wizard-s4-title = Sua privacidade
wizard-p1 = Sem conta e sem telemetria. O deadloader não envia nada a lugar nenhum, exceto as requisições ao GameBanana da etapa anterior.
wizard-p1-local = Sem conta e sem telemetria. Você escolheu o modo local, então o deadloader não envia absolutamente nada.
wizard-p2a = Os logs ficam neste PC, na pasta
wizard-p2b = — e ocultam seu nome de usuário do Windows:
wizard-p3 = A janela do deadloader é desenhada pelo Microsoft WebView2, parte do Windows. O deadloader desliga os recursos on-line do próprio WebView2; o Windows atualiza o WebView2 independentemente do deadloader.
wizard-p4 = Você sempre pode ver o que o deadloader fez em
wizard-p4-end = .

# Step 5
wizard-s5-title = Atualizações do deadloader
wizard-u1 = Ao iniciar, o deadloader pode verificar no GitHub se existe uma versão mais nova dele.
wizard-updates-label = Procurar novas versões do deadloader no GitHub ao iniciar
wizard-updates-on = Ligado
wizard-updates-off = Desligado (padrão)
wizard-updates-na = Indisponível no modo local
wizard-u2a = Quando ligado, o deadloader faz uma requisição para
wizard-u2b = a cada início e avisa se existe uma nova versão.
wizard-u3 = Atualizar é sempre manual: você mesmo baixa a nova versão do GitHub. O deadloader nunca substitui a si mesmo.
wizard-u4 = Indisponível: o modo local não faz nenhuma requisição de rede. A opção fica disponível quando você liga o catálogo nas configurações.
wizard-u5 = Você pode mudar isso depois em Configurações → Geral.

# Step 6
wizard-s6-title = Configuração
wizard-s6-lead = O deadloader procurou o jogo e verificou algumas coisas. Nada foi alterado ainda, e você pode mudar tudo isso depois nas configurações.
wizard-checking = Verificando…
wizard-topic-game = Jogo
wizard-topic-exe = Local do deadloader
wizard-topic-disk = Espaço em disco
wizard-topic-foreign = Pasta de mods do jogo
wizard-topic-gameinfo = Arquivo do jogo para mods
wizard-game-found = Deadlock encontrado
wizard-game-chosen = Pasta escolhida
wizard-change = Alterar…
wizard-game-missing = Deadlock não encontrado
wizard-game-missing-text = O Steam ou o jogo não está instalado, ou a pasta do jogo foi movida. Escolha você mesmo a pasta do jogo — agora ou depois em Configurações → Jogo.
wizard-browse = Procurar…
wizard-browse-title = Escolha a pasta do Deadlock
wizard-help = Ajuda
wizard-help-a = No Steam: clique com o botão direito em Deadlock → Gerenciar → Ver arquivos locais. Escolha essa pasta; ela geralmente termina em
wizard-help-b = .
wizard-exe-ok = Numa pasta permanente
wizard-exe-downloads = O deadloader está na sua pasta Downloads
wizard-exe-desktop = O deadloader está na sua Área de Trabalho
wizard-exe-warn-a = Os dados dele ficam ao lado do exe, então uma limpeza apagaria sua biblioteca e seus perfis. Feche o deadloader, mova a pasta inteira para um lugar permanente, por exemplo
wizard-exe-warn-b = , e abra-o de lá.
wizard-open-folder = Abrir pasta
wizard-opened = Aberta no Explorador de Arquivos
wizard-linked = Os mods são vinculados — sem espaço extra em disco
wizard-linked-text = O deadloader e o jogo estão na mesma unidade ({ $drive }).
wizard-copied = Os mods serão copiados, não vinculados
wizard-copied-text = O deadloader está em { $data } e o jogo em { $game }. Cada mod ativado ocupa seu tamanho uma segunda vez; a biblioteca mostra isso como “copiado — +N MB”.
wizard-learn-more = Saiba mais
wizard-drives-more = Um vínculo deixa o jogo usar o arquivo da biblioteca sem uma segunda cópia. O Windows só consegue vincular arquivos na mesma unidade. Se o deadloader também estiver em { $game }, nada é copiado.
wizard-foreign-none = Nenhum arquivo de mod de fora
wizard-foreign-none-text = A pasta de mods não contém nada que o deadloader não conheça.
wizard-foreign-some =
    { $count ->
        [one] { $count } arquivo de outra origem
       *[other] { $count } arquivos de outra origem
    }
wizard-foreign-some-text = Não foi o deadloader que os colocou lá — provavelmente outro gerenciador de mods ou uma instalação manual. Ele os deixa como estão; revise-os depois na biblioteca.
wizard-show-files = Mostrar arquivos
wizard-hide-files = Ocultar arquivos
wizard-from-dmm = Deadlock Mod Manager
wizard-gi-setup = Será configurado
wizard-gi-setup-a = O deadloader adiciona uma linha ao
wizard-gi-setup-b = quando você aplica mods pela primeira vez. Nada é alterado agora.
wizard-gi-ours = Já configurado
wizard-gi-ours-text = A linha do deadloader já está lá.
wizard-gi-foreign = Alterado por outro programa
wizard-gi-foreign-text = Ele não parece o original — o Deadlock Mod Manager costuma fazer isso. O deadloader não sobrescreve alterações de outros e vai explicar o que fazer depois da configuração.
wizard-what-is = O que é isso?
wizard-gi-more = é um arquivo do jogo que diz ao Deadlock quais pastas carregar. Normalmente se resolve verificando os arquivos do jogo no Steam; depois o deadloader adiciona a linha dele.

# E1
wizard-nw-title = O deadloader não consegue salvar os dados dele.
wizard-nw-why-a = A pasta ao lado de
wizard-nw-why-b = é somente leitura: o Windows não deixa programas gravarem lá (por exemplo, dentro de Arquivos de Programas).
wizard-nw-path = Pasta do deadloader
wizard-nw-do = O que fazer
wizard-nw-s1 = Feche o deadloader.
wizard-nw-s2a = Mova a pasta inteira para um lugar onde você possa gravar, por exemplo
wizard-nw-s2b = . Não use Arquivos de Programas.
wizard-nw-s3a = Inicie o
wizard-nw-s3b = a partir do novo lugar.
wizard-nw-why-folder = O deadloader mantém tudo ao lado do exe para não espalhar nada pelo seu PC: apagar a pasta remove o deadloader por completo.
wizard-nw-details = Detalhes
wizard-nw-copy = Copiar detalhes
wizard-nw-copied = Copiado
wizard-nw-no-log = Não salvo no log — o deadloader não consegue gravar lá.
wizard-exit = Sair
