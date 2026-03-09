extern crate toml;
extern crate serde;

use toml::value::Array;
use serde::Deserialize;

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

pub fn decode_file(content: &str) -> Config {
    toml::from_str(content).expect("[ERROR] - The config file cannot be decoded")
}