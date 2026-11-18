// Entrada do frontend: monta o React na página e carrega a fonte e os estilos.
// Os estilos ficam em src/estilos/, um arquivo por funcionalidade.

import React from "react";
import ReactDOM from "react-dom/client";
import "@fontsource-variable/inter";
import "./estilos/base.css";
import "./estilos/botoes.css";
import "./estilos/layout.css";
import "./estilos/arquivos.css";
import "./estilos/cofre.css";
import "./estilos/notas-lista.css";
import "./estilos/notas-editor.css";
import "./estilos/notas-leitura.css";
import "./estilos/notas-backlinks.css";
import "./estilos/tarefas.css";
import "./estilos/agente.css";
import "./estilos/grafo.css";
import "./estilos/grafo-painel.css";
import "./estilos/notas-conflito.css";
import "./estilos/cofre-inicio.css";
import "./estilos/responsivo.css";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
