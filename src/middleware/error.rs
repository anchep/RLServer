use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use actix_web::middleware::Next;
use actix_web::body::BoxBody;
use log::error;
use crate::errors::AppError;

/// 错误处理中间件
pub async fn error_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    next.call(req).await
}