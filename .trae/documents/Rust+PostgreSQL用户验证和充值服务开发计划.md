# Rust+PostgreSQL用户验证和充值服务开发计划

## 项目结构

```
RustServer/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── config.rs            # 配置管理
│   ├── database/
│   │   ├── mod.rs           # 数据库模块入口
│   │   ├── connection.rs    # 数据库连接管理
│   │   └── models.rs        # 数据模型定义
│   ├── handlers/
│   │   ├── mod.rs           # 处理器模块入口
│   │   ├── auth.rs          # 认证相关处理器
│   │   ├── user.rs          # 用户管理处理器
│   │   ├── recharge.rs      # 充值相关处理器
│   │   ├── software.rs      # 软件管理处理器
│   │   └── heartbeat.rs     # 心跳处理处理器
│   ├── middleware/
│   │   ├── mod.rs           # 中间件模块入口
│   │   ├── auth.rs          # 认证中间件
│   │   └── logger.rs        # 日志中间件
│   ├── services/
│   │   ├── mod.rs           # 服务模块入口
│   │   ├── auth.rs          # 认证服务
│   │   ├── user.rs          # 用户服务
│   │   ├── recharge.rs      # 充值服务
│   │   ├── software.rs      # 软件服务
│   │   └── heartbeat.rs     # 心跳服务
│   ├── utils/
│   │   ├── mod.rs           # 工具模块入口
│   │   ├── crypto.rs        # 加密工具
│   │   ├── jwt.rs           # JWT工具
│   │   └── logger.rs        # 日志工具
│   ├── background/          # 后台任务
│   │   ├── mod.rs           # 后台任务模块入口
│   │   └── inactive_cleanup.rs # 清理不活跃用户任务
│   └── routes.rs            # 路由定义
├── migrations/              # 数据库迁移文件
├── Dockerfile               # Docker构建文件
├── docker-compose.yml       # Docker Compose配置
├── Cargo.toml               # 项目依赖配置
└── .env.example             # 环境变量示例
```

## 核心功能实现

### 1. 数据库设计

**用户表 (users)**

* id: 主键

* username: 用户名

* password\_hash: 密码哈希

* vip\_level: VIP等级 (1-10)

* vip\_expires\_at: VIP到期时间

* last\_login\_at: 最后登录时间

* last\_login\_hardware: 最后登录硬件码

* last\_login\_version: 最后登录软件版本

* last\_login\_ip: 最后登录IP

* created\_at: 创建时间

* updated\_at: 更新时间

**软件表 (software)**

* id: 主键

* name: 软件名称

* required\_vip\_level: 所需VIP等级 (0表示免费)

* created\_at: 创建时间

* updated\_at: 更新时间

**充值卡密表 (recharge\_cards)**

* id: 主键

* card\_code: 卡密

* amount: 金额/天数

* vip\_level: 充值后获得的VIP等级

* duration\_days: 有效天数

* is\_used: 是否已使用

* used\_at: 使用时间

* used\_by: 使用用户ID

* created\_at: 创建时间

**充值日志表 (recharge\_logs)**

* id: 主键

* user\_id: 用户ID

* card\_code: 卡密

* vip\_level: 获得的VIP等级

* duration\_days: 增加的天数

* recharge\_time: 充值时间

* created\_at: 创建时间

**登录日志表 (login\_logs)**

* id: 主键

* user\_id: 用户ID

* login\_time: 登录时间

* hardware\_code: 硬件码

* software\_version: 软件版本

* ip\_address: IP地址

* status: 登录状态

* created\_at: 创建时间

**在线用户表 (online\_users)**

* id: 主键

* user\_id: 用户ID

* session\_token: 会话令牌

* login\_time: 登录时间

* hardware\_code: 硬件码

* software\_version: 软件版本

* ip\_address: IP地址

* last\_activity\_at: 最后活动时间

* status\_interval: 状态上传间隔（分钟）

* created\_at: 创建时间

### 2. 核心功能模块

#### 2.1 认证模块

* 用户注册

* 用户登录 (新登录踢掉旧客户端)

* 密码加密 (使用bcrypt)

* JWT生成与验证

* 单设备登录限制

#### 2.2 用户管理模块

* 获取用户信息

* 更新用户信息

* 检查VIP状态

* 获取可用软件列表

#### 2.3 充值模块

* 卡密充值

* 充值记录查询

* 自动更新VIP到期时间

* 返回新的VIP等级和到期时间

#### 2.4 软件管理模块

* 获取软件列表

* 检查用户是否有权限使用软件

#### 2.5 日志模块

* 注册日志

* 登录日志

* 充值日志

#### 2.6 心跳模块

* 客户端每10分钟上传一次状态信息

* 更新用户最后活动时间

* 支持配置状态上传间隔

#### 2.7 后台任务

* 定期清理不活跃用户（超过配置间隔无心跳）

* 维护在线用户表的准确性

### 3. API设计

#### 认证相关

* POST /api/auth/register - 用户注册

* POST /api/auth/login - 用户登录

* POST /api/auth/logout - 用户登出

#### 用户相关

* GET /api/users/me - 获取当前用户信息

* GET /api/users/software - 获取可用软件列表

#### 充值相关

* POST /api/recharge - 卡密充值

* GET /api/recharge/logs - 获取充值记录

#### 软件相关

* GET /api/software - 获取所有软件列表

#### 心跳相关

* POST /api/heartbeat - 上传状态信息

### 4. 关键流程设计

#### 4.1 新设备登录流程

1. 客户端发送登录请求
2. 服务器验证用户名密码
3. 检查在线用户表，删除该用户所有现有会话
4. 创建新会话，记录到在线用户表
5. 返回JWT和新会话信息
6. 其他客户端的旧会话失效，收到下线提示

#### 4.2 心跳处理流程

1. 客户端定期（默认10分钟）发送心跳请求，包含session\_token
2. 服务器验证session\_token有效性
3. 更新在线用户表中的last\_activity\_at字段
4. 返回成功响应

#### 4.3 不活跃用户清理流程

1. 后台任务每5分钟运行一次
2. 查询在线用户表中last\_activity\_at超过配置间隔的记录
3. 删除这些记录，标记用户为离线
4. 记录相关日志

#### 4.4 充值流程

1. 客户端发送充值请求，包含卡密和用户认证信息
2. 服务器验证卡密有效性
3. 检查卡密是否已被使用
4. 标记卡密为已使用
5. 记录充值日志
6. 更新用户VIP等级和到期时间

   * 如果用户当前没有VIP或VIP已过期，从当前时间开始计算

   * 如果用户当前有VIP且未过期，从原有到期时间开始累加
7. 返回新的VIP等级和到期时间给客户端
8. 更新在线用户信息

### 5. 技术栈

* **Rust** - 后端开发语言

* **Actix Web** - Web框架

* **Diesel** - ORM框架

* **PostgreSQL** - 数据库

* **JWT** - 认证机制

* **bcrypt** - 密码加密

* **dotenv** - 环境变量管理

* **log/fern** - 日志系统

* **Tokio** - 异步运行时

* **Docker** - 容器化部署

* **Docker Compose** - 服务编排

### 6. 开发步骤

1. 初始化Rust项目
2. 配置依赖项
3. 创建数据库模型和迁移
4. 实现数据库连接管理
5. 实现工具函数 (加密、JWT、日志等)
6. 实现核心服务层
7. 实现API处理器
8. 配置路由
9. 添加中间件
10. 实现单设备登录限制
11. 实现心跳机制
12. 实现后台清理任务
13. 创建Docker和Docker Compose配置
14. 测试各个功能模块
15. 部署配置

## 安全性考虑

1. 密码使用bcrypt加密存储
2. 使用JWT进行无状态认证
3. 单设备登录限制
4. 完善的日志记录
5. 输入验证和参数过滤
6. 防止SQL注入
7. 防止跨站请求伪造
8. 定期更新依赖库

## 部署配置

### Dockerfile

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/
RUN echo 'fn main() { println!("Hello, world!"); }' > src/main.rs
RUN cargo build --release
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends postgresql-client
WORKDIR /app
COPY --from=builder /app/target/release/rust-server .
COPY .env.example .env
COPY migrations/ migrations/
EXPOSE 28001
CMD ["./rust-server"]
```

### docker-compose.yml

```yaml
version: '3.8'

services:
  db:
    image: postgres:15
    restart: always
    environment:
      POSTGRES_USER: ${DB_USER}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_DB: ${DB_NAME}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"
  
  app:
    build: .
    restart: always
    depends_on:
      - db
    environment:
      DATABASE_URL: postgres://${DB_USER}:${DB_PASSWORD}@db:5432/${DB_NAME}
      JWT_SECRET: ${JWT_SECRET}
      HEARTBEAT_INTERVAL: ${HEARTBEAT_INTERVAL}
      CLEANUP_INTERVAL: ${CLEANUP_INTERVAL}
      SERVER_PORT: 28001
    ports:
      - "28001:28001"
    volumes:
      - .env:/app/.env
    command: ./rust-server

volumes:
  postgres_data:
```

### 环境变量示例 (.env.example)

```
DATABASE_URL=postgres://admin:password@localhost:5432/rust_server
JWT_SECRET=your-secret-key-here
HEARTBEAT_INTERVAL=600  # 10分钟，单位秒
CLEANUP_INTERVAL=300    # 5分钟，单位秒
SERVER_PORT=28001
```

## 监控与维护

1. 实现日志轮转，定期归档日志文件
2. 添加监控指标收集（可选，如Prometheus）
3. 数据库定期备份策略
4. 错误告警机制（可选，如通过邮件或短信通知）

## 扩展考虑

1. 添加邮箱验证功能
2. 支持多种充值方式
3. 添加管理员后台
4. 支持批量生成充值卡密
5. 添加用户消费记录
6. 实现API速率限制
7. 添加WebSocket支持，实现实时通知

