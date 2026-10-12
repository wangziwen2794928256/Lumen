// Editable, deterministic Canvas composition. Only synthetic UI captures are used.
import { chromium } from '@playwright/test';
import { readFile, writeFile } from 'node:fs/promises';
const root = new URL('../../docs/distribution/assets/github/', import.meta.url);
const browser = await chromium.launch({ channel: 'msedge' });
const page = await browser.newPage();
for (const lang of ['zh-CN', 'en']) {
  const en = lang === 'en';
  const sources = {};
  for (const name of ['today', 'focus', 'vault', 'modules'])
    sources[name] =
      'data:image/png;base64,' +
      (await readFile(new URL(`source/${name}-${lang}.png`, root))).toString('base64');
  for (const kind of ['hero', 'workflow', 'modules']) {
    const png = await page.evaluate(
      async ({ sources, en, kind }) => {
        await document.fonts.ready;
        const c = document.createElement('canvas');
        c.width = 1920;
        c.height = kind === 'hero' ? 1540 : 1120;
        const g = c.getContext('2d');
        const dark = kind === 'modules';
        const ink = dark ? '#f5f3eb' : '#253f36',
          muted = dark ? '#c0cec2' : '#607267';
        g.fillStyle = dark ? '#203e34' : '#f2f2e9';
        g.fillRect(0, 0, c.width, c.height);
        const grad = g.createRadialGradient(1700, 80, 10, 1700, 80, 1200);
        grad.addColorStop(0, dark ? '#41634a' : '#dbe6d4');
        grad.addColorStop(1, dark ? '#203e34' : '#f2f2e9');
        g.fillStyle = grad;
        g.fillRect(0, 0, c.width, c.height);
        const text = (s, x, y, size = 32, color = ink, bold = false) => {
          g.font = `${bold ? 600 : 400} ${size}px "Segoe UI", "Microsoft YaHei", sans-serif`;
          g.fillStyle = color;
          g.fillText(s, x, y);
          if (g.measureText(s).width > 1920 - x - 50) throw Error('Text overflow: ' + s);
        };
        const box = (x, y, w, h, fill, r = 22) => {
          g.beginPath();
          g.roundRect(x, y, w, h, r);
          g.fillStyle = fill;
          g.fill();
        };
        const imgs = {};
        for (const [k, v] of Object.entries(sources)) {
          const i = new Image();
          i.src = v;
          await i.decode();
          imgs[k] = i;
        }
        const shot = (name, x, y, w, h, sx, sy, sw, sh) => {
          g.save();
          g.shadowColor = '#152b3024';
          g.shadowBlur = 30;
          g.shadowOffsetY = 12;
          box(x, y, w, h, '#fff', 18);
          g.restore();
          g.save();
          g.beginPath();
          g.roundRect(x, y, w, h, 18);
          g.clip();
          g.drawImage(imgs[name], sx, sy, sw, sh, x, y, w, h);
          g.restore();
        };
        text('◌  Lumen', 72, 100, 52, ink, true);
        text('LOCAL-FIRST  /  OPEN SOURCE  /  MIT', 1160, 96, 25, muted);
        if (kind === 'hero') {
          text(
            en ? 'A workspace for your growth.' : '把成长，记在自己的电脑里。',
            72,
            222,
            en ? 94 : 100,
            ink,
            true,
          );
          text(
            en
              ? 'Lightweight. Modular. Open source. Yours to shape.'
              : '一个轻量的本地成长工作台。任务、专注、日记、作品，按需启用。',
            76,
            294,
            36,
            muted,
          );
          const labels = en
            ? ['LIGHTWEIGHT', 'MODULAR', '100% OPEN SOURCE', 'LOCAL DATA']
            : ['轻量使用', '模块扩展', '完全开源 · MIT', '本地保存'];
          let x = 76;
          for (const label of labels) {
            g.font = '600 27px "Segoe UI", "Microsoft YaHei"';
            const w = g.measureText(label).width + 44;
            box(x, 329, w, 54, '#dce5d7', 27);
            text(label, x + 22, 365, 27, '#385241', true);
            x += w + 18;
          }
          shot('today', 72, 425, 1776, 992, 0, 0, 1440, 804);
          text(
            en
              ? 'Actual application UI · Synthetic demo records · Chinese / English'
              : '当前源码真实界面 · 合成示例数据 · 中文 / English',
            76,
            1488,
            27,
            muted,
          );
        } else if (kind === 'workflow') {
          text(
            en ? 'Start small. Keep what you learn.' : '打开之后，先做这三件事。',
            72,
            220,
            84,
            ink,
            true,
          );
          text(
            en
              ? 'Plan today, focus on one task, then keep your work.'
              : '安排今天，专注一件事，再把自己的成果留下来。',
            76,
            286,
            35,
            muted,
          );
          const cards = en
            ? [
                [
                  '01',
                  'Plan today',
                  'Pick a task. Set a realistic estimate.',
                  'today',
                  250,
                  232,
                  870,
                  494,
                ],
                [
                  '02',
                  'Make time',
                  'Choose your task and start focusing.',
                  'focus',
                  256,
                  237,
                  860,
                  489,
                ],
                [
                  '03',
                  'Keep your work',
                  'Save projects, skills and milestones.',
                  'vault',
                  256,
                  375,
                  440,
                  250,
                ],
              ]
            : [
                ['01', '安排今天', '选一件事，给它一个合适的时间。', 'today', 250, 232, 870, 494],
                [
                  '02',
                  '开始专注',
                  '选好任务，按下开始，记录实际投入。',
                  'focus',
                  256,
                  237,
                  860,
                  489,
                ],
                ['03', '留下作品', '记录项目、技能和阶段成果。', 'vault', 256, 375, 440, 250],
              ];
          cards.forEach((a, i) => {
            const x = 72 + i * 600;
            box(x, 346, 576, 590, '#fcfcf8');
            text(a[0], x + 28, 411, 35, '#789178', true);
            text(a[1], x + 28, 479, 44, ink, true);
            text(a[2], x + 28, 530, en ? 25 : 27, muted);
            shot(a[3], x + 24, 575, 528, 300, ...a.slice(4));
          });
          text(
            en
              ? 'Study for an exam. Learn a language. Build a project. Make it yours.'
              : '考研 · 考公 · 学语言 · 科研 · 做项目 —— 记录每一段认真成长的时间。',
            76,
            1010,
            34,
            ink,
          );
          text(
            en
              ? 'Cropped actual UI · Synthetic examples · Guide links below'
              : '真实页面局部 · 合成示例 · 图片下方提供可点击的使用指引',
            76,
            1070,
            25,
            muted,
          );
        } else {
          text(
            en ? 'Keep what fits. Build what is missing.' : '需要的留下，缺少的自己写。',
            72,
            215,
            en ? 76 : 92,
            ink,
            true,
          );
          text(
            en
              ? 'Enable, hide, pin and reorder modules in Settings.'
              : '在设置里开关、隐藏、置顶、拖动排序，整理自己的侧栏。',
            76,
            282,
            34,
            muted,
          );
          shot('modules', 72, 340, 1070, 638, 0, 0, 1144, 682);
          const items = en
            ? [
                [
                  '01  YOUR SIDEBAR',
                  'Enable only the modules you need.',
                  'Disabling keeps your records.',
                ],
                [
                  '02  YOUR COMPONENTS',
                  'Manifest → Registry → Route / Widget',
                  'Start from the module tutorial.',
                ],
                [
                  '03  YOUR SOURCE CODE',
                  'MIT source + bilingual documentation',
                  'Editable slides and reproducible assets.',
                ],
              ]
            : [
                ['01  按需启用', '把常用模块留在手边。', '停用模块不会删除已有记录。'],
                [
                  '02  自己写组件',
                  'Manifest → Registry → Route / Widget',
                  '从官方模块教程开始，复用现有能力。',
                ],
                [
                  '03  源码与资料都给你',
                  'MIT 源码 · 中英技术文档',
                  '可编辑 PPT · 可复现的图片源文件',
                ],
              ];
          items.forEach((a, i) => {
            const y = 390 + i * 205;
            text(a[0], 1190, y, 35, ink, true);
            text(a[1], 1190, y + 57, en ? 27 : 30, muted);
            text(a[2], 1190, y + 102, en ? 26 : 29, muted);
          });
          text(
            en
              ? 'Available now: trusted source modules. Isolated community runtime is future work.'
              : '当前支持可信源码模块接入；隔离社区插件运行时属于后续规划。',
            76,
            1055,
            29,
            muted,
          );
        }
        return c.toDataURL('image/png').split(',')[1];
      },
      { sources, en, kind },
    );
    await writeFile(new URL(`${kind}-${lang}.png`, root), Buffer.from(png, 'base64'));
    console.log(`${kind}-${lang}.png`);
  }
}
await browser.close();
