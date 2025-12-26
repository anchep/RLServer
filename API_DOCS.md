# RLServer API 接口文档

## 1. 认证相关接口

### 1.1 用户注册

**请求方式**: POST
**请求地址**: `/api/auth/register`
**认证要求**: 无需认证
**请求体**: 
```json
{
  "username": "string",
  "password": "string",
  "email": "string"
}
```

**响应**: 
```json
{
  "message": "Registration successful. Please check your email for verification code.",
  "user": {
    "id": 1,
    "username": "string",
    "password_hash": "string",
    "email": "string",
    "email_verified": false,
    "vip_level": 0,
    "vip_expires_at": null,
    "last_login_at": null,
    "last_login_hardware": null,
    "last_login_version": null,
    "last_login_ip": null,
    "created_at": "2025-12-23T14:30:11Z",
    "updated_at": "2025-12-23T14:30:11Z"
  }
}
```

### 1.2 用户登录

**请求方式**: POST
**请求地址**: `/api/auth/login`
**认证要求**: 无需认证
**请求体**: 
```json
{
  "username": "string",
  "password": "string",
  "hardware_code": "string",
  "software_version": "string"
}
```

**响应**: 
```json
{
  "message": "Login successful",
  "user": {
    "id": 1,
    "username": "string",
    "password_hash": "string",
    "email": "string",
    "email_verified": false,
    "vip_level": 0,
    "vip_expires_at": null,
    "last_login_at": "2025-12-23T14:47:52Z",
    "last_login_hardware": "string",
    "last_login_version": "string",
    "last_login_ip": "string",
    "created_at": "2025-12-23T14:30:11Z",
    "updated_at": "2025-12-23T14:47:52Z"
  },
  "token": "string"
}
```

### 1.3 刷新访问令牌

**请求方式**: POST
**请求地址**: `/api/auth/refresh`
**认证要求**: 无需认证
**请求体**: 
```json
{
  "refresh_token": "string"
}
```

**响应**: 
```json
{
  "message": "Token refreshed successfully",
  "user": {
    "id": 1,
    "username": "string",
    "password_hash": "string",
    "email": "string",
    "email_verified": false,
    "vip_level": 0,
    "vip_expires_at": null,
    "last_login_at": "2025-12-23T14:47:52Z",
    "last_login_hardware": "string",
    "last_login_version": "string",
    "last_login_ip": "string",
    "created_at": "2025-12-23T14:30:11Z",
    "updated_at": "2025-12-23T14:47:52Z"
  },
  "token": "string"
}
```

### 1.4 密码重置请求

**请求方式**: POST
**请求地址**: `/api/auth/reset-password`
**认证要求**: 无需认证
**请求体**: 
```json
{
  "email": "string"
}
```

**响应**: 
```json
{
  "message": "Password reset email sent successfully"
}
```

### 1.5 验证密码重置令牌并更新密码

**请求方式**: POST
**请求地址**: `/api/auth/reset-password/verify`
**认证要求**: 无需认证
**请求体**: 
```json
{
  "token": "string",
  "new_password": "string"
}
```

**响应**: 
```json
{
  "message": "Password reset successful"
}
```

## 2. 用户管理接口

### 2.1 获取当前用户信息

**请求方式**: GET
**请求地址**: `/api/protected/users/me`
**认证要求**: 需要认证 (Bearer Token)
**响应**: 
```json
{
  "id": 1,
  "username": "string",
  "email": "string",
  "email_verified": false,
  "vip_level": 0,
  "vip_expires_at": null,
  "last_login_at": "2025-12-23T14:47:52Z",
  "last_login_hardware": "string",
  "last_login_version": "string",
  "last_login_ip": "string",
  "created_at": "2025-12-23T14:30:11Z",
  "updated_at": "2025-12-23T14:47:52Z"
}
```

### 2.2 获取可用软件列表

**请求方式**: GET
**请求地址**: `/api/protected/users/software`
**认证要求**: 需要认证 (Bearer Token)
**响应**: 
```json
[
  {
    "id": 1,
    "name": "string",
    "required_vip_level": 0,
    "created_at": "2025-12-23T14:30:11Z",
    "updated_at": "2025-12-23T14:30:11Z"
  }
]
```

### 2.3 用户登出

**请求方式**: POST
**请求地址**: `/api/auth/logout`
**认证要求**: 无需认证 (Bearer Token 在请求头中传递)
**请求体**: 
```json
{
  "hardware_code": "string",
  "software_version": "string"
}
```

**响应**: 
```json
{
  "message": "Logout successful"
}
```

**错误响应**: 
```json
{
  "error": "Logout token error"
}
```

## 3. 邮箱验证相关接口

### 3.1 验证邮箱

**请求方式**: POST
**请求地址**: `/api/protected/email/verify`
**认证要求**: 需要认证 (Bearer Token)
**请求体**: 
```json
{
  "code": "string"
}
```

**响应**: 
```json
{
  "message": "Email verified successfully"
}
```

### 3.2 重新发送验证邮件

**请求方式**: POST
**请求地址**: `/api/protected/email/resend`
**认证要求**: 需要认证 (Bearer Token)
**响应**: 
```json
{
  "message": "Verification email resent successfully"
}
```

## 4. 充值相关接口

### 4.1 卡密充值

**请求方式**: POST
**请求地址**: `/api/protected/recharge`
**认证要求**: 需要认证 (Bearer Token)
**请求体**: 
```json
{
  "card_code": "string"
}
```

**响应**: 
```json
{
  "message": "Recharge successful",
  "new_vip_level": 1,
  "new_vip_expires_at": "2025-12-23T14:47:52Z"
}
```

### 4.2 获取充值记录

**请求方式**: GET
**请求地址**: `/api/protected/recharge/logs`
**认证要求**: 需要认证 (Bearer Token)
**响应**: 
```json
[
  {
    "id": 1,
    "user_id": 1,
    "card_code": "string",
    "vip_level": 1,
    "duration_days": 30,
    "recharge_time": "2025-12-23T14:47:52Z",
    "created_at": "2025-12-23T14:47:52Z"
  }
]
```

## 5. 软件相关接口

### 5.1 获取所有软件列表

**请求方式**: GET
**请求地址**: `/api/protected/software`
**认证要求**: 需要认证 (Bearer Token)
**响应**: 
```json
[
  {
    "id": 1,
    "name": "string",
    "required_vip_level": 0,
    "created_at": "2025-12-23T14:30:11Z",
    "updated_at": "2025-12-23T14:30:11Z"
  }
]
```

### 5.2 检查软件访问权限

**请求方式**: GET
**请求地址**: `/api/protected/software/{software_id}/access`
**认证要求**: 需要认证 (Bearer Token)
**路径参数**: 
- `software_id`: 软件ID

**响应**: 
```json
{
  "has_access": true,
  "vip_level": 1,
  "required_vip_level": 0,
  "vip_expires_at": "2025-12-23T14:47:52Z"
}
```

## 6. 心跳相关接口

### 6.1 发送心跳

**请求方式**: POST
**请求地址**: `/api/heartbeat`
**认证要求**: 无需认证
**请求体**: 
```json
{
  "session_token": "string",
  "hardware_code": "string",
  "software_version": "string"
}
```

**响应**: 
```json
{
  "message": "Heartbeat received successfully"
}
```

## 7. 认证方式

所有需要认证的接口，必须在请求头中添加以下认证信息：

```
Authorization: Bearer <token>
```

其中 `<token>` 是通过登录接口获取的访问令牌。

## 8. 错误响应格式

当请求失败时，API会返回以下格式的错误响应：

```json
{
  "error": "错误描述信息"
}
```

常见错误码：
- 400 Bad Request: 请求参数错误
- 401 Unauthorized: 认证失败或令牌无效
- 403 Forbidden: 没有权限访问该资源
- 404 Not Found: 请求的资源不存在
- 500 Internal Server Error: 服务器内部错误

## 9. 数据类型说明

| 数据类型 | 描述 | 示例 |
| --- | --- | --- |
| string | 字符串类型 | "test_user" |
| integer | 整数类型 | 123 |
| boolean | 布尔类型 | true/false |
| null | 空值 | null |
| timestamp | 时间戳（ISO 8601格式） | "2025-12-23T14:30:11Z" |

## 10. 安全注意事项

1. 所有API请求都应使用HTTPS协议
2. 访问令牌有效期为1小时，过期后需要使用刷新令牌获取新令牌
3. 密码重置令牌有效期为1小时
4. 请妥善保管您的令牌，不要泄露给他人
5. 建议定期更换密码，使用强密码

## 11. 速率限制

API实施了速率限制，以保护服务器资源和防止恶意请求。当前限制为：

- 每秒最多2个请求
- 允许突发5个请求

当超出限制时，API会返回`429 Too Many Requests`响应。

## 12. 输入验证

所有API请求都会进行严格的输入验证，包括：

- **用户名**：3-20个字符
- **密码**：至少8个字符，包含大小写字母、数字和特殊字符
- **邮箱**：有效的邮箱格式
- **硬件码**：1-100个字符
- **软件版本**：1-50个字符

验证失败时，API会返回`400 Bad Request`响应，包含具体的错误信息。

## 13. 安全特性

- **HTTPS支持**：所有请求建议通过HTTPS发送
- **密码加密**：使用bcrypt算法加密存储密码
- **JWT认证**：使用JSON Web Token进行身份验证
- **速率限制**：防止API滥用
- **输入验证**：防止恶意输入
- **IP地址记录**：记录用户登录和操作的IP地址

## 14. 错误响应

| 错误码 | 描述 | 示例 |
| --- | --- | --- |
| 400 | 请求参数错误 | `{"error": "Invalid email format"}` |
| 401 | 认证失败 | `{"error": "Invalid token"}` |
| 403 | 没有权限 | `{"error": "Access denied"}` |
| 404 | 资源不存在 | `{"error": "User not found"}` |
| 409 | 冲突 | `{"error": "Username already exists"}` |
| 429 | 请求过于频繁 | `{"error": "Rate limit exceeded"}` |
| 500 | 服务器错误 | `{"error": "Internal server error"}` |

## 15. 版本控制

API版本信息通过URL路径进行控制，当前版本为v1（默认）。未来版本升级会在URL中体现，例如：

```
/api/v2/auth/login
```

## 16. 联系信息

如有任何API相关问题或建议，请联系技术支持：
- 邮箱：support@rlserver.com
- 文档更新时间：2025-12-23
