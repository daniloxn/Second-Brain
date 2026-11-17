// Trilho vertical de ícones à esquerda: troca a seção ativa ao clicar.

import { SECOES_DO_MENU, type IdDaSecao } from "./secoes";

interface Props {
  idDaSecaoAtiva: IdDaSecao;
  aoEscolherSecao: (idEscolhido: IdDaSecao) => void;
}

export function TrilhoDeSecoes({ idDaSecaoAtiva, aoEscolherSecao }: Props) {
  return (
    <nav className="trilho" aria-label="Seções">
      {SECOES_DO_MENU.map((secao) => {
        const estaAtiva = secao.id === idDaSecaoAtiva;
        return (
          <button
            key={secao.id}
            type="button"
            className="trilho-item"
            aria-label={secao.titulo}
            aria-current={estaAtiva ? "page" : undefined}
            title={secao.titulo}
            onClick={() => aoEscolherSecao(secao.id)}
          >
            <secao.icone size={20} aria-hidden="true" />
          </button>
        );
      })}
    </nav>
  );
}
