use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::database::models::Claims;
use crate::config::Config;
use crate::errors::AppError;

type Result<T> = std::result::Result<T, AppError>;

/// 生成访问令牌
pub fn generate_access_token(user_id: i32, username: &str, config: &Config) -> Result<String> {
    let expiration = Utc::now() + Duration::hours(1); // 访问令牌有效期1小时
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration.timestamp(),
        token_type: "access".to_string(),
    };
    
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_bytes()))?;
    Ok(token)
}

/// 生成刷新令牌
pub fn generate_refresh_token(user_id: i32, username: &str, config: &Config) -> Result<String> {
    let expiration = Utc::now() + Duration::days(30); // 刷新令牌有效期30天
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration.timestamp(),
        token_type: "refresh".to_string(),
    };
    
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_bytes()))?;
    Ok(token)
}

/// 验证访问令牌
pub fn verify_access_token(token: &str, config: &Config) -> Result<Claims> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(config.jwt_secret.as_bytes()), &validation)?;
    
    if decoded.claims.token_type != "access" {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }
    
    Ok(decoded.claims)
}

/// 验证刷新令牌
pub fn verify_refresh_token(token: &str, config: &Config) -> Result<Claims> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(config.jwt_secret.as_bytes()), &validation)?;
    
    if decoded.claims.token_type != "refresh" {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }
    
    Ok(decoded.claims)
}

/// 生成密码重置令牌
pub fn generate_reset_token(user_id: i32, email: &str, config: &Config) -> Result<String> {
    let expiration = Utc::now() + Duration::hours(1); // 重置令牌有效期1小时
    let claims = Claims {
        sub: user_id.to_string(),
        username: email.to_string(), // 使用邮箱作为username字段
        exp: expiration.timestamp(),
        token_type: "reset".to_string(),
    };
    
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_bytes()))?;
    Ok(token)
}

/// 生成激活令牌
pub fn generate_activation_token(user_id: i32, email: &str, config: &Config) -> Result<String> {
    let expiration = Utc::now() + Duration::hours(24); // 激活令牌有效期24小时
    let claims = Claims {
        sub: user_id.to_string(),
        username: email.to_string(), // 使用邮箱作为username字段
        exp: expiration.timestamp(),
        token_type: "activation".to_string(),
    };
    
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_bytes()))?;
    Ok(token)
}

/// 验证激活令牌
pub fn verify_activation_token(token: &str, config: &Config) -> Result<Claims> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(config.jwt_secret.as_bytes()), &validation)?;
    
    if decoded.claims.token_type != "activation" {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }
    
    Ok(decoded.claims)
}
