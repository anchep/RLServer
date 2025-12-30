use actix_web::{web, HttpResponse, HttpRequest, Responder, HttpMessage};
use actix_session::SessionExt;
use tera::Tera;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use chrono::{DateTime, Utc, Duration};
use log::info;

use crate::admin::services::user::{get_users, get_user_by_id, update_user_vip, update_user_status, get_online_users, get_online_users_stats};
use crate::admin::services::admin_user::log_admin_operation;
use crate::database::models::User;
use crate::errors::ServiceError;
use crate::config::Config;
use crate::utils::ip::get_client_ip;

// 用户列表页面
pub async fn user_list(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    config: web::Data<Config>,
    web::Query(query_params): web::Query<serde_json::Value>,
) -> impl Responder {
    // 从查询参数获取筛选条件
    let username = query_params.get("username").and_then(|v| v.as_str());
    let email = query_params.get("email").and_then(|v| v.as_str());
    
    // 处理status参数，支持字符串和布尔值
    let status = match query_params.get("status") {
        Some(v) if v.is_string() => {
            let s = v.as_str().unwrap_or("");
            match s {
                "true" => Some(true),
                "false" => Some(false),
                _ => None
            }
        },
        Some(v) => v.as_bool(),
        None => None
    };
    
    // 处理vip_level参数，支持字符串和数字
    let vip_level = match query_params.get("vip_level") {
        Some(v) if v.is_string() => {
            let s = v.as_str().unwrap_or("");
            if s.is_empty() {
                None
            } else {
                s.parse::<i32>().ok()
            }
        },
        Some(v) => v.as_i64().map(|v| v as i32),
        None => None
    };
    let page = query_params.get("page").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
    let page_size = query_params.get("page_size").and_then(|v| v.as_i64()).unwrap_or(10) as i32;
    
    // 调用服务获取用户列表
    let (users, total) = match get_users(&pool, username, email, status, vip_level, page, page_size) {
        Ok(result) => result,
        Err(e) => {
            // 记录错误并返回空列表
            eprintln!("获取用户列表失败: {}", e);
            (vec![], 0)
        }
    };
    
    // 计算总页数
    let total_pages = if total == 0 {
        0
    } else {
        ((total as f32) / (page_size as f32)).ceil() as i32
    };
    
    // 创建上下文
    let mut context = tera::Context::new();
    
    // 添加会话信息
    let session = req.get_session();
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("username", &username);
        context.insert("is_authenticated", &true);
    }
    
    // 添加用户列表数据到上下文
    context.insert("users", &users);
    context.insert("total", &total);
    context.insert("page", &page);
    context.insert("page_size", &page_size);
    context.insert("total_pages", &total_pages);
    context.insert("username_filter", &username.unwrap_or_default());
    context.insert("email_filter", &email.unwrap_or_default());
    context.insert("status_filter", &status);
    context.insert("vip_level_filter", &vip_level);
    
    // 渲染用户列表页面
    match data.render("admin/user/list.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            // 输出详细的错误信息到日志
            eprintln!("模板渲染错误: {:#?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {:#?}. 模板路径: admin/user/list.html.", e))
        }
    }
}

// 用户编辑页面
pub async fn user_edit(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> impl Responder {
    // 获取用户ID
    let user_id = path.into_inner();
    // 获取用户详情
    match get_user_by_id(&pool, user_id) {
        Ok(user) => {
            let mut context = tera::Context::new();
            context.insert("user", &user);
            
            // 获取当前管理员ID
            if let Some(admin_id) = req.extensions().get::<i32>() {
                context.insert("current_admin_id", admin_id);
            }
            
            let rendered = data.render("admin/user/edit.html", &context).unwrap();
            HttpResponse::Ok().body(rendered)
        },
        Err(_) => {
            HttpResponse::NotFound().body("用户不存在")
        }
    }
}

// 编辑用户VIP页面
pub async fn user_edit_vip(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> impl Responder {
    // 获取用户ID
    let user_id = path.into_inner();
    // 获取用户详情
    match get_user_by_id(&pool, user_id) {
        Ok(user) => {
            let mut context = tera::Context::new();
            context.insert("user", &user);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            let rendered = data.render("admin/user/edit_vip.html", &context).unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(rendered)
        },
        Err(_) => {
            HttpResponse::NotFound().body("用户不存在")
        }
    }
}

// 保存用户VIP信息
pub async fn user_save_vip(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    web::Form(form): web::Form<serde_json::Value>,
) -> impl Responder {
    // 从表单获取数据并正确转换类型
    let user_id = form.get("user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
        
    let vip_level = form.get("vip_level")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
        
    let expires_days = form.get("expires_days")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
        
    let note = form.get("note").and_then(|v| v.as_str());
    
    // 计算到期时间
    let vip_expires_at = if expires_days > 0 {
        Some(Utc::now() + Duration::days(expires_days as i64))
    } else {
        None
    };
    
    // 更新用户VIP信息
    match update_user_vip(&pool, user_id, Some(vip_level), vip_expires_at, note) {
        Ok(updated_user) => {
            // 记录管理员操作日志
            if let Some(admin_id) = req.extensions().get::<i32>() {
                let ip = get_client_ip(&req);
                let _ = log_admin_operation(
                    &pool,
                    *admin_id,
                    "update_user_vip",
                    Some(&format!("更新用户ID {} 的VIP信息: 等级 {}, 有效期 {} 天", user_id, vip_level, expires_days)),
                    ip
                );
            }
            
            // 返回JSON成功响应
            HttpResponse::Ok()
                .content_type("application/json")
                .json(serde_json::json!({
                    "success": true,
                    "message": "更新用户VIP信息成功"
                }))
        },
        Err(err) => {
            // 输出错误信息到日志
            eprintln!("更新用户VIP信息失败: {:#?}", err);
            
            // 返回JSON错误响应
            HttpResponse::Ok()
                .content_type("application/json")
                .json(serde_json::json!({
                    "success": false,
                    "message": "更新用户VIP信息失败"
                }))
        }
    }
}

// 切换用户状态
pub async fn user_toggle_status(
    req: HttpRequest,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<(i32, bool)>,
) -> impl Responder {
    // 获取路径参数
    let (user_id, status) = path.into_inner();
    // 更新用户状态
    match update_user_status(&pool, user_id, status) {
        Ok(_) => {
            // 记录管理员操作日志
            if let Some(admin_id) = req.extensions().get::<i32>() {
                let ip = get_client_ip(&req);
                let _ = log_admin_operation(
                    &pool,
                    *admin_id,
                    "update_user_status",
                    Some(&format!("更新用户ID {} 的状态为: {}", user_id, status)),
                    ip
                );
            }
            
            // 返回JSON响应
            HttpResponse::Ok().json(serde_json::json!({"success": true, "message": "用户状态更新成功"}))
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"success": false, "message": "更新用户状态失败"}))
        }
    }
}

// 获取用户详情API
pub async fn user_detail_api(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> impl Responder {
    // 获取用户ID
    let user_id = path.into_inner();
    match get_user_by_id(&pool, user_id) {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        },
        Err(_) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "用户不存在"}))
        }
    }
}

// 用户详情页面
pub async fn user_detail(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> impl Responder {
    // 获取用户ID
    let user_id = path.into_inner();
    // 获取用户详情
    match get_user_by_id(&pool, user_id) {
        Ok(user) => {
            let mut context = tera::Context::new();
            context.insert("user", &user);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            let rendered = data.render("admin/user/detail.html", &context).unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(rendered)
        },
        Err(_) => {
            HttpResponse::NotFound().body("用户不存在")
        }
    }
}

// 用户登录记录页面
pub async fn user_login_history(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
    web::Query(query_params): web::Query<serde_json::Value>,
) -> impl Responder {
    // 获取用户ID
    let user_id = path.into_inner();
    
    // 获取分页参数
    let page = query_params.get("page").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
    let page_size = query_params.get("page_size").and_then(|v| v.as_i64()).unwrap_or(20) as i32;
    
    // 获取日期筛选参数
    use chrono::NaiveDate;
    let start_date = query_params.get("start_date")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        .map(|nd| chrono::DateTime::<chrono::Utc>::from_utc(nd, chrono::Utc));
    
    let end_date = query_params.get("end_date")
        .and_then(|v| v.as_str())
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .map(|d| d.and_hms_opt(23, 59, 59).unwrap())
        .map(|nd| chrono::DateTime::<chrono::Utc>::from_utc(nd, chrono::Utc));
    
    // 获取用户登录记录
    match crate::admin::services::user::get_user_login_history(&pool, user_id, start_date, end_date, page, page_size) {
        Ok((login_history, total)) => {
            let mut context = tera::Context::new();
            context.insert("login_history", &login_history);
            context.insert("user_id", &user_id);
            context.insert("page", &page);
            context.insert("page_size", &page_size);
            context.insert("total", &total);
            context.insert("total_pages", &((total + page_size as i64 - 1) / page_size as i64));
            
            // 将原始日期筛选参数传回模板，以便保持筛选状态
            context.insert("start_date", &query_params.get("start_date").and_then(|v| v.as_str()).unwrap_or_default());
            context.insert("end_date", &query_params.get("end_date").and_then(|v| v.as_str()).unwrap_or_default());
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            let rendered = data.render("admin/user/login_history.html", &context).unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(rendered)
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("获取登录记录失败")
        }
    }
}

// 在线用户页面
pub async fn online_users(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    web::Query(query_params): web::Query<serde_json::Value>,
) -> impl Responder {
    // 获取分页参数
    let page = query_params.get("page").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
    let page_size = query_params.get("page_size").and_then(|v| v.as_i64()).unwrap_or(20) as i32;
    
    // 获取在线用户统计信息
    let (total_online, total_active, average_duration, max_duration) = get_online_users_stats(&pool).unwrap_or((0, 0, 0.0, 0));
    
    // 获取在线用户列表
    match get_online_users(&pool, page, page_size) {
        Ok((online_users, total)) => {
            let mut context = tera::Context::new();
            context.insert("online_users", &online_users);
            context.insert("page", &page);
            context.insert("page_size", &page_size);
            context.insert("total", &total);
            context.insert("total_pages", &((total + page_size as i64 - 1) / page_size as i64));
            context.insert("total_online", &total_online);
            context.insert("total_active", &total_active);
            context.insert("average_duration", &average_duration);
            context.insert("max_duration", &max_duration);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            let rendered = data.render("admin/user/online_users.html", &context).unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(rendered)
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("获取在线用户失败")
        }
    }
}

// 黑名单列表页面
pub async fn blacklist_list(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    web::Query(query_params): web::Query<serde_json::Value>,
) -> impl Responder {
    // 获取分页参数
    let page = query_params.get("page").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
    let page_size = query_params.get("page_size").and_then(|v| v.as_i64()).unwrap_or(20) as i32;
    
    // 获取黑名单列表
    match crate::admin::services::user::get_blacklist(&pool, page, page_size) {
        Ok((blacklist, total)) => {
            let mut context = tera::Context::new();
            context.insert("blacklist", &blacklist);
            context.insert("page", &page);
            context.insert("page_size", &page_size);
            context.insert("total", &total);
            context.insert("total_pages", &((total + page_size as i64 - 1) / page_size as i64));
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            let rendered = data.render("admin/user/blacklist.html", &context).unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(rendered)
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("获取黑名单失败")
        }
    }
}

// 添加黑名单页面
pub async fn blacklist_add_get(
    req: HttpRequest,
    data: web::Data<Tera>,
) -> impl Responder {
    let mut context = tera::Context::new();
    
    // 添加会话信息
    let session = req.get_session();
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("username", &username);
        context.insert("is_authenticated", &true);
    }
    
    let rendered = data.render("admin/user/blacklist_add.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered)
}

// 添加黑名单处理
pub async fn blacklist_add_post(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    web::Form(form): web::Form<serde_json::Value>,
) -> impl Responder {
    let username = form.get("username").and_then(|v| v.as_str());
    let hardware_code = form.get("hardware_code").and_then(|v| v.as_str());
    let ip_address = form.get("ip_address").and_then(|v| v.as_str());
    
    // 添加到黑名单
    match crate::admin::services::user::add_to_blacklist(&pool, username, hardware_code, ip_address) {
        Ok(_) => {
            // 重定向到黑名单列表
            HttpResponse::Found()
                .append_header(("Location", "/admin/dashboard/users/blacklist"))
                .finish()
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("添加到黑名单失败")
        }
    }
}

// 从黑名单移除
pub async fn blacklist_remove(
    req: HttpRequest,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> impl Responder {
    let blacklist_id = path.into_inner();
    
    // 从黑名单移除
    match crate::admin::services::user::remove_from_blacklist(&pool, blacklist_id) {
        Ok(_) => {
            // 重定向到黑名单列表
            HttpResponse::Found()
                .append_header(("Location", "/admin/dashboard/users/blacklist"))
                .finish()
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("从黑名单移除失败")
        }
    }
}

// 添加用户页面
pub async fn user_add_get(
    req: HttpRequest,
    data: web::Data<Tera>,
) -> impl Responder {
    let mut context = tera::Context::new();
    
    // 添加会话信息
    let session = req.get_session();
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("username", &username);
        context.insert("is_authenticated", &true);
    }
    
    // 渲染添加用户页面
    match data.render("admin/user/add.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            // 输出详细的错误信息到日志
            eprintln!("模板渲染错误: {:#?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {:#?}. 模板路径: admin/user/add.html.", e))
        }
    }
}

// 添加用户处理
pub async fn user_add_post(
    req: HttpRequest,
    data: web::Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    config: web::Data<Config>,
    web::Form(form): web::Form<serde_json::Value>,
) -> impl Responder {
    // 获取表单数据
    let username = form.get("username").and_then(|v| v.as_str()).unwrap_or("");
    let password = form.get("password").and_then(|v| v.as_str()).unwrap_or("");
    let email = form.get("email").and_then(|v| v.as_str()).unwrap_or("");
    let vip_level = form.get("vip_level").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let status = form.get("status").and_then(|v| v.as_bool()).unwrap_or(true);
    
    // 创建用户
    match crate::admin::services::user::create_user(&pool, username, password, email, vip_level, status, &config) {
        Ok(_) => {
            // 记录管理员操作日志
            if let Some(admin_id) = req.extensions().get::<i32>() {
                let ip = get_client_ip(&req);
                let _ = log_admin_operation(
                    &pool,
                    *admin_id,
                    "create_user",
                    Some(&format!("创建新用户: {}", username)),
                    ip
                );
            }
            
            // 重定向到用户列表页面
            HttpResponse::Found()
                .append_header(("Location", "/admin/dashboard/users"))
                .finish()
        },
        Err(e) => {
            // 渲染错误页面
            let mut context = tera::Context::new();
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 添加错误信息
            context.insert("error", e.message());
            context.insert("username_value", &username);
            context.insert("email_value", &email);
            context.insert("vip_level_value", &vip_level);
            context.insert("status_value", &status);
            
            // 渲染添加用户页面，显示错误信息
            match data.render("admin/user/add.html", &context) {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(e) => {
                    eprintln!("模板渲染错误: {:#?}", e);
                    HttpResponse::InternalServerError()
                        .content_type("text/html; charset=utf-8")
                        .body(format!("模板渲染错误: {:#?}. 模板路径: admin/user/add.html.", e))
                }
            }
        }
    }
}