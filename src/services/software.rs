use diesel::prelude::*;
use anyhow::Result;
use chrono::Utc;
use crate::database::{models::*, Pool};
use crate::schema::*;

pub async fn get_all_software(pool: &Pool) -> Result<Vec<Software>> {
    let mut conn = pool.get()?;
    
    let software_list = software::table
        .order_by(software::id)
        .load::<Software>(&mut conn)?;
    
    Ok(software_list)
}

pub async fn check_software_access(pool: &Pool, user_id: i32, software_id: i32) -> Result<bool> {
    let mut conn = pool.get()?;
    
    // 获取用户信息
    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)?;
    
    // 获取软件信息
    let software = software::table
        .find(software_id)
        .first::<Software>(&mut conn)?;
    
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
    
    // 检查是否有权限使用
    Ok(current_vip_level >= software.required_vip_level)
}
