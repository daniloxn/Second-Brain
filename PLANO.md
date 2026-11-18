# Segundo Cérebro: plano, fase atual e pendências

Aplicativo desktop para Windows (Tauri 2) que funciona como segundo cérebro pessoal: notas em Markdown com links entre elas, tarefas com prazo e prioridade, cofre de senhas, explorador de arquivos, grafo 3D e um agente de IA que lê e escreve no cofre a partir de linguagem natural.

**Ponto atual (2026-11-18): validação da fase 5 — agente OpenCode no app Tauri.** As fases 4, 6, 7 e 8 estão concluídas em código. Segredos, importação, projetos e os provedores extras ainda precisam de validação no Tauri real.

## 1. Decisões de base

| Tema | Decisão | Motivo |
|---|---|---|
| Shell | Tauri 2, backend em Rust | Leve e nativo no Windows |
| Frontend | React + TypeScript + Vite | Editor, grafo 3D e painéis |
| Fonte da verdade | Pasta do cofre com arquivos `.md` | Os dados ficam legíveis e funcionam com Git |
| Sincronização | OneDrive: o cofre fica dentro de uma pasta sincronizada pelo cliente do OneDrive; o app continua trabalhando com arquivos locais | Não depende de API ou conta do OneDrive no app |
| Índice | SQLite derivado, reconstruível. **Adiado:** o grafo lê as notas direto do disco a cada abertura (82 notas em 34 ms). O SQLite entra quando houver busca de texto ou o cofre passar de ~1500 notas | Busca (ainda não existe) |
| Grafo 3D | `three` direto, sem React Three Fiber. Posições calculadas no Rust. Visual de universo (galáxia, não cérebro) | Cálculo testado no backend; a tela só desenha |
| Segredos | Arquivo criptografado com senha mestra (Argon2id + AES-GCM) | Senhas nunca ficam em texto puro (fase 4) |
| Editor | CodeMirror 6 | Markdown com destaque de sintaxe |
| Agente | Adaptadores para OpenCode, Codex, Claude Code e Ollama local, com interface comum `ProvedorAgente` | O provedor pode ser trocado na tela do agente |

## 2. Estrutura do cofre

```
<cofre>/
  _sistema/          configurações, AGENTS.md (regras para o agente), historico-agente.log, conflitos/
  _segredos/         senhas criptografadas (fase 4)
  tarefas/           uma tarefa por arquivo .md, com frontmatter
  notas/
    ideias/  estudos/  anotacoes/
  documentos/        arquivos importados (fase 6)
  projetos/          área dos agentes de programação (fase 7)
  prompts/  skills/
```

Só é cofre do app a pasta que tem `_sistema/`. Uma pasta comum (por exemplo, um cofre do Obsidian) nunca é tratada como cofre sem confirmação explícita.

Frontmatter de uma tarefa:

```yaml
---
titulo: "Entregar relatório"
status: aberta            # aberta | fazendo | feita | arquivada
prioridade: alta          # alta | media | baixa
prazo: 2026-11-18         # AAAA-MM-DD ou AAAA-MM-DD HH:MM
origem: agente
tags: [oficina, dados]
---
Descrição livre em Markdown.
```

## 3. Roteiro de fases

Ordem combinada: 0 → 1 → 2 → 5 → 3 → 4 → 6 → 7 → 8. O agente entrou antes do grafo porque economiza mais tempo no dia a dia.

| Fase | Entrega | Situação |
|---|---|---|
| 0. Base | React + Vite no Tauri, escolher ou criar o cofre, árvore de arquivos | Concluída (código) |
| 1. Notas | Editor Markdown, wikilinks, backlinks, salvamento com checagem de versão | Concluída (código) |
| 2. Tarefas | Frontmatter, lista por urgência, avisos com notificação do sistema | Concluída (código) |
| 5. Agente | Adaptador do OpenCode, pedido em linguagem natural, histórico com antes e depois, desfazer, aquecimento ao abrir | Em andamento (código pronto, testes pendentes) |
| 3. Grafo 3D | Universo 3D: cada pasta é uma constelação ao redor de um buraco negro "Home", com filtros e busca (índice SQLite adiado, ver seção 1) | Concluída (código) |
| 4. Segredos | Cofre criptografado, senha mestra, copiar com tempo limite | Concluída (código; validar no Tauri real) |
| 6. Importação | Arrastar e soltar com instrução para o agente | Concluída (código; validar no Tauri real) |
| 7. Projetos dos agentes | Visão por projeto, `MAPA.md` e `LOG.md` | Concluída (código; validar no Tauri real) |
| 8. Provedores extras | OpenCode, Codex, Claude e LLM local | Concluída (código; validar cada CLI instalado no Tauri real) |

## 4e. Fases 7 e 8: projetos e provedores extras

- A aba Projetos lista pastas em `projetos/`, permite criar um projeto com `PROJETO.md`, `MAPA.md` e `LOG.md` e mostra os documentos renderizados em Markdown. O agente pode manter esses arquivos no próprio espaço de trabalho.
- A tela do agente permite escolher OpenCode, Codex, Claude Code ou Ollama local; a escolha fica guardada no navegador deste usuário. Cada CLI é detectado pelo nome do comando no PATH, e a autenticação e instalação continuam sob responsabilidade do usuário.
- Codex executa em `workspace-write`; Claude Code usa `acceptEdits` e bloqueia a ferramenta Bash. Ambos recebem acesso à pasta do cofre como área de trabalho. O prompt comum mantém as regras de confirmação antes das alterações.
- Ollama executa `llama3.2` localmente para conversa. Essa integração não concede ferramentas para editar arquivos; modelos ou nomes diferentes ainda não são configuráveis pela tela.
- Os projetos e seus documentos seguem a sincronização local do OneDrive junto com o cofre. A escolha do provedor fica no armazenamento local do navegador e não é sincronizada.
- A integração real depende de cada CLI estar instalado, autenticado e funcionando. Não foi executada uma rodada de agente por Codex, Claude Code ou Ollama neste ambiente.
- OpenCode é o padrão atual. O app executa `opencode run --standalone --format json --agent segundo-cerebro-app <prompt>` com o diretório de trabalho no cofre e passa `OPENCODE_CONFIG_CONTENT` com a definição temporária do agente e suas permissões.
- Executar apenas o comando no CMD, sem essa variável de ambiente, resulta em `Agent not found: "segundo-cerebro-app"`; isso é esperado porque o agente não é gravado nas configurações globais do OpenCode. No app, a configuração é passada ao processo antes de iniciar.
- O erro anterior `Unrecognized flag: --dir` foi corrigido removendo a opção; o app já define o diretório atual do processo como a pasta do cofre. A execução real pelo app com o OpenCode instalado ainda está pendente.

## 4c. Fase 4 (segredos): o que está pronto

- O arquivo `_segredos/cofre.enc` contém apenas um envelope com versão, salt, nonce e conteúdo cifrado. Argon2id deriva uma chave de 256 bits da senha mestra; AES-256-GCM autentica e cifra as credenciais. A senha mestra não é gravada.
- Títulos, usuários, senhas e anotações ficam dentro do conteúdo cifrado. A listagem devolve os metadados, nunca as senhas; copiar uma senha exige confirmação explícita e a senha mestra é pedida a cada desbloqueio.
- A cópia usa o clipboard sensível do Tauri e tenta limpar a senha após 30 segundos, apenas se o clipboard ainda contiver o valor copiado. No Windows, o clipboard sensível é excluído do histórico e da sincronização do clipboard.
- Os arquivos do cofre podem ficar numa pasta sincronizada pelo OneDrive. Para evitar cópias de conflito no arquivo criptografado, usar o cofre em um dispositivo por vez; o app ainda não reconcilia alterações simultâneas entre dispositivos.
- A senha mestra tem exatamente seis caracteres para novos cofres e não pode ser recuperada. Cofres existentes continuam aceitando a senha original. O primeiro teste de criação, desbloqueio, cópia e limpeza do clipboard no app Tauri real ainda está pendente.

## 4d. Fase 6 (importação): o que está pronto

- Soltar arquivos na janela copia-os para `documentos/`, preserva a extensão e evita sobrescrever nomes já existentes.
- A importação aceita até 20 arquivos, com limite de 50 MB por arquivo e 100 MB por lote. O conteúdo original não é alterado.
- Depois da cópia, a aba do agente abre com uma proposta de análise preenchida para revisão. O agente recebe a regra para tratar o material importado como dados não confiáveis e não executar instruções contidas nos documentos.
- Os documentos ficam na pasta sincronizada do cofre e, portanto, seguem a sincronização pelo OneDrive.
- A primeira importação de arquivo pelo Tauri real, incluindo o caminho do OneDrive, ainda precisa ser conferida.

## 4. Fase 5 (agente): o que está pronto

- **Como o agente roda:** `opencode run` em modo não interativo, com a pasta de trabalho definida pelo processo como o cofre e perfil temporário que limita leituras e edições ao cofre e bloqueia shell, subagentes e acesso externo.
- **Prompt:** traz `_sistema/AGENTS.md`, a data e a hora, as convenções de registro (tarefa em `tarefas/` com frontmatter) e o pedido.
- **Confirmação antes de mudar:** o prompt manda o agente propor (título, prazo, prioridade, onde) e perguntar antes de criar ou alterar qualquer coisa. Só muda arquivo depois do "sim" do usuário. A regra fica no prompt, não só no `AGENTS.md`, para valer também em cofres já criados.
- **Contexto da conversa:** cada pedido leva as últimas 12 falas da tela (até 1500 caracteres cada), para o agente entender "isso" e "sim". Por isso a conversa que some ao trocar de seção agora afeta o contexto (ver pendências).
- **Registro:** antes e depois de cada pedido, o app compara a foto das notas e das pastas do cofre. Cada mudança vai para `_sistema/historico-agente.log` com o texto de antes e de depois. Pedido que falha também é registrado.
- **Desfazer:** confere que o cofre ainda está como o agente deixou. Se o usuário mexeu depois, para sem alterar nada. Pede confirmação antes de apagar.
- **Aquecimento:** ao abrir o app, uma saudação roda em segundo plano, numa pasta temporária fora do cofre. A resposta vira a primeira fala da tela do agente.
- **Resposta progressiva:** o OpenCode roda com `--format json`; cada evento de texto é encaminhado pelo backend como evento Tauri e aparece no chat durante a geração, com cursor animado. Codex, Claude Code e Ollama ainda exibem a resposta ao final da execução.
- **Checagem de versão nas notas:** se a nota mudou no disco enquanto estava aberta, o salvamento não sobrescreve. A tela oferece usar a versão do disco ou manter a sua. Antes de descartar ou sobrescrever, o texto vai para `_sistema/conflitos/`.
- **Verificação:** Interface conferida no navegador com backend simulado. `npm.cmd run build` e `cargo check` passam. A saudação do OpenCode ainda precisa ser conferida no app real; validações anteriores do agente eram do provedor anterior.

## 4b. Fase 3 (grafo): o que está pronto

- **Backend (`src-tauri/src/grafo/`):** lê as notas e tarefas do cofre, liga por wikilink e posiciona cada ponto em 3D. Fica de fora `_sistema/`, `_segredos/` e o `AGENTS.md` da raiz. Tarefas (`tarefas/`) viram pontos laranja; link para nota inexistente, para si mesma ou repetido não gera ligação; nome repetido vale a primeira nota em ordem alfabética (mesma regra do clique no wikilink). Limite de 1500 pontos, com aviso na tela.
- **Layout em formato de galáxia:** um disco achatado (mais grosso no meio) com um núcleo vazio no centro, onde fica o buraco negro. Cada pasta é uma constelação com âncora própria, espalhada em espiral; ligações aproximam as notas e todas se repelem. É determinístico: a posição inicial vem do nome da nota, então criar notas novas não embaralha as antigas. O backend envia também a forma da galáxia (raio, espessura e raio do núcleo), e a tela desenha o buraco negro e as órbitas com ela. A primeira versão da fase usava dois hemisférios em formato de cérebro e foi trocada por esta antes de qualquer commit.
- **Tela (`src/grafo/`), estilo universo:** o desenho ocupa a área toda e o resto flutua por cima. Estrelas com halo e cintilação (shader próprio, uma por nota; tamanho cresce com as ligações), ligações como fios de luz, buraco negro com disco de acreção que gira e anel de luz, três órbitas com cometas, fundo com 1600 estrelas e nebulosas. Cada constelação tem rótulo com nome e quantidade de notas, e o botão "Home" no buraco negro reinicia a vista. Controles flutuantes: aproximar, afastar, reiniciar, pausar e tela cheia. Arrastar gira, a roda aproxima, o mouse sobre uma estrela mostra nome, pasta e ligações, e o clique abre a nota.
- **Painel de filtros** (botão "Filtros", fechável): busca (as estrelas que não casam se apagam e os resultados viram uma lista clicável), tarefas, notas sem ligações e constelações com a cor de cada uma. Carrega sob demanda, porque o `three` pesa ~590 kB. Respeita `prefers-reduced-motion` (a vista começa pausada) e funciona em tela estreita, onde o painel vira uma folha no rodapé.
- **Testes:** 99 testes Rust passam (22 novos: grafo e o limite de leitura de arquivo). Build do frontend passa. Interface conferida no navegador com o cálculo real do Rust sobre um cofre de 83 notas: hover, busca, filtros, controles, Home e tela de celular. Um defeito achado e corrigido nesse teste: os rótulos apareciam por cima do painel.

## 5. Pendências

### Grafo (fase 3)
- Não foi aberto no app Tauri real, só no navegador com backend simulado. Conferir no app com o cofre de verdade (hoje com poucas notas).
- A conferência visual foi feita em celular (375 px) e por medidas do DOM em 1280×800, porque o painel do navegador estava estreito. Falta olhar o universo em tela larga, o desempenho (quadros por segundo) com muitas notas e se a tela cheia funciona na janela do Tauri.
- Rótulos de constelações próximas podem se sobrepor quando o universo está pequeno na tela. Se incomodar, esconder o rótulo que colide.
- O resumo "Seu universo" só mostra contagens. A ideia da referência (um resumo do que mudou desde a última visita, escrito pelo agente) depende do agente e fica para depois.
- Ainda não há estrelas cadentes nem arrastar uma estrela; só cometas nas órbitas.
- Notas com `[[wikilink]]` para nota que não existe ainda não aparecem (o Obsidian mostra como ponto fantasma). Decidir se vale mostrar.
- Sem filtro por etiqueta (`tags`) nem por data. Entra quando houver índice.
- Cofres acima de 1500 notas são cortados em ordem alfabética, sem escolher as mais ligadas.
- Teclado: o desenho 3D não é navegável por teclado. A busca e a lista de resultados são o caminho acessível para abrir uma nota.

### Agente (fase 5)
- O servidor MCP local ainda não existe. Hoje o agente age sobre os arquivos do cofre, e não por ferramentas próprias (`criar_tarefa`, `buscar`, `ler_nota`). O MCP continua previsto e é a forma de ligar os próximos CLIs.
- Pedido interrompido pelo fechamento do app não é registrado no histórico, e o processo do agente continua sozinho até terminar ou estourar o tempo limite.
- A conversa some ao trocar de seção, porque fica só na memória da tela. O histórico em disco permanece. Agora isso também apaga o contexto que o agente recebe; guardar a conversa fora da tela resolve os dois.
- Testar no app real que o agente propõe e espera o "sim" (a regra está no prompt e os testes só conferem o texto dele).
- Cada pedido abre um processo novo do OpenCode, então a primeira resposta pode levar de alguns segundos a mais de um minuto. O aquecimento confirma que o agente responde, mas não mantém o modelo carregado.
- A chamada do OpenCode pelo app ainda precisa de validação ponta a ponta. O comando isolado no CMD sem `OPENCODE_CONFIG_CONTENT` não encontra o agente temporário; o perfil é injetado pelo app em cada processo.
- Confirmar no app Tauri real que a versão instalada do OpenCode emite eventos de texto JSONL e que a resposta aparece progressivamente antes de a rodada terminar.
- O serviço do modelo respondeu "sem capacidade" (503) em alguns testes. É indisponibilidade do lado do servidor.
- Falta testar um pedido real que cria tarefa, do começo ao fim, pelo app.
- Antes de cada provedor novo, confirmar a permissão de terminal do CLI.

### Notas (fase 1)
- Renomear e apagar notas ainda não existem.
- Dois arquivos com o mesmo nome em pastas diferentes: o wikilink abre o primeiro em ordem alfabética.

### Tarefas (fase 2)
- O aviso de notificação do sistema não foi testado no Windows.
- Os avisos já enviados ficam só na memória, então reiniciar o app pode repetir um aviso.
- Recorrência diária, edição e exclusão estão disponíveis na tela; confirmar os fluxos no app Tauri real.

### Geral
- A configuração do app ainda aponta para um cofre antigo de teste. Ajustar para abrir o cofre atual.
- O identificador do app (`com.exemplo.tauribasico`) deve ser trocado antes de gerar o instalador.
- Validar a sincronização do cofre pela pasta OneDrive em dois dispositivos e documentar a recuperação de cópias de conflito.

## 6. Regras do projeto

- Nomes específicos e únicos. Nada de `data`, `item`, `value`, `temp`, `res`, `obj`, `helper`, `utils`, `manager`, `info`, `aux`.
- Comentários em português no início de cada arquivo e em cada função, componente e comando. Comentário explica o porquê.
- Um arquivo por funcionalidade, de preferência até cerca de 200 linhas.
- Contratos tipados nos dois lados (struct com `Serialize` no Rust e `interface` espelhada no TypeScript). Ao mudar um, muda o outro na mesma alteração.
- Lógica de regra fica fora dos componentes, em funções puras testáveis.
- Antes de dar uma tarefa por concluída: `npm run build`, `cargo check` sem avisos e testes unitários para a lógica de regra.
- Segredos só são lidos com confirmação explícita, a cada uso. O agente não tem acesso a eles por padrão.
- O agente só enxerga a pasta do cofre ativo.

## 7. Decisões tomadas e em aberto

Tomadas:
- Frontend em React + TypeScript (em vez de JavaScript puro).
- Os arquivos `.md` são a fonte da verdade.
- O agente usa OpenCode por padrão; os demais provedores podem ser selecionados na tela.
- O cofre de segredos pode ser usado por agentes só com confirmação por ação.
- A sincronização escolhida é OneDrive, por meio da pasta local sincronizada pelo cliente; o app não acessa uma API da Microsoft.

Em aberto:
- Política para resolver conflito se o mesmo cofre for alterado simultaneamente em mais de um dispositivo.
