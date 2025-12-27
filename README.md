# Rust+PostgreSQL用户验证和充值服务 

一款基于Rust和PostgreSQL开发的Linux服务器服务程序，用于软件用户的验证和充值工作。

## 功能特性

### 用户管理
- 用户注册、登录
- 密码加密保存
- VIP等级管理
- 登录日志记录
- 单设备登录限制

### 软件管理
- 软件列表管理
- VIP等级与软件关联
- 不同VIP等级使用不同软件
- 免费软件支持

### 充值系统
- 充值卡密管理
- 充值日志记录
- 自动更新VIP到期时间

### 心跳机制
- 客户端定期上传状态
- 后台清理不活跃用户

## 技术栈

- **开发语言**: Rust 2021
- **Web框架**: Actix Web 4.5.1
- **数据库**: PostgreSQL
- **ORM框架**: Diesel 2.2.1
- **异步支持**: Tokio
- **认证机制**: JWT
- **密码加密**: bcrypt
- **日志系统**: fern + log

## 项目结构

```
RLServer/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── database/            # 数据库相关代码
│   ├── handlers/            # 请求处理器
│   ├── middleware/          # 中间件
│   ├── services/            # 业务逻辑
│   ├── utils/               # 工具函数
│   ├── background/          # 后台任务
│   ├── schema.rs            # 数据库表结构
│   └── routes.rs            # 路由定义
├── migrations/              # 数据库迁移文件
├── Cargo.toml               # 项目依赖
├── .env                     # 环境变量配置
├── .env.example             # 环境变量示例
├── docker-compose.yml       # Docker Compose配置
├── nginx.conf               # Nginx配置
├── generate_ssl_cert.sh     # Linux/macOS SSL证书生成脚本
└── generate_ssl_cert.ps1    # Windows SSL证书生成脚本
```

## 环境配置

1. **安装Rust**: 请参考 [Rust官方网站](https://www.rust-lang.org/) 安装Rust环境
2. **安装PostgreSQL**: 安装并配置PostgreSQL数据库
3. **配置环境变量**: 复制 `.env.example` 为 `.env` 并修改相应配置

## 运行项目

### 开发模式

```bash
cargo run
```

### 生产模式

```bash
cargo build --release
./target/release/rlserver
```

## API文档

### 公开API

#### 用户注册
- **URL**: `/api/auth/register`
- **方法**: `POST`
- **请求体**:
  ```json
  {
    "username": "test_user",
    "password": "password123",
    "email": "user@example.com"
  }
  ```

#### 用户登录
- **URL**: `/api/auth/login`
- **方法**: `POST`
- **请求体**:
  ```json
  {
    "username": "test_user",
    "password": "password123",
    "hardware_code": "hw-123456",
    "software_version": "v1.0.0"
  }
  ```
- **响应示例**:
  ```json
  {
    "message": "Login successful",
    "token": "jwt-token",
    "vip_level": 1,
    "vip_expires_at": "2026-12-23T14:30:11Z"
  }
  ```

#### 心跳上传
- **URL**: `/api/heartbeat`
- **方法**: `POST`
- **请求体**:
  ```json
  {
    "session_token": "jwt-token",
    "hardware_code": "hw-123456",
    "software_version": "v1.0.0"
  }
  ```

#### 刷新访问令牌
- **URL**: `/api/auth/refresh`
- **方法**: `POST`
- **请求体**:
  ```json
  {
    "refresh_token": "refresh-token"
  }
  ```
- **响应示例**:
  ```json
  {
    "message": "Token refreshed successfully",
    "token": "new-jwt-token",
    "vip_level": 1,
    "vip_expires_at": "2026-12-23T14:30:11Z"
  }
  ```

#### 密码重置请求
- **URL**: `/api/auth/reset-password`
- **方法**: `POST`
- **请求体**:
  ```json
  {
    "email": "user@example.com"
  }
  ```

#### 验证密码重置令牌并更新密码
- **URL**: `/api/auth/verify-reset-password`
- **方法**: `POST`
- **请求体**:
  ```json
  {
    "token": "reset-token",
    "new_password": "new-password123"
  }
  ```

### 受保护API (需要认证)

所有受保护API需要在请求头中添加 `Authorization: Bearer <token>`

#### 获取当前用户信息
- **URL**: `/api/protected/users/me`
- **方法**: `GET`

#### 获取可用软件列表
- **URL**: `/api/protected/users/software`
- **方法**: `GET`
- **响应示例**:
  ```json
  {
    "vip_level": 1,
    "vip_expires_at": "2026-12-23T14:30:11Z",
    "software_list": [
      {
        "id": 1,
        "name": "software_a",
        "chinese_name": "软件A",
        "description": "这是软件A的简介",
        "detailed_description": "这是软件A的详细介绍，包含软件的功能、特点等信息",
        "executable_name": "software_a.exe",
        "md5_checksum": "1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t",
        "requires_admin": false,
        "required_vip_level": 0,
        "created_at": "2025-12-23T14:30:11Z",
        "updated_at": "2025-12-23T14:30:11Z"
      }
    ]
  }
  ```

#### 用户登出
- **URL**: `/api/auth/logout`
- **方法**: `POST`

#### 卡密充值
- **URL**: `/api/protected/recharge`
- **方法**: `POST`
- **请求体**:
  ```json
  {
    "card_code": "RC-123456-7890"
  }
  ```
- **响应示例**:
  ```json
  {
    "message": "Recharge successful",
    "vip_level": 1,
    "vip_expires_at": "2026-12-23T14:30:11Z",
    "recharge_log": {
      "id": 1,
      "user_id": 1,
      "card_code": "RC-123456-7890",
      "vip_level": 1,
      "duration_days": 30,
      "recharge_time": "2025-12-23T14:30:11Z",
      "created_at": "2025-12-23T14:30:11Z"
    }
  }
  ```

#### 获取充值记录
- **URL**: `/api/protected/recharge/logs`
- **方法**: `GET`

#### 获取所有软件列表
- **URL**: `/api/protected/software`
- **方法**: `GET`

#### 检查软件访问权限
- **URL**: `/api/protected/software/{software_id}/access`
- **方法**: `GET`

## 数据库表结构

### users (用户表)
- id: 主键
- username: 用户名
- password_hash: 密码哈希
- email: 邮箱
- email_verified: 邮箱是否已验证
- vip_level: VIP等级
- vip_expires_at: VIP到期时间
- last_login_at: 最后登录时间
- last_login_hardware: 最后登录硬件码
- last_login_version: 最后登录软件版本
- last_login_ip: 最后登录IP
- last_logout_at: 最后登出时间
- created_at: 创建时间
- updated_at: 更新时间

### verification_codes (验证码表)
- id: 主键
- user_id: 用户ID
- email: 邮箱
- code: 验证码
- expires_at: 过期时间
- used: 是否已使用
- created_at: 创建时间

### software (软件表)
- id: 主键
- name: 软件名称
- chinese_name: 软件中文名
- description: 软件简介
- detailed_description: 软件详细介绍
- executable_name: 可执行程序名
- md5_checksum: MD5校验码
- requires_admin: 是否需要管理员权限
- required_vip_level: 所需VIP等级 (0表示免费)
- created_at: 创建时间
- updated_at: 更新时间

### recharge_cards (充值卡密表)
- id: 主键
- card_code: 卡密
- amount: 金额
- vip_level: 充值后获得的VIP等级
- duration_days: 有效天数
- is_used: 是否已使用
- used_at: 使用时间
- used_by: 使用用户ID
- created_at: 创建时间

### recharge_logs (充值日志表)
- id: 主键
- user_id: 用户ID
- card_code: 卡密
- vip_level: 获得的VIP等级
- duration_days: 增加的天数
- recharge_time: 充值时间
- created_at: 创建时间

### login_logs (登录日志表)
- id: 主键
- user_id: 用户ID
- login_time: 登录时间
- hardware_code: 硬件码
- software_version: 软件版本
- ip_address: IP地址
- status: 登录状态
- created_at: 创建时间

### online_users (在线用户表)
- id: 主键
- user_id: 用户ID
- session_token: 会话令牌
- login_time: 登录时间
- hardware_code: 硬件码
- software_version: 软件版本
- ip_address: IP地址
- last_activity_at: 最后活动时间
- status_interval: 状态上传间隔（分钟）
- created_at: 创建时间

## 单设备登录实现

1. 用户登录时，生成唯一的会话令牌
2. 将用户的会话信息存储到 `online_users` 表中
3. 如果该用户已有其他会话，删除旧会话记录
4. 客户端每次请求携带会话令牌
5. 心跳机制定期更新用户活动时间
6. 后台任务清理不活跃用户

## 心跳机制

1. 客户端每10分钟（可配置）发送一次心跳请求
2. 服务器更新用户的最后活动时间
3. 后台任务每5分钟（可配置）清理超过10分钟未活动的用户

## 部署

### Docker部署

#### 1. 准备工作

- 确保已安装Docker和Docker Compose
- 克隆项目代码
- 进入项目目录

#### 2. SSL证书生成

服务支持HTTPS，需要生成SSL证书：

1. 确保 `ssl` 目录存在
2. 生成自签名证书或使用真实证书
3. 证书文件需命名为：
   - `fullchain.pem`: 证书文件（包含完整证书链）
   - `privkey.pem`: 私钥文件

**注意**: 在生产环境中，请使用真实的SSL证书（如Let's Encrypt）

#### 3. 配置文件

- `docker-compose.yml`: 包含服务配置，SSL证书卷挂载和Nginx配置
- `.env`: 包含环境变量配置，根据 `.env.example` 修改
- `nginx.conf`: Nginx配置文件，包含HTTP到HTTPS重定向和反向代理配置

#### 4. 启动服务

```bash
docker-compose up -d --build
```

#### 5. 访问服务

- **HTTP**: http://localhost:28001（会自动重定向到HTTPS）
- **HTTPS**: https://localhost:28043（推荐使用）

#### 6. 服务架构

```
┌───────────────────┐     ┌───────────────────┐     ┌───────────────────┐
│   客户端浏览器/应用   │────▶│        Nginx        │────▶│     Rust后端服务      │
└───────────────────┘     └───────────────────┘     └───────────────────┘
                                    │                         │
                                    │                         │
                                    ▼                         ▼
                            ┌───────────────────┐     ┌───────────────────┐
                            │      SSL证书      │     │    PostgreSQL数据库  │
                            └───────────────────┘     └───────────────────┘
```

#### 7. SSL证书说明

- 证书文件位于 `ssl` 目录
- Nginx自动加载证书文件
- 证书已配置为受信任，浏览器不会显示SSL警告

### 手动部署

#### 1. 安装依赖

- 安装Rust和Cargo
- 安装PostgreSQL

#### 2. 配置数据库

- 创建数据库
- 运行迁移脚本

#### 3. 配置环境变量

复制 `.env.example` 为 `.env` 并修改配置

#### 4. 构建和运行

```bash
# 构建
cargo build --release

# 运行
./target/release/rlserver
```

#### 5. Nginx配置

配置Nginx作为反向代理，启用HTTPS

```nginx
server {
    listen 80;
    server_name localhost;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name localhost;
    
    ssl_certificate /path/to/fullchain.pem;
    ssl_certificate_key /path/to/privkey.pem;
    
    location / {
        proxy_pass http://localhost:28001;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

### 环境变量配置

主要环境变量说明：

| 变量名 | 说明 | 默认值 |
|-------|------|--------|
| DATABASE_URL | 数据库连接URL | postgres://admin:password@db:5432/rl_server |
| JWT_SECRET | JWT签名密钥 | your-secret-key-here |
| SERVER_PORT | 服务器端口 | 28001 |
| HEARTBEAT_INTERVAL | 心跳间隔（秒） | 600 |
| CLEANUP_INTERVAL | 清理间隔（秒） | 300 |
| HTTPS_ENABLED | 是否启用HTTPS | false |
| HTTPS_CERT_PATH | HTTPS证书文件路径 | ./ssl/cert.pem |
| HTTPS_KEY_PATH | HTTPS私钥文件路径 | ./ssl/key.pem |
| HTTPS_PORT | HTTPS端口 | 28043 |
| PASSWORD_MIN_LENGTH | 密码最小长度 | 8 |
| PASSWORD_REQUIRE_UPPERCASE | 是否要求大写字母 | true |
| PASSWORD_REQUIRE_LOWERCASE | 是否要求小写字母 | true |
| PASSWORD_REQUIRE_DIGIT | 是否要求数字 | true |
| PASSWORD_REQUIRE_SPECIAL | 是否要求特殊字符 | true |

## 日志管理

日志默认输出到控制台，格式如下：

```
[2025-12-22T22:00:00Z] [rlserver] [INFO] Starting server on port 28001
```

## 监控与维护

1. 定期备份数据库
2. 监控服务运行状态
3. 检查日志文件
4. 定期更新依赖

## 开发说明

### 代码风格

- 使用 `rustfmt` 进行代码格式化
- 使用 `clippy` 进行代码检查

### 测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
```

## 许可证

MIT
