use crate::preflight::{config_exists, create_base_config, CONFIG_PATH};
use crate::toml::decode_file;
use crate::validator::is_config_valid;
use std::fs;

pub mod preflight;
pub mod toml;
pub mod validator;

fn main() {
    if config_exists() {
        println!("[INFO] - Config file found");

        let contents = fs::read_to_string(CONFIG_PATH)
            .expect("[ERROR] - Should have been able to read the file");

        let decoded = decode_file(&contents);

        if is_config_valid(&decoded) {
            println!("[DEBUG] - {decoded:#?}");
            // Call each saver
            // Call each bundler
            // Call each transporter
        } else {
            panic!("[CRITICAL] - The provided {CONFIG_PATH} file has invalid configuration");
        }
    } else {
        println!("[INFO] - The config file doesn't exists");
        println!("[INFO] - creating a new one based on default template");
        let _ = create_base_config();
    }
}
