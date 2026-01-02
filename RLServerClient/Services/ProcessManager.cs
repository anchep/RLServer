using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;

namespace RLServerClient.Services
{
    public class ProcessManager
    {
        private readonly List<Process> _runningProcesses = new List<Process>();
        
        // 启动软件并添加到管理列表，支持参数传递
        public Process StartSoftware(string executablePath, string arguments = "")
        {
            // 检查文件是否存在
            if (!File.Exists(executablePath))
            {
                throw new FileNotFoundException($"Executable file not found: {executablePath}");
            }
            
            Process process = new Process();
            process.StartInfo.FileName = executablePath;
            process.StartInfo.Arguments = arguments;
            process.StartInfo.WorkingDirectory = Path.GetDirectoryName(executablePath);
            process.StartInfo.UseShellExecute = false;
            
            process.Start();
            _runningProcesses.Add(process);
            
            // 清理已退出的进程
            CleanupExitedProcesses();
            
            return process;
        }
        
        // 检查软件是否正在运行
        public bool IsSoftwareRunning(string executableName)
        {
            CleanupExitedProcesses();
            
            return _runningProcesses.Any(p => 
                !p.HasExited && 
                Path.GetFileName(p.StartInfo.FileName).Equals(executableName, StringComparison.OrdinalIgnoreCase));
        }
        
        // 获取运行中的软件进程
        public List<Process> GetRunningProcesses()
        {
            CleanupExitedProcesses();
            return new List<Process>(_runningProcesses);
        }
        
        // 获取运行中的软件进程数量
        public int GetRunningProcessCount()
        {
            CleanupExitedProcesses();
            return _runningProcesses.Count;
        }
        
        // 关闭所有由客户端启动的软件
        public void KillAllProcesses()
        {
            foreach (var process in _runningProcesses)
            {
                if (!process.HasExited)
                {
                    try
                    {
                        // 先尝试正常关闭
                        process.CloseMainWindow();
                        // 等待2秒，若未关闭则强制终止
                        if (!process.WaitForExit(2000))
                        {
                            process.Kill();
                        }
                    }
                    catch (Exception ex)
                    {
                        // 记录异常（这里先简化处理）
                        Console.WriteLine($"Failed to kill process {process.Id}: {ex.Message}");
                    }
                }
            }
            _runningProcesses.Clear();
        }
        
        // 清理已退出的进程
        private void CleanupExitedProcesses()
        {
            _runningProcesses.RemoveAll(p => p.HasExited);
        }
    }
}
