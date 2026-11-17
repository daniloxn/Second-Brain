# Segundo Cérebro: plano, fase atual e pendências

Aplicativo desktop para Windows (Tauri 2) que funciona como segundo cérebro pessoal: notas em Markdown com links entre elas, tarefas com prazo e prioridade, cofre de senhas, explorador de arquivos, grafo 3D e um agente de IA que lê e escreve no cofre a partir de linguagem natural.

**Fase atual: 5 (agente de IA).** A parte de código está pronta. Faltam testes de uso real (ver "Pendências").

## 1. Decisões de base

| Tema | Decisão | Motivo |
|---|---|---|
| Shell | Tauri 2, backend em Rust | Leve e nativo no Windows |
| Frontend | React + TypeScript + Vite | Editor, grafo 3D e painéis |
| Fonte da verdade | Pasta do cofre com arquivos `.md` | Os dados ficam legíveis e funcionam com Git |
| Índice | SQLite derivado, reconstruível | Busca e grafo (ainda não existe) |
| Segredos | Arquivo criptografado com senha mestra (Argon2id + AES-GCM) | Senhas nunca ficam em texto puro (fase 4) |
| Editor | CodeMirror 6 | Markdown com destaque de sintaxe |
| Agente | Antigravity pelo comando `agy`, com interface comum `ProvedorAgente` | Outros CLIs entram como novos adaptadores |

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
| 5. Agente | Adaptador do Antigravity, pedido em linguagem natural, histórico com antes e depois, desfazer, aquecimento ao abrir | Em andamento (código pronto, testes pendentes) |
| 3. Grafo 3D | Índice, grafo com layout em formato de cérebro, filtros | Pendente |
| 4. Segredos | Cofre criptografado, senha mestra, copiar com tempo limite | Pendente |
| 6. Importação | Arrastar e soltar com instrução para o agente | Pendente |
| 7. Projetos dos agentes | Visão por projeto, `MAPA.md` e `LOG.md` | Pendente |
| 8. Provedores extras | Codex, Claude e LLM local, cada um com o mesmo teste da fase 5 | Pendente |

## 4. Fase 5 (agente): o que está pronto

- **Como o agente roda:** `agy` em modo de impressão, com o prompt pela entrada padrão, `--mode accept-edits`, `--sandbox`, `--add-dir` e a pasta do cofre como diretório de trabalho.
- **Prompt:** traz `_sistema/AGENTS.md`, a data e a hora, as convenções de registro (tarefa em `tarefas/` com frontmatter) e o pedido.
- **Registro:** antes e depois de cada pedido, o app compara a foto das notas e das pastas do cofre. Cada mudança vai para `_sistema/historico-agente.log` com o texto de antes e de depois. Pedido que falha também é registrado.
- **Desfazer:** confere que o cofre ainda está como o agente deixou. Se o usuário mexeu depois, para sem alterar nada. Pede confirmação antes de apagar.
- **Aquecimento:** ao abrir o app, uma saudação roda em segundo plano, numa pasta temporária fora do cofre. A resposta vira a primeira fala da tela do agente.
- **Checagem de versão nas notas:** se a nota mudou no disco enquanto estava aberta, o salvamento não sobrescreve. A tela oferece usar a versão do disco ou manter a sua. Antes de descartar ou sobrescrever, o texto vai para `_sistema/conflitos/`.
- **Testes:** 72 testes Rust passam. Build do frontend passa. Interface conferida no navegador com backend simulado. Saudação conferida no app real (Tauri em modo de desenvolvimento).

## 5. Pendências

### Agente (fase 5)
- O servidor MCP local ainda não existe. Hoje o agente age sobre os arquivos do cofre, e não por ferramentas próprias (`criar_tarefa`, `buscar`, `ler_nota`). O MCP continua previsto e é a forma de ligar os próximos CLIs.
- Pedido interrompido pelo fechamento do app não é registrado no histórico, e o processo do `agy` continua sozinho até terminar ou estourar o tempo limite.
- A conversa some ao trocar de seção, porque fica só na memória da tela. O histórico em disco permanece.
- Cada pedido abre um processo novo do `agy`, então a primeira resposta pode levar de alguns segundos a mais de um minuto. O aquecimento confirma que o agente responde, mas não mantém o modelo carregado.
- O serviço do modelo respondeu "sem capacidade" (503) em alguns testes. É indisponibilidade do lado do servidor.
- Falta testar um pedido real que cria tarefa, do começo ao fim, pelo app.
- Antes de cada provedor novo, confirmar a permissão de terminal do CLI.

### Notas (fase 1)
- Renomear e apagar notas ainda não existem.
- Dois arquivos com o mesmo nome em pastas diferentes: o wikilink abre o primeiro em ordem alfabética.

### Tarefas (fase 2)
- O aviso de notificação do sistema não foi testado no Windows.
- Os avisos já enviados ficam só na memória, então reiniciar o app pode repetir um aviso.
- Editar título, prazo e descrição pela tela, e excluir tarefa, ainda não existem.

### Geral
- A configuração do app ainda aponta para um cofre antigo de teste. Ajustar para abrir o cofre atual.
- O identificador do app (`com.exemplo.tauribasico`) deve ser trocado antes de gerar o instalador.
- Sincronização do cofre (OneDrive ou Git) ainda não foi decidida. Mexe com a segurança dos dados, então precisa de decisão antes de ligar.

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
- O agente começa pelo Antigravity, com o adaptador por `agy`.
- O cofre de segredos pode ser usado por agentes só com confirmação por ação.

Em aberto:
- Sincronização do cofre (somente local, OneDrive ou Git).
