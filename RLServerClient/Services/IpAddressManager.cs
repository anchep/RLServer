using System;
using System.Net;
using System.Net.Http;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace RLServerClient.Services
{
    public class IpAddressManager
    {
        private string _cachedIpAddress = string.Empty;
        private DateTime _lastUpdateTime = DateTime.MinValue;
        private readonly TimeSpan _updateInterval = TimeSpan.FromHours(1);
        private readonly HttpClient _httpClient = new HttpClient();
        private readonly string _ipApiUrl = "https://myip.ipip.net";
        
        public async Task<string> GetPublicIpAddressAsync()
        {
            // 检查缓存是否有效
            if (!string.IsNullOrEmpty(_cachedIpAddress) && 
                (DateTime.Now - _lastUpdateTime) < _updateInterval)
            {
                return _cachedIpAddress;
            }
            
            try
            {
                // 从指定URL获取IP
                string htmlContent = await _httpClient.GetStringAsync(_ipApiUrl);
                string ipAddress = ParseIpFromHtml(htmlContent);
                
                if (!string.IsNullOrEmpty(ipAddress))
                {
                    _cachedIpAddress = ipAddress;
                    _lastUpdateTime = DateTime.Now;
                    return ipAddress;
                }
            }
            catch (Exception ex)
            {
                // 记录异常（这里先简化处理）
                Console.WriteLine($"Failed to get IP from {_ipApiUrl}: {ex.Message}");
            }
            
            // 降级方案：返回本地IP
            return GetLocalIpAddress();
        }
        
        // 解析HTML内容，提取IP地址
        private string ParseIpFromHtml(string htmlContent)
        {
            // 示例返回格式："当前 IP：192.168.1.1 来自：XX省XX市"
            // 使用正则表达式提取IP地址
            var match = Regex.Match(htmlContent, @"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}");
            return match.Success ? match.Value : string.Empty;
        }
        
        // 获取本地IP地址（降级方案）
        private string GetLocalIpAddress()
        {
            var host = Dns.GetHostEntry(Dns.GetHostName());
            foreach (var ip in host.AddressList)
            {
                if (ip.AddressFamily == System.Net.Sockets.AddressFamily.InterNetwork)
                {
                    return ip.ToString();
                }
            }
            return "127.0.0.1";
        }
    }
}