use std::process::Command;
use tokio::time::{sleep, Duration};

fn backup_database() {
    Command::new("pg_dump")
        .args([
            "-U",
            "postgres",
            "-f",
            "backups/school_v1.sql",
            "school",
        ])
        .status()
        .expect("Failed to execute pg_dump");
}


async fn main (){

loop {
    backup_database();

    sleep(Duration::from_secs(60 * 60)).await;
}
}