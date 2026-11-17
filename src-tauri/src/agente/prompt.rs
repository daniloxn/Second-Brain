//! Monta o texto enviado ao agente: as regras do cofre, a data de hoje, as convenções de
//! registro e o pedido do usuário. Função pura: não lê o disco, então é testada direto.

/// Limite do pedido em caracteres. Pedido maior que isso quase sempre é um texto colado por engano.
const LIMITE_DO_PEDIDO_EM_CARACTERES: usize = 4000;

/// Confere o pedido digitado: sem espaços sobrando, não vazio e dentro do limite.
pub fn validar_pedido(pedido_digitado: &str) -> Result<String, String> {
    let pedido = pedido_digitado.trim();
    if pedido.is_empty() {
        return Err("escreva um pedido para o agente".into());
    }
    if pedido.chars().count() > LIMITE_DO_PEDIDO_EM_CARACTERES {
        return Err(format!("o pedido passou de {LIMITE_DO_PEDIDO_EM_CARACTERES} caracteres"));
    }
    Ok(pedido.to_string())
}

/// Prompt completo. `regras_do_cofre` é o conteúdo de `_sistema/AGENTS.md`, se existir.
/// `agora` é a data e hora local já formatadas.
pub fn montar_prompt(pedido: &str, regras_do_cofre: Option<&str>, agora: &str, pasta_do_cofre: &str) -> String {
    let regras = regras_do_cofre.unwrap_or("(Sem regras escritas. Siga as convenções que já existem no cofre.)");
    let linhas = [
        format!("Você é o agente do app Segundo Cérebro. Trabalhe só dentro da pasta do cofre: {pasta_do_cofre}"),
        format!("Agora são {agora}."),
        String::new(),
        "## Regras do cofre (_sistema/AGENTS.md)".to_string(),
        regras.to_string(),
        String::new(),
        "## Como registrar".to_string(),
        "- Tarefa: crie um arquivo .md em tarefas/ com este frontmatter:".to_string(),
        "---".to_string(),
        "titulo: \"título curto\"".to_string(),
        "status: aberta".to_string(),
        "prioridade: alta ou media ou baixa".to_string(),
        "prazo: AAAA-MM-DD (ou AAAA-MM-DD HH:MM, se o pedido tiver hora)".to_string(),
        "origem: agente".to_string(),
        "tags: [tag1, tag2]".to_string(),
        "---".to_string(),
        "Descrição da tarefa, em texto livre.".to_string(),
        "- Crie e edite arquivos com a ferramenta de edição de arquivos. Não use comandos de terminal.".to_string(),
        "- Nunca apague arquivos nem pastas.".to_string(),
        "- Não mexa em _segredos/, em pastas que começam com ponto, nem em _sistema/historico-agente.log.".to_string(),
        "- Ao final, responda em português, em poucas frases, dizendo o que fez.".to_string(),
        String::new(),
        "## Pedido".to_string(),
        pedido.to_string(),
    ];
    linhas.join("\n")
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn pedido_vazio_e_recusado_e_pedido_com_espacos_e_limpo() {
        assert!(validar_pedido("   ").is_err());
        assert_eq!(validar_pedido("  criar tarefa  ").unwrap(), "criar tarefa");
    }

    #[test]
    fn prompt_traz_pasta_data_regras_e_pedido() {
        let prompt = montar_prompt("me lembre amanhã", Some("Regra X"), "2026-11-17 10:00", "C:\\cofre");
        assert!(prompt.contains("C:\\cofre"));
        assert!(prompt.contains("2026-11-17 10:00"));
        assert!(prompt.contains("Regra X"));
        assert!(prompt.ends_with("me lembre amanhã"));
    }

    #[test]
    fn prompt_sem_regras_usa_texto_padrao() {
        let prompt = montar_prompt("oi", None, "agora", "C:\\cofre");
        assert!(prompt.contains("Sem regras escritas"));
    }
}
