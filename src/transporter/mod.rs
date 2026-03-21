use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::toml::{Google};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::logger::{info};
use crate::saver::DEST_FOLDER;
use std::path::Path;
use std::fs;
use reqwest::blocking::{Client, multipart};

#[derive(Serialize)]
struct JwtClaims {
    iss: String,   // service account email
    scope: String, // Google API scopes
    aud: String,   // token URL
    exp: u64,      // expiry
    iat: u64,      // issued at
    sub: String,   // impersonation email
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
        sub: google.personal_email.clone(),
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

pub fn upload_to_drive(access_token: &str, folder_id: &str) -> Result<String, String> {
    let backup_dir = Path::new(DEST_FOLDER);
    let client = Client::new();

    let entries = fs::read_dir(backup_dir)
        .map_err(|e| format!("Failed to read backup directory: {e}"))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid filename")?
            .to_string();

        info(&format!("Uploading {filename} to Google Drive..."));

        let file_content = fs::read(&path)
            .map_err(|e| format!("Failed to read file {filename}: {e}"))?;

        // Metadata part: specify filename and parent folder
        let metadata = serde_json::json!({
            "name": filename,
            "parents": [folder_id]
        });

        let form = multipart::Form::new()
            .part(
                "metadata",
                multipart::Part::text(metadata.to_string())
                    .mime_str("application/json")
                    .map_err(|e| format!("Invalid mime type: {e}"))?,
            )
            .part(
                "file",
                multipart::Part::bytes(file_content)
                    .file_name(filename.clone())
                    .mime_str("application/octet-stream")
                    .map_err(|e| format!("Invalid mime type: {e}"))?,
            );

        let response = client
            .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart&supportsAllDrives=true")
            .bearer_auth(access_token)
            .multipart(form)
            .send()
            .map_err(|e| format!("Upload request failed for {filename}: {e}"))?;

        if response.status().is_success() {
            info(&format!("{filename} uploaded successfully."));
        } else {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(format!("Upload failed for {filename} with status {status}: {body}"));
        }
    }

    Ok(String::from("Upload OK"))
}