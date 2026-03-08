use crate::preflight::{config_exists, create_base_config, CONFIG_PATH};
use std::fs;

pub mod preflight;

fn main() {
    /* TODO:
        - Preflight check: 
            do we have a configuration file in /etc/rust-backuper/backuper.conf
                is the file valid? 
            if not, create one and ask user to configure it
        - Savers:
            do a backup for each category of the backuper.conf files
        - Bundlers:
            pack elements with "archive = true" in config file in "zip"
        - Transporters:
            sync elements to the configured endpoint (Google Drive)
    */
    if config_exists() {
        println!("Config file found");

        let _contents = fs::read_to_string(CONFIG_PATH)
            .expect("Should have been able to read the file");

        // Decode TOML
        // Validate syntax
        // Call each saver
        // Call each bundler
        // Call each transporter
    } else {
        println!("The config file doesn't exists");
        println!("creating a new one based on default template");
        let _ = create_base_config();
    }
}
