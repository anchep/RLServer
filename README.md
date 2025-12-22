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
RustServer/
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
└── .env.example             # 环境变量示例
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
./target/release/rust-server
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
    "password": "password123"
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

### 受保护API (需要认证)

所有受保护API需要在请求头中添加 `Authorization: Bearer <token>`

#### 获取当前用户信息
- **URL**: `/api/protected/users/me`
- **方法**: `GET`

#### 获取可用软件列表
- **URL**: `/api/protected/users/software`
- **方法**: `GET`

#### 用户登出
- **URL**: `/api/protected/users/logout`
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
- vip_level: VIP等级
- vip_expires_at: VIP到期时间
- last_login_at: 最后登录时间
- last_login_hardware: 最后登录硬件码
- last_login_version: 最后登录软件版本
- last_login_ip: 最后登录IP
- created_at: 创建时间
- updated_at: 更新时间

### software (软件表)
- id: 主键
- name: 软件名称
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

1. 确保已安装Docker和Docker Compose
2. 创建 `docker-compose.yml` 文件（示例已提供）
3. 运行以下命令启动服务：

```bash
docker-compose up -d
```

## 日志管理

日志默认输出到控制台，格式如下：

```
[2025-12-22T22:00:00Z] [rust_server] [INFO] Starting server on port 28001
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
