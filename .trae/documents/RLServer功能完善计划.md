## RLServer功能完善计划

### 1. 数据库整合
- **合并所有迁移文件**：将多个迁移文件的变更整合到一个完整的数据库结构文件中
  - 创建一个新的完整迁移文件，包含所有表结构和字段
  - 移除现有的多个迁移目录
  - 确保users表包含所有字段：id, username, password_hash, email, email_verified, vip_level, vip_expires_at, last_login_at, last_login_hardware, last_login_version, last_login_ip, last_logout_at, created_at, updated_at
  - 包含所有表：users, software, recharge_cards, recharge_logs, login_logs, online_users, verification_codes
  - 包含所有索引和约束

### 2. 软件列表功能增强
- **修改响应结构**：在返回可用软件列表时，同时返回用户的VIP等级和到期时间
- **创建新的响应结构体**：在`handlers/user.rs`中创建包含软件列表、vip_level和vip_expires_at的新结构体
- **更新API文档**：添加软件列表接口的新响应格式

### 3. HTTPS配置实现
- **更新nginx.conf**：添加HTTPS服务器块，配置SSL证书路径
- **创建自签名证书脚本**：确保部署流程中包含生成SSL证书的步骤
- **更新docker-compose.yml**：确保HTTPS端口（28043）正确映射
- **配置HTTP自动重定向到HTTPS**：增强安全性

### 4. 构建和启动服务
- **编译应用**：使用docker-compose构建镜像
- **启动服务**：运行docker-compose up -d启动所有服务
- **验证服务状态**：检查容器状态和日志，确保服务正常运行

### 5. 更新文档
- **API_DOCS.md**：
  - 更新软件列表接口的响应格式
  - 添加HTTPS访问说明
  - 调整注册/登录响应示例（已完成）

- **部署流程.md**：
  - 添加HTTPS配置步骤
  - 更新软件列表功能测试
  - 添加SSL证书生成说明

### 6. 测试验证
- **测试软件列表接口**：验证返回包含VIP信息
- **测试HTTPS访问**：确保能通过HTTPS访问API
- **测试注册/登录流程**：确保基本功能正常

### 7. 最终验证
- 确保所有服务正常运行
- 确保API文档与实际实现一致
- 确保部署流程完整准确