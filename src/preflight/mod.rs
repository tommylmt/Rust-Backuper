extern crate reqwest;

use std::path::Path;
use std::fs;

pub const CONFIG_PATH: &str = "/etc/rust-backuper/backuper.conf";

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
    let body = reqwest::blocking::get("https://raw.githubusercontent.com/tommylmt/Rust-Backuper/refs/heads/main/conf/sample.conf")?
        .text()?;
    
    println!("body: {body:?}");

    let path = std::path::Path::new(CONFIG_PATH);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    fs::write(CONFIG_PATH, body)?;

    Ok(())
}