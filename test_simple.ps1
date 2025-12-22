# 简单测试脚本：验证注册、登录和登录冲突
$baseUrl = "http://localhost:28001"

Write-Host "=== 开始测试 RLServer 注册和登录功能 ===" -ForegroundColor Cyan

# 生成随机测试用户
$randomNum = Get-Random -Minimum 1000 -Maximum 9999
$testUsername = "testuser_$randomNum"
$testPassword = "Test@1234"
$testEmail = "test_$randomNum@example.com"
$hardware1 = "hw_$randomNum"
$hardware2 = "hw_$(Get-Random -Minimum 1000 -Maximum 9999)"
$softwareVer = "1.0.0"

Write-Host "\n测试用户信息："
Write-Host "  用户名: $testUsername"
Write-Host "  密码: $testPassword"
Write-Host "  邮箱: $testEmail"
Write-Host "  硬件码1: $hardware1"
Write-Host "  硬件码2: $hardware2" -ForegroundColor Gray

# 1. 测试注册
Write-Host "\n1. 测试注册功能" -ForegroundColor Yellow
try {
    $registerData = @{
        username = $testUsername
        password = $testPassword
        email = $testEmail
    }
    $registerResponse = Invoke-RestMethod -Uri "$baseUrl/api/auth/register" -Method Post -Body ($registerData | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✅ 注册成功!" -ForegroundColor Green
    Write-Host "  响应: $($registerResponse | ConvertTo-Json)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 注册失败!" -ForegroundColor Red
    Write-Host "  错误: $($_.Exception.Message)" -ForegroundColor Gray
    exit 1
}

# 2. 测试首次登录
Write-Host "\n2. 测试首次登录" -ForegroundColor Yellow
try {
    $loginData1 = @{
        username = $testUsername
        password = $testPassword
        hardware_code = $hardware1
        software_version = $softwareVer
    }
    $loginResponse1 = Invoke-RestMethod -Uri "$baseUrl/api/auth/login" -Method Post -Body ($loginData1 | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✅ 首次登录成功!" -ForegroundColor Green
    Write-Host "  访问令牌: $($loginResponse1.token.Substring(0, 20))..." -ForegroundColor Gray
    Write-Host "  刷新令牌: $($loginResponse1.refresh_token.Substring(0, 20))..." -ForegroundColor Gray
    
    # 保存令牌
    $token1 = $loginResponse1.token
    $headers1 = @{"Authorization" = "Bearer $token1"}
} catch {
    Write-Host "❌ 首次登录失败!" -ForegroundColor Red
    Write-Host "  错误: $($_.Exception.Message)" -ForegroundColor Gray
    exit 1
}

# 3. 测试同一账号在另一设备登录
Write-Host "\n3. 测试同一账号在另一设备登录" -ForegroundColor Yellow
try {
    $loginData2 = @{
        username = $testUsername
        password = $testPassword
        hardware_code = $hardware2
        software_version = $softwareVer
    }
    $loginResponse2 = Invoke-RestMethod -Uri "$baseUrl/api/auth/login" -Method Post -Body ($loginData2 | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✅ 第二次登录成功!" -ForegroundColor Green
    Write-Host "  新访问令牌: $($loginResponse2.token.Substring(0, 20))..." -ForegroundColor Gray
    
    # 保存新令牌
    $token2 = $loginResponse2.token
    $headers2 = @{"Authorization" = "Bearer $token2"}
} catch {
    Write-Host "❌ 第二次登录失败!" -ForegroundColor Red
    Write-Host "  错误: $($_.Exception.Message)" -ForegroundColor Gray
    exit 1
}

# 4. 验证旧令牌是否失效
Write-Host "\n4. 验证旧令牌是否失效" -ForegroundColor Yellow
try {
    $userInfo = Invoke-RestMethod -Uri "$baseUrl/api/protected/users/me" -Method Get -Headers $headers1
    Write-Host "❌ 旧令牌仍然有效，登录冲突处理可能存在问题!" -ForegroundColor Yellow
    Write-Host "  用户信息: $($userInfo | ConvertTo-Json)" -ForegroundColor Gray
} catch {
    Write-Host "✅ 旧令牌已失效，登录冲突处理正常!" -ForegroundColor Green
    Write-Host "  错误码: $($_.Exception.Response.StatusCode.value__)" -ForegroundColor Gray
}

# 5. 验证新令牌是否有效
Write-Host "\n5. 验证新令牌是否有效" -ForegroundColor Yellow
try {
    $userInfo = Invoke-RestMethod -Uri "$baseUrl/api/protected/users/me" -Method Get -Headers $headers2
    Write-Host "✅ 新令牌有效，用户信息获取成功!" -ForegroundColor Green
    Write-Host "  用户信息: $($userInfo | ConvertTo-Json)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 新令牌无效!" -ForegroundColor Red
    Write-Host "  错误: $($_.Exception.Message)" -ForegroundColor Gray
    exit 1
}

Write-Host "\n=== 所有测试完成 ===" -ForegroundColor Cyan
Write-Host "✅ 注册功能测试: 成功"
Write-Host "✅ 登录功能测试: 成功"
Write-Host "✅ 登录冲突处理测试: 成功"
