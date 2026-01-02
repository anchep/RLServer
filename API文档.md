# API文档

## 项目概述
本项目是一个基于Rust + Actix Web的后台管理系统，提供了用户管理、软件管理、卡密管理等功能。

## 技术栈
- **后端框架**：Actix Web
- **ORM**：Diesel
- **数据库**：PostgreSQL
- **模板引擎**：Tera
- **认证**：JWT
- **部署**：Docker Compose

## 基础路径
所有API请求的基础路径为：`http://localhost:28001`

## 认证机制

### JWT Token
本项目使用JWT（JSON Web Token）进行认证。登录成功后，服务器会返回两个token：
- **access_token**：用于访问受保护的API，有效期较短
- **refresh_token**：用于刷新access_token，有效期较长

### 请求头
访问受保护API时，需要在请求头中包含Authorization字段：
```
Authorization: Bearer <access_token>
```

## 错误处理

所有API错误都会返回统一的错误格式：
```json
{
  "error": "错误信息"
}
```

## 状态码
- **200**：请求成功
- **400**：请求参数错误
- **401**：未认证或认证失败
- **403**：没有权限访问该资源
- **404**：资源不存在
- **500**：服务器内部错误

## API参考

### 1. 健康检查

#### 1.1 检查服务状态
- **URL**：`/health`
- **方法**：`GET`
- **认证**：不需要
- **功能**：检查服务是否正常运行
- **响应示例**：
  ```json
  "ok"
  ```
- **状态码**：
  - 200：服务正常

### 2. 认证相关API

#### 2.1 用户注册
- **URL**：`/api/auth/register`
- **方法**：`POST`
- **认证**：不需要
- **功能**：注册新用户
- **请求体**：
  ```json
  {
    "username": "user123",
    "email": "user@example.com",
    "password": "a1234567",
    "hardware_code": "hard-123456",
    "software_version": "v1.0.0"
  }
  ```
- **请求参数说明**：
  - username：String，3-20个字符
  - email：String，有效的邮箱格式
  - password：String，至少8个字符
  - hardware_code：String，1-100个字符，设备硬件标识符
  - software_version：String，客户端软件版本
- **响应示例**：
  ```json
  {
    "message": "User registered successfully, please verify your email",
    "activation_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
  ```
- **状态码**：
  - 200：注册成功
  - 400：参数错误或用户已存在

#### 2.2 用户登录
- **URL**：`/api/auth/login`
- **方法**：`POST`
- **认证**：不需要
- **功能**：用户登录，获取访问令牌
- **请求体**：
  ```json
  {
    "username": "user123",
    "password": "a1234567",
    "hardware_code": "hard-123456",
    "software_version": "v1.0.0",
    "ip_address": "192.168.1.100"
  }
  ```
- **请求参数说明**：
  - username：String，用户名
  - password：String，密码
  - hardware_code：String，设备硬件标识符
  - software_version：String，客户端软件版本
  - ip_address：String，用户自行上传的IP地址
- **响应示例**：
  ```json
  {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "vip_level": 1,
    "vip_expires_at": "2025-12-31T23:59:59Z"
  }
  ```
- **状态码**：
  - 200：登录成功
  - 401：用户名或密码错误

#### 2.3 刷新令牌
- **URL**：`/api/auth/refresh`
- **方法**：`POST`
- **认证**：不需要
- **功能**：使用刷新令牌获取新的访问令牌
- **请求体**：
  ```json
  {
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
  ```
- **请求参数说明**：
  - refresh_token：String，刷新令牌
- **响应示例**：
  ```json
  {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "vip_level": 1,
    "vip_expires_at": "2025-12-31T23:59:59Z"
  }
  ```
- **状态码**：
  - 200：刷新成功
  - 401：刷新令牌无效

#### 2.4 发送密码重置邮件
- **URL**：`/api/auth/reset-password`
- **方法**：`POST`
- **认证**：不需要
- **功能**：发送密码重置邮件
- **请求体**：
  ```json
  {
    "email": "user@example.com"
  }
  ```
- **请求参数说明**：
  - email：String，有效的邮箱格式
- **响应示例**：
  ```json
  {
    "message": "Password reset email sent successfully"
  }
  ```
- **状态码**：
  - 200：邮件发送成功
  - 404：邮箱不存在

#### 2.5 验证重置密码
- **URL**：`/api/auth/reset-password/verify`
- **方法**：`POST`
- **认证**：不需要
- **功能**：验证重置密码验证码并更新密码
- **请求体**：
  ```json
  {
    "username": "user123",
    "email": "user@example.com",
    "code": "123456",
    "new_password": "newpassword123"
  }
  ```
- **请求参数说明**：
  - username：String，用户名
  - email：String，有效的邮箱格式
  - code：String，6位验证码
  - new_password：String，至少8个字符
- **响应示例**：
  ```json
  {
    "message": "Password reset successfully"
  }
  ```
- **状态码**：
  - 200：密码重置成功
  - 400：验证码无效或已过期

#### 2.6 退出登录
- **URL**：`/api/auth/logout`
- **方法**：`POST`
- **认证**：不需要
- **功能**：使当前session_token失效
- **请求体**：
  ```json
  {
    "session_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
  ```
- **请求参数说明**：
  - session_token：String，会话令牌
- **响应示例**：
  ```json
  {
    "message": "Logout successful"
  }
  ```
- **状态码**：
  - 200：退出成功

#### 2.7 邮箱验证
- **URL**：`/api/auth/verify-email`
- **方法**：`POST`
- **认证**：不需要
- **功能**：验证邮箱
- **请求体**：
  ```json
  {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "code": "123456"
  }
  ```
- **请求参数说明**：
  - token：String，激活令牌
  - code：String，6位验证码
- **响应示例**：
  ```json
  {
    "message": "Email verified successfully"
  }
  ```
- **状态码**：
  - 200：验证成功
  - 400：令牌或验证码无效

### 3. 心跳API

#### 3.1 保持连接活跃
- **URL**：`/api/heartbeat`
- **方法**：`POST`
- **认证**：不需要
- **功能**：保持连接活跃，更新用户在线状态
- **请求体**：
  ```json
  {
    "session_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "hardware_code": "hard-123456",
    "software_version": "v1.0.0"
  }
  ```
- **请求参数说明**：
  - session_token：String，会话令牌
  - hardware_code：String，设备硬件标识符
  - software_version：String，客户端软件版本
- **响应示例**：
  ```json
  {
    "message": "Heartbeat updated successfully"
  }
  ```
- **状态码**：
  - 200：心跳更新成功
  - 400：无效令牌

### 4. 受保护的API（需要认证）

#### 4.1 获取当前用户信息
- **URL**：`/api/protected/users/me`
- **方法**：`GET`
- **认证**：需要
- **功能**：获取当前登录用户的信息
- **响应示例**：
  ```json
  {
    "id": 1,
    "username": "user123",
    "email": "user@example.com",
    "vip_level": 1,
    "vip_end_time": "2025-12-31T23:59:59Z",
    "created_at": "2025-01-01T00:00:00Z",
    "updated_at": "2025-01-01T00:00:00Z",
    "is_active": true
  }
  ```
- **状态码**：
  - 200：获取成功
  - 401：未认证

#### 4.2 获取用户可用软件
- **URL**：`/api/protected/users/software`
- **方法**：`GET`
- **认证**：需要
- **功能**：获取用户可用的软件列表
- **响应示例**：
  ```json
  [
    {
      "id": 1,
      "name": "Software1",
      "chinese_name": "软件1",
      "description": "这是软件1",
      "version": "1.0.0",
      "is_active": true
    },
    {
      "id": 2,
      "name": "Software2",
      "chinese_name": "软件2",
      "description": "这是软件2",
      "version": "2.0.0",
      "is_active": true
    }
  ]
  ```
- **状态码**：
  - 200：获取成功
  - 401：未认证

#### 4.3 使用卡密充值
- **URL**：`/api/protected/recharge`
- **方法**：`POST`
- **认证**：需要
- **功能**：使用卡密为用户充值
- **请求体**：
  ```json
  {
    "card_code": "CARD-123456789"
  }
  ```
- **请求参数说明**：
  - card_code：String，卡密代码
- **响应示例**：
  ```json
  {
    "message": "Recharge successful",
    "vip_level": 2,
    "vip_expires_at": "2026-01-01T00:00:00Z",
    "recharge_log": {
      "id": 1,
      "card_code": "CARD-123456789",
      "amount": 30,
      "created_at": "2025-01-01T00:00:00Z"
    }
  }
  ```
- **状态码**：
  - 200：充值成功
  - 400：卡密无效或已使用
  - 401：未认证

#### 4.4 获取用户充值记录
- **URL**：`/api/protected/recharge/logs`
- **方法**：`GET`
- **认证**：需要
- **功能**：获取用户的充值记录
- **响应示例**：
  ```json
  [
    {
      "id": 1,
      "card_code": "CARD-123456789",
      "amount": 30,
      "created_at": "2025-01-01T00:00:00Z"
    },
    {
      "id": 2,
      "card_code": "CARD-987654321",
      "amount": 15,
      "created_at": "2024-12-01T00:00:00Z"
    }
  ]
  ```
- **状态码**：
  - 200：获取成功
  - 401：未认证

#### 4.5 获取所有软件
- **URL**：`/api/protected/software`
- **方法**：`GET`
- **认证**：需要
- **功能**：获取所有软件列表
- **响应示例**：
  ```json
  [
    {
      "id": 1,
      "name": "Software1",
      "chinese_name": "软件1",
      "description": "这是软件1",
      "version": "1.0.0",
      "is_active": true
    },
    {
      "id": 2,
      "name": "Software2",
      "chinese_name": "软件2",
      "description": "这是软件2",
      "version": "2.0.0",
      "is_active": true
    }
  ]
  ```
- **状态码**：
  - 200：获取成功
  - 401：未认证

#### 4.6 检查软件访问权限
- **URL**：`/api/protected/software/{software_id}/access`
- **方法**：`GET`
- **认证**：需要
- **功能**：检查用户是否有权限访问指定软件
- **路径参数**：
  - software_id：Integer，软件ID
- **响应示例**：
  ```json
  {
    "has_access": true,
    "message": "You have access to this software"
  }
  ```
- **状态码**：
  - 200：检查成功
  - 401：未认证
  - 403：没有访问权限
  - 404：软件不存在

## 部署流程

### 1. 环境要求
- Docker 27.0+
- Docker Compose 2.29+
- Rust 1.91.1 (stable)

### 2. 部署步骤

#### 2.1 克隆代码
```bash
git clone <仓库地址>
cd RLServer
```

#### 2.2 配置环境变量
```bash
cp .env.example .env
# 修改.env文件中的配置，包括数据库连接、JWT密钥等
```

#### 2.3 配置Nginx反向代理（获取真实IP）
1. 确保`nginx.conf`文件中配置了`realip_module`：
   ```nginx
   set_real_ip_from 172.0.0.0/8;
   set_real_ip_from 192.168.0.0/16;
   real_ip_header X-Forwarded-For;
   real_ip_recursive on;
   ```

2. 配置宿主机公网IP（可选，用于外部访问）：
   - Linux系统：在`docker-compose.yml`中使用`HOST_PUBLIC_IP=$(curl -s ifconfig.me)`
   - Windows系统：手动获取公网IP并设置到环境变量中

#### 2.4 构建并启动服务
```bash
docker-compose up --build -d
```

#### 2.5 创建管理员用户
```bash
docker-compose exec app cargo run --bin create_admin
```

### 3. 访问应用
- **后台管理**：`http://localhost:28001/admin/login`
- **API文档**：`http://localhost:28001/API文档.md`

## 项目结构

```
RLServer/
├── src/                    # 源代码目录
│   ├── admin/              # 后台管理相关代码
│   ├── database/           # 数据库相关代码
│   ├── handlers/           # 请求处理器
│   ├── middleware/         # 中间件
│   ├── services/           # 业务逻辑
│   ├── utils/              # 工具函数
│   ├── background/         # 后台任务
│   ├── routes.rs           # API路由配置
│   ├── main.rs             # 应用入口
├── templates/              # 模板文件
├── static/                 # 静态资源
├── migrations/             # 数据库迁移文件
├── .env                    # 环境变量配置
├── .env.example            # 环境变量示例
├── docker-compose.yml      # Docker服务配置
├── nginx.conf              # Nginx配置
├── Dockerfile              # Rust应用构建配置
├── Cargo.toml              # Rust依赖配置
├── 开发环境.md             # 开发环境配置文档
└── API文档.md              # API文档
```

## 安全注意事项

1. **密码安全**：所有密码都使用bcrypt进行哈希存储
2. **JWT密钥**：请确保在生产环境中使用强密钥
3. **HTTPS**：建议在生产环境中启用HTTPS
4. **权限控制**：所有敏感操作都需要相应的权限
5. **SQL注入防护**：使用Diesel ORM避免SQL注入
6. **XSS防护**：使用Tera模板引擎自动转义HTML

## 更新日志

### v1.0.0
- 初始版本，包含基本功能
- 用户管理、软件管理、卡密管理
- 后台管理系统
- JWT认证
- Docker部署

### v1.1.0
- 添加了卡密时长筛选功能
- 添加了选中卡密导出功能
- 优化了API文档
- 修复了若干bug

### v1.1.1
- 修复了复制卡密按钮重定向到批量删除的问题
- 完善了真实IP地址获取机制，添加了Nginx realip_module配置
- 优化了卡密列表筛选功能，支持更多时长选项
- 更新了部署流程，添加了IP配置说明
- 修复了Windows环境变量语法错误

## 联系方式

如有问题或建议，请联系：
- 邮箱：<邮箱地址>
- GitHub：<GitHub地址>
