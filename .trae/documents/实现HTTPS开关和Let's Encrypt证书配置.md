# 实现HTTPS开关和Let's Encrypt证书配置

## 1. 配置HTTPS开关

### 1.1 修改.env文件
- 添加HTTPS_ENABLED变量，用于控制是否启用HTTPS
- 添加HTTPS_CERT_PATH和HTTPS_KEY_PATH变量，用于指定证书文件路径
- 添加HTTPS_PORT变量，用于指定HTTPS端口

### 1.2 修改src/config.rs
- 在Config结构体中添加https_enabled、https_cert_path、https_key_path和https_port字段
- 在Config::new()方法中读取这些环境变量

### 1.3 修改src/main.rs
- 在HttpServer启动时，根据https_enabled配置决定是使用HTTP还是HTTPS
- 如果启用HTTPS，使用配置的证书文件路径

## 2. 配置Let's Encrypt DNS验证

### 2.1 修改docker-compose.yml
- 添加Certbot服务，用于请求Let's Encrypt证书
- 配置Certbot使用DNS验证方式
- 添加证书存储卷

### 2.2 修改nginx.conf
- 添加HTTPS服务器配置
- 配置SSL证书路径
- 添加HTTP到HTTPS的重定向（可选）

### 2.3 创建证书申请脚本
- 创建脚本用于生成Let's Encrypt证书
- 提示用户修改DNS记录以完成验证
- 自动部署证书到Nginx

## 3. 实现证书自动更新

### 3.1 添加Certbot自动更新服务
- 在docker-compose.yml中配置Certbot自动更新
- 配置更新后的钩子脚本，用于重新加载Nginx

## 4. 测试和验证

### 4.1 测试HTTPS开关功能
- 测试启用和禁用HTTPS时的服务器行为
- 验证证书路径配置是否正确

### 4.2 测试Let's Encrypt证书
- 测试证书申请流程
- 验证证书是否正确部署
- 测试证书自动更新

## 5. 文档更新

### 5.1 更新README.md
- 添加HTTPS配置说明
- 添加Let's Encrypt证书申请说明
- 更新Docker部署说明

### 5.2 更新.env.example
- 添加新的HTTPS相关环境变量示例

## 技术要点

- 使用Actix Web的HttpServer::bind_rustls()方法启用HTTPS
- 使用rustls库处理SSL
- 使用Certbot的dns-01验证方式
- 确保证书文件权限正确
- 实现平滑的证书更新机制

## 预期结果

- 服务器可以通过.env配置灵活启用或禁用HTTPS
- 支持使用Let's Encrypt证书，通过DNS验证获取
- 证书自动更新，无需手动干预
- 提供清晰的用户指引，帮助完成DNS验证
- 完整的文档说明