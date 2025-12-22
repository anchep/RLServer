use dotenv::dotenv;
use std::env;
use std::time::Duration;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub heartbeat_interval: Duration,
    pub cleanup_interval: Duration,
    pub server_port: u16,
    // SMTP配置
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_email: String,
    pub smtp_ssl: bool,
    pub smtp_timeout: Duration,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            heartbeat_interval: Duration::from_secs(
                env::var("HEARTBEAT_INTERVAL").unwrap_or("600".to_string()).parse().unwrap_or(600)
            ),
            cleanup_interval: Duration::from_secs(
                env::var("CLEANUP_INTERVAL").unwrap_or("300".to_string()).parse().unwrap_or(300)
            ),
            server_port: env::var("SERVER_PORT").unwrap_or("28001".to_string()).parse().unwrap_or(28001),
            // SMTP配置
            smtp_host: env::var("SMTP_HOST").unwrap_or("smtp.example.com".to_string()),
            smtp_port: env::var("SMTP_PORT").unwrap_or("587".to_string()).parse().unwrap_or(587),
            smtp_username: env::var("SMTP_USERNAME").unwrap_or("username".to_string()),
            smtp_password: env::var("SMTP_PASSWORD").unwrap_or("password".to_string()),
            smtp_from_email: env::var("SMTP_FROM_EMAIL").unwrap_or("no-reply@example.com".to_string()),
            smtp_ssl: env::var("SMTP_SSL").unwrap_or("false".to_string()).parse().unwrap_or(false),
            smtp_timeout: Duration::from_secs(
                env::var("SMTP_TIMEOUT").unwrap_or("30".to_string()).parse().unwrap_or(30)
            ),
        }
    }
}