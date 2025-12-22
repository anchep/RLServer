use diesel::prelude::*;
use anyhow::Result;
use chrono::Utc;
use crate::database::{models::*, Pool};
use crate::schema::*;

pub async fn get_user_info(pool: &Pool, user_id: i32) -> Result<User> {
    let mut conn = pool.get()?;
    
    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)?;
    
    Ok(user)
}

pub async fn get_available_software(pool: &Pool, user_id: i32) -> Result<Vec<Software>> {
    let mut conn = pool.get()?;
    
    // 获取用户信息
    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)?;
    
    // 检查VIP是否过期
    let current_vip_level = if let Some(expires_at) = user.vip_expires_at {
        if expires_at > Utc::now() {
            user.vip_level
        } else {
            0
        }
    } else {
        0
    };
    
    // 获取可用软件列表
    let software_list = software::table
        .filter(software::required_vip_level.le(current_vip_level))
        .load::<Software>(&mut conn)?;
    
    Ok(software_list)
}

pub async fn update_user_vip(pool: &Pool, user_id: i32, new_vip_level: i32, duration_days: i32) -> Result<User> {
    let mut conn = pool.get()?;
    
    // 获取当前用户信息
    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)?;
    
    // 计算新的到期时间
    let new_expires_at = match user.vip_expires_at {
        Some(expires_at) => {
            if expires_at > Utc::now() {
                expires_at + chrono::Duration::days(duration_days as i64)
            } else {
                Utc::now() + chrono::Duration::days(duration_days as i64)
            }
        }
        None => {
            Utc::now() + chrono::Duration::days(duration_days as i64)
        }
    };
    
    // 更新用户VIP信息
    let updated_user = diesel::update(users::table.find(user_id))
        .set((
            users::vip_level.eq(new_vip_level),
            users::vip_expires_at.eq(new_expires_at),
            users::updated_at.eq(Utc::now()),
        ))
        .get_result::<User>(&mut conn)?;
    
    Ok(updated_user)
}
