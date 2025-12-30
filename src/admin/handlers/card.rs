use actix_web::{web, HttpResponse, HttpRequest, Responder};
use actix_session::SessionExt;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use tera::{Tera, Context};
use crate::admin::services::card::*;
use crate::admin::services::admin_user::log_admin_operation;
use crate::errors::ServiceError;

// 卡密列表页面GET请求处理器
pub async fn list_get(
    req: HttpRequest,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
    web::Query(query): web::Query<CardListQuery>,
) -> impl Responder {
    // 获取查询参数
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    
    // 处理status参数：将字符串转换为Option<bool>
    let status = match query.status {
        Some(s) if !s.is_empty() => {
            s.parse::<i32>().ok().map(|x| x == 1)
        },
        _ => None,
    };
    
    // 处理vip_level参数：将字符串转换为Option<i32>
    let vip_level = match query.vip_level {
        Some(vl) if !vl.is_empty() => {
            vl.parse::<i32>().ok()
        },
        _ => None,
    };
    
    let card_type = vip_level;
    
    // 获取卡密列表
    match get_recharge_cards(&pool, status, vip_level, page, page_size) {
        Ok((card_list, total)) => {
            let mut context = Context::new();
            context.insert("card_list", &card_list);
            context.insert("total", &total);
            context.insert("page", &page);
            context.insert("page_size", &page_size);
            context.insert("status", &status);
            context.insert("card_type", &card_type);
            context.insert("vip_level", &vip_level);
            
            // 计算总页数
            let total_pages = (total + page_size as i64 - 1) / page_size as i64;
            context.insert("total_pages", &total_pages);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染卡密列表页面
            match tera.render("admin/card/list.html", &context) {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(e) => {
                    use std::error::Error;
                    let mut error_details = format!("Error: {:#?}\n\n", e);
                    error_details.push_str("Context variables: ");
                    error_details.push_str(&format!("card_list: {} items, total: {}, page: {}, page_size: {}", card_list.len(), total, page, page_size));
                    
                    let mut current = Some(&e as &dyn Error);
                    let mut count = 0;
                    error_details.push_str("\n\nError chain: ");
                    
                    while let Some(err) = current {
                        error_details.push_str(&format!("{:?}\n  ", err));
                        current = err.source();
                        count += 1;
                        if count > 5 {
                            error_details.push_str("...");
                            break;
                        }
                    }
                    
                    eprintln!("{}", error_details);
                    HttpResponse::InternalServerError()
                        .content_type("text/html; charset=utf-8")
                        .body(format!("<pre>{}</pre>", error_details))
                }
            }
        },
        Err(e) => {
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(format!("<h1>卡密列表</h1><div style='color: red'>{}</div>", e.message()))
        }
    }
}

// 生成卡密页面GET请求处理器
pub async fn generate_get(
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
    
    // 设置默认表单值
    context.insert("vip_level", &0);
    context.insert("duration", &30);
    context.insert("price", &0);
    context.insert("quantity", &10);
    
    // 渲染生成卡密页面
    match tera.render("admin/card/generate.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            eprintln!("模板渲染错误: {}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {}", e))
        }
    }
}

// 生成卡密页面POST请求处理器
pub async fn generate_post(
    req: HttpRequest,
    form: web::Form<GenerateCardForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    // 获取会话中的管理员ID
    let session = req.get_session();
    let admin_id = if let Ok(Some(id)) = session.get::<i32>("admin_id") {
        id
    } else {
        return HttpResponse::Found()
            .header(LOCATION, "/admin/login")
            .finish();
    };
    
    let vip_level = form.vip_level;
    let duration = form.duration;
    let price = form.price;
    let quantity = form.quantity;
    
    // 生成卡密
    match generate_recharge_cards(&pool, 0, vip_level, duration, price, quantity) {
        Ok(generated_cards) => {
            // 记录操作日志
            let ip = req.connection_info().remote_addr().map(|s| s.to_string());
            let _ = log_admin_operation(&pool, admin_id, "generate_cards", Some(&format!("生成卡密 {} 张，VIP等级：{}，时长：{}天，售价：¥{}", quantity, vip_level, duration, price)), ip);
            
            let mut context = Context::new();
            context.insert("generated_cards", &generated_cards);
            context.insert("quantity", &quantity);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染生成结果页面
            match tera.render("admin/card/generate_result.html", &context) {
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
            let mut context = Context::new();
            context.insert("error", e.message());
            context.insert("vip_level", &vip_level);
            context.insert("duration", &duration);
            context.insert("price", &price);
            context.insert("quantity", &quantity);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染生成卡密页面，显示错误信息
            match tera.render("admin/card/generate.html", &context) {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(err) => {
                    eprintln!("模板渲染错误: {}", err);
                    HttpResponse::InternalServerError()
                        .content_type("text/html; charset=utf-8")
                        .body(format!("生成卡密失败: {}", e.message()))
                }
            }
        }
    }
}

// 卡密详情页面GET请求处理器
pub async fn detail_get(
    req: HttpRequest,
    path: web::Path<(i32,)>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let card_id = path.0;
    
    // 获取卡密详情
    match get_recharge_card_by_id(&pool, card_id) {
        Ok(card) => {
            let mut context = Context::new();
            context.insert("card", &card);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染卡密详情页面
            match tera.render("admin/card/detail.html", &context) {
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
            // 卡密不存在，重定向到卡密列表页面，显示错误信息
            HttpResponse::Found()
                .header(LOCATION, format!("/admin/dashboard/cards?error={}", urlencoding::encode(e.message())))
                .finish()
        }
    }
}

// 更新卡密售价POST请求处理器
pub async fn update_price_post(
    req: HttpRequest,
    path: web::Path<(i32,)>,
    form: web::Form<UpdatePriceForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    let card_id = path.0;
    let price = form.price;
    
    // 获取会话中的管理员ID
    let session = req.get_session();
    let admin_id = if let Ok(Some(id)) = session.get::<i32>("admin_id") {
        id
    } else {
        return HttpResponse::Found()
            .header(LOCATION, "/admin/login")
            .finish();
    };
    
    // 更新卡密售价
    match update_card_price(&pool, card_id, price) {
        Ok(updated_card) => {
            // 记录操作日志
            let ip = req.connection_info().remote_addr().map(|s| s.to_string());
            let _ = log_admin_operation(&pool, admin_id, "update_card_price", Some(&format!("更新卡密ID {} 售价为：¥{}", card_id, price)), ip);
            
            // 重定向到卡密详情页面
            HttpResponse::Found()
                .header(LOCATION, format!("/admin/dashboard/cards/detail/{}", card_id))
                .finish()
        },
        Err(e) => {
            // 重定向到卡密详情页面，显示错误信息
            HttpResponse::Found()
                .header(LOCATION, format!("/admin/dashboard/cards/detail/{}?error={}", card_id, urlencoding::encode(e.message())))
                .finish()
        }
    }
}

// 删除卡密POST请求处理器
pub async fn delete_post(
    req: HttpRequest,
    path: web::Path<(i32,)>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    let card_id = path.0;
    
    // 获取会话中的管理员ID
    let session = req.get_session();
    let admin_id = if let Ok(Some(id)) = session.get::<i32>("admin_id") {
        id
    } else {
        return HttpResponse::Found()
            .header(LOCATION, "/admin/login")
            .finish();
    };
    
    // 删除卡密
    match delete_recharge_card(&pool, card_id) {
        Ok(_) => {
            // 记录操作日志
            let ip = req.connection_info().remote_addr().map(|s| s.to_string());
            let _ = log_admin_operation(&pool, admin_id, "delete_card", Some(&format!("删除卡密ID：{}", card_id)), ip);
            
            // 重定向到卡密列表页面
            HttpResponse::Found()
                .header(LOCATION, "/admin/dashboard/cards")
                .finish()
        },
        Err(e) => {
            // 重定向到卡密列表页面，显示错误信息
            HttpResponse::Found()
                .header(LOCATION, format!("/admin/dashboard/cards?error={}", urlencoding::encode(e.message())))
                .finish()
        }
    }
}

// 定义批量删除表单结构体
#[derive(Debug, serde::Deserialize)]
pub struct BatchDeleteForm {
    #[serde(deserialize_with = "deserialize_card_ids")]
    card_ids: Vec<i32>,
}

// 自定义反序列化函数，处理单个字符串和数组情况
fn deserialize_card_ids<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::*;

    // 尝试将输入解析为字符串
    struct StringOrVecVisitor;

    impl<'de> Visitor<'de> for StringOrVecVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or vec of strings")
        }

        // 处理单个字符串
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            // 尝试将字符串解析为i32
            match value.parse::<i32>() {
                Ok(id) => Ok(vec![id]),
                Err(_) => {
                    // 尝试将字符串按逗号分割成多个i32
                    let ids: Result<Vec<i32>, _> = value.split(',').map(|s| s.parse::<i32>()).collect();
                    ids.map_err(|e| Error::custom(format!("Failed to parse card_ids: {}", e)))
                }
            }
        }

        // 处理字符串数组
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut ids = Vec::new();

            while let Some(id_str) = seq.next_element::<String>()? {
                match id_str.parse::<i32>() {
                    Ok(id) => ids.push(id),
                    Err(e) => return Err(Error::custom(format!("Failed to parse card_id: {}", e))),
                }
            }

            Ok(ids)
        }
    }

    deserializer.deserialize_any(StringOrVecVisitor)
}

// 批量删除卡密POST请求处理器
pub async fn batch_delete_post(
    req: HttpRequest,
    form: web::Form<BatchDeleteForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    // 获取会话中的管理员ID
    let session = req.get_session();
    let admin_id = if let Ok(Some(id)) = session.get::<i32>("admin_id") {
        id
    } else {
        return HttpResponse::Found()
            .header(LOCATION, "/admin/login")
            .finish();
    };
    
    // 从表单中获取卡密ID列表
    let card_ids = form.card_ids.clone();
    
    if card_ids.is_empty() {
        return HttpResponse::Found()
            .header(LOCATION, "/admin/dashboard/cards?error=请选择要删除的卡密")
            .finish();
    }
    
    // 批量删除卡密
    match batch_delete_recharge_cards(&pool, &card_ids) {
        Ok(deleted_count) => {
            // 记录操作日志
            let ip = req.connection_info().remote_addr().map(|s| s.to_string());
            let _ = log_admin_operation(&pool, admin_id, "batch_delete_cards", Some(&format!("批量删除卡密 {} 张", deleted_count)), ip);
            
            // 重定向到卡密列表页面
            HttpResponse::Found()
                .header(LOCATION, "/admin/dashboard/cards")
                .finish()
        },
        Err(e) => {
            // 重定向到卡密列表页面，显示错误信息
            HttpResponse::Found()
                .header(LOCATION, format!("/admin/dashboard/cards?error={}", urlencoding::encode(e.message())))
                .finish()
        }
    }
}

// 充值历史页面GET请求处理器
pub async fn history_get(
    req: HttpRequest,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
    web::Query(query): web::Query<HistoryQuery>,
) -> impl Responder {
    // 获取查询参数
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    
    // 解析user_id，处理空字符串情况
    let user_id = match &query.user_id {
        Some(id_str) if !id_str.is_empty() => {
            id_str.parse::<i32>().ok()
        },
        _ => None
    };
    
    let card_code = query.card_code.as_deref().filter(|&s| !s.is_empty());
    
    // 解析start_time，处理空字符串情况
    let start_time = match &query.start_time {
        Some(time_str) if !time_str.is_empty() => {
            // 处理datetime-local格式: 2023-12-28T14:30
            if time_str.contains('T') {
                let datetime = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M")
                    .ok();
                datetime.map(|dt| dt.timestamp())
            } else {
                time_str.parse::<i64>().ok()
            }
        },
        _ => None
    };
    
    // 解析end_time，处理空字符串情况
    let end_time = match &query.end_time {
        Some(time_str) if !time_str.is_empty() => {
            // 处理datetime-local格式: 2023-12-28T14:30
            if time_str.contains('T') {
                let datetime = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M")
                    .ok();
                datetime.map(|dt| dt.timestamp())
            } else {
                time_str.parse::<i64>().ok()
            }
        },
        _ => None
    };
    
    // 转换时间格式
    let start_time_dt = start_time.map(|t| chrono::DateTime::from_timestamp(t, 0).unwrap_or(chrono::Utc::now()));
    let end_time_dt = end_time.map(|t| chrono::DateTime::from_timestamp(t, 0).unwrap_or(chrono::Utc::now()));
    
    // 获取充值历史
    match get_recharge_history(&pool, user_id, card_code, start_time_dt, end_time_dt, page, page_size) {
        Ok((history_list, total)) => {
            let mut context = Context::new();
            context.insert("history_list", &history_list);
            context.insert("total", &total);
            context.insert("page", &page);
            context.insert("page_size", &page_size);
            context.insert("user_id", &query.user_id.as_deref().unwrap_or(""));
            context.insert("card_code", &query.card_code);
            context.insert("start_time", &query.start_time);
            context.insert("end_time", &query.end_time);
            
            // 计算总页数
            let total_pages = (total + page_size as i64 - 1) / page_size as i64;
            context.insert("total_pages", &total_pages);
            
            // 添加会话信息
            let session = req.get_session();
            if let Ok(Some(username)) = session.get::<String>("username") {
                context.insert("username", &username);
                context.insert("is_authenticated", &true);
            }
            
            // 渲染充值历史页面
            match tera.render("admin/card/history.html", &context) {
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
                .body(format!("<h1>充值历史</h1><div style='color: red'>{}</div>", e.message()))
        }
    }
}

// 卡密列表查询参数
#[derive(Debug, serde::Deserialize)]
pub struct CardListQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub status: Option<String>,
    pub vip_level: Option<String>,
}

// 生成卡密表单
#[derive(Debug, serde::Deserialize)]
pub struct GenerateCardForm {
    pub vip_level: i32,
    pub duration: i32,
    pub price: f64,
    pub quantity: i32,
}

// 更新卡密售价表单
#[derive(Debug, serde::Deserialize)]
pub struct UpdatePriceForm {
    pub price: f64,
}

// 充值历史查询参数
#[derive(Debug, serde::Deserialize)]
pub struct HistoryQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub user_id: Option<String>,
    pub card_code: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}
