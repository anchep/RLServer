using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace RLServerClient.Services
{
    public class ApiClient : IDisposable
    {
        private readonly HttpClient _httpClient = new HttpClient();
        private readonly IpAddressManager _ipManager = new IpAddressManager();
        private readonly HardwareIdGenerator _hardwareIdGenerator = new HardwareIdGenerator();
        private readonly string _hardwareId;
        
        // 定义日志事件
        public event Action<string> OnLogAdded;
        
        public string AuthToken { get; set; } = string.Empty;
        public string RefreshToken { get; set; } = string.Empty;
        
        // 获取硬件ID
        public string HardwareId => _hardwareId;
        
        public ApiClient(string baseUrl = "http://localhost:28001")
        {
            // 初始化硬件ID
            _hardwareId = _hardwareIdGenerator.GenerateHardwareId();
            
            // 设置基础地址
            _httpClient.BaseAddress = new Uri(baseUrl);
            
            // 设置默认请求头
            _httpClient.DefaultRequestHeaders.Accept.Add(new MediaTypeWithQualityHeaderValue("application/json"));
        }
        
        // 添加请求头
        private async Task AddRequestHeadersAsync(HttpRequestMessage request)
        {
            // 添加硬件ID
            request.Headers.Add("X-Hardware-Id", _hardwareId);
            
            // 添加IP地址
            string ipAddress = await _ipManager.GetPublicIpAddressAsync();
            request.Headers.Add("X-Client-IP", ipAddress);
            
            // 添加认证头（如果有token）
            if (!string.IsNullOrEmpty(AuthToken))
            {
                request.Headers.Authorization = new AuthenticationHeaderValue("Bearer", AuthToken);
            }
        }
        
        // GET请求
        public async Task<T> GetAsync<T>(string endpoint)
        {
            // 记录请求开始
            string requestId = Guid.NewGuid().ToString().Substring(0, 8);
            DateTime startTime = DateTime.Now;
            
            string log = $"[{startTime:yyyy-MM-dd HH:mm:ss}] [{requestId}] GET {endpoint}";
            OnLogAdded?.Invoke(log);
            
            HttpRequestMessage request = new HttpRequestMessage(HttpMethod.Get, endpoint);
            await AddRequestHeadersAsync(request);
            
            HttpResponseMessage response = await _httpClient.SendAsync(request);
            DateTime endTime = DateTime.Now;
            TimeSpan duration = endTime - startTime;
            
            string responseBody = await response.Content.ReadAsStringAsync();
            
            // 记录响应
            log = $"[{endTime:yyyy-MM-dd HH:mm:ss}] [{requestId}] GET {endpoint} => {response.StatusCode} ({duration.TotalMilliseconds:F2}ms)\n{responseBody}";
            OnLogAdded?.Invoke(log);
            
            await HandleResponseAsync(response);
            
            return JsonConvert.DeserializeObject<T>(responseBody);
        }
        
        // POST请求
        public async Task<T> PostAsync<T>(string endpoint, object data)
        {
            // 记录请求开始
            string requestId = Guid.NewGuid().ToString().Substring(0, 8);
            DateTime startTime = DateTime.Now;
            string requestBody = JsonConvert.SerializeObject(data);
            
            string log = $"[{startTime:yyyy-MM-dd HH:mm:ss}] [{requestId}] POST {endpoint}\n{requestBody}";
            OnLogAdded?.Invoke(log);
            
            HttpRequestMessage request = new HttpRequestMessage(HttpMethod.Post, endpoint)
            {
                Content = new StringContent(requestBody, Encoding.UTF8, "application/json")
            };
            await AddRequestHeadersAsync(request);
            
            HttpResponseMessage response = await _httpClient.SendAsync(request);
            DateTime endTime = DateTime.Now;
            TimeSpan duration = endTime - startTime;
            
            string responseBody = await response.Content.ReadAsStringAsync();
            
            // 记录响应
            log = $"[{endTime:yyyy-MM-dd HH:mm:ss}] [{requestId}] POST {endpoint} => {response.StatusCode} ({duration.TotalMilliseconds:F2}ms)\n{responseBody}";
            OnLogAdded?.Invoke(log);
            
            await HandleResponseAsync(response);
            
            return JsonConvert.DeserializeObject<T>(responseBody);
        }
        
        // 处理响应，包括错误处理和重试逻辑
        private async Task HandleResponseAsync(HttpResponseMessage response)
        {
            // 如果是401未授权，尝试刷新token（这里简化处理，实际应该实现刷新token逻辑）
            if (response.StatusCode == System.Net.HttpStatusCode.Unauthorized)
            {
                // TODO: 实现刷新token逻辑
                throw new UnauthorizedAccessException("Token expired or invalid");
            }
            
            // 如果是404 Not Found，抛出更详细的错误信息
            if (response.StatusCode == System.Net.HttpStatusCode.NotFound)
            {
                throw new HttpRequestException($"API endpoint not found: {response.RequestMessage?.RequestUri?.AbsoluteUri}");
            }
            
            // 确保请求成功，否则抛出详细的错误信息
            try
            {
                response.EnsureSuccessStatusCode();
            }
            catch (HttpRequestException ex)
            {
                // 添加更详细的错误信息
                throw new HttpRequestException($"API request failed: {ex.Message}" + 
                    $"\nURL: {response.RequestMessage?.RequestUri?.AbsoluteUri}" + 
                    $"\nStatus Code: {response.StatusCode}" + 
                    $"\nResponse Content: {await response.Content.ReadAsStringAsync()}");
            }
        }
        
        // 检查服务器状态
        public async Task<bool> CheckServerHealthAsync()
        {
            try
            {
                HttpResponseMessage response = await _httpClient.GetAsync("/health");
                return response.IsSuccessStatusCode;
            }
            catch (Exception)
            {
                return false;
            }
        }
        
        // 获取公网IP地址
        public async Task<string> GetPublicIpAddressAsync()
        {
            return await _ipManager.GetPublicIpAddressAsync();
        }
        
        // 实现IDisposable接口
        public void Dispose()
        {
            _httpClient.Dispose();
        }
    }
}