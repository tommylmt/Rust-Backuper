use redis::{Client, Commands, RedisResult};
use std::thread;
use std::time::Duration;
use crate::logger::info;
use crate::toml::Cache;

pub fn dump_cache(cache: &Cache) -> bool {
    let url = format!("redis://{}:{}", cache.host, cache.port);
    info(&format!("Connecting to Redis at {url}"));

    let client = Client::open(url).expect("Invalid Redis URL");
    let mut con = client.get_connection().expect("Failed to connect to Redis");

    let last_save_before: i64 = redis::cmd("LASTSAVE")
        .query(&mut con)
        .expect("Failed to get LASTSAVE");

    // Trigger background save
    let _: () = redis::cmd("BGSAVE")
        .query(&mut con)
        .expect("Failed to trigger BGSAVE");

    info(&"BGSAVE triggered, waiting for completion...");

    loop {
        thread::sleep(Duration::from_secs(1));

        let last_save_after: i64 = redis::cmd("LASTSAVE")
            .query(&mut con)
            .expect("Failed to poll LASTSAVE");

        if last_save_after > last_save_before {
            info(&"Redis dump completed.");
            break;
        }
    }

    true
}