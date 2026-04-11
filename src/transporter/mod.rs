use crate::toml::{Google};
use crate::logger::{info, ok};
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

        // Search for existing file with same name in the folder
        let existing_file_id = find_existing_file(&client, access_token, folder_id, &filename)?;

        match existing_file_id {
            Some(file_id) => {
                info(&format!("{filename} already exists, overriding..."));
                update_file(&client, access_token, &file_id, file_content, &filename)?;
            }
            None => {
                info(&format!("{filename} does not exist, creating..."));
                create_file(&client, access_token, folder_id, file_content, &filename)?;
            }
        }
    }

    Ok(String::from("Upload OK"))
}

fn find_existing_file(
    client: &Client,
    access_token: &str,
    folder_id: &str,
    filename: &str,
) -> Result<Option<String>, String> {
    let query = format!("name='{}' and '{}' in parents and trashed=false", filename, folder_id);

    let response = client
        .get("https://www.googleapis.com/drive/v3/files")
        .bearer_auth(access_token)
        .query(&[
            ("q", query.as_str()),
            ("fields", "files(id, name)"),
            ("supportsAllDrives", "true"),
            ("includeItemsFromAllDrives", "true"),
        ])
        .send()
        .map_err(|e| format!("Search request failed: {e}"))?;

    let json: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed to parse search response: {e}"))?;

    let file_id = json["files"]
        .as_array()
        .and_then(|files| files.first())
        .and_then(|file| file["id"].as_str())
        .map(|s| s.to_string());

    Ok(file_id)
}

fn create_file(
    client: &Client,
    access_token: &str,
    folder_id: &str,
    file_content: Vec<u8>,
    filename: &str,
) -> Result<(), String> {
    let metadata = serde_json::json!({
        "name": filename,
        "parents": [folder_id]
    });

    let form = build_multipart_form(metadata.to_string(), file_content, filename)?;

    let response = client
        .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart&supportsAllDrives=true")
        .bearer_auth(access_token)
        .multipart(form)
        .send()
        .map_err(|e| format!("Create request failed for {filename}: {e}"))?;

    handle_response(response, filename)
}

fn update_file(
    client: &Client,
    access_token: &str,
    file_id: &str,
    file_content: Vec<u8>,
    filename: &str,
) -> Result<(), String> {
    // Note: no "parents" in metadata for updates
    let metadata = serde_json::json!({
        "name": filename,
    });

    let form = build_multipart_form(metadata.to_string(), file_content, filename)?;

    let url = format!(
        "https://www.googleapis.com/upload/drive/v3/files/{}?uploadType=multipart&supportsAllDrives=true",
        file_id
    );

    let response = client
        .patch(&url)
        .bearer_auth(access_token)
        .multipart(form)
        .send()
        .map_err(|e| format!("Update request failed for {filename}: {e}"))?;

    handle_response(response, filename)
}

fn build_multipart_form(
    metadata: String,
    file_content: Vec<u8>,
    filename: &str,
) -> Result<multipart::Form, String> {
    let form = multipart::Form::new()
        .part(
            "metadata",
            multipart::Part::text(metadata)
                .mime_str("application/json")
                .map_err(|e| format!("Invalid mime type: {e}"))?,
        )
        .part(
            "file",
            multipart::Part::bytes(file_content)
                .file_name(filename.to_string())
                .mime_str("application/octet-stream")
                .map_err(|e| format!("Invalid mime type: {e}"))?,
        );

    Ok(form)
}

fn handle_response(response: reqwest::blocking::Response, filename: &str) -> Result<(), String> {
    if response.status().is_success() {
        ok(&format!("{filename} uploaded successfully."));
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        Err(format!("Upload failed for {filename} with status {status}: {body}"))
    }
}