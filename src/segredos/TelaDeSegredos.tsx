// Interface do cofre de segredos. A lista exige desbloqueio e a senha nunca é persistida.

import { useEffect, useRef, useState, type FormEvent } from "react";
import { clear, readText, writeSecret } from "@tauri-apps/plugin-clipboard-manager";
import { apagarSegredo, copiarSenhaDoSegredo, criarCofreDeSegredos, criarSegredo, existeCofreDeSegredos, listarSegredos } from "./chamadasSegredos";
import type { NovoSegredo, ResumoDeSegredo } from "./contratoSegredos";
import { FormularioNovoSegredo } from "./FormularioNovoSegredo";
import { ItemDeSegredo } from "./ItemDeSegredo";

const TEMPO_DE_LIMPEZA_DA_AREA_DE_TRANSFERENCIA = 30_000;

/** Exibe o estado inicial, desbloqueado ou configurado do cofre criptografado. */
export function TelaDeSegredos() {
  const [estaCarregando, setEstaCarregando] = useState(true);
  const [cofreConfigurado, setCofreConfigurado] = useState(false);
  const [estaDesbloqueado, setEstaDesbloqueado] = useState(false);
  const [senhaMestra, setSenhaMestra] = useState("");
  const [senhaDigitada, setSenhaDigitada] = useState("");
  const [confirmacaoDaSenha, setConfirmacaoDaSenha] = useState("");
  const [segredos, setSegredos] = useState<ResumoDeSegredo[]>([]);
  const [estaCriandoSegredo, setEstaCriandoSegredo] = useState(false);
  const [mensagem, setMensagem] = useState<string | null>(null);
  const timerDaLimpeza = useRef<number | null>(null);
  const versaoDaCopia = useRef(0);

  useEffect(() => {
    existeCofreDeSegredos()
      .then(setCofreConfigurado)
      .catch((erro) => setMensagem(String(erro)))
      .finally(() => setEstaCarregando(false));
  }, []);

  /** Desbloqueia e carrega somente os campos que podem ser mostrados na lista. */
  async function desbloquear(evento: FormEvent) {
    evento.preventDefault();
    try {
      const segredosDesbloqueados = await listarSegredos(senhaDigitada);
      setSegredos(segredosDesbloqueados);
      setSenhaMestra(senhaDigitada);
      setSenhaDigitada("");
      setEstaDesbloqueado(true);
      setMensagem(null);
    } catch (erro) {
      setMensagem(String(erro));
    }
  }

  /** Cria o arquivo criptografado depois de confirmar a senha escolhida duas vezes. */
  async function criarCofre(evento: FormEvent) {
    evento.preventDefault();
    if (senhaDigitada !== confirmacaoDaSenha) {
      setMensagem("As senhas mestras não são iguais.");
      return;
    }
    if ([...senhaDigitada].length !== 6) {
      setMensagem("A senha mestra precisa ter exatamente 6 caracteres.");
      return;
    }
    try {
      await criarCofreDeSegredos(senhaDigitada);
      setCofreConfigurado(true);
      setSenhaMestra(senhaDigitada);
      setSenhaDigitada("");
      setConfirmacaoDaSenha("");
      setEstaDesbloqueado(true);
      setMensagem(null);
    } catch (erro) {
      setMensagem(String(erro));
    }
  }

  /** Cifra uma nova credencial e substitui a lista visível pelos resumos atualizados. */
  async function salvarNovoSegredo(entrada: NovoSegredo) {
    const resumos = await criarSegredo(senhaMestra, entrada);
    setSegredos(resumos);
    setEstaCriandoSegredo(false);
    setMensagem("Segredo salvo criptografado.");
  }

  /** Pede confirmação antes de buscar a senha e copiá-la como texto sensível. */
  async function confirmarCopia(segredo: ResumoDeSegredo) {
    if (!window.confirm(`Copiar a senha de “${segredo.titulo}” para a área de transferência?`)) return;
    try {
      const senhaCopiada = await copiarSenhaDoSegredo(segredo.id, senhaMestra);
      await writeSecret(senhaCopiada);
      versaoDaCopia.current += 1;
      const versaoDestaCopia = versaoDaCopia.current;
      if (timerDaLimpeza.current !== null) window.clearTimeout(timerDaLimpeza.current);
      timerDaLimpeza.current = window.setTimeout(async () => {
        try {
          if (versaoDaCopia.current === versaoDestaCopia && await readText() === senhaCopiada) await clear();
        } catch {
          // Se o sistema negar acesso, preserva o clipboard e não interrompe o app.
        }
      }, TEMPO_DE_LIMPEZA_DA_AREA_DE_TRANSFERENCIA);
      setMensagem("Senha copiada. O app tentará limpá-la em 30 segundos se ela ainda estiver na área de transferência.");
    } catch (erro) {
      setMensagem(String(erro));
    }
  }

  /** Confirma a exclusão e cifra novamente o cofre sem o registro escolhido. */
  async function confirmarExclusao(segredo: ResumoDeSegredo) {
    if (!window.confirm(`Apagar “${segredo.titulo}” do cofre de segredos?`)) return;
    try {
      setSegredos(await apagarSegredo(segredo.id, senhaMestra));
      setMensagem("Segredo apagado.");
    } catch (erro) {
      setMensagem(String(erro));
    }
  }

  /** Fecha a sessão da tela e remove os dados visíveis do estado do componente. */
  function bloquearCofre() {
    setSenhaMestra("");
    setSegredos([]);
    setEstaDesbloqueado(false);
    setEstaCriandoSegredo(false);
    setMensagem(null);
  }

  if (estaCarregando) return <p className="suave">Verificando cofre de segredos…</p>;

  return (
    <div className="tela-segredos">
      <header className="topo-segredos">
        <div>
          <h2 className="rotulo">Cofre de segredos</h2>
          <p className="suave">As credenciais ficam criptografadas e separadas dos arquivos Markdown.</p>
        </div>
        {estaDesbloqueado && <button type="button" className="botao" onClick={bloquearCofre}>Bloquear cofre</button>}
      </header>

      {!cofreConfigurado && (
        <form className="form-segredo" onSubmit={criarCofre}>
          <h3>Criar senha mestra</h3>
          <p className="suave">A senha mestra não é salva. Se esquecê-la, não será possível recuperar os segredos.</p>
          <label className="campo-rotulado"><span>Senha mestra (exatamente 6 caracteres)</span><input className="campo" type="password" autoComplete="new-password" maxLength={12} required value={senhaDigitada} onChange={(evento) => setSenhaDigitada([...evento.target.value].slice(0, 6).join(""))} /></label>
          <label className="campo-rotulado"><span>Confirme a senha mestra</span><input className="campo" type="password" autoComplete="new-password" maxLength={12} required value={confirmacaoDaSenha} onChange={(evento) => setConfirmacaoDaSenha([...evento.target.value].slice(0, 6).join(""))} /></label>
          <button type="submit" className="botao botao-primario">Criar cofre criptografado</button>
        </form>
      )}

      {cofreConfigurado && !estaDesbloqueado && (
        <form className="form-segredo" onSubmit={desbloquear}>
          <h3>Desbloquear</h3>
          <label className="campo-rotulado"><span>Senha mestra</span><input className="campo" type="password" autoComplete="current-password" required autoFocus value={senhaDigitada} onChange={(evento) => setSenhaDigitada(evento.target.value)} /></label>
          <button type="submit" className="botao botao-primario">Desbloquear cofre</button>
        </form>
      )}

      {estaDesbloqueado && (
        <>
          <div className="linha-topo">
            <h3 className="rotulo">{segredos.length === 1 ? "1 segredo" : `${segredos.length} segredos`}</h3>
            {!estaCriandoSegredo && <button type="button" className="botao botao-primario" onClick={() => setEstaCriandoSegredo(true)}>Novo segredo</button>}
          </div>
          {estaCriandoSegredo && <FormularioNovoSegredo aoSalvar={salvarNovoSegredo} aoCancelar={() => setEstaCriandoSegredo(false)} />}
          {segredos.length === 0 && !estaCriandoSegredo && <p className="suave">Ainda não há credenciais salvas neste cofre.</p>}
          <div className="lista-segredos">
            {segredos.map((segredo) => <ItemDeSegredo key={segredo.id} segredo={segredo} aoCopiar={confirmarCopia} aoApagar={confirmarExclusao} />)}
          </div>
        </>
      )}

      {mensagem && <p role="status" className={mensagem.includes("incorreta") || mensagem.includes("não") ? "erro" : "suave"}>{mensagem}</p>}
    </div>
  );
}
