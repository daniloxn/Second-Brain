//! Cifra o conteúdo inteiro do cofre com uma chave derivada da senha mestra.
//! O envelope expõe só versão, salt, nonce e texto cifrado.

use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use argon2::Argon2;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

const VERSAO_DO_ENVELOPE: u8 = 1;
const TAMANHO_DO_SALT: usize = 16;
const TAMANHO_DO_NONCE: usize = 12;
const TAMANHO_DA_CHAVE: usize = 32;

/// Campos públicos do arquivo cifrado; o conteúdo da credencial fica no campo `cifrado`.
#[derive(Serialize, Deserialize)]
struct EnvelopeCriptografado {
    versao: u8,
    salt: String,
    nonce: String,
    cifrado: String,
}

/// Cifra um conteúdo usando Argon2id e AES-256-GCM com salt e nonce novos.
pub(super) fn cifrar(conteudo: &[u8], senha_mestra: &str) -> Result<Vec<u8>, String> {
    let mut salt = [0; TAMANHO_DO_SALT];
    let mut nonce = [0; TAMANHO_DO_NONCE];
    getrandom::fill(&mut salt).map_err(|erro| format!("não foi possível gerar o salt: {erro}"))?;
    getrandom::fill(&mut nonce).map_err(|erro| format!("não foi possível gerar o nonce: {erro}"))?;

    let chave = derivar_chave(senha_mestra, &salt)?;
    let cifrador = Aes256Gcm::new_from_slice(chave.as_ref())
        .map_err(|_| "não foi possível preparar a cifra".to_string())?;
    let nonce = Nonce::try_from(nonce.as_slice())
        .map_err(|_| "não foi possível preparar o nonce".to_string())?;
    let texto_cifrado = cifrador
        .encrypt(&nonce, conteudo)
        .map_err(|_| "não foi possível cifrar o cofre".to_string())?;
    let envelope = EnvelopeCriptografado {
        versao: VERSAO_DO_ENVELOPE,
        salt: codificar_hex(&salt),
        nonce: codificar_hex(&nonce),
        cifrado: codificar_hex(&texto_cifrado),
    };
    serde_json::to_vec(&envelope).map_err(|erro| erro.to_string())
}

/// Abre um envelope depois de validar a versão e autenticar a senha mestra.
pub(super) fn decifrar(arquivo: &[u8], senha_mestra: &str) -> Result<Zeroizing<Vec<u8>>, String> {
    let envelope: EnvelopeCriptografado = serde_json::from_slice(arquivo)
        .map_err(|_| "arquivo do cofre de segredos inválido".to_string())?;
    if envelope.versao != VERSAO_DO_ENVELOPE {
        return Err(format!("versão do cofre de segredos não suportada: {}", envelope.versao));
    }

    let salt = decodificar_hex(&envelope.salt)?;
    let nonce = decodificar_hex(&envelope.nonce)?;
    let texto_cifrado = decodificar_hex(&envelope.cifrado)?;
    if salt.len() != TAMANHO_DO_SALT || nonce.len() != TAMANHO_DO_NONCE {
        return Err("parâmetros do cofre de segredos inválidos".into());
    }

    let chave = derivar_chave(senha_mestra, &salt)?;
    let cifrador = Aes256Gcm::new_from_slice(chave.as_ref())
        .map_err(|_| "não foi possível preparar a cifra".to_string())?;
    let nonce = Nonce::try_from(nonce.as_slice())
        .map_err(|_| "parâmetros do cofre de segredos inválidos".to_string())?;
    let conteudo = cifrador
        .decrypt(&nonce, texto_cifrado.as_ref())
        .map_err(|_| "senha mestra incorreta ou arquivo de segredos danificado".to_string())?;
    Ok(Zeroizing::new(conteudo))
}

/// Deriva 256 bits com Argon2id, usando os parâmetros padrão da biblioteca.
fn derivar_chave(senha_mestra: &str, salt: &[u8]) -> Result<Zeroizing<[u8; TAMANHO_DA_CHAVE]>, String> {
    let mut chave = Zeroizing::new([0; TAMANHO_DA_CHAVE]);
    Argon2::default()
        .hash_password_into(senha_mestra.as_bytes(), salt, chave.as_mut())
        .map_err(|erro| format!("não foi possível derivar a chave do cofre: {erro}"))?;
    Ok(chave)
}

/// Converte bytes para hexadecimal, sem incluir segredos no nome do arquivo.
fn codificar_hex(bytes: &[u8]) -> String {
    const DIGITOS: &[u8; 16] = b"0123456789abcdef";
    let mut texto = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        texto.push(DIGITOS[(byte >> 4) as usize] as char);
        texto.push(DIGITOS[(byte & 0x0f) as usize] as char);
    }
    texto
}

/// Lê hexadecimal estrito para rejeitar envelopes incompletos ou adulterados.
fn decodificar_hex(texto: &str) -> Result<Vec<u8>, String> {
    if texto.len() % 2 != 0 {
        return Err("arquivo do cofre de segredos inválido".into());
    }
    texto
        .as_bytes()
        .chunks_exact(2)
        .map(|par| {
            let alto = (par[0] as char).to_digit(16);
            let baixo = (par[1] as char).to_digit(16);
            match (alto, baixo) {
                (Some(alto), Some(baixo)) => Ok(((alto << 4) | baixo) as u8),
                _ => Err("arquivo do cofre de segredos inválido".into()),
            }
        })
        .collect()
}
