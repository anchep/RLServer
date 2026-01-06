use diesel::prelude::*;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use crate::schema::users;
use crate::database::models::User;
use crate::errors::ServiceError;
use chrono::{DateTime, Utc};
use crate::utils::crypto::hash_password;
use crate::config::Config;

// 获取用户列表
pub fn get_users(
    pool: &Pool<ConnectionManager<PgConnection>>,
    username: Option<&str>,
    email: Option<&str>,
    status: Option<bool>,
    vip_level: Option<i32>,
    page: i32,
    page_size: i32,
) -> Result<(Vec<User>, i64), ServiceError> {
    let mut conn = pool.get()?;
    
    let mut query = users::table
        .select(User::as_select())
        .order_by(users::created_at.desc())
        .into_boxed();
    
    // 创建一个用于计算总数的查询
    let mut count_query = users::table
        .count()
        .into_boxed();
    
    // 添加筛选条件到计数查询
    if let Some(username_val) = username {
        query = query.filter(users::username.like(format!("%{username_val}%")));
        count_query = count_query.filter(users::username.like(format!("%{username_val}%")));
    }
    
    if let Some(email_val) = email {
        query = query.filter(users::email.like(format!("%{email_val}%")));
        count_query = count_query.filter(users::email.like(format!("%{email_val}%")));
    }
    
    if let Some(status_val) = status {
        query = query.filter(users::status.eq(status_val));
        count_query = count_query.filter(users::status.eq(status_val));
    }
    
    if let Some(vip_level_val) = vip_level {
        query = query.filter(users::vip_level.eq(vip_level_val));
        count_query = count_query.filter(users::vip_level.eq(vip_level_val));
    }
    
    // 获取总条数
    let total = count_query.first::<i64>(&mut conn)?;
    
    // 分页
    let offset = (page - 1) * page_size;
    let user_list = query
        .offset(offset.into())
        .limit(page_size.into())
        .load::<User>(&mut conn)?;
    
    Ok((user_list, total))
}

// 获取用户详情
pub fn get_user_by_id(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32,
) -> Result<User, ServiceError> {
    let mut conn = pool.get()?;
    
    let user = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .first::<User>(&mut conn)?;
    
    Ok(user)
}

// 更新用户VIP信息
pub fn update_user_vip(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32,
    vip_level: Option<i32>,
    vip_expires_at: Option<DateTime<Utc>>,
    note: Option<&str>,
) -> Result<User, ServiceError> {
    let mut conn = pool.get()?;
    
    // 检查用户是否存在
    let existing_user = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .optional()?;
    
    if existing_user.is_none() {
        return Err(ServiceError::NotFound("用户不存在".to_string()));
    }
    
    // 执行更新
    let updated_user = diesel::update(users::table.filter(users::id.eq(user_id)))
        .set((
            users::vip_level.eq(vip_level.unwrap_or(existing_user.as_ref().unwrap().vip_level)),
            users::vip_expires_at.eq(vip_expires_at),
            users::note.eq(note.unwrap_or(&existing_user.as_ref().unwrap().note)),
            users::updated_at.eq(Utc::now()),
        ))
        .get_result::<User>(&mut conn)?;

    Ok(updated_user)
}

// 更新用户状态
pub fn update_user_status(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32,
    status: bool,
) -> Result<User, ServiceError> {
    let mut conn = pool.get()?;
    
    // 检查用户是否存在
    let existing_user = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .optional()?;
    
    if existing_user.is_none() {
        return Err(ServiceError::NotFound("用户不存在".to_string()));
    }
    
    // 更新状态
    let updated_user = diesel::update(users::table.filter(users::id.eq(user_id)))
        .set((
            users::status.eq(status),
            users::updated_at.eq(Utc::now()),
        ))
        .get_result::<User>(&mut conn)?;
    
    Ok(updated_user)
}

// 获取用户统计信息
pub fn get_user_stats(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<(i64, i64, i64), ServiceError> {
    let mut conn = pool.get()?;
    
    // 总用户数
    let total = users::table
        .count()
        .first::<i64>(&mut conn)?;
    
    // 活跃用户数
    let active = users::table
        .filter(users::status.eq(true))
        .count()
        .first::<i64>(&mut conn)?;
    
    // VIP用户数
    let vip = users::table
        .filter(users::vip_level.gt(0))
        .count()
        .first::<i64>(&mut conn)?;
    
    Ok((total, active, vip))
}

// 获取用户登录记录
pub fn get_user_login_history(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    page: i32,
    page_size: i32,
) -> Result<(Vec<crate::database::models::LoginLog>, i64), ServiceError> {
    use crate::schema::login_logs;
    
    let mut conn = pool.get()?;
    
    let mut query = login_logs::table
        .filter(login_logs::user_id.eq(user_id))
        .order_by(login_logs::login_time.desc())
        .into_boxed();
    
    // 创建一个用于计算总数的查询
    let mut count_query = login_logs::table
        .filter(login_logs::user_id.eq(user_id))
        .into_boxed();
    
    // 添加日期筛选条件
    if let Some(start) = start_date {
        query = query.filter(login_logs::login_time.ge(start));
        count_query = count_query.filter(login_logs::login_time.ge(start));
    }
    
    if let Some(end) = end_date {
        query = query.filter(login_logs::login_time.le(end));
        count_query = count_query.filter(login_logs::login_time.le(end));
    }
    
    // 获取总条数
    let total = count_query
        .count()
        .first::<i64>(&mut conn)?;
    
    // 分页
    let offset = (page - 1) * page_size;
    let login_history = query
        .offset(offset.into())
        .limit(page_size.into())
        .load::<crate::database::models::LoginLog>(&mut conn)?;
    
    Ok((login_history, total))
}

// 获取在线用户列表
pub fn get_online_users(
    pool: &Pool<ConnectionManager<PgConnection>>,
    page: i32,
    page_size: i32,
) -> Result<(Vec<(crate::database::models::OnlineUser, crate::database::models::User)>, i64), ServiceError> {
    use crate::schema::{online_users, users};
    
    let mut conn = pool.get()?;
    
    // 联表查询在线用户和用户信息
    let mut query = online_users::table
        .inner_join(users::table.on(online_users::user_id.eq(users::id)))
        .order_by(online_users::login_time.desc())
        .into_boxed();
    
    // 获取总条数
    let total = online_users::table
        .count()
        .first::<i64>(&mut conn)?;
    
    // 分页
    let offset = (page - 1) * page_size;
    let online_users_list = query
        .offset(offset.into())
        .limit(page_size.into())
        .load::<(crate::database::models::OnlineUser, crate::database::models::User)>(&mut conn)?;
    
    Ok((online_users_list, total))
}

// 获取在线用户统计信息
pub fn get_online_users_stats(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<(i64, i64, f64, i64), ServiceError> {
    use crate::schema::{online_users, users};
    use diesel::dsl::count_distinct;
    
    let mut conn = pool.get()?;
    
    // 在线用户总数
    let total_online = online_users::table
        .count()
        .first::<i64>(&mut conn)?;
    
    // VIP在线用户数
    let vip_online = online_users::table
        .inner_join(users::table.on(online_users::user_id.eq(users::id)))
        .filter(users::vip_level.gt(0))
        .count()
        .first::<i64>(&mut conn)?;
    
    // 活跃设备数
    let active_devices = online_users::table
        .select(count_distinct(online_users::hardware_code))
        .first::<i64>(&mut conn)?;
    
    // 平均在线时长（分钟）
    let avg_duration = 0.0; // 简化处理，实际应该计算登录时间到当前时间的差值
    
    Ok((total_online, vip_online, avg_duration, active_devices))
}

// 获取黑名单列表
pub fn get_blacklist(
    pool: &Pool<ConnectionManager<PgConnection>>,
    page: i32,
    page_size: i32,
) -> Result<(Vec<crate::database::models::Blacklist>, i64), ServiceError> {
    use crate::schema::blacklist;
    
    let mut conn = pool.get()?;
    
    let mut query = blacklist::table
        .order_by(blacklist::created_at.desc())
        .into_boxed();
    
    // 获取总条数
    let total = blacklist::table
        .count()
        .first::<i64>(&mut conn)?;
    
    // 分页
    let offset = (page - 1) * page_size;
    let blacklist = query
        .offset(offset.into())
        .limit(page_size.into())
        .load::<crate::database::models::Blacklist>(&mut conn)?;
    
    Ok((blacklist, total))
}

// 添加到黑名单
pub fn add_to_blacklist(
    pool: &Pool<ConnectionManager<PgConnection>>,
    username: Option<&str>,
    hardware_code: Option<&str>,
    ip_address: Option<&str>,
) -> Result<crate::database::models::Blacklist, ServiceError> {
    use crate::schema::blacklist;
    
    let mut conn = pool.get()?;
    
    // 检查是否已存在相同的用户名、硬件码或IP地址
    let existing_entry = blacklist::table
        .filter(
            blacklist::username.eq(username)
                .or(blacklist::hardware_code.eq(hardware_code))
                .or(blacklist::ip_address.eq(ip_address))
        )
        .first::<crate::database::models::Blacklist>(&mut conn)
        .optional()?;
    
    if existing_entry.is_some() {
        return Err(ServiceError::DuplicateEntry("该用户名、硬件码或IP地址已在黑名单中".to_string()));
    }
    
    let new_blacklist_entry = diesel::insert_into(blacklist::table)
        .values((
            blacklist::username.eq(username),
            blacklist::hardware_code.eq(hardware_code),
            blacklist::ip_address.eq(ip_address),
            blacklist::created_at.eq(Utc::now()),
        ))
        .get_result::<crate::database::models::Blacklist>(&mut conn)?;
    
    Ok(new_blacklist_entry)
}

// 从黑名单移除
pub fn remove_from_blacklist(
    pool: &Pool<ConnectionManager<PgConnection>>,
    blacklist_id: i32,
) -> Result<usize, ServiceError> {
    use crate::schema::blacklist;
    
    let mut conn = pool.get()?;
    
    let deleted_count = diesel::delete(blacklist::table.filter(blacklist::id.eq(blacklist_id)))
        .execute(&mut conn)?;
    
    Ok(deleted_count)
}

// 创建新用户
pub fn create_user(
    pool: &Pool<ConnectionManager<PgConnection>>,
    username: &str,
    password: &str,
    email: &str,
    vip_level: i32,
    status: bool,
    config: &Config,
) -> Result<User, ServiceError> {
    let mut conn = pool.get()?;
    
    // 检查用户名是否已存在
    let existing_user = users::table
        .filter(users::username.eq(username))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .optional()?;
    
    if existing_user.is_some() {
        return Err(ServiceError::BadRequest("用户名已存在".to_string()));
    }
    
    // 检查邮箱是否已存在
    let existing_email = users::table
        .filter(users::email.eq(email))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .optional()?;
    
    if existing_email.is_some() {
        return Err(ServiceError::BadRequest("邮箱已存在".to_string()));
    }
    
    // 加密密码
    let password_hash = hash_password(password, config)?;
    
    // 创建用户
    let new_user = diesel::insert_into(users::table)
        .values((
            users::username.eq(username),
            users::password_hash.eq(&password_hash),
            users::email.eq(email),
            users::email_verified.eq(false),
            users::vip_level.eq(vip_level),
            users::created_at.eq(Utc::now()),
            users::updated_at.eq(Utc::now()),
            users::note.eq(""),
            users::status.eq(status),
        ))
        .get_result::<User>(&mut conn)?;
    
    Ok(new_user)
}

// 更新用户密码
pub fn update_user_password(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32,
    password: &str,
    config: &Config,
) -> Result<User, ServiceError> {
    let mut conn = pool.get()?;
    
    // 检查用户是否存在
    let existing_user = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .optional()?;
    
    if existing_user.is_none() {
        return Err(ServiceError::NotFound("用户不存在".to_string()));
    }
    
    // 加密密码
    let password_hash = hash_password(password, config)?;
    
    // 更新密码
    let updated_user = diesel::update(users::table.filter(users::id.eq(user_id)))
        .set((
            users::password_hash.eq(&password_hash),
            users::updated_at.eq(Utc::now()),
        ))
        .get_result::<User>(&mut conn)?;
    
    Ok(updated_user)
}