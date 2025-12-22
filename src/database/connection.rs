use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Arc;

// 定义连接类型
pub type DbConnection = PgConnection;
// 定义连接池类型
pub type DbPool = Arc<r2d2::Pool<ConnectionManager<DbConnection>>>;
// 定义连接池错误类型
pub type DbPoolError = r2d2::Error;

// 初始化数据库连接池
pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<DbConnection>::new(database_url);
    
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect(&format!("Failed to create pool for database: {}", database_url));
    
    Arc::new(pool)
}

// 从连接池获取连接
pub async fn get_connection(pool: &DbPool) -> Result<r2d2::PooledConnection<ConnectionManager<DbConnection>>, DbPoolError> {
    let pool = pool.clone();
    
    tokio::task::spawn_blocking(move || {
        pool.get()
    })
    .await
    .expect("Failed to spawn blocking task")
}