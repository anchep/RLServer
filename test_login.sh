#!/bin/bash

echo "=== 测试登录功能，检查VIP信息是否更新 ==="

# 测试登录功能
echo "\n1. 测试登录功能"
login_response=$(docker run --rm --network rustserver_default curlimages/curl -s -X POST -H "Content-Type: application/json" -d '{"username":"test_user2","password":"123456","hardware_code":"hw-test","software_version":"v1.0.0"}' http://app:28001/api/auth/login)
echo "登录响应: $login_response"

# 检查VIP等级
echo "\n2. 检查VIP等级是否更新为1"
vip_level=$(echo $login_response | docker run --rm -i stedolan/jq -r '.user.vip_level')
echo "VIP等级: $vip_level"

if [ "$vip_level" -eq "1" ]; then
    echo "✅ VIP等级已成功更新为1"
else
    echo "❌ VIP等级更新失败，当前等级: $vip_level"
fi

# 检查VIP过期时间
echo "\n3. 检查VIP过期时间"
vip_expires_at=$(echo $login_response | docker run --rm -i stedolan/jq -r '.user.vip_expires_at')
echo "VIP过期时间: $vip_expires_at"

if [ "$vip_expires_at" != "null" ]; then
    echo "✅ VIP过期时间已设置"
else
    echo "❌ VIP过期时间未设置"
fi
