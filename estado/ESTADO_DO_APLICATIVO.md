# Estado do aplicativo Segundo Cérebro

Registro corrente do app: o que já funciona, o que foi corrigido e o que falta validar.
Atualizar este arquivo a cada mudança relevante. O roteiro de fases continua no `PLANO.md`
na raiz; aqui ficam as correções e descobertas de execução.

**Última atualização:** 2026-11-18

## Situação atual

- Fase 5 (agente): código pronto. O OpenCode passou a ser iniciado corretamente no Windows
  após a correção de resolução de executável descrita abaixo.
- Fases 0–4, 6–8: concluídas em código; validação no app Tauri real ainda pendente
  (detalhes no `PLANO.md`).

## Correções aplicadas

### Prompt multilinha falhava no atalho npm (2026-11-18)

O erro `não foi possível iniciar OpenCode: batch file arguments are invalid`
vinha da execução de `opencode.cmd` pelo Rust com quebras de linha no prompt.
Testar apenas `--version` ou um pedido simples no PowerShell não cobria esse caso.

O executor agora prefere `opencode.exe` no PATH ou o binário
`node_modules/@opencode/cli/bin/opencode.exe` ao lado do atalho npm. O prompt é
passado diretamente ao executável, sem interpretação pelo `cmd.exe`.
Se o binário não existir, a mensagem solicita reparar a instalação.

Validação: 96 testes unitários passaram. O teste opt-in
`opencode_real_recebe_prompt_multilinha` também passou, usando `executar_provedor`
e o adaptador real com o perfil restrito, prompt com CRLF/LF, aspas e caracteres
de shell. A resposta final e os trechos transmitidos foram `OK`.
A interação pela janela do Tauri ainda precisa de conferência.

### OpenCode não iniciava no Windows (2026-11-18)

**Sintoma:** ao escolher OpenCode, o pedido não chegava a ser executado. A tela indicava
o comando como instalado, mas a resposta falhava com
`não foi possível iniciar opencode: program not found`. O aquecimento ao abrir o app também
falhava e nenhuma tentativa entrava em `_sistema/historico-agente.log`.

**Causa:** o npm instala o OpenCode em `C:\Users\danil\AppData\Roaming\npm` como três arquivos:

| Arquivo | Papel |
|---|---|
| `opencode` | script `#!/bin/sh`, não executável pelo Windows |
| `opencode.cmd` | atalho correto do Windows (o que o PowerShell usa) |
| `opencode.ps1` | script do PowerShell |

O PowerShell escolhe o `.cmd`, mas `std::process::Command` do Rust **não consulta `PATHEXT`**:
com o nome sem extensão, ele tenta o script `sh` (ou um `opencode.exe` inexistente) e falha.
Ao mesmo tempo, `comando_esta_instalado()` usava `where opencode`, que encontrava o `.cmd`
e devolvia "instalado" — daí a inconsistência entre a checagem e a execução.

Prova obtida com um diagnóstico temporário fora do projeto:

| Chamada | Resultado |
|---|---|
| `where opencode` | ok — encontra `opencode` e `opencode.cmd` |
| `Command::new("opencode")` | `program not found` |
| `Command::new("opencode.cmd")` | ok — `opencode v2.3.2` |
| `cmd /c opencode` | ok — `opencode v2.3.2` |

**Correção:** `src-tauri/src/agente/executar.rs` agora resolve o executável antes de criar
o processo:

- `localizar_no_path()` — roda `where` (Windows) ou `which` (outros) e devolve os caminhos;
- `escolher_caminho_executavel()` — função pura que prefere `.exe`, depois `.cmd`/`.bat`,
  e ignora o script sem extensão;
- `resolver_executavel()` — junta os dois e devolve o caminho usado por `Command::new`;
- `comando_esta_instalado()` passou a usar a mesma resolução, então "instalado" agora quer
  dizer "há um executável que o app consegue rodar".

A correção vale para os outros provedores também (Codex, Claude Code e Ollama), que podem
ter o mesmo formato de instalação.

**Comando enviado ao CLI (inalterado):**

```bash
opencode run --standalone --format json --agent segundo-cerebro-app "<prompt completo>"
```

com diretório de trabalho na pasta do cofre, `stdin` nulo e a variável de ambiente
`OPENCODE_CONFIG_CONTENT` contendo o perfil restrito (`opencode.rs`).

**Validação executada:**

- `cargo test`: 96 testes passam (93 anteriores + 3 novos para a escolha do executável);
- `cargo check`: sem avisos;
- `npm run build`: passa;
- o comando completo com `OPENCODE_CONFIG_CONTENT` foi reproduzido no shell e respondeu "OK".

**Ainda pendente:** conferir o aquecimento e um pedido real pela janela do Tauri. O comando
foi validado no shell e a lógica de resolução tem teste unitário, mas a interface do app
não foi aberta nesta máquina durante a correção.

## Pendências de validação no app real

- Aquecimento do agente e resposta progressiva (eventos JSONL) na janela do Tauri.
- Cofre de segredos (criação, desbloqueio, copiar e limpar clipboard).
- Importação de documentos por arrastar e soltar.
- Visão de projetos e os demais provedores (Codex, Claude Code, Ollama).
- Grafo 3D com o cofre de verdade, desempenho e tela cheia.
