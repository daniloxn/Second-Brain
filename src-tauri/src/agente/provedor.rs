//! Interface comum dos provedores de agente. Cada CLI é um adaptador que implementa
//! `ProvedorAgente`; o resto do app não conhece os detalhes do comando.

use super::{antigravity::AdaptadorAntigravity, claude::AdaptadorClaude, codex::AdaptadorCodex, contrato::ModoDeExecucao, ollama::AdaptadorOllama, opencode::AdaptadorOpenCode};
use std::sync::{OnceLock, RwLock};

/// Provedor ativo nesta sessão. O frontend recupera a escolha do armazenamento local ao abrir.
static PROVEDOR_SELECIONADO: OnceLock<RwLock<String>> = OnceLock::new();

/// Identificadores estáveis disponíveis na tela de agente.
pub const PROVEDORES_DISPONIVEIS: [(&str, &str, &str); 5] = [
    ("opencode", "OpenCode", "opencode"),
    ("codex", "Codex", "codex"),
    ("claude", "Claude Code", "claude"),
    ("antigravity", "Antigravity", "agy"),
    ("ollama", "Ollama local", "ollama"),
];

/// Contrato de um provedor. Montar os argumentos e ler a saída são funções puras,
/// então dá para testar sem rodar nenhum programa.
pub trait ProvedorAgente {
    /// Nome para mostrar na tela.
    fn nome(&self) -> &'static str;

    /// Nome do programa no PATH.
    fn comando(&self) -> &'static str;

    /// Argumentos para uma rodada sem interação. A pasta do cofre também é o diretório de trabalho.
    /// O prompt entra pela entrada padrão, exceto nos CLIs que exigem argumento posicional.
    /// No modo `Planejar` o CLI precisa ficar sem permissão de edição.
    fn montar_argumentos(&self, pasta_do_cofre: &str, modo: ModoDeExecucao) -> Vec<String>;

    /// Alguns CLIs, como OpenCode, recebem o prompt como argumento posicional.
    fn prompt_como_argumento(&self) -> bool { false }

    /// Texto que vem antes do prompt quando ele é argumento. O Antigravity exige `--print=<prompt>`.
    fn prefixo_do_argumento_do_prompt(&self) -> &'static str { "" }

    /// Informa se o CLI publica partes da resposta enquanto o modelo ainda gera texto.
    fn transmite_resposta(&self) -> bool { false }

    /// Converte uma linha do fluxo de saída em texto novo para a conversa.
    fn ler_trecho_de_resposta(&self, _linha: &str) -> Result<Option<String>, String> { Ok(None) }

    /// Converte uma linha do fluxo em um passo do agente (uma ferramenta usada), para mostrar na tela.
    /// Só os CLIs que publicam esses eventos devolvem algo.
    fn ler_passo_de_resposta(&self, _linha: &str) -> Option<String> { None }

    /// Diz se o CLI consegue pesquisar na web sem editar arquivos, o que o modo `Consultar` exige.
    fn suporta_consulta_com_web(&self) -> bool { false }

    /// Configuração isolada por execução para restringir permissões do CLI, quando necessário.
    fn configuracao_de_execucao(&self, _modo: ModoDeExecucao) -> Option<&'static str> { None }

    /// Texto da resposta a partir da saída do programa. Erro se a saída não for uma resposta válida.
    fn ler_resposta(&self, saida: &str) -> Result<String, String>;
}

/// Salva a escolha atual depois de validar o identificador recebido da tela.
pub fn selecionar_provedor(id: &str) -> Result<(), String> {
    if !PROVEDORES_DISPONIVEIS.iter().any(|(id_disponivel, _, _)| *id_disponivel == id) {
        return Err("provedor de agente desconhecido".into());
    }
    let selecao = PROVEDOR_SELECIONADO.get_or_init(|| RwLock::new("opencode".into()));
    *selecao.write().unwrap_or_else(|erro| erro.into_inner()) = id.to_string();
    Ok(())
}

/// Identificador selecionado nesta sessão.
pub fn id_do_provedor_ativo() -> String {
    PROVEDOR_SELECIONADO
        .get_or_init(|| RwLock::new("opencode".into()))
        .read()
        .unwrap_or_else(|erro| erro.into_inner())
        .clone()
}

/// Cria o adaptador escolhido pela tela.
pub fn provedor_ativo() -> Box<dyn ProvedorAgente> {
    match id_do_provedor_ativo().as_str() {
        "opencode" => Box::new(AdaptadorOpenCode),
        "codex" => Box::new(AdaptadorCodex),
        "claude" => Box::new(AdaptadorClaude),
        "antigravity" => Box::new(AdaptadorAntigravity),
        "ollama" => Box::new(AdaptadorOllama),
        _ => Box::new(AdaptadorOpenCode),
    }
}
