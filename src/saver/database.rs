use crate::toml::Db;
use std::process::Command;
use crate::logger::{info};
use crate::saver::DEST_FOLDER;

pub fn dump_database(database: &Db) -> bool {
    info(&"running a database dump");
    let mut output = true;
    let options = [ 
        "-u",
        &database.user, 
        "--password", 
        &database.password, 
        "--host", 
        &database.host, 
        "--port", 
        &database.port.to_string(), 
        &database.database,
        ">",
        DEST_FOLDER
    ];

    if database.driver == "mysql" {
        info(&format!("backuping database {} using mysqldump", database.database));
        let status = Command::new("mysqldump")
            .args(options)
            .status()
            .expect("Failed to save database with mysqldump")
        ;

        output = status.success();
    } else if database.driver == "postresql" {
        info(&format!("backuping database {} using pg_dump", database.database));
        let status = Command::new("pg_dump")
            .args(options)
            .status()
            .expect("Failed to save database with mysqldump")
        ;

        output = status.success();
    }

    output
}