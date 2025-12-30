use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, web};
use actix_web::middleware::Next;
use actix_web::body::BoxBody;
use crate::utils::jwt::verify_access_token;
use crate::config::Config;

// 认证中间件
pub async fn auth_middleware(
    mut req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    // 获取配置
    let config = match req.app_data::<web::Data<Config>>() {
        Some(config) => config,
        None => {
            let response = actix_web::HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Config not found" }));
            return Ok(req.into_response(response));
        }
    };
    
    // 从请求头获取token
    let auth_header = req.headers().get(actix_web::http::header::AUTHORIZATION);
    
    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str.trim_start_matches("Bearer ");
                
                // 验证token
                match verify_access_token(token, config) {
                    Ok(claims) => {
                        // 将用户ID存储到请求扩展中
                        let user_id = claims.sub.parse::<i32>().unwrap_or(0);
                        req.extensions_mut().insert(user_id);
                        return next.call(req).await;
                    }
                    Err(_) => {
                        // Token无效
                        let response = actix_web::HttpResponse::Unauthorized()
                            .json(serde_json::json!({ "error": "Invalid token" }));
                        return Ok(req.into_response(response));
                    }
                }
            }
        }
    }
    
    // 没有提供有效的token
    let response = actix_web::HttpResponse::Unauthorized()
        .json(serde_json::json!({ "error": "Authorization token required" }));
    Ok(req.into_response(response))
}
