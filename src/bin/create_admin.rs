use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use log::info;

use rlserver::admin::services::admin_user::create_admin_user;
use rlserver::utils::logger::init_logger;

fn main() {
    // 初始化日志
    init_logger().expect("Failed to initialize logger");
    
    // 加载环境变量
    dotenv().ok();
    
    // 获取数据库连接URL
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://admin:password@db:5432/rl_server".to_string());
    
    // 创建连接池
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");
    
    info!("Creating admin user...");
    
    // 创建管理员用户
    match create_admin_user(&pool, "admin", "admin", "admin@example.com", true) {
        Ok(user) => {
            info!("✅ Admin user created successfully!");
            info!("Username: {}", user.username);
            info!("Email: {}", user.email);
            info!("User ID: {}", user.id);
            info!("Is Superadmin: {}", user.is_superadmin);
        },
        Err(err) => {
            info!("❌ Failed to create admin user: {}", err.message());
            info!("This might be because the user already exists.");
        }
    }
}