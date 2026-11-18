// Formulário para criar uma pasta de projeto com documentos iniciais para o agente.

import { useState, type FormEvent } from "react";

interface Props {
  aoCriar: (nome: string) => Promise<void>;
}

/** Valida que o nome não está vazio antes de pedir a criação ao backend. */
export function FormularioNovoProjeto({ aoCriar }: Props) {
  const [nomeDigitado, setNomeDigitado] = useState("");
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  const [estaCriando, setEstaCriando] = useState(false);

  /** Envia o nome, mantendo o formulário aberto quando a pasta já existe ou é inválida. */
  async function enviarFormulario(evento: FormEvent) {
    evento.preventDefault();
    if (estaCriando || !nomeDigitado.trim()) return;
    setEstaCriando(true);
    setMensagemDeErro(null);
    try {
      await aoCriar(nomeDigitado);
      setNomeDigitado("");
    } catch (erro) {
      setMensagemDeErro(String(erro));
    } finally {
      setEstaCriando(false);
    }
  }

  return (
    <form className="form-novo-projeto" onSubmit={enviarFormulario}>
      <label className="campo-rotulado">
        <span>Novo projeto</span>
        <input className="campo" value={nomeDigitado} maxLength={100} placeholder="Ex.: painel de produção" onChange={(evento) => setNomeDigitado(evento.target.value)} />
      </label>
      <button className="botao botao-primario" type="submit" disabled={estaCriando || !nomeDigitado.trim()}>Criar projeto</button>
      {mensagemDeErro && <p role="alert" className="erro">{mensagemDeErro}</p>}
    </form>
  );
}
