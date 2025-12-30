use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse, HttpMessage};
use actix_session::{Session, SessionExt};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

// 后台认证中间件
pub struct AdminAuth;

impl<S> Transform<S, ServiceRequest> for AdminAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AdminAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminAuthMiddleware { service }))
    }
}

#[derive(Clone)]
pub struct AdminAuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AdminAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        
        match session.get::<i32>("admin_id") {
            Ok(Some(admin_id)) => {
                // 已认证，将admin_id放入请求扩展中
                req.extensions_mut().insert(admin_id);
                // 继续处理请求
                let fut = self.service.call(req);
                Box::pin(async move { fut.await })
            },
            _ => {
                // 会话获取失败或没有管理员ID
                // 构建重定向响应
                let response = HttpResponse::Found()
                    .append_header((actix_web::http::header::LOCATION, "/admin/login"))
                    .finish();
                // 转换为正确的 ServiceResponse 类型
                let res = req.into_response(response);
                Box::pin(async move { Ok(res) })
            }
        }
    }
}


