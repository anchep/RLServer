use actix_web::{web, HttpResponse, HttpRequest, HttpMessage, Responder};
use serde::{Deserialize, Serialize};
use crate::database::Pool;
use crate::services::email::*;
use crate::errors::AppError;
use crate::config::Config;

// 邮箱验证请求DTO
#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub code: String,
}

// 验证邮箱
pub async fn verify_email_handler(
    pool: web::Data<Pool>,
    req: web::Json<VerifyEmailRequest>,
    request: HttpRequest,
) -> impl Responder {
    // 从请求扩展中获取用户ID
    let user_id = match request.extensions().get::<i32>() {
        Some(user_id) => *user_id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Unauthorized" }));
        }
    };
    
    match verify_email_code(&pool, user_id, &req.code).await {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({ "message": "Email verified successfully" }))
        },
        Err(err) => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}

// 重新发送验证码
pub async fn resend_verification_email_handler(
    pool: web::Data<Pool>,
    config: web::Data<Config>,
    request: HttpRequest,
) -> impl Responder {
    // 从请求扩展中获取用户ID
    let user_id = match request.extensions().get::<i32>() {
        Some(user_id) => *user_id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Unauthorized" }));
        }
    };
    
    // 获取用户信息
    let user = match crate::services::auth::get_user_by_id(&pool, user_id).await {
        Ok(user) => user,
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({ "error": err.to_string() }));
        }
    };
    
    match send_verification_email(&pool, &user, &config).await {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({ "message": "Verification code sent successfully" }))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}