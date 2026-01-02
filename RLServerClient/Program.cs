using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.IO;

namespace RLServerClient
{
    internal static class Program
    {
        /// <summary>
        /// 应用程序的主入口点。
        /// </summary>
        [STAThread]
        static void Main()
        {
            // 添加全局异常处理
            Application.ThreadException += new System.Threading.ThreadExceptionEventHandler(Application_ThreadException);
            AppDomain.CurrentDomain.UnhandledException += new UnhandledExceptionEventHandler(CurrentDomain_UnhandledException);
            
            try
            {
                Application.EnableVisualStyles();
                Application.SetCompatibleTextRenderingDefault(false);
                Application.Run(new Form1());
            }
            catch (Exception ex)
            {
                ShowException(ex, "Main Exception");
            }
        }
        
        // 处理UI线程异常
        private static void Application_ThreadException(object sender, System.Threading.ThreadExceptionEventArgs e)
        {
            ShowException(e.Exception, "UI Thread Exception");
        }
        
        // 处理非UI线程异常
        private static void CurrentDomain_UnhandledException(object sender, UnhandledExceptionEventArgs e)
        {
            Exception ex = e.ExceptionObject as Exception;
            if (ex != null)
            {
                ShowException(ex, "Unhandled Exception");
            }
        }
        
        // 显示异常信息
        private static void ShowException(Exception ex, string title)
        {
            string message = $"{title}: {ex.Message}\n\nStack Trace:\n{ex.StackTrace}";
            
            // 保存异常信息到日志文件
            try
            {
                string logPath = Path.Combine(Application.StartupPath, "error.log");
                File.WriteAllText(logPath, message);
            }
            catch { }
            
            // 显示异常信息
            MessageBox.Show(message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            Application.Exit();
        }
    }
}