use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use futures::future::{ok, Either, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use crate::utils::ip::get_client_ip;

// 日志中间件
pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = RequestLoggerService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerService {
            service,
        })
    }
}

pub struct RequestLoggerService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            
            let duration = start.elapsed();
            let status = res.status();
            let req = res.request();
            let method = req.method();
            let uri = req.uri();
            let client_ip = get_client_ip(req).unwrap_or("0.0.0.0".to_string());
            let user_agent = req.headers().get(header::USER_AGENT)
                .map(|h| h.to_str().unwrap_or("unknown"))
                .unwrap_or("unknown");
            
            // 记录请求日志
            log::info!(
                "{} {} {} {} {:?} {:?}",
                client_ip,
                method,
                uri.path(),
                status.as_u16(),
                duration,
                user_agent
            );
            
            Ok(res)
        })
    }
}