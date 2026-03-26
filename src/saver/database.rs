use crate::toml::Db;
use std::process::Command;
use std::fs::File;
use crate::logger::info;
use crate::saver::DEST_FOLDER;

pub fn dump_database(database: &Db) -> bool {
    info("running a database dump");

    let destination = format!("{}database.sql", DEST_FOLDER);

    // Build the port string here so it lives long enough
    let port_str = database.port.to_string();
    let password_arg = format!("--password={}", database.password);

    if database.driver == "mysql" {
        info(&format!("backuping database {} using mysqldump", database.database));

        let options = [
            "-u", &database.user,
            &password_arg,
            "--host", &database.host,
            "--port", &port_str,
            &database.database,
        ];

        run_dump("mysqldump", &options, &destination)

    } else if database.driver == "postgresql" {
        info(&format!("backuping database {} using pg_dump", database.database));

        // pg_dump uses PGPASSWORD env var instead of a --password flag
        let options = [
            "-U", &database.user,
            "--host", &database.host,
            "--port", &port_str,
            &database.database,
        ];

        run_dump_with_env("pg_dump", &options, &destination, &database.password)

    } else {
        eprintln!("Unsupported database driver: {}", database.driver);
        false
    }
}

fn run_dump(command: &str, args: &[&str], destination: &str) -> bool {
    let output_file = File::create(destination)
        .expect("Failed to create destination file");

    let status = Command::new(command)
        .args(args)
        .stdout(output_file)   // redirect stdout to the file directly
        .status()
        .expect(&format!("Failed to run {}", command));

    status.success()
}

fn run_dump_with_env(command: &str, args: &[&str], destination: &str, password: &str) -> bool {
    let output_file = File::create(destination)
        .expect("Failed to create destination file");

    let status = Command::new(command)
        .args(args)
        .env("PGPASSWORD", password)  // pg_dump reads password from env
        .stdout(output_file)
        .status()
        .expect(&format!("Failed to run {}", command));

    status.success()
}