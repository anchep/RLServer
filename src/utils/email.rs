use regex::Regex;
use crate::errors::AppError;

type Result<T> = std::result::Result<T, AppError>;

/// 验证邮箱格式
pub fn validate_email(email: &str) -> Result<()> {
    // 简单的邮箱格式正则表达式
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    
    if email_regex.is_match(email) {
        Ok(())
    } else {
        Err(AppError::BadRequest("Invalid email format".to_string()))
    }
}

/// 生成6位数字验证码
pub fn generate_verification_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(100000..999999).to_string()
}
