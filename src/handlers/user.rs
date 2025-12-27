use actix_web::{web, Responder, HttpResponse, HttpMessage};
use serde::{Deserialize, Serialize};
use crate::database::models::*;
use crate::services::user::*;
use crate::database::Pool;

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
        Ok(user) => HttpResponse::Ok().json(user),
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
