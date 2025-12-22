use bcrypt::{hash, verify, DEFAULT_COST};
use crate::errors::AppError;

type Result<T> = std::result::Result<T, AppError>;

/// 加密密码
pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?) 
}

/// 验证密码
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    Ok(verify(password, hash)?) 
}
