using System;
using System.IO;
using System.IO.Compression;
using System.Net.Http;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace RLServerClient.Utils
{
    public class DownloadManager
    {
        private readonly HttpClient _httpClient = new HttpClient();
        private readonly string _downloadBaseUrl = "http://www.280i.com/soft/280i-Login3/";
        private readonly string _tmpDirectory;
        private readonly string _binDirectory;
        
        // 下载进度委托
        public delegate void DownloadProgressHandler(long totalBytes, long bytesReceived, double percentage);
        public event DownloadProgressHandler DownloadProgressChanged;
        
        public DownloadManager()
        {
            // 初始化目录
            string appDir = Application.StartupPath;
            _tmpDirectory = Path.Combine(appDir, "tmp");
            _binDirectory = Path.Combine(appDir, "bin");
            
            // 确保目录存在
            Directory.CreateDirectory(_tmpDirectory);
            Directory.CreateDirectory(_binDirectory);
        }
        
        // 检查软件是否已存在
        public bool IsSoftwareExist(string executableName)
        {
            string executablePath = Path.Combine(_binDirectory, executableName);
            return File.Exists(executablePath);
        }
        
        // 下载并安装软件
        public async Task<bool> DownloadAndInstallSoftwareAsync(string softwareName, string executableName)
        {
            try
            {
                // 构建下载URL和文件名
                string fileName = $"{softwareName}.zip";
                string downloadUrl = $"{_downloadBaseUrl}{fileName}";
                string tempFilePath = Path.Combine(_tmpDirectory, fileName);
                
                // 下载文件
                bool downloadSuccess = await DownloadFileAsync(downloadUrl, tempFilePath);
                if (!downloadSuccess)
                {
                    return false;
                }
                
                // 解压文件到bin目录
                ExtractZipFile(tempFilePath, _binDirectory);
                
                // 检查可执行文件是否存在
                string executablePath = Path.Combine(_binDirectory, executableName);
                if (!File.Exists(executablePath))
                {
                    throw new FileNotFoundException($"Executable file not found after extraction: {executablePath}");
                }
                
                // 清理临时文件
                File.Delete(tempFilePath);
                
                return true;
            }
            catch (Exception ex)
            {
                // 记录异常（这里先简化处理）
                Console.WriteLine($"Failed to download and install software {softwareName}: {ex.Message}");
                return false;
            }
        }
        
        // 下载文件
        private async Task<bool> DownloadFileAsync(string url, string filePath)
        {
            try
            {
                using (HttpResponseMessage response = await _httpClient.GetAsync(url, HttpCompletionOption.ResponseHeadersRead))
                {
                    response.EnsureSuccessStatusCode();
                    
                    long totalBytes = response.Content.Headers.ContentLength ?? -1L;
                    long bytesReceived = 0;
                    
                    using (Stream contentStream = await response.Content.ReadAsStreamAsync())
                    using (Stream fileStream = new FileStream(filePath, FileMode.Create, FileAccess.Write, FileShare.None, 8192, true))
                    {
                        byte[] buffer = new byte[8192];
                        int bytesRead;
                        
                        while ((bytesRead = await contentStream.ReadAsync(buffer, 0, buffer.Length)) > 0)
                        {
                            await fileStream.WriteAsync(buffer, 0, bytesRead);
                            
                            bytesReceived += bytesRead;
                            
                            // 计算进度并触发事件
                            if (totalBytes > 0)
                            {
                                double percentage = (bytesReceived * 100.0) / totalBytes;
                                DownloadProgressChanged?.Invoke(totalBytes, bytesReceived, percentage);
                            }
                        }
                    }
                }
                
                return true;
            }
            catch (Exception ex)
            {
                // 记录异常（这里先简化处理）
                Console.WriteLine($"Failed to download file from {url}: {ex.Message}");
                return false;
            }
        }
        
        // 解压文件
        private void ExtractZipFile(string zipFilePath, string extractPath)
        {
            try
            {
                ZipFile.ExtractToDirectory(zipFilePath, extractPath, true);
            }
            catch (Exception ex)
            {
                // 记录异常（这里先简化处理）
                Console.WriteLine($"Failed to extract zip file {zipFilePath}: {ex.Message}");
                throw;
            }
        }
    }
}
