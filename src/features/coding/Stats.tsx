import type { Analytics } from './model';
import { firstPass, minutes } from './model';
import { codingText as t } from './text';
export function Stats({ value: a }: { value: Analytics }) {
  return (
    <section className="ui-card coding-stats" aria-label={t.analytics}>
      <h2>
        {t.analytics} · {a.from} — {a.until}
      </h2>
      <dl className="coding-metrics">
        <div>
          <dt>练习题数</dt>
          <dd>{a.problems}</dd>
        </div>
        <div>
          <dt>首次解出题数</dt>
          <dd>{a.solved}</dd>
        </div>
        <div>
          <dt>尝试次数</dt>
          <dd>{a.attempts}</dd>
        </div>
        <div>
          <dt>首过率</dt>
          <dd>{firstPass(a)}</dd>
        </div>
        <div>
          <dt>手工补记</dt>
          <dd>{minutes(a.manual_seconds)}</dd>
        </div>
        <div>
          <dt>实际专注</dt>
          <dd>{minutes(a.focus_seconds)}</dd>
        </div>
        <div>
          <dt>平均每题用时</dt>
          <dd>
            {a.problems ? minutes((a.manual_seconds + a.focus_seconds) / a.problems) : t.noSamples}
          </dd>
        </div>
        <div>
          <dt>参赛次数</dt>
          <dd>{a.contests}</dd>
        </div>
      </dl>
      <p className="muted">
        练习题数按本周有尝试的不同题目计算。首过率以本周首次尝试的题目为分母；首次尝试通过为分子。用时归属尝试日期，补记只填写专注之外的额外时间。
      </p>
      <div className="coding-distributions">
        {[
          ['难度分布', a.difficulty],
          ['主题分布', a.topics],
        ].map(([name, rows]) => (
          <div key={String(name)}>
            <h3>{String(name)}</h3>
            {(rows as Analytics['topics']).length ? (
              <ul>
                {(rows as Analytics['topics']).map((r) => (
                  <li key={r.label}>
                    <span>{t.difficulty[r.label as keyof typeof t.difficulty] ?? r.label}</span>
                    <span>{r.count} 题</span>
                  </li>
                ))}
              </ul>
            ) : (
              <p>{t.noSamples}</p>
            )}
          </div>
        ))}
      </div>
    </section>
  );
}
