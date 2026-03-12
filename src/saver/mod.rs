use crate::toml::{Config};
use std::fs;
use struct_iterable::Iterable;
use std::path::Path;
use crate::logger::{info, ok};

pub mod database;
pub mod files;
pub mod cache;

const DEST_FOLDER: &str = "/tmp/rustbackuper/";

pub fn do_save(config: Config) -> Result<(), std::string::String> {
    ensure_dest_folder_is_clean();

    if let Some(saver) = &config.saver {
        for (field, saver_options) in saver.iter() {
            println!("Processing saver : {field}");

            let is_saved: bool = match field {
                "db" => {
                    println!("Will call database!");
                    
                    true
                },
                "cache" => {
                    println!("Will call database!");
                    
                    true
                },
                "files" => {
                    println!("Will call file saver");

                    true
                }
                _ => {
                    println!("Unknown value");

                    false
                }
            };

            if !!!is_saved {
                let mut message = String::from("An error occured while running saver ");
                message.push_str(field);

                return Err(message);
            }
        }
    }

    Ok(())
}

fn ensure_dest_folder_is_clean() {
    if Path::new(DEST_FOLDER).exists() {
        fs::remove_dir_all(DEST_FOLDER).unwrap();
    }

    fs::create_dir(DEST_FOLDER).unwrap();
}