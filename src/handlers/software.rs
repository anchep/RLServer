use actix_web::{web, Responder, HttpResponse, HttpMessage};
use crate::database::models::*;
use crate::services::software::*;
use crate::database::Pool;

// 获取所有软件列表
pub async fn get_all_software_handler(
    pool: web::Data<Pool>,
) -> impl Responder {
    match get_all_software(&pool).await {
        Ok(software_list) => HttpResponse::Ok().json(software_list),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": err.to_string() }))
    }
}

// 检查软件访问权限
pub async fn check_software_access_handler(
    pool: web::Data<Pool>,
    software_id: web::Path<i32>,
    req_ext: actix_web::HttpRequest,
) -> impl Responder {
    // 从请求扩展中获取用户ID
    let user_id = if let Some(user_id) = req_ext.extensions().get::<i32>() {
        *user_id
    } else {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Unauthorized" }));
    };
    
    match check_software_access(&pool, user_id, software_id.into_inner()).await {
        Ok(has_access) => HttpResponse::Ok().json(serde_json::json!({ "has_access": has_access })),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": err.to_string() }))
    }
}
