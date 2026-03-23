//! OAuth 2.0 PKCE flow for X API endpoints that require it (bookmarks, etc.)
//!
//! Flow:
//! 1. Generate PKCE code_verifier + code_challenge
//! 2. Open browser to X authorization URL
//! 3. Listen on localhost:3000/callback for the redirect
//! 4. Exchange authorization code for access token
//! 5. Save tokens to config

use crate::config;
use crate::errors::XmasterError;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const AUTH_URL: &str = "https://x.com/i/oauth2/authorize";
const TOKEN_URL: &str = "https://api.x.com/2/oauth2/token";
const REDIRECT_URI: &str = "http://localhost:3000/callback";
const SCOPES: &str = "tweet.read tweet.write users.read bookmark.read bookmark.write offline.access";

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuth2Tokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub scope: Option<String>,
}

/// Generate a random PKCE code verifier (43-128 chars, URL-safe)
fn generate_code_verifier() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    base64_url_encode(&bytes)
}

/// Generate code challenge from verifier (SHA256 + base64url)
fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    base64_url_encode(&hash)
}

fn base64_url_encode(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// Build the authorization URL for the browser
pub fn build_auth_url(client_id: &str) -> (String, String) {
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);
    let state = generate_code_verifier(); // random state for CSRF protection

    let url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        AUTH_URL,
        urlencoding::encode(client_id),
        urlencoding::encode(REDIRECT_URI),
        urlencoding::encode(SCOPES),
        urlencoding::encode(&state),
        urlencoding::encode(&code_challenge),
    );

    (url, code_verifier)
}

/// Exchange authorization code for tokens
pub async fn exchange_code(
    client_id: &str,
    client_secret: &str,
    code: &str,
    code_verifier: &str,
) -> Result<OAuth2Tokens, XmasterError> {
    let client = reqwest::Client::new();

    let params = [
        ("code", code),
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("redirect_uri", REDIRECT_URI),
        ("code_verifier", code_verifier),
    ];

    let resp = client
        .post(TOKEN_URL)
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await?;

    if !resp.status().is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(XmasterError::AuthMissing {
            provider: "x-oauth2",
            message: format!("Token exchange failed: {text}"),
        });
    }

    let tokens: OAuth2Tokens = resp.json().await?;
    Ok(tokens)
}

/// Refresh an expired access token
pub async fn refresh_token(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<OAuth2Tokens, XmasterError> {
    let client = reqwest::Client::new();

    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", client_id),
    ];

    let resp = client
        .post(TOKEN_URL)
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await?;

    if !resp.status().is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(XmasterError::AuthMissing {
            provider: "x-oauth2",
            message: format!("Token refresh failed: {text}. Run: xmaster config auth"),
        });
    }

    let tokens: OAuth2Tokens = resp.json().await?;
    Ok(tokens)
}

/// Save OAuth 2.0 tokens to config
pub fn save_tokens(tokens: &OAuth2Tokens) -> Result<(), XmasterError> {
    let path = config::config_path();
    let existing = if path.exists() {
        std::fs::read_to_string(&path).unwrap_or_default()
    } else {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        String::new()
    };

    let mut doc: toml::Table = existing
        .parse()
        .map_err(|e: toml::de::Error| XmasterError::Config(format!("Parse error: {e}")))?;

    let keys = doc
        .entry("keys".to_string())
        .or_insert_with(|| toml::Value::Table(toml::Table::new()));

    if let toml::Value::Table(ref mut t) = keys {
        t.insert(
            "oauth2_access_token".to_string(),
            toml::Value::String(tokens.access_token.clone()),
        );
        if let Some(ref rt) = tokens.refresh_token {
            t.insert(
                "oauth2_refresh_token".to_string(),
                toml::Value::String(rt.clone()),
            );
        }
    }

    let toml_str = toml::to_string_pretty(&doc)
        .map_err(|e| XmasterError::Config(format!("Serialize error: {e}")))?;
    std::fs::write(&path, toml_str)?;

    // Secure permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}

/// Make an authenticated GET request using OAuth 2.0 bearer token
pub async fn oauth2_get(
    url: &str,
    access_token: &str,
) -> Result<serde_json::Value, XmasterError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await?;

    if resp.status().as_u16() == 401 {
        return Err(XmasterError::AuthMissing {
            provider: "x-oauth2",
            message: "OAuth 2.0 token expired. Run: xmaster config auth".into(),
        });
    }

    if !resp.status().is_success() {
        let status = resp.status().as_u16();
        let text = resp.text().await.unwrap_or_default();
        return Err(XmasterError::Api {
            provider: "x",
            code: "api_error",
            message: format!("HTTP {status}: {text}"),
        });
    }

    let json: serde_json::Value = resp.json().await?;
    Ok(json)
}
