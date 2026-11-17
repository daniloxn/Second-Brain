//! Estrutura padrão de uma pasta de cofre (ver PLANO.md, seção 3).
//! Cria as pastas que faltam e o AGENTS.md, sem sobrescrever nada que já exista.

use std::fs;
use std::path::Path;

/// Pastas que todo cofre precisa ter, relativas à raiz.
const PASTAS_PADRAO_DO_COFRE: &[&str] = &[
    "_sistema",
    "_segredos",
    "tarefas",
    "notas/ideias",
    "notas/estudos",
    "notas/anotacoes",
    "documentos",
    "projetos",
    "prompts",
    "skills",
];

/// Conteúdo inicial do AGENTS.md, com as regras que qualquer agente deve seguir ao mexer no cofre.
const CONTEUDO_AGENTS_MD: &str = "# Regras para agentes

Este é o cofre do Segundo Cérebro. Antes de alterar qualquer coisa:

- Tarefas ficam em `tarefas/` com frontmatter: `status`, `prioridade`, `prazo`, `origem`.
- Notas usam wikilinks `[[nome-da-nota]]` para se conectarem.
- Projetos de programação ficam em `projetos/<nome>/` com `PROJETO.md`, `MAPA.md` e `LOG.md`.
- Nunca grave senhas em texto puro. Segredos ficam em `_segredos/`, criptografados.
- Não apague arquivos sem confirmação do usuário.
";

/// Pasta que só existe em cofres criados pelo app. É a marca que diz "este é um cofre do Segundo Cérebro".
pub(crate) const NOME_DA_PASTA_DO_SISTEMA: &str = "_sistema";

/// Verdadeiro se a pasta foi criada pelo app (tem a pasta `_sistema`).
/// Usado para não tratar uma pasta comum, como um cofre de Obsidian, como se fosse nossa.
pub(crate) fn e_cofre_do_app(pasta: &Path) -> bool {
    pasta.join(NOME_DA_PASTA_DO_SISTEMA).is_dir()
}

/// Cria a pasta raiz (se preciso), as pastas padrão e o AGENTS.md.
/// Não altera arquivos que já existam.
pub(crate) fn criar_estrutura_padrao_do_cofre(pasta_raiz: &Path) -> Result<(), String> {
    if pasta_raiz.exists() && !pasta_raiz.is_dir() {
        return Err(format!("{} não é uma pasta", pasta_raiz.display()));
    }
    fs::create_dir_all(pasta_raiz).map_err(|e| e.to_string())?;

    for pasta_relativa in PASTAS_PADRAO_DO_COFRE {
        fs::create_dir_all(pasta_raiz.join(pasta_relativa)).map_err(|e| e.to_string())?;
    }

    let arquivo_agents = pasta_raiz.join("AGENTS.md");
    if !arquivo_agents.exists() {
        fs::write(&arquivo_agents, CONTEUDO_AGENTS_MD).map_err(|e| e.to_string())?;
    }
    Ok(())
}
