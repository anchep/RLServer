use actix_web::{web, Responder, HttpResponse, HttpRequest, HttpMessage};
use serde::{Deserialize, Serialize};
use crate::database::models::*;
use crate::services::recharge::*;
use crate::database::Pool;

#[derive(Debug, Serialize)]
struct RechargeResponse {
    message: String,
    user: User,
    recharge_log: RechargeLog,
}

// 卡密充值
pub async fn recharge_handler(
    pool: web::Data<Pool>,
    req: web::Json<RechargeRequest>,
    req_ext: actix_web::HttpRequest,
) -> impl Responder {
    // 从请求扩展中获取用户ID
    let user_id = if let Some(user_id) = req_ext.extensions().get::<i32>() {
        *user_id
    } else {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Unauthorized" }));
    };
    
    match recharge_with_card(&pool, user_id, &req.card_code).await {
        Ok((user, recharge_log)) => {
            HttpResponse::Ok().json(RechargeResponse {
                message: "Recharge successful".to_string(),
                user,
                recharge_log,
            })
        }
        Err(err) => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}

// 获取充值记录
pub async fn get_recharge_logs_handler(
    pool: web::Data<Pool>,
    req_ext: actix_web::HttpRequest,
) -> impl Responder {
    // 从请求扩展中获取用户ID
    let user_id = if let Some(user_id) = req_ext.extensions().get::<i32>() {
        *user_id
    } else {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Unauthorized" }));
    };
    
    match get_recharge_logs(&pool, user_id).await {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": err.to_string() }))
    }
}
