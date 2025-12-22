# 测试充值功能的PowerShell脚本

# 1. 登录获取Token
Write-Host "正在登录..."
$loginResponse = Invoke-WebRequest -Uri "http://localhost:28001/api/auth/login" -Method POST -ContentType "application/json" -Body '{"username":"test_user2","password":"123456","hardware_code":"hw-test","software_version":"v1.0.0"}' -UseBasicParsing
$loginJson = $loginResponse.Content | ConvertFrom-Json
$token = $loginJson.token
Write-Host "登录成功，获取到Token: $token"

# 2. 测试充值功能
Write-Host "`n正在使用卡密CARD-001充值..."
try {
    $rechargeResponse = Invoke-WebRequest -Uri "http://localhost:28001/api/protected/recharge" -Method POST -ContentType "application/json" -Headers @{"Authorization"="Bearer $token"} -Body '{"card_code":"CARD-001"}' -UseBasicParsing
    Write-Host "充值成功！"
    Write-Host "状态码: $($rechargeResponse.StatusCode)"
    Write-Host "响应内容:"
    $rechargeResponse.Content
} catch {
    Write-Host "充值失败:"
    Write-Host "错误信息: $($_.Exception.Message)"
    if ($_.Exception.Response) {
        Write-Host "响应状态码: $($_.Exception.Response.StatusCode)"
        $errorContent = $_.Exception.Response.GetResponseStream() | Get-Content -Encoding utf8
        Write-Host "响应内容: $errorContent"
    }
}
