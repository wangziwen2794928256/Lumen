import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  reporter: [
    ['list'],
    ['json', { outputFile: process.env.LUMEN_E2E_REPORT ?? 'docs/testing/e2e-results.json' }],
  ],
  use: {
    baseURL: 'http://127.0.0.1:1420',
    browserName: 'chromium',
    channel: process.env.LUMEN_E2E_BROWSER ?? 'msedge',
    trace: 'retain-on-failure',
  },
  webServer: { command: 'pnpm dev', url: 'http://127.0.0.1:1420', reuseExistingServer: false },
  projects: [
    { name: '1280x720', use: { viewport: { width: 1280, height: 720 } } },
    { name: '1440x900', use: { viewport: { width: 1440, height: 900 } } },
    { name: '1920x1080', use: { viewport: { width: 1920, height: 1080 } } },
  ],
});
