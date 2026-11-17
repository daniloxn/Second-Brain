// Formulário para criar uma pasta dentro do cofre. Aceita caminho com vários níveis, como `projetos/app`.

import { useState, type FormEvent } from "react";
import { FolderPlus } from "lucide-react";

interface Props {
  /** Tenta criar a pasta. Lança erro com a mensagem a mostrar. */
  aoCriarPasta: (caminhoRelativo: string) => Promise<void>;
}

export function FormularioNovaPastaNoCofre({ aoCriarPasta }: Props) {
  const [caminhoDigitado, setCaminhoDigitado] = useState("");
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  const [estaEnviando, setEstaEnviando] = useState(false);

  async function enviarFormulario(evento: FormEvent) {
    evento.preventDefault();
    if (estaEnviando) return;
    setEstaEnviando(true);
    setMensagemDeErro(null);
    try {
      await aoCriarPasta(caminhoDigitado);
      setCaminhoDigitado("");
    } catch (erro) {
      setMensagemDeErro(String(erro));
    } finally {
      setEstaEnviando(false);
    }
  }

  return (
    <form className="form-nova-pasta" onSubmit={enviarFormulario}>
      <input
        className="campo"
        type="text"
        value={caminhoDigitado}
        placeholder="Nova pasta (ex.: projetos/meu-app)"
        aria-label="Caminho da nova pasta dentro do cofre"
        aria-invalid={mensagemDeErro ? true : undefined}
        onChange={(evento) => setCaminhoDigitado(evento.target.value)}
      />
      <button type="submit" className="botao" disabled={estaEnviando || caminhoDigitado.trim() === ""}>
        <FolderPlus size={15} aria-hidden="true" />
        Criar pasta
      </button>
      {mensagemDeErro && <p role="alert" className="erro">{mensagemDeErro}</p>}
    </form>
  );
}
