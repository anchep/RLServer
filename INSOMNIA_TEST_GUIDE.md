# Insomnia 测试退出登录功能指南

## 1. 创建新请求
1. 打开 Insomnia
2. 点击左上角的 `+` 按钮或选择 `File > New Request`
3. 为请求命名，例如：`Logout Test`

## 2. 设置请求基本信息

### 请求方法
- 在请求URL栏左侧的下拉菜单中选择 **POST**

### 请求URL
根据您的测试环境选择：
- HTTP: `http://localhost:28001/api/auth/logout`
- HTTPS: `https://localhost:28043/api/auth/logout`

## 3. 设置请求头 (Headers)
点击 **Headers** 标签页，添加以下头部：

| 头部名称 | 值 | 类型 |
|---------|-----|------|
| Content-Type | application/json | 自动完成（选择application/json） |
| Authorization | Bearer <your_token_here> | 手动输入（替换<your_token_here>为实际token） |

**示例：**
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6InRlc3RfdXNlcl8xMjM0NTYiLCJleHAiOjE3NDExMzYwMDB9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
```

## 4. 设置请求体 (Body)
点击 **Body** 标签页，选择 **JSON** 格式，然后输入以下内容：

```json
{
  "hardware_code": "test_hw",
  "software_version": "1.0.0"
}
```

## 5. 执行请求
点击右上角的 **Send** 按钮执行请求

## 6. 验证响应

### 预期响应结果

#### 场景1：第一次使用有效Token退出
- **状态码**: 200 OK
- **响应体**: 
  ```json
  {
    "message": "Logout successful"
  }
  ```

#### 场景2：重复使用同一Token退出
- **状态码**: 401 Unauthorized
- **响应体**: 
  ```json
  {
    "error": "Logout token error"
  }
  ```

#### 场景3：使用无效Token退出
- **状态码**: 401 Unauthorized
- **响应体**: 
  ```json
  {
    "error": "Logout token error"
  }
  ```

## 7. 如何获取有效Token

### 方法1：使用测试脚本
运行 `test_heartbeat_logout.ps1` 脚本，登录后会显示Token信息

### 方法2：在Insomnia中手动获取
1. 创建一个新的POST请求到 `/api/auth/login`
2. 请求体：
   ```json
   {
     "username": "your_username",
     "password": "your_password",
     "hardware_code": "test_hw",
     "software_version": "1.0.0"
   }
   ```
3. 执行登录请求，从响应中获取Token
4. 将Token复制到退出登录请求的Authorization头中

## 8. 测试心跳包功能（可选）

### 创建心跳包请求
1. 创建新的POST请求到 `/api/heartbeat`
2. 请求头：只需要 `Content-Type: application/json`
3. 请求体：
   ```json
   {
     "session_token": "<your_token_here>",
     "hardware_code": "test_hw",
     "software_version": "1.0.0"
   }
   ```

### 预期响应
- 有效Token: `{"message": "Heartbeat updated successfully"}`
- 无效Token: `{"error": "找不到此token"}`

## 9. 常见问题排查

### 问题：请求失败，显示连接被拒绝
- 检查服务器是否正在运行
- 检查端口是否正确（HTTP: 28001, HTTPS: 28043）

### 问题：退出登录总是返回成功
- 确保您使用的是正确的退出登录URL：`/api/auth/logout`
- 确保服务器已更新到最新代码
- 尝试重新启动服务器：`docker-compose restart app`

### 问题：心跳包使用无效Token返回成功
- 确保服务器代码已包含心跳包无效Token检查
- 验证 `src/services/heartbeat.rs` 中的 `update_heartbeat` 函数是否检查了 `updated_rows`

## 10. 测试完整流程

1. **注册新用户** → `/api/auth/register`
2. **登录获取Token** → `/api/auth/login`
3. **发送心跳（有效Token）** → `/api/heartbeat` (预期成功)
4. **第一次退出登录** → `/api/auth/logout` (预期成功)
5. **发送心跳（已退出的Token）** → `/api/heartbeat` (预期失败：找不到此token)
6. **第二次退出登录** → `/api/auth/logout` (预期失败：Logout token error)
7. **使用无效Token退出** → `/api/auth/logout` (预期失败：Logout token error)

通过以上步骤，您可以完整测试退出登录和心跳包功能的正确性。