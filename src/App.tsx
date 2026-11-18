// Composição da interface: junta o cofre aberto, a navegação e a tela de cada seção.
// A lógica de cada funcionalidade fica nas pastas dela; aqui só se decide o que mostrar.

import { lazy, Suspense, useState } from "react";
import { useCofreAberto } from "./cofre/useCofreAberto";
import { SeletorDeCofre } from "./cofre/SeletorDeCofre";
import { VisaoGeralDoCofre } from "./cofre/VisaoGeralDoCofre";
import { ExploradorDeArquivos } from "./arquivos/ExploradorDeArquivos";
import { ListaDeNotas } from "./notas/ListaDeNotas";
import { TelaDeTarefas } from "./tarefas/TelaDeTarefas";
import { TelaDoAgente } from "./agente/TelaDoAgente";
import { abrirOuCriarNotaPeloLink, criarNovaNotaNaPastaPadrao } from "./notas/acoesDeNota";
import { nomeDaNotaNoAlvo } from "./notas/nomesDeNota";
import { TrilhoDeSecoes } from "./navegacao/TrilhoDeSecoes";
import { CabecalhoDaSecao } from "./navegacao/CabecalhoDaSecao";
import { SecaoEmBreve } from "./navegacao/SecaoEmBreve";
import { acharSecaoPeloId, SECAO_INICIAL, type IdDaSecao } from "./navegacao/secoes";

// Editor e renderer de Markdown pesam: carregam só quando uma nota é aberta.
const NotaAbertaSobDemanda = lazy(() =>
  import("./notas/NotaAberta").then((modulo) => ({ default: modulo.NotaAberta })),
);

// A biblioteca 3D (three) é a parte mais pesada do app: carrega só quando o grafo é aberto.
const TelaDoGrafoSobDemanda = lazy(() =>
  import("./grafo/TelaDoGrafo").then((modulo) => ({ default: modulo.TelaDoGrafo })),
);

export default function App() {
  const [idDaSecaoAtiva, setIdDaSecaoAtiva] = useState<IdDaSecao>(SECAO_INICIAL);
  const [caminhoDaNotaAberta, setCaminhoDaNotaAberta] = useState<string | null>(null);
  // Muda a cada "Atualizar", para telas com dados próprios montarem de novo e buscarem.
  const [versaoDaAtualizacao, setVersaoDaAtualizacao] = useState(0);
  const cofreAberto = useCofreAberto({ aoAbrirCofre: () => setCaminhoDaNotaAberta(null) });

  /** Abre uma nota na seção Notas. */
  function abrirNota(caminhoDaNota: string) {
    setIdDaSecaoAtiva("notas");
    setCaminhoDaNotaAberta(caminhoDaNota);
  }

  /** Cria uma nota pelo formulário da lista e a abre. Erros sobem para o formulário. */
  async function criarNotaPelaLista(nomeDigitado: string) {
    if (!cofreAberto.pastaDoCofre) return;
    const caminhoDaNovaNota = await criarNovaNotaNaPastaPadrao(cofreAberto.pastaDoCofre, nomeDigitado);
    await cofreAberto.atualizarIndices();
    abrirNota(caminhoDaNovaNota);
  }

  /** Abre a nota de um wikilink clicado. Cria a nota se ela não existir. */
  async function abrirNotaDoWikilink(alvoDoLink: string) {
    if (!cofreAberto.pastaDoCofre) return;
    try {
      const caminhoDaNota = await abrirOuCriarNotaPeloLink(cofreAberto.pastaDoCofre, alvoDoLink);
      await cofreAberto.atualizarIndices();
      abrirNota(caminhoDaNota);
    } catch (erro) {
      cofreAberto.informarErro(`Não foi possível abrir "${nomeDaNotaNoAlvo(alvoDoLink)}": ${String(erro)}`);
    }
  }

  /**
   * Recarrega a árvore, as notas e as telas que buscam dados por conta própria
   * (trocando a `key` da tela de tarefas, ela monta de novo e busca a lista).
   */
  async function atualizarTudo() {
    try {
      await cofreAberto.atualizarIndices();
      setVersaoDaAtualizacao((versaoAnterior) => versaoAnterior + 1);
      cofreAberto.informarErro(null);
    } catch (erro) {
      cofreAberto.informarErro(String(erro));
    }
  }

  if (cofreAberto.carregandoInicio) {
    return (
      <main className="tela-cheia">
        <p className="suave">Abrindo seu cofre…</p>
      </main>
    );
  }

  const pastaDoCofre = cofreAberto.pastaDoCofre;
  const arvoreDoCofre = cofreAberto.arvoreDoCofre;
  // Sem cofre aberto, ou com uma pasta comum esperando confirmação, a tela inicial aparece.
  const deveMostrarTelaInicial = !pastaDoCofre || !arvoreDoCofre || cofreAberto.pastaAguardandoConfirmacao !== null;
  if (deveMostrarTelaInicial) {
    return (
      <SeletorDeCofre
        mensagemDeErro={cofreAberto.mensagemDeErro}
        pastaAguardandoConfirmacao={cofreAberto.pastaAguardandoConfirmacao}
        aoEscolherPastaExistente={cofreAberto.escolherPasta}
        aoConfirmarConversao={cofreAberto.confirmarConversaoDaPasta}
        aoCancelarConversao={cofreAberto.cancelarConversaoDaPasta}
        aoCriarCofre={cofreAberto.criarCofreNovo}
      />
    );
  }

  const secaoAtiva = acharSecaoPeloId(idDaSecaoAtiva);
  const estaLendoUmaNota = idDaSecaoAtiva === "notas" && caminhoDaNotaAberta !== null;
  const nomeDoCofre = pastaDoCofre.split(/[\\/]/).filter(Boolean).pop() ?? pastaDoCofre;
  // Nota aberta e grafo ocupam a área toda, sem margem nem rolagem própria da página.
  const classeDoConteudo = estaLendoUmaNota
    ? "conteudo conteudo-nota"
    : idDaSecaoAtiva === "grafo"
      ? "conteudo conteudo-grafo"
      : "conteudo";

  /** Tela da área principal, de acordo com a seção escolhida. */
  const conteudoDaSecao = () => {
    if (idDaSecaoAtiva === "arquivos") {
      return <VisaoGeralDoCofre arvoreDoCofre={arvoreDoCofre} aoCriarPasta={cofreAberto.criarPastaNoCofreAtual} />;
    }
    if (idDaSecaoAtiva === "notas" && caminhoDaNotaAberta) {
      return (
        <Suspense fallback={<p className="suave">Carregando editor…</p>}>
          <NotaAbertaSobDemanda
            key={caminhoDaNotaAberta}
            caminhoDaNota={caminhoDaNotaAberta}
            pastaDoCofre={pastaDoCofre}
            aoClicarNoWikilink={abrirNotaDoWikilink}
            aoClicarNoBacklink={abrirNota}
            aoVoltarParaLista={() => setCaminhoDaNotaAberta(null)}
          />
        </Suspense>
      );
    }
    if (idDaSecaoAtiva === "notas") {
      return (
        <ListaDeNotas
          notas={cofreAberto.indiceDeNotas}
          pastaDoCofre={pastaDoCofre}
          aoAbrirNota={abrirNota}
          aoCriarNota={criarNotaPelaLista}
        />
      );
    }
    if (idDaSecaoAtiva === "tarefas") {
      return <TelaDeTarefas key={versaoDaAtualizacao} />;
    }
    if (idDaSecaoAtiva === "agente") {
      return <TelaDoAgente />;
    }
    if (idDaSecaoAtiva === "grafo") {
      return (
        <Suspense fallback={<p className="suave">Carregando o grafo…</p>}>
          <TelaDoGrafoSobDemanda key={versaoDaAtualizacao} aoAbrirNota={abrirNota} />
        </Suspense>
      );
    }
    return <SecaoEmBreve secao={secaoAtiva} />;
  };

  return (
    <div className="app">
      <TrilhoDeSecoes idDaSecaoAtiva={idDaSecaoAtiva} aoEscolherSecao={setIdDaSecaoAtiva} />

      <ExploradorDeArquivos
        nomeDoCofre={nomeDoCofre}
        arvoreDoCofre={arvoreDoCofre}
        caminhoDaNotaAberta={caminhoDaNotaAberta}
        aoAbrirNota={abrirNota}
      />

      <div className="principal">
        {!estaLendoUmaNota && (
          <CabecalhoDaSecao
            tituloDaSecao={secaoAtiva.titulo}
            caminhoDoCofre={pastaDoCofre}
            aoAtualizar={atualizarTudo}
            aoTrocarCofre={cofreAberto.escolherPasta}
          />
        )}

        {cofreAberto.mensagemDeErro && <p role="alert" className="erro faixa">{cofreAberto.mensagemDeErro}</p>}

        <section className={classeDoConteudo}>
          {conteudoDaSecao()}
        </section>
      </div>
    </div>
  );
}
