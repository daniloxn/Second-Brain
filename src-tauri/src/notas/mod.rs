//! Funcionalidade "notas": arquivos .md do cofre, validação de nomes,
//! wikilinks e backlinks. Cada parte fica em seu próprio arquivo.

pub mod comandos;
mod backlinks;
mod conflito;
mod indice;
mod nome_da_nota;
mod versao;
mod wikilinks;
