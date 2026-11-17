// Formulário para criar uma nota nova pelo nome. Mostra a mensagem do backend se o nome for recusado.

import { useState, type FormEvent, type KeyboardEvent } from "react";

interface Props {
  /** Tenta criar a nota. Deve lançar erro com a mensagem a mostrar. */
  aoEnviarNome: (nomeDigitado: string) => Promise<void>;
  aoCancelar: () => void;
}

export function FormularioNovaNota({ aoEnviarNome, aoCancelar }: Props) {
  const [nomeDigitado, setNomeDigitado] = useState("");
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  const [estaEnviando, setEstaEnviando] = useState(false);

  async function enviarFormulario(evento: FormEvent) {
    evento.preventDefault();
    if (estaEnviando) return;
    setEstaEnviando(true);
    setMensagemDeErro(null);
    try {
      await aoEnviarNome(nomeDigitado);
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
    <form className="form-nova-nota" onSubmit={enviarFormulario}>
      <input
        autoFocus
        className="campo"
        type="text"
        value={nomeDigitado}
        placeholder="Nome da nota"
        aria-label="Nome da nota"
        aria-invalid={mensagemDeErro ? true : undefined}
        onChange={(evento) => setNomeDigitado(evento.target.value)}
        onKeyDown={aoPressionarTeclaNoCampo}
      />
      <p className="suave pequeno">Será criada em notas/anotacoes.</p>
      <div className="linha-acoes">
        <button type="submit" className="botao botao-primario" disabled={estaEnviando}>
          Criar
        </button>
        <button type="button" className="botao" onClick={aoCancelar}>
          Cancelar
        </button>
      </div>
      {mensagemDeErro && <p role="alert" className="erro">{mensagemDeErro}</p>}
    </form>
  );
}
