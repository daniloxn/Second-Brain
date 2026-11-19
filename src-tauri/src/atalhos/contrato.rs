//! Atalhos globais do app: quais ações existem, a combinação de cada uma e a validação do texto
//! digitado. Função pura: não registra nada no sistema, então é testada direto.

use crate::configuracao::AtalhosDoApp;
use serde::{Deserialize, Serialize};

/// Modificadores aceitos. Sem um deles, a tecla sozinha seria capturada em qualquer programa.
const MODIFICADORES: [&str; 4] = ["Ctrl", "Alt", "Shift", "Super"];

/// Ações que um atalho global pode disparar.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AcaoDeAtalho {
    AbrirIlha,
    AbrirJanelaPrincipal,
}

impl AcaoDeAtalho {
    /// Todas as ações, na ordem em que aparecem na tela de configurações.
    pub const TODAS: [AcaoDeAtalho; 2] = [AcaoDeAtalho::AbrirIlha, AcaoDeAtalho::AbrirJanelaPrincipal];
}

/// Regras sobre as combinações guardadas em `AtalhosDoApp` (que mora em `configuracao`).
impl AtalhosDoApp {
    /// Combinação atual de uma ação.
    pub fn da_acao(&self, acao: AcaoDeAtalho) -> &str {
        match acao {
            AcaoDeAtalho::AbrirIlha => &self.abrir_ilha,
            AcaoDeAtalho::AbrirJanelaPrincipal => &self.abrir_janela_principal,
        }
    }

    /// Troca a combinação de uma ação. A validação fica com quem chama.
    pub fn definir(&mut self, acao: AcaoDeAtalho, combinacao: String) {
        match acao {
            AcaoDeAtalho::AbrirIlha => self.abrir_ilha = combinacao,
            AcaoDeAtalho::AbrirJanelaPrincipal => self.abrir_janela_principal = combinacao,
        }
    }

    /// Diz se outra ação já usa a mesma combinação. Duas ações com a mesma tecla disparariam juntas.
    pub fn outra_acao_usa(&self, acao: AcaoDeAtalho, combinacao: &str) -> bool {
        AcaoDeAtalho::TODAS
            .iter()
            .filter(|outra| **outra != acao)
            .any(|outra| self.da_acao(*outra).eq_ignore_ascii_case(combinacao))
    }
}

/// Confere o texto de uma combinação: pelo menos um modificador e exatamente uma tecla comum.
/// Devolve a combinação limpa, com espaços das partes tirados.
pub fn validar_combinacao(texto: &str) -> Result<String, String> {
    let partes: Vec<&str> = texto.split('+').map(str::trim).collect();
    if partes.iter().any(|parte| parte.is_empty()) {
        return Err("a combinação está incompleta".into());
    }
    let (modificadores, teclas): (Vec<&str>, Vec<&str>) = partes
        .iter()
        .copied()
        .partition(|parte| MODIFICADORES.iter().any(|modificador| modificador.eq_ignore_ascii_case(parte)));
    if modificadores.is_empty() {
        return Err("use Ctrl, Alt, Shift ou Super junto da tecla, para não capturar a digitação comum".into());
    }
    if teclas.len() != 1 {
        return Err("a combinação precisa de exatamente uma tecla comum".into());
    }
    Ok(partes.join("+"))
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn combinacao_valida_e_devolvida_limpa() {
        assert_eq!(validar_combinacao(" Ctrl + Alt + k ").unwrap(), "Ctrl+Alt+k");
    }

    #[test]
    fn tecla_sem_modificador_e_recusada() {
        assert!(validar_combinacao("K").is_err());
    }

    #[test]
    fn duas_teclas_comuns_sao_recusadas() {
        assert!(validar_combinacao("Ctrl+K+L").is_err());
    }

    #[test]
    fn combinacao_repetida_entre_acoes_e_detectada() {
        let mut atalhos = AtalhosDoApp::default();
        atalhos.definir(AcaoDeAtalho::AbrirJanelaPrincipal, "Ctrl+Alt+Space".into());
        assert!(atalhos.outra_acao_usa(AcaoDeAtalho::AbrirIlha, "ctrl+alt+space"));
        assert!(!atalhos.outra_acao_usa(AcaoDeAtalho::AbrirIlha, "Ctrl+Alt+J"));
    }

    #[test]
    fn config_antiga_sem_atalhos_usa_os_padroes() {
        let configuracao: crate::configuracao::Configuracao = serde_json::from_str(r#"{"cofre": null}"#).unwrap();
        assert_eq!(configuracao.atalhos, AtalhosDoApp::default());
    }

    #[test]
    fn atalhos_sao_gravados_em_camel_case() {
        let texto = serde_json::to_string(&AtalhosDoApp::default()).unwrap();
        assert!(texto.contains("abrirJanelaPrincipal"));
    }
}
