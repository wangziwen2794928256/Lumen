import { locale, translate } from '../../lib/locale';
import type { Monthly } from './model';
function download(blob: Blob, name: string) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = name;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
export function exportJson(month: Monthly) {
  const fields = [
    'version',
    'visibility',
    'month',
    'from',
    'until',
    'closed',
    'generated_at',
    'totals',
    'days',
    'subjects',
    'topics',
    'music',
    'words',
    'counts',
    'task_summary',
  ] as const;
  const safe = Object.fromEntries(fields.map((key) => [key, month[key]]));
  download(
    new Blob([JSON.stringify(safe, null, 2)], { type: 'application/json' }),
    `Lumen-${month.month}-private.json`,
  );
}
export function exportPng(month: Monthly) {
  const canvas = document.createElement('canvas');
  canvas.width = 1200;
  canvas.height = 800;
  const ctx = canvas.getContext('2d');
  if (!ctx) throw Error('Canvas unavailable');
  ctx.fillStyle = '#f6f4ee';
  ctx.fillRect(0, 0, 1200, 800);
  ctx.fillStyle = '#203c31';
  ctx.font = '32px sans-serif';
  ctx.fillText(`Lumen · ${month.month} Snapshot`, 60, 70);
  ctx.font = '20px sans-serif';
  const rows =
    locale === 'en'
      ? [
          `Private snapshot · ${month.from} — ${month.until}`,
          `Actual focus ${month.totals.deep_minutes.toFixed(1)} minutes`,
          `Coding ${month.totals.coding_minutes.toFixed(1)} minutes (may overlap focus)`,
          `Planned ${month.totals.planned_minutes.toFixed(1)} minutes`,
          `Tasks ${month.totals.tasks_completed} · Journals ${month.totals.journals}`,
          `Topics ${
            month.topics
              .slice(0, 6)
              .map((i) => i.label)
              .join(' · ') || translate('暂无记录')
          }`,
          `Words ${
            month.words
              .slice(0, 6)
              .map((i) => i.word)
              .join(' · ') || translate('暂无记录')
          }`,
        ]
      : [
          `私密快照 · ${month.from} — ${month.until}`,
          `实际专注 ${month.totals.deep_minutes.toFixed(1)} 分钟`,
          `编码 ${month.totals.coding_minutes.toFixed(1)} 分钟（可能与专注重叠）`,
          `计划 ${month.totals.planned_minutes.toFixed(1)} 分钟`,
          `任务 ${month.totals.tasks_completed} · 日记 ${month.totals.journals}`,
          `主题 ${
            month.topics
              .slice(0, 6)
              .map((i) => i.label)
              .join(' · ') || translate('暂无记录')
          }`,
          `词语 ${
            month.words
              .slice(0, 6)
              .map((i) => i.word)
              .join(' · ') || translate('暂无记录')
          }`,
        ];
  rows.forEach((r, i) => ctx.fillText(r, 60, 135 + i * 45, 1080));
  const max = Math.max(1, ...month.days.map((d) => d.deep_minutes));
  ctx.strokeStyle = '#477058';
  ctx.lineWidth = 3;
  ctx.beginPath();
  month.days.forEach((d, i) => {
    const x = 60 + (i / Math.max(1, month.days.length - 1)) * 1080,
      y = 740 - (d.deep_minutes / max) * 220;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  });
  ctx.stroke();
  canvas.toBlob((blob) => {
    if (blob) download(blob, `Lumen-${month.month}-private.png`);
  }, 'image/png');
}
