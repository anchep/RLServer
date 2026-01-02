namespace RLServerClient
{
    partial class Form1
    {
        /// <summary>
        /// 必需的设计器变量。
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// 清理所有正在使用的资源。
        /// </summary>
        /// <param name="disposing">如果应释放托管资源，为 true；否则为 false。</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows 窗体设计器生成的代码

        /// <summary>
        /// 设计器支持所需的方法 - 不要修改
        /// 使用代码编辑器修改此方法的内容。
        /// </summary>
        private void InitializeComponent()
        {
            tabControl1 = new TabControl();
            tabPageStatus = new TabPage();
            btnRefreshServerStatus = new Button();
            groupBoxDebug = new GroupBox();
            labelServerStatus = new Label();
            labelResponseTime = new Label();
            labelLastCheckTime = new Label();
            labelPublicIp = new Label();
            tabPageLogin = new TabPage();
            panelUserInfo = new Panel();
            btnLogout = new Button();
            btnChangePassword = new Button();
            labelVipExpiresValue = new Label();
            labelVipExpires = new Label();
            labelVipLevelValue = new Label();
            labelVipLevel = new Label();
            labelUsernameValue = new Label();
            labelUsername = new Label();
            panelForgotPassword = new Panel();
            lblForgotPasswordTitle = new Label();
            lblForgotPasswordDesc = new Label();
            lblForgotEmail = new Label();
            txtForgotEmail = new TextBox();
            btnSendResetCode = new Button();
            btnSwitchToLoginFromForgot = new Button();
            panelRegister = new Panel();
            lblRegisterUsername = new Label();
            txtRegisterUsername = new TextBox();
            lblRegisterEmail = new Label();
            txtRegisterEmail = new TextBox();
            lblRegisterPassword = new Label();
            txtRegisterPassword = new TextBox();
            lblRegisterConfirmPassword = new Label();
            txtRegisterConfirmPassword = new TextBox();
            btnRegister = new Button();
            btnSwitchToLoginFromRegister = new Button();
            panelLogin = new Panel();
            lblLoginUsername = new Label();
            txtLoginUsername = new TextBox();
            lblLoginPassword = new Label();
            txtLoginPassword = new TextBox();
            btnLogin = new Button();
            btnSwitchToRegister = new Button();
            btnSwitchToForgotPassword = new Button();
            btnConfigServer = new Button();
            lblServerUrlConfig = new Label();
            txtServerUrlConfig = new TextBox();
            panelVerifyEmail = new Panel();
            lblVerifyEmailTitle = new Label();
            lblVerifyEmailDesc = new Label();
            lblVerifyCode = new Label();
            txtVerifyCode = new TextBox();
            btnVerifyEmail = new Button();
            btnResendVerifyCode = new Button();
            btnSwitchToLoginFromVerify = new Button();
            panelResetPassword = new Panel();
            lblResetPasswordTitle = new Label();
            lblResetPasswordDesc = new Label();
            lblResetVerifyCode = new Label();
            txtResetVerifyCode = new TextBox();
            lblResetNewPassword = new Label();
            txtResetNewPassword = new TextBox();
            lblResetConfirmPassword = new Label();
            txtResetConfirmPassword = new TextBox();
            btnResetPassword = new Button();
            btnSwitchToLoginFromReset = new Button();
            tabPageSoftware = new TabPage();
            labelSoftwareDetails = new Label();
            btnStartSoftware = new Button();
            listViewSoftware = new ListView();
            columnHeaderName = new ColumnHeader();
            columnHeaderChineseName = new ColumnHeader();
            columnHeaderVersion = new ColumnHeader();
            columnHeaderStatus = new ColumnHeader();
            flowLayoutPanelCards = new FlowLayoutPanel();
            btnCardView = new Button();
            btnListView = new Button();
            tabPageRecharge = new TabPage();
            lvRechargeRecords = new ListView();
            chCardCode = new ColumnHeader();
            chAmount = new ColumnHeader();
            chCreatedAt = new ColumnHeader();
            btnRefreshRechargeRecords = new Button();
            lblRechargeRecords = new Label();
            btnRecharge = new Button();
            txtCardCode = new TextBox();
            lblCardCode = new Label();
            lblRechargeResult = new Label();
            tabPageBuyCards = new TabPage();
            webView2BuyCards = new Microsoft.Web.WebView2.WinForms.WebView2();
            tabPage1 = new TabPage();
            txtApiLog = new TextBox();
            labelServerUrl = new Label();
            labelHardwareId = new Label();
            labelSoftwareVersion = new Label();
            labelHeartbeatStatus = new Label();
            labelStatusCheckStatus = new Label();
            labelCurrentUser = new Label();
            labelSoftwareCount = new Label();
            labelProcessCount = new Label();
            btnRefreshDebugInfo = new Button();
            tabControl1.SuspendLayout();
            tabPageStatus.SuspendLayout();
            tabPageLogin.SuspendLayout();
            panelUserInfo.SuspendLayout();
            panelForgotPassword.SuspendLayout();
            panelRegister.SuspendLayout();
            panelLogin.SuspendLayout();
            panelVerifyEmail.SuspendLayout();
            panelResetPassword.SuspendLayout();
            tabPageSoftware.SuspendLayout();
            tabPageRecharge.SuspendLayout();
            tabPageBuyCards.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)webView2BuyCards).BeginInit();
            tabPage1.SuspendLayout();
            SuspendLayout();
            // 
            // tabControl1
            // 
            tabControl1.Controls.Add(tabPageStatus);
            tabControl1.Controls.Add(tabPageLogin);
            tabControl1.Controls.Add(tabPageSoftware);
            tabControl1.Controls.Add(tabPageRecharge);
            tabControl1.Controls.Add(tabPageBuyCards);
            tabControl1.Controls.Add(tabPage1);
            tabControl1.Dock = DockStyle.Fill;
            tabControl1.Location = new Point(0, 0);
            tabControl1.Margin = new Padding(4);
            tabControl1.Name = "tabControl1";
            tabControl1.SelectedIndex = 0;
            tabControl1.Size = new Size(949, 624);
            tabControl1.TabIndex = 0;
            // 
            // tabPageStatus
            // 
            tabPageStatus.Controls.Add(btnRefreshServerStatus);
            tabPageStatus.Controls.Add(groupBoxDebug);
            tabPageStatus.Controls.Add(labelServerStatus);
            tabPageStatus.Controls.Add(labelResponseTime);
            tabPageStatus.Controls.Add(labelLastCheckTime);
            tabPageStatus.Controls.Add(labelPublicIp);
            tabPageStatus.Location = new Point(4, 26);
            tabPageStatus.Margin = new Padding(4);
            tabPageStatus.Name = "tabPageStatus";
            tabPageStatus.Padding = new Padding(4);
            tabPageStatus.Size = new Size(941, 594);
            tabPageStatus.TabIndex = 0;
            tabPageStatus.Text = "服务器状态";
            tabPageStatus.UseVisualStyleBackColor = true;
            // 
            // btnRefreshServerStatus
            // 
            btnRefreshServerStatus.Location = new Point(23, 198);
            btnRefreshServerStatus.Margin = new Padding(4);
            btnRefreshServerStatus.Name = "btnRefreshServerStatus";
            btnRefreshServerStatus.Size = new Size(88, 33);
            btnRefreshServerStatus.TabIndex = 4;
            btnRefreshServerStatus.Text = "刷新状态";
            btnRefreshServerStatus.UseVisualStyleBackColor = true;
            // 
            // groupBoxDebug
            // 
            groupBoxDebug.Location = new Point(23, 255);
            groupBoxDebug.Margin = new Padding(4);
            groupBoxDebug.Name = "groupBoxDebug";
            groupBoxDebug.Padding = new Padding(4);
            groupBoxDebug.Size = new Size(875, 312);
            groupBoxDebug.TabIndex = 5;
            groupBoxDebug.TabStop = false;
            groupBoxDebug.Text = "调试信息";
            // 
            // labelServerStatus
            // 
            labelServerStatus.AutoSize = true;
            labelServerStatus.Location = new Point(23, 28);
            labelServerStatus.Margin = new Padding(4, 0, 4, 0);
            labelServerStatus.Name = "labelServerStatus";
            labelServerStatus.Size = new Size(80, 17);
            labelServerStatus.TabIndex = 0;
            labelServerStatus.Text = "服务器状态：";
            // 
            // labelResponseTime
            // 
            labelResponseTime.AutoSize = true;
            labelResponseTime.Location = new Point(23, 71);
            labelResponseTime.Margin = new Padding(4, 0, 4, 0);
            labelResponseTime.Name = "labelResponseTime";
            labelResponseTime.Size = new Size(68, 17);
            labelResponseTime.TabIndex = 1;
            labelResponseTime.Text = "响应时间：";
            // 
            // labelLastCheckTime
            // 
            labelLastCheckTime.AutoSize = true;
            labelLastCheckTime.Location = new Point(23, 113);
            labelLastCheckTime.Margin = new Padding(4, 0, 4, 0);
            labelLastCheckTime.Name = "labelLastCheckTime";
            labelLastCheckTime.Size = new Size(92, 17);
            labelLastCheckTime.TabIndex = 2;
            labelLastCheckTime.Text = "上次检测时间：";
            // 
            // labelPublicIp
            // 
            labelPublicIp.AutoSize = true;
            labelPublicIp.Location = new Point(23, 156);
            labelPublicIp.Margin = new Padding(4, 0, 4, 0);
            labelPublicIp.Name = "labelPublicIp";
            labelPublicIp.Size = new Size(79, 17);
            labelPublicIp.TabIndex = 3;
            labelPublicIp.Text = "当前公网IP：";
            // 
            // tabPageLogin
            // 
            tabPageLogin.Controls.Add(panelUserInfo);
            tabPageLogin.Controls.Add(panelForgotPassword);
            tabPageLogin.Controls.Add(panelRegister);
            tabPageLogin.Controls.Add(panelLogin);
            tabPageLogin.Controls.Add(panelVerifyEmail);
            tabPageLogin.Controls.Add(panelResetPassword);
            tabPageLogin.Location = new Point(4, 26);
            tabPageLogin.Margin = new Padding(4);
            tabPageLogin.Name = "tabPageLogin";
            tabPageLogin.Padding = new Padding(4);
            tabPageLogin.Size = new Size(941, 594);
            tabPageLogin.TabIndex = 1;
            tabPageLogin.Text = "登录/注册";
            tabPageLogin.UseVisualStyleBackColor = true;
            // 
            // panelUserInfo
            // 
            panelUserInfo.Controls.Add(btnLogout);
            panelUserInfo.Controls.Add(btnChangePassword);
            panelUserInfo.Controls.Add(labelVipExpiresValue);
            panelUserInfo.Controls.Add(labelVipExpires);
            panelUserInfo.Controls.Add(labelVipLevelValue);
            panelUserInfo.Controls.Add(labelVipLevel);
            panelUserInfo.Controls.Add(labelUsernameValue);
            panelUserInfo.Controls.Add(labelUsername);
            panelUserInfo.Dock = DockStyle.Fill;
            panelUserInfo.Location = new Point(4, 4);
            panelUserInfo.Margin = new Padding(4);
            panelUserInfo.Name = "panelUserInfo";
            panelUserInfo.Size = new Size(933, 586);
            panelUserInfo.TabIndex = 6;
            panelUserInfo.Visible = false;
            // 
            // btnLogout
            // 
            btnLogout.Location = new Point(401, 327);
            btnLogout.Name = "btnLogout";
            btnLogout.Size = new Size(120, 30);
            btnLogout.TabIndex = 11;
            btnLogout.Text = "退出登录";
            btnLogout.UseVisualStyleBackColor = true;
            // 
            // btnChangePassword
            // 
            btnChangePassword.Location = new Point(233, 327);
            btnChangePassword.Name = "btnChangePassword";
            btnChangePassword.Size = new Size(120, 30);
            btnChangePassword.TabIndex = 10;
            btnChangePassword.Text = "修改密码";
            btnChangePassword.UseVisualStyleBackColor = true;
            // 
            // labelVipExpiresValue
            // 
            labelVipExpiresValue.AutoSize = true;
            labelVipExpiresValue.Location = new Point(115, 140);
            labelVipExpiresValue.Name = "labelVipExpiresValue";
            labelVipExpiresValue.Size = new Size(44, 17);
            labelVipExpiresValue.TabIndex = 9;
            labelVipExpiresValue.Text = "未登录";
            // 
            // labelVipExpires
            // 
            labelVipExpires.AutoSize = true;
            labelVipExpires.Location = new Point(20, 140);
            labelVipExpires.Name = "labelVipExpires";
            labelVipExpires.Size = new Size(87, 17);
            labelVipExpires.TabIndex = 8;
            labelVipExpires.Text = "VIP到期时间：";
            // 
            // labelVipLevelValue
            // 
            labelVipLevelValue.AutoSize = true;
            labelVipLevelValue.Location = new Point(90, 110);
            labelVipLevelValue.Name = "labelVipLevelValue";
            labelVipLevelValue.Size = new Size(44, 17);
            labelVipLevelValue.TabIndex = 7;
            labelVipLevelValue.Text = "未登录";
            // 
            // labelVipLevel
            // 
            labelVipLevel.AutoSize = true;
            labelVipLevel.Location = new Point(20, 110);
            labelVipLevel.Name = "labelVipLevel";
            labelVipLevel.Size = new Size(63, 17);
            labelVipLevel.TabIndex = 6;
            labelVipLevel.Text = "VIP等级：";
            // 
            // labelUsernameValue
            // 
            labelUsernameValue.AutoSize = true;
            labelUsernameValue.Location = new Point(90, 50);
            labelUsernameValue.Name = "labelUsernameValue";
            labelUsernameValue.Size = new Size(44, 17);
            labelUsernameValue.TabIndex = 3;
            labelUsernameValue.Text = "未登录";
            // 
            // labelUsername
            // 
            labelUsername.AutoSize = true;
            labelUsername.Location = new Point(20, 50);
            labelUsername.Name = "labelUsername";
            labelUsername.Size = new Size(56, 17);
            labelUsername.TabIndex = 2;
            labelUsername.Text = "用户名：";
            // 
            // panelForgotPassword
            // 
            panelForgotPassword.Controls.Add(lblForgotPasswordTitle);
            panelForgotPassword.Controls.Add(lblForgotPasswordDesc);
            panelForgotPassword.Controls.Add(lblForgotEmail);
            panelForgotPassword.Controls.Add(txtForgotEmail);
            panelForgotPassword.Controls.Add(btnSendResetCode);
            panelForgotPassword.Controls.Add(btnSwitchToLoginFromForgot);
            panelForgotPassword.Dock = DockStyle.Fill;
            panelForgotPassword.Location = new Point(4, 4);
            panelForgotPassword.Margin = new Padding(4);
            panelForgotPassword.Name = "panelForgotPassword";
            panelForgotPassword.Size = new Size(933, 586);
            panelForgotPassword.TabIndex = 3;
            panelForgotPassword.Visible = false;
            // 
            // lblForgotPasswordTitle
            // 
            lblForgotPasswordTitle.AutoSize = true;
            lblForgotPasswordTitle.Font = new Font("宋体", 12F, FontStyle.Bold, GraphicsUnit.Point, 134);
            lblForgotPasswordTitle.Location = new Point(264, 20);
            lblForgotPasswordTitle.Name = "lblForgotPasswordTitle";
            lblForgotPasswordTitle.Size = new Size(75, 16);
            lblForgotPasswordTitle.TabIndex = 0;
            lblForgotPasswordTitle.Text = "忘记密码";
            // 
            // lblForgotPasswordDesc
            // 
            lblForgotPasswordDesc.AutoSize = true;
            lblForgotPasswordDesc.Location = new Point(160, 50);
            lblForgotPasswordDesc.Name = "lblForgotPasswordDesc";
            lblForgotPasswordDesc.Size = new Size(284, 34);
            lblForgotPasswordDesc.TabIndex = 1;
            lblForgotPasswordDesc.Text = "请输入您的注册邮箱，我们将发送验证码到您的邮箱\n用于重置密码。";
            // 
            // lblForgotEmail
            // 
            lblForgotEmail.AutoSize = true;
            lblForgotEmail.Location = new Point(289, 129);
            lblForgotEmail.Name = "lblForgotEmail";
            lblForgotEmail.Size = new Size(44, 17);
            lblForgotEmail.TabIndex = 2;
            lblForgotEmail.Text = "邮箱：";
            // 
            // txtForgotEmail
            // 
            txtForgotEmail.Location = new Point(236, 176);
            txtForgotEmail.Name = "txtForgotEmail";
            txtForgotEmail.Size = new Size(150, 23);
            txtForgotEmail.TabIndex = 3;
            // 
            // btnSendResetCode
            // 
            btnSendResetCode.Location = new Point(261, 229);
            btnSendResetCode.Name = "btnSendResetCode";
            btnSendResetCode.Size = new Size(100, 30);
            btnSendResetCode.TabIndex = 4;
            btnSendResetCode.Text = "发送验证码";
            btnSendResetCode.UseVisualStyleBackColor = true;
            // 
            // btnSwitchToLoginFromForgot
            // 
            btnSwitchToLoginFromForgot.Location = new Point(251, 289);
            btnSwitchToLoginFromForgot.Name = "btnSwitchToLoginFromForgot";
            btnSwitchToLoginFromForgot.Size = new Size(120, 30);
            btnSwitchToLoginFromForgot.TabIndex = 5;
            btnSwitchToLoginFromForgot.Text = "返回登录";
            btnSwitchToLoginFromForgot.UseVisualStyleBackColor = true;
            // 
            // panelRegister
            // 
            panelRegister.Controls.Add(lblRegisterUsername);
            panelRegister.Controls.Add(txtRegisterUsername);
            panelRegister.Controls.Add(lblRegisterEmail);
            panelRegister.Controls.Add(txtRegisterEmail);
            panelRegister.Controls.Add(lblRegisterPassword);
            panelRegister.Controls.Add(txtRegisterPassword);
            panelRegister.Controls.Add(lblRegisterConfirmPassword);
            panelRegister.Controls.Add(txtRegisterConfirmPassword);
            panelRegister.Controls.Add(btnRegister);
            panelRegister.Controls.Add(btnSwitchToLoginFromRegister);
            panelRegister.Dock = DockStyle.Fill;
            panelRegister.Location = new Point(4, 4);
            panelRegister.Margin = new Padding(4);
            panelRegister.Name = "panelRegister";
            panelRegister.Size = new Size(933, 586);
            panelRegister.TabIndex = 1;
            panelRegister.Visible = false;
            // 
            // lblRegisterUsername
            // 
            lblRegisterUsername.AutoSize = true;
            lblRegisterUsername.Location = new Point(255, 33);
            lblRegisterUsername.Name = "lblRegisterUsername";
            lblRegisterUsername.Size = new Size(56, 17);
            lblRegisterUsername.TabIndex = 0;
            lblRegisterUsername.Text = "用户名：";
            // 
            // txtRegisterUsername
            // 
            txtRegisterUsername.Location = new Point(208, 53);
            txtRegisterUsername.Name = "txtRegisterUsername";
            txtRegisterUsername.Size = new Size(150, 23);
            txtRegisterUsername.TabIndex = 1;
            // 
            // lblRegisterEmail
            // 
            lblRegisterEmail.AutoSize = true;
            lblRegisterEmail.Location = new Point(261, 96);
            lblRegisterEmail.Name = "lblRegisterEmail";
            lblRegisterEmail.Size = new Size(44, 17);
            lblRegisterEmail.TabIndex = 2;
            lblRegisterEmail.Text = "邮箱：";
            // 
            // txtRegisterEmail
            // 
            txtRegisterEmail.Location = new Point(208, 113);
            txtRegisterEmail.Name = "txtRegisterEmail";
            txtRegisterEmail.Size = new Size(150, 23);
            txtRegisterEmail.TabIndex = 3;
            // 
            // lblRegisterPassword
            // 
            lblRegisterPassword.AutoSize = true;
            lblRegisterPassword.Location = new Point(261, 160);
            lblRegisterPassword.Name = "lblRegisterPassword";
            lblRegisterPassword.Size = new Size(44, 17);
            lblRegisterPassword.TabIndex = 4;
            lblRegisterPassword.Text = "密码：";
            // 
            // txtRegisterPassword
            // 
            txtRegisterPassword.Location = new Point(208, 180);
            txtRegisterPassword.Name = "txtRegisterPassword";
            txtRegisterPassword.Size = new Size(150, 23);
            txtRegisterPassword.TabIndex = 5;
            txtRegisterPassword.UseSystemPasswordChar = true;
            // 
            // lblRegisterConfirmPassword
            // 
            lblRegisterConfirmPassword.AutoSize = true;
            lblRegisterConfirmPassword.Location = new Point(249, 220);
            lblRegisterConfirmPassword.Name = "lblRegisterConfirmPassword";
            lblRegisterConfirmPassword.Size = new Size(68, 17);
            lblRegisterConfirmPassword.TabIndex = 6;
            lblRegisterConfirmPassword.Text = "确认密码：";
            // 
            // txtRegisterConfirmPassword
            // 
            txtRegisterConfirmPassword.Location = new Point(208, 239);
            txtRegisterConfirmPassword.Name = "txtRegisterConfirmPassword";
            txtRegisterConfirmPassword.Size = new Size(150, 23);
            txtRegisterConfirmPassword.TabIndex = 7;
            txtRegisterConfirmPassword.UseSystemPasswordChar = true;
            // 
            // btnRegister
            // 
            btnRegister.Location = new Point(233, 302);
            btnRegister.Name = "btnRegister";
            btnRegister.Size = new Size(100, 30);
            btnRegister.TabIndex = 8;
            btnRegister.Text = "注册";
            btnRegister.UseVisualStyleBackColor = true;
            // 
            // btnSwitchToLoginFromRegister
            // 
            btnSwitchToLoginFromRegister.Location = new Point(215, 370);
            btnSwitchToLoginFromRegister.Name = "btnSwitchToLoginFromRegister";
            btnSwitchToLoginFromRegister.Size = new Size(136, 30);
            btnSwitchToLoginFromRegister.TabIndex = 9;
            btnSwitchToLoginFromRegister.Text = "已有账号？点击登录";
            btnSwitchToLoginFromRegister.UseVisualStyleBackColor = true;
            // 
            // panelLogin
            // 
            panelLogin.Controls.Add(lblLoginUsername);
            panelLogin.Controls.Add(txtLoginUsername);
            panelLogin.Controls.Add(lblLoginPassword);
            panelLogin.Controls.Add(txtLoginPassword);
            panelLogin.Controls.Add(btnLogin);
            panelLogin.Controls.Add(btnSwitchToRegister);
            panelLogin.Controls.Add(btnSwitchToForgotPassword);
            panelLogin.Controls.Add(btnConfigServer);
            panelLogin.Controls.Add(lblServerUrlConfig);
            panelLogin.Controls.Add(txtServerUrlConfig);
            panelLogin.Dock = DockStyle.Fill;
            panelLogin.Location = new Point(4, 4);
            panelLogin.Margin = new Padding(4);
            panelLogin.Name = "panelLogin";
            panelLogin.Size = new Size(933, 586);
            panelLogin.TabIndex = 0;
            // 
            // lblLoginUsername
            // 
            lblLoginUsername.AutoSize = true;
            lblLoginUsername.Location = new Point(264, 33);
            lblLoginUsername.Name = "lblLoginUsername";
            lblLoginUsername.Size = new Size(56, 17);
            lblLoginUsername.TabIndex = 0;
            lblLoginUsername.Text = "用户名：";
            // 
            // txtLoginUsername
            // 
            txtLoginUsername.Location = new Point(208, 64);
            txtLoginUsername.Name = "txtLoginUsername";
            txtLoginUsername.Size = new Size(168, 23);
            txtLoginUsername.TabIndex = 1;
            // 
            // lblLoginPassword
            // 
            lblLoginPassword.AutoSize = true;
            lblLoginPassword.Location = new Point(270, 96);
            lblLoginPassword.Name = "lblLoginPassword";
            lblLoginPassword.Size = new Size(44, 17);
            lblLoginPassword.TabIndex = 2;
            lblLoginPassword.Text = "密码：";
            // 
            // txtLoginPassword
            // 
            txtLoginPassword.Location = new Point(208, 120);
            txtLoginPassword.Name = "txtLoginPassword";
            txtLoginPassword.Size = new Size(168, 23);
            txtLoginPassword.TabIndex = 3;
            txtLoginPassword.UseSystemPasswordChar = true;
            // 
            // btnLogin
            // 
            btnLogin.Location = new Point(208, 170);
            btnLogin.Name = "btnLogin";
            btnLogin.Size = new Size(168, 30);
            btnLogin.TabIndex = 4;
            btnLogin.Text = "登录";
            btnLogin.UseVisualStyleBackColor = true;
            // 
            // btnSwitchToRegister
            // 
            btnSwitchToRegister.Location = new Point(227, 236);
            btnSwitchToRegister.Name = "btnSwitchToRegister";
            btnSwitchToRegister.Size = new Size(130, 30);
            btnSwitchToRegister.TabIndex = 5;
            btnSwitchToRegister.Text = "没有账号？点击注册";
            btnSwitchToRegister.UseVisualStyleBackColor = true;
            btnSwitchToRegister.Click += btnSwitchToRegister_Click;
            // 
            // btnSwitchToForgotPassword
            // 
            btnSwitchToForgotPassword.Location = new Point(245, 291);
            btnSwitchToForgotPassword.Name = "btnSwitchToForgotPassword";
            btnSwitchToForgotPassword.Size = new Size(94, 30);
            btnSwitchToForgotPassword.TabIndex = 6;
            btnSwitchToForgotPassword.Text = "忘记密码？";
            btnSwitchToForgotPassword.UseVisualStyleBackColor = true;
            // 
            // btnConfigServer
            // 
            btnConfigServer.Location = new Point(244, 350);
            btnConfigServer.Name = "btnConfigServer";
            btnConfigServer.Size = new Size(100, 30);
            btnConfigServer.TabIndex = 7;
            btnConfigServer.Text = "配置服务器";
            btnConfigServer.UseVisualStyleBackColor = true;
            // 
            // lblServerUrlConfig
            // 
            lblServerUrlConfig.AutoSize = true;
            lblServerUrlConfig.Location = new Point(197, 400);
            lblServerUrlConfig.Name = "lblServerUrlConfig";
            lblServerUrlConfig.Size = new Size(79, 17);
            lblServerUrlConfig.TabIndex = 8;
            lblServerUrlConfig.Text = "服务器URL：";
            // 
            // txtServerUrlConfig
            // 
            txtServerUrlConfig.Location = new Point(290, 400);
            txtServerUrlConfig.Name = "txtServerUrlConfig";
            txtServerUrlConfig.Size = new Size(200, 23);
            txtServerUrlConfig.TabIndex = 9;
            txtServerUrlConfig.Text = "http://localhost:28001";
            // 
            // panelVerifyEmail
            // 
            panelVerifyEmail.Controls.Add(lblVerifyEmailTitle);
            panelVerifyEmail.Controls.Add(lblVerifyEmailDesc);
            panelVerifyEmail.Controls.Add(lblVerifyCode);
            panelVerifyEmail.Controls.Add(txtVerifyCode);
            panelVerifyEmail.Controls.Add(btnVerifyEmail);
            panelVerifyEmail.Controls.Add(btnResendVerifyCode);
            panelVerifyEmail.Controls.Add(btnSwitchToLoginFromVerify);
            panelVerifyEmail.Dock = DockStyle.Fill;
            panelVerifyEmail.Location = new Point(4, 4);
            panelVerifyEmail.Margin = new Padding(4);
            panelVerifyEmail.Name = "panelVerifyEmail";
            panelVerifyEmail.Size = new Size(933, 586);
            panelVerifyEmail.TabIndex = 2;
            panelVerifyEmail.Visible = false;
            // 
            // lblVerifyEmailTitle
            // 
            lblVerifyEmailTitle.AutoSize = true;
            lblVerifyEmailTitle.Font = new Font("宋体", 12F, FontStyle.Bold, GraphicsUnit.Point, 134);
            lblVerifyEmailTitle.Location = new Point(264, 20);
            lblVerifyEmailTitle.Name = "lblVerifyEmailTitle";
            lblVerifyEmailTitle.Size = new Size(75, 16);
            lblVerifyEmailTitle.TabIndex = 0;
            lblVerifyEmailTitle.Text = "邮箱验证";
            // 
            // lblVerifyEmailDesc
            // 
            lblVerifyEmailDesc.AutoSize = true;
            lblVerifyEmailDesc.Location = new Point(171, 50);
            lblVerifyEmailDesc.Name = "lblVerifyEmailDesc";
            lblVerifyEmailDesc.Size = new Size(260, 34);
            lblVerifyEmailDesc.TabIndex = 1;
            lblVerifyEmailDesc.Text = "我们已向您的邮箱发送了一封验证邮件，请输入\n验证码以完成注册。";
            // 
            // lblVerifyCode
            // 
            lblVerifyCode.AutoSize = true;
            lblVerifyCode.Location = new Point(273, 103);
            lblVerifyCode.Name = "lblVerifyCode";
            lblVerifyCode.Size = new Size(56, 17);
            lblVerifyCode.TabIndex = 2;
            lblVerifyCode.Text = "验证码：";
            // 
            // txtVerifyCode
            // 
            txtVerifyCode.Location = new Point(226, 142);
            txtVerifyCode.Name = "txtVerifyCode";
            txtVerifyCode.Size = new Size(150, 23);
            txtVerifyCode.TabIndex = 3;
            // 
            // btnVerifyEmail
            // 
            btnVerifyEmail.Location = new Point(244, 198);
            btnVerifyEmail.Name = "btnVerifyEmail";
            btnVerifyEmail.Size = new Size(100, 30);
            btnVerifyEmail.TabIndex = 4;
            btnVerifyEmail.Text = "验证";
            btnVerifyEmail.UseVisualStyleBackColor = true;
            // 
            // btnResendVerifyCode
            // 
            btnResendVerifyCode.Location = new Point(239, 250);
            btnResendVerifyCode.Name = "btnResendVerifyCode";
            btnResendVerifyCode.Size = new Size(110, 30);
            btnResendVerifyCode.TabIndex = 5;
            btnResendVerifyCode.Text = "重新发送";
            btnResendVerifyCode.UseVisualStyleBackColor = true;
            // 
            // btnSwitchToLoginFromVerify
            // 
            btnSwitchToLoginFromVerify.Location = new Point(184, 302);
            btnSwitchToLoginFromVerify.Name = "btnSwitchToLoginFromVerify";
            btnSwitchToLoginFromVerify.Size = new Size(220, 30);
            btnSwitchToLoginFromVerify.TabIndex = 6;
            btnSwitchToLoginFromVerify.Text = "返回登录";
            btnSwitchToLoginFromVerify.UseVisualStyleBackColor = true;
            // 
            // panelResetPassword
            // 
            panelResetPassword.Controls.Add(lblResetPasswordTitle);
            panelResetPassword.Controls.Add(lblResetPasswordDesc);
            panelResetPassword.Controls.Add(lblResetVerifyCode);
            panelResetPassword.Controls.Add(txtResetVerifyCode);
            panelResetPassword.Controls.Add(lblResetNewPassword);
            panelResetPassword.Controls.Add(txtResetNewPassword);
            panelResetPassword.Controls.Add(lblResetConfirmPassword);
            panelResetPassword.Controls.Add(txtResetConfirmPassword);
            panelResetPassword.Controls.Add(btnResetPassword);
            panelResetPassword.Controls.Add(btnSwitchToLoginFromReset);
            panelResetPassword.Dock = DockStyle.Fill;
            panelResetPassword.Location = new Point(4, 4);
            panelResetPassword.Margin = new Padding(4);
            panelResetPassword.Name = "panelResetPassword";
            panelResetPassword.Size = new Size(933, 586);
            panelResetPassword.TabIndex = 4;
            panelResetPassword.Visible = false;
            // 
            // lblResetPasswordTitle
            // 
            lblResetPasswordTitle.AutoSize = true;
            lblResetPasswordTitle.Font = new Font("宋体", 12F, FontStyle.Bold, GraphicsUnit.Point, 134);
            lblResetPasswordTitle.Location = new Point(264, 20);
            lblResetPasswordTitle.Name = "lblResetPasswordTitle";
            lblResetPasswordTitle.Size = new Size(75, 16);
            lblResetPasswordTitle.TabIndex = 0;
            lblResetPasswordTitle.Text = "重置密码";
            // 
            // lblResetPasswordDesc
            // 
            lblResetPasswordDesc.AutoSize = true;
            lblResetPasswordDesc.Location = new Point(189, 54);
            lblResetPasswordDesc.Name = "lblResetPasswordDesc";
            lblResetPasswordDesc.Size = new Size(224, 17);
            lblResetPasswordDesc.TabIndex = 1;
            lblResetPasswordDesc.Text = "请输入验证码和新密码以重置您的密码。";
            // 
            // lblResetVerifyCode
            // 
            lblResetVerifyCode.AutoSize = true;
            lblResetVerifyCode.Location = new Point(273, 89);
            lblResetVerifyCode.Name = "lblResetVerifyCode";
            lblResetVerifyCode.Size = new Size(56, 17);
            lblResetVerifyCode.TabIndex = 2;
            lblResetVerifyCode.Text = "验证码：";
            // 
            // txtResetVerifyCode
            // 
            txtResetVerifyCode.Location = new Point(226, 124);
            txtResetVerifyCode.Name = "txtResetVerifyCode";
            txtResetVerifyCode.Size = new Size(150, 23);
            txtResetVerifyCode.TabIndex = 3;
            // 
            // lblResetNewPassword
            // 
            lblResetNewPassword.AutoSize = true;
            lblResetNewPassword.Location = new Point(273, 165);
            lblResetNewPassword.Name = "lblResetNewPassword";
            lblResetNewPassword.Size = new Size(56, 17);
            lblResetNewPassword.TabIndex = 4;
            lblResetNewPassword.Text = "新密码：";
            // 
            // txtResetNewPassword
            // 
            txtResetNewPassword.Location = new Point(226, 200);
            txtResetNewPassword.Name = "txtResetNewPassword";
            txtResetNewPassword.Size = new Size(150, 23);
            txtResetNewPassword.TabIndex = 5;
            txtResetNewPassword.UseSystemPasswordChar = true;
            // 
            // lblResetConfirmPassword
            // 
            lblResetConfirmPassword.AutoSize = true;
            lblResetConfirmPassword.Location = new Point(267, 241);
            lblResetConfirmPassword.Name = "lblResetConfirmPassword";
            lblResetConfirmPassword.Size = new Size(68, 17);
            lblResetConfirmPassword.TabIndex = 6;
            lblResetConfirmPassword.Text = "确认密码：";
            // 
            // txtResetConfirmPassword
            // 
            txtResetConfirmPassword.Location = new Point(226, 276);
            txtResetConfirmPassword.Name = "txtResetConfirmPassword";
            txtResetConfirmPassword.Size = new Size(150, 23);
            txtResetConfirmPassword.TabIndex = 7;
            txtResetConfirmPassword.UseSystemPasswordChar = true;
            // 
            // btnResetPassword
            // 
            btnResetPassword.Location = new Point(251, 317);
            btnResetPassword.Name = "btnResetPassword";
            btnResetPassword.Size = new Size(100, 30);
            btnResetPassword.TabIndex = 8;
            btnResetPassword.Text = "重置密码";
            btnResetPassword.UseVisualStyleBackColor = true;
            // 
            // btnSwitchToLoginFromReset
            // 
            btnSwitchToLoginFromReset.Location = new Point(248, 365);
            btnSwitchToLoginFromReset.Name = "btnSwitchToLoginFromReset";
            btnSwitchToLoginFromReset.Size = new Size(106, 30);
            btnSwitchToLoginFromReset.TabIndex = 9;
            btnSwitchToLoginFromReset.Text = "返回登录";
            btnSwitchToLoginFromReset.UseVisualStyleBackColor = true;
            // 
            // tabPageSoftware
            // 
            tabPageSoftware.Controls.Add(labelSoftwareDetails);
            tabPageSoftware.Controls.Add(btnStartSoftware);
            tabPageSoftware.Controls.Add(listViewSoftware);
            tabPageSoftware.Controls.Add(flowLayoutPanelCards);
            tabPageSoftware.Controls.Add(btnCardView);
            tabPageSoftware.Controls.Add(btnListView);
            tabPageSoftware.Location = new Point(4, 26);
            tabPageSoftware.Margin = new Padding(4);
            tabPageSoftware.Name = "tabPageSoftware";
            tabPageSoftware.Padding = new Padding(4);
            tabPageSoftware.Size = new Size(941, 594);
            tabPageSoftware.TabIndex = 3;
            tabPageSoftware.Text = "软件管理";
            tabPageSoftware.UseVisualStyleBackColor = true;
            // 
            // labelSoftwareDetails
            // 
            labelSoftwareDetails.AutoSize = true;
            labelSoftwareDetails.Location = new Point(12, 428);
            labelSoftwareDetails.Margin = new Padding(4, 0, 4, 0);
            labelSoftwareDetails.Name = "labelSoftwareDetails";
            labelSoftwareDetails.Size = new Size(56, 17);
            labelSoftwareDetails.TabIndex = 4;
            labelSoftwareDetails.Text = "软件详情";
            // 
            // btnStartSoftware
            // 
            btnStartSoftware.Location = new Point(12, 391);
            btnStartSoftware.Margin = new Padding(4);
            btnStartSoftware.Name = "btnStartSoftware";
            btnStartSoftware.Size = new Size(88, 33);
            btnStartSoftware.TabIndex = 3;
            btnStartSoftware.Text = "启动";
            btnStartSoftware.UseVisualStyleBackColor = true;
            // 
            // listViewSoftware
            // 
            listViewSoftware.Columns.AddRange(new ColumnHeader[] { columnHeaderName, columnHeaderChineseName, columnHeaderVersion, columnHeaderStatus });
            listViewSoftware.FullRowSelect = true;
            listViewSoftware.GridLines = true;
            listViewSoftware.Location = new Point(12, 55);
            listViewSoftware.Margin = new Padding(4);
            listViewSoftware.Name = "listViewSoftware";
            listViewSoftware.Size = new Size(590, 316);
            listViewSoftware.TabIndex = 2;
            listViewSoftware.UseCompatibleStateImageBehavior = false;
            listViewSoftware.View = View.Details;
            // 
            // columnHeaderName
            // 
            columnHeaderName.Text = "软件名";
            columnHeaderName.Width = 150;
            // 
            // columnHeaderChineseName
            // 
            columnHeaderChineseName.Text = "中文名";
            columnHeaderChineseName.Width = 150;
            // 
            // columnHeaderVersion
            // 
            columnHeaderVersion.Text = "版本号";
            columnHeaderVersion.Width = 100;
            // 
            // columnHeaderStatus
            // 
            columnHeaderStatus.Text = "状态";
            columnHeaderStatus.Width = 100;
            // 
            // flowLayoutPanelCards
            // 
            flowLayoutPanelCards.AutoScroll = true;
            flowLayoutPanelCards.Location = new Point(12, 55);
            flowLayoutPanelCards.Name = "flowLayoutPanelCards";
            flowLayoutPanelCards.Size = new Size(580, 316);
            flowLayoutPanelCards.TabIndex = 5;
            flowLayoutPanelCards.Visible = false;
            // 
            // btnCardView
            // 
            btnCardView.Location = new Point(106, 14);
            btnCardView.Margin = new Padding(4);
            btnCardView.Name = "btnCardView";
            btnCardView.Size = new Size(88, 33);
            btnCardView.TabIndex = 1;
            btnCardView.Text = "卡片视图";
            btnCardView.UseVisualStyleBackColor = true;
            // 
            // btnListView
            // 
            btnListView.Location = new Point(12, 14);
            btnListView.Margin = new Padding(4);
            btnListView.Name = "btnListView";
            btnListView.Size = new Size(88, 33);
            btnListView.TabIndex = 0;
            btnListView.Text = "列表视图";
            btnListView.UseVisualStyleBackColor = true;
            // 
            // tabPageRecharge
            // 
            tabPageRecharge.Controls.Add(lvRechargeRecords);
            tabPageRecharge.Controls.Add(btnRefreshRechargeRecords);
            tabPageRecharge.Controls.Add(lblRechargeRecords);
            tabPageRecharge.Controls.Add(btnRecharge);
            tabPageRecharge.Controls.Add(txtCardCode);
            tabPageRecharge.Controls.Add(lblCardCode);
            tabPageRecharge.Controls.Add(lblRechargeResult);
            tabPageRecharge.Location = new Point(4, 26);
            tabPageRecharge.Margin = new Padding(4);
            tabPageRecharge.Name = "tabPageRecharge";
            tabPageRecharge.Padding = new Padding(4);
            tabPageRecharge.Size = new Size(941, 594);
            tabPageRecharge.TabIndex = 4;
            tabPageRecharge.Text = "充值";
            tabPageRecharge.UseVisualStyleBackColor = true;
            // 
            // lvRechargeRecords
            // 
            lvRechargeRecords.Columns.AddRange(new ColumnHeader[] { chCardCode, chAmount, chCreatedAt });
            lvRechargeRecords.FullRowSelect = true;
            lvRechargeRecords.GridLines = true;
            lvRechargeRecords.Location = new Point(20, 220);
            lvRechargeRecords.Name = "lvRechargeRecords";
            lvRechargeRecords.Size = new Size(560, 280);
            lvRechargeRecords.TabIndex = 7;
            lvRechargeRecords.UseCompatibleStateImageBehavior = false;
            lvRechargeRecords.View = View.Details;
            // 
            // chCardCode
            // 
            chCardCode.Text = "卡密";
            chCardCode.Width = 200;
            // 
            // chAmount
            // 
            chAmount.Text = "金额";
            chAmount.Width = 100;
            // 
            // chCreatedAt
            // 
            chCreatedAt.Text = "充值时间";
            chCreatedAt.Width = 250;
            // 
            // btnRefreshRechargeRecords
            // 
            btnRefreshRechargeRecords.Location = new Point(500, 180);
            btnRefreshRechargeRecords.Name = "btnRefreshRechargeRecords";
            btnRefreshRechargeRecords.Size = new Size(80, 30);
            btnRefreshRechargeRecords.TabIndex = 6;
            btnRefreshRechargeRecords.Text = "刷新";
            btnRefreshRechargeRecords.UseVisualStyleBackColor = true;
            // 
            // lblRechargeRecords
            // 
            lblRechargeRecords.AutoSize = true;
            lblRechargeRecords.Location = new Point(24, 184);
            lblRechargeRecords.Name = "lblRechargeRecords";
            lblRechargeRecords.Size = new Size(68, 17);
            lblRechargeRecords.TabIndex = 5;
            lblRechargeRecords.Text = "充值记录：";
            // 
            // btnRecharge
            // 
            btnRecharge.Location = new Point(250, 100);
            btnRecharge.Name = "btnRecharge";
            btnRecharge.Size = new Size(100, 30);
            btnRecharge.TabIndex = 3;
            btnRecharge.Text = "充值";
            btnRecharge.UseVisualStyleBackColor = true;
            // 
            // txtCardCode
            // 
            txtCardCode.Location = new Point(150, 60);
            txtCardCode.Name = "txtCardCode";
            txtCardCode.Size = new Size(300, 23);
            txtCardCode.TabIndex = 2;
            // 
            // lblCardCode
            // 
            lblCardCode.AutoSize = true;
            lblCardCode.Location = new Point(104, 64);
            lblCardCode.Name = "lblCardCode";
            lblCardCode.Size = new Size(44, 17);
            lblCardCode.TabIndex = 1;
            lblCardCode.Text = "卡密：";
            // 
            // lblRechargeResult
            // 
            lblRechargeResult.AutoSize = true;
            lblRechargeResult.Location = new Point(254, 144);
            lblRechargeResult.Name = "lblRechargeResult";
            lblRechargeResult.Size = new Size(0, 17);
            lblRechargeResult.TabIndex = 4;
            // 
            // tabPageBuyCards
            // 
            tabPageBuyCards.Controls.Add(webView2BuyCards);
            tabPageBuyCards.Location = new Point(4, 26);
            tabPageBuyCards.Margin = new Padding(4);
            tabPageBuyCards.Name = "tabPageBuyCards";
            tabPageBuyCards.Padding = new Padding(4);
            tabPageBuyCards.Size = new Size(941, 594);
            tabPageBuyCards.TabIndex = 6;
            tabPageBuyCards.Text = "购卡";
            tabPageBuyCards.UseVisualStyleBackColor = true;
            // 
            // webView2BuyCards
            // 
            webView2BuyCards.AllowExternalDrop = true;
            webView2BuyCards.CreationProperties = null;
            webView2BuyCards.DefaultBackgroundColor = Color.White;
            webView2BuyCards.Dock = DockStyle.Fill;
            webView2BuyCards.Location = new Point(4, 4);
            webView2BuyCards.Margin = new Padding(4);
            webView2BuyCards.Name = "webView2BuyCards";
            webView2BuyCards.Size = new Size(933, 586);
            webView2BuyCards.TabIndex = 0;
            webView2BuyCards.ZoomFactor = 1D;
            // 
            // tabPage1
            // 
            tabPage1.Controls.Add(txtApiLog);
            tabPage1.Location = new Point(4, 26);
            tabPage1.Name = "tabPage1";
            tabPage1.Padding = new Padding(3);
            tabPage1.Size = new Size(941, 594);
            tabPage1.TabIndex = 7;
            tabPage1.Text = "Log";
            tabPage1.UseVisualStyleBackColor = true;
            // 
            // txtApiLog
            // 
            txtApiLog.Dock = DockStyle.Fill;
            txtApiLog.Location = new Point(3, 3);
            txtApiLog.Multiline = true;
            txtApiLog.Name = "txtApiLog";
            txtApiLog.ReadOnly = true;
            txtApiLog.ScrollBars = ScrollBars.Both;
            txtApiLog.Size = new Size(935, 588);
            txtApiLog.TabIndex = 0;
            // 
            // labelServerUrl
            // 
            labelServerUrl.AutoSize = true;
            labelServerUrl.Location = new Point(20, 25);
            labelServerUrl.Name = "labelServerUrl";
            labelServerUrl.Size = new Size(53, 12);
            labelServerUrl.TabIndex = 0;
            labelServerUrl.Text = "服务器URL：";
            // 
            // labelHardwareId
            // 
            labelHardwareId.AutoSize = true;
            labelHardwareId.Location = new Point(20, 50);
            labelHardwareId.Name = "labelHardwareId";
            labelHardwareId.Size = new Size(53, 12);
            labelHardwareId.TabIndex = 1;
            labelHardwareId.Text = "硬件ID：";
            // 
            // labelSoftwareVersion
            // 
            labelSoftwareVersion.AutoSize = true;
            labelSoftwareVersion.Location = new Point(20, 75);
            labelSoftwareVersion.Name = "labelSoftwareVersion";
            labelSoftwareVersion.Size = new Size(53, 12);
            labelSoftwareVersion.TabIndex = 2;
            labelSoftwareVersion.Text = "软件版本：";
            // 
            // labelHeartbeatStatus
            // 
            labelHeartbeatStatus.AutoSize = true;
            labelHeartbeatStatus.Location = new Point(20, 100);
            labelHeartbeatStatus.Name = "labelHeartbeatStatus";
            labelHeartbeatStatus.Size = new Size(65, 12);
            labelHeartbeatStatus.TabIndex = 3;
            labelHeartbeatStatus.Text = "心跳状态：";
            // 
            // labelStatusCheckStatus
            // 
            labelStatusCheckStatus.AutoSize = true;
            labelStatusCheckStatus.Location = new Point(20, 125);
            labelStatusCheckStatus.Name = "labelStatusCheckStatus";
            labelStatusCheckStatus.Size = new Size(65, 12);
            labelStatusCheckStatus.TabIndex = 4;
            labelStatusCheckStatus.Text = "状态检测：";
            // 
            // labelCurrentUser
            // 
            labelCurrentUser.AutoSize = true;
            labelCurrentUser.Location = new Point(20, 150);
            labelCurrentUser.Name = "labelCurrentUser";
            labelCurrentUser.Size = new Size(53, 12);
            labelCurrentUser.TabIndex = 5;
            labelCurrentUser.Text = "当前用户：";
            // 
            // labelSoftwareCount
            // 
            labelSoftwareCount.AutoSize = true;
            labelSoftwareCount.Location = new Point(20, 175);
            labelSoftwareCount.Name = "labelSoftwareCount";
            labelSoftwareCount.Size = new Size(53, 12);
            labelSoftwareCount.TabIndex = 6;
            labelSoftwareCount.Text = "软件数量：";
            // 
            // labelProcessCount
            // 
            labelProcessCount.AutoSize = true;
            labelProcessCount.Location = new Point(20, 200);
            labelProcessCount.Name = "labelProcessCount";
            labelProcessCount.Size = new Size(53, 12);
            labelProcessCount.TabIndex = 7;
            labelProcessCount.Text = "进程数量：";
            // 
            // btnRefreshDebugInfo
            // 
            btnRefreshDebugInfo.Location = new Point(0, 0);
            btnRefreshDebugInfo.Name = "btnRefreshDebugInfo";
            btnRefreshDebugInfo.Size = new Size(75, 23);
            btnRefreshDebugInfo.TabIndex = 0;
            // 
            // Form1
            // 
            AutoScaleDimensions = new SizeF(7F, 17F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(949, 624);
            Controls.Add(tabControl1);
            Margin = new Padding(4);
            Name = "Form1";
            Text = "RLServer客户端";
            tabControl1.ResumeLayout(false);
            tabPageStatus.ResumeLayout(false);
            tabPageStatus.PerformLayout();
            tabPageLogin.ResumeLayout(false);
            panelUserInfo.ResumeLayout(false);
            panelUserInfo.PerformLayout();
            panelForgotPassword.ResumeLayout(false);
            panelForgotPassword.PerformLayout();
            panelRegister.ResumeLayout(false);
            panelRegister.PerformLayout();
            panelLogin.ResumeLayout(false);
            panelLogin.PerformLayout();
            panelVerifyEmail.ResumeLayout(false);
            panelVerifyEmail.PerformLayout();
            panelResetPassword.ResumeLayout(false);
            panelResetPassword.PerformLayout();
            tabPageSoftware.ResumeLayout(false);
            tabPageSoftware.PerformLayout();
            tabPageRecharge.ResumeLayout(false);
            tabPageRecharge.PerformLayout();
            tabPageBuyCards.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)webView2BuyCards).EndInit();
            tabPage1.ResumeLayout(false);
            tabPage1.PerformLayout();
dotnet build            ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.TabControl tabControl1;
        private System.Windows.Forms.TabPage tabPageStatus;
        private System.Windows.Forms.TabPage tabPageLogin;
        private System.Windows.Forms.TabPage tabPageSoftware;
        private System.Windows.Forms.TabPage tabPageRecharge;
        private System.Windows.Forms.TabPage tabPageBuyCards;
        private System.Windows.Forms.Label labelServerStatus;
        private System.Windows.Forms.Label labelResponseTime;
        private System.Windows.Forms.Label labelLastCheckTime;
        private System.Windows.Forms.Label labelPublicIp;
        private System.Windows.Forms.Button btnRefreshServerStatus;
        private System.Windows.Forms.GroupBox groupBoxDebug;
        private System.Windows.Forms.Label labelServerUrl;
        private System.Windows.Forms.Label labelHardwareId;
        private System.Windows.Forms.Label labelSoftwareVersion;
        private System.Windows.Forms.Label labelHeartbeatStatus;
        private System.Windows.Forms.Label labelStatusCheckStatus;
        private System.Windows.Forms.Label labelCurrentUser;
        private System.Windows.Forms.Label labelSoftwareCount;
        private System.Windows.Forms.Label labelProcessCount;
        private System.Windows.Forms.Button btnRefreshDebugInfo;
        private System.Windows.Forms.Button btnListView;
        // 登录/注册页面控件
        private System.Windows.Forms.Panel panelLogin;
        private System.Windows.Forms.Panel panelRegister;
        private System.Windows.Forms.Panel panelVerifyEmail;
        private System.Windows.Forms.Panel panelForgotPassword;
        private System.Windows.Forms.Panel panelResetPassword;
        // 登录面板控件
        private System.Windows.Forms.Label lblLoginUsername;
        private System.Windows.Forms.TextBox txtLoginUsername;
        private System.Windows.Forms.Label lblLoginPassword;
        private System.Windows.Forms.TextBox txtLoginPassword;
        private System.Windows.Forms.Button btnLogin;
        private System.Windows.Forms.Button btnSwitchToRegister;
        private System.Windows.Forms.Button btnSwitchToForgotPassword;
        private System.Windows.Forms.Button btnConfigServer;
        private System.Windows.Forms.Label lblServerUrlConfig;
        private System.Windows.Forms.TextBox txtServerUrlConfig;
        // 注册面板控件
        private System.Windows.Forms.Label lblRegisterUsername;
        private System.Windows.Forms.TextBox txtRegisterUsername;
        private System.Windows.Forms.Label lblRegisterEmail;
        private System.Windows.Forms.TextBox txtRegisterEmail;
        private System.Windows.Forms.Label lblRegisterPassword;
        private System.Windows.Forms.TextBox txtRegisterPassword;
        private System.Windows.Forms.Label lblRegisterConfirmPassword;
        private System.Windows.Forms.TextBox txtRegisterConfirmPassword;
        private System.Windows.Forms.Button btnRegister;
        private System.Windows.Forms.Button btnSwitchToLoginFromRegister;
        // 邮箱验证面板控件
        private System.Windows.Forms.Label lblVerifyEmailTitle;
        private System.Windows.Forms.Label lblVerifyEmailDesc;
        private System.Windows.Forms.Label lblVerifyCode;
        private System.Windows.Forms.TextBox txtVerifyCode;
        private System.Windows.Forms.Button btnVerifyEmail;
        private System.Windows.Forms.Button btnResendVerifyCode;
        private System.Windows.Forms.Button btnSwitchToLoginFromVerify;
        // 忘记密码面板控件
        private System.Windows.Forms.Label lblForgotPasswordTitle;
        private System.Windows.Forms.Label lblForgotPasswordDesc;
        private System.Windows.Forms.Label lblForgotEmail;
        private System.Windows.Forms.TextBox txtForgotEmail;
        private System.Windows.Forms.Button btnSendResetCode;
        private System.Windows.Forms.Button btnSwitchToLoginFromForgot;
        // 重置密码面板控件
        private System.Windows.Forms.Label lblResetPasswordTitle;
        private System.Windows.Forms.Label lblResetPasswordDesc;
        private System.Windows.Forms.Label lblResetVerifyCode;
        private System.Windows.Forms.TextBox txtResetVerifyCode;
        private System.Windows.Forms.Label lblResetNewPassword;
        private System.Windows.Forms.TextBox txtResetNewPassword;
        private System.Windows.Forms.Label lblResetConfirmPassword;
        private System.Windows.Forms.TextBox txtResetConfirmPassword;
        private System.Windows.Forms.Button btnResetPassword;
        private System.Windows.Forms.Button btnSwitchToLoginFromReset;
        private System.Windows.Forms.Button btnCardView;
        private System.Windows.Forms.ListView listViewSoftware;
        private System.Windows.Forms.ColumnHeader columnHeaderName;
        private System.Windows.Forms.ColumnHeader columnHeaderChineseName;
        private System.Windows.Forms.ColumnHeader columnHeaderVersion;
        private System.Windows.Forms.ColumnHeader columnHeaderStatus;
        private System.Windows.Forms.Button btnStartSoftware;
        private System.Windows.Forms.Label labelSoftwareDetails;
        private System.Windows.Forms.FlowLayoutPanel flowLayoutPanelCards;
        private Microsoft.Web.WebView2.WinForms.WebView2 webView2BuyCards;
        private System.Windows.Forms.Panel panelUserInfo;
        private System.Windows.Forms.Button btnChangePassword;
        private System.Windows.Forms.Button btnLogout;
        private System.Windows.Forms.Label labelUsername;
        private System.Windows.Forms.Label labelUsernameValue;
        private System.Windows.Forms.Label labelVipLevel;
        private System.Windows.Forms.Label labelVipLevelValue;
        private System.Windows.Forms.Label labelVipExpires;
        private System.Windows.Forms.Label labelVipExpiresValue;
        // 充值页面控件
        private System.Windows.Forms.Label lblCardCode;
        private System.Windows.Forms.TextBox txtCardCode;
        private System.Windows.Forms.Button btnRecharge;
        private System.Windows.Forms.Label lblRechargeResult;
        // 充值记录控件
        private System.Windows.Forms.Label lblRechargeRecords;
        private System.Windows.Forms.Button btnRefreshRechargeRecords;
        private System.Windows.Forms.ListView lvRechargeRecords;
        private System.Windows.Forms.ColumnHeader chCardCode;
        private System.Windows.Forms.ColumnHeader chAmount;
        private System.Windows.Forms.ColumnHeader chCreatedAt;
        private System.Windows.Forms.TextBox txtApiLog;
        private TabPage tabPage1;
    }
}