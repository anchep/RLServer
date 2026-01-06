use actix_web::HttpRequest;
use log::{debug, info};

/// 从请求中提取真实客户端IP地址
/// 优先检查 X-Forwarded-For 头，然后是 X-Real-IP 头，最后是连接信息中的远程地址
pub fn get_client_ip(req: &HttpRequest) -> Option<String> {
    // 调试日志：记录所有相关头信息
    debug!("Request headers: {:?}", req.headers());
    
    // 开发环境中，直接从请求头中获取真实IP地址
    // 检查 X-Forwarded-For 头，获取第一个IP地址
    if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            debug!("X-Forwarded-For header value: '{}'", forwarded_str);
            // X-Forwarded-For 格式: client, proxy1, proxy2
            // 获取第一个IP地址
            if let Some(first_ip) = forwarded_str.split(',').next().map(|s| s.trim()) {
                if !first_ip.is_empty() {
                    debug!("Extracted first IP from X-Forwarded-For: '{}'", first_ip);
                    return Some(first_ip.to_string());
                }
            }
        }
    } else {
        debug!("X-Forwarded-For header not found");
    }
    
    // 检查 X-Real-IP 头
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            debug!("X-Real-IP header value: '{}'", real_ip_str);
            if !real_ip_str.is_empty() {
                debug!("Using X-Real-IP: '{}'", real_ip_str);
                return Some(real_ip_str.to_string());
            }
        }
    } else {
        debug!("X-Real-IP header not found");
    }
    
    // 最后检查连接信息中的远程地址
    if let Some(remote_addr) = req.connection_info().remote_addr() {
        debug!("Using remote_addr from connection_info: '{}'", remote_addr);
        Some(remote_addr.to_string())
    } else {
        debug!("remote_addr not found in connection_info");
        None
    }
}