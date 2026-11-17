# Regras do projeto Segundo Cérebro

Enquanto o projeto estiver pequeno, todo código novo ou alterado segue as regras abaixo. Quando o projeto crescer, revisar esta lista com o usuário.

## 1. Nomes específicos e únicos

- Use nomes que digam exatamente o que a coisa é. Ex.: `caminhoDaNotaAberta`, `backlinksDaNota`, `salvarNotaComAtraso`.
- Não use nomes genéricos: `data`, `item`, `value`, `temp`, `res`, `obj`, `handle`, `helper`, `utils`, `manager`, `info`, `aux`.
- Dois conceitos diferentes não podem ter o mesmo nome em lugares diferentes do projeto.
- Nomes de arquivo seguem o conteúdo: `NotaAbrir.tsx`, `backlinks.rs`, nunca `index2.tsx` ou `misc.ts`.

## 2. Comentários em português

- Todo arquivo começa com um comentário dizendo o que ele faz e o que ele não faz.
- Toda função, componente e comando Tauri tem um comentário curto explicando o papel dela.
- Trechos com regra não óbvia (ordem de operações, limites, casos de erro, formato de caminho do Windows) têm comentário na linha ou logo acima.
- Não comente o óbvio. Comentário explica o porquê, não repete o código.

## 3. Um arquivo, uma funcionalidade

- Separe por funcionalidade, não por tipo técnico. Cada funcionalidade tem seus componentes, chamadas ao backend e tipos juntos.
- No Rust, cada domínio fica em seu próprio módulo (`cofre`, `notas`, `backlinks`, `configuracao`). `lib.rs` só registra módulos e comandos.
- No frontend, cada funcionalidade fica em sua pasta (`src/notas/`, `src/cofre/`, `src/arquivos/`). Componentes de tela não chamam `invoke` diretamente: passam pela camada de chamadas da própria funcionalidade.
- Um arquivo não passa de cerca de 200 linhas. Se passar, divida por responsabilidade.
- Uma função faz uma coisa. Se precisar de comentário para explicar que faz "duas coisas", divida.

## 4. Código que pode mudar sem quebrar o resto

- Cada módulo expõe só o que os outros precisam. Tudo o mais fica privado.
- Os contratos ficam tipados: `struct` com `Serialize` no Rust e `interface` espelhada no TypeScript, definidos no módulo da funcionalidade dona dos dados.
- Não crie dependências circulares entre módulos.
- Lógica de regra (validar nome, montar caminho, procurar links) fica fora dos componentes de tela, em funções puras que podem ser testadas.
- Ao alterar um contrato (nome de comando, campo de struct), atualize os dois lados na mesma mudança.

## 5. Validação antes de dar uma tarefa por concluída

- `npm run build` (TypeScript e Vite) precisa passar.
- `cargo check` precisa passar em `src-tauri`, sem avisos novos.
- Lógica de regra no Rust tem teste unitário em `#[cfg(test)]`.
- Mudança de interface precisa ser conferida no navegador quando possível.

## 6. Estilo

- Prefira funções pequenas e explícitas a abstrações genéricas.
- Não crie arquivo utilitário genérico. Se algo é usado por duas funcionalidades, ele vai para o módulo de domínio mais próximo.
- Evite código comentado, `console.log` esquecido e arquivos de rascunho dentro de `src/`.
