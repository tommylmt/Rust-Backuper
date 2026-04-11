use std::fs;
use crate::toml::Files;
use std::path::Path;
use std::process::Command;
use crate::logger::{info, error, ok};
use crate::saver::DEST_FOLDER;

pub fn dump_files(files_config: &Files) -> bool {
    let dest_dir = Path::new(DEST_FOLDER);
    fs::create_dir_all(dest_dir).expect("Failed to create destination directory");

    let excluded: Vec<String> = files_config
        .exclude
        .as_ref()
        .map(|e| e.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let files_to_backup = collect_files(files_config);

    if files_to_backup.is_empty() {
        info("No files to backup, skipping.");
        return true;
    }

    if files_config.archive {
        create_archive(files_config, &files_to_backup)
    } else {
        copy_files(&files_to_backup, dest_dir, &excluded)
    }
}

fn collect_files(files_config: &Files) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    // Collect exclude list
    let excluded: Vec<String> = files_config
        .exclude
        .as_ref()
        .map(|e| e.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    // Add explicitly listed files
    if let Some(files) = &files_config.files {
        for file in files {
            if let Some(f) = file.as_str() {
                if !is_excluded(f, &excluded) {
                    result.push(f.to_string());
                }
            }
        }
    }

    // Walk the path directory if provided
    if let Some(path) = &files_config.path {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path().to_string_lossy().to_string();

                if is_excluded(&entry_path, &excluded) {
                    info(&format!("Excluding {entry_path}"));
                    continue;
                }

                result.push(entry_path);
            }
        }
    }

    result
}

fn create_archive(files_config: &Files, files: &[String]) -> bool {
    let archive_path = format!("{}/files_backup.tar.gz", DEST_FOLDER);

    info(&format!("Creating archive at {archive_path}..."));

    // Build tar command
    let mut cmd = Command::new("tar");
    cmd.arg("-czf").arg(&archive_path);

    // Add all files/dirs to archive
    for file in files {
        cmd.arg(file);
    }

    let status = cmd.status().expect("Failed to execute tar");

    if !status.success() {
        error(&"Failed to create archive");
        return false;
    }

    // Protect with password using openssl if archive_password is set
    if !files_config.archive_password.is_empty() {
        info("Encrypting archive with password...");

        let encrypted_path = format!("{}/files_backup.tar.gz.enc", DEST_FOLDER);

        let status = Command::new("openssl")
            .args([
                "enc",
                "-aes-256-cbc",
                "-pbkdf2",
                "-in", &archive_path,
                "-out", &encrypted_path,
                "-k", &files_config.archive_password,
            ])
            .status()
            .expect("Failed to execute openssl");

        if !status.success() {
            error("Failed to encrypt archive");
            return false;
        }

        // Remove unencrypted archive
        fs::remove_file(&archive_path).expect("Failed to remove unencrypted archive");
        info(&format!("Archive encrypted at {encrypted_path}"));
    }

    ok(&format!("Archive created at {archive_path}"));
    
    true
}

fn copy_files(files: &[String], dest_dir: &Path, excluded: &[String]) -> bool {
    for file in files {
        let src = Path::new(file);

        if !src.exists() {
            error(&format!("File not found: {file}"));
            return false;
        }

        let filename = src.file_name().unwrap().to_string_lossy();
        let dest = dest_dir.join(filename.as_ref());

        if src.is_dir() {
            copy_dir_recursive(src, &dest, excluded);
        } else {
            fs::copy(src, &dest)
                .map_err(|e| error(&format!("Failed to copy {file}: {e}")))
                .ok();
        }

        info(&format!("Copied {file}"));
    }

    true
}

fn copy_dir_recursive(src: &Path, dest: &Path, excluded: &[String]) {
    fs::create_dir_all(dest).expect("Failed to create directory");

    for entry in fs::read_dir(src).expect("Failed to read dir").flatten() {
        let src_path = entry.path();
        let src_str = src_path.to_string_lossy().to_string();

        // Check exclusion at every level of recursion
        if is_excluded(&src_str, excluded) {
            info(&format!("Excluding {src_str}"));
            continue;
        }

        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path, excluded);
        } else {
            fs::copy(&src_path, &dest_path)
                .map_err(|e| error(&format!("Failed to copy {}: {e}", src_path.display())))
                .ok();
        }
    }
}

fn is_excluded(path: &str, excluded: &[String]) -> bool {
    let path = Path::new(path).canonicalize().unwrap_or_else(|_| Path::new(path).to_path_buf());

    excluded.iter().any(|e| {
        let excluded_path = Path::new(e).canonicalize().unwrap_or_else(|_| Path::new(e).to_path_buf());
        path.starts_with(&excluded_path)
    })
}