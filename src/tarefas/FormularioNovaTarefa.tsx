// Formulário para criar uma tarefa: título, prioridade, prazo (data e hora opcional) e tags.
// Monta o objeto do contrato; não sabe validar regras de negócio, isso é do backend.

import { useState, type FormEvent, type KeyboardEvent } from "react";
import type { NovaTarefaParaCriar, PrioridadeDaTarefa } from "./contratoTarefas";
import { rotuloDaPrioridade } from "./formatacaoTarefas";

interface Props {
  /** Tenta criar a tarefa. Lança erro com a mensagem a mostrar. */
  aoEnviar: (entrada: NovaTarefaParaCriar) => Promise<void>;
  aoCancelar: () => void;
}

const PRIORIDADES_PARA_ESCOLHER: PrioridadeDaTarefa[] = ["alta", "media", "baixa"];

/** Separa as tags digitadas por vírgula, descartando vazios. */
function tagsDoTexto(textoDigitado: string): string[] {
  return textoDigitado
    .split(",")
    .map((tag) => tag.trim())
    .filter((tag) => tag.length > 0);
}

export function FormularioNovaTarefa({ aoEnviar, aoCancelar }: Props) {
  const [titulo, setTitulo] = useState("");
  const [prioridade, setPrioridade] = useState<PrioridadeDaTarefa>("media");
  const [dataDoPrazo, setDataDoPrazo] = useState("");
  const [horaDoPrazo, setHoraDoPrazo] = useState("");
  const [tagsDigitadas, setTagsDigitadas] = useState("");
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  const [estaEnviando, setEstaEnviando] = useState(false);

  /** Junta data e hora no formato que o backend espera. Sem data, não há prazo. */
  function montarPrazo(): string | undefined {
    if (!dataDoPrazo) return undefined;
    return horaDoPrazo ? `${dataDoPrazo} ${horaDoPrazo}` : dataDoPrazo;
  }

  async function enviarFormulario(evento: FormEvent) {
    evento.preventDefault();
    if (estaEnviando) return;
    setEstaEnviando(true);
    setMensagemDeErro(null);
    try {
      await aoEnviar({
        titulo,
        prioridade,
        prazo: montarPrazo(),
        tags: tagsDoTexto(tagsDigitadas),
      });
    } catch (erro) {
      setMensagemDeErro(String(erro));
    } finally {
      setEstaEnviando(false);
    }
  }

  // Esc fecha o formulário sem criar nada.
  function aoPressionarTeclaNoCampo(evento: KeyboardEvent<HTMLInputElement>) {
    if (evento.key === "Escape") aoCancelar();
  }

  return (
    <form className="form-nova-tarefa" onSubmit={enviarFormulario}>
      <input
        autoFocus
        className="campo"
        type="text"
        value={titulo}
        placeholder="O que precisa ser feito?"
        aria-label="Título da tarefa"
        aria-invalid={mensagemDeErro ? true : undefined}
        onChange={(evento) => setTitulo(evento.target.value)}
        onKeyDown={aoPressionarTeclaNoCampo}
      />

      <div className="linha-campos">
        <label className="campo-rotulado">
          <span>Prioridade</span>
          <select className="campo" value={prioridade} onChange={(evento) => setPrioridade(evento.target.value as PrioridadeDaTarefa)}>
            {PRIORIDADES_PARA_ESCOLHER.map((opcao) => (
              <option key={opcao} value={opcao}>{rotuloDaPrioridade(opcao)}</option>
            ))}
          </select>
        </label>

        <label className="campo-rotulado">
          <span>Prazo</span>
          <input className="campo" type="date" value={dataDoPrazo} onChange={(evento) => setDataDoPrazo(evento.target.value)} />
        </label>

        <label className="campo-rotulado">
          <span>Hora</span>
          <input
            className="campo"
            type="time"
            value={horaDoPrazo}
            disabled={!dataDoPrazo}
            onChange={(evento) => setHoraDoPrazo(evento.target.value)}
          />
        </label>
      </div>

      <label className="campo-rotulado">
        <span>Tags (separadas por vírgula)</span>
        <input className="campo" type="text" value={tagsDigitadas} placeholder="oficina, dados" onChange={(evento) => setTagsDigitadas(evento.target.value)} />
      </label>

      <div className="linha-acoes">
        <button type="submit" className="botao botao-primario" disabled={estaEnviando}>
          Criar tarefa
        </button>
        <button type="button" className="botao" onClick={aoCancelar}>
          Cancelar
        </button>
      </div>
      {mensagemDeErro && <p role="alert" className="erro">{mensagemDeErro}</p>}
    </form>
  );
}
