import { useEffect, useState, useCallback } from 'react';
import { Link, useSearchParams } from 'react-router-dom';
import { isTauri } from '@tauri-apps/api/core';
import { Button } from '../../components/ui/button';
import { Card } from '../../components/ui/card';
import { insightsApi as api } from '../../features/insights/api';
import { insightsText as t } from '../../features/insights/text';
import {
  today,
  shift,
  minutes,
  ratio,
  statuses,
  recordHref,
  type Facts,
  type Day,
  type Word,
  type Review,
  type Monthly,
  type Action,
  type Preferences,
  type RecordLink,
} from '../../features/insights/model';
import { Trend, Distribution, Heatmap } from '../../features/insights/Charts';
import { exportJson, exportPng } from '../../features/insights/export';
const views = ['overview', 'heatmap', 'words', 'music', 'review', 'month', 'settings'] as const;
type View = (typeof views)[number];
function readDraft<T>(key: string): T | null {
  try {
    return JSON.parse(localStorage.getItem(key) ?? 'null') as T | null;
  } catch {
    return null;
  }
}
function Records({ records }: { records: RecordLink[] }) {
  return (
    <ul className="insight-records">
      {records.length === 0 ? (
        <li>{t.empty}</li>
      ) : (
        records.map((r) => (
          <li key={`${r.kind}-${r.id}-${r.date}`}>
            <Link to={recordHref(r)}>
              {r.date} · {r.title}
            </Link>
          </li>
        ))
      )}
    </ul>
  );
}
function Summary({ data }: { data: Facts }) {
  const deep = data.days.reduce((a, d) => a + d.deep_minutes, 0),
    coding = data.days.reduce((a, d) => a + d.coding_minutes, 0),
    plan = data.days.reduce((a, d) => a + d.planned_minutes, 0);
  return (
    <dl className="insight-metrics">
      <div>
        <dt>实际专注</dt>
        <dd>{minutes(deep)}</dd>
      </div>
      <div>
        <dt>编码用时</dt>
        <dd>{minutes(coding)}</dd>
      </div>
      <div>
        <dt>计划预算</dt>
        <dd>{minutes(plan)}</dd>
      </div>
      <div>
        <dt>计划任务当前完成比例</dt>
        <dd>{ratio(data.task_summary.completed, data.task_summary.total)}</dd>
      </div>
      <div>
        <dt>近 7 日一致性</dt>
        <dd>
          {ratio(data.consistency.days7.active, data.consistency.days7.eligible)} ·{' '}
          {data.consistency.days7.protected} 个特殊日
        </dd>
      </div>
      <div>
        <dt>近 28 日一致性</dt>
        <dd>
          {ratio(data.consistency.days28.active, data.consistency.days28.eligible)} ·{' '}
          {data.consistency.days28.protected} 个特殊日
        </dd>
      </div>
    </dl>
  );
}
function WordCloud({ words }: { words: Word[] }) {
  const [selectedWord, setSelectedWord] = useState('');
  const selected = words.find((w) => w.word === selectedWord);
  return (
    <>
      <p>{t.cloud}</p>
      <div className="insight-words">
        {words.length === 0 ? (
          <p>{t.empty}</p>
        ) : (
          words.map((w) => (
            <button
              type="button"
              key={w.word}
              onClick={() => setSelectedWord(w.word)}
              style={{ fontSize: `${Math.min(2, 1 + Math.log2(w.count + 1) / 6)}rem` }}
              aria-label={`${w.word}，${w.count} 次，查看来源`}
            >
              {w.word} <small>{w.count}</small>
            </button>
          ))
        )}
      </div>
      {selected && (
        <section aria-label="词语来源">
          <h3>{selected.word} · 来源</h3>
          <Records records={selected.records} />
        </section>
      )}
    </>
  );
}
function Music({ items }: { items: Facts['music'] }) {
  const [mode, setMode] = useState('song');
  const [metric, setMetric] = useState('count');
  const grouped = new Map<string, number>();
  for (const i of items) {
    const key =
      mode === 'artist'
        ? i.artist || '未标注艺术家'
        : mode === 'album'
          ? i.album || '未标注专辑'
          : i.label;
    grouped.set(
      key,
      (grouped.get(key) ?? 0) + (metric === 'count' ? (i.count ?? 0) : (i.seconds ?? 0) / 60),
    );
  }
  return (
    <>
      <p>音乐与日记词语分开；当前库中的播放记录，来源接入将在音乐阶段完善。</p>
      <label>
        音乐维度{' '}
        <select value={mode} onChange={(e) => setMode(e.target.value)}>
          <option value="song">歌曲</option>
          <option value="artist">艺术家</option>
          <option value="album">专辑</option>
        </select>
      </label>
      <label>
        音乐指标{' '}
        <select value={metric} onChange={(e) => setMetric(e.target.value)}>
          <option value="count">播放次数</option>
          <option value="minutes">收听分钟</option>
        </select>
      </label>
      <Distribution
        mode="count"
        items={[...grouped]
          .sort((a, b) => b[1] - a[1])
          .map(([label, count]) => ({ label, count: Math.round(count * 10) / 10 }))}
      />
    </>
  );
}
function Settings({ data, onSave }: { data: Facts; onSave: (a: Action) => Promise<boolean> }) {
  const key = 'lumen.insights.preferences.pending';
  const [draft, setDraft] = useState(
    () =>
      readDraft<Extract<Action, { type: 'preferences' }>>(key) ?? {
        type: 'preferences' as const,
        request_id: crypto.randomUUID(),
        revision: data.preferences_revision,
        value: data.preferences,
      },
  );
  const [busy, setBusy] = useState(false);
  const [stopText, setStopText] = useState(draft.value.stop_words.join(' '));
  function change(value: Preferences) {
    const next = { ...draft, value, request_id: crypto.randomUUID() };
    setDraft(next);
    localStorage.setItem(key, JSON.stringify(next));
  }
  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        setBusy(true);
        void onSave(draft)
          .then((ok) => {
            if (ok) localStorage.removeItem(key);
          })
          .finally(() => setBusy(false));
      }}
    >
      <fieldset disabled={busy}>
        <legend>活动强度算法</legend>
        <p>
          强度 = 六项信号 ×
          各自权重之和，再按四个阈值划分。权重可为零；这是活动量，不是成绩或道德分。
        </p>
        {Boolean(readDraft(key)) && <p role="status">{t.pending}</p>}
        <div className="insight-fields">
          {['实际专注分钟', '编码分钟', '完成任务', '日记数', '练习题数', '音乐播放数'].map(
            (s, i) => (
              <label key={s}>
                {s}权重
                <input
                  type="number"
                  min="0"
                  max="100"
                  step="0.1"
                  value={draft.value.weights[i]}
                  onChange={(e) =>
                    change({
                      ...draft.value,
                      weights: draft.value.weights.map((w, n) =>
                        n === i ? Number(e.target.value) : w,
                      ),
                    })
                  }
                />
              </label>
            ),
          )}
          {draft.value.thresholds.map((v, i) => (
            <label key={i}>
              强度阈值 {i + 1}
              <input
                type="number"
                min="0.1"
                max="100000"
                step="0.1"
                value={v}
                onChange={(e) =>
                  change({
                    ...draft.value,
                    thresholds: draft.value.thresholds.map((w, n) =>
                      n === i ? Number(e.target.value) : w,
                    ),
                  })
                }
              />
            </label>
          ))}
        </div>
        <label>
          自定义停用词（空格分隔）
          <textarea
            value={stopText}
            onChange={(e) => {
              setStopText(e.target.value);
              change({ ...draft.value, stop_words: e.target.value.split(/\s+/).filter(Boolean) });
            }}
          />
        </label>
        <label>
          <input
            type="checkbox"
            checked={draft.value.memories}
            onChange={(e) => change({ ...draft.value, memories: e.target.checked })}
          />{' '}
          在今日显示往年记录
        </label>
        <Button type="submit">保存活动配置</Button>
        <Button
          type="button"
          variant="secondary"
          onClick={() => {
            const next = {
              ...draft,
              revision: data.preferences_revision,
              request_id: crypto.randomUUID(),
            };
            setDraft(next);
            localStorage.setItem(key, JSON.stringify(next));
          }}
        >
          采用当前版本并保留输入
        </Button>
      </fieldset>
    </form>
  );
}
function ReviewPanel({ week, onSaved }: { week: string; onSaved: () => void }) {
  const [review, setReview] = useState<Review | null>(null),
    [error, setError] = useState(''),
    [busy, setBusy] = useState(false);
  const [draft, setDraft] = useState<Extract<Action, { type: 'review' }> | null>(null);
  const key = `lumen.insights.review.${week}.pending`;
  useEffect(() => {
    let live = true;
    api
      .review(week)
      .then((r) => {
        if (!live) return;
        setReview(r);
        setDraft(
          readDraft(key) ?? {
            type: 'review',
            request_id: crypto.randomUUID(),
            week: r.week,
            revision: r.revision,
            body: r.body,
            next_focus: r.next_focus,
          },
        );
      })
      .catch(() => {
        if (live) setError(t.error);
      });
    return () => {
      live = false;
    };
  }, [week, key]);
  function change(value: Partial<{ body: string; next_focus: string }>) {
    if (!draft) return;
    const d = { ...draft, ...value, request_id: crypto.randomUUID() };
    setDraft(d);
    localStorage.setItem(key, JSON.stringify(d));
  }
  async function save() {
    if (!draft) return;
    setBusy(true);
    try {
      await api.action(draft);
      localStorage.removeItem(key);
      const r = await api.review(week);
      setReview(r);
      setDraft({ ...draft, revision: r.revision, request_id: crypto.randomUUID() });
      setError('');
      onSaved();
    } catch {
      setError(t.error);
    } finally {
      setBusy(false);
    }
  }
  return (
    <section>
      {error && <p role="alert">{error}</p>}
      {review && draft ? (
        <>
          <h2>{review.week} · 本周事实</h2>
          <Summary data={review.facts} />
          <p>{t.definition}</p>
          <p>
            未完成 {review.facts.task_summary.unfinished} · 移动 {review.facts.task_summary.moved}{' '}
            个计划任务。较上一等长时段，专注变化{' '}
            {minutes(
              review.facts.days.reduce((s, d) => s + d.deep_minutes, 0) -
                review.facts.previous.deep_minutes,
            )}
            。
          </p>
          <h3>科目分配</h3>
          <Distribution items={review.facts.subjects} />
          <h3>编码主题</h3>
          <Distribution mode="count" items={review.facts.topics} />
          <h3>日记与音乐摘选</h3>
          <Records
            records={review.facts.records.filter((r) => r.kind === 'journal' || r.kind === 'music')}
          />
          <h3>本周词语</h3>
          <WordCloud words={review.facts.words} />
          <form
            onSubmit={(e) => {
              e.preventDefault();
              void save();
            }}
          >
            <fieldset disabled={busy}>
              <legend>我的心得 · 由你填写</legend>
              {Boolean(readDraft(key)) && <p role="status">{t.pending}</p>}
              <label>
                My Reflection
                <textarea
                  value={draft.body}
                  onChange={(e) => change({ body: e.target.value })}
                  maxLength={100000}
                />
              </label>
              <label>
                下周关注
                <textarea
                  value={draft.next_focus}
                  onChange={(e) => change({ next_focus: e.target.value })}
                  maxLength={10000}
                />
              </label>
              <Button type="submit">保存我的心得</Button>
              <Button
                type="button"
                variant="secondary"
                onClick={() => {
                  void api
                    .review(week)
                    .then((latest) => {
                      const d = {
                        ...draft,
                        revision: latest.revision,
                        request_id: crypto.randomUUID(),
                      };
                      setReview(latest);
                      setDraft(d);
                      localStorage.setItem(key, JSON.stringify(d));
                    })
                    .catch(() => setError(t.error));
                }}
              >
                采用当前版本并保留输入
              </Button>
            </fieldset>
          </form>
        </>
      ) : (
        <p>正在读取周回顾…</p>
      )}
    </section>
  );
}
function MonthPanel({ month }: { month: string }) {
  const [data, setData] = useState<Monthly | null>(null),
    [error, setError] = useState('');
  useEffect(() => {
    let live = true;
    api
      .month(month, today())
      .then((d) => {
        if (live) setData(d);
      })
      .catch(() => {
        if (live) setError(t.error);
      });
    return () => {
      live = false;
    };
  }, [month]);
  return (
    <section>
      {error && <p role="alert">{error}</p>}
      {data ? (
        <>
          <h2>{month} Snapshot</h2>
          <p>
            {data.closed ? '已封存月份；保留生成时的统计。' : '进行中的月份；读取时自动更新。'} ·{' '}
            {data.from} — {data.until}
          </p>
          <p>
            专注 {minutes(data.totals.deep_minutes)} · 编码 {minutes(data.totals.coding_minutes)} ·
            日记 {data.totals.journals}
          </p>
          <p>
            竞赛 {data.counts.competitions} · 成就 {data.counts.achievements} · 项目{' '}
            {data.counts.projects} · 照片 {data.counts.photos}
          </p>
          <h3>科目</h3>
          <Distribution items={data.subjects} />
          <h3>主题</h3>
          <Distribution mode="count" items={data.topics} />
          <h3>词语</h3>
          <p>{data.words.map((w) => `${w.word} (${w.count})`).join(' · ') || t.empty}</p>
          <h3>音乐</h3>
          <Music items={data.music} />
          <h3>日记摘选</h3>
          <Records records={data.highlights ?? []} />
          <p>{t.private}</p>
          <Button onClick={() => exportJson(data)}>导出私密 JSON</Button>
          <Button variant="secondary" onClick={() => exportPng(data)}>
            导出私密 PNG
          </Button>
        </>
      ) : (
        <p>正在生成月快照…</p>
      )}
    </section>
  );
}
export function InsightsPage() {
  const [params] = useSearchParams();
  const current = today();
  const [view, setView] = useState<View>(() =>
    views.includes(params.get('view') as View) ? (params.get('view') as View) : 'overview',
  );
  const [range, setRange] = useState('7'),
    [from, setFrom] = useState(shift(current, -6)),
    [until, setUntil] = useState(current),
    [week, setWeek] = useState(params.get('week') ?? current),
    [month, setMonth] = useState(current.slice(0, 7));
  const [data, setData] = useState<Facts | null>(null),
    [error, setError] = useState(''),
    [notice, setNotice] = useState(''),
    [loading, setLoading] = useState(false),
    [day, setDay] = useState<Day | null>(null),
    [detail, setDetail] = useState<Facts | null>(null);
  const refresh = useCallback(async () => {
    if (!isTauri()) return;
    setLoading(true);
    try {
      setData(await api.snapshot(from, until, today()));
      setError('');
    } catch {
      setError(t.error);
    } finally {
      setLoading(false);
    }
  }, [from, until]);
  useEffect(() => {
    let live = true;
    if (!isTauri()) return;
    api
      .snapshot(from, until, current)
      .then((d) => {
        if (live) {
          setData(d);
          setError('');
        }
      })
      .catch(() => {
        if (live) setError(t.error);
      })
      .finally(() => {
        if (live) setLoading(false);
      });
    return () => {
      live = false;
    };
  }, [from, until, current]);
  useEffect(() => {
    let live = true;
    if (day)
      api
        .snapshot(day.date, day.date, current)
        .then((d) => {
          if (live) setDetail(d);
        })
        .catch(() => {
          if (live) setError(t.error);
        });
    return () => {
      live = false;
    };
  }, [day, current]);
  async function action(a: Action) {
    try {
      await api.action(a);
      await refresh();
      setNotice(t.saved);
      return true;
    } catch {
      setError(t.error);
      return false;
    }
  }
  return (
    <section className="page-stack insights-page">
      <header className="page-heading">
        <p className="eyebrow">INSIGHTS / 洞察与回顾</p>
        <h1>{t.title}</h1>
        <p>{t.description}</p>
      </header>
      {!isTauri() ? (
        <Card>
          <p>{t.desktop}</p>
        </Card>
      ) : (
        <>
          <div className="insight-controls">
            <label>
              时间范围
              <select
                value={range}
                onChange={(e) => {
                  setRange(e.target.value);
                  if (e.target.value !== 'custom') {
                    setFrom(shift(current, 1 - Number(e.target.value)));
                    setUntil(current);
                  }
                }}
              >
                {[
                  ['7', '近 7 天'],
                  ['30', '近 30 天'],
                  ['90', '近 90 天'],
                  ['365', '近一年'],
                  ['custom', '自定义'],
                ].map(([v, s]) => (
                  <option key={v} value={v}>
                    {s}
                  </option>
                ))}
              </select>
            </label>
            <label>
              开始日期
              <input
                type="date"
                value={from}
                onChange={(e) => {
                  setRange('custom');
                  setFrom(e.target.value);
                }}
              />
            </label>
            <label>
              结束日期
              <input
                type="date"
                value={until}
                onChange={(e) => {
                  setRange('custom');
                  setUntil(e.target.value);
                }}
              />
            </label>
            <Button variant="secondary" onClick={() => void refresh()}>
              {t.refresh}
            </Button>
          </div>
          <nav className="insight-tabs" aria-label="洞察视图">
            {views.map((v) => (
              <Button
                key={v}
                variant={view === v ? 'default' : 'secondary'}
                aria-pressed={view === v}
                onClick={() => {
                  setView(v);
                  setDay(null);
                }}
              >
                {t[v]}
              </Button>
            ))}
          </nav>
          {error && <p role="alert">{error}</p>}
          {notice && <p role="status">{notice}</p>}
          {loading && <p role="status">正在读取本机记录…</p>}
          {data && (
            <>
              <p className="secondary-text">{t.definition}</p>
              {day ? (
                <Card>
                  <div className="insight-controls">
                    <h2>
                      {day.date} · {t.day}
                    </h2>
                    <Button variant="secondary" onClick={() => setDay(null)}>
                      返回洞察
                    </Button>
                  </div>
                  {detail && detail.from === day.date ? (
                    <>
                      <Summary data={detail} />
                      <label>
                        当日状态
                        <select
                          value={detail.days[0].status}
                          onChange={(e) => {
                            const status = e.target.value;
                            void action({
                              type: 'day',
                              request_id: crypto.randomUUID(),
                              date: day.date,
                              revision: detail.days[0].status_revision,
                              status,
                            }).then((ok) => {
                              if (ok) {
                                setDay({ ...day, status });
                              }
                            });
                          }}
                        >
                          {Object.entries(statuses)
                            .filter(
                              ([v]) =>
                                !['competition', 'paper_sprint', 'other'].includes(v) ||
                                detail.days[0].status === v,
                            )
                            .map(([v, s]) => (
                              <option key={v} value={v}>
                                {s}
                              </option>
                            ))}
                        </select>
                      </label>
                      <h3>科目与实际时间</h3>
                      <Distribution items={detail.subjects} />
                      {[
                        ['study', '专注'],
                        ['task', '任务'],
                        ['problem', '编码与题目'],
                        ['journal', '日记'],
                        ['music', '音乐'],
                        ['event', '日程'],
                        ['contest', '竞赛'],
                        ['achievement', '成就'],
                        ['photo', '照片'],
                        ['project', '项目'],
                      ].map(([kind, label]) => (
                        <section key={kind}>
                          <h3>{label}</h3>
                          <Records records={detail.records.filter((r) => r.kind === kind)} />
                        </section>
                      ))}
                    </>
                  ) : (
                    <p>正在读取当日…</p>
                  )}
                </Card>
              ) : view === 'overview' ? (
                <>
                  <Summary data={data} />
                  <div className="insight-grid">
                    <Card>
                      <Trend days={data.days} field="deep_minutes" title="实际专注趋势" />
                    </Card>
                    <Card>
                      <h2>科目分配</h2>
                      <Distribution items={data.subjects} />
                    </Card>
                    <Card>
                      <Trend days={data.days} field="planned_minutes" title="计划预算" />
                      <p>
                        实际专注 {minutes(data.days.reduce((a, d) => a + d.deep_minutes, 0))}
                        ；计划与实际并列展示，超出预算不裁剪。
                      </p>
                    </Card>
                    <Card>
                      <Trend days={data.days} field="coding_minutes" title="编码趋势" />
                    </Card>
                    <Card>
                      <h2>Journey 计划任务进度</h2>
                      <Distribution mode="progress" items={data.journeys} />
                    </Card>
                    <Card>
                      <Trend days={data.days} field="capacity_minutes" title="每日容量" />
                    </Card>
                  </div>
                </>
              ) : view === 'heatmap' ? (
                <Card>
                  <h2>{current.slice(0, 4)} 年度活动</h2>
                  <Heatmap days={data.heatmap} onDay={setDay} />
                </Card>
              ) : view === 'words' ? (
                <Card>
                  <h2>
                    {t.words} · {from} — {until}
                  </h2>
                  <WordCloud words={data.words} />
                </Card>
              ) : view === 'music' ? (
                <Card>
                  <h2>{t.music}</h2>
                  <Music items={data.music} />
                </Card>
              ) : view === 'review' ? (
                <Card>
                  <label>
                    回顾所在周
                    <input type="date" value={week} onChange={(e) => setWeek(e.target.value)} />
                  </label>
                  <ReviewPanel key={week} week={week} onSaved={() => void refresh()} />
                </Card>
              ) : view === 'month' ? (
                <Card>
                  <label>
                    快照月份
                    <input
                      type="month"
                      max={current.slice(0, 7)}
                      value={month}
                      onChange={(e) => setMonth(e.target.value)}
                    />
                  </label>
                  <MonthPanel key={month} month={month} />
                </Card>
              ) : (
                <Card>
                  <Settings key={data.preferences_revision} data={data} onSave={action} />
                </Card>
              )}
            </>
          )}
        </>
      )}
    </section>
  );
}
