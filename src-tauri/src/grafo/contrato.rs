//! Formatos que o grafo envia para a tela, em camelCase como nas outras funcionalidades.
//! Mudar um campo aqui exige mudar `src/grafo/contratoGrafo.ts` na mesma alteração.

use serde::Serialize;

/// Tipo de um ponto do grafo. Tarefas são arquivos .md em `tarefas/`; o resto é nota.
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TipoDoNo {
    Nota,
    Tarefa,
}

/// Uma nota ou tarefa no grafo, já com a posição no espaço 3D.
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NoDoGrafo {
    /// Caminho completo do arquivo, o mesmo que a tela usa para abrir a nota.
    pub caminho: String,
    /// Nome sem extensão.
    pub nome: String,
    /// Pasta relativa ao cofre, com `/`. Vazio quando a nota está na raiz.
    pub pasta: String,
    pub tipo: TipoDoNo,
    /// Quantidade de ligações que entram ou saem deste ponto.
    pub conexoes: usize,
    /// Posição `[x, y, z]` dentro do disco da galáxia.
    pub posicao: [f32; 3],
}

/// Uma ligação por wikilink. `origem` e `destino` são posições na lista `nos`.
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LigacaoDoGrafo {
    pub origem: usize,
    pub destino: usize,
}

/// Forma da galáxia onde as notas foram posicionadas: um disco no plano x-z, mais grosso no meio,
/// com um núcleo vazio no centro. A tela usa estes números para desenhar o buraco negro e as órbitas;
/// quem manda na forma é o backend.
#[derive(Serialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VolumeDaGalaxia {
    /// Raio do disco.
    pub raio: f32,
    /// Metade da espessura do disco no centro.
    pub semi_espessura: f32,
    /// Raio do núcleo, onde nenhuma nota entra.
    pub raio_do_nucleo: f32,
}

/// Grafo inteiro do cofre. `truncado` avisa que havia mais notas do que o limite do grafo.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrafoDoCofre {
    pub nos: Vec<NoDoGrafo>,
    pub ligacoes: Vec<LigacaoDoGrafo>,
    pub volume: VolumeDaGalaxia,
    pub truncado: bool,
}
