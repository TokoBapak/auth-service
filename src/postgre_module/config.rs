use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

pub struct AppConfig {
    pub db_name: String,
    pub db_port: String,
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            db_name: env::var("DB_NAME").expect("DB_NAME not set in .env"),
            db_port: env::var("DB_PORT").expect("DB_PORT not set in .env"),
            db_host: env::var("DB_HOST").expect("DB_HOST not set in .env"),
            db_user: env::var("DB_USER").expect("DB_USER not set in .env"),
            db_password: env::var("DB_PASSWORD").expect("DB_PASSWORD not set in .env"),
        }
    }
}

// Variabel statis untuk menyimpan konfigurasi secara global
lazy_static! {
    pub static ref APP_CONFIG: AppConfig = AppConfig::load();
    pub static ref PARAMS: String = format!(
        "dbname={} port={} host={} user={} password={}",
        APP_CONFIG.db_name,
        APP_CONFIG.db_port,
        APP_CONFIG.db_host,
        APP_CONFIG.db_user,
        APP_CONFIG.db_password
    );
}