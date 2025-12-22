# 简单测试脚本 - 逐步测试API端点

# 1. 测试登录功能
Write-Host "=== 测试登录功能 ==="
$loginUrl = "http://localhost:28001/api/auth/login"
$loginBody = '{"username":"test_user2","password":"123456","hardware_code":"hw-test","software_version":"v1.0.0"}'

try {
    $loginResponse = Invoke-WebRequest -Uri $loginUrl -Method POST -ContentType "application/json" -Body $loginBody -UseBasicParsing
    Write-Host "✅ 登录成功！状态码: $($loginResponse.StatusCode)"
    Write-Host "响应内容: $($loginResponse.Content)"
    
    # 提取Token
    $loginJson = $loginResponse.Content | ConvertFrom-Json
    $token = $loginJson.token
    Write-Host "获取到Token: $token"
    
    # 2. 测试认证中间件 - 获取用户信息
    Write-Host "`n=== 测试获取用户信息 ==="
    $userInfoUrl = "http://localhost:28001/api/protected/users/me"
    try {
        $userInfoResponse = Invoke-WebRequest -Uri $userInfoUrl -Method GET -Headers @{"Authorization"="Bearer $token"} -UseBasicParsing
        Write-Host "✅ 获取用户信息成功！状态码: $($userInfoResponse.StatusCode)"
        Write-Host "响应内容: $($userInfoResponse.Content)"
        
        # 3. 测试充值功能
        Write-Host "`n=== 测试充值功能 ==="
        $rechargeUrl = "http://localhost:28001/api/protected/recharge"
        $rechargeBody = '{"card_code":"CARD-001"}'
        try {
            $rechargeResponse = Invoke-WebRequest -Uri $rechargeUrl -Method POST -ContentType "application/json" -Headers @{"Authorization"="Bearer $token"} -Body $rechargeBody -UseBasicParsing
            Write-Host "✅ 充值成功！状态码: $($rechargeResponse.StatusCode)"
            Write-Host "响应内容: $($rechargeResponse.Content)"
        } catch {
            Write-Host "❌ 充值失败！"
            Write-Host "错误信息: $($_.Exception.Message)"
            if ($_.Exception.Response) {
                Write-Host "响应状态码: $($_.Exception.Response.StatusCode)"
            }
        }
    } catch {
        Write-Host "❌ 获取用户信息失败！"
        Write-Host "错误信息: $($_.Exception.Message)"
        if ($_.Exception.Response) {
            Write-Host "响应状态码: $($_.Exception.Response.StatusCode)"
        }
    }
} catch {
    Write-Host "❌ 登录失败！"
    Write-Host "错误信息: $($_.Exception.Message)"
    if ($_.Exception.Response) {
        Write-Host "响应状态码: $($_.Exception.Response.StatusCode)"
    }
}
