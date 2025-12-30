use actix_web::{web, HttpResponse, HttpRequest, Responder};
use actix_session::SessionExt;
use tera::{Tera, Context};
use chrono::{Utc, Duration};

// 销售业绩统计页面GET请求处理器
pub async fn sales_get(
    req: HttpRequest,
    tera: web::Data<Tera>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    // 创建上下文
    let mut context = Context::new();
    
    // 添加会话信息
    let session = req.get_session();
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("username", &username);
        context.insert("is_authenticated", &true);
    }
    
    // 默认统计时间范围：最近30天
    let end_time = Utc::now();
    let start_time = end_time - Duration::days(30);
    
    // 转换为时间戳
    let end_timestamp = end_time.timestamp();
    let start_timestamp = start_time.timestamp();
    
    // 获取销售统计数据
    let sales_stats = crate::admin::services::stats::get_sales_stats(
        &pool,
        start_timestamp,
        end_timestamp
    );
    
    if let Ok(stats) = sales_stats {
        context.insert("sales_stats", &stats);
    } else {
        context.insert("sales_stats", &serde_json::json!({"total_sales": 0.0, "sales_count": 0, "avg_sales": 0.0}));
    }
    
    // 添加时间范围到上下文
    context.insert("start_time", &start_timestamp);
    context.insert("end_time", &end_timestamp);
    
    // 渲染销售业绩统计页面
    match tera.render("admin/stats/sales.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            // 输出详细的错误信息到日志
            eprintln!("模板渲染错误: {:#?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {:#?}. 模板路径: admin/stats/sales.html.", e))
        }
    }
}

// 用户统计页面GET请求处理器
pub async fn users_get(
    req: HttpRequest,
    tera: web::Data<Tera>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    // 创建上下文
    let mut context = Context::new();
    
    // 添加会话信息
    let session = req.get_session();
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("username", &username);
        context.insert("is_authenticated", &true);
    }
    
    // 获取用户统计数据
    let user_stats = crate::admin::services::stats::get_user_stats(&pool);
    
    if let Ok(stats) = user_stats {
        context.insert("user_stats", &stats);
    } else {
        context.insert("user_stats", &serde_json::json!({"total_users": 0, "active_users": 0, "online_users": 0, "vip_users": 0}));
    }
    
    // 渲染用户统计页面
    match tera.render("admin/stats/users.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            // 输出详细的错误信息到日志
            eprintln!("模板渲染错误: {:#?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {:#?}. 模板路径: admin/stats/users.html.", e))
        }
    }
}

// 卡密统计页面GET请求处理器
pub async fn cards_get(
    req: HttpRequest,
    tera: web::Data<Tera>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    // 创建上下文
    let mut context = Context::new();
    
    // 添加会话信息
    let session = req.get_session();
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("username", &username);
        context.insert("is_authenticated", &true);
    }
    
    // 获取卡密统计数据
    let card_stats = crate::admin::services::stats::get_card_stats(&pool);
    
    if let Ok(stats) = card_stats {
        context.insert("card_stats", &stats);
    } else {
        context.insert("card_stats", &serde_json::json!({"total_cards": 0, "unused_cards": 0, "used_cards": 0, "total_sales": 0.0}));
    }
    
    // 渲染卡密统计页面
    match tera.render("admin/stats/cards.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            // 输出详细的错误信息到日志
            eprintln!("模板渲染错误: {:#?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {:#?}. 模板路径: admin/stats/cards.html.", e))
        }
    }
}

// 综合统计页面GET请求处理器
pub async fn overview_get(
    req: HttpRequest,
    tera: web::Data<Tera>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    // 创建上下文
    let mut context = Context::new();
    
    // 添加会话信息
    let session = req.get_session();
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("username", &username);
        context.insert("is_authenticated", &true);
    }
    
    // 获取综合统计数据
    let overview_stats = crate::admin::services::stats::get_stats(&pool);
    
    if let Ok(stats) = overview_stats {
        context.insert("overview_stats", &stats);
    } else {
        context.insert("overview_stats", &serde_json::json!({"total_users": 0, "total_software": 0, "total_cards": 0, "used_cards": 0, "total_sales": 0.0}));
    }
    
    // 渲染综合统计页面
    match tera.render("admin/stats/overview.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            // 输出详细的错误信息到日志
            eprintln!("模板渲染错误: {:#?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {:#?}. 模板路径: admin/stats/overview.html.", e))
        }
    }
}
