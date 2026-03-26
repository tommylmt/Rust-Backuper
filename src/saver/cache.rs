use redis::Client;
use std::thread;
use std::time::Duration;
use std::process::Command;
use crate::logger::info;
use crate::toml::Cache;
use crate::saver::DEST_FOLDER;

pub fn dump_cache(cache: &Cache) -> bool {
    let url = format!("redis://{}:{}", cache.host, cache.port);
    info(&format!("Connecting to Redis at {url}"));

    let client = Client::open(url).expect("Invalid Redis URL");
    let mut con = client.get_connection().expect("Failed to connect to Redis");

    let rdb_dir: String = redis::cmd("CONFIG")
        .arg("GET")
        .arg("dir")
        .query::<Vec<String>>(&mut con)
        .expect("Failed to get Redis dir config")
        .into_iter()
        .nth(1)
        .expect("Empty CONFIG GET dir response");

    let rdb_filename: String = redis::cmd("CONFIG")
        .arg("GET")
        .arg("dbfilename")
        .query::<Vec<String>>(&mut con)
        .expect("Failed to get Redis dbfilename config")
        .into_iter()
        .nth(1)
        .expect("Empty CONFIG GET dbfilename response");

    info(&format!("Redis RDB location inside container: {rdb_dir}/{rdb_filename}"));

    let last_save_before: i64 = redis::cmd("LASTSAVE")
        .query(&mut con)
        .expect("Failed to get LASTSAVE");

    let _: () = redis::cmd("BGSAVE")
        .query(&mut con)
        .expect("Failed to trigger BGSAVE");

    info("BGSAVE triggered, waiting for completion...");

    // Poll with a timeout to avoid looping forever
    let timeout = Duration::from_secs(60);
    let start = std::time::Instant::now();

    loop {
        thread::sleep(Duration::from_secs(1));

        if start.elapsed() > timeout {
            eprintln!("Timed out waiting for BGSAVE to complete");
            return false;
        }

        let last_save_after: i64 = redis::cmd("LASTSAVE")
            .query(&mut con)
            .expect("Failed to poll LASTSAVE");

        if last_save_after > last_save_before {
            info("Redis dump completed.");
            break;
        }
    }

    copy_rdb_from_container(
        &cache.container_name,
        &format!("{}/{}", rdb_dir, rdb_filename),
        &format!("{}redis.rdb", DEST_FOLDER),
    )
}

fn copy_rdb_from_container(container: &str, src: &str, dest: &str) -> bool {
    info(&format!("Copying RDB from container '{container}:{src}' to '{dest}'"));

    let status = Command::new("docker")
        .args(["cp", &format!("{container}:{src}"), dest])
        .status();

    match status {
        Ok(s) if s.success() => {
            info(&format!("RDB successfully saved to {dest}"));
            true
        }
        Ok(s) => {
            eprintln!("docker cp failed with exit code: {}", s.code().unwrap_or(-1));
            false
        }
        Err(e) => {
            eprintln!("Failed to run docker cp: {e}");
            false
        }
    }
}