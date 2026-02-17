use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OAuthStart {
    pub auth_url: String,
    pub code_verifier: String,
    pub code_challenge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub trait OAuthProvider {
    fn exchange_code(&self, code: &str, code_verifier: &str) -> anyhow::Result<TokenResponse>;
}

pub fn start_oauth(
    client_id: &str,
    redirect_uri: &str,
    scopes: &[&str],
) -> anyhow::Result<OAuthStart> {
    if client_id.trim().is_empty() {
        return Err(anyhow::anyhow!("client_id must not be empty"));
    }
    if redirect_uri.trim().is_empty() {
        return Err(anyhow::anyhow!("redirect_uri must not be empty"));
    }

    let code_verifier = generate_code_verifier();
    let code_challenge = code_challenge(&code_verifier);
    let scope = scopes.join(" ");

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={client_id}&redirect_uri={redirect_uri}&scope={scope}&code_challenge={code_challenge}&code_challenge_method=S256&access_type=offline&prompt=consent"
    );

    Ok(OAuthStart {
        auth_url,
        code_verifier,
        code_challenge,
    })
}

pub fn exchange_token(
    provider: &dyn OAuthProvider,
    code: &str,
    code_verifier: &str,
) -> anyhow::Result<TokenResponse> {
    if code.trim().is_empty() {
        return Err(anyhow::anyhow!("authorization code must not be empty"));
    }
    if code_verifier.trim().is_empty() {
        return Err(anyhow::anyhow!("code verifier must not be empty"));
    }
    provider.exchange_code(code, code_verifier)
}

fn generate_code_verifier() -> String {
    // UUID v4を連結してPKCE要件(43-128 chars)を満たす。
    let raw = format!(
        "{}{}{}",
        Uuid::new_v4().simple(),
        Uuid::new_v4().simple(),
        Uuid::new_v4().simple()
    );
    raw.chars().take(96).collect()
}

fn code_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}
