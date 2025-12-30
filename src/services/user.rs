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

// 获取软件列表和用户VIP信息
pub async fn get_software_with_vip_info(pool: &Pool, user_id: i32) -> Result<(i32, Option<chrono::DateTime<chrono::Utc>>, Vec<Software>)> {
    let mut conn = pool.get()?;
    
    // 获取用户信息
    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)?;
    
    // 检查VIP是否过期
    let (vip_level, vip_expires_at) = if let Some(expires_at) = user.vip_expires_at {
        if expires_at > Utc::now() {
            (user.vip_level, Some(expires_at))
        } else {
            (0, None)
        }
    } else {
        (0, None)
    };
    
    // 获取可用软件列表
    let software_list = software::table
        .filter(software::required_vip_level.le(vip_level))
        .load::<Software>(&mut conn)?;
    
    Ok((vip_level, vip_expires_at, software_list))
}
