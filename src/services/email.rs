use log::{info, error};
use crate::errors::AppError;
use crate::utils::email::generate_verification_code;
use crate::database::{models::User, Pool, verification_code::VerificationCode};
use crate::schema::verification_codes;
use crate::config::Config;
use diesel::prelude::*;
use chrono::{Utc, Duration};
use lettre::{Message, Transport};
use lettre::transport::smtp::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;

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
    
    // 构建邮件内容 - 使用配置中的模板
    let subject = &config.email_verification_subject;
    let body = config.email_verification_template
        .replace("{code}", &code)
        .replace("{expiry}", "30 minutes")
        .replace("{username}", &user.username);
    
    // 发送真实邮件
    match send_email(&user.email, subject, body, config).await {
        Ok(_) => {
            info!("Verification code sent to {}", user.email);
            info!("SMTP Config: Host: {}, Port: {}, From: {}", config.smtp_host, config.smtp_port, config.smtp_from_email);
        },
        Err(err) => {
            error!("Failed to send verification email to {}: {}", user.email, err);
            // 继续执行，不中断流程
        }
    }
    
    Ok(())
}

/// 验证邮箱验证码
pub async fn verify_email_code(pool: &Pool, user_id: i32, code: &str) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 查找所有相关验证码（不考虑是否有效）
    let verification_codes = verification_codes::table
        .filter(verification_codes::user_id.eq(user_id))
        .filter(verification_codes::code.eq(code))
        .load::<VerificationCode>(&mut conn)?;
    
    if verification_codes.is_empty() {
        return Err(AppError::BadRequest("Invalid verification code".to_string()));
    }
    
    // 检查验证码状态
    for vc in verification_codes {
        if vc.used {
            return Err(AppError::BadRequest("Verification code has already been used".to_string()));
        }
        
        if vc.expires_at <= Utc::now() {
            return Err(AppError::BadRequest("Verification code has expired".to_string()));
        }
        
        // 找到有效的验证码，更新为已使用
        diesel::update(verification_codes::table.find(vc.id))
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
        
        return Ok(());
    }
    
    // 理论上不会执行到这里
    return Err(AppError::BadRequest("Invalid verification code".to_string()));
}

/// 发送密码重置邮件
pub async fn send_password_reset_email(pool: &Pool, user: &User, reset_token: &str, config: &Config) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 构建邮件内容 - 使用配置中的模板
    let subject = &config.password_reset_subject;
    let reset_link = format!("http://localhost:28001/api/auth/reset-password?token={}", reset_token);
    let body = config.password_reset_template
        .replace("{username}", &user.username)
        .replace("{reset_link}", &reset_link)
        .replace("{expiry}", "1 hour");
    
    // 发送真实邮件
    match send_email(&user.email, subject, body, config).await {
        Ok(_) => {
            info!("Password reset email sent to {}", user.email);
            info!("SMTP Config: Host: {}, Port: {}, From: {}", config.smtp_host, config.smtp_port, config.smtp_from_email);
        },
        Err(err) => {
            error!("Failed to send password reset email to {}: {}", user.email, err);
            // 继续执行，不中断流程
        }
    }
    
    Ok(())
}

/// 实际发送邮件的辅助函数
async fn send_email(to: &str, subject: &str, body: String, config: &Config) -> Result<()> {
    // 创建邮件
    let from_address = format!("RLServer <{}{}{}", "<", config.smtp_from_email, ">").replace("<<", "<").replace(">>", ">");
    info!("Creating email from: {}, to: {}, subject: {}", from_address, to, subject);
    
    // 使用lettre 0.11的方式构建发件人和收件人地址
    let email = Message::builder()
        .from(from_address.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body)?;
    
    // 配置SMTP传输 - 针对SMTPS端口465的明确SSL配置
    let mailer = SmtpTransport::builder_dangerous(&config.smtp_host)
        .port(config.smtp_port)
        .credentials(Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        ))
        // 对于465端口，使用SSL包装器
        .tls(lettre::transport::smtp::client::Tls::Wrapper(
            // 构建TLS参数
            lettre::transport::smtp::client::TlsParameters::builder(config.smtp_host.clone())
                // 禁用证书验证（开发环境）
                .dangerous_accept_invalid_certs(true)
                .dangerous_accept_invalid_hostnames(true)
                .build()
                .unwrap(),
        ))
        .build();
    
    // 发送邮件
    match mailer.send(&email) {
        Ok(_) => {
            info!("Email sent successfully to {}", to);
            Ok(())
        },
        Err(e) => {
            error!("Failed to send email to {}: {:?}", to, e);
            Err(AppError::InternalServerError(format!("Failed to send email: {:?}", e)))
        }
    }
}
