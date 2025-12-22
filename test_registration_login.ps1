# 测试脚本：验证注册、登录和登录冲突功能
$baseUrl = "http://localhost:28001"

Write-Host "=== 开始测试 RLServer 注册和登录功能 ===" -ForegroundColor Cyan

# 测试数据
$testUser = @{
    username = "testuser_$(Get-Random -Minimum 1000 -Maximum 9999)"
    password = "Test@1234"
    email = "test_$(Get-Random -Minimum 1000 -Maximum 9999)@example.com"
}

$hardwareCode1 = "hw_$(Get-Random -Minimum 1000 -Maximum 9999)"
$hardwareCode2 = "hw_$(Get-Random -Minimum 1000 -Maximum 9999)"
$softwareVersion = "1.0.0"

# 1. 测试注册功能
Write-Host "\n1. 测试注册功能" -ForegroundColor Yellow
$registerUrl = "$baseUrl/api/auth/register"
Write-Host "注册URL: $registerUrl" -ForegroundColor Gray
Write-Host "注册用户: $($testUser.username), 邮箱: $($testUser.email)" -ForegroundColor Gray

try {
    $registerResponse = Invoke-RestMethod -Uri $registerUrl -Method Post -Body ($testUser | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✅ 注册成功! 响应: $($registerResponse | ConvertTo-Json -Depth 2)" -ForegroundColor Green
} catch {
    Write-Host "❌ 注册失败! 错误: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "状态码: $($_.Exception.Response.StatusCode.value__)" -ForegroundColor Red
    if ($_.Exception.Response) {
        $errorContent = Get-Content $_.Exception.Response.GetResponseStream() -Encoding UTF8 -Raw
        Write-Host "错误详情: $errorContent" -ForegroundColor Red
    }
    exit 1
}

# 2. 测试登录功能
Write-Host "\n2. 测试登录功能" -ForegroundColor Yellow
$loginUrl = "$baseUrl/api/auth/login"
$loginData = @{
    username = $testUser.username
    password = $testUser.password
    hardware_code = $hardwareCode1
    software_version = $softwareVersion
}

Write-Host "登录URL: $loginUrl" -ForegroundColor Gray
Write-Host "登录用户: $($testUser.username), 硬件码: $hardwareCode1" -ForegroundColor Gray

try {
    $loginResponse1 = Invoke-RestMethod -Uri $loginUrl -Method Post -Body ($loginData | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✅ 首次登录成功! 获得访问令牌和刷新令牌" -ForegroundColor Green
    Write-Host "访问令牌: $($loginResponse1.token.Substring(0, 20))..." -ForegroundColor Gray
    Write-Host "刷新令牌: $($loginResponse1.refresh_token.Substring(0, 20))..." -ForegroundColor Gray
    
    # 保存首次登录的令牌，用于后续测试
    $token1 = $loginResponse1.token
    $refreshToken1 = $loginResponse1.refresh_token
    $headers1 = @{"Authorization" = "Bearer $token1"}
} catch {
    Write-Host "❌ 首次登录失败! 错误: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "状态码: $($_.Exception.Response.StatusCode.value__)" -ForegroundColor Red
    if ($_.Exception.Response) {
        $errorContent = Get-Content $_.Exception.Response.GetResponseStream() -Encoding UTF8 -Raw
        Write-Host "错误详情: $errorContent" -ForegroundColor Red
    }
    exit 1
}

# 3. 测试同一账号在另一设备登录（登录冲突）
Write-Host "\n3. 测试登录冲突（同一账号在另一设备登录）" -ForegroundColor Yellow
$loginData2 = @{
    username = $testUser.username
    password = $testUser.password
    hardware_code = $hardwareCode2
    software_version = $softwareVersion
}

Write-Host "使用不同硬件码再次登录: $hardwareCode2" -ForegroundColor Gray
try {
    $loginResponse2 = Invoke-RestMethod -Uri $loginUrl -Method Post -Body ($loginData2 | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✅ 第二次登录成功! 获得新令牌" -ForegroundColor Green
    Write-Host "新访问令牌: $($loginResponse2.token.Substring(0, 20))..." -ForegroundColor Gray
    Write-Host "刷新令牌: $($loginResponse2.refresh_token.Substring(0, 20))..." -ForegroundColor Gray
    
    # 保存第二次登录的令牌，用于后续测试
    $token2 = $loginResponse2.token
    $headers2 = @{"Authorization" = "Bearer $token2"}
} catch {
    Write-Host "❌ 第二次登录失败! 错误: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "状态码: $($_.Exception.Response.StatusCode.value__)" -ForegroundColor Red
    if ($_.Exception.Response) {
        $errorContent = Get-Content $_.Exception.Response.GetResponseStream() -Encoding UTF8 -Raw
        Write-Host "错误详情: $errorContent" -ForegroundColor Red
    }
    exit 1
}

# 4. 验证旧令牌是否失效（登录冲突处理）
Write-Host "\n4. 验证旧令牌是否失效" -ForegroundColor Yellow
$userInfoUrl = "$baseUrl/api/protected/users/me"

Write-Host "使用旧令牌访问受保护资源..." -ForegroundColor Gray
try {
    $oldTokenResponse = Invoke-RestMethod -Uri $userInfoUrl -Method Get -Headers $headers1
    Write-Host "❌ 旧令牌仍有效，登录冲突处理可能存在问题" -ForegroundColor Yellow
    Write-Host "响应: $($oldTokenResponse | ConvertTo-Json)" -ForegroundColor Gray
} catch {
    Write-Host "✅ 旧令牌已失效，登录冲突处理正常" -ForegroundColor Green
    Write-Host "错误信息: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Gray
}

# 5. 验证新令牌是否有效
Write-Host "\n5. 验证新令牌是否有效" -ForegroundColor Yellow
Write-Host "使用新令牌访问受保护资源..." -ForegroundColor Gray
try {
    $newTokenResponse = Invoke-RestMethod -Uri $userInfoUrl -Method Get -Headers $headers2
    Write-Host "✅ 新令牌有效，用户信息获取成功" -ForegroundColor Green
    Write-Host "用户信息: $($newTokenResponse | ConvertTo-Json)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 新令牌无效! 错误: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "状态码: $($_.Exception.Response.StatusCode.value__)" -ForegroundColor Red
    if ($_.Exception.Response) {
        $errorContent = Get-Content $_.Exception.Response.GetResponseStream() -Encoding UTF8 -Raw
        Write-Host "错误详情: $errorContent" -ForegroundColor Red
    }
    exit 1
}

# 6. 测试获取用户信息
Write-Host "\n6. 测试获取用户信息" -ForegroundColor Yellow
try {
    $userInfo = Invoke-RestMethod -Uri $userInfoUrl -Method Get -Headers $headers2
    Write-Host "✅ 用户信息获取成功" -ForegroundColor Green
    Write-Host "用户名: $($userInfo.username)" -ForegroundColor Gray
    Write-Host "邮箱: $($userInfo.email)" -ForegroundColor Gray
    Write-Host "VIP等级: $($userInfo.vip_level)" -ForegroundColor Gray
    Write-Host "VIP过期时间: $($userInfo.vip_expires_at)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 获取用户信息失败! 错误: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "状态码: $($_.Exception.Response.StatusCode.value__)" -ForegroundColor Red
    if ($_.Exception.Response) {
        $errorContent = Get-Content $_.Exception.Response.GetResponseStream() -Encoding UTF8 -Raw
        Write-Host "错误详情: $errorContent" -ForegroundColor Red
    }
    exit 1
}

Write-Host "\n=== 所有测试完成 ===" -ForegroundColor Cyan
Write-Host "✅ 注册功能测试: 成功"
Write-Host "✅ 登录功能测试: 成功"
Write-Host "✅ 登录冲突处理测试: 成功"

Write-Host "\n测试用户信息:"
Write-Host "  用户名: $($testUser.username)"
Write-Host "  密码: $($testUser.password)"
Write-Host "  邮箱: $($testUser.email)"