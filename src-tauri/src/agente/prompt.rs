//! Monta o texto enviado ao agente: as regras do cofre, a data de hoje, as convenções de
//! registro e o pedido do usuário. Função pura: não lê o disco, então é testada direto.

use super::contrato::FalaDaConversa;

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

/// Quantas falas anteriores da conversa vão para o prompt. As mais antigas ficam de fora.
const LIMITE_DE_FALAS_DA_CONVERSA: usize = 12;

/// Tamanho máximo de cada fala da conversa. Uma resposta longa do agente não pode ocupar o prompt todo.
const LIMITE_DE_CADA_FALA_EM_CARACTERES: usize = 1500;

/// Texto da conversa anterior, uma fala por linha, só com as últimas falas e cada uma cortada no limite.
/// Papéis desconhecidos são ignorados. Devolve vazio quando não há conversa.
fn formatar_conversa_anterior(conversa_anterior: &[FalaDaConversa]) -> String {
    let falas_validas: Vec<String> = conversa_anterior
        .iter()
        .filter_map(|fala| {
            let nome_do_papel = match fala.papel.as_str() {
                "voce" => "Usuário",
                "agente" => "Agente",
                _ => return None,
            };
            let texto: String = fala.texto.trim().chars().take(LIMITE_DE_CADA_FALA_EM_CARACTERES).collect();
            Some(format!("{nome_do_papel}: {texto}"))
        })
        .collect();
    let primeira_fala_usada = falas_validas.len().saturating_sub(LIMITE_DE_FALAS_DA_CONVERSA);
    falas_validas[primeira_fala_usada..].join("\n")
}

/// Prompt completo. `regras_do_cofre` é o conteúdo de `_sistema/AGENTS.md`, se existir.
/// `agora` é a data e hora local já formatadas. `conversa_anterior` são as falas da tela antes deste pedido.
pub fn montar_prompt(
    pedido: &str,
    conversa_anterior: &[FalaDaConversa],
    regras_do_cofre: Option<&str>,
    agora: &str,
    pasta_do_cofre: &str,
) -> String {
    let regras = regras_do_cofre.unwrap_or("(Sem regras escritas. Siga as convenções que já existem no cofre.)");
    let conversa = formatar_conversa_anterior(conversa_anterior);
    let conversa_ou_aviso = if conversa.is_empty() { "(Nenhuma conversa anterior.)".to_string() } else { conversa };
    let linhas = [
        format!("Você é o agente do app Segundo Cérebro. Trabalhe só dentro da pasta do cofre: {pasta_do_cofre}"),
        format!("Agora são {agora}."),
        String::new(),
        "## Regras do cofre (_sistema/AGENTS.md)".to_string(),
        regras.to_string(),
        String::new(),
        "## Quando registrar ou alterar (regra obrigatória)".to_string(),
        "- Nunca crie, edite ou mova arquivos sem confirmação explícita do usuário.".to_string(),
        "- Se o usuário pedir algo que muda o cofre, ou se você achar que vale registrar algo, NÃO mexa nos arquivos. Responda com a proposta (o que vai criar ou alterar, título, prazo, prioridade, onde) e pergunte se pode registrar.".to_string(),
        "- Só mude arquivos quando a mensagem atual confirmar uma proposta sua que está na conversa anterior (\"sim\", \"pode\", \"ok\"), ou quando o usuário já disser com clareza o que quer e mandar registrar.".to_string(),
        "- Use a conversa anterior para entender \"isso\", \"aquilo\" e \"depois\". Se mesmo assim não ficar claro o que registrar, pergunte. Nunca crie tarefa com título vago como \"fazer isso\".".to_string(),
        "- Perguntas e conversa sem mudança: só responda, sem tocar em arquivos.".to_string(),
        String::new(),
        "## Conversa anterior".to_string(),
        conversa_ou_aviso,
        String::new(),
        "## Como consultar o cofre (vale para todo pedido)".to_string(),
        "- Para ler notas, listar tarefas ou ver pastas, use as ferramentas de leitura e de listagem de arquivos.".to_string(),
        "- Use só as ferramentas de ler, listar e editar arquivos. Não use nenhuma outra: nem comandos de terminal (Bash, PowerShell, cmd), nem workflows (como RunWorkflow), nem busca na web. O app nega essas ferramentas e você ficaria sem resposta.".to_string(),
        "- Sempre termine com uma resposta em texto, mesmo que seja só dizer que não encontrou nada.".to_string(),
        String::new(),
        "## Como registrar (depois de confirmado)".to_string(),
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
        "- Crie e edite arquivos com a ferramenta de edição de arquivos.".to_string(),
        "- Nunca apague arquivos nem pastas.".to_string(),
        "- Não mexa em _segredos/, em pastas que começam com ponto, nem em _sistema/historico-agente.log.".to_string(),
        "- Ao final, responda em português, em poucas frases, dizendo o que fez.".to_string(),
        String::new(),
        "## Mensagem atual do usuário".to_string(),
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
        let prompt = montar_prompt("me lembre amanhã", &[], Some("Regra X"), "2026-11-17 10:00", "C:\\cofre");
        assert!(prompt.contains("C:\\cofre"));
        assert!(prompt.contains("2026-11-17 10:00"));
        assert!(prompt.contains("Regra X"));
        assert!(prompt.ends_with("me lembre amanhã"));
    }

    #[test]
    fn prompt_sem_regras_usa_texto_padrao() {
        let prompt = montar_prompt("oi", &[], None, "agora", "C:\\cofre");
        assert!(prompt.contains("Sem regras escritas"));
    }

    #[test]
    fn prompt_manda_perguntar_antes_de_registrar() {
        let prompt = montar_prompt("faço isso depois", &[], None, "agora", "C:\\cofre");
        assert!(prompt.contains("Nunca crie, edite ou mova arquivos sem confirmação explícita"));
        assert!(prompt.contains("Nenhuma conversa anterior"));
        assert!(prompt.contains("RunWorkflow"));
    }

    fn fala(papel: &str, texto: &str) -> FalaDaConversa {
        FalaDaConversa { papel: papel.into(), texto: texto.into() }
    }

    #[test]
    fn conversa_anterior_entra_no_prompt_e_ignora_papel_desconhecido() {
        let conversa = [fala("voce", "vou estudar Rust"), fala("erro", "falhou"), fala("agente", "Quer registrar?")];
        let prompt = montar_prompt("sim", &conversa, None, "agora", "C:\\cofre");
        assert!(prompt.contains("Usuário: vou estudar Rust\nAgente: Quer registrar?"));
        assert!(!prompt.contains("falhou"));
    }

    #[test]
    fn conversa_anterior_guarda_so_as_ultimas_falas_e_corta_falas_longas() {
        let conversa: Vec<FalaDaConversa> = (0..20).map(|numero| fala("voce", &format!("fala {numero}"))).collect();
        let texto = formatar_conversa_anterior(&conversa);
        assert!(!texto.contains("fala 7\n"));
        assert!(texto.contains("fala 8"));
        assert!(texto.contains("fala 19"));

        let longa = [fala("agente", &"a".repeat(5000))];
        let cortada = formatar_conversa_anterior(&longa);
        assert_eq!(cortada.chars().count(), "Agente: ".chars().count() + LIMITE_DE_CADA_FALA_EM_CARACTERES);
    }
}
