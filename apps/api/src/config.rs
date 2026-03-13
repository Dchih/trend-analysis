use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub bind_addr: String,
    pub database_url: String,
    pub redis_url: String,
    pub redis_stream: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            bind_addr: env::var("PRODUCT_RADAR_BIND_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            database_url: env::var("PRODUCT_RADAR_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres@localhost:5432/product_radar".to_string()),
            redis_url: env::var("PRODUCT_RADAR_REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string()),
            redis_stream: env::var("PRODUCT_RADAR_REDIS_STREAM")
                .unwrap_or_else(|_| "collect_tasks".to_string()),
        }
    }
}
