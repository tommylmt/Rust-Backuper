use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::toml::{Google};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct JwtClaims {
    iss: String,   // service account email
    scope: String, // Google API scopes
    aud: String,   // token URL
    exp: u64,      // expiry
    iat: u64,      // issued at
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

pub fn get_google_access_token(google: &Google) -> Result<String, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs();

    let claims = JwtClaims {
        iss: google.email.clone(),
        scope: "https://www.googleapis.com/auth/drive".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: now + 3600,
        iat: now,
    };

    // Google requires RSA256 and the private key in PEM format
    let encoding_key = EncodingKey::from_rsa_pem(google.private_key.as_bytes())
        .map_err(|e| format!("Invalid private key: {e}"))?;

    let header = Header::new(Algorithm::RS256);

    let jwt = encode(&header, &claims, &encoding_key)
        .map_err(|e| format!("Failed to encode JWT: {e}"))?;

    let body = format!(
        "grant_type={}&assertion={}",
        urlencoding::encode("urn:ietf:params:oauth:grant-type:jwt-bearer"),
        urlencoding::encode(&jwt)
    );

    // Exchange JWT for an OAuth2 access token
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .map_err(|e| format!("Token request failed: {e}"))?;

    let token: TokenResponse = response
        .json()
        .map_err(|e| format!("Failed to parse token response: {e}"))?;

    Ok(token.access_token)
}