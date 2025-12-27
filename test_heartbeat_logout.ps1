# Test script: Verify heartbeat and logout functionality
# Make sure the server is running on port 28001 before running this script

$baseUrl = "http://localhost:28001/api"
$headers = @{}
$token = ""

Write-Host "=== Heartbeat and Logout Functionality Test ===" -ForegroundColor Cyan

# 1. Register a new user
Write-Host "
1. Register a new user"
$random = Get-Random
$username = "test_user_$random"
$password = "Password123!"
$email = "test_$random@example.com"

$registerBody = @{
    username = $username
    password = $password
    email = $email
}

try {
    $registerResponse = Invoke-RestMethod -Uri "$baseUrl/auth/register" -Method Post -Body ($registerBody | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✓ Registration successful: $username" -ForegroundColor Green
} catch {
    Write-Host "✗ Registration failed: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Red
    Write-Host "  Error details: $($_.ErrorDetails.Message)" -ForegroundColor Red
    exit 1
}

# 2. Login to get valid token
Write-Host "
2. Login to get valid token"
$loginBody = @{
    username = $username
    password = $password
    hardware_code = "test_hw"
    software_version = "1.0.0"
}

try {
    $loginResponse = Invoke-RestMethod -Uri "$baseUrl/auth/login" -Method Post -Body ($loginBody | ConvertTo-Json) -ContentType "application/json"
    $token = $loginResponse.token
    $headers = @{Authorization = "Bearer $token"}
    Write-Host "✓ Login successful, token obtained" -ForegroundColor Green
} catch {
    Write-Host "✗ Login failed: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Red
    Write-Host "  Error details: $($_.ErrorDetails.Message)" -ForegroundColor Red
    exit 1
}

# 3. Send heartbeat with valid token
Write-Host "
3. Send heartbeat with valid token"
$validHeartbeatBody = @{
    session_token = $token
    hardware_code = "test_hw"
    software_version = "1.0.0"
}

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/heartbeat" -Method Post -Body ($validHeartbeatBody | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✓ Heartbeat successful: $($response.message)" -ForegroundColor Green
} catch {
    Write-Host "✗ Heartbeat failed: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Red
    Write-Host "  Error details: $($_.ErrorDetails.Message)" -ForegroundColor Red
}

# 4. Send heartbeat with invalid token
Write-Host "
4. Send heartbeat with invalid token"
$invalidHeartbeatBody = @{
    session_token = "invalid_token_$random"
    hardware_code = "test_hw"
    software_version = "1.0.0"
}

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/heartbeat" -Method Post -Body ($invalidHeartbeatBody | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✗ Expected failure, but request succeeded: $($response.message)" -ForegroundColor Yellow
    Write-Host "  Error: Invalid token should return error, but returned success message" -ForegroundColor Red
} catch {
    Write-Host "✓ Expected behavior: Heartbeat request failed" -ForegroundColor Green
    Write-Host "  Status code: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Green
    Write-Host "  Error details: $($_.ErrorDetails.Message)" -ForegroundColor Green
    if ($_.ErrorDetails.Message -like "*invalid token*" -or $_.ErrorDetails.Message -like "*Invalid token*" -or $_.ErrorDetails.Message -like "*No record found*") {
        Write-Host "  ✓ Error message matches expectation: Contains invalid token hint" -ForegroundColor Green
    } else {
        Write-Host "  ✗ Error message does not match expectation: Should contain invalid token hint" -ForegroundColor Yellow
    }
}

# 5. Send logout request with valid token
Write-Host "
5. Send logout request with valid token"
$logoutBody = @{
    session_token = $token
}

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/auth/logout" -Method Post -Body ($logoutBody | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✓ Logout successful: $($response.message)" -ForegroundColor Green
} catch {
    Write-Host "✗ Logout failed: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Red
    Write-Host "  Error details: $($_.ErrorDetails.Message)" -ForegroundColor Red
}

# 6. Send logout request with invalid token
Write-Host "
6. Send logout request with invalid token"
$invalidLogoutBody = @{
    session_token = "invalid_token_$random"
}

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/auth/logout" -Method Post -Body ($invalidLogoutBody | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✗ Expected failure, but request succeeded: $($response.message)" -ForegroundColor Yellow
    Write-Host "  Error: Invalid token should return error, but returned success message" -ForegroundColor Red
} catch {
    Write-Host "✓ Expected behavior: Logout request failed" -ForegroundColor Green
    Write-Host "  Status code: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Green
    Write-Host "  Error details: $($_.ErrorDetails.Message)" -ForegroundColor Green
    if ($_.ErrorDetails.Message -like "*Logout token error*" -or $_.ErrorDetails.Message -like "*Invalid token*" -or $_.ErrorDetails.Message -like "*Unauthorized*" -or $_.ErrorDetails.Message -like "*No record found*" -or $_.ErrorDetails.Message -like "*invalid token*") {
        Write-Host "  ✓ Error message matches expectation: Contains invalid token hint" -ForegroundColor Green
    } else {
        Write-Host "  ✗ Error message does not match expectation: Should contain invalid token hint" -ForegroundColor Yellow
    }
}

# 7. Send heartbeat with original token again (should fail, since already logged out)
Write-Host "
7. Send heartbeat with original token again (should fail, since already logged out)"

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/heartbeat" -Method Post -Body ($validHeartbeatBody | ConvertTo-Json) -ContentType "application/json"
    Write-Host "✗ Expected failure, but request succeeded: $($response.message)" -ForegroundColor Yellow
    Write-Host "  Error: Original token should be invalid after logout, but heartbeat succeeded" -ForegroundColor Red
} catch {
    Write-Host "✓ Expected behavior: Heartbeat request failed" -ForegroundColor Green
    Write-Host "  Status code: $($_.Exception.Response.StatusCode.value__) - $($_.Exception.Response.StatusDescription)" -ForegroundColor Green
    Write-Host "  Error details: $($_.ErrorDetails.Message)" -ForegroundColor Green
    if ($_.ErrorDetails.Message -like "*invalid token*" -or $_.ErrorDetails.Message -like "*Invalid token*" -or $_.ErrorDetails.Message -like "*No record found*" -or $_.ErrorDetails.Message -like "*Unauthorized*") {
        Write-Host "  ✓ Error message matches expectation: Contains invalid token hint" -ForegroundColor Green
    } else {
        Write-Host "  ✗ Error message does not match expectation: Should contain invalid token hint" -ForegroundColor Yellow
    }
}

Write-Host "
=== Test Completed ===" -ForegroundColor Cyan
Write-Host "
Summary:" -ForegroundColor Yellow
Write-Host "- Heartbeat functionality:" -ForegroundColor Yellow
Write-Host "  ✓ Valid token should return success message"
Write-Host "  ✓ Invalid token should return error message, indicating 'token not found'"
Write-Host "
- Logout functionality:" -ForegroundColor Yellow
Write-Host "  ✓ Valid token should return success message"
Write-Host "  ✓ Invalid token should return error message, indicating 'Logout token error'"
Write-Host "  ✓ Original token should be invalid after logout"
Write-Host "
Note: If test results do not match expectations, it may be because the modified code has not been deployed to the server." -ForegroundColor Yellow
