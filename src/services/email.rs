use log::info;
use crate::errors::AppError;
use crate::utils::email::generate_verification_code;
use crate::database::{models::User, Pool, verification_code::VerificationCode};
use crate::schema::verification_codes;
use crate::config::Config;
use diesel::prelude::*;
use chrono::{Utc, Duration};

type Result<T> = std::result::Result<T, AppError>;

/// 发送邮箱验证码
pub async fn send_verification_email(pool: &Pool, user: &User, config: &Config) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 生成验证码
    let code = generate_verification_code();
    let expires_at = Utc::now() + Duration::minutes(30); // 验证码30分钟内有效
    
    // 存储验证码
    diesel::insert_into(verification_codes::table)
        .values((
            verification_codes::user_id.eq(user.id),
            verification_codes::email.eq(&user.email),
            verification_codes::code.eq(&code),
            verification_codes::expires_at.eq(expires_at),
            verification_codes::used.eq(false),
            verification_codes::created_at.eq(Utc::now()),
        ))
        .execute(&mut conn)?;
    
    // 构建邮件内容
    let subject = "Email Verification Code";
    let body = format!("Your verification code is: {}\n\nThis code will expire in 30 minutes.", code);
    
    // 模拟发送邮件
    info!("Verification code sent to {}: {}", user.email, code);
    info!("Email content: Subject: {}, Body: {}", subject, body);
    info!("SMTP Config: Host: {}, Port: {}, From: {}", config.smtp_host, config.smtp_port, config.smtp_from_email);
    
    Ok(())
}

/// 验证邮箱验证码
pub async fn verify_email_code(pool: &Pool, user_id: i32, code: &str) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 查找有效的验证码
    let verification_code = verification_codes::table
        .filter(verification_codes::user_id.eq(user_id))
        .filter(verification_codes::code.eq(code))
        .filter(verification_codes::used.eq(false))
        .filter(verification_codes::expires_at.gt(Utc::now()))
        .first::<VerificationCode>(&mut conn)?;
    
    // 更新验证码为已使用
    diesel::update(verification_codes::table.find(verification_code.id))
        .set((
            verification_codes::used.eq(true),
        ))
        .execute(&mut conn)?;
    
    // 更新用户邮箱验证状态
    use crate::schema::users;
    diesel::update(users::table.find(user_id))
        .set((
            users::email_verified.eq(true),
            users::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)?;
    
    Ok(())
}

/// 发送密码重置邮件
pub async fn send_password_reset_email(pool: &Pool, user: &User, reset_token: &str, config: &Config) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 构建邮件内容
    let subject = "Password Reset Request";
    let reset_link = format!("http://localhost:28001/api/auth/reset-password?token={}", reset_token);
    let body = format!("Hello {},\n\nYou requested a password reset for your account. Please click the following link to reset your password:\n\n{}\n\nThis link will expire in 1 hour.\n\nIf you didn't request this, please ignore this email.\n\nBest regards,\nRLServer Team", user.username, reset_link);
    
    // 模拟发送邮件
    info!("Password reset email sent to {}: {}", user.email, reset_link);
    info!("Email content: Subject: {}, Body: {}", subject, body);
    info!("SMTP Config: Host: {}, Port: {}, From: {}", config.smtp_host, config.smtp_port, config.smtp_from_email);
    
    Ok(())
}
