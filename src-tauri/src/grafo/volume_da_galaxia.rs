//! A forma da galáxia: um disco achatado e mais grosso no meio, com um núcleo vazio no centro
//! (o buraco negro "Home"). Diz onde cada constelação (pasta) começa e mantém qualquer ponto
//! dentro do disco e fora do núcleo. Não simula forças: isso é do layout. Função pura.

use super::contrato::VolumeDaGalaxia;

/// Posição `[x, y, z]` no espaço do grafo. O disco fica no plano x-z e a espessura é o eixo y.
pub(crate) type Ponto = [f32; 3];

/// Raio do disco da galáxia.
const RAIO_DA_GALAXIA: f32 = 130.0;

/// Metade da espessura do disco no centro. Ela diminui até zero na borda (formato de lente).
const SEMI_ESPESSURA_NO_CENTRO: f32 = 28.0;

/// Raio do núcleo: nenhuma estrela entra nele. A tela desenha o buraco negro ali.
const RAIO_DO_NUCLEO: f32 = 20.0;

/// Folga entre o núcleo e a estrela mais próxima, para nenhuma ficar colada no buraco negro.
/// Também absorve o arredondamento de uma casa decimal que o layout faz no fim.
const FOLGA_ENTRE_O_NUCLEO_E_AS_ESTRELAS: f32 = 2.0;

/// Parte do raio que as estrelas podem usar. Deixa uma folga antes da borda do disco.
const FRACAO_USAVEL_DO_RAIO: f32 = 0.92;

/// Onde ficam a primeira e a última constelação, como fração do raio. A primeira não encosta no núcleo.
const FRACAO_DO_RAIO_DA_PRIMEIRA_ANCORA: f32 = 0.38;
const FRACAO_DO_RAIO_DA_ULTIMA_ANCORA: f32 = 0.78;

/// Altura (eixo y) que separa constelações vizinhas na lista, para o desenho não ficar plano.
const DESNIVEL_ENTRE_ANCORAS: f32 = 7.0;

/// Ângulo dourado em radianos. Espalha as constelações em espiral, sem se alinharem.
const ANGULO_DOURADO: f32 = 2.399_963;

/// Forma da galáxia para a tela desenhar o núcleo e o contorno.
pub(crate) fn volume_da_galaxia() -> VolumeDaGalaxia {
    VolumeDaGalaxia {
        raio: RAIO_DA_GALAXIA,
        semi_espessura: SEMI_ESPESSURA_NO_CENTRO,
        raio_do_nucleo: RAIO_DO_NUCLEO,
    }
}

/// Distância do ponto ao eixo central do disco.
fn distancia_do_eixo(ponto: Ponto) -> f32 {
    ponto[0].hypot(ponto[2])
}

/// Metade da espessura do disco a essa distância do eixo: máxima no centro e zero na borda.
fn semi_espessura_a(distancia_do_eixo: f32) -> f32 {
    let fracao_do_raio = (distancia_do_eixo / RAIO_DA_GALAXIA).min(1.0);
    SEMI_ESPESSURA_NO_CENTRO * (1.0 - fracao_do_raio * fracao_do_raio)
}

/// Traz o ponto para dentro do disco e para fora do núcleo. Um ponto exatamente no eixo é empurrado para +x.
pub(crate) fn manter_na_galaxia(ponto: Ponto) -> Ponto {
    let limite_interno = RAIO_DO_NUCLEO + FOLGA_ENTRE_O_NUCLEO_E_AS_ESTRELAS;
    let limite_externo = RAIO_DA_GALAXIA * FRACAO_USAVEL_DO_RAIO;
    let distancia = distancia_do_eixo(ponto);

    let (x, z) = if distancia == 0.0 {
        (limite_interno, 0.0)
    } else if distancia < limite_interno || distancia > limite_externo {
        let distancia_corrigida = distancia.clamp(limite_interno, limite_externo);
        let fator = distancia_corrigida / distancia;
        (ponto[0] * fator, ponto[2] * fator)
    } else {
        (ponto[0], ponto[2])
    };

    let limite_da_altura = semi_espessura_a(x.hypot(z));
    [x, ponto[1].clamp(-limite_da_altura, limite_da_altura), z]
}

/// Ponto de partida de uma constelação. As pastas se espalham em espiral: o ângulo avança o ângulo
/// dourado a cada pasta e a distância do centro cresce com a raiz da posição, para sobrar espaço
/// também no meio do disco.
pub(crate) fn ancora_do_grupo(grupo: usize, quantidade_de_grupos: usize) -> Ponto {
    let posicao_normalizada = ((grupo as f32 + 0.5) / quantidade_de_grupos.max(1) as f32).sqrt();
    let fracao_do_raio = FRACAO_DO_RAIO_DA_PRIMEIRA_ANCORA
        + (FRACAO_DO_RAIO_DA_ULTIMA_ANCORA - FRACAO_DO_RAIO_DA_PRIMEIRA_ANCORA) * posicao_normalizada;
    let distancia = RAIO_DA_GALAXIA * fracao_do_raio;
    let angulo = grupo as f32 * ANGULO_DOURADO;
    let desnivel = ((grupo % 3) as f32 - 1.0) * DESNIVEL_ENTRE_ANCORAS;
    [distancia * angulo.cos(), desnivel, distancia * angulo.sin()]
}

/// Verdadeiro se o ponto está dentro do disco e fora do núcleo. Só os testes precisam perguntar isso.
#[cfg(test)]
pub(crate) fn esta_dentro_da_galaxia(ponto: Ponto) -> bool {
    let distancia = distancia_do_eixo(ponto);
    // Cobre o arredondamento do layout (uma casa decimal move um ponto até uns 0,07).
    let tolerancia = 0.1;
    distancia >= RAIO_DO_NUCLEO - tolerancia
        && distancia <= RAIO_DA_GALAXIA
        && ponto[1].abs() <= semi_espessura_a(distancia) + tolerancia
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn pontos_muito_longe_voltam_para_dentro_do_disco_e_fora_do_nucleo() {
        let pontos_extremos = [
            [500.0, 0.0, 0.0],
            [-500.0, 300.0, -200.0],
            [0.0, 0.0, 0.0],
            [1.0, 90.0, 1.0],
            [-80.0, -90.0, 120.0],
            [0.0, 5.0, 130.0],
        ];
        for ponto in pontos_extremos {
            let mantido = manter_na_galaxia(ponto);
            assert!(esta_dentro_da_galaxia(mantido), "{ponto:?} ficou fora: {mantido:?}");
        }
    }

    #[test]
    fn ponto_ja_dentro_nao_se_move() {
        let ponto = [60.0, 3.0, -40.0];
        assert_eq!(manter_na_galaxia(ponto), ponto);
    }

    #[test]
    fn a_espessura_cai_do_centro_para_a_borda() {
        assert!(semi_espessura_a(0.0) > semi_espessura_a(RAIO_DA_GALAXIA / 2.0));
        assert!(semi_espessura_a(RAIO_DA_GALAXIA / 2.0) > semi_espessura_a(RAIO_DA_GALAXIA));
        assert_eq!(semi_espessura_a(RAIO_DA_GALAXIA * 2.0), 0.0);
    }

    #[test]
    fn ancoras_ficam_dentro_e_nunca_juntas() {
        let ancoras: Vec<Ponto> = (0..12).map(|grupo| ancora_do_grupo(grupo, 12)).collect();
        for ancora in &ancoras {
            assert!(esta_dentro_da_galaxia(*ancora), "{ancora:?}");
        }
        for primeira in 0..ancoras.len() {
            for segunda in (primeira + 1)..ancoras.len() {
                let distancia = (0..3).map(|eixo| (ancoras[primeira][eixo] - ancoras[segunda][eixo]).powi(2)).sum::<f32>().sqrt();
                assert!(distancia > 15.0, "âncoras {primeira} e {segunda} a {distancia}");
            }
        }
    }

    #[test]
    fn um_unico_grupo_tem_ancora_valida() {
        let ancora = ancora_do_grupo(0, 1);
        assert!(ancora.iter().all(|valor| valor.is_finite()));
        assert!(esta_dentro_da_galaxia(ancora));
    }

    #[test]
    fn o_volume_enviado_para_a_tela_e_o_mesmo_que_limita_os_pontos() {
        let volume = volume_da_galaxia();
        assert!(esta_dentro_da_galaxia([volume.raio, 0.0, 0.0]));
        assert!(!esta_dentro_da_galaxia([volume.raio + 1.0, 0.0, 0.0]));
        assert!(!esta_dentro_da_galaxia([volume.raio_do_nucleo - 1.0, 0.0, 0.0]));
        // O disco é uma lente: no centro tem a espessura total, mas na borda do núcleo já é um pouco mais fino.
        assert!(esta_dentro_da_galaxia([0.0, volume.semi_espessura - 1.0, volume.raio_do_nucleo]));
        assert!(!esta_dentro_da_galaxia([0.0, volume.semi_espessura + 1.0, volume.raio_do_nucleo]));
    }

    #[test]
    fn nenhuma_estrela_fica_colada_no_nucleo() {
        let mantido = manter_na_galaxia([0.5, 0.0, 0.5]);
        assert!(distancia_do_eixo(mantido) >= RAIO_DO_NUCLEO + FOLGA_ENTRE_O_NUCLEO_E_AS_ESTRELAS - 0.001);
    }
}
