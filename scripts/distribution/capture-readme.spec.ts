import { test, expect } from '@playwright/test';
import { installModuleRegistryFixture } from '../../tests/fixtures/module-registry';
import { insightFacts } from '../../tests/fixtures/insights';
for (const language of ['zh-CN', 'en']) {
  test(`capture truthful GitHub product screenshots ${language}`, async ({ page }) => {
    const facts = insightFacts();
    facts.words = [{ word: language === 'en' ? 'Practice' : '练习', count: 4, records: [] }];
    facts.subjects = [{ label: language === 'en' ? 'Mathematics' : '数学', minutes: 30 }];
    facts.journeys = [
      { label: language === 'en' ? 'Foundations' : '基础巩固', total: 3, completed: 1 },
    ];
    facts.music = [];
    await page.addInitScript(
      ({ language, facts }) => {
        localStorage.setItem('lumen.locale.v1', language);
        localStorage.setItem(
          'lumen.appearance.v1',
          JSON.stringify({ mode: 'light', accent: 'sage', collapsed: false }),
        );
        const en = language === 'en',
          date = new Date().toLocaleDateString('sv-SE');
        const task = (id: string, title: string, minutes: number) => ({
          id,
          title,
          body: '',
          due_date: date,
          estimated_minutes: minutes,
          remaining_minutes: minutes,
          priority: 1,
          lifecycle: 'planned',
          revision: 0,
          deleted_at: null,
        });
        const today = {
          date,
          tasks: [
            task(
              '00000000-0000-4000-8000-000000000001',
              en ? 'Mathematics: limits and continuity' : '数学：极限与连续',
              45,
            ),
            task(
              '00000000-0000-4000-8000-000000000002',
              en ? 'Reading: annotate one article' : '阅读：精读一篇文章',
              30,
            ),
            task(
              '00000000-0000-4000-8000-000000000003',
              en ? 'Coding: review a graph problem' : '编程：复盘一道图论题',
              30,
            ),
          ],
          trash: [],
          session: null,
          capacity_minutes: 180,
          planned_minutes: 105,
          actual_seconds: 1800,
          journey: en ? 'A season of focused growth' : '一段专心成长的时间',
          notes: [
            {
              id: 'note',
              body: en ? 'One clear explanation is worth keeping.' : '留下一次真正想清楚的过程。',
            },
          ],
          timeline: [],
        };
        Object.assign(window, {
          isTauri: true,
          __TAURI_INTERNALS__: {
            invoke: async (command: string) => {
              if (command === 'today_snapshot') return today;
              if (command === 'desktop_status')
                return {
                  onboarding: false,
                  preferences: {
                    close_to_tray: false,
                    notifications_enabled: false,
                    focus_finished: false,
                    upcoming_event: false,
                    weekly_review: false,
                    backup_failure: false,
                    revision: 0,
                  },
                  autostart: false,
                  notification_permission: 'default',
                  runtime: { tray_ready: false, error_code: null },
                };
              if (command === 'deep_link_initial') return null;
              if (command === 'insights_memories') return [];
              if (command === 'insights_snapshot') return facts;
              if (command === 'vault_snapshot')
                return {
                  artifacts: [
                    {
                      id: '00000000-0000-4000-8000-000000000004',
                      title: en ? 'Personal research notebook' : '个人研究笔记项目',
                      description: en
                        ? 'A synthetic project for the product demonstration.'
                        : '用于产品演示的合成项目，不是真实用户数据。',
                      kind: 'project',
                      artifact_date: date,
                      role: en ? 'Creator' : '创作者',
                      organization: '',
                      skills: ['TypeScript', 'Research'],
                      technologies: ['React', 'SQLite'],
                      repository_url: '',
                      project_url: '',
                      paper_url: '',
                      journey_id: null,
                      competition_id: null,
                      visibility: 'private',
                      revision: 1,
                      archived_at: null,
                      deleted_at: null,
                    },
                  ],
                  journeys: [],
                  competitions: [],
                  limit: 1000,
                };
              throw Error('Unsupported synthetic demo command');
            },
          },
        });
      },
      { language, facts },
    );
    await installModuleRegistryFixture(page);
    for (const route of ['/', '/focus', '/vault', '/insights', '/settings']) {
      await page.goto('/#' + route);
      await expect(page.locator('main h1')).toBeVisible();
      if (route === '/')
        await expect(
          page.getByRole('heading', {
            name: language === 'en' ? 'Mathematics: limits and continuity' : '数学：极限与连续',
          }),
        ).toBeVisible();
      if (route === '/vault')
        await expect(
          page
            .getByText(language === 'en' ? 'Personal research notebook' : '个人研究笔记项目', {
              exact: true,
            })
            .first(),
        ).toBeVisible();
      if (language === 'en') {
        await expect(page.locator('main')).not.toContainText('分钟');
        if (route === '/insights')
          await expect(page.getByText('30.0 minutes', { exact: true }).first()).toBeVisible();
      }
      await page.evaluate(() => document.fonts.ready);
      if (route === '/settings') {
        await page
          .getByRole('heading', {
            name: language === 'en' ? 'Module management' : '模块管理',
            exact: true,
          })
          .scrollIntoViewIfNeeded();
        const card = page.locator('.ui-card').filter({
          has: page.getByRole('heading', {
            name: language === 'en' ? 'Module management' : '模块管理',
            exact: true,
          }),
        });
        await card.screenshot({
          path: `docs/distribution/assets/github/source/modules-${language}.png`,
        });
      } else {
        await page.screenshot({
          path: `docs/distribution/assets/github/source/${route.replaceAll('/', '') || 'today'}-${language}.png`,
        });
      }
    }
  });
}
