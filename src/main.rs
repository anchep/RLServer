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
    
    // 启动HTTP服务器
    let server_port = config.server_port;
    info!("Starting server on port {}", server_port);
    
    // 配置API速率限制
    let governor_config = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();
    
    HttpServer::new(move || {
        App::new()
            // 添加日志中间件，配置为使用X-Forwarded-For头
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            // 添加API速率限制中间件
            .wrap(Governor::new(governor_config.clone()))
            // 注册数据库连接池
            .app_data(web::Data::new(pool.clone()))
            // 注册配置
            .app_data(web::Data::new(config.clone()))
            // 配置路由
            .configure(configure_routes)
    })
    // 配置信任的代理，用于获取真实客户端IP
    .forwarded_header(actix_web::forwarded::ForwardedHeader::XForwardedFor)
    .bind(format!("0.0.0.0:{}", server_port))?
    .run()
    .await
}
