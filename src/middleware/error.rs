use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use actix_web::middleware::Next;
use actix_web::body::BoxBody;
use log::error;
use crate::errors::AppError;

