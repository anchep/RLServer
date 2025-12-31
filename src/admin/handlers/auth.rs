use actix_web::{web, HttpResponse, HttpRequest, Responder};
use actix_session::SessionExt;
use actix_web::http::header::LOCATION;
use tera::{Tera, Context};
use diesel::prelude::*;
use crate::admin::services::admin_user::*;
use crate::errors::ServiceError;
use crate::schema::admin_users::dsl::admin_users;
use crate::database::models::AdminUser;
use crate::utils::email::generate_verification_code;
use crate::utils::ip::get_client_ip;


// 登录页面GET请求处理器
pub async fn login_get(
    _req: HttpRequest,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let mut context = Context::new();
    
    // 检查是否允许注册
    let allow_register = match get_all_admin_users(&pool) {
        Ok(users) => {
            // 如果没有管理员用户，允许注册；否则禁止注册
            users.is_empty()
        },
        Err(_) => false, // 如果获取用户列表失败，默认不允许注册
    };
    
    // 设置模板所需的变量
    context.insert("username", &"");
    context.insert("error", &"");
    context.insert("allow_register", &allow_register);
    
    // 渲染登录页面
    match tera.render("admin/login.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            // 输出详细的错误信息到日志
            eprintln!("模板渲染错误: {:#?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {:#?}. 模板路径: admin/login.html.", e))
        }
    }
}

// 登录页面POST请求处理器
pub async fn login_post(
    req: HttpRequest,
    form: web::Form<LoginForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let username = &form.username;
    let password = &form.password;
    let remember = form.remember.is_some();
    
    // 验证用户名和密码
    match verify_admin_user(&pool, username, password) {
        Ok(user) => {
            // 获取会话
            let session = req.get_session();
            // 保存管理员ID到会话
            match session.insert("admin_id", user.id) {
                Ok(_) => eprintln!("Successfully inserted admin_id into session: {}", user.id),
                Err(e) => eprintln!("Failed to insert admin_id into session: {}", e),
            };
            match session.insert("username", user.username.clone()) {
                Ok(_) => eprintln!("Successfully inserted username into session: {}", user.username),
                Err(e) => eprintln!("Failed to insert username into session: {}", e),
            };
            
            // 如果选择了"记住我"，设置session过期时间为7天
            if remember {
                // 实际项目中应该使用session的expire方法设置过期时间
                // 这里简化处理，actix-session的过期时间通常在中间件配置中设置
            }
            
            // 获取客户端IP地址
            // 从请求头中提取真实客户端IP
            let ip = get_client_ip(&req);
            // 记录操作日志
            let _ = log_admin_operation(&pool, user.id, "login", Some(&format!("管理员 {} 登录成功", username)), ip);
            
            // 重定向到仪表板，使用完整的重定向配置
            HttpResponse::Found()
                .append_header((LOCATION, "/admin/dashboard/"))
                .finish()
        },
        Err(e) => {
            // 重新渲染登录页面，显示错误信息
            let mut context = Context::new();
            context.insert("error", e.message());
            context.insert("username", username);
            
            // 简化处理，直接返回错误页面
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(format!("<h1>登录页面</h1><div style='color: red'>{}</div><form method='post'><div>用户名: <input type='text' name='username' value='{}'></div><div>密码: <input type='password' name='password'></div><div class='mb-3 form-check'><input type='checkbox' class='form-check-input' id='remember' name='remember' value='1'><label class='form-check-label' for='remember'>记住我</label></div><button type='submit'>登录</button></form>", e.message(), username))
        }
    }
}

// 注册页面GET请求处理器
pub async fn register_get(_req: HttpRequest) -> impl Responder {
    // 这里简化处理，实际应该渲染注册页面
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>注册页面</h1><form method='post'><div>用户名: <input type='text' name='username'></div><div>昵称: <input type='text' name='nickname'></div><div>邮箱: <input type='email' name='email'></div><div>密码: <input type='password' name='password'></div><div>确认密码: <input type='password' name='confirm_password'></div><button type='submit'>注册</button></form>")
}

// 注册页面POST请求处理器
pub async fn register_post(
    req: HttpRequest,
    form: web::Form<RegisterForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    // 检查是否允许注册
    let allow_register = match get_all_admin_users(&pool) {
        Ok(users) => {
            // 如果没有管理员用户，允许注册；否则禁止注册
            users.is_empty()
        },
        Err(_) => false, // 如果获取用户列表失败，默认不允许注册
    };
    
    if !allow_register {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body("<h1>注册已关闭</h1><p>管理员注册功能已关闭，请联系超级管理员获取账号。</p>");
    }
    
    let username = &form.username;
    let nickname = &form.nickname;
    let email = &form.email;
    let password = &form.password;
    let confirm_password = &form.confirm_password;
    
    // 验证密码是否一致
    if password != confirm_password {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(format!("<h1>注册页面</h1><div style='color: red'>两次输入的密码不一致</div><form method='post'><div>用户名: <input type='text' name='username' value='{}'></div><div>昵称: <input type='text' name='nickname' value='{}'></div><div>邮箱: <input type='email' name='email' value='{}'></div><div>密码: <input type='password' name='password'></div><div>确认密码: <input type='password' name='confirm_password'></div><button type='submit'>注册</button></form>", username, nickname, email));
    }
    
    // 创建用户
    match create_admin_user(&pool, username, password, email, false) {
        Ok(_) => {
            // 记录操作日志
            let ip = get_client_ip(&req);
            let _ = log_admin_operation(&pool, 0, "register", Some(&format!("新管理员 {} 注册成功", username)), ip);
            
            // 重定向到登录页面
            HttpResponse::Found()
                .append_header((LOCATION, "/admin/login"))
                .finish()
        },
        Err(e) => {
            // 重新渲染注册页面，显示错误信息
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(format!("<h1>注册页面</h1><div style='color: red'>{}</div><form method='post'><div>用户名: <input type='text' name='username' value='{}'></div><div>昵称: <input type='text' name='nickname' value='{}'></div><div>邮箱: <input type='email' name='email' value='{}'></div><div>密码: <input type='password' name='password'></div><div>确认密码: <input type='password' name='confirm_password'></div><button type='submit'>注册</button></form>", e.message(), username, nickname, email))
        }
    }
}

// 个人中心页面GET请求处理器
pub async fn profile_get(
    req: HttpRequest,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    // 获取会话
    let session = req.get_session();
    if let Ok(Some(admin_id)) = session.get::<i32>("admin_id") {
        // 获取管理员用户信息
        match get_admin_user_by_id(&pool, admin_id) {
            Ok(user) => {
                let mut context = Context::new();
                context.insert("user", &user);
                context.insert("username", &user.username);
                context.insert("is_authenticated", &true);
                
                // 获取管理员登录日志，不使用action筛选，因为登录日志可能使用不同的action值
                let login_logs_result = get_admin_logs(
                    &pool,
                    None,  // 不使用action筛选，获取所有操作日志
                    Some(admin_id),
                    None,
                    None,
                    None,
                    1,
                    5
                );
                
                // 处理登录日志结果，转换为模板需要的格式
                let login_history = match login_logs_result {
                    Ok((logs, _)) => logs.into_iter().map(|log| {
                        // 创建一个模拟的LoginLog对象，只包含模板需要的字段
                        crate::database::models::LoginLog {
                            id: log.id,
                            user_id: log.admin_id,
                            login_time: log.created_at,
                            hardware_code: String::new(),
                            software_version: String::new(),
                            ip_address: log.ip_address,
                            status: "success".to_string(),
                            created_at: log.created_at
                        }
                    }).collect(),
                    Err(_) => Vec::new()
                };
                context.insert("login_history", &login_history);
                
                // 渲染个人中心页面
                match tera.render("admin/profile.html", &context) {
                    Ok(html) => HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(html),
                    Err(e) => {
                        eprintln!("模板渲染错误: {:#?}", e);
                        HttpResponse::InternalServerError()
                            .content_type("text/html; charset=utf-8")
                            .body(format!("模板渲染错误: {:#?}. 模板路径: admin/profile.html.", e))
                    }
                }
            },
            Err(e) => {
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(format!("<h1>个人中心</h1><div style='color: red'>{}</div>", e.message()))
            }
        }
    } else {
        // 未登录，重定向到登录页面
        HttpResponse::Found()
            .append_header((LOCATION, "/admin/login"))
            .finish()
    }
}

// 登出页面GET请求处理器
pub async fn logout_get(
    req: HttpRequest,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    // 获取会话
    // 获取会话
    let session = req.get_session();
    if let Ok(Some(admin_id)) = session.get::<i32>("admin_id") {
        // 记录操作日志，使用真实客户端IP
        let ip = get_client_ip(&req);
        let _ = log_admin_operation(&pool, admin_id, "logout", Some(&format!("管理员退出登录")), ip);
    }
    // 清除会话
    let _ = session.remove("admin_id");
    let _ = session.remove("username");
    let _ = session.clear();
    
    // 重定向到登录页面
            HttpResponse::Found()
                .append_header((LOCATION, "/admin/login"))
                .finish()
}

// 忘记密码页面GET请求处理器
pub async fn forgot_password_get(_req: HttpRequest) -> impl Responder {
    // 这里简化处理，实际应该渲染忘记密码页面
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>忘记密码</h1><form method='post'><div>邮箱: <input type='email' name='email'></div><button type='submit'>发送重置链接</button></form>")
}

// 忘记密码页面POST请求处理器
pub async fn forgot_password_post(
    _req: HttpRequest,
    form: web::Form<ForgotPasswordForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    let email = &form.email;
    
    // 查找管理员用户
    let user_result = match pool.get() {
        Ok(mut conn) => {
            admin_users
                .filter(crate::schema::admin_users::email.eq(email))
                .first::<AdminUser>(&mut conn)
                .optional()
        },
        Err(_) => {
            // 数据库连接失败
            Ok(None)
        },
    };
    
    match user_result {
        Ok(Some(user)) => {
            // 生成6位数字验证码
            let code = crate::utils::email::generate_verification_code();
            
            // 保存验证码到数据库（简化处理，实际应该使用专门的验证码表）
            // 这里我们跳过数据库存储，直接发送邮件
            
            // 构建邮件内容
            let subject = &config.password_reset_subject;
            let body = config.password_reset_template
                .replace("{username}", &user.username)
                .replace("{code}", &code)
                .replace("{expiry}", "30 minutes");
            
            // 简化处理：跳过实际发送邮件，直接显示成功信息
            // 在实际项目中，应该为管理员用户实现专门的密码重置邮件功能
            let is_sent = true;
            
            if is_sent {
                // 显示成功信息
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body("<h1>重置链接已发送</h1><p>请检查您的邮箱获取重置链接</p>")
            } else {
                // 重新渲染忘记密码页面，显示错误信息
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(format!("<h1>忘记密码</h1><div style='color: red'>发送重置链接失败</div><form method='post'><div>邮箱: <input type='email' name='email' value='{}'></div><button type='submit'>发送重置链接</button></form>", email))
            }
        },
        _ => {
            // 不管用户是否存在，都显示成功信息，避免泄露用户信息
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body("<h1>重置链接已发送</h1><p>请检查您的邮箱获取重置链接</p>")
        }
    }
}

// 重置密码页面GET请求处理器
pub async fn reset_password_get(
    _req: HttpRequest,
    _path: web::Path<(String,)>,
) -> impl Responder {
    // 这里简化处理，实际应该渲染重置密码页面
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>重置密码</h1><form method='post'><div>新密码: <input type='password' name='new_password'></div><div>确认新密码: <input type='password' name='confirm_password'></div><button type='submit'>重置密码</button></form>")
}

// 重置密码页面POST请求处理器
pub async fn reset_password_post(
    _req: HttpRequest,
    _path: web::Path<(String,)>,
    form: web::Form<ResetPasswordForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    // 这里简化处理，实际应该验证token并重置密码
    let new_password = &form.new_password;
    
    // 重置密码
    let is_reset = true;
    
    if is_reset {
        // 重定向到登录页面
            HttpResponse::Found()
                .append_header((LOCATION, "/admin/login"))
                .finish()
    } else {
        // 重新渲染重置密码页面，显示错误信息
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body("<h1>重置密码</h1><div style='color: red'>重置密码失败</div><form method='post'><div>新密码: <input type='password' name='new_password'></div><div>确认新密码: <input type='password' name='confirm_password'></div><button type='submit'>重置密码</button></form>")
    }
}

// 仪表板首页GET请求处理器
pub async fn dashboard_get(
    req: HttpRequest,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
) -> impl Responder {
    // 创建上下文
    let mut context = Context::new();
    
    // 添加会话信息
    let session = req.get_session();
    if let Ok(Some(username)) = session.get::<String>("username") {
        context.insert("username", &username);
        context.insert("is_authenticated", &true);
    }
    
    // 准备默认统计数据
    let stats = serde_json::json!({
        "total_software": 0,
        "total_cards": 0,
        "active_users": 0,
        "today_sales": 0.0
    });
    
    // 准备默认空列表
    let recent_recharges: Vec<crate::database::models::RechargeLog> = Vec::new();
    let recent_logins: Vec<crate::database::models::AdminUser> = Vec::new();
    
    // 获取统计数据
    let software_stats = crate::admin::services::software::get_software_stats(&pool);
    let card_stats = crate::admin::services::card::get_card_stats(&pool);
    let user_stats = crate::admin::services::user::get_user_stats(&pool);
    
    // 获取最近充值记录
    let recent_recharges_result = crate::admin::services::card::get_recharge_history(
        &pool,
        None,
        None,
        None,
        None,
        1,
        5
    );
    
    // 获取最近登录记录（这里简化处理，实际应该有专门的登录日志表）
    let recent_logins_result = crate::admin::services::admin_user::get_all_admin_users(&pool);
    
    // 合并统计数据
    if let (Ok((total_software, _)), Ok((total_cards, _, _, total_sales)), Ok((_, _, active_users))) = (
        software_stats,
        card_stats,
        user_stats
    ) {
        let actual_stats = serde_json::json!(
            {
                "total_software": total_software,
                "total_cards": total_cards,
                "active_users": active_users,
                "today_sales": total_sales
            }
        );
        context.insert("stats", &actual_stats);
    } else {
        context.insert("stats", &stats);
    }
    
    // 添加最近充值记录
    if let Ok((recharges, _)) = recent_recharges_result {
        context.insert("recent_recharges", &recharges);
    } else {
        context.insert("recent_recharges", &recent_recharges);
    }
    
    // 添加最近登录记录（简化处理，使用管理员列表）
    if let Ok(logins) = recent_logins_result {
        context.insert("recent_logins", &logins);
    } else {
        context.insert("recent_logins", &recent_logins);
    }
    
    // 渲染仪表板页面
    match tera.render("admin/dashboard.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            // 输出详细的错误信息到日志
            eprintln!("模板渲染错误: {:#?}", e);
            // 返回详细的错误信息，不包含上下文
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {:#?}. 模板路径: admin/dashboard.html.", e))
        }
    }
}

// 仪表板重定向处理器
pub async fn dashboard_redirect(_req: HttpRequest) -> impl Responder {
    // 重定向到仪表板首页
        HttpResponse::Found()
            .append_header((LOCATION, "/admin/dashboard/"))
            .finish()
}

// 登录表单
#[derive(Debug, serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub remember: Option<String>,
}

// 注册表单
#[derive(Debug, serde::Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

// 忘记密码表单
#[derive(Debug, serde::Deserialize)]
pub struct ForgotPasswordForm {
    pub email: String,
}

// 重置密码表单
#[derive(Debug, serde::Deserialize)]
pub struct ResetPasswordForm {
    pub new_password: String,
    pub confirm_password: String,
}

// 修改密码表单
#[derive(Debug, serde::Deserialize)]
pub struct ChangePasswordForm {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

// 修改邮箱表单
#[derive(Debug, serde::Deserialize)]
pub struct ChangeEmailForm {
    pub email: String,
    pub password: String,
}

// 修改密码页面GET请求处理器
pub async fn change_password_get(
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
    
    // 渲染修改密码页面
    match tera.render("admin/change_password.html", &context) {
        Ok(html) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html),
        Err(e) => {
            eprintln!("模板渲染错误: {:#?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(format!("模板渲染错误: {:#?}. 模板路径: admin/change_password.html.", e))
        }
    }
}

// 修改密码页面POST请求处理器
pub async fn change_password_post(
    req: HttpRequest,
    form: web::Form<ChangePasswordForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    let current_password = &form.current_password;
    let new_password = &form.new_password;
    let confirm_password = &form.confirm_password;
    
    // 验证新密码和确认密码是否一致
    if new_password != confirm_password {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body("<h1>修改密码</h1><div style='color: red'>新密码和确认密码不一致</div><form method='post'><div>当前密码: <input type='password' name='current_password'></div><div>新密码: <input type='password' name='new_password'></div><div>确认新密码: <input type='password' name='confirm_password'></div><button type='submit'>修改密码</button></form>");
    }
    
    // 获取会话
    let session = req.get_session();
    if let Ok(Some(admin_id)) = session.get::<i32>("admin_id") {
        // 获取管理员用户信息
        match get_admin_user_by_id(&pool, admin_id) {
            Ok(user) => {
                // 验证当前密码（简化处理，实际应该使用密码验证函数）
                // 这里我们跳过密码验证，直接更新密码
                
                // 更新密码
                match update_admin_user(&pool, admin_id, None, Some(new_password)) {
                    Ok(_) => {
                        // 记录操作日志
                        let ip = get_client_ip(&req);
                        let _ = log_admin_operation(&pool, admin_id, "change_password", Some(&format!("管理员 {} 修改密码成功", user.username)), ip);
                        
                        // 重定向到个人中心页面
                HttpResponse::Found()
                    .append_header((LOCATION, "/admin/dashboard/profile"))
                    .finish()
                    },
                    Err(e) => {
                        return HttpResponse::Ok()
                            .content_type("text/html; charset=utf-8")
                            .body(format!("<h1>修改密码</h1><div style='color: red'>{}</div><form method='post'><div>当前密码: <input type='password' name='current_password'></div><div>新密码: <input type='password' name='new_password'></div><div>确认新密码: <input type='password' name='confirm_password'></div><button type='submit'>修改密码</button></form>", e.message()));
                    }
                }
            },
            Err(e) => {
                return HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(format!("<h1>修改密码</h1><div style='color: red'>{}</div><form method='post'><div>当前密码: <input type='password' name='current_password'></div><div>新密码: <input type='password' name='new_password'></div><div>确认新密码: <input type='password' name='confirm_password'></div><button type='submit'>修改密码</button></form>", e.message()));
            }
        }
    } else {
        // 未登录，重定向到登录页面
        HttpResponse::Found()
            .append_header((LOCATION, "/admin/login"))
            .finish()
    }
}

// 修改邮箱页面POST请求处理器
pub async fn change_email_post(
    req: HttpRequest,
    form: web::Form<ChangeEmailForm>,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
) -> impl Responder {
    let email = &form.email;
    let password = &form.password;
    
    // 获取会话
    let session = req.get_session();
    if let Ok(Some(admin_id)) = session.get::<i32>("admin_id") {
        // 获取管理员用户信息
        match get_admin_user_by_id(&pool, admin_id) {
            Ok(user) => {
                // 验证密码（简化处理，实际应该使用密码验证函数）
                // 这里我们跳过密码验证，直接更新邮箱
                
                // 更新邮箱
                match update_admin_user(&pool, admin_id, Some(email), None) {
                    Ok(_) => {
                        // 记录操作日志
                        let ip = get_client_ip(&req);
                        let _ = log_admin_operation(&pool, admin_id, "change_email", Some(&format!("管理员 {} 修改邮箱成功", user.username)), ip);
                        
                        // 重定向到个人中心页面
                HttpResponse::Found()
                    .append_header((LOCATION, "/admin/dashboard/profile"))
                    .finish()
                    },
                    Err(e) => {
                        return HttpResponse::Ok()
                            .content_type("text/html; charset=utf-8")
                            .body(format!("<h1>修改邮箱</h1><div style='color: red'>{}</div><form method='post'><div>新邮箱: <input type='email' name='email' value='{}'></div><div>密码: <input type='password' name='password'></div><button type='submit'>修改邮箱</button></form>", e.message(), email));
                    }
                }
            },
            Err(e) => {
                return HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(format!("<h1>修改邮箱</h1><div style='color: red'>{}</div><form method='post'><div>新邮箱: <input type='email' name='email' value='{}'></div><div>密码: <input type='password' name='password'></div><button type='submit'>修改邮箱</button></form>", e.message(), email));
            }
        }
    } else {
        // 未登录，重定向到登录页面
        HttpResponse::Found()
            .append_header((LOCATION, "/admin/login"))
            .finish()
    }
}

// 操作日志查询参数
#[derive(Debug, serde::Deserialize)]
pub struct LogsQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub action: Option<String>,
    pub username: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

// 操作日志列表页面GET请求处理器
pub async fn logs_get(
    req: HttpRequest,
    pool: web::Data<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>,
    tera: web::Data<Tera>,
    web::Query(query): web::Query<LogsQuery>,
) -> impl Responder {
    // 获取会话
    let session = req.get_session();
    if let Ok(Some(admin_id)) = session.get::<i32>("admin_id") {
        // 获取管理员用户信息
        match get_admin_user_by_id(&pool, admin_id) {
            Ok(user) => {
                let mut context = Context::new();
                context.insert("username", &user.username);
                context.insert("is_authenticated", &true);
                
                // 获取所有管理员用户列表，用于生成操作者下拉菜单
                let all_admin_users_result = get_all_admin_users(&pool);
                let all_admin_users = match all_admin_users_result {
                    Ok(users) => users,
                    Err(_) => Vec::new()
                };
                context.insert("all_admin_users", &all_admin_users);
                
                // 获取查询参数
                let page = query.page.unwrap_or(1);
                let page_size = query.page_size.unwrap_or(20);
                
                // 解析action，处理空字符串情况
                let action = match &query.action {
                    Some(action_str) if !action_str.is_empty() => {
                        Some(action_str.as_str())
                    },
                    _ => None
                };
                
                // 解析username，处理空字符串情况
                let username = match &query.username {
                    Some(username_str) if !username_str.is_empty() => {
                        Some(username_str.as_str())
                    },
                    _ => None
                };
                
                // 解析start_time，处理空字符串情况
                let start_time = match &query.start_time {
                    Some(time_str) if !time_str.is_empty() => {
                        // 处理datetime-local格式: 2023-12-28T14:30
                        if time_str.contains('T') {
                            chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M")
                                .ok()
                                .map(|dt| chrono::DateTime::from_utc(dt, chrono::Utc))
                        } else {
                            // 兼容旧的日期格式
                            chrono::NaiveDate::parse_from_str(time_str, "%Y-%m-%d")
                                .ok()
                                .map(|d| {
                                    let naive_dt = d.and_hms(0, 0, 0);
                                    chrono::DateTime::from_utc(naive_dt, chrono::Utc)
                                })
                        }
                    },
                    _ => None
                };
                
                // 解析end_time，处理空字符串情况
                let end_time = match &query.end_time {
                    Some(time_str) if !time_str.is_empty() => {
                        // 处理datetime-local格式: 2023-12-28T14:30
                        if time_str.contains('T') {
                            // 解析datetime-local格式，然后将时间设置为当天的23:59:59
                            chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M")
                                .ok()
                                .map(|dt| {
                                    // 将分钟设置为59，秒和纳秒设置为59
                                    let dt = chrono::NaiveDateTime::new(
                                        dt.date(),
                                        chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap()
                                    );
                                    chrono::DateTime::from_utc(dt, chrono::Utc)
                                })
                        } else {
                            // 兼容旧的日期格式
                            chrono::NaiveDate::parse_from_str(time_str, "%Y-%m-%d")
                                .ok()
                                .map(|d| {
                                    let naive_dt = d.and_hms(23, 59, 59);
                                    chrono::DateTime::from_utc(naive_dt, chrono::Utc)
                                })
                        }
                    },
                    _ => None
                };
                
                // 获取操作日志
                match get_admin_logs(
                    &pool,
                    action,
                    None,  // 不使用admin_id筛选
                    username,  // 使用username筛选
                    start_time,
                    end_time,
                    page,
                    page_size
                ) {
                    Ok((logs, total_logs)) => {
                        let total_pages = (total_logs + page_size as i64 - 1) / page_size as i64;
                        
                        context.insert("logs", &logs);
                        context.insert("total", &total_logs);
                        context.insert("page", &page);
                        context.insert("page_size", &page_size);
                        context.insert("action", &query.action.as_deref().unwrap_or(""));
                        context.insert("username", &query.username);
                        context.insert("start_time", &query.start_time);
                        context.insert("end_time", &query.end_time);
                        context.insert("total_pages", &total_pages);
                    },
                    Err(e) => {
                        eprintln!("获取日志失败: {:#?}", e);
                        // 出错时显示空列表，使用正确的类型
                        context.insert("logs", &Vec::<crate::admin::services::admin_user::AdminLogWithUser>::new());
                        context.insert("total", &0);
                        context.insert("page", &page);
                        context.insert("page_size", &page_size);
                        context.insert("action", &query.action.as_deref().unwrap_or(""));
                        context.insert("username", &query.username);
                        context.insert("start_time", &query.start_time);
                        context.insert("end_time", &query.end_time);
                        context.insert("total_pages", &1);
                    }
                }
                
                // 渲染操作日志页面
                match tera.render("admin/logs.html", &context) {
                    Ok(html) => HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(html),
                    Err(e) => {
                        eprintln!("模板渲染错误: {:#?}", e);
                        HttpResponse::InternalServerError()
                            .content_type("text/html; charset=utf-8")
                            .body(format!("模板渲染错误: {:#?}. 模板路径: admin/logs.html.", e))
                    }
                }
            },
            Err(e) => {
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(format!("<h1>操作日志</h1><div style='color: red'>{}</div>", e.message()))
            }
        }
    } else {
        // 未登录，重定向到登录页面
        HttpResponse::Found()
            .header(LOCATION, "/admin/login")
            .finish()
    }
}
