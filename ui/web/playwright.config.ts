import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests-e2e',

  use: {
    baseURL: process.env.UI_BASE_URL || 'http://localhost:5173',
    headless: true,
  },
  webServer: {
    command: 'npm run dev',
    url: process.env.UI_BASE_URL || 'http://localhost:5173',
    reuseExistingServer: true,
    cwd: './',
    stdout: 'pipe',
    stderr: 'pipe',
  },
  timeout: 60_000,
  retries: 0,
});
