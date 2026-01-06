using System;
using System.Management;
using System.Security.Cryptography;
using System.Text;

namespace RLServerClient.Services
{
    public class HardwareIdGenerator
    {
        public string GenerateHardwareId()
        {
            try
            {
                // 获取CPUID
                string cpuId = GetCpuId();
                
                // 获取网卡MAC地址
                string macAddress = GetMacAddress();
                
                // 组合字符串：CPUID + MAC地址 + "280i.com"
                string combinedString = $"{cpuId}{macAddress}280i.com";
                
                // 生成哈希值
                int hashCode = combinedString.GetHashCode();
                
                // 返回硬件ID（转换为十六进制字符串，确保唯一性）
                return hashCode.ToString("X");
            }
            catch (Exception ex)
            {
                // 记录异常（这里先简化处理）
                Console.WriteLine($"Failed to generate hardware ID: {ex.Message}");
                
                // 降级方案：使用随机生成的ID（不推荐，但确保程序能运行）
                return GenerateFallbackHardwareId();
            }
        }
        
        // 获取CPU ID
        private string GetCpuId()
        {
            using (ManagementClass mc = new ManagementClass("win32_processor"))
            {
                using (ManagementObjectCollection moc = mc.GetInstances())
                {
                    foreach (ManagementObject mo in moc)
                    {
                        string cpuId = mo.Properties["processorId"].Value?.ToString() ?? string.Empty;
                        if (!string.IsNullOrEmpty(cpuId))
                        {
                            return cpuId;
                        }
                    }
                }
            }
            
            // 如果无法获取CPUID，使用其他标识
            return GetMotherboardId();
        }
        
        // 获取主板ID（作为CPUID的备选）
        private string GetMotherboardId()
        {
            using (ManagementClass mc = new ManagementClass("Win32_BaseBoard"))
            {
                using (ManagementObjectCollection moc = mc.GetInstances())
                {
                    foreach (ManagementObject mo in moc)
                    {
                        string motherboardId = mo.Properties["SerialNumber"].Value?.ToString() ?? string.Empty;
                        if (!string.IsNullOrEmpty(motherboardId))
                        {
                            return motherboardId;
                        }
                    }
                }
            }
            
            return string.Empty;
        }
        
        // 获取网卡MAC地址
        private string GetMacAddress()
        {
            using (ManagementClass mc = new ManagementClass("Win32_NetworkAdapterConfiguration"))
            {
                using (ManagementObjectCollection moc = mc.GetInstances())
                {
                    foreach (ManagementObject mo in moc)
                    {
                        // 只获取启用的网卡
                        if ((bool?)mo.Properties["IPEnabled"].Value == true)
                        {
                            string macAddress = mo.Properties["MacAddress"].Value?.ToString() ?? string.Empty;
                            if (!string.IsNullOrEmpty(macAddress))
                            {
                                return macAddress.Replace(":", string.Empty);
                            }
                        }
                    }
                }
            }
            
            return string.Empty;
        }
        
        // 降级方案：生成随机硬件ID
        private string GenerateFallbackHardwareId()
        {
            using (RandomNumberGenerator rng = RandomNumberGenerator.Create())
            {
                byte[] randomBytes = new byte[16];
                rng.GetBytes(randomBytes);
                return BitConverter.ToString(randomBytes).Replace("-", string.Empty);
            }
        }
    }
}