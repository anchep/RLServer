<#
.SYNOPSIS
生成自签名SSL证书脚本
用于开发环境或测试环境
#>

Write-Host "=== 生成自签名SSL证书 ===" -ForegroundColor Cyan

# 创建SSL证书目录
if (-not (Test-Path -Path "ssl" -PathType Container)) {
    New-Item -Path "ssl" -ItemType Directory | Out-Null
    Write-Host "创建ssl目录成功" -ForegroundColor Green
}

# 生成自签名证书
Write-Host "生成证书..." -ForegroundColor Yellow
try {
    $cert = New-SelfSignedCertificate -Subject "localhost" -DnsName "localhost" -CertStoreLocation "cert:LocalMachineMy" -NotAfter (Get-Date).AddDays(365) -KeyAlgorithm RSA -KeyLength 4096 -HashAlgorithm SHA256 -FriendlyName "RLServer SSL Certificate" -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.1")
    
    # 导出证书和私钥
    $certPath = "cert:LocalMachineMy\$($cert.Thumbprint)"
    $password = ConvertTo-SecureString -String "password" -Force -AsPlainText
    
    # 导出证书（公钥）
    Export-Certificate -Cert $certPath -FilePath "ssl\cert.pem" -Type CERT | Out-Null
    
    # 导出PFX格式（包含私钥）
    Export-PfxCertificate -Cert $certPath -FilePath "ssl\cert.pfx" -Password $password | Out-Null
    
    # 使用OpenSSL将PFX转换为PEM格式（私钥）
    if (Get-Command openssl -ErrorAction SilentlyContinue) {
        openssl pkcs12 -in "ssl\cert.pfx" -nocerts -out "ssl\key.pem" -nodes -password pass:password
        
        # 清理PFX文件
        Remove-Item -Path "ssl\cert.pfx" -Force
        
        Write-Host "=== 证书生成完成 ===" -ForegroundColor Cyan
        Write-Host "证书文件:" -ForegroundColor Yellow
        Write-Host "  ssl/cert.pem - SSL证书" -ForegroundColor Green
        Write-Host "  ssl/key.pem  - 私钥" -ForegroundColor Green
        Write-Host ""
        Write-Host "使用说明:" -ForegroundColor Yellow
        Write-Host "  1. 确保证书文件存在于 ssl/ 目录下" -ForegroundColor Green
        Write-Host "  2. 在生产环境中，请使用真实的SSL证书（如Let's Encrypt）" -ForegroundColor Green
        Write-Host ""
    } else {
        Write-Host "请安装OpenSSL并添加到环境变量，以便生成私钥文件" -ForegroundColor Red
        Write-Host "或手动将PFX文件转换为PEM格式" -ForegroundColor Red
    }
} catch {
    Write-Host "证书生成失败: $($_.Exception.Message)" -ForegroundColor Red
}