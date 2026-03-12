use crate::preflight::{config_exists, create_base_config, cron_exists, create_base_cronjob, CONFIG_PATH};
use crate::toml::decode_file;
use std::fs;
use crate::logger::{info, ok, warning, critical};

pub mod logger;
pub mod preflight;
pub mod toml;

fn main() {
    // Version display
    if std::env::args().nth(1) == Some("--version".to_string()) {
        let version = env!("CARGO_PKG_VERSION");

        println!("Version {version}");
        std::process::exit(0)
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

        // for each saver, process in tmp folder
        // then the transporter
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
