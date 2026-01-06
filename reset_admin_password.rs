use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

mod schema {
    table! {
        admin_users (
            id -> Integer,
            username -> Varchar,
            password_hash -> Varchar,
            email -> Varchar,
            is_super_admin -> Bool,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        );
    }
}

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
    
    // 更新admin用户的密码
    use schema::admin_users::dsl::*;
    
    let result = diesel::update(admin_users.filter(username.eq("admin")))
        .set(password_hash.eq(hashed_password))
        .execute(&mut conn);
    
    match result {
        Ok(count) => println!("Successfully updated {} user(s)", count),
        Err(e) => println!("Error updating user: {}", e),
    }
}