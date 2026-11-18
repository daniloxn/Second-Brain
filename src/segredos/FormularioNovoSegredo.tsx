// Formulário de credencial. Mantém o texto da senha só no estado temporário do componente.

import { useState, type FormEvent } from "react";
import type { NovoSegredo } from "./contratoSegredos";

interface Props {
  aoSalvar: (entrada: NovoSegredo) => Promise<void>;
  aoCancelar: () => void;
}

/** Coleta a credencial e limpa os campos depois que ela for gravada. */
export function FormularioNovoSegredo({ aoSalvar, aoCancelar }: Props) {
  const [titulo, setTitulo] = useState("");
  const [usuario, setUsuario] = useState("");
  const [senha, setSenha] = useState("");
  const [anotacoes, setAnotacoes] = useState("");
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  const [estaSalvando, setEstaSalvando] = useState(false);

  /** Envia os campos uma vez e deixa o erro visível sem descartar o que foi digitado. */
  async function enviarFormulario(evento: FormEvent) {
    evento.preventDefault();
    if (estaSalvando) return;
    setEstaSalvando(true);
    setMensagemDeErro(null);
    try {
      await aoSalvar({ titulo, usuario, senha, anotacoes });
      setTitulo("");
      setUsuario("");
      setSenha("");
      setAnotacoes("");
    } catch (erro) {
      setMensagemDeErro(String(erro));
    } finally {
      setEstaSalvando(false);
    }
  }

  return (
    <form className="form-segredo" onSubmit={enviarFormulario}>
      <h3>Novo segredo</h3>
      <label className="campo-rotulado">
        <span>Nome</span>
        <input className="campo" autoFocus required maxLength={160} value={titulo} onChange={(evento) => setTitulo(evento.target.value)} placeholder="Ex.: E-mail pessoal" />
      </label>
      <label className="campo-rotulado">
        <span>Usuário ou e-mail</span>
        <input className="campo" autoComplete="off" maxLength={512} value={usuario} onChange={(evento) => setUsuario(evento.target.value)} />
      </label>
      <label className="campo-rotulado">
        <span>Senha</span>
        <input className="campo" type="password" autoComplete="new-password" maxLength={8192} required value={senha} onChange={(evento) => setSenha(evento.target.value)} />
      </label>
      <label className="campo-rotulado">
        <span>Anotações</span>
        <textarea className="campo" rows={3} maxLength={8192} value={anotacoes} onChange={(evento) => setAnotacoes(evento.target.value)} />
      </label>
      <div className="linha-acoes">
        <button type="submit" className="botao botao-primario" disabled={estaSalvando}>Salvar segredo</button>
        <button type="button" className="botao" onClick={aoCancelar}>Cancelar</button>
      </div>
      {mensagemDeErro && <p role="alert" className="erro">{mensagemDeErro}</p>}
    </form>
  );
}
