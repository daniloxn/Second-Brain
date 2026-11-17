// Tela inicial, mostrada sem cofre aberto: criar um cofre novo, abrir uma pasta que já é cofre,
// ou confirmar a transformação de uma pasta comum em cofre.

import { Brain, FolderOpen } from "lucide-react";
import { FormularioCriarCofre } from "./FormularioCriarCofre";

interface Props {
  mensagemDeErro: string | null;
  /** Pasta comum que o usuário escolheu e que precisa de confirmação. */
  pastaAguardandoConfirmacao: string | null;
  aoEscolherPastaExistente: () => void;
  aoConfirmarConversao: () => void;
  aoCancelarConversao: () => void;
  aoCriarCofre: (pastaPai: string, nomeDoCofre: string) => Promise<void>;
}

export function SeletorDeCofre({
  mensagemDeErro,
  pastaAguardandoConfirmacao,
  aoEscolherPastaExistente,
  aoConfirmarConversao,
  aoCancelarConversao,
  aoCriarCofre,
}: Props) {
  return (
    <main className="tela-inicial">
      <div className="boas-vindas">
        <Brain size={40} aria-hidden="true" className="marca" />
        <h1>Segundo Cérebro</h1>
        <p className="suave">Crie um cofre novo para o app, ou abra uma pasta que já é um cofre dele.</p>
      </div>

      {pastaAguardandoConfirmacao && (
        <section className="caixa-confirmacao" role="alertdialog" aria-labelledby="titulo-confirmacao">
          <h2 id="titulo-confirmacao" className="titulo-caixa">Esta pasta não é um cofre do app</h2>
          <p className="suave pequeno">{pastaAguardandoConfirmacao}</p>
          <p>
            Se continuar, o app cria as pastas dele dentro dela. Nada que já existe será apagado ou movido.
          </p>
          <div className="linha-acoes">
            <button type="button" className="botao botao-primario" onClick={aoConfirmarConversao}>
              Criar as pastas do app aqui
            </button>
            <button type="button" className="botao" onClick={aoCancelarConversao}>
              Cancelar
            </button>
          </div>
        </section>
      )}

      <div className="colunas-inicio">
        <section className="cartao-inicio" aria-labelledby="titulo-criar">
          <h2 id="titulo-criar" className="titulo-caixa">Criar um cofre novo</h2>
          <FormularioCriarCofre aoCriarCofre={aoCriarCofre} />
        </section>

        <section className="cartao-inicio" aria-labelledby="titulo-abrir">
          <h2 id="titulo-abrir" className="titulo-caixa">Abrir um cofre existente</h2>
          <p className="suave pequeno">Escolha uma pasta que já foi criada pelo app.</p>
          <button type="button" className="botao" onClick={aoEscolherPastaExistente}>
            <FolderOpen size={15} aria-hidden="true" />
            Escolher pasta
          </button>
        </section>
      </div>

      {mensagemDeErro && <p role="alert" className="erro">{mensagemDeErro}</p>}
    </main>
  );
}
