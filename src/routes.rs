use actix_web::{web, App};
use crate::handlers::*;
use crate::middleware::auth::auth_middleware;

// 配置路由
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // 公开路由 - 无需认证
    cfg.service(
        web::scope("/api")
            // 认证相关路由
            .service(web::resource("/auth/register").route(web::post().to(auth::register_handler)))
            .service(web::resource("/auth/login").route(web::post().to(auth::login_handler)))
            .service(web::resource("/auth/refresh").route(web::post().to(auth::refresh_token_handler)))
            .service(web::resource("/auth/reset-password").route(web::post().to(auth::reset_password_handler)))
            .service(web::resource("/auth/reset-password/verify").route(web::post().to(auth::verify_reset_password_handler)))
            
            // 心跳路由
            .service(web::resource("/heartbeat").route(web::post().to(heartbeat::heartbeat_handler)))
            
            // 需要认证的路由
            .service(
                web::scope("/protected")
                    .wrap(actix_web::middleware::from_fn(auth_middleware))
                    
                    // 用户相关路由
                    .service(web::resource("/users/me").route(web::get().to(user::get_user_info_handler)))
                    .service(web::resource("/users/software").route(web::get().to(user::get_available_software_handler)))
                    .service(web::resource("/users/logout").route(web::post().to(auth::logout_handler)))
                    
                    // 邮箱验证相关路由
                    .service(web::resource("/email/verify").route(web::post().to(email::verify_email_handler)))
                    .service(web::resource("/email/resend").route(web::post().to(email::resend_verification_email_handler)))
                    
                    // 充值相关路由
                    .service(web::resource("/recharge").route(web::post().to(recharge::recharge_handler)))
                    .service(web::resource("/recharge/logs").route(web::get().to(recharge::get_recharge_logs_handler)))
                    
                    // 软件相关路由
                    .service(web::resource("/software").route(web::get().to(software::get_all_software_handler)))
                    .service(web::resource("/software/{software_id}/access").route(web::get().to(software::check_software_access_handler)))
            )
    );
}
