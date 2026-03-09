use crate::preflight::{config_exists, create_base_config, CONFIG_PATH};
use crate::toml::decode_file;
use crate::validator::is_config_valid;
use std::fs;
use crate::logger::{info, ok, warning, critical};

pub mod logger;
pub mod preflight;
pub mod toml;
pub mod validator;

fn main() {
    info(&"Started backuper");

    if config_exists() {
        info(&"Config file found");

        let contents = fs::read_to_string(CONFIG_PATH)
            .expect("[ERROR] - Should have been able to read the file");
        let decoded = decode_file(&contents);

        ok(&"Config file decoded");

        if is_config_valid(&decoded) {
            warning(&"{decoded:#?}");
            // Call each saver
            // Call each bundler
            // Call each transporter
        } else {
            critical(&"The provided {CONFIG_PATH} file has invalid configuration");
        }
    } else {
        info(&"The config file doesn't exists");
        info(&"creating a new one based on default template");
        let _ = create_base_config();
    }
}
