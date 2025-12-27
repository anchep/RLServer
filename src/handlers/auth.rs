use actix_web::{web, Responder, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::database::models::*;
use crate::services::auth::*;
use crate::database::Pool;
use crate::config::Config;
use crate::errors::AppError;

#[derive(Debug, Serialize)]
struct RegisterResponse {
    message: String,
    activation_token: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    message: String,
    token: String,
    vip_level: i32,
    vip_expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

// 用户注册
pub async fn register_handler(
    pool: web::Data<Pool>,
    config: web::Data<Config>,
    req: web::Json<RegisterRequest>,
    req_addr: actix_web::HttpRequest,
) -> impl Responder {
    // 验证请求参数
    if let Err(err) = req.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }));
    }
    
    match register_user(&pool, req.into_inner(), &config).await {
        Ok((user, activation_token)) => {
            HttpResponse::Ok().json(RegisterResponse {
                message: "Registration successful. Please check your email for verification code.".to_string(),
                activation_token,
            })
        }
        Err(err) => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}

// 用户登录
pub async fn login_handler(
    pool: web::Data<Pool>,
    config: web::Data<Config>,
    req: web::Json<LoginRequest>,
    req_addr: actix_web:: HttpRequest,
) -> impl Responder {
    // 验证请求参数
    if let Err(err) = req.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }));
    }
    
    // 获取客户端IP
    let conn_info = req_addr.connection_info();
    let ip = conn_info.realip_remote_addr().unwrap_or("0.0.0.0");
    
    match login_user(&pool, req.into_inner(), ip, &config).await {
        Ok((user, token)) => {
            HttpResponse::Ok().json(LoginResponse {
                message: "Login successful".to_string(),
                token,
                vip_level: user.vip_level,
                vip_expires_at: user.vip_expires_at,
            })
        }
        Err(err) => {
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}

// 用户登出
pub async fn logout_handler(
    pool: web::Data<Pool>,
    logout_req: web::Json<LogoutRequest>,
) -> impl Responder {
    // 验证请求参数
    if let Err(err) = logout_req.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }));
    }
    
    // 从请求体获取token
    let session_token = &logout_req.session_token;
    
    match logout_user(&pool, session_token).await {
        Ok(_) => {
            return HttpResponse::Ok().json(serde_json::json!({ "message": "Logout successful" }));
        }
        Err(AppError::Unauthorized(msg)) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({ "error": msg }));
        }
        Err(err) => {
            return HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }));
        }
    }
}

// 刷新令牌请求DTO
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    refresh_token: String,
}

// 刷新访问令牌
pub async fn refresh_token_handler(
    pool: web::Data<Pool>,
    config: web::Data<Config>,
    req: web::Json<RefreshTokenRequest>,
) -> impl Responder {
    match refresh_access_token(&pool, &req.refresh_token, &config).await {
        Ok((user, token)) => {
            HttpResponse::Ok().json(LoginResponse {
                message: "Token refreshed successfully".to_string(),
                token,
                vip_level: user.vip_level,
                vip_expires_at: user.vip_expires_at,
            })
        }
        Err(err) => {
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}

// 密码重置请求
pub async fn reset_password_handler(
    pool: web::Data<Pool>,
    config: web::Data<Config>,
    req: web::Json<ResetPasswordRequest>,
) -> impl Responder {
    // 验证请求参数
    if let Err(err) = req.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }));
    }
    
    match request_password_reset(&pool, req.into_inner(), &config).await {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({ "message": "Password reset email sent successfully" }))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}

// 验证密码重置令牌并更新密码
pub async fn verify_reset_password_handler(
    pool: web::Data<Pool>,
    config: web::Data<Config>,
    req: web::Json<VerifyResetPasswordRequest>,
) -> impl Responder {
    // 验证请求参数
    if let Err(err) = req.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }));
    }
    
    match verify_reset_password(&pool, req.into_inner(), &config).await {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({ "message": "Password reset successful" }))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}
