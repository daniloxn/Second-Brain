// Estado do cofre aberto: pasta, árvore, índice de notas, erros e a pasta que espera confirmação.
// Reabre o último cofre quando o app inicia, e só usa pastas que são cofres do app.

import { useCallback, useEffect, useRef, useState } from "react";
import {
  abrirCofre,
  cofreAtual,
  criarCofre,
  criarPastaNoCofre,
  escolherPastaDoCofre,
  listarArvore,
  type NoArvore,
} from "./chamadasCofre";
import { listarNotas, type NotaIndice } from "../notas/chamadasNotas";

interface OpcoesDoCofre {
  /** Chamado sempre que um cofre é aberto ou criado, para o app fechar o que estava aberto. */
  aoAbrirCofre?: () => void;
}

export function useCofreAberto({ aoAbrirCofre }: OpcoesDoCofre = {}) {
  const [pastaDoCofre, setPastaDoCofre] = useState<string | null>(null);
  const [arvoreDoCofre, setArvoreDoCofre] = useState<NoArvore | null>(null);
  const [indiceDeNotas, setIndiceDeNotas] = useState<NotaIndice[]>([]);
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  const [carregandoInicio, setCarregandoInicio] = useState(true);
  // Pasta comum que o usuário escolheu e que ainda precisa de confirmação para virar cofre.
  const [pastaAguardandoConfirmacao, setPastaAguardandoConfirmacao] = useState<string | null>(null);

  // Guarda o callback numa ref para que as funções abaixo continuem estáveis.
  const aoAbrirCofreRef = useRef(aoAbrirCofre);
  aoAbrirCofreRef.current = aoAbrirCofre;

  /** Carrega árvore e notas de um cofre já validado pelo backend e o torna o cofre atual. */
  const carregarCofreAberto = useCallback(async (caminhoDoCofre: string) => {
    const [arvore, notas] = await Promise.all([listarArvore(caminhoDoCofre), listarNotas()]);
    setPastaDoCofre(caminhoDoCofre);
    setArvoreDoCofre(arvore);
    setIndiceDeNotas(notas);
    setPastaAguardandoConfirmacao(null);
    aoAbrirCofreRef.current?.();
  }, []);

  /**
   * Tenta abrir uma pasta. Se ela não for um cofre do app e `converterPasta` for falso,
   * não abre: guarda a pasta para o usuário confirmar.
   */
  const abrirPasta = useCallback(
    async (caminhoEscolhido: string, converterPasta: boolean) => {
      setMensagemDeErro(null);
      try {
        const resposta = await abrirCofre(caminhoEscolhido, converterPasta);
        if (resposta.precisaConfirmar || !resposta.caminho) {
          setPastaAguardandoConfirmacao(caminhoEscolhido);
          return;
        }
        await carregarCofreAberto(resposta.caminho);
      } catch (erro) {
        setMensagemDeErro(String(erro));
      }
    },
    [carregarCofreAberto],
  );

  /** Escolhe uma pasta existente pela janela do sistema e tenta abri-la. */
  const escolherPasta = useCallback(async () => {
    const pastaEscolhida = await escolherPastaDoCofre("Escolha a pasta do cofre");
    if (pastaEscolhida) await abrirPasta(pastaEscolhida, false);
  }, [abrirPasta]);

  /** Usuário confirmou que a pasta pendente vira cofre: o app cria só o que falta nela. */
  const confirmarConversaoDaPasta = useCallback(async () => {
    if (pastaAguardandoConfirmacao) await abrirPasta(pastaAguardandoConfirmacao, true);
  }, [abrirPasta, pastaAguardandoConfirmacao]);

  /** Usuário desistiu da pasta pendente. Nada é alterado. */
  const cancelarConversaoDaPasta = useCallback(() => {
    setPastaAguardandoConfirmacao(null);
  }, []);

  /** Cria um cofre novo e o abre. Lança erro para o formulário mostrar a mensagem. */
  const criarCofreNovo = useCallback(
    async (pastaPai: string, nomeDoCofre: string) => {
      const caminhoDoNovoCofre = await criarCofre(pastaPai, nomeDoCofre);
      await carregarCofreAberto(caminhoDoNovoCofre);
    },
    [carregarCofreAberto],
  );

  /** Cria uma pasta dentro do cofre atual e recarrega a árvore. Lança erro para quem chamou mostrar. */
  const criarPastaNoCofreAtual = useCallback(
    async (caminhoRelativo: string) => {
      if (!pastaDoCofre) return;
      await criarPastaNoCofre(caminhoRelativo);
      setArvoreDoCofre(await listarArvore(pastaDoCofre));
    },
    [pastaDoCofre],
  );

  /** Recarrega só a árvore e o índice de notas, sem trocar de cofre. */
  const atualizarIndices = useCallback(async () => {
    if (!pastaDoCofre) return;
    const [arvore, notas] = await Promise.all([listarArvore(pastaDoCofre), listarNotas()]);
    setArvoreDoCofre(arvore);
    setIndiceDeNotas(notas);
  }, [pastaDoCofre]);

  // Na inicialização, reabre o último cofre salvo, se ele ainda for um cofre do app.
  useEffect(() => {
    cofreAtual()
      .then((ultimoCofre) => (ultimoCofre ? carregarCofreAberto(ultimoCofre) : undefined))
      .catch((erro) => setMensagemDeErro(String(erro)))
      .finally(() => setCarregandoInicio(false));
  }, [carregarCofreAberto]);

  return {
    pastaDoCofre,
    arvoreDoCofre,
    indiceDeNotas,
    mensagemDeErro,
    carregandoInicio,
    pastaAguardandoConfirmacao,
    escolherPasta,
    confirmarConversaoDaPasta,
    cancelarConversaoDaPasta,
    criarCofreNovo,
    criarPastaNoCofreAtual,
    atualizarIndices,
    informarErro: setMensagemDeErro,
  };
}
