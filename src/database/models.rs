use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

// 用户表
#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(treat_none_as_null = true)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub email_verified: bool,
    pub vip_level: i32,
    pub vip_expires_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub last_login_hardware: Option<String>,
    pub last_login_version: Option<String>,
    pub last_login_ip: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 软件表
#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::software)]
#[diesel(treat_none_as_null = true)]
pub struct Software {
    pub id: i32,
    pub name: String,
    pub required_vip_level: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 充值卡密表
#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::recharge_cards)]
#[diesel(treat_none_as_null = true)]
pub struct RechargeCard {
    pub id: i32,
    pub card_code: String,
    pub amount: i32,
    pub vip_level: i32,
    pub duration_days: i32,
    pub is_used: bool,
    pub used_at: Option<DateTime<Utc>>,
    pub used_by: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// 充值日志表
#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::recharge_logs)]
#[diesel(treat_none_as_null = true)]
pub struct RechargeLog {
    pub id: i32,
    pub user_id: i32,
    pub card_code: String,
    pub vip_level: i32,
    pub duration_days: i32,
    pub recharge_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// 登录日志表
#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::login_logs)]
#[diesel(treat_none_as_null = true)]
pub struct LoginLog {
    pub id: i32,
    pub user_id: i32,
    pub login_time: DateTime<Utc>,
    pub hardware_code: String,
    pub software_version: String,
    pub ip_address: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

// 在线用户表
#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::online_users)]
#[diesel(treat_none_as_null = true)]
pub struct OnlineUser {
    pub id: i32,
    pub user_id: i32,
    pub session_token: String,
    pub login_time: DateTime<Utc>,
    pub hardware_code: String,
    pub software_version: String,
    pub ip_address: String,
    pub last_activity_at: DateTime<Utc>,
    pub status_interval: i32,
    pub created_at: DateTime<Utc>,
}

// 注册请求DTO
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20, message = "Username must be between 3 and 20 characters"))]
    pub username: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

// 登录请求DTO
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 20, message = "Username must be between 3 and 20 characters"))]
    pub username: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    #[validate(length(min = 1, max = 100, message = "Hardware code must be between 1 and 100 characters"))]
    pub hardware_code: String,
    
    #[validate(length(min = 1, max = 50, message = "Software version must be between 1 and 50 characters"))]
    pub software_version: String,
}

// 密码重置请求DTO
#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

// 验证密码重置令牌并更新密码DTO
#[derive(Debug, Deserialize, Validate)]
pub struct VerifyResetPasswordRequest {
    #[validate(length(min = 1, message = "Token must not be empty"))]
    pub token: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,
}

// 充值请求DTO
#[derive(Debug, Deserialize)]
pub struct RechargeRequest {
    pub card_code: String,
}

// 心跳请求DTO
#[derive(Debug, Deserialize)]
pub struct HeartbeatRequest {
    pub session_token: String,
    pub hardware_code: String,
    pub software_version: String,
}

// 退出登录请求DTO
#[derive(Debug, Deserialize, Validate)]
pub struct LogoutRequest {
    #[validate(length(min = 1, max = 100, message = "Hardware code must be between 1 and 100 characters"))]
    pub hardware_code: String,
    
    #[validate(length(min = 1, max = 50, message = "Software version must be between 1 and 50 characters"))]
    pub software_version: String,
}

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub username: String,
    pub exp: i64, // 过期时间
    pub token_type: String, // 令牌类型：access或refresh
}
