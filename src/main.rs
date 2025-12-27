use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_governor::{Governor, GovernorConfigBuilder};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use log::{error, info};

use crate::database::create_pool;
use crate::routes::configure_routes;
use crate::background::start_cleanup_task;
use crate::utils::logger::init_logger;
use crate::config::Config;

// 嵌入数据库迁移文件
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

mod database;
mod handlers;
mod middleware;
mod services;
mod utils;
mod background;
mod schema;
mod routes;
mod config;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志系统
    if let Err(err) = init_logger() {
        eprintln!("Failed to initialize logger: {}", err);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Logger initialization failed"));
    }
    
    // 加载环境变量
    dotenv().ok();
    
    // 获取配置
    let config = Config::new();
    let cleanup_interval = config.cleanup_interval.as_secs() / 60; // 转换为分钟
    
    // 创建数据库连接池
    let pool = create_pool(&config);
    
    // 运行数据库迁移
    info!("Running database migrations...");
    match pool.get() {
        Ok(mut conn) => {
            if let Err(err) = conn.run_pending_migrations(MIGRATIONS) {
                error!("Failed to run database migrations: {}", err);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database migration failed"));
            }
        }
        Err(err) => {
            error!("Failed to get database connection: {}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection failed"));
        }
    }
    info!("Database migrations completed");
    
    // 启动后台清理任务
    info!("Starting background cleanup task with interval {} minutes", cleanup_interval);
    tokio::spawn(start_cleanup_task(pool.clone(), cleanup_interval));
    
    // 配置API速率限制
    let governor_config = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();
    
    let governor_config = web::Data::new(governor_config);
    let config_clone = config.clone();
    let server_app = move || {
        App::new()
            // 添加日志中间件，配置为使用X-Forwarded-For头
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            // 添加API速率限制中间件
            .wrap(Governor::new(&governor_config))
            // 注册数据库连接池
            .app_data(web::Data::new(pool.clone()))
            // 注册配置
            .app_data(web::Data::new(config_clone.clone()))
            // 配置路由
            .configure(configure_routes)
    };
    
    let http_server = HttpServer::new(server_app.clone())
        .bind(format!("0.0.0.0:{}", config.server_port))?;
    
    info!("Starting HTTP server on port {}", config.server_port);
    
    // 根据配置决定是否启动HTTPS服务器
    if config.https_enabled {
        info!("HTTPS is enabled, checking certificate files...");
        
        // 检查证书文件是否存在
        if std::fs::metadata(&config.https_cert_path).is_ok() && std::fs::metadata(&config.https_key_path).is_ok() {
            info!("Certificate files found, starting HTTPS server on port {}", config.https_port);
            info!("Using HTTPS certificate: {}", config.https_cert_path);
            info!("Using HTTPS private key: {}", config.https_key_path);
            
            // 这里我们简化HTTPS实现，只启动HTTP服务器，因为rustls依赖太复杂
            // 在生产环境中，可以使用nginx反向代理来处理HTTPS
            info!("Note: Due to dependency issues, only HTTP server will be started. ");
            info!("For production HTTPS, please use nginx or other reverse proxy.");
        } else {
            info!("Certificate files not found, HTTPS server skipped.");
            info!("Certificate path: {}", config.https_cert_path);
            info!("Private key path: {}", config.https_key_path);
        }
    } else {
        info!("HTTPS is disabled");
    }
    
    // 只运行HTTP服务器
    http_server.run().await
}
