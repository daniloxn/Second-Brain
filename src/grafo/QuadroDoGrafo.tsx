// Quadro do grafo: cria a cena 3D dentro de uma div, aplica a visão atual, mostra o nome da nota
// sob o mouse e os controles flutuantes (aproximar, afastar, reiniciar, pausar, tela cheia).
// Cuida só do ciclo de vida da cena; as regras ficam em outros arquivos.

import { useEffect, useRef, useState } from "react";
import { Maximize, Minimize, Minus, Pause, Play, Plus, RotateCcw } from "lucide-react";
import { criarCenaDoGrafo, type CenaDoGrafo } from "./cenaDoGrafo";
import type { GrafoDoCofre } from "./contratoGrafo";
import type { VisaoDoGrafo } from "./filtrosDoGrafo";

interface Props {
  grafo: GrafoDoCofre;
  visao: VisaoDoGrafo;
  aoAbrirNota: (caminhoDaNota: string) => void;
}

/** Estrela sob o mouse e onde mostrar a dica, em pixels dentro do quadro. */
interface DicaSobOMouse {
  posicaoDoNo: number;
  x: number;
  y: number;
}

export function QuadroDoGrafo({ grafo, visao, aoAbrirNota }: Props) {
  const quadro = useRef<HTMLDivElement>(null);
  const recipienteDaCena = useRef<HTMLDivElement>(null);
  const cenaCriada = useRef<CenaDoGrafo | null>(null);
  // O clique usa a função mais recente sem recriar a cena a cada renderização.
  const aoAbrirNotaAtual = useRef(aoAbrirNota);
  aoAbrirNotaAtual.current = aoAbrirNota;
  const [dica, setDica] = useState<DicaSobOMouse | null>(null);
  const [erroDeInicio, setErroDeInicio] = useState<string | null>(null);
  const [pausado, setPausado] = useState(false);
  const [emTelaCheia, setEmTelaCheia] = useState(false);

  // Cria a cena quando o grafo muda e a destrói ao sair. A visão é aplicada no efeito seguinte.
  useEffect(() => {
    if (!recipienteDaCena.current) return;
    try {
      const cena = criarCenaDoGrafo(recipienteDaCena.current, grafo, {
        aoPassarSobreNo: (posicaoDoNo, x, y) => setDica(posicaoDoNo === null ? null : { posicaoDoNo, x, y }),
        aoClicarNoNo: (posicaoDoNo) => aoAbrirNotaAtual.current(grafo.nos[posicaoDoNo].caminho),
      });
      cenaCriada.current = cena;
      setPausado(cena.estaPausado());
      setErroDeInicio(null);
    } catch (erro) {
      setErroDeInicio(`Não foi possível iniciar o gráfico 3D: ${String(erro)}`);
    }
    return () => {
      cenaCriada.current?.destruir();
      cenaCriada.current = null;
      setDica(null);
    };
  }, [grafo]);

  useEffect(() => {
    cenaCriada.current?.aplicarVisao(visao);
  }, [grafo, visao]);

  // O usuário também sai da tela cheia pela tecla Esc, então o ícone segue o estado real do navegador.
  useEffect(() => {
    const acompanharTelaCheia = () => setEmTelaCheia(document.fullscreenElement === quadro.current);
    document.addEventListener("fullscreenchange", acompanharTelaCheia);
    return () => document.removeEventListener("fullscreenchange", acompanharTelaCheia);
  }, []);

  /** Entra ou sai da tela cheia. Se o sistema recusar, nada muda. */
  async function alternarTelaCheia() {
    try {
      if (document.fullscreenElement) await document.exitFullscreen();
      else await quadro.current?.requestFullscreen();
    } catch {
      // Recusa do sistema (ex.: janela sem permissão): o botão simplesmente não faz nada.
    }
  }

  const noDaDica = dica ? grafo.nos[dica.posicaoDoNo] : null;

  return (
    <div ref={quadro} className="grafo-quadro">
      <div ref={recipienteDaCena} className="grafo-cena" role="img" aria-label="Universo 3D das notas e das ligações entre elas" />
      {erroDeInicio && <p role="alert" className="erro grafo-aviso">{erroDeInicio}</p>}
      {dica && noDaDica && (
        <div className="grafo-dica" style={{ left: dica.x + 14, top: dica.y + 14 }}>
          <strong>{noDaDica.nome}</strong>
          <span>{noDaDica.pasta === "" ? "raiz do cofre" : noDaDica.pasta}</span>
          <span>{noDaDica.conexoes === 1 ? "1 ligação" : `${noDaDica.conexoes} ligações`}</span>
        </div>
      )}
      <div className="grafo-controles" role="group" aria-label="Controles da vista">
        <button type="button" className="grafo-controle" aria-label="Afastar" onClick={() => cenaCriada.current?.afastar()}>
          <Minus size={16} aria-hidden="true" />
        </button>
        <button type="button" className="grafo-controle" aria-label="Aproximar" onClick={() => cenaCriada.current?.aproximar()}>
          <Plus size={16} aria-hidden="true" />
        </button>
        <button type="button" className="grafo-controle" aria-label="Reiniciar a vista" onClick={() => cenaCriada.current?.reiniciarVista()}>
          <RotateCcw size={16} aria-hidden="true" />
        </button>
        <button
          type="button"
          className="grafo-controle"
          aria-label={pausado ? "Retomar o movimento" : "Pausar o movimento"}
          onClick={() => setPausado(cenaCriada.current?.alternarPausa() ?? false)}
        >
          {pausado ? <Play size={16} aria-hidden="true" /> : <Pause size={16} aria-hidden="true" />}
        </button>
        <button type="button" className="grafo-controle" aria-label={emTelaCheia ? "Sair da tela cheia" : "Tela cheia"} onClick={alternarTelaCheia}>
          {emTelaCheia ? <Minimize size={16} aria-hidden="true" /> : <Maximize size={16} aria-hidden="true" />}
        </button>
      </div>
    </div>
  );
}
