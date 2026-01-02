using System;
using System.Collections.Generic;
using System.Configuration;
using System.IO;
using System.Xml.Linq;

namespace RLServerClient.Utils
{
    public class ConfigManager
    {
        private readonly string _configFilePath;
        private Dictionary<string, string> _settings = new Dictionary<string, string>();
        
        public ConfigManager()
        {
            // 初始化配置文件路径
            string appDir = AppDomain.CurrentDomain.BaseDirectory;
            _configFilePath = Path.Combine(appDir, "app.config");
            
            // 加载配置
            LoadConfig();
        }
        
        // 加载配置
        private void LoadConfig()
        {
            try
            {
                if (File.Exists(_configFilePath))
                {
                    XDocument doc = XDocument.Load(_configFilePath);
                    XElement appSettings = doc.Element("configuration")?.Element("appSettings");
                    
                    if (appSettings != null)
                    {
                        foreach (XElement setting in appSettings.Elements("add"))
                        {
                            string key = setting.Attribute("key")?.Value ?? string.Empty;
                            string value = setting.Attribute("value")?.Value ?? string.Empty;
                            
                            if (!string.IsNullOrEmpty(key))
                            {
                                _settings[key] = value;
                            }
                        }
                    }
                }
            }
            catch (Exception ex)
            {
                // 记录异常（这里先简化处理）
                Console.WriteLine($"Failed to load config: {ex.Message}");
                _settings = new Dictionary<string, string>();
            }
        }
        
        // 保存配置
        private void SaveConfig()
        {
            try
            {
                XDocument doc = new XDocument(
                    new XDeclaration("1.0", "utf-8", "yes"),
                    new XElement("configuration",
                        new XElement("appSettings")
                    )
                );
                
                XElement appSettings = doc.Element("configuration")?.Element("appSettings");
                if (appSettings != null)
                {
                    foreach (var setting in _settings)
                    {
                        appSettings.Add(new XElement("add",
                            new XAttribute("key", setting.Key),
                            new XAttribute("value", setting.Value)
                        ));
                    }
                }
                
                doc.Save(_configFilePath);
            }
            catch (Exception ex)
            {
                // 记录异常（这里先简化处理）
                Console.WriteLine($"Failed to save config: {ex.Message}");
            }
        }
        
        // 获取配置值
        public string GetSetting(string key, string defaultValue = "")
        {
            if (_settings.TryGetValue(key, out string value))
            {
                return value;
            }
            return defaultValue;
        }
        
        // 设置配置值
        public void SetSetting(string key, string value)
        {
            _settings[key] = value;
            SaveConfig();
        }
    }
}
