use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use diesel::prelude::*;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use crate::schema::{recharge_cards, recharge_logs};
use crate::database::models::{RechargeCard, RechargeLog};
use crate::errors::ServiceError;
use chrono::{Utc, DateTime};
use rand::Rng;

// 生成卡密
pub fn generate_recharge_cards(
    pool: &Pool<ConnectionManager<PgConnection>>,
    amount: i32,
    vip_level: i32,
    duration_days: i32,
    price: f64,
    quantity: i32,
) -> Result<Vec<RechargeCard>, ServiceError> {
    let mut conn = pool.get()?;
    let mut cards = Vec::new();
    
    for _ in 0..quantity {
        // 生成随机卡密
        let card_code = generate_unique_code(&mut conn)?;
        
        // 为每个卡密创建一个新的BigDecimal实例
        let price_bd = BigDecimal::parse_bytes(price.to_string().as_bytes(), 10).unwrap_or(BigDecimal::from(0));
        
        // 创建卡密
        let card = diesel::insert_into(recharge_cards::table)
            .values((
                recharge_cards::card_code.eq(&card_code),
                recharge_cards::amount.eq(amount),
                recharge_cards::vip_level.eq(vip_level),
                recharge_cards::duration_days.eq(duration_days),
                recharge_cards::is_used.eq(false), // 初始状态：未使用
                recharge_cards::price.eq(price_bd),
                recharge_cards::created_at.eq(Utc::now()),
            ))
            .get_result::<RechargeCard>(&mut conn)?;
        
        cards.push(card);
    }
    
    Ok(cards)
}

// 生成唯一的卡密
fn generate_unique_code(
    conn: &mut PgConnection,
) -> Result<String, ServiceError> {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    
    // 生成16位随机卡密，格式：XXXX-XXXX-XXXX-XXXX
    loop {
        let card_code = (0..4).map(|_| {
            (0..4).map(|_| chars[rng.gen_range(0..chars.len())]).collect::<String>()
        }).collect::<Vec<String>>().join("-");
        
        // 检查卡密是否已存在
        let existing = recharge_cards::table
            .filter(recharge_cards::card_code.eq(&card_code))
            .select(RechargeCard::as_select())
            .first::<RechargeCard>(conn)
            .optional()?;
        
        if existing.is_none() {
            return Ok(card_code);
        }
    }
}

// 获取卡密列表
pub fn get_recharge_cards(
    pool: &Pool<ConnectionManager<PgConnection>>,
    status: Option<bool>,
    vip_level: Option<i32>,
    page: i32,
    page_size: i32,
) -> Result<(Vec<RechargeCard>, i64), ServiceError> {
    let mut conn = pool.get()?;
    
    // 构建基本查询
    let base_query = recharge_cards::table
        .select(RechargeCard::as_select())
        .order_by(recharge_cards::created_at.desc())
        .into_boxed();
    
    // 构建计数查询
    let mut count_query = recharge_cards::table
        .into_boxed();
    
    // 添加筛选条件
    if let Some(status_val) = status {
        count_query = count_query.filter(recharge_cards::is_used.eq(status_val));
    }
    
    if let Some(vip_level_val) = vip_level {
        count_query = count_query.filter(recharge_cards::vip_level.eq(vip_level_val));
    }
    
    // 获取总条数
    let total = count_query.count().first::<i64>(&mut conn)?;
    
    // 构建数据查询
    let mut data_query = base_query;
    
    // 添加相同的筛选条件
    if let Some(status_val) = status {
        data_query = data_query.filter(recharge_cards::is_used.eq(status_val));
    }
    
    if let Some(vip_level_val) = vip_level {
        data_query = data_query.filter(recharge_cards::vip_level.eq(vip_level_val));
    }
    
    // 分页
    let offset = (page - 1) * page_size;
    let cards = data_query
        .offset(offset.into())
        .limit(page_size.into())
        .load::<RechargeCard>(&mut conn)?;
    
    Ok((cards, total))
}

// 获取卡密详情
pub fn get_recharge_card_by_id(
    pool: &Pool<ConnectionManager<PgConnection>>,
    card_id: i32,
) -> Result<RechargeCard, ServiceError> {
    let mut conn = pool.get()?;
    
    let card = recharge_cards::table
        .filter(recharge_cards::id.eq(card_id))
        .select(RechargeCard::as_select())
        .first::<RechargeCard>(&mut conn)?;
    
    Ok(card)
}

// 获取卡密详情
pub fn get_recharge_card_by_code(
    pool: &Pool<ConnectionManager<PgConnection>>,
    card_code: &str,
) -> Result<RechargeCard, ServiceError> {
    let mut conn = pool.get()?;
    
    let card = recharge_cards::table
        .filter(recharge_cards::card_code.eq(card_code))
        .select(RechargeCard::as_select())
        .first::<RechargeCard>(&mut conn)?;
    
    Ok(card)
}

// 更新卡密售价
pub fn update_card_price(
    pool: &Pool<ConnectionManager<PgConnection>>,
    card_id: i32,
    price: f64,
) -> Result<RechargeCard, ServiceError> {
    let mut conn = pool.get()?;
    let price_bd = BigDecimal::parse_bytes(price.to_string().as_bytes(), 10).unwrap_or(BigDecimal::from(0));
    
    // 检查卡密是否存在
    let existing_card = recharge_cards::table
        .filter(recharge_cards::id.eq(card_id))
        .first::<RechargeCard>(&mut conn)
        .optional()?;
    
    if existing_card.is_none() {
        return Err(ServiceError::NotFound("卡密不存在".to_string()));
    }
    
    // 更新售价
    let updated_card = diesel::update(recharge_cards::table.filter(recharge_cards::id.eq(card_id)))
        .set((
            recharge_cards::price.eq(price_bd),
        ))
        .get_result::<RechargeCard>(&mut conn)?;
    
    Ok(updated_card)
}



// 删除卡密
pub fn delete_recharge_card(
    pool: &Pool<ConnectionManager<PgConnection>>,
    card_id: i32,
) -> Result<(), ServiceError> {
    let mut conn = pool.get()?;
    
    // 检查卡密是否存在
    let existing_card = recharge_cards::table
        .filter(recharge_cards::id.eq(card_id))
        .select(RechargeCard::as_select())
        .first::<RechargeCard>(&mut conn)
        .optional()?;
    
    if existing_card.is_none() {
        return Err(ServiceError::NotFound("卡密不存在".to_string()));
    }
    
    // 删除卡密
    diesel::delete(recharge_cards::table.filter(recharge_cards::id.eq(card_id)))
        .execute(&mut conn)?;
    
    Ok(())
}

// 批量删除卡密
pub fn batch_delete_recharge_cards(
    pool: &Pool<ConnectionManager<PgConnection>>,
    card_ids: &[i32],
) -> Result<usize, ServiceError> {
    let mut conn = pool.get()?;
    
    if card_ids.is_empty() {
        return Ok(0);
    }
    
    // 批量删除卡密
    let deleted_count = diesel::delete(recharge_cards::table.filter(recharge_cards::id.eq_any(card_ids)))
        .execute(&mut conn)?;
    
    Ok(deleted_count)
}

// 获取卡密充值历史
pub fn get_recharge_history(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: Option<i32>,
    card_code: Option<&str>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    page: i32,
    page_size: i32,
) -> Result<(Vec<RechargeLog>, i64), ServiceError> {
    let mut conn = pool.get()?;
    
    // 构建基本查询
    let base_query = recharge_logs::table
        .select(RechargeLog::as_select())
        .order_by(recharge_logs::created_at.desc())
        .into_boxed();
    
    // 构建计数查询
    let mut count_query = recharge_logs::table
        .into_boxed();
    
    // 添加筛选条件
    if let Some(user_id_val) = user_id {
        count_query = count_query.filter(recharge_logs::user_id.eq(user_id_val));
    }
    
    if let Some(card_code_val) = card_code {
        count_query = count_query.filter(recharge_logs::card_code.eq(card_code_val));
    }
    
    if let Some(start_time_val) = start_time {
        count_query = count_query.filter(recharge_logs::created_at.ge(start_time_val));
    }
    
    if let Some(end_time_val) = end_time {
        count_query = count_query.filter(recharge_logs::created_at.le(end_time_val));
    }
    
    // 获取总条数
    let total = count_query.count().first::<i64>(&mut conn)?;
    
    // 构建数据查询
    let mut data_query = base_query;
    
    // 添加相同的筛选条件
    if let Some(user_id_val) = user_id {
        data_query = data_query.filter(recharge_logs::user_id.eq(user_id_val));
    }
    
    if let Some(card_code_val) = card_code {
        data_query = data_query.filter(recharge_logs::card_code.eq(card_code_val));
    }
    
    if let Some(start_time_val) = start_time {
        data_query = data_query.filter(recharge_logs::created_at.ge(start_time_val));
    }
    
    if let Some(end_time_val) = end_time {
        data_query = data_query.filter(recharge_logs::created_at.le(end_time_val));
    }
    
    // 分页
    let offset = (page - 1) * page_size;
    let histories = data_query
        .offset(offset.into())
        .limit(page_size.into())
        .load::<RechargeLog>(&mut conn)?;
    
    Ok((histories, total))
}

// 获取卡密统计信息
pub fn get_card_stats(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<(i64, i64, i64, f64), ServiceError> {
    let mut conn = pool.get()?;
    
    // 总卡密数
    let total = recharge_cards::table
        .count()
        .first::<i64>(&mut conn)?;
    
    // 未使用卡密数
    let unused = recharge_cards::table
        .filter(recharge_cards::is_used.eq(false))
        .count()
        .first::<i64>(&mut conn)?;
    
    // 已使用卡密数
    let used = recharge_cards::table
        .filter(recharge_cards::is_used.eq(true))
        .count()
        .first::<i64>(&mut conn)?;
    
    // 总销售金额
    let total_sales_bd = recharge_cards::table
        .filter(recharge_cards::is_used.eq(true)) // 已使用的卡密
        .select(diesel::dsl::sum(recharge_cards::price))
        .first::<Option<BigDecimal>>(&mut conn)?
        .unwrap_or(BigDecimal::from(0));
    
    // 转换为f64
    let total_sales = total_sales_bd.to_f64().unwrap_or(0.0);
    
    Ok((total, unused, used, total_sales))
}


