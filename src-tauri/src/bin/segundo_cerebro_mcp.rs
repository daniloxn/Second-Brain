//! Executável do servidor MCP do Segundo Cérebro. Conversa com o cliente (Claude Code, Codex, OpenCode)
//! pela entrada e saída padrão. Não abre janela: a lógica fica em `tauri_app_lib::mcp`.

fn main() {
    let argumentos: Vec<String> = std::env::args().skip(1).collect();
    if let Err(erro) = tauri_app_lib::mcp::executar_servidor(&argumentos) {
        // Sai pela saída de erro: a saída padrão é o canal do protocolo.
        eprintln!("servidor MCP não iniciou: {erro}");
        std::process::exit(1);
    }
}
