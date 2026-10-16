## Notifications

n1 = Deadlock foi atualizado — mods restaurados.
    .why = A atualização redefiniu o gameinfo.gi, então o deadloader adicionou suas linhas de novo.
n2 = { $name } instalado e ativado.
n3 =
    { $count ->
        [one] { $count } atualização de mod disponível.
       *[other] { $count } atualizações de mods disponíveis.
    }
n4 = Deadlock está em execução.
    .why = As alterações serão aplicadas automaticamente quando você fechar o jogo.
    .pending =
        { $count ->
            [one] { $count } alteração pendente
           *[other] { $count } alterações pendentes
        }
n5 =
    { $reason ->
        [completed] Uma alteração interrompida foi concluída.
       *[undone] Uma alteração interrompida foi desfeita; a pasta do jogo está como antes.
    }
n6 =
    { $count ->
        [one] Foi encontrado { $count } arquivo na pasta de mods do jogo que não foi criado pelo deadloader.
       *[other] Foram encontrados { $count } arquivos na pasta de mods do jogo que não foram criados pelo deadloader.
    }
n7 =
    { $when ->
        [today] Offline — exibindo o catálogo salvo às { $saved }.
       *[earlier] Offline — exibindo o catálogo salvo em { $saved }.
    }
    .why = Os downloads serão retomados quando você voltar a ficar online.
n8 =
    { $count ->
        [one] Perfil “{ $name }” aplicado — { $count } mod ativado.
       *[other] Perfil “{ $name }” aplicado — { $count } mods ativados.
    }
n9 = Uma nova versão do deadloader está disponível: { $version }.
n10 = O modo local está ativado.
    .why = O catálogo está desativado e o deadloader não faz nenhuma solicitação de rede.
n11 = Os mods são copiados em vez de vinculados, porque o deadloader e o jogo estão em unidades diferentes.
    .why = Isso ocupa { $size } a mais de espaço em disco.
n12 =
    { $count ->
        [one] Deadlock foi fechado — { $count } alteração aplicada.
       *[other] Deadlock foi fechado — { $count } alterações aplicadas.
    }

## Errors: what happened, why, what to do

e1 = O deadloader não consegue salvar seus dados.
    .why =
        { $reason ->
            [access_denied] A pasta do deadloader.exe é somente leitura — por exemplo, o Windows não permite que programas gravem em “Arquivos de Programas” (Program Files).
            [in_use] Outro programa, geralmente um antivírus, está com um dos arquivos do deadloader aberto.
           *[other] O Windows não conseguiu gravar na pasta do deadloader.exe.
        }
    .todo =
        { $reason ->
            [in_use] Feche os outros programas e tente novamente.
           *[other] Mova o deadloader para uma pasta em que você possa gravar, por exemplo D:\Games\deadloader.
        }
e2 = Deadlock não foi encontrado.
    .why = Steam ou o jogo não estão instalados, ou a pasta do jogo foi movida.
    .todo = Escolha a pasta do jogo manualmente.
e3 = Um arquivo do jogo foi alterado por outro programa.
    .why = O gameinfo.gi não parece o original — geralmente quem o altera é o Deadlock Mod Manager. O deadloader não sobrescreve alterações de terceiros.
    .todo = Verifique a integridade dos arquivos do jogo no Steam; depois o deadloader adicionará suas linhas.
e4 = O catálogo não está disponível.
    .why = O GameBanana mudou sua API, então o deadloader não consegue lê-la. Sua biblioteca e seus perfis funcionam normalmente.
    .todo = Verifique se há uma atualização do deadloader ou tente novamente mais tarde.
e5 = O GameBanana está sobrecarregado.
    .why =
        { $reason ->
            [rate_limited] Solicitações demais em pouco tempo (HTTP 429).
            [server] O servidor do GameBanana informou um erro.
           *[other] Solicitações demais ou um erro no servidor.
        }
    .todo =
        { $wait ->
            [seconds]
                { $retry_in_s ->
                    [one] O deadloader tentará novamente em { $retry_in_s } segundo, automaticamente.
                   *[other] O deadloader tentará novamente em { $retry_in_s } segundos, automaticamente.
                }
           *[later] O deadloader tentará novamente em instantes, automaticamente.
        }
e6 = Falha no download.
    .why =
        { $reason ->
            [local_mode] O deadloader está no modo local, então não faz solicitações de rede.
            [dns] O endereço do servidor de download não foi encontrado.
            [connect] O deadloader não conseguiu se conectar ao servidor de download.
            [tls] Não foi possível estabelecer uma conexão segura com o servidor de download.
            [timeout] O servidor demorou demais para responder.
            [connection_lost] A conexão foi perdida.
            [http_status] O servidor respondeu com um erro (HTTP { $status }).
           *[other] O download foi interrompido.
        }
    .todo =
        { $reason ->
            [local_mode] Ative o catálogo em Configurações para baixar mods.
           *[other] Ele será retomado de onde parou.
        }
e7 = O arquivo baixado está corrompido.
    .why =
        { $reason ->
            [size] O tamanho dele não corresponde ao informado pelo GameBanana.
           *[checksum] A soma de verificação dele não corresponde à informada pelo GameBanana.
        }
    .todo = Baixe-o novamente.
e8 = Não é possível extrair o arquivo compactado.
    .why =
        { $reason ->
            [not_an_archive] O arquivo não é um arquivo ZIP, 7z ou RAR — talvez o download esteja incompleto.
            [damaged] O arquivo compactado está corrompido.
            [encrypted] O arquivo compactado é protegido por senha.
            [unsupported] Ele usa um método de compactação que o deadloader não suporta.
            [not_a_vpk] { $entry } não é um arquivo .vpk válido.
           *[other] Ele está corrompido ou usa um formato não suportado.
        }
    .todo = Tente outro arquivo deste mod ou informe o problema ao autor.
e9 = O arquivo compactado é grande demais para ser extraído com segurança.
    .why =
        { $reason ->
            [too_large]
                { $measured ->
                    [yes] Extraído, ele ocuparia { $unpacked }, e o limite é { $limit }.
                   *[no] Extraído, ele ultrapassa { $limit }.
                }
            [too_many_files]
                { $found ->
                    [one] Ele contém { $found } arquivo de mod, e o limite é { $limit }.
                   *[other] Ele contém { $found } arquivos de mod, e o limite é { $limit }.
                }
            [ratio] { $entry } está compactado de forma suspeita — mais de { $limit } vezes. Isso é típico de um arquivo compactado malicioso.
            [memory] Para extraí-lo são necessários { $needed } de memória, e o limite é { $limit }.
           *[other] Ele ultrapassa um dos limites de segurança para arquivos compactados.
        }
    .todo =
        { $reason ->
            [memory] Tente outro arquivo deste mod.
            [ratio] Não o instale a menos que confie na origem; só então aumente o limite em Configurações → Avançado.
           *[other] Se você confia nele, aumente o limite em Configurações → Downloads.
        }
e10 = Nenhum arquivo de mod neste arquivo compactado.
    .why =
        { $listing ->
            [files] Ele não contém arquivos .vpk, apenas { $contents }.
           *[empty] O arquivo compactado está vazio.
        }
    .todo = Este arquivo não é um mod de Deadlock em um formato que o deadloader consiga instalar.
e11 = Não é possível extrair arquivos RAR neste PC.
    .why = A ferramenta integrada do Windows não consegue ler este RAR, e nem o 7-Zip nem o WinRAR foram encontrados.
    .todo = Instale o 7-Zip (é gratuito) e tente novamente; o arquivo baixado foi mantido.
e12 = Espaço em disco insuficiente.
    .why =
        { $reason ->
            [space] São necessários { $needed }, e só há { $available } livres.
           *[full] O disco está cheio.
        }
    .todo = Libere espaço ou exclua versões antigas de mods na Biblioteca — depois tente novamente.
e13 = Não é possível aplicar as alterações enquanto o Deadlock está em execução.
    .why = O Windows bloqueia os arquivos do jogo enquanto ele está em execução.
    .todo = Feche o Deadlock; o deadloader aplicará as alterações logo em seguida.
e14 = Um arquivo de mod está ausente ou corrompido na biblioteca.
    .why = Ele foi alterado ou excluído fora do deadloader.
    .todo = Repare o mod para baixar o arquivo novamente, ou remova o mod.
e15 = Não foi possível abrir o link.
    .why = Nenhum navegador padrão está definido no Windows.
    .todo = Copie o link e abra-o você mesmo.
e16 = Solicitação bloqueada.
    .why =
        { $reason ->
            [not_https] O servidor redirecionou para um endereço sem HTTPS, e o deadloader nunca segue esses redirecionamentos.
            [bad_redirect] O servidor enviou um redirecionamento que o deadloader não consegue seguir.
            [too_many_redirects] O servidor redirecionou vezes demais.
           *[not_listed] { $host } não está na lista de hosts permitidos, então o deadloader não se conectou a ele.
        }
    .todo =
        { $reason ->
            [not_listed] Não é preciso fazer nada — o deadloader só se comunica com o GameBanana. Se você confia neste host, adicione-o em Configurações → Avançado.
           *[not_https] Não é preciso fazer nada — o deadloader só segue redirecionamentos seguros. Tente novamente mais tarde.
        }
e17 = Não foi possível importar o perfil.
    .why =
        { $reason ->
            [newer] Ele foi criado por uma versão mais recente do deadloader.
            [unreadable] Não é possível ler o arquivo — ele pode ter sido movido, excluído ou aberto por outro programa.
           *[not_a_profile] O arquivo não é um perfil do deadloader.
        }
    .todo =
        { $reason ->
            [newer] Atualize o deadloader e importe o perfil novamente.
            [unreadable] Verifique se o arquivo ainda existe e se nenhum outro programa está com ele aberto, depois tente novamente.
           *[not_a_profile] Verifique se você escolheu o arquivo certo.
        }
e18 = Não foi possível alterar os arquivos na pasta do jogo.
    .why =
        { $reason ->
            [access_denied] O Windows negou o acesso à pasta do jogo.
            [in_use] Um arquivo está em uso por outro programa, como um antivírus ou outro gerenciador de mods.
           *[other] O Windows não conseguiu alterar os arquivos.
        }
    .todo = Feche os outros programas e tente novamente.
e19 = Não foi possível carregar a imagem.
    .why =
        { $reason ->
            [no_response] O host não respondeu.
            [not_an_image] O arquivo não é uma imagem.
            [too_large] A imagem é grande demais.
           *[other] O host não enviou uma imagem utilizável.
        }
    .todo = O espaço reservado continua; você pode tentar novamente.
e20 = Há mods demais ativados.
    .why =
        { $free ->
            [one] O jogo pode carregar mais { $free } arquivo de mod, e este perfil precisa de { $wanted }.
           *[other] O jogo pode carregar mais { $free } arquivos de mod, e este perfil precisa de { $wanted }.
        }
    .todo = Desative alguns mods ou escolha menos complementos.
e21 = Já existe um perfil chamado “{ $name }”.
    .why = Os nomes são comparados sem diferenciar maiúsculas de minúsculas, como nomes de arquivos.
    .todo = Escolha outro nome.
e22 = “{ $name }” não pode ser usado como nome de perfil.
    .why = O Windows não aceita esse nome de arquivo — por exemplo, CON, um ponto no final ou qualquer um dos caracteres \ / : * ? " < > |.
    .todo = Escolha outro nome.
e23 = O deadloader não consegue ler seu arquivo { $file }.
    .why =
        { $reason ->
            [damaged_at] O arquivo está corrompido (linha { $line }, coluna { $column }).
            [damaged] O arquivo está corrompido.
            [newer] Ele foi criado por uma versão mais recente do deadloader.
           *[invalid] O conteúdo dele não é o que o deadloader espera.
        }
    .todo =
        { $kind ->
            [settings] Por enquanto, o deadloader usa as configurações padrão e deixa o arquivo como está.
            [manifest] Restaure o manifest.json.bak ou atualize o deadloader.
           *[other] Restaure o arquivo de um backup ou atualize o deadloader.
        }
e24 = O arquivo não está mais no GameBanana.
    .why = O autor o removeu ou substituiu (HTTP { $status }).
    .todo = Abra a página do mod para escolher outro arquivo ou verifique se há atualizações do mod.
e25 = O GameBanana recusou a solicitação.
    .why = O servidor respondeu com o código { $status }. Normalmente isso é temporário.
    .todo = Tente novamente mais tarde. Se continuar acontecendo, informe na página do deadloader no GitHub.
internal = Algo deu errado dentro do deadloader.
    .why = É um erro do deadloader, não algo que você fez.
    .todo = Tente novamente. Se acontecer de novo, relate o problema com a entrada do log em Detalhes.

## Buttons named in the texts above

action-undo = Desfazer
action-review = Revisar
action-retry = Tentar novamente
action-retry-now = Tentar agora
action-browse = Procurar…
action-verify-in-steam = Verificar no Steam
action-show-differences = Mostrar diferenças
action-details = Detalhes
action-repair = Reparar
action-copy-link = Copiar link
action-open-folder = Abrir pasta
action-open-release-page = Abrir página da versão
action-enable-catalog = Ativar catálogo
action-learn-more = Saiba mais
action-view-log = Ver log

## Shared pieces

log-entry = Entrada do log { $id }
list-more = mais { $count }

## Sizes, labelled like Windows Explorer labels them

size-bytes =
    { $count ->
        [one] { $count } byte
       *[other] { $count } bytes
    }
size-kb = { $value } KB
size-mb = { $value } MB
size-gb = { $value } GB
size-tb = { $value } TB
