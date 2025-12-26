use actix_web::{ResponseError, HttpResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 标准化错误响应格式
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// 错误代码
    pub code: u16,
    /// 错误消息
    pub message: String,
    /// 可选的错误详情
    pub details: Option<String>,
}

/// 自定义错误类型
#[derive(Debug)]
pub enum AppError {
    /// 无效的请求数据
    BadRequest(String),
    /// 未授权
    Unauthorized(String),
    /// 禁止访问
    Forbidden(String),
    /// 资源未找到
    NotFound(String),
    /// 内部服务器错误
    InternalServerError(String),
    /// 数据库错误
    DatabaseError(String),
    /// JWT认证错误
    JwtError(String),
    /// 密码错误
    PasswordError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::JwtError(msg) => write!(f, "JWT Error: {}", msg),
            AppError::PasswordError(msg) => write!(f, "Password Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
            AppError::PasswordError(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            details: None,
        })
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::JwtError(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::PasswordError(err.to_string())
    }
}

impl From<r2d2::Error> for AppError {
    fn from(err: r2d2::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::BadRequest(err.to_string())
    }
}

impl From<regex::Error> for AppError {
    fn from(err: regex::Error) -> Self {
        AppError::BadRequest(err.to_string())
    }
}