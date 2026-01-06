use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::config::Config;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub fn create_pool(config: &Config) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub mod models;
pub mod verification_code;
