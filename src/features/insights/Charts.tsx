import type { Day, LabelValue } from './model';
import { minutes, ratio, statuses } from './model';
import { useState } from 'react';
export function Trend({
  days,
  field,
  title,
}: {
  days: Day[];
  field: 'deep_minutes' | 'coding_minutes' | 'planned_minutes' | 'capacity_minutes';
  title: string;
}) {
  const max = Math.max(1, ...days.map((d) => d[field]));
  const sample = days;
  const [tablePage, setTablePage] = useState(0);
  const page = Math.min(tablePage, Math.max(0, Math.ceil(days.length / 90) - 1));
  return (
    <figure className="insight-chart">
      <figcaption>{title}</figcaption>
      <svg
        viewBox="0 0 600 110"
        role="img"
        aria-label={`${title}，${days.length} 天。详细值见下方数据。`}
      >
        <polyline
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          points={sample
            .map(
              (d, i) =>
                `${(i / Math.max(1, sample.length - 1)) * 598},${105 - (d[field] / max) * 100}`,
            )
            .join(' ')}
        />
      </svg>
      <details>
        <summary>查看数据</summary>
        <div className="insight-table-scroll">
          <table>
            <thead>
              <tr>
                <th>日期</th>
                <th>{title}</th>
              </tr>
            </thead>
            <tbody>
              {days.slice(page * 90, page * 90 + 90).map((d) => (
                <tr key={d.date}>
                  <td>{d.date}</td>
                  <td>{minutes(d[field])}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <button type="button" disabled={page === 0} onClick={() => setTablePage(page - 1)}>
          上一组日期
        </button>{' '}
        {page + 1}/{Math.max(1, Math.ceil(days.length / 90))}{' '}
        <button
          type="button"
          disabled={(page + 1) * 90 >= days.length}
          onClick={() => setTablePage(page + 1)}
        >
          下一组日期
        </button>
      </details>
    </figure>
  );
}
export function Distribution({
  items,
  mode = 'minutes',
}: {
  items: LabelValue[];
  mode?: 'minutes' | 'progress' | 'count';
}) {
  return (
    <ul className="insight-distribution">
      {items.length === 0 ? (
        <li>暂无记录</li>
      ) : (
        items.map((i, n) => (
          <li key={`${i.label}-${n}`}>
            <span>{i.label}</span>
            <strong>
              {mode === 'minutes'
                ? minutes(i.minutes ?? 0)
                : mode === 'progress'
                  ? `${i.completed ?? 0}/${i.total ?? 0} · ${ratio(i.completed ?? 0, i.total ?? 0)}`
                  : (i.count ?? 0)}
            </strong>
          </li>
        ))
      )}
    </ul>
  );
}
export function Heatmap({ days, onDay }: { days: Day[]; onDay: (day: Day) => void }) {
  return (
    <div>
      <p className="secondary-text">
        颜色表示活动强度。特殊日保留记录，免除一致性分母，不影响计划容量。
      </p>
      <div className="insight-heatmap" aria-label="年度活动热图">
        {Array.from(
          { length: days[0] ? (new Date(days[0].date + 'T12:00:00').getDay() + 6) % 7 : 0 },
          (_, i) => (
            <span key={`pad-${i}`} aria-hidden="true" />
          ),
        )}
        {days.map((d) => (
          <button
            type="button"
            key={d.date}
            data-intensity={d.intensity}
            aria-label={`${d.date} ${statuses[d.status] ?? d.status} 活动强度 ${d.intensity}`}
            title={`${d.date} · 强度 ${d.intensity}`}
            onClick={() => onDay(d)}
            className={d.status === 'normal' ? '' : 'protected-day'}
          />
        ))}
      </div>
      <p>每列一周，周一至周日。强度 0–4；描边为特殊日。Tab 浏览，Enter 查看当日。</p>
    </div>
  );
}
