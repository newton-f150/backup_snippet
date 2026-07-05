use crate::config::Config;
use std::{fs, process::Command};

pub fn run() {
    let config = Config::load();

    fs::create_dir_all(&config.backup_dir).unwrap();

    let file = format!("{}/school.sql", config.backup_dir);

    Command::new("pg_dump")
        .env("PGPASSWORD", config.db_password)
        .args([
            "-U",
            &config.db_user,
            "-f",
            &file,
            &config.db_name,
        ])
        .status()
        .unwrap();

    println!("Backup completed.");
}