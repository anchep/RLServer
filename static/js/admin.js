// 后台管理系统JavaScript

// DOM加载完成后执行
document.addEventListener('DOMContentLoaded', function() {
    // 初始化侧边栏下拉菜单
    initSidebar();
    
    // 初始化表格操作
    initTableOperations();
    
    // 初始化表单验证
    initFormValidation();
    
    // 初始化统计图表（如果有）
    initCharts();
    
    // 初始化模态框
    initModals();
});

// 初始化侧边栏
function initSidebar() {
    // 获取当前URL
    const currentUrl = window.location.pathname;
    
    // 自动展开当前活动菜单项
    const navLinks = document.querySelectorAll('.nav-link');
    let activeLink = null;
    
    // 为当前URL匹配的链接添加active类
    navLinks.forEach(link => {
        const linkUrl = link.getAttribute('href');
        if (linkUrl && currentUrl === linkUrl) {
            link.classList.add('active');
            activeLink = link;
        }
    });
    
    // 展开包含活动链接的父菜单
    if (activeLink) {
        const parentDropdown = activeLink.closest('.collapse');
        if (parentDropdown) {
            parentDropdown.classList.add('show');
            const parentToggle = parentDropdown.previousElementSibling;
            if (parentToggle) {
                parentToggle.classList.add('active');
            }
        } else {
            // 检查是否是子菜单链接
            const parentLi = activeLink.closest('li');
            if (parentLi && parentLi.closest('.collapse')) {
                const collapseElement = parentLi.closest('.collapse');
                collapseElement.classList.add('show');
                const toggleLink = collapseElement.previousElementSibling;
                if (toggleLink) {
                    toggleLink.classList.add('active');
                }
            }
        }
    }
    
    // 移动端菜单切换
    const menuToggle = document.getElementById('menu-toggle');
    if (menuToggle) {
        menuToggle.addEventListener('click', function() {
            document.querySelector('.sidebar').classList.toggle('open');
        });
    }
    
    // 阻止菜单链接点击时自动关闭
    const dropdownToggles = document.querySelectorAll('.nav-link.dropdown-toggle');
    dropdownToggles.forEach(toggle => {
        toggle.addEventListener('click', function(e) {
            // 阻止默认行为，防止链接跳转
            e.preventDefault();
            
            // 切换当前菜单的展开/折叠状态
            const targetId = this.getAttribute('href');
            const targetCollapse = document.querySelector(targetId);
            if (targetCollapse) {
                targetCollapse.classList.toggle('show');
                this.classList.toggle('active');
            }
        });
    });
}

// 初始化表格操作
function initTableOperations() {
    // 为所有删除按钮添加确认事件
    const deleteButtons = document.querySelectorAll('.btn-delete');
    deleteButtons.forEach(button => {
        button.addEventListener('click', function(e) {
            e.preventDefault();
            const confirmDelete = confirm('确定要删除吗？此操作不可恢复。');
            if (confirmDelete) {
                window.location.href = this.getAttribute('href');
            }
        });
    });
    
    // 为所有状态切换按钮添加确认事件
    const statusButtons = document.querySelectorAll('.btn-toggle-status');
    statusButtons.forEach(button => {
        button.addEventListener('click', function(e) {
            e.preventDefault();
            const currentStatus = this.getAttribute('data-current-status') === 'true';
            const newStatus = !currentStatus;
            const statusText = newStatus ? '启用' : '禁用';
            const confirmToggle = confirm(`确定要${statusText}吗？`);
            if (confirmToggle) {
                window.location.href = this.getAttribute('href');
            }
        });
    });
    
    // 表格行悬停效果
    const tableRows = document.querySelectorAll('.table tbody tr');
    tableRows.forEach(row => {
        row.addEventListener('mouseenter', function() {
            this.style.backgroundColor = '#f8f9fa';
        });
        
        row.addEventListener('mouseleave', function() {
            this.style.backgroundColor = '';
        });
    });
}

// 初始化表单验证
function initFormValidation() {
    // 为所有表单添加提交事件监听
    const forms = document.querySelectorAll('form');
    forms.forEach(form => {
        form.addEventListener('submit', function(e) {
            // 检查必填字段
            const requiredFields = this.querySelectorAll('[required]');
            let isValid = true;
            
            requiredFields.forEach(field => {
                if (!field.value.trim()) {
                    isValid = false;
                    field.classList.add('is-invalid');
                    field.scrollIntoView({ behavior: 'smooth', block: 'center' });
                } else {
                    field.classList.remove('is-invalid');
                }
            });
            
            // 检查密码一致性
            const passwordField = this.querySelector('[name="password"]');
            const confirmPasswordField = this.querySelector('[name="confirm_password"]');
            
            if (passwordField && confirmPasswordField) {
                if (passwordField.value !== confirmPasswordField.value) {
                    isValid = false;
                    confirmPasswordField.classList.add('is-invalid');
                    confirmPasswordField.setCustomValidity('两次输入的密码不一致');
                    confirmPasswordField.scrollIntoView({ behavior: 'smooth', block: 'center' });
                } else {
                    confirmPasswordField.classList.remove('is-invalid');
                    confirmPasswordField.setCustomValidity('');
                }
            }
            
            if (!isValid) {
                e.preventDefault();
            }
        });
    });
    
    // 实时验证密码强度
    const passwordInputs = document.querySelectorAll('[name="password"]');
    passwordInputs.forEach(input => {
        input.addEventListener('input', function() {
            const password = this.value;
            const strengthIndicator = this.nextElementSibling;
            
            if (strengthIndicator && strengthIndicator.classList.contains('password-strength')) {
                const strength = checkPasswordStrength(password);
                updatePasswordStrengthIndicator(strengthIndicator, strength);
            }
        });
    });
}

// 检查密码强度
function checkPasswordStrength(password) {
    if (password.length < 6) return 0;
    if (password.length < 8) return 1;
    if (/[A-Z]/.test(password) && /[a-z]/.test(password) && /[0-9]/.test(password)) return 2;
    if (/[A-Z]/.test(password) && /[a-z]/.test(password) && /[0-9]/.test(password) && /[^A-Za-z0-9]/.test(password)) return 3;
    return 1;
}

// 更新密码强度指示器
function updatePasswordStrengthIndicator(indicator, strength) {
    const levels = ['弱', '中', '强', '很强'];
    const colors = ['danger', 'warning', 'success', 'primary'];
    
    indicator.textContent = `密码强度: ${levels[strength]}`;
    indicator.className = `password-strength text-${colors[strength]}`;
}

// 初始化统计图表
function initCharts() {
    // 如果页面包含图表容器，则初始化图表
    const chartContainers = document.querySelectorAll('.chart-container');
    chartContainers.forEach(container => {
        const chartType = container.dataset.chartType || 'bar';
        const chartData = container.dataset.chartData;
        
        if (chartData) {
            try {
                const data = JSON.parse(chartData);
                // 这里可以使用Chart.js或其他图表库初始化图表
                console.log('初始化图表:', chartType, data);
            } catch (error) {
                console.error('图表数据解析错误:', error);
            }
        }
    });
}

// 初始化模态框
function initModals() {
    // 为所有模态框添加关闭事件
    const modals = document.querySelectorAll('.modal');
    modals.forEach(modal => {
        const closeButtons = modal.querySelectorAll('[data-bs-dismiss="modal"]');
        closeButtons.forEach(button => {
            button.addEventListener('click', function() {
                modal.classList.remove('show');
                document.body.classList.remove('modal-open');
                const backdrop = document.querySelector('.modal-backdrop');
                if (backdrop) {
                    backdrop.remove();
                }
            });
        });
    });
}

// 显示消息提示
function showMessage(message, type = 'info', duration = 3000) {
    // 创建消息元素
    const messageElement = document.createElement('div');
    messageElement.className = `alert alert-${type} alert-dismissible fade show`;
    messageElement.role = 'alert';
    messageElement.innerHTML = `
        ${message}
        <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>
    `;
    
    // 添加到页面
    const content = document.querySelector('.content');
    if (content) {
        content.insertBefore(messageElement, content.firstChild);
        
        // 自动关闭
        setTimeout(() => {
            messageElement.classList.remove('show');
            setTimeout(() => {
                messageElement.remove();
            }, 150);
        }, duration);
    }
}

// 格式化日期
function formatDate(timestamp, format = 'YYYY-MM-DD HH:mm:ss') {
    const date = new Date(timestamp * 1000);
    
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');
    
    return format
        .replace('YYYY', year)
        .replace('MM', month)
        .replace('DD', day)
        .replace('HH', hours)
        .replace('mm', minutes)
        .replace('ss', seconds);
}

// 格式化金额
function formatAmount(amount, currency = '¥') {
    return `${currency}${parseFloat(amount).toFixed(2)}`;
}

// 复制文本到剪贴板
async function copyToClipboard(text) {
    try {
        await navigator.clipboard.writeText(text);
        showMessage('已复制到剪贴板', 'success');
        return true;
    } catch (error) {
        console.error('复制失败:', error);
        showMessage('复制失败，请手动复制', 'danger');
        return false;
    }
}

// 刷新页面数据
function refreshPage() {
    window.location.reload();
}

// 跳转到指定页面
function goToPage(url) {
    window.location.href = url;
}

// 批量操作处理
function handleBulkOperation(operation) {
    const selectedIds = [];
    const checkboxes = document.querySelectorAll('.bulk-checkbox:checked');
    
    checkboxes.forEach(checkbox => {
        selectedIds.push(checkbox.value);
    });
    
    if (selectedIds.length === 0) {
        showMessage('请选择要操作的项目', 'warning');
        return;
    }
    
    const confirmMessage = {
        delete: '确定要删除选中的项目吗？',
        enable: '确定要启用选中的项目吗？',
        disable: '确定要禁用选中的项目吗？'
    };
    
    if (confirm(confirmMessage[operation] || '确定要执行此操作吗？')) {
        // 这里可以通过AJAX提交批量操作
        console.log(`执行批量${operation}操作，选中ID:`, selectedIds);
        showMessage(`批量${operation}操作已执行`, 'success');
    }
}

// 导出数据
function exportData(type = 'csv') {
    // 这里可以实现数据导出功能
    console.log(`导出数据为${type}格式`);
    showMessage('数据导出功能开发中', 'info');
}

// 搜索功能
function searchData() {
    const searchInput = document.getElementById('search-input');
    const searchTerm = searchInput ? searchInput.value.trim() : '';
    
    if (searchTerm) {
        console.log('搜索:', searchTerm);
        // 这里可以实现搜索功能
    } else {
        showMessage('请输入搜索关键词', 'warning');
    }
}

// 高级筛选
function applyFilters() {
    // 获取所有筛选条件
    const filters = {};
    const filterInputs = document.querySelectorAll('.filter-input');
    
    filterInputs.forEach(input => {
        if (input.value.trim()) {
            filters[input.name] = input.value.trim();
        }
    });
    
    console.log('应用筛选条件:', filters);
    // 这里可以实现筛选功能
    showMessage('筛选条件已应用', 'success');
}

// 重置筛选条件
function resetFilters() {
    const filterInputs = document.querySelectorAll('.filter-input');
    filterInputs.forEach(input => {
        input.value = '';
    });
    
    console.log('重置筛选条件');
    // 这里可以实现重置功能
    showMessage('筛选条件已重置', 'info');
}

// 日期范围选择器初始化
function initDateRangePicker() {
    const dateRangeInputs = document.querySelectorAll('.date-range-picker');
    dateRangeInputs.forEach(input => {
        // 这里可以初始化日期范围选择器库
        console.log('初始化日期范围选择器');
    });
}

// 数字输入框限制
function restrictNumberInput(input) {
    input.addEventListener('input', function() {
        this.value = this.value.replace(/[^0-9.]/g, '');
        // 限制只能有一个小数点
        const parts = this.value.split('.');
        if (parts.length > 2) {
            this.value = parts[0] + '.' + parts.slice(1).join('');
        }
    });
}

// 初始化数字输入框限制
const numberInputs = document.querySelectorAll('input[type="number"], .number-input');
numberInputs.forEach(input => {
    restrictNumberInput(input);
});