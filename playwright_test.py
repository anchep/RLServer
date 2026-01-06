import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        # 启动浏览器
        browser = await p.chromium.launch(headless=False)
        page = await browser.new_page()
        
        try:
            # 访问网站主页
            print("访问网站主页...")
            await page.goto("http://localhost:28001")
            await page.wait_for_load_state("networkidle")
            
            # 截图主页
            await page.screenshot(path="homepage.png")
            print("主页截图已保存为 homepage.png")
            
            # 检查页面标题
            title = await page.title()
            print(f"页面标题: {title}")
            
            # 检查页面内容
            content = await page.content()
            print(f"页面内容长度: {len(content)} 字符")
            
            # 检查是否有错误信息
            if "error" in content.lower() or "Error" in content:
                print("页面中包含错误信息")
            else:
                print("页面中未发现明显错误")
            
            # 尝试访问登录页面
            print("\n访问登录页面...")
            await page.goto("http://localhost:28001/admin/login")
            await page.wait_for_load_state("networkidle")
            
            # 截图登录页面
            await page.screenshot(path="login_page.png")
            print("登录页面截图已保存为 login_page.png")
            
            # 检查登录页面元素
            if await page.locator("input[name='username']").is_visible() and await page.locator("input[name='password']").is_visible():
                print("登录页面包含用户名和密码输入框")
            else:
                print("登录页面缺少用户名或密码输入框")
            
            # 检查管理控制台
            print("\n检查管理控制台...")
            await page.goto("http://localhost:28001/admin/dashboard")
            await page.wait_for_load_state("networkidle")
            
            # 截图管理控制台
            await page.screenshot(path="dashboard.png")
            print("管理控制台截图已保存为 dashboard.png")
            
        except Exception as e:
            print(f"测试过程中发生错误: {e}")
        finally:
            # 关闭浏览器
            await browser.close()

if __name__ == "__main__":
    asyncio.run(main())
