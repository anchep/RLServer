use actix_web::{web, HttpResponse, HttpRequest, Responder};
use actix_session::{Session, SessionExt};
use actix_web::http::header::LOCATION;
use tera::{Tera, Context};
use crate::admin::services::software::*;
use crate::admin::services::admin_user::log_admin_operation;
use crate::errors::ServiceError;
use crate::utils::ip::get_client_ip;

// 软件列表页面GET请求处理器
pub async fn list_get(
    req: HttpRequest,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    // 获取软件列表
    match get_all_software(&pool) {
        Ok(software_list) => {
            let mut context = Context::new();
            context.insert("software_list", &software_list);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染软件列表页面
            match tera.render("admin/software/list.html", &context) {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(e) => {
                    eprintln!("模板渲染错误: {}", e);
                    HttpResponse::InternalServerError()
                        .content_type("text/html; charset=utf-8")
                        .body("模板渲染错误")
                }
            }
        },
        Err(e) => {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(format!("<h1>软件列表</h1><div style='color: red'>{}</div>", e.message()))
        }
    }
}

// 添加软件页面GET请求处理器
pub async fn add_get(
    req: HttpRequest,
    tera: web::Data<Tera>,
) -> impl Responder {
    let mut context = Context::new();
    
    // 添加会话信息
    let session = req.get_session();
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("username", &username);
        context.insert("is_authenticated", &true);
    }
    
    // 渲染添加软件页面
    match tera.render("admin/software/add.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            eprintln!("模板渲染错误: {}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body("模板渲染错误")
        }
    }
}

// 添加软件页面POST请求处理器
pub async fn add_post(
    req: HttpRequest,
    form: web::Form<AddSoftwareForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let name = &form.name;
    let chinese_name = &form.chinese_name;
    let description = &form.description;
    let detailed_description = &form.detailed_description;
    let executable_name = &form.executable_name;
    let md5_checksum = &form.md5_checksum;
    let requires_admin = form.requires_admin.is_some();
    let required_vip_level = form.required_vip_level;
    let status = true; // 默认启用
    
    // 创建软件
    match create_software(
        &pool,
        name,
        chinese_name,
        description,
        detailed_description,
        executable_name,
        md5_checksum,
        requires_admin,
        required_vip_level,
        status,
    ) {
        Ok(_) => {
            // 获取管理员ID
            let session = req.get_session();
            if let Ok(Some(admin_id)) = session.get::<i32>("admin_id") {
                // 记录操作日志
                let ip = get_client_ip(&req);
                let _ = log_admin_operation(&pool, admin_id, "create_software", Some(&format!("创建软件: {}", name)), ip);
            }
            
            // 重定向到软件列表页面
            HttpResponse::Found()
                .header(LOCATION, "/admin/dashboard/software")
                .finish()
        },
        Err(e) => {
            let mut context = Context::new();
            context.insert("error", e.message());
            context.insert("name", name);
            context.insert("chinese_name", chinese_name);
            context.insert("description", description);
            context.insert("detailed_description", detailed_description);
            context.insert("executable_name", executable_name);
            context.insert("md5_checksum", md5_checksum);
            context.insert("requires_admin", &requires_admin);
            context.insert("required_vip_level", &required_vip_level);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染添加软件页面，显示错误信息
            match tera.render("admin/software/add.html", &context) {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(err) => {
                    eprintln!("模板渲染错误: {}", err);
                    HttpResponse::InternalServerError()
                        .content_type("text/html; charset=utf-8")
                        .body(format!("添加软件失败: {}", e.message()))
                }
            }
        }
    }
}

// 编辑软件页面GET请求处理器
pub async fn edit_get(
    req: HttpRequest,
    path: web::Path<(i32,)>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let id = path.0;
    
    // 获取软件详情
    match get_software_by_id(&pool, id) {
        Ok(software) => {
            let mut context = Context::new();
            context.insert("software", &software);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染编辑软件页面
            match tera.render("admin/software/edit.html", &context) {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(e) => {
                    eprintln!("模板渲染错误: {}", e);
                    HttpResponse::InternalServerError()
                        .content_type("text/html; charset=utf-8")
                        .body("模板渲染错误")
                }
            }
        },
        Err(e) => {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(format!("<h1>编辑软件</h1><div style='color: red'>{}</div>", e.message()))
        }
    }
}

// 编辑软件页面POST请求处理器
pub async fn edit_post(
    req: HttpRequest,
    path: web::Path<(i32,)>,
    form: web::Form<EditSoftwareForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let id = path.0;
    let name = &form.name;
    let chinese_name = &form.chinese_name;
    let description = &form.description;
    let detailed_description = &form.detailed_description;
    let executable_name = &form.executable_name;
    let md5_checksum = &form.md5_checksum;
    let requires_admin = form.requires_admin.is_some();
    let required_vip_level = form.required_vip_level;
    
    // 更新软件
    match update_software(
        &pool,
        id,
        name,
        chinese_name,
        description,
        detailed_description,
        executable_name,
        md5_checksum,
        requires_admin,
        required_vip_level,
    ) {
        Ok(_) => {
            // 获取管理员ID
            let session = req.get_session();
            if let Ok(Some(admin_id)) = session.get::<i32>("admin_id") {
                // 记录操作日志
                let ip = get_client_ip(&req);
                let _ = log_admin_operation(&pool, admin_id, "update_software", Some(&format!("更新软件: {}", name)), ip);
            }
            
            // 重定向到软件列表页面
            HttpResponse::Found()
                .header(LOCATION, "/admin/dashboard/software")
                .finish()
        },
        Err(e) => {
            // 获取软件详情用于重新渲染
            let software = get_software_by_id(&pool, id);
            
            let mut context = Context::new();
            context.insert("error", e.message());
            context.insert("name", name);
            context.insert("chinese_name", chinese_name);
            context.insert("description", description);
            context.insert("detailed_description", detailed_description);
            context.insert("executable_name", executable_name);
            context.insert("md5_checksum", md5_checksum);
            context.insert("requires_admin", &requires_admin);
            context.insert("required_vip_level", &required_vip_level);
            
            if let Ok(software) = software {
                context.insert("software", &software);
            }
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染编辑软件页面，显示错误信息
            match tera.render("admin/software/edit.html", &context) {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(err) => {
                    eprintln!("模板渲染错误: {}", err);
                    HttpResponse::InternalServerError()
                        .content_type("text/html; charset=utf-8")
                        .body(format!("更新软件失败: {}", e.message()))
                }
            }
        }
    }
}

// 删除软件页面POST请求处理器
pub async fn delete_post(
    req: HttpRequest,
    path: web::Path<(i32,)>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    let id = path.0;
    
    // 删除软件
    match delete_software(&pool, id) {
        Ok(_) => {
            // 获取管理员ID
            let session = req.get_session();
            if let Ok(Some(admin_id)) = session.get::<i32>("admin_id") {
                // 记录操作日志
                let ip = get_client_ip(&req);
                let _ = log_admin_operation(&pool, admin_id, "delete_software", Some(&format!("删除软件ID: {}", id)), ip);
            }
            
            // 重定向到软件列表页面
            HttpResponse::Found()
                .header(LOCATION, "/admin/dashboard/software")
                .finish()
        },
        Err(e) => {
            // 重定向到软件列表页面，显示错误信息
            HttpResponse::Found()
                .header(LOCATION, format!("/admin/dashboard/software?error={}", urlencoding::encode(e.message())))
                .finish()
        }
    }
}

// 切换软件状态页面POST请求处理器
pub async fn toggle_post(
    req: HttpRequest,
    path: web::Path<(i32,)>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    let id = path.0;
    
    // 切换软件状态
    match toggle_software_status(&pool, id) {
        Ok(software) => {
            // 获取管理员ID
            let session = req.get_session();
            if let Ok(Some(admin_id)) = session.get::<i32>("admin_id") {
                let status_text = if software.status { "启用" } else { "禁用" };
                // 获取客户端IP地址
                let ip = get_client_ip(&req);
                // 记录操作日志
                let _ = log_admin_operation(&pool, admin_id, "toggle_software_status", Some(&format!("{}软件: {}", status_text, software.name)), ip);
            }
            
            // 重定向到软件列表页面
            HttpResponse::Found()
                .header(LOCATION, "/admin/dashboard/software")
                .finish()
        },
        Err(e) => {
            // 重定向到软件列表页面，显示错误信息
            HttpResponse::Found()
                .header(LOCATION, format!("/admin/dashboard/software?error={}", urlencoding::encode(e.message())))
                .finish()
        }
    }
}

// 软件详情页面GET请求处理器
pub async fn detail_get(
    req: HttpRequest,
    path: web::Path<(i32,)>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let id = path.0;
    
    // 获取软件详情
    match get_software_by_id(&pool, id) {
        Ok(software) => {
            let mut context = Context::new();
            context.insert("software", &software);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染软件详情页面
            match tera.render("admin/software/detail.html", &context) {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(e) => {
                    eprintln!("模板渲染错误: {}", e);
                    HttpResponse::InternalServerError()
                        .content_type("text/html; charset=utf-8")
                        .body("模板渲染错误")
                }
            }
        },
        Err(e) => {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(format!("<h1>软件详情</h1><div style='color: red'>{}</div>", e.message()))
        }
    }
}

// 添加软件表单
#[derive(Debug, serde::Deserialize)]
pub struct AddSoftwareForm {
    pub name: String,
    pub chinese_name: String,
    pub description: String,
    pub detailed_description: String,
    pub executable_name: String,
    pub md5_checksum: String,
    pub requires_admin: Option<String>,
    pub required_vip_level: i32,
}

// 编辑软件表单
#[derive(Debug, serde::Deserialize)]
pub struct EditSoftwareForm {
    pub name: String,
    pub chinese_name: String,
    pub description: String,
    pub detailed_description: String,
    pub executable_name: String,
    pub md5_checksum: String,
    pub requires_admin: Option<String>,
    pub required_vip_level: i32,
}

