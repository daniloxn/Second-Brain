// Regras de apresentação do resumo semanal. Funções puras, sem componente nem backend.

/**
 * Tempo em texto legível. Até um minuto mostra segundos; a partir daí, minutos e segundos.
 * Ex.: 45000 vira "45 s", 200000 vira "3 min 20 s".
 */
export function formatarTempoDoAgente(milissegundos: number): string {
  const segundosTotais = Math.round(milissegundos / 1000);
  if (segundosTotais < 60) return `${segundosTotais} s`;
  const minutos = Math.floor(segundosTotais / 60);
  const segundos = segundosTotais % 60;
  return segundos === 0 ? `${minutos} min` : `${minutos} min ${segundos} s`;
}

/** Texto do período, como "13/11 a 19/11", a partir das datas `AAAA-MM-DD` que o backend envia. */
export function periodoDaSemana(inicio: string, fim: string): string {
  const formatar = (data: string) => data.slice(8, 10) + "/" + data.slice(5, 7);
  return `${formatar(inicio)} a ${formatar(fim)}`;
}
