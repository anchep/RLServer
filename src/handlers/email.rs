use actix_web::{web, HttpResponse, HttpRequest, HttpMessage, Responder};
use serde::{Deserialize, Serialize};
use crate::database::Pool;
use crate::services::email::*;
use crate::errors::AppError;
use crate::config::Config;

// 通过激活令牌验证邮箱请求DTO
#[derive(Debug, Deserialize)]
pub struct VerifyEmailWithTokenRequest {
    pub token: String,
    pub code: String,
}

// 通过激活令牌验证邮箱
pub async fn verify_email_with_token_handler(
    pool: web::Data<Pool>,
    config: web::Data<Config>,
    req: web::Json<VerifyEmailWithTokenRequest>,
) -> impl Responder {
    // 验证激活令牌
    match crate::utils::jwt::verify_activation_token(&req.token, &config) {
        Ok(claims) => {
            // 从令牌中获取用户ID
            let user_id = match claims.sub.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    return HttpResponse::BadRequest().json(serde_json::json!({ "error": "Invalid token format" }));
                }
            };
            
            // 验证邮箱验证码
            match verify_email_code(&pool, user_id, &req.code).await {
                Ok(_) => {
                    HttpResponse::Ok().json(serde_json::json!({ "message": "Email verified successfully" }))
                },
                Err(err) => {
                    HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }))
                }
            }
        },
        Err(err) => {
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}