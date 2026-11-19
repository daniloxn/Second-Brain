//! Funcionalidade "notas": arquivos .md do cofre, validação de nomes,
//! wikilinks e backlinks. Cada parte fica em seu próprio arquivo.
//! `indice` e `wikilinks` são abertos ao crate porque o grafo reaproveita a busca de notas
//! e a leitura de links; o resto continua privado.

pub mod comandos;
mod backlinks;
mod conflito;
pub(crate) mod consulta;
pub(crate) mod indice;
mod nome_da_nota;
mod versao;
pub(crate) mod wikilinks;
