import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: '.',
  testMatch: 'capture-readme.spec.ts',
  fullyParallel: false,
  workers: 1,
  reporter: [['list'], ['json', { outputFile: 'work/phase23/capture-results.json' }]],
  use: {
    baseURL: 'http://127.0.0.1:1420',
    browserName: 'chromium',
    channel: 'msedge',
    viewport: { width: 1440, height: 960 },
    deviceScaleFactor: 1,
    trace: 'retain-on-failure',
  },
  webServer: { command: 'pnpm dev', url: 'http://127.0.0.1:1420', reuseExistingServer: false },
});
