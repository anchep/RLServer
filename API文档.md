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

## 路由结构

### 1. 健康检查
- **路径**：`/health`
- **方法**：GET
- **认证**：不需要
- **功能**：检查服务是否正常运行
- **返回示例**：
  ```json
  "ok"
  ```

### 2. 公开API (/api)

#### 2.1 认证相关
- **注册**：`POST /api/auth/register`
  - 功能：注册新用户
  - 参数：username, email, password

- **登录**：`POST /api/auth/login`
  - 功能：用户登录，返回JWT token
  - 参数：username, password

- **刷新token**：`POST /api/auth/refresh`
  - 功能：使用刷新token获取新的访问token
  - 参数：refresh_token

- **重置密码**：`POST /api/auth/reset-password`
  - 功能：发送密码重置邮件
  - 参数：email

- **验证重置密码**：`POST /api/auth/reset-password/verify`
  - 功能：验证重置密码验证码
  - 参数：email, code

- **退出登录**：`POST /api/auth/logout`
  - 功能：使当前token失效
  - 参数：token

- **邮箱验证**：`POST /api/auth/verify-email`
  - 功能：验证邮箱
  - 参数：token

#### 2.2 其他公开API
- **心跳**：`POST /api/heartbeat`
  - 功能：保持连接活跃
  - 参数：user_id

### 3. 保护API (/api/protected)
需要在请求头中包含有效的JWT token：`Authorization: Bearer <token>`

#### 3.1 用户相关
- **获取用户信息**：`GET /api/protected/users/me`
  - 功能：获取当前登录用户信息

- **获取可用软件**：`GET /api/protected/users/software`
  - 功能：获取用户可用的软件列表

#### 3.2 充值相关
- **充值**：`POST /api/protected/recharge`
  - 功能：使用卡密充值
  - 参数：card_code

- **充值记录**：`GET /api/protected/recharge/logs`
  - 功能：获取用户的充值记录

#### 3.3 软件相关
- **获取所有软件**：`GET /api/protected/software`
  - 功能：获取所有软件列表

- **检查软件访问权限**：`GET /api/protected/software/{software_id}/access`
  - 功能：检查用户是否有权限访问指定软件

### 4. 后台管理API (/admin)

#### 4.1 认证
- **登录**：`GET/POST /admin/login`
  - 功能：管理员登录

- **注册**：`GET/POST /admin/register`
  - 功能：注册管理员账号

- **退出登录**：`GET /admin/logout`
  - 功能：管理员退出登录

- **忘记密码**：`GET/POST /admin/forgot-password`
  - 功能：管理员忘记密码

- **重置密码**：`GET/POST /admin/reset-password/{token}`
  - 功能：管理员重置密码

#### 4.2 仪表板
- **首页**：`GET /admin/dashboard/`
  - 功能：仪表盘首页

- **个人中心**：`GET /admin/dashboard/profile`
  - 功能：管理员个人中心

- **修改密码**：`GET/POST /admin/dashboard/change-password`
  - 功能：修改管理员密码

- **修改邮箱**：`POST /admin/dashboard/change-email`
  - 功能：修改管理员邮箱

- **操作日志**：`GET /admin/dashboard/logs`
  - 功能：查看管理员操作日志

#### 4.3 软件管理
- **软件列表**：`GET /admin/dashboard/software`
  - 功能：查看软件列表

- **添加软件**：`GET/POST /admin/dashboard/software/add`
  - 功能：添加新软件

- **软件详情**：`GET /admin/dashboard/software/detail/{id}`
  - 功能：查看软件详情

- **编辑软件**：`GET/POST /admin/dashboard/software/edit/{id}`
  - 功能：编辑软件信息

- **删除软件**：`POST /admin/dashboard/software/delete/{id}`
  - 功能：删除软件

- **切换软件状态**：`POST /admin/dashboard/software/toggle/{id}`
  - 功能：切换软件的启用/禁用状态

#### 4.4 卡密管理
- **卡密列表**：`GET /admin/dashboard/cards`
  - 功能：查看卡密列表，支持筛选和分页
  - 参数：status, vip_level, duration, page, page_size
  - 时长筛选：支持1天、7天、15天、30天、90天、180天、365天

- **生成卡密**：`GET/POST /admin/dashboard/cards/generate`
  - 功能：生成新的卡密
  - 参数：vip_level, duration, price, quantity

- **充值历史**：`GET /admin/dashboard/cards/history`
  - 功能：查看充值历史记录
  - 参数：user_id, card_code, start_time, end_time, page, page_size

- **卡密详情**：`GET /admin/dashboard/cards/detail/{id}`
  - 功能：查看卡密详情

- **更新卡密售价**：`POST /admin/dashboard/cards/update-price/{id}`
  - 功能：更新卡密的售价
  - 参数：price

- **删除卡密**：`POST /admin/dashboard/cards/delete/{id}`
  - 功能：删除单个卡密

- **批量删除卡密**：`POST /admin/dashboard/cards/batch-delete`
  - 功能：批量删除选中的卡密
  - 参数：card_ids（数组）

- **导出卡密**：`客户端JavaScript实现`
  - 功能：导出选中的卡密为CSV文件
  - 操作：在卡密列表页面勾选要导出的卡密，点击"导出选中卡密"按钮
  - 格式：CSV格式，包含字段：卡密代码、类型、时长（天）、售价、状态

#### 4.5 用户管理
- **用户列表**：`GET /admin/dashboard/users`
  - 功能：查看用户列表

- **添加用户**：`GET/POST /admin/dashboard/users/add`
  - 功能：添加新用户

- **用户详情**：`GET /admin/dashboard/users/detail/{id}`
  - 功能：查看用户详情

- **编辑用户**：`GET /admin/dashboard/users/edit/{id}`
  - 功能：编辑用户信息

- **编辑VIP**：`GET /admin/dashboard/users/edit-vip/{id}`
  - 功能：编辑用户VIP信息

- **保存VIP**：`POST /admin/dashboard/users/save-vip`
  - 功能：保存用户VIP信息

- **切换用户状态**：`POST /admin/dashboard/users/toggle-status/{id}/{status}`
  - 功能：切换用户的启用/禁用状态

- **登录历史**：`GET /admin/dashboard/users/login-history/{id}`
  - 功能：查看用户登录历史

- **在线用户**：`GET /admin/dashboard/users/online`
  - 功能：查看在线用户列表

- **黑名单管理**：`GET /admin/dashboard/users/blacklist`
  - 功能：查看黑名单列表

- **添加到黑名单**：`GET/POST /admin/dashboard/users/blacklist/add`
  - 功能：将用户添加到黑名单

- **从黑名单移除**：`POST /admin/dashboard/users/blacklist/remove/{id}`
  - 功能：将用户从黑名单移除

#### 4.6 统计分析
- **综合统计**：`GET /admin/dashboard/stats/overview`
  - 功能：查看综合统计数据

- **销售统计**：`GET /admin/dashboard/stats/sales`
  - 功能：查看销售统计数据

- **用户统计**：`GET /admin/dashboard/stats/users`
  - 功能：查看用户统计数据

- **卡密统计**：`GET /admin/dashboard/stats/cards`
  - 功能：查看卡密统计数据

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

### 4. 常用命令

#### 4.1 查看日志
```bash
# 查看应用日志
docker-compose logs -f app

# 查看数据库日志
docker-compose logs -f db

# 查看Nginx日志
docker-compose logs -f nginx
```

#### 4.2 重启服务
```bash
docker-compose restart

# 重启特定服务
docker-compose restart app
```

#### 4.3 停止服务
```bash
docker-compose down

# 停止并删除所有资源（包括数据库数据）
docker-compose down -v
```

#### 4.4 进入容器
```bash
# 进入应用容器
docker-compose exec app bash

# 进入数据库容器
docker-compose exec db psql -U admin -d rl_server
```

### 5. 开发流程

#### 5.1 开发环境启动
```bash
docker-compose up --build -d
```

#### 5.2 代码质量检查
```bash
# 检查代码语法
cargo check

# 格式化代码
cargo fmt

# 检查代码风格
cargo clippy
```

#### 5.3 运行测试
```bash
cargo test
```

#### 5.4 重新构建
```bash
docker-compose up --build -d
```

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
