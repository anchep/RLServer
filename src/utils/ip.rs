use actix_web::HttpRequest;

/// 从请求中提取真实客户端IP地址
/// 优先检查 X-Forwarded-For 头，然后是 X-Real-IP 头，最后是连接信息中的远程地址
pub fn get_client_ip(req: &HttpRequest) -> Option<String> {
    // 检查 X-Forwarded-For 头，获取第一个IP地址
    if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            // X-Forwarded-For 格式: client, proxy1, proxy2
            // 获取第一个IP地址
            if let Some(first_ip) = forwarded_str.split(',').next().map(|s| s.trim()) {
                if !first_ip.is_empty() {
                    return Some(first_ip.to_string());
                }
            }
        }
    }
    
    // 检查 X-Real-IP 头
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            if !real_ip_str.is_empty() {
                return Some(real_ip_str.to_string());
            }
        }
    }
    
    // 最后检查连接信息中的远程地址
    req.connection_info().remote_addr().map(|s| s.to_string())
}