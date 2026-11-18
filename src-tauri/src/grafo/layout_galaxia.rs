//! Layout do grafo: calcula a posição 3D de cada nota dentro da galáxia.
//! Cada pasta é uma constelação que começa na sua âncora; ligações aproximam as notas e todas
//! se afastam um pouco. É determinístico (mesma entrada, mesmas posições), sem sorteio de verdade.
//! Função pura: não toca no disco.

use super::volume_da_galaxia::{ancora_do_grupo, manter_na_galaxia, Ponto};

/// O que o layout precisa saber de cada nota. A semente vem do nome da nota, então a posição
/// inicial dela não muda quando outras notas são criadas.
pub(crate) struct EntradaDeLayout {
    pub semente: u64,
    pub grupo: usize,
}

/// Distância ideal entre duas notas ligadas.
const COMPRIMENTO_IDEAL_DA_LIGACAO: f32 = 11.0;
const FORCA_DA_MOLA: f32 = 0.05;
const FORCA_DE_REPULSAO: f32 = 60.0;
/// Notas mais distantes que isso não se repelem. Deixa o cálculo mais leve.
const RAIO_DA_REPULSAO: f32 = 40.0;
const FORCA_DE_COESAO_COM_A_CONSTELACAO: f32 = 0.03;
/// Deslocamento máximo por rodada, do início ao fim. Diminui para o desenho assentar.
const PASSO_MAXIMO_INICIAL: f32 = 4.0;
const PASSO_MAXIMO_FINAL: f32 = 0.5;
/// Espalhamento da posição inicial ao redor da âncora da constelação.
const ESPALHAMENTO_INICIAL: f32 = 24.0;
/// Orçamento de cálculo: rodadas × pares de notas. Com muitas notas, há menos rodadas.
const ORCAMENTO_DE_PARES: u64 = 60_000_000;
const MINIMO_DE_RODADAS: u64 = 4;
const MAXIMO_DE_RODADAS: u64 = 80;

/// Posição de cada entrada, na mesma ordem. `ligacoes` são pares de posições na lista de entradas.
pub(crate) fn posicionar_na_galaxia(entradas: &[EntradaDeLayout], ligacoes: &[(usize, usize)]) -> Vec<Ponto> {
    let quantidade_de_grupos = entradas.iter().map(|entrada| entrada.grupo + 1).max().unwrap_or(0);
    let ancoras: Vec<Ponto> = entradas
        .iter()
        .map(|entrada| ancora_do_grupo(entrada.grupo, quantidade_de_grupos))
        .collect();
    let mut posicoes: Vec<Ponto> = entradas
        .iter()
        .zip(&ancoras)
        .map(|(entrada, ancora)| manter_na_galaxia(posicao_inicial(entrada.semente, *ancora)))
        .collect();

    let rodadas = (ORCAMENTO_DE_PARES / (entradas.len() as u64).pow(2).max(1)).clamp(MINIMO_DE_RODADAS, MAXIMO_DE_RODADAS);
    let mut forcas: Vec<Ponto> = vec![[0.0; 3]; entradas.len()];

    for rodada in 0..rodadas {
        let progresso = rodada as f32 / rodadas as f32;
        let passo_maximo = PASSO_MAXIMO_INICIAL * (1.0 - progresso) + PASSO_MAXIMO_FINAL * progresso;
        forcas.fill([0.0; 3]);
        aplicar_repulsao(&posicoes, &mut forcas);
        aplicar_molas(&posicoes, ligacoes, &mut forcas);
        aplicar_coesao_com_a_constelacao(&posicoes, &ancoras, &mut forcas);
        for (posicao, forca) in posicoes.iter_mut().zip(&forcas) {
            *posicao = manter_na_galaxia(somar(*posicao, limitar_tamanho(*forca, passo_maximo)));
        }
    }
    posicoes.into_iter().map(arredondar_para_uma_casa).collect()
}

/// Posição de partida: a âncora da constelação mais um desvio que depende só da semente.
fn posicao_inicial(semente: u64, ancora: Ponto) -> Ponto {
    let mut estado = semente;
    let mut proximo_desvio = || (proximo_numero_entre_zero_e_um(&mut estado) - 0.5) * ESPALHAMENTO_INICIAL;
    [ancora[0] + proximo_desvio(), ancora[1] + proximo_desvio(), ancora[2] + proximo_desvio()]
}

/// Gerador SplitMix64: número entre 0 e 1 a partir de um estado que avança a cada chamada.
fn proximo_numero_entre_zero_e_um(estado: &mut u64) -> f32 {
    *estado = estado.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut misturado = *estado;
    misturado = (misturado ^ (misturado >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    misturado = (misturado ^ (misturado >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    misturado ^= misturado >> 31;
    (misturado >> 40) as f32 / (1u64 << 24) as f32
}

/// Todas as notas próximas se afastam, com força que cai com o quadrado da distância.
fn aplicar_repulsao(posicoes: &[Ponto], forcas: &mut [Ponto]) {
    for primeira in 0..posicoes.len() {
        for segunda in (primeira + 1)..posicoes.len() {
            let diferenca = subtrair(posicoes[primeira], posicoes[segunda]);
            let distancia_ao_quadrado = produto_interno(diferenca, diferenca);
            if distancia_ao_quadrado == 0.0 || distancia_ao_quadrado > RAIO_DA_REPULSAO * RAIO_DA_REPULSAO {
                continue;
            }
            let distancia_ao_quadrado = distancia_ao_quadrado.max(1.0);
            let empurrao = escalar(diferenca, FORCA_DE_REPULSAO / (distancia_ao_quadrado * distancia_ao_quadrado.sqrt()));
            forcas[primeira] = somar(forcas[primeira], empurrao);
            forcas[segunda] = subtrair(forcas[segunda], empurrao);
        }
    }
}

/// Notas ligadas se puxam até a distância ideal (ou se afastam, se estiverem coladas).
fn aplicar_molas(posicoes: &[Ponto], ligacoes: &[(usize, usize)], forcas: &mut [Ponto]) {
    for &(origem, destino) in ligacoes {
        let diferenca = subtrair(posicoes[destino], posicoes[origem]);
        let distancia = produto_interno(diferenca, diferenca).sqrt().max(0.01);
        let puxao = escalar(diferenca, (distancia - COMPRIMENTO_IDEAL_DA_LIGACAO) * FORCA_DA_MOLA / distancia);
        forcas[origem] = somar(forcas[origem], puxao);
        forcas[destino] = subtrair(forcas[destino], puxao);
    }
}

/// Cada nota é puxada de leve para a âncora da própria pasta, para as constelações não se misturarem.
fn aplicar_coesao_com_a_constelacao(posicoes: &[Ponto], ancoras: &[Ponto], forcas: &mut [Ponto]) {
    for ((posicao, ancora), forca) in posicoes.iter().zip(ancoras).zip(forcas.iter_mut()) {
        *forca = somar(*forca, escalar(subtrair(*ancora, *posicao), FORCA_DE_COESAO_COM_A_CONSTELACAO));
    }
}

fn somar(a: Ponto, b: Ponto) -> Ponto {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn subtrair(a: Ponto, b: Ponto) -> Ponto {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn escalar(ponto: Ponto, fator: f32) -> Ponto {
    [ponto[0] * fator, ponto[1] * fator, ponto[2] * fator]
}

fn produto_interno(a: Ponto, b: Ponto) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Encurta o vetor até o tamanho máximo, mantendo a direção.
fn limitar_tamanho(vetor: Ponto, tamanho_maximo: f32) -> Ponto {
    let tamanho = produto_interno(vetor, vetor).sqrt();
    if tamanho > tamanho_maximo { escalar(vetor, tamanho_maximo / tamanho) } else { vetor }
}

/// Uma casa decimal basta para a tela e deixa a resposta menor.
fn arredondar_para_uma_casa(ponto: Ponto) -> Ponto {
    ponto.map(|valor| (valor * 10.0).round() / 10.0)
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::grafo::volume_da_galaxia::esta_dentro_da_galaxia;

    fn entradas_de_teste(grupos: &[usize]) -> Vec<EntradaDeLayout> {
        grupos
            .iter()
            .enumerate()
            .map(|(posicao, &grupo)| EntradaDeLayout { semente: posicao as u64 * 7919 + 13, grupo })
            .collect()
    }

    fn centro_do_grupo(posicoes: &[Ponto], entradas: &[EntradaDeLayout], grupo: usize) -> Ponto {
        let do_grupo: Vec<&Ponto> = posicoes.iter().zip(entradas).filter(|(_, e)| e.grupo == grupo).map(|(p, _)| p).collect();
        let quantidade = do_grupo.len() as f32;
        [0, 1, 2].map(|eixo| do_grupo.iter().map(|ponto| ponto[eixo]).sum::<f32>() / quantidade)
    }

    #[test]
    fn sem_notas_nao_devolve_posicoes() {
        assert!(posicionar_na_galaxia(&[], &[]).is_empty());
    }

    #[test]
    fn mesma_entrada_da_as_mesmas_posicoes() {
        let entradas = entradas_de_teste(&[0, 0, 1, 1, 2]);
        let ligacoes = [(0, 1), (2, 3), (1, 4)];
        assert_eq!(posicionar_na_galaxia(&entradas, &ligacoes), posicionar_na_galaxia(&entradas, &ligacoes));
    }

    #[test]
    fn todas_as_posicoes_sao_finitas_e_ficam_dentro_da_galaxia() {
        let entradas = entradas_de_teste(&[0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 0, 0]);
        let ligacoes: Vec<(usize, usize)> = (0..14).map(|posicao| (posicao, posicao + 1)).collect();
        for posicao in posicionar_na_galaxia(&entradas, &ligacoes) {
            assert!(posicao.iter().all(|valor| valor.is_finite()), "{posicao:?}");
            assert!(esta_dentro_da_galaxia(posicao), "{posicao:?} fora da galáxia");
        }
    }

    #[test]
    fn constelacoes_diferentes_ficam_separadas() {
        let entradas = entradas_de_teste(&[0, 0, 0, 1, 1, 1]);
        let posicoes = posicionar_na_galaxia(&entradas, &[(0, 1), (3, 4)]);
        let primeira = centro_do_grupo(&posicoes, &entradas, 0);
        let segunda = centro_do_grupo(&posicoes, &entradas, 1);
        let distancia = (0..3).map(|eixo| (primeira[eixo] - segunda[eixo]).powi(2)).sum::<f32>().sqrt();
        assert!(distancia > 50.0, "constelações a {distancia} uma da outra");
    }

    #[test]
    fn notas_na_mesma_posicao_inicial_nao_geram_valor_invalido() {
        let entradas = vec![
            EntradaDeLayout { semente: 5, grupo: 0 },
            EntradaDeLayout { semente: 5, grupo: 0 },
        ];
        let posicoes = posicionar_na_galaxia(&entradas, &[(0, 1)]);
        assert!(posicoes.iter().flatten().all(|valor| valor.is_finite()));
    }
}
