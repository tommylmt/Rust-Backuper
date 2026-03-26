use crate::preflight::{config_exists, create_base_config, cron_exists, create_base_cronjob, CONFIG_PATH};
use crate::toml::{decode_file};
use crate::logger::{info, ok, critical, error};
use crate::saver::{do_save, ensure_dest_folder_is_clean};
use crate::transporter::{get_google_access_token, upload_to_drive};
use std::fs;
use std::env;

pub mod logger;
pub mod preflight;
pub mod toml;
pub mod saver;
pub mod transporter;

fn main() {
    // Version display
    if std::env::args().nth(1) == Some("--version".to_string()) {
        let version = env!("CARGO_PKG_VERSION");

        println!("Version {version}");
        std::process::exit(0)
    }

    let available_os = ["macos", "linux"];

    if !available_os.contains(&env::consts::OS) {
        critical(&"This program can only run on Mac or Linux");
        std::process::exit(1);
    }

    // Starting normal behavior
    info(&"Started backuper");

    if config_exists() {
        info(&"Config file found");

        let contents = fs::read_to_string(CONFIG_PATH)
            .expect("[ERROR] - Should have been able to read the file");
        let decoded = decode_file(&contents);

        ok(&"Config file decoded");
        ok(&"Configuration is valid");
        info(&"Processing");

        let _ = do_save(decoded);

        let google = decoded.transporter
            .as_ref()
            .and_then(|t| t.google.as_ref())
            .expect("Google transporter is not configured");

        info(&"Fetching a Google token");
        let token = get_google_access_token(google).unwrap_or_else(|err| {
            error(&format!("Unable to fetch token: {err:#?}"));
            err
        });

        ok(&"Google API token fetched");
        info(&"Sending data to Google Drive");

        let res = upload_to_drive(&token, &google.folder_id).unwrap_or_else(|err| {
            error(&format!("Unable to upload data: {err:#?}"));
            err
        });

        info(&format!("Uploaded data with response: {res}"));

        ok(&"Data successfully sent to Google");
        info(&"Cleaning rustbackuper folder");

        ensure_dest_folder_is_clean();
        ok(&"Successfully cleaned folder, exiting");
    } else {
        info(&"The config file doesn't exist");
        info(&"creating a new one based on default template");
        let _ = create_base_config();
        ok(&"Config file successfully created");

        if !!!cron_exists() {
            info(&"Cron file does not exist creating default file");
            let _ = create_base_cronjob();
            ok(&"Cron file successfully created");
        }
    }
}
