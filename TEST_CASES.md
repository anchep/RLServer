# 功能测试用例

## 1. 注册功能测试

### 测试用例1.1：正常注册
- **请求**：
  ```
  POST /api/auth/register
  {
    "username": "test_user",
    "password": "Password123!",
    "email": "test@example.com"
  }
  ```
- **预期响应**：
  ```
  {
    "message": "Registration successful. Please check your email for verification code.",
    "user": {
      "id": 1,
      "username": "test_user",
      "email": "test@example.com",
      "email_verified": false,
      "vip_level": 0,
      "vip_expires_at": null,
      "last_login_at": null,
      "last_login_hardware": null,
      "last_login_version": null,
      "last_login_ip": null,
      "created_at": "2025-12-23T12:00:00Z",
      "updated_at": "2025-12-23T12:00:00Z"
    }
  }
  ```

### 测试用例1.2：用户名已存在
- **请求**：
  ```
  POST /api/auth/register
  {
    "username": "test_user",
    "password": "Password123!",
    "email": "test2@example.com"
  }
  ```
- **预期响应**：
  ```
  {
    "error": "Username already exists"
  }
  ```

## 2. 登录功能测试

### 测试用例2.1：正常登录
- **请求**：
  ```
  POST /api/auth/login
  {
    "username": "test_user",
    "password": "Password123!",
    "hardware_code": "hw-123456",
    "software_version": "v1.0.0"
  }
  ```
- **预期响应**：
  ```
  {
    "message": "Login successful",
    "user": {
      "id": 1,
      "username": "test_user",
      "email": "test@example.com",
      "email_verified": false,
      "vip_level": 0,
      "vip_expires_at": null,
      "last_login_at": "2025-12-23T12:00:00Z",
      "last_login_hardware": "hw-123456",
      "last_login_version": "v1.0.0",
      "last_login_ip": "127.0.0.1",
      "created_at": "2025-12-23T12:00:00Z",
      "updated_at": "2025-12-23T12:00:00Z"
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
  ```

### 测试用例2.2：密码错误
- **请求**：
  ```
  POST /api/auth/login
  {
    "username": "test_user",
    "password": "WrongPassword!",
    "hardware_code": "hw-123456",
    "software_version": "v1.0.0"
  }
  ```
- **预期响应**：
  ```
  {
    "error": "Invalid password"
  }
  ```

## 3. 登录冲突测试

### 测试用例3.1：同一用户多设备登录
1. 使用设备1登录，获取token1
2. 使用设备2登录，获取token2
3. 使用token1访问受保护API
4. 预期结果：token1失效，返回401错误

- **请求3**：
  ```
  GET /api/protected/users/me
  Authorization: Bearer token1
  ```
- **预期响应**：
  ```
  {
    "error": "Invalid token"
  }
  ```

## 4. 充值功能测试

### 测试用例4.1：正常充值
- **前提条件**：
  - 已登录获取token
  - 存在有效的充值卡密
- **请求**：
  ```
  POST /api/protected/recharge
  Authorization: Bearer <token>
  {
    "card_code": "RC-123456-7890"
  }
  ```
- **预期响应**：
  ```
  {
    "message": "Recharge successful",
    "user": {
      "id": 1,
      "username": "test_user",
      "email": "test@example.com",
      "email_verified": false,
      "vip_level": 1,
      "vip_expires_at": "2026-01-23T12:00:00Z",
      "last_login_at": "2025-12-23T12:00:00Z",
      "last_login_hardware": "hw-123456",
      "last_login_version": "v1.0.0",
      "last_login_ip": "127.0.0.1",
      "created_at": "2025-12-23T12:00:00Z",
      "updated_at": "2025-12-23T12:00:00Z"
    }
  }
  ```

### 测试用例4.2：无效卡密
- **请求**：
  ```
  POST /api/protected/recharge
  Authorization: Bearer <token>
  {
    "card_code": "INVALID-CARD"
  }
  ```
- **预期响应**：
  ```
  {
    "error": "Invalid card code or card has been used"
  }
  ```

## 5. VIP到期测试

### 测试用例5.1：VIP到期后状态
- **前提条件**：
  - 用户VIP已到期
- **请求**：
  ```
  GET /api/protected/users/me
  Authorization: Bearer <token>
  ```
- **预期响应**：
  ```
  {
    "id": 1,
    "username": "test_user",
    "email": "test@example.com",
    "email_verified": false,
    "vip_level": 0, // VIP到期后恢复为0
    "vip_expires_at": "2025-12-23T12:00:00Z",
    "last_login_at": "2025-12-23T12:00:00Z",
    "last_login_hardware": "hw-123456",
    "last_login_version": "v1.0.0",
    "last_login_ip": "127.0.0.1",
    "created_at": "2025-12-23T12:00:00Z",
    "updated_at": "2025-12-23T12:00:00Z"
  }
  ```

## 6. 心跳功能测试

### 测试用例6.1：正常心跳
- **请求**：
  ```
  POST /api/heartbeat
  {
    "session_token": "<token>",
    "hardware_code": "hw-123456",
    "software_version": "v1.0.0"
  }
  ```
- **预期响应**：
  ```
  {
    "message": "Heartbeat updated successfully"
  }
  ```

### 测试用例6.2：无效token心跳
- **请求**：
  ```
  POST /api/heartbeat
  {
    "session_token": "invalid-token",
    "hardware_code": "hw-123456",
    "software_version": "v1.0.0"
  }
  ```
- **预期响应**：
  ```
  {
    "error": "Invalid token"
  }
  ```

## 7. 刷新令牌测试

### 测试用例7.1：正常刷新令牌
- **请求**：
  ```
  POST /api/protected/auth/refresh
  Authorization: Bearer <refresh_token>
  {
    "refresh_token": "<refresh_token>"
  }
  ```
- **预期响应**：
  ```
  {
    "message": "Token refreshed successfully",
    "token": "<new_access_token>",
    "refresh_token": "<new_refresh_token>"
  }
  ```

## 8. 退出登录测试

### 测试用例8.1：正常退出登录
- **请求**：
  ```
  POST /api/protected/users/logout
  Authorization: Bearer <token>
  ```
- **预期响应**：
  ```
  {
    "message": "Logout successful"
  }
  ```

### 测试用例8.2：退出后令牌失效
- **操作**：
  1. 登录获取token
  2. 退出登录
  3. 使用同一token访问受保护API
- **预期响应**：
  ```
  {
    "error": "Invalid token"
  }
  ```

# 测试说明

1. **注册**：用户注册时需要提供用户名、密码和邮箱，系统会发送验证码到邮箱（当前版本跳过验证直接登录）
2. **登录**：登录时需要提供用户名、密码、硬件码和软件版本，生成访问令牌和刷新令牌
3. **充值**：使用有效的充值卡密可以提升VIP等级和延长到期时间
4. **登录冲突**：同一用户只能在一个设备上登录，新登录会踢掉旧登录
5. **VIP到期**：VIP到期后会自动恢复为普通用户
6. **心跳**：客户端需要定期发送心跳，保持在线状态
7. **刷新令牌**：访问令牌过期后可以使用刷新令牌获取新的访问令牌
8. **退出登录**：退出后当前令牌失效，需要重新登录

# 环境配置

## .env文件配置示例
```
# 数据库连接配置
DATABASE_URL=postgres://admin:password@localhost:5432/rust_server

# JWT配置
JWT_SECRET=your-secret-key-here

# 服务器配置
SERVER_PORT=28001

# 心跳配置（秒）
HEARTBEAT_INTERVAL=600  # 10分钟

# 清理不活跃用户间隔（秒）
CLEANUP_INTERVAL=300    # 5分钟

# SMTP配置（可选，当前版本忽略）
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USERNAME=your-smtp-username
SMTP_PASSWORD=your-smtp-password
SMTP_FROM_EMAIL=no-reply@example.com
SMTP_SSL=false
SMTP_TIMEOUT=30
```

# 启动服务器

```bash
# 开发模式
cargo run

# 生产模式
cargo build --release
./target/release/rlserver
```

# 测试工具

可以使用以下工具进行API测试：
- Postman
- curl
- httpie
- Thunder Client（VS Code插件）

# 数据库初始化

服务器启动时会自动运行数据库迁移，创建所需的表结构。

# 注意事项

1. 确保PostgreSQL数据库已启动并创建了对应的数据库
2. 确保.env文件中的数据库连接信息正确
3. 当前版本跳过了邮箱验证，用户注册后可以直接登录
4. 测试充值功能时需要先创建充值卡密
5. 登录时硬件码和软件版本需要唯一标识客户端设备
