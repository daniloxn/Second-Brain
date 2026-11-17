// Formulário para criar um cofre novo: nome e local. O local começa em Documentos
// e pode ser trocado pela janela do sistema. Os erros do backend aparecem aqui.

import { useEffect, useState, type FormEvent } from "react";
import { FolderOpen } from "lucide-react";
import { escolherPastaDoCofre, pastaDeDocumentos } from "./chamadasCofre";

/** Nome sugerido para o cofre novo. */
const NOME_SUGERIDO_DO_COFRE = "Segundo Cérebro";

interface Props {
  /** Tenta criar o cofre. Lança erro com a mensagem a mostrar. */
  aoCriarCofre: (pastaPai: string, nomeDoCofre: string) => Promise<void>;
}

export function FormularioCriarCofre({ aoCriarCofre }: Props) {
  const [nomeDigitado, setNomeDigitado] = useState(NOME_SUGERIDO_DO_COFRE);
  const [pastaPai, setPastaPai] = useState<string | null>(null);
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  const [estaEnviando, setEstaEnviando] = useState(false);

  // Começa em Documentos, que é o lugar mais previsível para o usuário.
  useEffect(() => {
    pastaDeDocumentos()
      .then(setPastaPai)
      .catch(() => setPastaPai(null));
  }, []);

  async function escolherOutroLocal() {
    const novoLocal = await escolherPastaDoCofre("Escolha onde criar o cofre");
    if (novoLocal) setPastaPai(novoLocal);
  }

  async function enviarFormulario(evento: FormEvent) {
    evento.preventDefault();
    if (estaEnviando || !pastaPai) return;
    setEstaEnviando(true);
    setMensagemDeErro(null);
    try {
      await aoCriarCofre(pastaPai, nomeDigitado);
    } catch (erro) {
      setMensagemDeErro(String(erro));
    } finally {
      setEstaEnviando(false);
    }
  }

  return (
    <form className="form-criar-cofre" onSubmit={enviarFormulario}>
      <label className="campo-rotulado">
        <span>Nome do cofre</span>
        <input
          className="campo"
          type="text"
          value={nomeDigitado}
          aria-invalid={mensagemDeErro ? true : undefined}
          onChange={(evento) => setNomeDigitado(evento.target.value)}
        />
      </label>

      <div className="campo-rotulado">
        <span>Local</span>
        <div className="linha-local">
          <span className="caminho-local" title={pastaPai ?? ""}>{pastaPai ?? "Carregando…"}</span>
          <button type="button" className="botao" onClick={escolherOutroLocal}>
            <FolderOpen size={15} aria-hidden="true" />
            Outro local
          </button>
        </div>
      </div>

      <p className="suave pequeno">
        O app cria a pasta do cofre com as pastas dele dentro. Se já existir uma pasta com esse nome no local, nada é criado.
      </p>

      <button type="submit" className="botao botao-primario" disabled={estaEnviando || !pastaPai}>
        Criar cofre
      </button>
      {mensagemDeErro && <p role="alert" className="erro">{mensagemDeErro}</p>}
    </form>
  );
}
