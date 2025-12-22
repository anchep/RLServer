use diesel::prelude::*;
use anyhow::Result;
use chrono::Utc;
use crate::database::{models::*, Pool};
use crate::schema::*;

pub async fn update_heartbeat(pool: &Pool, session_token: &str, hardware_code: &str, software_version: &str) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 更新在线用户的最后活动时间
    diesel::update(online_users::table)
        .filter(online_users::session_token.eq(session_token))
        .set((
            online_users::last_activity_at.eq(Utc::now()),
            online_users::hardware_code.eq(hardware_code),
            online_users::software_version.eq(software_version),
        ))
        .execute(&mut conn)?;
    
    Ok(())
}

pub async fn cleanup_inactive_users(pool: &Pool, inactive_interval: i64) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 计算不活跃时间阈值
    let inactive_threshold = Utc::now() - chrono::Duration::minutes(inactive_interval as i64);
    
    // 删除超过阈值的在线用户记录
    diesel::delete(online_users::table)
        .filter(online_users::last_activity_at.lt(inactive_threshold))
        .execute(&mut conn)?;
    
    Ok(())
}
