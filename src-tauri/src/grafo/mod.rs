//! Funcionalidade "grafo": transforma as notas e tarefas do cofre em pontos e ligações
//! com posição 3D em formato de galáxia (cada pasta é uma constelação). Só o comando fica aberto;
//! cada etapa é um arquivo. A tela desenha e filtra; o cálculo todo fica aqui, onde é testado.

pub mod comandos;
mod contrato;
mod layout_galaxia;
mod leitura;
mod ligacoes;
mod montagem;
mod volume_da_galaxia;
