use actix_web::{web, Responder, HttpResponse};
use crate::database::models::*;
use crate::services::heartbeat::*;
use crate::database::Pool;

// 上传心跳
pub async fn heartbeat_handler(
    pool: web::Data<Pool>,
    req: web::Json<HeartbeatRequest>,
) -> impl Responder {
    match update_heartbeat(
        &pool, 
        &req.session_token, 
        &req.hardware_code, 
        &req.software_version
    ).await {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({ "message": "Heartbeat updated successfully" }))
        }
        Err(err) => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": err.to_string() }))
        }
    }
}
