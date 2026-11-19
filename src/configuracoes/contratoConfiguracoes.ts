// Tipos da tela de configurações, espelhados de src-tauri/src/configuracao.rs (AtalhosDoApp)
// e src-tauri/src/atalhos/contrato.rs (AcaoDeAtalho). Ao mudar um campo lá, mude aqui na mesma alteração.

/** Ação que um atalho global dispara. Espelha `AcaoDeAtalho` no Rust. */
export type AcaoDeAtalho = "abrirIlha" | "abrirJanelaPrincipal";

/** Combinação de cada atalho global, no formato do plugin (ex.: `Ctrl+Alt+Space`). */
export interface AtalhosDoApp {
  abrirIlha: string;
  abrirJanelaPrincipal: string;
}
