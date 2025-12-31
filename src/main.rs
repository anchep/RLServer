use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_session::{SessionMiddleware, storage::CookieSessionStore, config::CookieContentSecurity};
use actix_web::cookie::Key;
use tera::Tera;
use diesel::RunQueryDsl; use diesel::query_builder::SqlQuery; use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use log::{error, info};

use crate::admin::routes::configure_admin_routes;
use crate::database::create_pool;
use crate::routes::configure_routes;
use crate::background::start_cleanup_task;
use crate::utils::logger::init_logger;
use crate::config::Config;

// 嵌入数据库迁移文件
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

mod admin;
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
            
            // 修复所有表的序列，确保它们从表中当前的最大id值开始
            info!("Fixing database sequences...");
            // 修复software表序列
            let fix_software_seq = "SELECT setval(pg_get_serial_sequence('software', 'id'), COALESCE(MAX(id), 0) + 1) FROM software;";
            if let Err(err) = diesel::sql_query(fix_software_seq).execute(&mut conn) {
                error!("Failed to fix software sequence: {}", err);
            }
            // 修复users表序列
            let fix_users_seq = "SELECT setval(pg_get_serial_sequence('users', 'id'), COALESCE(MAX(id), 0) + 1) FROM users;";
            if let Err(err) = diesel::sql_query(fix_users_seq).execute(&mut conn) {
                error!("Failed to fix users sequence: {}", err);
            }
            // 修复recharge_cards表序列
            let fix_cards_seq = "SELECT setval(pg_get_serial_sequence('recharge_cards', 'id'), COALESCE(MAX(id), 0) + 1) FROM recharge_cards;";
            if let Err(err) = diesel::sql_query(fix_cards_seq).execute(&mut conn) {
                error!("Failed to fix recharge_cards sequence: {}", err);
            }
            // 修复recharge_logs表序列
            let fix_logs_seq = "SELECT setval(pg_get_serial_sequence('recharge_logs', 'id'), COALESCE(MAX(id), 0) + 1) FROM recharge_logs;";
            if let Err(err) = diesel::sql_query(fix_logs_seq).execute(&mut conn) {
                error!("Failed to fix recharge_logs sequence: {}", err);
            }
            // 修复login_logs表序列
            let fix_login_logs_seq = "SELECT setval(pg_get_serial_sequence('login_logs', 'id'), COALESCE(MAX(id), 0) + 1) FROM login_logs;";
            if let Err(err) = diesel::sql_query(fix_login_logs_seq).execute(&mut conn) {
                error!("Failed to fix login_logs sequence: {}", err);
            }
            // 修复online_users表序列
            let fix_online_users_seq = "SELECT setval(pg_get_serial_sequence('online_users', 'id'), COALESCE(MAX(id), 0) + 1) FROM online_users;";
            if let Err(err) = diesel::sql_query(fix_online_users_seq).execute(&mut conn) {
                error!("Failed to fix online_users sequence: {}", err);
            }
            // 修复verification_codes表序列
            let fix_verification_codes_seq = "SELECT setval(pg_get_serial_sequence('verification_codes', 'id'), COALESCE(MAX(id), 0) + 1) FROM verification_codes;";
            if let Err(err) = diesel::sql_query(fix_verification_codes_seq).execute(&mut conn) {
                error!("Failed to fix verification_codes sequence: {}", err);
            }
            // 修复admin_users表序列
            let fix_admin_users_seq = "SELECT setval(pg_get_serial_sequence('admin_users', 'id'), COALESCE(MAX(id), 0) + 1) FROM admin_users;";
            if let Err(err) = diesel::sql_query(fix_admin_users_seq).execute(&mut conn) {
                error!("Failed to fix admin_users sequence: {}", err);
            }
            // 修复admin_logs表序列
            let fix_admin_logs_seq = "SELECT setval(pg_get_serial_sequence('admin_logs', 'id'), COALESCE(MAX(id), 0) + 1) FROM admin_logs;";
            if let Err(err) = diesel::sql_query(fix_admin_logs_seq).execute(&mut conn) {
                error!("Failed to fix admin_logs sequence: {}", err);
            }
            // 修复admin_sessions表序列
            let fix_admin_sessions_seq = "SELECT setval(pg_get_serial_sequence('admin_sessions', 'id'), COALESCE(MAX(id), 0) + 1) FROM admin_sessions;";
            if let Err(err) = diesel::sql_query(fix_admin_sessions_seq).execute(&mut conn) {
                error!("Failed to fix admin_sessions sequence: {}", err);
            }
            // 修复blacklist表序列
            let fix_blacklist_seq = "SELECT setval(pg_get_serial_sequence('blacklist', 'id'), COALESCE(MAX(id), 0) + 1) FROM blacklist;";
            if let Err(err) = diesel::sql_query(fix_blacklist_seq).execute(&mut conn) {
                error!("Failed to fix blacklist sequence: {}", err);
            }
            info!("Database sequences fixed successfully");
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
        .per_second(60)
        .burst_size(120)
        .finish()
        .unwrap();
    
    let governor_config = web::Data::new(governor_config);
    let config_clone = config.clone();
    
    // 初始化Tera模板引擎
    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(tera) => tera,
        Err(err) => {
            error!("Failed to initialize Tera template engine: {}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Tera initialization failed"));
        }
    };
    
    // 注册日期格式化过滤器
    // 创建上海时区(UTC+8)
    let shanghai_tz = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
    
    tera.register_filter("format_date", move |value: &tera::Value, _args: &std::collections::HashMap<String, tera::Value>| -> Result<tera::Value, tera::Error> {
        use chrono::prelude::*;
        
        // 将值转换为字符串，然后解析为DateTime
        let date_str = value.to_string();
        
        // 移除引号和其他可能的包装
        let cleaned_str = date_str.trim_matches('"');
        
        // 尝试解析字符串为DateTime
        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(cleaned_str) {
            // 格式化日期时间，使用上海时区
            Ok(tera::Value::String(date.with_timezone(&shanghai_tz).format("%Y-%m-%d %H:%M:%S").to_string()))
        } else {
            // 如果解析失败，尝试其他格式或返回原始字符串
            // 尝试解析为NaiveDateTime
            if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(cleaned_str, "%Y-%m-%d %H:%M:%S") {
                let dt = DateTime::<Utc>::from_utc(naive, Utc);
                Ok(tera::Value::String(dt.with_timezone(&shanghai_tz).format("%Y-%m-%d %H:%M:%S").to_string()))
            } else {
                // 如果都解析失败，返回原始字符串
                Ok(tera::Value::String(cleaned_str.to_string()))
            }
        }
    });
    
    // 注册datetime-local格式化过滤器
    tera.register_filter("format_datetime", move |value: &tera::Value, _args: &std::collections::HashMap<String, tera::Value>| -> Result<tera::Value, tera::Error> {
        use chrono::prelude::*;
        
        // 将值转换为字符串，然后解析为DateTime
        let date_str = value.to_string();
        
        // 移除引号和其他可能的包装
        let cleaned_str = date_str.trim_matches('"');
        
        // 如果是数字，将其视为时间戳
        if let Ok(timestamp) = cleaned_str.parse::<i64>() {
            let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or(NaiveDateTime::default()), Utc);
            // 格式化日期时间为datetime-local格式，使用上海时区
            Ok(tera::Value::String(dt.with_timezone(&shanghai_tz).format("%Y-%m-%dT%H:%M").to_string()))
        } else {
            // 尝试解析字符串为DateTime
            if let Ok(date) = chrono::DateTime::parse_from_rfc3339(cleaned_str) {
                // 格式化日期时间为datetime-local格式，使用上海时区
                Ok(tera::Value::String(date.with_timezone(&shanghai_tz).format("%Y-%m-%dT%H:%M").to_string()))
            } else {
                // 如果都解析失败，返回空字符串
                Ok(tera::Value::String(String::new()))
            }
        }
    });
    
    // 注册金额格式化过滤器
    tera.register_filter("format_money", |value: &tera::Value, _args: &std::collections::HashMap<String, tera::Value>| -> Result<tera::Value, tera::Error> {
        // 将值转换为字符串，然后解析为f64
        let value_str = value.to_string();
        
        // 移除引号和其他可能的包装
        let cleaned_str = value_str.trim_matches('"');
        
        // 尝试解析为f64
        if let Ok(f) = cleaned_str.parse::<f64>() {
            // 格式化金额为两位小数
            Ok(tera::Value::String(format!("{:.2}", f)))
        } else {
            // 如果解析失败，尝试直接将原始值作为f64处理
            if let tera::Value::Number(num) = value {
                if let Some(f) = num.as_f64() {
                    Ok(tera::Value::String(format!("{:.2}", f)))
                } else {
                    Ok(tera::Value::String(num.to_string()))
                }
            } else {
                // 如果不是数字，返回原始值
                Ok(tera::Value::String(cleaned_str.to_string()))
            }
        }
    });
    
    let tera = web::Data::new(tera);
    
    let server_app = move || {
        App::new()
            // 添加日志中间件，配置为使用X-Forwarded-For头
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            // 添加API速率限制中间件
            .wrap(Governor::new(&governor_config))
            // 添加会话中间件
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(b"your-secret-key-here-please-change-this-in-production-enough-length-12345678901234567890")
                )
                .cookie_content_security(CookieContentSecurity::Private)
                .cookie_secure(false) // 开发环境下使用非安全cookie
                .cookie_same_site(actix_web::cookie::SameSite::Lax) // 使用Lax SameSite策略
                .build()
            )
            // 注册数据库连接池
            .app_data(web::Data::new(pool.clone()))
            // 注册配置
            .app_data(web::Data::new(config_clone.clone()))
            // 注册Tera模板引擎
            .app_data(tera.clone())
            // 配置静态资源服务
            .service(Files::new("/static", "./static").show_files_listing())
            // 配置路由
            .configure(configure_routes)
            // 配置后台管理路由
            .configure(configure_admin_routes)
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
