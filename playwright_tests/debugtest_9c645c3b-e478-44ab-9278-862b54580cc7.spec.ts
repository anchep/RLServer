
import { test } from '@playwright/test';
import { expect } from '@playwright/test';

test('DebugTest_2025-12-28', async ({ page, context }) => {
  
    // Navigate to URL
    await page.goto('https://localhost:28043/admin/login');

    // Navigate to URL
    await page.goto('https://localhost:28043/admin/login');
});