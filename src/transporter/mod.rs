use crate::toml::{Google};
use crate::logger::{info};
use crate::saver::DEST_FOLDER;
use std::path::Path;
use std::fs;
use reqwest::blocking::{Client, multipart};



pub fn get_google_access_token(google: &Google) -> Result<String, String> {
    let client = Client::new();

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", google.client_id.as_str()),
            ("client_secret", google.client_secret.as_str()),
            ("refresh_token", google.refresh_token.as_str()),
            ("grant_type", "refresh_token"),
        ])
        .send()
        .map_err(|e| format!("Token request failed: {e}"))?;

    let json: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed to parse token response: {e}"))?;

    json["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("No access token in response: {json:#?}"))
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