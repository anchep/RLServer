use actix_web::{web, Responder, HttpResponse, HttpMessage};
use serde::{Deserialize, Serialize};
use crate::database::models::*;
use crate::services::user::*;
use crate::database::Pool;

// 用户信息响应结构体，只返回特定字段
#[derive(Debug, Serialize)]
struct UserInfoResponse {
    id: i32,
    username: String,
    email: String,
    vip_level: i32,
    vip_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    last_logout_at: Option<chrono::DateTime<chrono::Utc>>,
    status: bool,
}

// 软件列表响应结构体，包含软件列表和用户VIP信息
#[derive(Debug, Serialize)]
struct SoftwareListResponse {
    vip_level: i32,
    vip_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    software_list: Vec<Software>,
}

// 获取当前用户信息
pub async fn get_user_info_handler(
    pool: web::Data<Pool>,
    req_ext: actix_web::HttpRequest,
) -> impl Responder {
    // 从请求扩展中获取用户ID
    let user_id = if let Some(user_id) = req_ext.extensions().get::<i32>() {
        *user_id
    } else {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Unauthorized" }));
    };
    
    match get_user_info(&pool, user_id).await {
        Ok(user) => {
            // 创建只包含特定字段的响应结构体
            let user_response = UserInfoResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                vip_level: user.vip_level,
                vip_expires_at: user.vip_expires_at,
                last_login_at: user.last_login_at,
                last_logout_at: user.last_logout_at,
                status: user.status,
            };
            HttpResponse::Ok().json(user_response)
        },
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": err.to_string() }))
    }
}

// 获取可用软件列表
pub async fn get_available_software_handler(
    pool: web::Data<Pool>,
    req_ext: actix_web::HttpRequest,
) -> impl Responder {
    // 从请求扩展中获取用户ID
    let user_id = if let Some(user_id) = req_ext.extensions().get::<i32>() {
        *user_id
    } else {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Unauthorized" }));
    };
    
    match get_software_with_vip_info(&pool, user_id).await {
        Ok((vip_level, vip_expires_at, software_list)) => {
            HttpResponse::Ok().json(SoftwareListResponse {
                vip_level,
                vip_expires_at,
                software_list,
            })
        },
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": err.to_string() }))
    }
}
