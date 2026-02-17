use app_tauri::commands::auth::{exchange_token, start_oauth, OAuthProvider, TokenResponse};

struct FakeProvider;

impl OAuthProvider for FakeProvider {
    fn exchange_code(&self, code: &str, _code_verifier: &str) -> anyhow::Result<TokenResponse> {
        if code == "valid-code" {
            Ok(TokenResponse {
                access_token: "access".to_string(),
                refresh_token: "refresh".to_string(),
                expires_in: 3600,
            })
        } else {
            Err(anyhow::anyhow!("invalid_code"))
        }
    }
}

#[test]
fn start_oauth_generates_pkce_verifier_with_valid_length() {
    let start = start_oauth("client-id", "http://localhost/callback", &["calendar"])
        .expect("start oauth should succeed");

    assert!((43..=128).contains(&start.code_verifier.len()));
}

#[test]
fn start_oauth_generates_sha256_base64url_challenge() {
    let start = start_oauth("client-id", "http://localhost/callback", &["calendar"])
        .expect("start oauth should succeed");

    assert!(!start.code_challenge.contains('+'));
    assert!(!start.code_challenge.contains('/'));
    assert!(!start.code_challenge.contains('='));
}

#[test]
fn exchange_token_succeeds_with_valid_code() {
    let provider = FakeProvider;

    let token =
        exchange_token(&provider, "valid-code", "verifier").expect("exchange should succeed");

    assert_eq!(token.access_token, "access");
}

#[test]
fn exchange_token_fails_with_invalid_code() {
    let provider = FakeProvider;

    let result = exchange_token(&provider, "invalid", "verifier");

    assert!(result.is_err());
}
