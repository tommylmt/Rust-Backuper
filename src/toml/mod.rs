extern crate toml;
extern crate serde;

use toml::value::Array;
use serde::Deserialize;
use crate::logger::{warning, critical};

#[derive(Debug, Deserialize)]
struct Db {
    archive: bool,
    host: String,
    port: i32,
    user: String,
    password: String,
    database: String,
    driver: String,
}

#[derive(Debug, Deserialize)]
struct Cache {
    archive: bool,
    host: String,
    port: i32,
    driver: String,
}

#[derive(Debug, Deserialize)]
struct Files {
    archive: bool,
    path: Option<String>,
    files: Option<Array>,
    archive_password: String,
}

#[derive(Debug, Deserialize)]
struct Saver {
    db: Option<Db>,
    cache: Option<Cache>,
    files: Option<Files>,
}

#[derive(Debug, Deserialize)]
struct Google {
    client: String,
    token: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Transporter {
    google: Option<Google>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    saver: Option<Saver>,
    transporter: Option<Transporter>,
}

impl Config {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors: Vec<String> = Vec::new();

        match &self.saver {
            None => errors.push("At least one [saver] section must be defined".to_string()),
            Some(saver) => {
                if saver.db.is_none() && saver.cache.is_none() && saver.files.is_none() {
                    errors.push("At least one saver (db, cache, files) must be configured".to_string());
                }

                if let Some(db) = &saver.db {
                    if db.port <= 0 || db.port > 65535 {
                        errors.push(format!("saver.db.port is invalid: {}", db.port));
                    }
                    let supported_drivers = ["mysql", "postgresql", "sqlite"];
                    if !supported_drivers.contains(&db.driver.as_str()) {
                        errors.push(format!(
                            "saver.db.driver '{}' is not supported. Use one of: {:?}",
                            db.driver, supported_drivers
                        ));
                    }
                }

                if let Some(cache) = &saver.cache {
                    if cache.port <= 0 || cache.port > 65535 {
                        errors.push(format!("saver.cache.port is invalid: {}", cache.port));
                    }
                    let supported_drivers = ["redis"];
                    if !supported_drivers.contains(&cache.driver.as_str()) {
                        errors.push(format!(
                            "saver.cache.driver '{}' is not supported. Use one of: {:?}",
                            cache.driver, supported_drivers
                        ));
                    }
                }

                if let Some(files) = &saver.files {
                    if files.archive && files.archive_password.is_empty() {
                        errors.push("saver.files.archive_password must be set when archive = true".to_string());
                    }
                    if files.path.is_none() && files.files.is_none() {
                        errors.push("saver.files must define at least a path or a files array".to_string());
                    }
                }
            }
        }

        if let Some(transporter) = &self.transporter {
            if let Some(google) = &transporter.google {
                for (field, value) in [("client", &google.client), ("token", &google.token), ("url", &google.url)] {
                    if value == "change me" || value.is_empty() {
                        errors.push(format!("transporter.google.{} is not configured", field));
                    }
                }
            }
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

pub fn decode_file(content: &str) -> Config {
    let config: Config = toml::from_str(content).expect("[ERROR] - The config file cannot be decoded");
    
    if let Err(errors) = config.validate() {
        for e in &errors {
            warning(e);
        }

        critical(&"Config validation failed with warning(s)");
    }

    config
}