extern crate reqwest;

use std::path::Path;
use std::fs;

pub const CONFIG_PATH: &str = "/etc/rust-backuper/backuper.conf";
const CRON_PATH: &str = "/etc/cron.d/rust-backuper";

/*
* Check if the configuration file exists, else will create a new one based on the sample
*/
pub fn config_exists() -> bool {
    Path::new(CONFIG_PATH).exists()
}

/*
 * Retrieve the default backup conf from the Github repository and write it into new config file
 */
pub fn create_base_config() -> Result<(), Box<dyn std::error::Error>> {
    create_file(CONFIG_PATH, "sample.conf")
}

pub fn cron_exists() -> bool {
    Path::new(CRON_PATH).exists()
}

pub fn create_base_cronjob() -> Result<(), Box<dyn std::error::Error>> {
    create_file(CRON_PATH, "rust-backuper-cron")
}

fn create_file(dest: &str, remotename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut owned_string: String = "https://raw.githubusercontent.com/tommylmt/Rust-Backuper/refs/heads/main/conf/".to_owned();
    owned_string.push_str(remotename);

    let body = reqwest::blocking::get(owned_string)?
        .text()?;
    
    let path = std::path::Path::new(dest);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    fs::write(dest, body)?;

    Ok(())
}