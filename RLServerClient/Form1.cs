using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using RLServerClient.Services;
using RLServerClient.Models;
using System.Configuration;
using System.Timers;
using System.IO;
using RLServerClient.Utils;
using System.Reflection;

namespace RLServerClient
{
    public partial class Form1 : Form
    {
        private ApiClient _apiClient;
        private readonly ProcessManager _processManager;
        private readonly DownloadManager _downloadManager;
        private readonly ConfigManager _configManager;
        private readonly System.Timers.Timer _heartbeatTimer;
        private readonly System.Timers.Timer _serverStatusTimer;
        private User _currentUser = null;
        private List<Software> _softwareList = new List<Software>();
        private bool _isServerOnline = false;
        private string _activationToken = string.Empty;
        // 登录状态全局变量
        private bool _isLoggedIn = false;

        // 获取当前软件版本
        private string SoftwareVersion
        {
            get
            {
                try
                {
                    Assembly assembly = Assembly.GetExecutingAssembly();
                    Version version = assembly.GetName().Version;
                    return version.ToString();
                }
                catch (Exception ex)
                {
                    // 如果获取版本失败，返回默认版本
                    return "1.0.0.0";
                }
            }
        }

        public Form1()
        {
            InitializeComponent();

            // 绑定Form1_Load事件
            this.Load += Form1_Load;

            // 初始化ConfigManager
            _configManager = new ConfigManager();

            // 初始化ApiClient
            string serverUrl = ConfigurationManager.AppSettings["ServerUrl"] ?? "http://localhost:28001";
            _apiClient = new ApiClient(serverUrl);
            
            // 订阅API日志事件
            _apiClient.OnLogAdded += (log) =>
            {
                // 确保在UI线程上更新控件
                if (txtApiLog.InvokeRequired)
                {
                    txtApiLog.Invoke(new Action<string>((l) =>
                    {
                        txtApiLog.AppendText(l + Environment.NewLine + Environment.NewLine);
                        // 滚动到底部
                        txtApiLog.SelectionStart = txtApiLog.TextLength;
                        txtApiLog.ScrollToCaret();
                    }), log);
                }
                else
                {
                    txtApiLog.AppendText(log + Environment.NewLine + Environment.NewLine);
                    // 滚动到底部
                    txtApiLog.SelectionStart = txtApiLog.TextLength;
                    txtApiLog.ScrollToCaret();
                }
            };

            // 从本地加载token
            LoadTokenFromLocal();

            // 初始化ProcessManager
            _processManager = new ProcessManager();

            // 初始化DownloadManager
            _downloadManager = new DownloadManager();
            _downloadManager.DownloadProgressChanged += OnDownloadProgressChanged;

            // 初始化心跳定时器（30秒一次）
            _heartbeatTimer = new System.Timers.Timer(30000);
            _heartbeatTimer.Elapsed += OnHeartbeatTimerElapsed;

            // 初始化服务器状态检测定时器（5秒一次，仅在心跳失败时启用）
            _serverStatusTimer = new System.Timers.Timer(5000);
            _serverStatusTimer.Elapsed += OnServerStatusTimerElapsed;
            _serverStatusTimer.Enabled = false;

            // 绑定事件
            this.FormClosing += OnFormClosing;
            btnStartSoftware.Click += OnBtnStartSoftwareClick;
            btnListView.Click += OnBtnListViewClick;
            btnCardView.Click += OnBtnCardViewClick;
            listViewSoftware.SelectedIndexChanged += OnListViewSoftwareSelectedIndexChanged;
            btnRefreshServerStatus.Click += OnBtnRefreshServerStatusClick;
            btnRefreshDebugInfo.Click += OnBtnRefreshDebugInfoClick;

            // 登录/注册页面事件绑定
            btnLogin.Click += OnBtnLoginClick;
            btnSwitchToRegister.Click += OnBtnSwitchToRegisterClick;
            btnSwitchToForgotPassword.Click += OnBtnSwitchToForgotPasswordClick;
            btnRegister.Click += OnBtnRegisterClick;
            btnSwitchToLoginFromRegister.Click += OnBtnSwitchToLoginFromRegisterClick;
            btnVerifyEmail.Click += OnBtnVerifyEmailClick;
            btnResendVerifyCode.Click += OnBtnResendVerifyCodeClick;
            btnSwitchToLoginFromVerify.Click += OnBtnSwitchToLoginFromVerifyClick;
            btnSendResetCode.Click += OnBtnSendResetCodeClick;
            btnSwitchToLoginFromForgot.Click += OnBtnSwitchToLoginFromForgotClick;
            btnResetPassword.Click += OnBtnResetPasswordClick;
            btnSwitchToLoginFromReset.Click += OnBtnSwitchToLoginFromResetClick;
            btnConfigServer.Click += OnBtnConfigServerClick;
            // 充值页面事件绑定
            btnRecharge.Click += OnBtnRechargeClick;
            btnRefreshRechargeRecords.Click += OnBtnRefreshRechargeRecordsClick;

            // 用户信息页面事件绑定
            btnChangePassword.Click += OnBtnChangePasswordClick;
            btnLogout.Click += OnBtnLogoutClick;

            // 显示初始调试信息
            DisplayDebugInfo();

            // 初始化WebView2
            InitializeWebView2Async();

            // 首次检测服务器状态
            CheckServerStatusAsync();

            // 程序启动时不自动获取用户信息和软件列表，只在用户主动登录后才获取
            // 加载软件列表（模拟数据，实际应该从API获取）
            LoadSoftwareList();
        }

        // Form1_Load事件处理函数，用于初始化控件
        private void Form1_Load(object sender, EventArgs e)
        {
            // 确保调试信息标签被添加到groupBoxDebug中
            if (!groupBoxDebug.Controls.Contains(labelServerUrl))
            {
                groupBoxDebug.Controls.Add(labelServerUrl);
                groupBoxDebug.Controls.Add(labelHardwareId);
                groupBoxDebug.Controls.Add(labelSoftwareVersion);
                groupBoxDebug.Controls.Add(labelHeartbeatStatus);
                groupBoxDebug.Controls.Add(labelStatusCheckStatus);
                groupBoxDebug.Controls.Add(labelCurrentUser);
                groupBoxDebug.Controls.Add(labelSoftwareCount);
                groupBoxDebug.Controls.Add(labelProcessCount);
                groupBoxDebug.Controls.Add(btnRefreshDebugInfo);
            }

            // 确保用户信息控件被添加到panelUserInfo中
            if (!panelUserInfo.Controls.Contains(labelUsername))
            {
                panelUserInfo.Controls.Add(labelUsername);
                panelUserInfo.Controls.Add(labelUsernameValue);
                panelUserInfo.Controls.Add(labelVipLevel);
                panelUserInfo.Controls.Add(labelVipLevelValue);
                panelUserInfo.Controls.Add(labelVipExpires);
                panelUserInfo.Controls.Add(labelVipExpiresValue);
            }
        }


        // 初始化WebView2
        private async void InitializeWebView2Async()
        {
            try
            {
                await webView2BuyCards.EnsureCoreWebView2Async();
                // 设置购卡页面URL
                webView2BuyCards.CoreWebView2.Navigate("https://card.280i.com");
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Failed to initialize WebView2: {ex.Message}", "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        // 加载软件列表（模拟数据）
        private void LoadSoftwareList()
        {
            // 模拟数据（实际应该从API获取）
            _softwareList = new List<Software>
            {
                new Software
                {
                    Id = 1,
                    Name = "FishingMan",
                    ChineseName = "钓鱼小精灵",
                    Description = "自动钓鱼，放开双手",
                    Details = "这是一个自动钓鱼软件，可以帮助你自动完成钓鱼任务。",
                    ExecutableName = "FishingMan.exe",
                    Version = "2.3.0",
                    IsActive = true
                },
                new Software
                {
                    Id = 2,
                    Name = "MouseGo",
                    ChineseName = "自动拾取道具",
                    Description = "根据鼠标标停或执行任务",
                    Details = "这是一个自动拾取道具软件，可以帮助你自动拾取游戏中的道具。",
                    ExecutableName = "MouseGo.exe",
                    Version = "1.5.0",
                    IsActive = true
                },
            };

            // 显示软件列表
            DisplaySoftwareList();
        }

        // 从API加载用户可用的软件列表
        private async Task LoadUserSoftwareListAsync()
        {
            if (!_isLoggedIn)
            {
                return;
            }

            try
            {
                // 调用API获取用户可用软件列表
                dynamic response = await _apiClient.GetAsync<dynamic>("/api/protected/users/software");
                // 解析正确的响应格式，提取software_list字段
                _softwareList = Newtonsoft.Json.JsonConvert.DeserializeObject<List<Software>>(response.software_list.ToString());

                // 显示软件列表
                DisplaySoftwareList();

                // 如果当前在软件管理页面，更新卡片视图
                if (tabControl1.SelectedTab == tabPageSoftware && !listViewSoftware.Visible)
                {
                    DisplayCardView();
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"加载软件列表失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
                // 加载失败时，使用模拟数据
                LoadSoftwareList();
            }
        }

        // 显示软件列表
        private void DisplaySoftwareList()
        {
            listViewSoftware.Items.Clear();

            foreach (var software in _softwareList)
            {
                string status = IsSoftwareRunning(software.ExecutableName) ? "运行中" : "未运行";
                ListViewItem item = new ListViewItem(new[]
                {
                    software.Name,
                    software.ChineseName,
                    software.Version,
                    status
                });
                item.Tag = software;
                listViewSoftware.Items.Add(item);
            }
        }

        // 检查软件是否正在运行
        private bool IsSoftwareRunning(string executableName)
        {
            return _processManager.IsSoftwareRunning(executableName);
        }

        // 启动软件
        private async void OnBtnStartSoftwareClick(object sender, EventArgs e)
        {
            if (listViewSoftware.SelectedItems.Count == 0)
            {
                MessageBox.Show("请选择一个软件", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                return;
            }

            Software selectedSoftware = listViewSoftware.SelectedItems[0].Tag as Software;
            if (selectedSoftware == null)
            {
                return;
            }

            try
            {
                // 构建完整的可执行文件路径
                string executablePath = Path.Combine(Application.StartupPath, "bin", selectedSoftware.ExecutableName);

                // 检查文件是否存在，若不存在则下载
                if (!File.Exists(executablePath))
                {
                    DialogResult result = MessageBox.Show($"软件 {selectedSoftware.ChineseName} 未找到，是否下载？", "提示",
                        MessageBoxButtons.YesNo, MessageBoxIcon.Question);
                    if (result == DialogResult.Yes)
                    {
                        // 显示下载进度对话框
                        using (var progressForm = new DownloadProgressForm())
                        {
                            // 下载并安装软件
                            bool success = await _downloadManager.DownloadAndInstallSoftwareAsync(
                                selectedSoftware.Name, selectedSoftware.ExecutableName);

                            if (success)
                            {
                                MessageBox.Show($"软件 {selectedSoftware.ChineseName} 下载完成", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                            }
                            else
                            {
                                MessageBox.Show($"软件 {selectedSoftware.ChineseName} 下载失败", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
                                return;
                            }
                        }
                    }
                    else
                    {
                        return;
                    }
                }

                // 构建参数
                string arguments = BuildSoftwareArguments(selectedSoftware);

                // 启动软件
                _processManager.StartSoftware(executablePath, arguments);

                // 软件启动后，客户端自动最小化
                this.WindowState = FormWindowState.Minimized;

                // 更新软件状态
                DisplaySoftwareList();

                MessageBox.Show($"软件 {selectedSoftware.ChineseName} 已启动", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
            }
            catch (Exception ex)
            {
                MessageBox.Show($"启动软件失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        // 构建软件启动参数（预留功能）
        private string BuildSoftwareArguments(Software software)
        {
            // 基础参数：软件名
            List<string> args = new List<string>();
            args.Add($"--software={software.Name}");

            // 预留：可根据需要添加更多参数
            // 例如：
            // args.Add($"--userId={CurrentUser.Id}");
            // args.Add($"--token={CurrentToken}");
            // args.Add($"--hardwareId={_hardwareId}");

            // 合并软件自带的参数
            if (!string.IsNullOrEmpty(software.StartupArguments))
            {
                args.Add(software.StartupArguments);
            }

            return string.Join(" ", args);
        }

        // 下载进度变化事件
        private void OnDownloadProgressChanged(long totalBytes, long bytesReceived, double percentage)
        {
            // 这里可以更新下载进度显示
            Console.WriteLine($"Download Progress: {percentage:F2}%");
        }

        // 切换到列表视图
        private void OnBtnListViewClick(object sender, EventArgs e)
        {
            listViewSoftware.View = View.Details;
            listViewSoftware.Visible = true;
            flowLayoutPanelCards.Visible = false;
        }

        // 切换到卡片视图
        private void OnBtnCardViewClick(object sender, EventArgs e)
        {
            listViewSoftware.Visible = false;
            flowLayoutPanelCards.Visible = true;
            DisplayCardView();
        }

        // 显示卡片视图
        private void DisplayCardView()
        {
            // 清空现有卡片
            flowLayoutPanelCards.Controls.Clear();

            // 为每个软件创建卡片
            foreach (var software in _softwareList)
            {
                // 创建卡片面板
                Panel card = new Panel();
                card.BorderStyle = BorderStyle.FixedSingle;
                card.Size = new Size(180, 150);
                card.Padding = new Padding(10);
                card.Margin = new Padding(10);
                card.Tag = software;

                // 添加卡片事件处理
                card.Click += OnCardClick;
                card.Cursor = Cursors.Hand;

                // 创建卡片内容
                Label lblName = new Label();
                lblName.Text = software.ChineseName;
                lblName.Font = new Font(lblName.Font, FontStyle.Bold);
                lblName.Location = new Point(0, 0);
                lblName.Size = new Size(160, 20);
                card.Controls.Add(lblName);

                Label lblVersion = new Label();
                lblVersion.Text = $"版本：{software.Version}";
                lblVersion.Location = new Point(0, 30);
                lblVersion.Size = new Size(160, 20);
                card.Controls.Add(lblVersion);

                Label lblStatus = new Label();
                string status = IsSoftwareRunning(software.ExecutableName) ? "运行中" : "未运行";
                lblStatus.Text = $"状态：{status}";
                lblStatus.ForeColor = IsSoftwareRunning(software.ExecutableName) ? Color.Green : Color.Red;
                lblStatus.Location = new Point(0, 60);
                lblStatus.Size = new Size(160, 20);
                card.Controls.Add(lblStatus);

                Label lblDescription = new Label();
                lblDescription.Text = software.Description;
                lblDescription.Location = new Point(0, 90);
                lblDescription.Size = new Size(160, 40);
                lblDescription.AutoSize = false;
                lblDescription.ForeColor = Color.Gray;
                card.Controls.Add(lblDescription);

                // 添加到流布局面板
                flowLayoutPanelCards.Controls.Add(card);
            }
        }

        // 卡片点击事件
        private void OnCardClick(object sender, EventArgs e)
        {
            // 获取被点击的卡片
            Panel card = sender as Panel;
            if (card != null)
            {
                // 获取卡片对应的软件
                Software software = card.Tag as Software;
                if (software != null)
                {
                    // 更新软件详情
                    labelSoftwareDetails.Text = $"{software.ChineseName} - {software.Details}";

                    // 在列表视图中选中对应的项
                    listViewSoftware.SelectedItems.Clear();
                    foreach (ListViewItem item in listViewSoftware.Items)
                    {
                        if (item.Tag == software)
                        {
                            item.Selected = true;
                            break;
                        }
                    }
                }
            }
        }

        // 选中软件时显示详情
        private void OnListViewSoftwareSelectedIndexChanged(object sender, EventArgs e)
        {
            if (listViewSoftware.SelectedItems.Count > 0)
            {
                Software selectedSoftware = listViewSoftware.SelectedItems[0].Tag as Software;
                if (selectedSoftware != null)
                {
                    labelSoftwareDetails.Text = $"{selectedSoftware.ChineseName} - {selectedSoftware.Details}";
                }
            }
        }

        // 心跳定时器事件
        private async void OnHeartbeatTimerElapsed(object sender, ElapsedEventArgs e)
        {
            try
            {
                // 发送心跳请求
                await _apiClient.PostAsync<string>("/api/heartbeat", new { 
                    session_token = _apiClient.AuthToken, 
                    hardware_code = _apiClient.HardwareId, 
                    software_version = SoftwareVersion 
                });

                // 心跳成功
                UpdateServerStatus(true, 0);
            }
            catch (Exception ex)
            {
                // 心跳失败，开启服务器状态检测
                UpdateServerStatus(false, -1);
                _serverStatusTimer.Enabled = true;
            }
        }

        // 服务器状态检测定时器事件
        private async void OnServerStatusTimerElapsed(object sender, ElapsedEventArgs e)
        {
            await CheckServerStatusAsync();
        }

        // 检查服务器状态
        private async Task CheckServerStatusAsync()
        {
            try
            {
                // 测量响应时间
                DateTime startTime = DateTime.Now;

                // 调用健康检查接口
                string result = await _apiClient.GetAsync<string>("/health");

                DateTime endTime = DateTime.Now;
                long responseTime = (long)(endTime - startTime).TotalMilliseconds;

                if (result == "ok")
                {
                    UpdateServerStatus(true, responseTime);
                    _serverStatusTimer.Enabled = false;
                }
                else
                {
                    UpdateServerStatus(false, responseTime);
                }
            }
            catch (Exception ex)
            {
                UpdateServerStatus(false, -1);
                // 更新调试信息，显示具体错误
                this.Invoke((System.Windows.Forms.MethodInvoker)delegate
                {
                    labelStatusCheckStatus.Text = $"状态检测：失败 - {ex.Message}";
                });
            }
        }

        // 更新服务器状态
        private async void UpdateServerStatus(bool isOnline, long responseTime)
        {
            _isServerOnline = isOnline;

            // 获取当前公网IP
            string publicIp = await _apiClient.GetPublicIpAddressAsync();

            // 更新UI（需要Invoke，因为定时器事件在非UI线程）
            this.Invoke((System.Windows.Forms.MethodInvoker)delegate
            {
                labelServerStatus.Text = $"服务器状态：{(isOnline ? "在线" : "离线")}";
                labelServerStatus.ForeColor = isOnline ? Color.Green : Color.Red;
                labelResponseTime.Text = $"响应时间：{(responseTime >= 0 ? $"{responseTime}ms" : "-ms")}";
                labelLastCheckTime.Text = $"上次检测时间：{DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss")}";
                labelPublicIp.Text = $"当前公网IP：{publicIp}";
            });
        }

        // 窗口关闭事件
        private async void OnFormClosing(object sender, FormClosingEventArgs e)
        {
            // 停止定时器
            _heartbeatTimer.Stop();
            _serverStatusTimer.Stop();
            
            // 关闭所有由客户端启动的软件
            _processManager.KillAllProcesses();
            
            // 如果已登录，调用API退出
            if (_isLoggedIn)
            {
                try
                {
                    await _apiClient.PostAsync<object>("/api/auth/logout", new { session_token = _apiClient.AuthToken });
                }
                catch (Exception ex)
                {
                    // 忽略退出API调用失败，确保程序能正常关闭
                    Console.WriteLine($"退出API调用失败：{ex.Message}");
                }
            }
            
            // 释放资源
            _apiClient.Dispose();
            _heartbeatTimer.Dispose();
            _serverStatusTimer.Dispose();
        }

        // 检查VIP到期时间
        private void HandleVipExpirationCheck(User user)
        {
            // 如果VipEndTime为null，则使用CreatedAt作为默认值
            DateTime vipEndTime = user.VipEndTime.GetValueOrDefault(user.CreatedAt);
            // 将UTC时间转换为本地时间
            DateTime localVipEndTime = vipEndTime.ToLocalTime();
            
            // 检查VIP是否即将到期（1天内）
            TimeSpan timeUntilExpiration = localVipEndTime - DateTime.Now;
            if (timeUntilExpiration > TimeSpan.Zero && timeUntilExpiration < TimeSpan.FromDays(1))
            {
                // 获取上次提示日期
                string lastPromptDate = _configManager.GetSetting("LastVipExpirationPromptDate", string.Empty);
                string today = DateTime.Now.ToString("yyyy-MM-dd");

                // 仅当今天未提示过时才显示提示
                if (lastPromptDate != today)
                {
                    string message = $"您的VIP将于{localVipEndTime.ToString("yyyy-MM-dd HH:mm:ss")}到期，请及时续费！";
                    MessageBox.Show(message, "VIP到期提醒", MessageBoxButtons.OK, MessageBoxIcon.Warning);

                    // 更新上次提示日期
                    _configManager.SetSetting("LastVipExpirationPromptDate", today);
                }
            }
        }

        // 更新用户信息显示
        private void UpdateUserInfoDisplay()
        {
            if (_currentUser != null)
            {

                // 更新用户名
                labelUsernameValue.Text = _currentUser.Username;

                // 更新VIP等级
                labelVipLevelValue.Text = _currentUser.VipLevel.ToString();

                // 更新VIP到期时间
                // 如果VipEndTime为null，则使用CreatedAt作为默认值
                DateTime vipEndTime = _currentUser.VipEndTime.GetValueOrDefault(_currentUser.CreatedAt);
                // 将UTC时间转换为本地时间
                DateTime localVipEndTime = vipEndTime.ToLocalTime();
                labelVipExpiresValue.Text = localVipEndTime.ToString("yyyy-MM-dd HH:mm:ss");

            }
            else
            {
                // 如果用户未登录，显示默认值
                labelUsernameValue.Text = "未登录";
                labelVipLevelValue.Text = "未登录";
                labelVipExpiresValue.Text = "未登录";
            }
        }

        // 刷新服务器状态按钮点击事件
        private async void OnBtnRefreshServerStatusClick(object sender, EventArgs e)
        {
            await CheckServerStatusAsync();
        }

        // 刷新调试信息按钮点击事件
        private void OnBtnRefreshDebugInfoClick(object sender, EventArgs e)
        {
            DisplayDebugInfo();
        }

        // 从本地加载token
        private void LoadTokenFromLocal()
        {
            try
            {
                // 从配置管理器获取保存的token
                string accessToken = _configManager.GetSetting("AccessToken", string.Empty);
                string refreshToken = _configManager.GetSetting("RefreshToken", string.Empty);

                if (!string.IsNullOrEmpty(accessToken))
                {
                    // 设置token到ApiClient，但不自动登录
                    _apiClient.AuthToken = accessToken;
                    _apiClient.RefreshToken = refreshToken;
                    // 不自动设置登录状态，用户需要主动登录
                    // _isLoggedIn = true;
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"加载本地token失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        // 显示调试信息
        private void DisplayDebugInfo()
        {
            // 服务器URL
            string serverUrl = ConfigurationManager.AppSettings["ServerUrl"] ?? "http://localhost:28001";
            labelServerUrl.Text = $"服务器URL：{serverUrl}";

            // 硬件ID（需要通过反射获取，因为ApiClient中的_hardwareId是私有字段）
            string hardwareId = "N/A";
            try
            {
                var hardwareIdField = _apiClient.GetType().GetField("_hardwareId", System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Instance);
                if (hardwareIdField != null)
                {
                    hardwareId = hardwareIdField.GetValue(_apiClient) as string;
                }
            }
            catch (Exception ex)
            {
                hardwareId = "获取失败";
            }
            labelHardwareId.Text = $"硬件ID：{hardwareId}";

            // 软件版本
            labelSoftwareVersion.Text = $"软件版本：{SoftwareVersion}";

            // 心跳状态
            labelHeartbeatStatus.Text = $"心跳状态：{(_heartbeatTimer.Enabled ? "已启用" : "已禁用")} (间隔：{_heartbeatTimer.Interval}ms)";

            // 状态检测状态
            labelStatusCheckStatus.Text = $"状态检测：{(_serverStatusTimer.Enabled ? "已启用" : "已禁用")} (间隔：{_serverStatusTimer.Interval}ms)";

            // 当前用户
            string currentUser = _currentUser != null ? $"{_currentUser.Username} ({_currentUser.Email})" : "未登录";
            labelCurrentUser.Text = $"当前用户：{currentUser}";

            // 软件数量
            labelSoftwareCount.Text = $"软件数量：{_softwareList.Count}";

            // 进程数量
            int processCount = _processManager.GetRunningProcessCount();
            labelProcessCount.Text = $"进程数量：{processCount}";
        }

        // 加载用户信息和软件列表
        private async void LoadUserInfoAndSoftwareAsync()
        {
            try
            {
                // 获取用户信息
                var userInfo = await _apiClient.GetAsync<User>("/api/protected/users/me");
                _currentUser = userInfo;

                // 更新调试信息
                DisplayDebugInfo();

                // 显示用户信息
                UpdateUserInfoDisplay();

                // 获取用户可用的软件列表
                await LoadUserSoftwareListAsync();

                // 启动心跳
                _heartbeatTimer.Enabled = true;

                // 检查VIP到期时间
                HandleVipExpirationCheck(_currentUser);
            }
            catch (Exception ex)
            {
                MessageBox.Show($"加载用户信息和软件列表失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
                // 加载失败时，重置登录状态
                _isLoggedIn = false;
                _apiClient.AuthToken = string.Empty;
                _apiClient.RefreshToken = string.Empty;
                _configManager.SetSetting("AccessToken", string.Empty);
                _configManager.SetSetting("RefreshToken", string.Empty);
            }
        }

        // ========================== 登录/注册页面事件处理 ==========================

        // 显示指定面板，隐藏其他面板
        private void ShowPanel(Panel panelToShow)
        {
            // 隐藏所有面板
            panelLogin.Visible = false;
            panelRegister.Visible = false;
            panelVerifyEmail.Visible = false;
            panelForgotPassword.Visible = false;
            panelResetPassword.Visible = false;
            panelUserInfo.Visible = false;

            // 显示指定面板
            panelToShow.Visible = true;
        }

        // 登录按钮点击事件
        private async void OnBtnLoginClick(object sender, EventArgs e)
        {
            try
            {
                // 获取输入的用户名和密码
                string username = txtLoginUsername.Text.Trim();
                string password = txtLoginPassword.Text.Trim();

                // 验证输入
                if (string.IsNullOrEmpty(username) || string.IsNullOrEmpty(password))
                {
                    MessageBox.Show("请输入用户名和密码", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                // 显示加载提示
                Cursor.Current = Cursors.WaitCursor;

                // 获取IP地址
                string ipAddress = await _apiClient.GetPublicIpAddressAsync();

                // 调用登录API
                var loginRequest = new { username, password, hardware_code = _apiClient.HardwareId, software_version = SoftwareVersion, ip_address = ipAddress };
                dynamic loginResponse = null;

                try
                {
                    loginResponse = await _apiClient.PostAsync<dynamic>($"/api/auth/login", loginRequest);
                }
                catch (HttpRequestException httpEx)
                {
                    // 处理HTTP请求异常
                    MessageBox.Show($"登录请求失败：{httpEx.Message}", "网络错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
                    return;
                }

                // 解析登录响应
                if (loginResponse != null)
                {
                    
                    // 检查是否包含token（登录成功）
                    if (loginResponse.token != null)
                    {
                        // 保存认证信息
                        string accessToken = Convert.ToString(loginResponse.token);
                        string refreshToken = Convert.ToString(loginResponse.token ?? string.Empty);
                        _apiClient.AuthToken = accessToken;
                        _apiClient.RefreshToken = refreshToken;

                        // 保存token到本地
                        _configManager.SetSetting("AccessToken", accessToken);
                        _configManager.SetSetting("RefreshToken", refreshToken);

                        // 更新登录状态
                        _isLoggedIn = true;

                        // 获取用户信息
                        var userInfo = await _apiClient.GetAsync<User>("/api/protected/users/me");
                        _currentUser = userInfo;

                        // 更新调试信息
                        DisplayDebugInfo();

                        // 显示用户信息
                        UpdateUserInfoDisplay();

                        // 显示用户信息面板
                        ShowPanel(panelUserInfo);

                        // 启动心跳
                        _heartbeatTimer.Enabled = true;

                        // 检查VIP到期时间
                        HandleVipExpirationCheck(_currentUser);

                        // 获取用户可用的软件列表
                        await LoadUserSoftwareListAsync();
                    }
                    else
                    {
                        // 登录失败，根据API返回的错误信息提示用户
                        string errorMessage = "登录失败，请检查用户名和密码";
                        if (loginResponse.message != null)
                        {
                            errorMessage = Convert.ToString(loginResponse.message);
                        }
                        else if (loginResponse.error != null)
                        {
                            errorMessage = Convert.ToString(loginResponse.error);
                        }

                        MessageBox.Show(errorMessage, "登录失败", MessageBoxButtons.OK, MessageBoxIcon.Error);

                        // 保持在登录面板，让用户可以继续登录操作
                        ShowPanel(panelLogin);
                    }
                }
                else
                {
                    // 无响应情况
                    MessageBox.Show("登录失败，服务器未返回响应", "登录失败", MessageBoxButtons.OK, MessageBoxIcon.Error);

                    // 保持在登录面板，让用户可以继续登录操作
                    ShowPanel(panelLogin);
                }
            }
            catch (UnauthorizedAccessException ex)
            {
                MessageBox.Show("用户名或密码错误", "登录失败", MessageBoxButtons.OK, MessageBoxIcon.Error);

                // 保持在登录面板，让用户可以继续登录操作
                ShowPanel(panelLogin);
            }
            catch (Exception ex)
            {
                // 处理其他异常
                MessageBox.Show($"登录失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);

                // 保持在登录面板，让用户可以继续登录操作
                ShowPanel(panelLogin);
            }
            finally
            {
                Cursor.Current = Cursors.Default;
            }
        }

        // 切换到注册页面
        private void OnBtnSwitchToRegisterClick(object sender, EventArgs e)
        {
            ShowPanel(panelRegister);
        }

        // 切换到忘记密码页面
        private void OnBtnSwitchToForgotPasswordClick(object sender, EventArgs e)
        {
            ShowPanel(panelForgotPassword);
        }

        // 注册按钮点击事件
        private async void OnBtnRegisterClick(object sender, EventArgs e)
        {
            try
            {
                // 获取输入的注册信息
                string username = txtRegisterUsername.Text.Trim();
                string email = txtRegisterEmail.Text.Trim();
                string password = txtRegisterPassword.Text.Trim();
                string confirmPassword = txtRegisterConfirmPassword.Text.Trim();

                // 验证输入
                if (string.IsNullOrEmpty(username) || string.IsNullOrEmpty(email) || string.IsNullOrEmpty(password))
                {
                    MessageBox.Show("请填写所有必填字段", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                if (password != confirmPassword)
                {
                    MessageBox.Show("两次输入的密码不一致", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                // 显示加载提示
                Cursor.Current = Cursors.WaitCursor;

                // 获取IP地址
                string ipAddress = await _apiClient.GetPublicIpAddressAsync();

                // 调用注册API
                var registerRequest = new { username, email, password, hardware_code = _apiClient.HardwareId, software_version = SoftwareVersion, ip_address = ipAddress };
                var registerResponse = await _apiClient.PostAsync<dynamic>($"/api/auth/register", registerRequest);

                // 解析注册响应
                if (registerResponse != null && registerResponse.activation_token != null)
                {
                    // 保存激活令牌
                    _activationToken = registerResponse.activation_token.ToString();

                    // 注册成功，切换到邮箱验证页面
                    ShowPanel(panelVerifyEmail);
                    MessageBox.Show("注册成功，我们已向您的邮箱发送了验证码，请查收。", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"注册失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            finally
            {
                Cursor.Current = Cursors.Default;
            }
        }

        // 从注册页面切换到登录页面
        private void OnBtnSwitchToLoginFromRegisterClick(object sender, EventArgs e)
        {
            ShowPanel(panelLogin);
        }

        // 邮箱验证按钮点击事件
        private async void OnBtnVerifyEmailClick(object sender, EventArgs e)
        {
            try
            {
                // 获取输入的验证码
                string verifyCode = txtVerifyCode.Text.Trim();

                // 验证输入
                if (string.IsNullOrEmpty(verifyCode))
                {
                    MessageBox.Show("请输入验证码", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                // 显示加载提示
                Cursor.Current = Cursors.WaitCursor;

                // 调用邮箱验证API
                var verifyRequest = new { code = verifyCode, token = _activationToken };
                var verifyResponse = await _apiClient.PostAsync<dynamic>($"/api/auth/verify-email", verifyRequest);

                // 解析验证响应
                if (verifyResponse != null)
                {
                    // 验证成功，切换到登录页面
                    ShowPanel(panelLogin);
                    MessageBox.Show("邮箱验证成功", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"验证失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            finally
            {
                Cursor.Current = Cursors.Default;
            }
        }

        // 重新发送验证码按钮点击事件
        private async void OnBtnResendVerifyCodeClick(object sender, EventArgs e)
        {
            try
            {
                // 显示加载提示
                Cursor.Current = Cursors.WaitCursor;

                // 调用重新发送验证码API
                var resendResponse = await _apiClient.PostAsync<dynamic>($"/api/auth/resend-verify-code", new { });

                // 解析响应
                if (resendResponse != null)
                {
                    MessageBox.Show("验证码已重新发送", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"发送验证码失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            finally
            {
                Cursor.Current = Cursors.Default;
            }
        }

        // 从邮箱验证页面切换到登录页面
        private void OnBtnSwitchToLoginFromVerifyClick(object sender, EventArgs e)
        {
            ShowPanel(panelLogin);
        }

        // 发送重置密码验证码按钮点击事件
        private async void OnBtnSendResetCodeClick(object sender, EventArgs e)
        {
            try
            {
                // 获取输入的邮箱
                string email = txtForgotEmail.Text.Trim();

                // 验证输入
                if (string.IsNullOrEmpty(email))
                {
                    MessageBox.Show("请输入邮箱地址", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                // 显示加载提示
                Cursor.Current = Cursors.WaitCursor;

                // 调用发送重置密码验证码API
                var resetCodeRequest = new { email };
                var resetCodeResponse = await _apiClient.PostAsync<dynamic>($"/api/auth/reset-password", resetCodeRequest);

                // 解析响应
                if (resetCodeResponse != null)
                {
                    // 发送成功，切换到重置密码页面
                    ShowPanel(panelResetPassword);
                    MessageBox.Show("重置密码验证码已发送到您的邮箱", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"发送验证码失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            finally
            {
                Cursor.Current = Cursors.Default;
            }
        }

        // 从忘记密码页面切换到登录页面
        private void OnBtnSwitchToLoginFromForgotClick(object sender, EventArgs e)
        {
            ShowPanel(panelLogin);
        }

        // 重置密码按钮点击事件
        private async void OnBtnResetPasswordClick(object sender, EventArgs e)
        {
            try
            {
                // 获取输入的验证码和新密码
                string verifyCode = txtResetVerifyCode.Text.Trim();
                string newPassword = txtResetNewPassword.Text.Trim();
                string confirmPassword = txtResetConfirmPassword.Text.Trim();

                // 验证输入
                if (string.IsNullOrEmpty(verifyCode) || string.IsNullOrEmpty(newPassword))
                {
                    MessageBox.Show("请填写所有必填字段", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                if (newPassword != confirmPassword)
                {
                    MessageBox.Show("两次输入的密码不一致", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                // 显示加载提示
                Cursor.Current = Cursors.WaitCursor;

                // 调用重置密码API
                var resetPasswordRequest = new { email = txtForgotEmail.Text.Trim(), code = verifyCode, new_password = newPassword };
                var resetPasswordResponse = await _apiClient.PostAsync<dynamic>($"/api/auth/reset-password/verify", resetPasswordRequest);

                // 解析响应
                if (resetPasswordResponse != null)
                {
                    // 重置成功，切换到登录页面
                    ShowPanel(panelLogin);
                    MessageBox.Show("密码重置成功", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"重置密码失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            finally
            {
                Cursor.Current = Cursors.Default;
            }
        }

        // 从重置密码页面切换到登录页面
        private void OnBtnSwitchToLoginFromResetClick(object sender, EventArgs e)
        {
            ShowPanel(panelLogin);
        }

        // 配置服务器按钮点击事件
        private void OnBtnConfigServerClick(object sender, EventArgs e)
        {
            try
            {
                // 获取输入的服务器URL
                string newServerUrl = txtServerUrlConfig.Text.Trim();

                // 验证输入
                if (string.IsNullOrEmpty(newServerUrl))
                {
                    MessageBox.Show("请输入服务器URL", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                // 验证URL格式
                if (!Uri.TryCreate(newServerUrl, UriKind.Absolute, out Uri uriResult) ||
                    (uriResult.Scheme != Uri.UriSchemeHttp && uriResult.Scheme != Uri.UriSchemeHttps))
                {
                    MessageBox.Show("请输入有效的HTTP或HTTPS URL", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                // 重新初始化ApiClient
                _apiClient.Dispose();
                _apiClient = new ApiClient(newServerUrl);

                // 更新调试信息
                DisplayDebugInfo();

                // 显示配置成功消息
                MessageBox.Show($"服务器URL配置成功：{newServerUrl}", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
            }
            catch (Exception ex)
            {
                MessageBox.Show($"配置服务器失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        // 充值按钮点击事件
        private async void OnBtnRechargeClick(object sender, EventArgs e)
        {
            try
            {
                // 获取输入的卡密
                string cardCode = txtCardCode.Text.Trim();

                // 验证输入
                if (string.IsNullOrEmpty(cardCode))
                {
                    MessageBox.Show("请输入卡密", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    return;
                }

                // 显示加载提示
                Cursor.Current = Cursors.WaitCursor;

                // 调用充值API
                var rechargeRequest = new { card_code = cardCode };
                var rechargeResponse = await _apiClient.PostAsync<dynamic>($"/api/protected/recharge", rechargeRequest);

                // 解析充值响应
                if (rechargeResponse != null && rechargeResponse.message != null)
                {
                    // 更新充值结果显示
                    lblRechargeResult.Text = rechargeResponse.message.ToString();
                    lblRechargeResult.ForeColor = Color.Green;

                    // 清空卡密输入框
                    txtCardCode.Text = string.Empty;

                    // 更新用户信息
                    var userInfo = await _apiClient.GetAsync<User>("/api/protected/users/me");
                    _currentUser = userInfo;
                    UpdateUserInfoDisplay();
                    DisplayDebugInfo();

                    // 刷新充值记录
                    await LoadRechargeRecordsAsync();
                }
            }
            catch (Exception ex)
            {
                // 更新充值结果显示
                lblRechargeResult.Text = $"充值失败：{ex.Message}";
                lblRechargeResult.ForeColor = Color.Red;
            }
            finally
            {
                Cursor.Current = Cursors.Default;
            }
        }

        // 刷新充值记录按钮点击事件
        private async void OnBtnRefreshRechargeRecordsClick(object sender, EventArgs e)
        {
            await LoadRechargeRecordsAsync();
        }

        // 加载充值记录
        private async Task LoadRechargeRecordsAsync()
        {
            try
            {
                // 显示加载提示
                Cursor.Current = Cursors.WaitCursor;

                // 调用充值记录API
                var rechargeRecords = await _apiClient.GetAsync<List<RechargeRecord>>("/api/protected/recharge/logs");

                // 清空现有记录
                lvRechargeRecords.Items.Clear();

                // 添加新记录
                foreach (var record in rechargeRecords)
                {
                    ListViewItem item = new ListViewItem(new[]
                    {
                        record.CardCode,
                        record.Amount.ToString(),
                        record.CreatedAt.ToString("yyyy-MM-dd HH:mm:ss")
                    });
                    lvRechargeRecords.Items.Add(item);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"加载充值记录失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            finally
            {
                Cursor.Current = Cursors.Default;
            }
        }

        // 修改密码按钮点击事件
        private void OnBtnChangePasswordClick(object sender, EventArgs e)
        {
            // 切换到忘记密码面板，用户可以通过邮箱验证来修改密码
            ShowPanel(panelForgotPassword);
            MessageBox.Show("我们将通过邮箱验证的方式来修改密码，请输入您的注册邮箱。", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
        }

        // 退出登录按钮点击事件
        private async void OnBtnLogoutClick(object sender, EventArgs e)
        {
            try
            {
                // 先调用API退出
                try
                {
                    await _apiClient.PostAsync<object>("/api/auth/logout", new { session_token = _apiClient.AuthToken });
                }
                catch (Exception ex)
                {
                    // 忽略退出API调用失败
                    Console.WriteLine($"退出API调用失败：{ex.Message}");
                }

                // 清除登录状态
                _isLoggedIn = false;

                // 清除token
                _apiClient.AuthToken = string.Empty;
                _apiClient.RefreshToken = string.Empty;

                // 清除本地保存的token
                _configManager.SetSetting("AccessToken", string.Empty);
                _configManager.SetSetting("RefreshToken", string.Empty);

                // 清除当前用户信息
                _currentUser = null;

                // 停止心跳
                _heartbeatTimer.Enabled = false;

                // 重置用户信息显示
                UpdateUserInfoDisplay();

                // 切换到登录面板
                ShowPanel(panelLogin);

                // 清空输入框
                txtLoginUsername.Text = string.Empty;
                txtLoginPassword.Text = string.Empty;

                MessageBox.Show("退出登录成功", "提示", MessageBoxButtons.OK, MessageBoxIcon.Information);
            }
            catch (Exception ex)
            {
                MessageBox.Show($"退出登录失败：{ex.Message}", "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        private void btnSwitchToRegister_Click(object sender, EventArgs e)
        {

        }
    }

    // 下载进度对话框（简化实现）
    public class DownloadProgressForm : Form
    {
        private readonly ProgressBar _progressBar;
        private readonly Label _label;
        
        public DownloadProgressForm()
        {
            this.Text = "下载进度";
            this.Size = new Size(300, 120);
            this.StartPosition = FormStartPosition.CenterParent;
            this.FormBorderStyle = FormBorderStyle.FixedDialog;
            this.MaximizeBox = false;
            this.MinimizeBox = false;
            
            _progressBar = new ProgressBar();
            _progressBar.Dock = DockStyle.Top;
            _progressBar.Height = 20;
            _progressBar.Minimum = 0;
            _progressBar.Maximum = 100;
            
            _label = new Label();
            _label.Dock = DockStyle.Fill;
            _label.TextAlign = ContentAlignment.MiddleCenter;
            _label.Text = "正在下载...";
            
            this.Controls.Add(_label);
            this.Controls.Add(_progressBar);
        }
        
        // 更新进度
        public void UpdateProgress(double percentage)
        {
            this.Invoke((System.Windows.Forms.MethodInvoker)delegate
            {
                _progressBar.Value = (int)Math.Min(percentage, 100);
                _label.Text = $"正在下载... {percentage:F2}%";
            });
        }
    }
}