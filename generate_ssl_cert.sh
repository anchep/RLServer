#!/bin/bash

# 生成自签名SSL证书脚本
# 用于开发环境或测试环境

echo "=== 生成自签名SSL证书 ==="

# 创建SSL证书目录
mkdir -p ssl

# 生成自签名证书
echo "生成证书..."
openssl req -x509 -newkey rsa:4096 -keyout ssl/key.pem -out ssl/cert.pem -days 365 -nodes -subj "/CN=localhost"

echo "=== 证书生成完成 ==="
echo "证书文件:"
echo "  ssl/cert.pem - SSL证书"
echo "  ssl/key.pem  - 私钥"
echo ""
echo "使用说明:"
echo "  1. 确保证书文件存在于 ssl/ 目录下"
echo "  2. 在生产环境中，请使用真实的SSL证书（如Let's Encrypt）"
echo ""
