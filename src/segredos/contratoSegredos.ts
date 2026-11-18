// Tipos públicos do cofre de segredos. Senhas só aparecem na resposta de uma ação de cópia.

/** Campos que podem ser listados após o desbloqueio; não inclui a senha. */
export interface ResumoDeSegredo {
  id: string;
  titulo: string;
  usuario: string;
  anotacoes: string;
}

/** Campos de uma credencial nova. */
export interface NovoSegredo {
  titulo: string;
  usuario: string;
  senha: string;
  anotacoes: string;
}
