use diesel::prelude::*;
use chrono::{Utc, DateTime, Duration};
use crate::database::{models::*, Pool};
use crate::utils::{crypto::*, jwt::*};
use crate::schema::*;
use crate::config::Config;
use crate::errors::AppError;

type Result<T> = std::result::Result<T, AppError>;

pub async fn register_user(pool: &Pool, req: RegisterRequest, config: &Config) -> Result<(User, String)> {
    let mut conn = pool.get()?;
    
    // 检查黑名单
    let is_blacklisted = blacklist::table
        .filter(
            blacklist::username.eq(&req.username)
            .or(blacklist::hardware_code.eq(&req.hardware_code))
            .or(blacklist::ip_address.eq(&req.ip_address))
        )
        .first::<crate::database::models::Blacklist>(&mut conn)
        .optional()?;
    
    if is_blacklisted.is_some() {
        return Err(AppError::BadRequest("Device exception, cannot communicate".to_string()));
    }
    
    // 检查用户名是否已存在
    let existing_user = users::table
        .filter(users::username.eq(&req.username))
        .first::<User>(&mut conn)
        .optional()?;
    
    if existing_user.is_some() {
        return Err(AppError::BadRequest("Username already exists".to_string()));
    }
    
    // 检查邮箱是否已存在
    let existing_email = users::table
        .filter(users::email.eq(&req.email))
        .first::<User>(&mut conn)
        .optional()?;
    
    if existing_email.is_some() {
        return Err(AppError::BadRequest("Email already exists".to_string()));
    }
    
    // 验证邮箱格式
    crate::utils::email::validate_email(&req.email)?;
    
    // 加密密码
    let password_hash = hash_password(&req.password, config)?;
    
    // 创建用户
    let new_user = diesel::insert_into(users::table)
        .values((
            users::username.eq(&req.username),
            users::password_hash.eq(&password_hash),
            users::email.eq(&req.email),
            users::email_verified.eq(false),
            users::vip_level.eq(0),
            users::last_login_hardware.eq(&req.hardware_code),
            users::last_login_ip.eq(&req.ip_address),
            users::last_login_at.eq(Utc::now()),
            users::created_at.eq(Utc::now()),
            users::updated_at.eq(Utc::now()),
        ))
        .get_result::<User>(&mut conn)?;
    
    // 发送邮箱验证码
    let activation_token = crate::services::email::send_verification_email(pool, &new_user, config).await?;
    
    // 返回用户信息和激活token
    Ok((new_user, activation_token))
}

pub async fn login_user(pool: &Pool, req: LoginRequest, ip: &str, config: &Config) -> Result<(User, String)> {
    let mut conn = pool.get()?;
    
    // 查找用户
    let user = users::table
        .filter(users::username.eq(&req.username))
        .first::<User>(&mut conn)?;
    
    // 验证密码
    let is_password_valid = verify_password(&req.password, &user.password_hash)?;
    if !is_password_valid {
        return Err(AppError::Unauthorized("Invalid password".to_string()));
    }
    
    // 无论邮箱是否已验证，都正常生成访问令牌
    let access_token = generate_access_token(user.id, &user.username, config)?;
    let refresh_token = generate_refresh_token(user.id, &user.username, config)?;
    
    // 记录登录日志
    diesel::insert_into(login_logs::table)
        .values((
            login_logs::user_id.eq(user.id),
            login_logs::login_time.eq(Utc::now()),
            login_logs::hardware_code.eq(&req.hardware_code),
            login_logs::software_version.eq(&req.software_version),
            login_logs::ip_address.eq(ip),
            login_logs::status.eq("success"),
            login_logs::created_at.eq(Utc::now()),
        ))
        .execute(&mut conn)?;
    
    // 踢掉旧的在线会话
    diesel::delete(online_users::table)
        .filter(online_users::user_id.eq(user.id))
        .execute(&mut conn)?;
    
    // 记录新的在线会话
    diesel::insert_into(online_users::table)
        .values((
            online_users::user_id.eq(user.id),
            online_users::session_token.eq(&access_token),
            online_users::login_time.eq(Utc::now()),
            online_users::hardware_code.eq(&req.hardware_code),
            online_users::software_version.eq(&req.software_version),
            online_users::ip_address.eq(ip),
            online_users::last_activity_at.eq(Utc::now()),
            online_users::status_interval.eq(10), // 默认10分钟上传一次状态
            online_users::created_at.eq(Utc::now()),
        ))
        .execute(&mut conn)?;
    
    // 更新用户最后登录信息
    let updated_user = diesel::update(users::table.find(user.id))
        .set((
            users::last_login_at.eq(Utc::now()),
            users::last_login_hardware.eq(&req.hardware_code),
            users::last_login_version.eq(&req.software_version),
            users::last_login_ip.eq(ip),
            users::updated_at.eq(Utc::now()),
        ))
        .get_result::<User>(&mut conn)?;
    
    Ok((updated_user, access_token))
}

pub async fn logout_user(pool: &Pool, session_token: &str) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 首先获取在线用户信息，以便获取用户ID
    let online_user = online_users::table
        .filter(online_users::session_token.eq(session_token))
        .first::<OnlineUser>(&mut conn)
        .optional()?;
    
    if online_user.is_none() {
        return Err(AppError::Unauthorized("Logout token error".to_string()));
    }
    
    let online_user = online_user.unwrap();
    
    // 删除在线会话
    diesel::delete(online_users::table)
        .filter(online_users::id.eq(online_user.id))
        .execute(&mut conn)?;
    
    // 更新用户的最后退出时间
    diesel::update(users::table.find(online_user.user_id))
        .set((
            users::last_logout_at.eq(Utc::now()),
            users::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)?;
    
    Ok(())
}

pub async fn get_user_by_id(pool: &Pool, user_id: i32) -> Result<User> {
    let mut conn = pool.get()?;
    
    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)?;
    
    Ok(user)
}

pub async fn get_online_user_by_token(pool: &Pool, session_token: &str) -> Result<OnlineUser> {
    let mut conn = pool.get()?;
    
    let online_user = online_users::table
        .filter(online_users::session_token.eq(session_token))
        .first::<OnlineUser>(&mut conn)?;
    
    Ok(online_user)
}

/// 刷新访问令牌
pub async fn refresh_access_token(pool: &Pool, refresh_token: &str, config: &Config) -> Result<(User, String)> {
    let mut conn = pool.get()?;
    
    // 验证刷新令牌
    let claims = verify_refresh_token(refresh_token, config)?;
    let user_id = claims.sub.parse::<i32>()?;
    
    // 查找用户
    let user = users::table
        .find(user_id)
        .first::<User>(&mut conn)?;
    
    // 查找在线用户记录
    let online_user = online_users::table
        .filter(online_users::user_id.eq(user_id))
        .first::<OnlineUser>(&mut conn)?;
    
    // 生成新的访问令牌
    let new_access_token = generate_access_token(user.id, &user.username, config)?;
    
    // 更新在线用户记录中的访问令牌
    diesel::update(online_users::table.find(online_user.id))
        .set((
            online_users::session_token.eq(&new_access_token),
        ))
        .execute(&mut conn)?;
    
    Ok((user, new_access_token))
}

/// 处理密码重置请求
pub async fn request_password_reset(pool: &Pool, req: ResetPasswordRequest, config: &Config) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 查找用户
    let user = users::table
        .filter(users::email.eq(&req.email))
        .first::<User>(&mut conn)?;

    // 生成验证码
    let code = crate::utils::email::generate_verification_code();
    let token = crate::utils::jwt::generate_reset_token(user.id, &user.email, config)?;
    let expires_at = Utc::now() + Duration::minutes(30); // 验证码30分钟内有效
    
    // 检查是否已经存在未使用的验证码记录
    let existing_code = verification_codes::table
        .filter(verification_codes::user_id.eq(user.id))
        .filter(verification_codes::used.eq(false))
        .first::<crate::database::verification_code::VerificationCode>(&mut conn)
        .optional()?;
    
    match existing_code {
        Some(vc) => {
            // 更新现有记录
            diesel::update(verification_codes::table.find(vc.id))
                .set((
                    verification_codes::code.eq(&code),
                    verification_codes::token.eq(&token),
                    verification_codes::expires_at.eq(expires_at),
                    verification_codes::used.eq(false),
                ))
                .execute(&mut conn)?;
        },
        None => {
            // 创建新记录
            diesel::insert_into(verification_codes::table)
                .values((
                    verification_codes::user_id.eq(user.id),
                    verification_codes::email.eq(&user.email),
                    verification_codes::code.eq(&code),
                    verification_codes::token.eq(&token),
                    verification_codes::expires_at.eq(expires_at),
                    verification_codes::used.eq(false),
                    verification_codes::created_at.eq(Utc::now()),
                ))
                .execute(&mut conn)?;
        }
    }
    
    // 发送密码重置邮件
    crate::services::email::send_password_reset_email(pool, &user, &code, config).await?;
    
    Ok(())
}

/// 验证密码重置验证码并更新密码
pub async fn verify_reset_password(pool: &Pool, req: VerifyResetPasswordRequest, config: &Config) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 查找用户
    let user = users::table
        .filter(users::username.eq(&req.username))
        .filter(users::email.eq(&req.email))
        .first::<User>(&mut conn)?;
    
    // 查找验证码记录
    let verification_code = verification_codes::table
        .filter(verification_codes::user_id.eq(user.id))
        .filter(verification_codes::code.eq(&req.code))
        .first::<crate::database::verification_code::VerificationCode>(&mut conn)
        .optional()?;
    
    // 验证验证码是否存在
    let verification_code = match verification_code {
        Some(vc) => vc,
        None => {
            return Err(AppError::BadRequest("Invalid verification code".to_string()));
        }
    };
    
    // 验证验证码
    if verification_code.used {
        return Err(AppError::BadRequest("Verification code has already been used".to_string()));
    }
    
    if verification_code.expires_at <= Utc::now() {
        return Err(AppError::BadRequest("Verification code has expired".to_string()));
    }
    
    // 先验证密码强度（在标记验证码为已使用之前）
    let password_hash = hash_password(&req.new_password, config)?;
    
    // 更新验证码为已使用
    diesel::update(verification_codes::table.find(verification_code.id))
        .set((
            verification_codes::used.eq(true),
        ))
        .execute(&mut conn)?;
    
    // 更新用户密码
    diesel::update(users::table.find(user.id))
        .set((
            users::password_hash.eq(&password_hash),
            users::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)?;
    
    Ok(())
}
