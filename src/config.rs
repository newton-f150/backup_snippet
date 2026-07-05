use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub backup_dir: String,
}

impl Config{
    pub fn load() -> Self{
        dotenvy.ok
    }
}