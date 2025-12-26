# API测试脚本
# 使用前请确保服务器已启动，默认端口28001

$baseUrl = "http://localhost:28001/api"
$headers = @{}
$token = ""

# 测试注册功能
Write-Host "=== 测试注册功能 ==="
$registerBody = @{
    username = "test_user_$(Get-Random)"
    password = "Password123!"
    email = "test_$(Get-Random)@example.com"
}

$registerResponse = Invoke-RestMethod -Uri "$baseUrl/auth/register" -Method Post -Body ($registerBody | ConvertTo-Json) -ContentType "application/json"
Write-Host "注册响应: $($registerResponse | ConvertTo-Json -Depth 10)" -ForegroundColor Green

# 测试登录功能
Write-Host "\n=== 测试登录功能 ==="
$loginBody = @{
    username = $registerBody.username
    password = $registerBody.password
    hardware_code = "hw-$(Get-Random)"
    software_version = "v1.0.0"
}

$loginResponse = Invoke-RestMethod -Uri "$baseUrl/auth/login" -Method Post -Body ($loginBody | ConvertTo-Json) -ContentType "application/json"
Write-Host "登录响应: $($loginResponse | ConvertTo-Json -Depth 10)" -ForegroundColor Green

# 保存令牌
$token = $loginResponse.token
$headers = @{
    Authorization = "Bearer $token"
}

# 测试获取用户信息
Write-Host "\n=== 测试获取用户信息 ==="
$userInfoResponse = Invoke-RestMethod -Uri "$baseUrl/protected/users/me" -Method Get -Headers $headers
Write-Host "用户信息: $($userInfoResponse | ConvertTo-Json -Depth 10)" -ForegroundColor Green

# 测试获取可用软件
Write-Host "\n=== 测试获取可用软件 ==="
$softwareResponse = Invoke-RestMethod -Uri "$baseUrl/protected/users/software" -Method Get -Headers $headers
Write-Host "可用软件: $($softwareResponse | ConvertTo-Json -Depth 10)" -ForegroundColor Green

# 测试心跳功能
Write-Host "\n=== 测试心跳功能 ==="
$heartbeatBody = @{
    session_token = $token
    hardware_code = $loginBody.hardware_code
    software_version = $loginBody.software_version
}

$heartbeatResponse = Invoke-RestMethod -Uri "$baseUrl/heartbeat" -Method Post -Body ($heartbeatBody | ConvertTo-Json) -ContentType "application/json"
Write-Host "心跳响应: $($heartbeatResponse | ConvertTo-Json -Depth 10)" -ForegroundColor Green

# 测试刷新令牌
Write-Host "\n=== 测试刷新令牌 ==="
# 注意：刷新令牌需要从登录响应中获取，当前代码中登录响应只返回了access_token
# 这里假设刷新令牌API可以使用access_token调用
$refreshBody = @{
    refresh_token = $token
}

try {
    $refreshResponse = Invoke-RestMethod -Uri "$baseUrl/auth/refresh" -Method Post -Body ($refreshBody | ConvertTo-Json) -ContentType "application/json"
    Write-Host "刷新令牌响应: $($refreshResponse | ConvertTo-Json -Depth 10)" -ForegroundColor Green
} catch {
    Write-Host "刷新令牌失败: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Red
    Write-Host "错误详情: $($_.ErrorDetails.Message)" -ForegroundColor Red
}

# 测试退出登录
Write-Host "\n=== 测试退出登录 ==="
$logoutBody = @{
    hardware_code = $loginBody.hardware_code
    software_version = $loginBody.software_version
}

try {
    $logoutResponse = Invoke-RestMethod -Uri "$baseUrl/auth/logout" -Method Post -Body ($logoutBody | ConvertTo-Json) -ContentType "application/json" -Headers $headers
    Write-Host "退出登录响应: $($logoutResponse | ConvertTo-Json -Depth 10)" -ForegroundColor Green
} catch {
    Write-Host "退出登录失败: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Red
    Write-Host "错误详情: $($_.ErrorDetails.Message)" -ForegroundColor Red
}

# 测试登录冲突（重新登录）
Write-Host "\n=== 测试登录冲突 ==="
$loginResponse2 = Invoke-RestMethod -Uri "$baseUrl/auth/login" -Method Post -Body ($loginBody | ConvertTo-Json) -ContentType "application/json"
Write-Host "第二次登录响应: $($loginResponse2 | ConvertTo-Json -Depth 10)" -ForegroundColor Green

# 测试旧令牌是否失效
Write-Host "\n=== 测试旧令牌是否失效 ==="
try {
    $userInfoResponse = Invoke-RestMethod -Uri "$baseUrl/protected/users/me" -Method Get -Headers $headers
    Write-Host "旧令牌仍有效，登录冲突处理可能存在问题" -ForegroundColor Yellow
} catch {
    Write-Host "旧令牌已失效，登录冲突处理正常" -ForegroundColor Green
    Write-Host "错误信息: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Green
}

Write-Host "\n=== 测试完成 ===" -ForegroundColor Cyan
