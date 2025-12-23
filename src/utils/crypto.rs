use bcrypt::{hash, verify, DEFAULT_COST};
use regex::Regex;
use crate::errors::AppError;

type Result<T> = std::result::Result<T, AppError>;

/// 检查密码复杂度
pub fn check_password_strength(password: &str) -> Result<()> {
    // 密码长度至少8位
    if password.len() < 8 {
        return Err(AppError::BadRequest("Password must be at least 8 characters long".to_string()));
    }
    
    // 包含至少一个大写字母
    let has_uppercase = Regex::new(r"[A-Z]")?;
    if !has_uppercase.is_match(password) {
        return Err(AppError::BadRequest("Password must contain at least one uppercase letter".to_string()));
    }
    
    // 包含至少一个小写字母
    let has_lowercase = Regex::new(r"[a-z]")?;
    if !has_lowercase.is_match(password) {
        return Err(AppError::BadRequest("Password must contain at least one lowercase letter".to_string()));
    }
    
    // 包含至少一个数字
    let has_digit = Regex::new(r"\d")?;
    if !has_digit.is_match(password) {
        return Err(AppError::BadRequest("Password must contain at least one digit".to_string()));
    }
    
    // 包含至少一个特殊字符
    let has_special = Regex::new(r"[!@#$%^&*(),.?\":{}|<>]")?;
    if !has_special.is_match(password) {
        return Err(AppError::BadRequest("Password must contain at least one special character".to_string()));
    }
    
    Ok(())
}

/// 加密密码
pub fn hash_password(password: &str) -> Result<String> {
    // 先检查密码强度
    check_password_strength(password)?;
    Ok(hash(password, DEFAULT_COST)?) 
}

/// 验证密码
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    Ok(verify(password, hash)?) 
}
