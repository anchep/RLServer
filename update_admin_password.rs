use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, DEFAULT_COST};

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
    
    // 使用raw SQL更新admin用户的密码
    let result = diesel::sql_query("UPDATE admin_users SET password_hash = $1 WHERE username = 'admin'")
        .bind::<diesel::sql_types::Text, _>(hashed_password)
        .execute(&mut conn);
    
    match result {
        Ok(count) => println!("Successfully updated {} user(s)", count),
        Err(e) => println!("Error updating user: {}", e),
    }
}