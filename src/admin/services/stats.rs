use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use diesel::prelude::*;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use crate::errors::ServiceError;
use chrono::{Utc, DateTime, Duration, NaiveDateTime};
use diesel::dsl::{sum, count};

// 获取统计信息
pub fn get_stats(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<serde_json::Value, ServiceError> {
    let mut conn = pool.get()?;
    
    // 总用户数
    let total_users = crate::schema::users::table.count().first::<i64>(&mut conn)?;
    
    // 总软件数
    let total_software = crate::schema::software::table.count().first::<i64>(&mut conn)?;
    
    // 总卡密数
    let total_cards = crate::schema::recharge_cards::table.count().first::<i64>(&mut conn)?;
    
    // 已使用卡密数
    let used_cards = crate::schema::recharge_cards::table
        .filter(crate::schema::recharge_cards::is_used.eq(true))
        .count()
        .first::<i64>(&mut conn)?;
    
    // 总销售金额
    let total_sales_bd = crate::schema::recharge_cards::table
        .filter(crate::schema::recharge_cards::is_used.eq(true))
        .select(sum(crate::schema::recharge_cards::price))
        .first::<Option<BigDecimal>>(&mut conn)?
        .unwrap_or(BigDecimal::from(0));
    
    // 转换为i32
    let total_sales = total_sales_bd.to_i32().unwrap_or(0);
    
    Ok(serde_json::json!({
        "total_users": total_users,
        "total_software": total_software,
        "total_cards": total_cards,
        "used_cards": used_cards,
        "total_sales": total_sales
    }))
}

// 获取用户统计
pub fn get_user_stats(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<serde_json::Value, ServiceError> {
    let mut conn = pool.get()?;
    
    // 总用户数
    let total_users = crate::schema::users::table.count().first::<i64>(&mut conn)?;
    
    // 活跃用户数（最近30天登录过）
    let thirty_days_ago = Utc::now() - Duration::days(30);
    let active_users = crate::schema::login_logs::table
        .filter(crate::schema::login_logs::login_time.ge(thirty_days_ago))
        .select(crate::schema::login_logs::user_id)
        .distinct()
        .load::<i32>(&mut conn)?
        .len() as i64;
    
    // 在线用户数
    let online_users = crate::schema::online_users::table.count().first::<i64>(&mut conn)?;
    
    // VIP用户数
    let vip_users = crate::schema::users::table
        .filter(crate::schema::users::vip_expires_at.gt(Utc::now()))
        .count()
        .first::<i64>(&mut conn)?;
    
    Ok(serde_json::json!(
        {
            "total_users": total_users,
            "active_users": active_users,
            "online_users": online_users,
            "vip_users": vip_users
        }
    ))
}

// 获取卡密统计
pub fn get_card_stats(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<serde_json::Value, ServiceError> {
    let mut conn = pool.get()?;
    
    // 总卡密数
    let total_cards = crate::schema::recharge_cards::table.count().first::<i64>(&mut conn)?;
    
    // 未使用卡密数
    let unused_cards = crate::schema::recharge_cards::table
        .filter(crate::schema::recharge_cards::is_used.eq(false))
        .count()
        .first::<i64>(&mut conn)?;
    
    // 已使用卡密数
    let used_cards = crate::schema::recharge_cards::table
        .filter(crate::schema::recharge_cards::is_used.eq(true))
        .count()
        .first::<i64>(&mut conn)?;
    
    // 总销售金额
    let total_sales_bd = crate::schema::recharge_cards::table
        .filter(crate::schema::recharge_cards::is_used.eq(true))
        .select(sum(crate::schema::recharge_cards::price))
        .first::<Option<BigDecimal>>(&mut conn)?
        .unwrap_or(BigDecimal::from(0));
    
    // 转换为i32
    let total_sales = total_sales_bd.to_i32().unwrap_or(0);
    
    Ok(serde_json::json!(
        {
            "total_cards": total_cards,
            "unused_cards": unused_cards,
            "used_cards": used_cards,
            "total_sales": total_sales
        }
    ))
}

// 获取销售业绩统计
pub fn get_sales_stats(
    pool: &Pool<ConnectionManager<PgConnection>>,
    start_time: i64,
    end_time: i64,
) -> Result<serde_json::Value, ServiceError> {
    let mut conn = pool.get()?;
    
    // 转换时间戳为DateTime<Utc>
    let start = DateTime::from_timestamp(start_time, 0).unwrap_or(Utc::now() - Duration::days(30));
    let end = DateTime::from_timestamp(end_time, 0).unwrap_or(Utc::now());
    
    // 计算时间范围内的销售总额
    let total_sales_bd = crate::schema::recharge_logs::table
        .inner_join(crate::schema::recharge_cards::table.on(crate::schema::recharge_logs::card_code.eq(crate::schema::recharge_cards::card_code)))
        .filter(crate::schema::recharge_logs::created_at.ge(start))
        .filter(crate::schema::recharge_logs::created_at.le(end))
        .select(sum(crate::schema::recharge_cards::price))
        .first::<Option<BigDecimal>>(&mut conn)?
        .unwrap_or(BigDecimal::from(0));
    
    // 转换为i32
    let total_sales = total_sales_bd.to_i32().unwrap_or(0);
    
    // 计算时间范围内的销售笔数
    let sales_count = crate::schema::recharge_logs::table
        .filter(crate::schema::recharge_logs::created_at.ge(start))
        .filter(crate::schema::recharge_logs::created_at.le(end))
        .count()
        .first::<i64>(&mut conn)?;
    
    // 计算平均每笔销售金额
    let avg_sales = if sales_count > 0 {
        total_sales / sales_count as i32
    } else {
        0
    };
    
    Ok(serde_json::json!(
        {
            "total_sales": total_sales,
            "sales_count": sales_count,
            "avg_sales": avg_sales,
            "start_time": start_time,
            "end_time": end_time
        }
    ))
}
