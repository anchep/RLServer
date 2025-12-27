use bcrypt::{hash, verify, DEFAULT_COST};
use regex::Regex;
use crate::errors::AppError;

type Result<T> = std::result::Result<T, AppError>;

/// 检查密码复杂度
pub fn check_password_strength(password: &str, config: &crate::config::Config) -> Result<()> {
    // 密码长度检查
    if password.len() < config.password_min_length {
        return Err(AppError::BadRequest(format!("Password must be at least {} characters long", config.password_min_length)));
    }
    
    // 包含至少一个大写字母
    if config.password_require_uppercase {
        let has_uppercase = Regex::new(r"[A-Z]")?;
        if !has_uppercase.is_match(password) {
            return Err(AppError::BadRequest("Password must contain at least one uppercase letter".to_string()));
        }
    }
    
    // 包含至少一个小写字母
    if config.password_require_lowercase {
        let has_lowercase = Regex::new(r"[a-z]")?;
        if !has_lowercase.is_match(password) {
            return Err(AppError::BadRequest("Password must contain at least one lowercase letter".to_string()));
        }
    }
    
    // 包含至少一个数字
    if config.password_require_digit {
        let has_digit = Regex::new(r"\d")?;
        if !has_digit.is_match(password) {
            return Err(AppError::BadRequest("Password must contain at least one digit".to_string()));
        }
    }
    
    // 包含至少一个特殊字符
    if config.password_require_special {
        let has_special = Regex::new(r#"[!@#$%^&*(),.?":{}|<>]"#)?;
        if !has_special.is_match(password) {
            return Err(AppError::BadRequest("Password must contain at least one special character".to_string()));
        }
    }
    
    Ok(())
}

/// 加密密码
pub fn hash_password(password: &str, config: &crate::config::Config) -> Result<String> {
    // 先检查密码强度
    check_password_strength(password, config)?;
    Ok(hash(password, DEFAULT_COST)?) 
}

/// 验证密码
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    Ok(verify(password, hash)?) 
}
