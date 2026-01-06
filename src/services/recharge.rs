use diesel::prelude::*;
use anyhow::Result;
use chrono::Utc;
use crate::database::{models::*, Pool};
use crate::schema::*;


pub async fn recharge_with_card(pool: &Pool, user_id: i32, card_code: &str) -> Result<(User, RechargeLog)> {
    let mut conn = pool.get()?;
    
    // 查找卡密
    let card = recharge_cards::table
        .filter(recharge_cards::card_code.eq(card_code))
        .first::<RechargeCard>(&mut conn)
        .optional()?;
    
    // 检查卡密是否存在
    let mut card = match card {
        Some(card) => card,
        None => {
            return Err(anyhow::anyhow!("Card not found"));
        }
    };
    
    // 检查卡密是否已使用
    if card.is_used {
        let used_at = card.used_at.expect("Used card should have used_at timestamp");
        return Err(anyhow::anyhow!("Card already used at {}", used_at));
    }
    
    // 获取当前用户信息
    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)?;
    
    // 计算新的到期时间
    let new_expires_at = match user.vip_expires_at {
        Some(expires_at) => {
            if expires_at > Utc::now() {
                expires_at + chrono::Duration::days(card.duration_days as i64)
            } else {
                Utc::now() + chrono::Duration::days(card.duration_days as i64)
            }
        }
        None => {
            Utc::now() + chrono::Duration::days(card.duration_days as i64)
        }
    };
    
    // 标记卡密为已使用
    diesel::update(recharge_cards::table.find(card.id))
        .set((
            recharge_cards::is_used.eq(true),
            recharge_cards::used_at.eq(Utc::now()),
            recharge_cards::used_by.eq(user_id),
        ))
        .execute(&mut conn)?;
    
    // 更新用户VIP信息
    let updated_user = diesel::update(users::table.find(user_id))
        .set((
            users::vip_level.eq(card.vip_level),
            users::vip_expires_at.eq(new_expires_at),
            users::updated_at.eq(Utc::now()),
        ))
        .get_result::<User>(&mut conn)?;
    
    // 记录充值日志
    let recharge_log = diesel::insert_into(recharge_logs::table)
        .values((
            recharge_logs::user_id.eq(user_id),
            recharge_logs::card_code.eq(card_code),
            recharge_logs::vip_level.eq(card.vip_level),
            recharge_logs::duration_days.eq(card.duration_days),
            recharge_logs::recharge_time.eq(Utc::now()),
            recharge_logs::created_at.eq(Utc::now()),
        ))
        .get_result::<RechargeLog>(&mut conn)?;
    
    Ok((updated_user, recharge_log))
}

pub async fn get_recharge_logs(pool: &Pool, user_id: i32) -> Result<Vec<RechargeLog>> {
    let mut conn = pool.get()?;
    
    let logs = recharge_logs::table
        .filter(recharge_logs::user_id.eq(user_id))
        .order_by(recharge_logs::created_at.desc())
        .load::<RechargeLog>(&mut conn)?;
    
    Ok(logs)
}
