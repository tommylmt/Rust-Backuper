use crate::toml::{Config};
use std::fs;
use struct_iterable::Iterable;
use std::path::Path;
use crate::logger::{info, error};
use database::dump_database;
use cache::dump_cache;
use files::dump_files;
use crate::toml::{Db, Cache, Files};

pub mod database;
pub mod files;
pub mod cache;

pub const DEST_FOLDER: &str = "/tmp/rustbackuper/";

pub fn do_save(config: &Config) -> Result<(), std::string::String> {
    ensure_dest_folder_is_clean();

    if let Some(saver) = &config.saver {
        for (field, saver_options) in saver.iter() {
            info(&format!("Processing saver : {field}"));

            let is_saved: bool = match field {
                "db" => {
                    match saver_options.downcast_ref::<Option<Db>>() {
                        Some(Some(db)) => dump_database(db),
                        _ => false,
                    }
                }
                "cache" => {
                    match saver_options.downcast_ref::<Option<Cache>>() {
                        Some(Some(cache)) => dump_cache(cache),
                        _ => false,
                    }
                },
                "files" => {
                    match saver_options.downcast_ref::<Option<Files>>() {
                        Some(Some(cache)) => dump_files(cache),
                        _ => false,
                    }
                }
                _ => false
            };

            if !!!is_saved {
                error(&format!("An occured when calling saver {field}"));
                let mut message = String::from("An error occured while running saver ");
                message.push_str(field);

                return Err(message);
            }
        }
    }

    Ok(())
}

pub fn ensure_dest_folder_is_clean() {
    if Path::new(DEST_FOLDER).exists() {
        fs::remove_dir_all(DEST_FOLDER).unwrap();
    }

    fs::create_dir(DEST_FOLDER).unwrap();
}