//! Interface comum dos provedores de agente. Cada CLI (Antigravity, depois Codex, Claude...)
//! é um adaptador que implementa `ProvedorAgente`. O resto do app não conhece o CLI.

use super::agy::AdaptadorAgy;

/// Contrato de um provedor. Montar os argumentos e ler a saída são funções puras,
/// então dá para testar sem rodar nenhum programa.
pub trait ProvedorAgente {
    /// Nome para mostrar na tela.
    fn nome(&self) -> &'static str;

    /// Nome do programa no PATH.
    fn comando(&self) -> &'static str;

    /// Argumentos para uma rodada sem interação. `pasta_do_cofre` é liberada para o agente.
    /// O prompt não vai nos argumentos: ele entra pela entrada padrão do processo.
    fn montar_argumentos(&self, pasta_do_cofre: &str) -> Vec<String>;

    /// Texto da resposta a partir da saída do programa. Erro se a saída não for uma resposta válida.
    fn ler_resposta(&self, saida: &str) -> Result<String, String>;
}

/// Provedor usado pelo app. Para trocar ou acrescentar um CLI, muda só esta função.
pub fn provedor_ativo() -> Box<dyn ProvedorAgente> {
    Box::new(AdaptadorAgy)
}
