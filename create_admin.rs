use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;

fn main() {
    dotenv().ok();
    
    // 获取数据库URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // 创建连接池
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");
    
    // 获取连接
    let mut conn = pool.get().expect("Failed to get connection");
    
    // 生成密码哈希
    let password = "admin";
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");
    
    // 使用raw SQL创建admin用户
    let result = diesel::sql_query("INSERT INTO admin_users (username, password_hash, email, is_superadmin, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind::<diesel::sql_types::Text, _>("admin")
        .bind::<diesel::sql_types::Text, _>(hashed_password)
        .bind::<diesel::sql_types::Text, _>("admin@example.com")
        .bind::<diesel::sql_types::Bool, _>(true)
        .bind::<diesel::sql_types::Timestamptz, _>(Utc::now())
        .bind::<diesel::sql_types::Timestamptz, _>(Utc::now())
        .execute(&mut conn);
    
    match result {
        Ok(count) => println!("Successfully created {} user(s)", count),
        Err(e) => println!("Error creating user: {}", e),
    }
}