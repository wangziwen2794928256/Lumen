import { test, expect } from '@playwright/test';
import { installModuleRegistryFixture } from '../fixtures/module-registry';
import { createServer } from 'node:http';
import { readFile, mkdtemp, writeFile, readdir } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

test('Profile manual public review export and spacing', async ({ page }, info) => {
  await page.addInitScript(() => {
    Object.assign(window, { isTauri: true });
    let profile = {
      display_name: '',
      headline: '',
      about: '',
      links: [],
      public: false,
      revision: 0,
    };
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {
      invoke: async (command: string, args: { profile: typeof profile }) => {
        if (command === 'deep_link_initial') return '/profile';
        if (command === 'profile_get') return profile;
        if (command === 'profile_save') {
          profile = { ...args.profile, revision: profile.revision + 1 };
          return null;
        }
        if (command === 'public_projection')
          return {
            schema_version: '1.0',
            review_hash: 'fixture',
            exported_at: '2026-10-08T00:00:00Z',
            profile: profile.public ? { ...profile, revision: 0 } : null,
            artifacts: [],
          };
        if (command === 'public_export')
          return {
            id: 'fixture',
            directory: 'isolated-fixture/website',
            sha256: 'a'.repeat(64),
            artifacts: 0,
          };
        if (command === 'public_preview') return 'http://127.0.0.1:12345/';
        throw Error('Unexpected fixture command');
      },
    };
  });
  await installModuleRegistryFixture(page);
  await page.goto('/#/profile');
  await expect(page.getByLabel('展示名称')).toBeEnabled();
  await expect(page.getByLabel('公开个人介绍和联系链接')).not.toBeChecked();
  await expect(page.getByRole('button', { name: '导出本地网站', exact: true })).toBeDisabled();
  await page.getByLabel('展示名称').fill('隔离展示名称');
  await page.getByLabel('关于我', { exact: true }).fill('PRIVATE_BIO_SENTINEL');
  await expect(page.getByRole('button', { name: '检查公开资料' })).toBeDisabled();
  await page.getByRole('button', { name: '保存档案' }).click();
  await expect(page.getByText('档案已保存。')).toBeVisible();
  await page.getByRole('button', { name: '检查公开资料' }).click();
  await page.getByText('public-profile.json', { exact: true }).click();
  await expect(page.locator('.profile-projection pre')).not.toContainText('PRIVATE_BIO_SENTINEL');
  await page.getByRole('button', { name: '导出本地网站', exact: true }).click();
  await expect(page.getByText('本地网站已导出', { exact: true })).toBeVisible();
  await page.getByLabel('公开个人介绍和联系链接').check();
  await page.getByRole('button', { name: '保存档案' }).click();
  await expect(page.getByText('档案已保存。')).toBeVisible();
  await page.getByRole('button', { name: '检查公开资料' }).click();
  await expect(page.locator('.profile-projection pre')).toContainText('PRIVATE_BIO_SENTINEL');
  for (const theme of ['light', 'dark']) {
    await page.evaluate((t) => (document.documentElement.dataset.theme = t), theme);
    await expect(page.locator('.profile-page')).toBeVisible();
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(
      true,
    );
    const buttons = await page.locator('.profile-actions').first().locator('button').boundingBox();
    expect(buttons?.height).toBeGreaterThan(30);
    await page.screenshot({
      path: `tests/screenshots/phase12-profile-${theme}-${info.project.name}.png`,
      fullPage: true,
    });
  }
  await page.getByRole('link', { name: '组件展台', exact: true }).click();
  await expect(page).toHaveURL(/#\/design$/);
  await expect(page.locator('.profile-page')).toHaveCount(0);
  // Observe settled navigation too: startup links must not replay after shell transitions.
  await page.waitForTimeout(250);
  await expect(page).toHaveURL(/#\/design$/);
});

test('Detached website reads only public JSON, renders text safely and rejects private schema', async ({
  page,
}, info) => {
  // An isolated directory with four static files. There is no SQLite or IPC service.
  const directory = await mkdtemp(join(tmpdir(), 'lumen-public-'));
  const payload = {
    schema_version: '1.0',
    exported_at: '2026-10-08T00:00:00Z',
    profile: {
      display_name: '公开作品集',
      headline: 'Local first',
      about: '仅公开介绍',
      links: [{ label: 'Example', url: 'https://example.org' }],
      public: true,
      revision: 0,
    },
    artifacts: [
      {
        id: '00000000-0000-4000-8000-000000000001',
        title: '公开项目',
        description: '<img src=x onerror="window.__xss=true">',
        kind: 'project',
        date: null,
        role: '作者',
        organization: '',
        skills: ['Rust'],
        technologies: [],
        links: [],
        public: true,
      },
    ],
  };
  for (const name of ['index.html', 'site.js', 'site.css'])
    await writeFile(join(directory, name), await readFile(join('website', name)));
  await writeFile(join(directory, 'public-profile.json'), JSON.stringify(payload));
  expect(await readdir(directory)).not.toContain('lumen.db');
  const requests: string[] = [];
  let invalid = false;
  const server = createServer(async (req, res) => {
    requests.push(req.url ?? '');
    const name = req.url === '/' ? 'index.html' : req.url?.slice(1);
    if (!['index.html', 'site.js', 'site.css', 'public-profile.json'].includes(name ?? '')) {
      res.writeHead(404).end();
      return;
    }
    const body =
      name === 'public-profile.json' && invalid
        ? JSON.stringify({ ...payload, private_journal: 'PRIVATE' })
        : await readFile(join(directory, name!));
    res
      .writeHead(200, {
        'Content-Type':
          name === 'site.js'
            ? 'text/javascript'
            : name === 'site.css'
              ? 'text/css'
              : name === 'public-profile.json'
                ? 'application/json'
                : 'text/html',
      })
      .end(body);
  });
  await new Promise<void>((resolve) => server.listen(0, '127.0.0.1', resolve));
  const port = (server.address() as { port: number }).port;
  try {
    await page.goto(`http://127.0.0.1:${port}/`);
    await expect(page.locator('#status')).toContainText('公开作品 1');
    await expect(page.locator('#projects article')).toContainText('<img src=x');
    expect(await page.evaluate(() => Object.hasOwn(window, '__xss'))).toBe(false);
    await expect(page.locator('#projects img')).toHaveCount(0);
    await expect(page.locator('#projects a')).toHaveAttribute(
      'href',
      'lumen://artifact/00000000-0000-4000-8000-000000000001',
    );
    await expect(page.getByRole('navigation').locator('a')).toHaveCount(8);
    for (const scheme of ['light', 'dark'] as const) {
      await page.emulateMedia({ colorScheme: scheme, reducedMotion: 'reduce' });
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(
        true,
      );
      await page.screenshot({
        path: `tests/screenshots/phase12-website-${scheme}-${info.project.name}.png`,
        fullPage: true,
      });
    }
    await page.keyboard.press('Tab');
    await expect(page.locator('.skip')).toBeFocused();
    expect(
      requests
        .filter((p) => p !== '/favicon.ico')
        .every((p) => ['/', '/site.js', '/site.css', '/public-profile.json'].includes(p)),
    ).toBe(true);
    invalid = true;
    await page.reload();
    await expect(page.locator('#status')).toContainText('格式不受支持');
    await expect(page.locator('#projects article')).toHaveCount(0);
  } finally {
    await new Promise<void>((resolve) => server.close(() => resolve()));
  }
});
