use actix_web::{web, App};
use crate::admin::handlers::{auth, card, software, user, stats};
use crate::admin::middleware::auth::AdminAuth;

// 配置后台管理路由
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    // 后台管理基础路径
    cfg.service(
        web::scope("/admin")
            // 根路径重定向到仪表盘
            .service(web::resource("/").route(web::get().to(auth::dashboard_redirect)))
            // 公开路由
            .service(web::resource("/login").route(web::get().to(auth::login_get)).route(web::post().to(auth::login_post)))
            .service(web::resource("/register").route(web::get().to(auth::register_get)).route(web::post().to(auth::register_post)))
            .service(web::resource("/logout").route(web::get().to(auth::logout_get)))
            .service(web::resource("/forgot-password").route(web::get().to(auth::forgot_password_get)).route(web::post().to(auth::forgot_password_post)))
            .service(web::resource("/reset-password/{token}").route(web::get().to(auth::reset_password_get)).route(web::post().to(auth::reset_password_post)))
            
            // 需要认证的路由
            .service(
                web::scope("/dashboard")
                    .wrap(AdminAuth)
                    
                    // 仪表板首页
                    .service(web::resource("/").route(web::get().to(auth::dashboard_get)))
                    
                    // 个人中心路由
                    .service(web::resource("/profile").route(web::get().to(auth::profile_get)))
                    .service(web::resource("/profile/").route(web::get().to(auth::profile_get)))
                    // 修改密码路由
                    .service(web::resource("/change-password").route(web::get().to(auth::change_password_get)).route(web::post().to(auth::change_password_post)))
                    .service(web::resource("/change-password/").route(web::get().to(auth::change_password_get)).route(web::post().to(auth::change_password_post)))
                    // 修改邮箱路由
                    .service(web::resource("/change-email").route(web::post().to(auth::change_email_post)))
                    .service(web::resource("/change-email/").route(web::post().to(auth::change_email_post)))
                    // 操作日志路由
                    .service(web::resource("/logs").route(web::get().to(auth::logs_get)))
                    .service(web::resource("/logs/").route(web::get().to(auth::logs_get)))
                    
                    // 软件管理
                    .service(web::resource("/software").route(web::get().to(software::list_get)))
                    .service(web::resource("/software/").route(web::get().to(software::list_get)))
                    .service(web::resource("/software/add").route(web::get().to(software::add_get)).route(web::post().to(software::add_post)))
                    .service(web::resource("/software/add/").route(web::get().to(software::add_get)).route(web::post().to(software::add_post)))
                    .service(web::resource("/software/detail/{id}").route(web::get().to(software::detail_get)))
                    .service(web::resource("/software/detail/{id}/").route(web::get().to(software::detail_get)))
                    .service(web::resource("/software/edit/{id}").route(web::get().to(software::edit_get)).route(web::post().to(software::edit_post)))
                    .service(web::resource("/software/edit/{id}/").route(web::get().to(software::edit_get)).route(web::post().to(software::edit_post)))
                    .service(web::resource("/software/delete/{id}").route(web::post().to(software::delete_post)))
                    .service(web::resource("/software/delete/{id}/").route(web::post().to(software::delete_post)))
                    .service(web::resource("/software/toggle/{id}").route(web::post().to(software::toggle_post)))
                    .service(web::resource("/software/toggle/{id}/").route(web::post().to(software::toggle_post)))
                    
                    // 卡密管理
                    .service(web::resource("/cards").route(web::get().to(card::list_get)))
                    .service(web::resource("/cards/").route(web::get().to(card::list_get)))
                    .service(web::resource("/cards/generate").route(web::get().to(card::generate_get)).route(web::post().to(card::generate_post)))
                    .service(web::resource("/cards/generate/").route(web::get().to(card::generate_get)).route(web::post().to(card::generate_post)))
                    .service(web::resource("/cards/history").route(web::get().to(card::history_get)))
                    .service(web::resource("/cards/history/").route(web::get().to(card::history_get)))
                    .service(web::resource("/cards/detail/{id}").route(web::get().to(card::detail_get)))
                    .service(web::resource("/cards/detail/{id}/").route(web::get().to(card::detail_get)))
                    .service(web::resource("/cards/update-price/{id}").route(web::post().to(card::update_price_post)))
                    .service(web::resource("/cards/update-price/{id}/").route(web::post().to(card::update_price_post)))
                    .service(web::resource("/cards/delete/{id}").route(web::post().to(card::delete_post)))
                    .service(web::resource("/cards/delete/{id}/").route(web::post().to(card::delete_post)))
                    .service(web::resource("/cards/batch-delete").route(web::post().to(card::batch_delete_post)))
                    .service(web::resource("/cards/batch-delete/").route(web::post().to(card::batch_delete_post)))
                    
                    // 用户管理
                    .service(web::resource("/users").route(web::get().to(user::user_list)))
                    .service(web::resource("/users/").route(web::get().to(user::user_list)))
                    .service(web::resource("/users/add").route(web::get().to(user::user_add_get)).route(web::post().to(user::user_add_post)))
                    .service(web::resource("/users/add/").route(web::get().to(user::user_add_get)).route(web::post().to(user::user_add_post)))
                    .service(web::resource("/users/detail/{id}").route(web::get().to(user::user_detail)))                   .service(web::resource("/users/detail/{id}/").route(web::get().to(user::user_detail)))                   .service(web::resource("/users/edit/{id}").route(web::get().to(user::user_edit)))                   .service(web::resource("/users/edit/{id}/").route(web::get().to(user::user_edit)))                   .service(web::resource("/users/edit-vip/{id}").route(web::get().to(user::user_edit_vip)))                   .service(web::resource("/users/edit-vip/{id}/").route(web::get().to(user::user_edit_vip)))                   .service(web::resource("/api/users/detail/{id}").route(web::get().to(user::user_detail_api)))                   .service(web::resource("/api/users/detail/{id}/").route(web::get().to(user::user_detail_api)))
                    .service(web::resource("/users/save-vip").route(web::post().to(user::user_save_vip)))
                    .service(web::resource("/users/save-vip/").route(web::post().to(user::user_save_vip)))
                    .service(web::resource("/users/toggle-status/{id}/{status}").route(web::post().to(user::user_toggle_status)))
                    .service(web::resource("/users/toggle-status/{id}/{status}/").route(web::post().to(user::user_toggle_status)))
                    .service(web::resource("/users/login-history/{id}").route(web::get().to(user::user_login_history)))
                    .service(web::resource("/users/login-history/{id}/").route(web::get().to(user::user_login_history)))
                    .service(web::resource("/users/online").route(web::get().to(user::online_users)))
                    .service(web::resource("/users/online/").route(web::get().to(user::online_users)))
                    .service(web::resource("/users/blacklist").route(web::get().to(user::blacklist_list)))
                    .service(web::resource("/users/blacklist/").route(web::get().to(user::blacklist_list)))
                    .service(web::resource("/users/blacklist/add").route(web::get().to(user::blacklist_add_get)).route(web::post().to(user::blacklist_add_post)))
                    .service(web::resource("/users/blacklist/add/").route(web::get().to(user::blacklist_add_get)).route(web::post().to(user::blacklist_add_post)))
                    .service(web::resource("/users/blacklist/remove/{id}").route(web::post().to(user::blacklist_remove)))
                    .service(web::resource("/users/blacklist/remove/{id}/").route(web::post().to(user::blacklist_remove)))
                    
                    // 统计分析
                    .service(web::resource("/stats/overview").route(web::get().to(stats::overview_get)))
                    .service(web::resource("/stats/overview/").route(web::get().to(stats::overview_get)))
                    .service(web::resource("/stats/sales").route(web::get().to(stats::sales_get)))
                    .service(web::resource("/stats/sales/").route(web::get().to(stats::sales_get)))
                    .service(web::resource("/stats/users").route(web::get().to(stats::users_get)))
                    .service(web::resource("/stats/users/").route(web::get().to(stats::users_get)))
                    .service(web::resource("/stats/cards").route(web::get().to(stats::cards_get)))
                    .service(web::resource("/stats/cards/").route(web::get().to(stats::cards_get)))
            )
    );
}