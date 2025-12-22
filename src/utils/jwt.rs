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

/// 生成JWT令牌（保留旧函数，用于兼容）
pub fn generate_token(user_id: i32, username: &str, config: &Config) -> Result<String> {
    generate_access_token(user_id, username, config)
}

/// 验证JWT令牌（保留旧函数，用于兼容）
pub fn verify_token(token: &str, config: &Config) -> Result<Claims> {
    verify_access_token(token, config)
}
