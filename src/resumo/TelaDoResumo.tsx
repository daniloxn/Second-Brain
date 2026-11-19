// Resumo da última semana: quanto o agente trabalhou e o que ele mudou, mais as tarefas concluídas.
// Os números vêm prontos do backend. O tempo é do agente, não do usuário.

import { useEffect, useState } from "react";
import { resumoDaSemana } from "./chamadasResumo";
import type { ResumoDaSemana } from "./contratoResumo";
import { formatarTempoDoAgente, periodoDaSemana } from "./regrasDoResumo";

export function TelaDoResumo() {
  const [resumo, setResumo] = useState<ResumoDaSemana | null>(null);
  const [erro, setErro] = useState<string | null>(null);

  useEffect(() => {
    resumoDaSemana().then(setResumo).catch((falha) => setErro(String(falha)));
  }, []);

  if (erro) return <p role="alert" className="erro">{erro}</p>;
  if (!resumo) return <p className="suave">Calculando a semana…</p>;

  const indicadores = [
    { rotulo: "Pedidos ao agente", valor: String(resumo.pedidosDoAgente) },
    { rotulo: "Tempo do agente", valor: formatarTempoDoAgente(resumo.tempoDoAgenteEmMs) },
    { rotulo: "Linhas alteradas pelo agente", valor: String(resumo.linhasAlteradasPeloAgente) },
    { rotulo: "Notas criadas pelo agente", valor: String(resumo.notasCriadasPeloAgente) },
    { rotulo: "Notas alteradas pelo agente", valor: String(resumo.notasAlteradasPeloAgente) },
    { rotulo: "Tarefas criadas pelo agente", valor: String(resumo.tarefasCriadasPeloAgente) },
    { rotulo: "Tarefas concluídas", valor: String(resumo.tarefasConcluidas) },
  ];

  return (
    <section className="resumo-semanal" aria-label="Resumo da semana">
      <h2>Últimos sete dias</h2>
      <p className="suave">{periodoDaSemana(resumo.inicio, resumo.fim)}</p>
      <ul className="resumo-semanal-indicadores">
        {indicadores.map((indicador) => (
          <li key={indicador.rotulo}>
            <span className="resumo-semanal-valor">{indicador.valor}</span>
            <span className="suave">{indicador.rotulo}</span>
          </li>
        ))}
      </ul>
      <p className="suave">
        O tempo soma só as rodadas do agente com medição; pedidos anteriores à medição ficam de fora
        ({resumo.pedidosComTempoMedido} de {resumo.pedidosDoAgente} com tempo). Isso mede o trabalho do agente, não o seu.
        Tarefas concluídas antes desta versão não têm data e não entram.
      </p>
    </section>
  );
}
