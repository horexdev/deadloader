# The catalog (S2), the mod page (S3) and their dialogs: D1 external link, D2 description image, D3 legal age.

## D1: an external link (31)
catalog-link-title = Abrir este link no navegador?
catalog-link-body = Ele sai do deadloader e abre no seu navegador padrão.
catalog-link-always = Sempre abrir links imediatamente
catalog-link-cancel = Cancelar
catalog-link-open = Abrir

## D3: legal age before mods with content ratings are shown (109)
catalog-nsfw-hidden = Ocultos
catalog-nsfw-blurred = Desfocados
catalog-nsfw-shown = Visíveis
catalog-age-title = Mostrar mods com conteúdo sensível?
catalog-age-body = Alguns mods no GameBanana são classificados por nudez, temas sexuais ou algo parecido. Para vê-los, confirme que você tem 18 anos ou mais, ou a maioridade onde mora.
catalog-age-choice = Você escolheu: { $mode }.
catalog-age-once = O deadloader pergunta isso só uma vez. Você pode voltar para “Ocultos” a qualquer momento.
catalog-age-confirm = Sou maior de idade
catalog-age-cancel = Cancelar

## S2: the catalog grid
catalog-title = Catálogo
catalog-section = Seção
catalog-section-mods = Mods
catalog-section-sounds = Sons
catalog-search = Buscar mods
catalog-search-sounds = Buscar sons
catalog-search-offline = A busca precisa de conexão
catalog-search-off = Busca indisponível — o catálogo não pode ser lido
catalog-search-clear = Limpar busca
catalog-results =
    { $count ->
        [one] { $count } resultado
       *[other] { $count } resultados
    }
catalog-sort = Ordenar
catalog-sort-newest = Mais novos
catalog-sort-latest_updated = Atualizados recentemente
catalog-sort-most_downloaded = Mais baixados
catalog-sort-most_liked = Mais curtidos
catalog-sort-most_viewed = Mais vistos
catalog-sort-most_commented = Mais comentados
catalog-sort-oldest = Mais antigos
catalog-sort-best = Mais relevantes
catalog-sort-best-title = Durante a busca, a ordem é por relevância
catalog-category = Categoria
catalog-category-all = Todas as categorias
catalog-hero = Herói
catalog-hero-all = Todos os heróis
catalog-hero-filter = Encontrar um herói
catalog-hero-none = Nenhum herói corresponde
catalog-filters = Filtros
catalog-filter-nsfw = Mods com classificação de conteúdo (NSFW)
catalog-filter-favourites = Só favoritos
catalog-filter-favourites-short = só favoritos
catalog-filter-installed = Ocultar instalados
catalog-filter-installed-short = instalados ocultos
catalog-nsfw-caption-hidden = NSFW ocultos
catalog-nsfw-caption-blurred = NSFW desfocados · clique num cartão para ver
catalog-nsfw-caption-shown = NSFW visíveis
catalog-loading = Carregando mods do GameBanana…
catalog-loading-sounds = Carregando sons do GameBanana…
catalog-loading-more = Carregando mais…
catalog-more-failed = Não foi possível carregar a próxima página.
catalog-grid = Mods
catalog-grid-sounds = Sons
catalog-likes =
    { $count ->
        [one] { $count } curtida
       *[other] { $count } curtidas
    }
catalog-favourite-add = Adicionar aos favoritos
catalog-favourite-remove = Remover dos favoritos
catalog-sensitive = Conteúdo sensível
catalog-click-show = Clique para ver
catalog-badge-installed = Instalado
catalog-badge-update = Atualização
catalog-card-installing = Instalando · { $percent } %
catalog-empty = Ainda não há mods aqui
catalog-empty-search = Nenhum mod corresponde a “{ $text }”
catalog-empty-favourites = Ainda não há favoritos
catalog-empty-filters = Filtros ativos: { $filters }. Remova-os para buscar em todos os mods ou tente uma palavra mais curta.
catalog-empty-shorter = Tente uma palavra mais curta ou diferente.
catalog-clear-filters = Limpar filtros
catalog-rate-retry = O deadloader tenta de novo sozinho em { $seconds } s — os mods abaixo continuam disponíveis.
catalog-retry-now = Tentar agora
catalog-try-again = Tentar de novo
catalog-check-update = Procurar atualização do deadloader
catalog-details = Detalhes · entrada do registro
catalog-copy-id = Copiar o ID da entrada
catalog-open-activity = Abrir em Atividade
catalog-offline-empty-title = Offline — nada salvo para mostrar
catalog-offline-empty-text = O deadloader não consegue acessar o GameBanana, e esta parte do catálogo não foi salva antes.
catalog-error-title = Não é possível mostrar o catálogo agora.
catalog-local-title = O catálogo está desligado (modo local)
catalog-local-why = No modo local, o deadloader não faz nenhuma requisição de rede. Sua biblioteca, seus perfis e Jogar funcionam normalmente, e você pode adicionar mods do disco.
catalog-local-do = Ligue o catálogo para explorar e instalar mods do GameBanana. O deadloader vai falar apenas com o GameBanana.
catalog-local-enable = Ligar o catálogo
catalog-local-library = Ir para a biblioteca

## S3: the mod page
catalog-breadcrumb = Navegação
catalog-back = Voltar ao catálogo
catalog-page-loading = Carregando a página do mod do GameBanana…
catalog-page-offline = Offline — mostrando a cópia salva em { $date }.
catalog-page-offline-why = Instalar arquivos novos precisa de conexão; trocar entre variantes baixadas continua funcionando.
catalog-page-offline-empty-title = Offline — esta página não foi salva
catalog-page-offline-empty-text = O deadloader não consegue acessar o GameBanana, e esta página nunca foi aberta.
catalog-page-local-title = Esta página de mod não foi salva
catalog-page-local-text = No modo local, o deadloader mostra só páginas de mods salvas antes.
catalog-removed-title = Este mod foi removido do GameBanana
catalog-removed-why = Ele foi retido ou excluído, então o deadloader não pode mostrar a página nem baixar os arquivos.
catalog-removed-keep = Sua cópia instalada ({ $copy }) continua na biblioteca e funcionando — só não vai receber atualizações.
catalog-show-in-library = Mostrar na biblioteca
catalog-installed = Instalado
catalog-by = por { $author }
catalog-version = Versão { $version }
catalog-views =
    { $count ->
        [one] { $count } visualização
       *[other] { $count } visualizações
    }
catalog-downloads =
    { $count ->
        [one] { $count } download
       *[other] { $count } downloads
    }
catalog-added = Adicionado { $when }
catalog-updated = Atualizado { $when }
catalog-favourite = Favorito
catalog-open-gamebanana = Abrir no GameBanana
catalog-gallery-alt = { $name }, imagem { $index } da galeria
catalog-gallery-count = { $index } / { $count }
catalog-gallery-previous = Imagem anterior
catalog-gallery-next = Próxima imagem
catalog-gallery-original = Ver original
catalog-gallery-hidden = As imagens de mods com classificação de conteúdo estão ocultas.
catalog-audio-preview = Este som tem uma prévia no GameBanana.
catalog-description = Descrição
catalog-description-more = Mostrar a descrição inteira
catalog-description-less = Mostrar menos
catalog-description-truncated = Só o começo desta descrição é mostrado.
catalog-opens-browser = abre no seu navegador
catalog-opens-here = Abre no deadloader
catalog-image-from = Imagem de
catalog-image-click = clique para carregar
catalog-image-failed = não foi possível carregar
catalog-image-alt = Imagem da descrição
catalog-youtube = Vídeo do YouTube
catalog-spoiler = Spoiler — clique para ver
catalog-spoiler-hide = Ocultar spoiler
catalog-files = Arquivos
catalog-files-clean =
    { $count ->
        [one] Passou na verificação antivírus do GameBanana
       *[other] Os { $count } passaram na verificação antivírus do GameBanana
    }
catalog-files-clean-dated =
    { $count ->
        [one] Passou na verificação antivírus do GameBanana · enviado em { $date }
       *[other] Os { $count } passaram na verificação antivírus do GameBanana · enviados em { $date }
    }
catalog-files-av = O resultado do antivírus do GameBanana aparece em cada arquivo.
catalog-variants-note = Estes arquivos mudam os mesmos arquivos do jogo, então são variantes — só uma fica ativa.
catalog-no-files = Este mod não tem arquivos para baixar.
catalog-older-files = Arquivos antigos ({ $count })
catalog-file-downloads =
    { $count ->
        [one] { $count } download
       *[other] { $count } downloads
    }
catalog-file-active = variante ativa
catalog-file-downloaded = baixado
catalog-file-install = Instalar
catalog-file-install-label = Instalar { $file } e torná-lo a variante ativa
catalog-file-install-title = Baixa este arquivo e o torna a variante ativa
catalog-file-install-offline = Instalar precisa de conexão
catalog-file-installing = Instalando…
catalog-file-installed = Instalado
catalog-file-switch = Trocar para
catalog-file-switch-label = Trocar para { $file }
catalog-file-switch-title = Já baixado — torná-lo a variante ativa
catalog-file-update = Atualizar
catalog-file-update-title = Uma versão mais nova substitui seu arquivo instalado — veja na biblioteca
catalog-av-clean = Limpo
catalog-av-clean-title = Antivírus do GameBanana: limpo
catalog-av-flagged = Sinalizado
catalog-av-flagged-title = Antivírus do GameBanana: { $verdict }
catalog-av-pending = Ainda não verificado
catalog-av-pending-title = O GameBanana ainda não terminou de verificar este arquivo
catalog-av-none = Sem verificação
catalog-av-none-title = O GameBanana não enviou resultado do antivírus para este arquivo
catalog-credits = Créditos
catalog-no-ratings = Sem classificação de conteúdo
catalog-no-licence = Sem texto de licença
catalog-no-sources = Sem fontes alternativas
catalog-rating-sexual_themes = Temas sexuais
catalog-rating-sexual_content = Conteúdo sexual
catalog-rating-skimpy_attire = Roupas reveladoras
catalog-rating-lewd_angles_and_poses = Ângulos e poses sugestivos
catalog-rating-full_nudity = Nudez total
catalog-licence = Licença
catalog-sources = Fontes alternativas
catalog-videos = Vídeos

## D2: a description picture from a host not allowed yet (122)
catalog-image-title = Carregar uma imagem de um host novo?
catalog-image-body-before = O deadloader vai baixar esta imagem de
catalog-image-body-after = . Esse host não é o GameBanana — a requisição aparece em Atividade como todas as outras.
catalog-image-note = As imagens têm tamanho limitado. Você pode remover hosts permitidos em Configurações → Catálogo.
catalog-image-always = Sempre permitir imagens deste host
catalog-image-cancel = Cancelar
catalog-image-load = Carregar imagem
