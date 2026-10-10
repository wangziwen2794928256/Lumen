import { useEffect, useState } from 'react';
import { Button } from '../../components/ui/button';
import { codingApi } from '../../features/coding/api';
import { Editor } from '../../features/coding/Editor';
import { Stats } from '../../features/coding/Stats';
import {
  today,
  fresh,
  readDraft,
  safeLink,
  minutes,
  platforms,
  type Filter,
  type Snapshot,
  type Draft,
  type Action,
} from '../../features/coding/model';
import { codingText as t, codingError } from '../../features/coding/text';
export function CodingPage() {
  const [filter, setFilter] = useState<Filter>({
      week: today(),
      query: '',
      platform: '',
      status: '',
      tag: '',
      trash: false,
      page: 0,
      selected: null,
    }),
    [data, setData] = useState<Snapshot | null>(null),
    [editor, setEditor] = useState<Draft | null>(null),
    [tab, setTab] = useState('problems'),
    [error, setError] = useState(''),
    [reload, setReload] = useState(0),
    [busy, setBusy] = useState(false),
    [pending, setPending] = useState(readDraft);
  useEffect(() => {
    let live = true;
    const timer = setTimeout(() => {
      void codingApi
        .snapshot(filter)
        .then((s) => {
          if (live) {
            setData(s);
            setError('');
          }
        })
        .catch((e) => {
          if (live) setError(codingError(e));
        });
    }, 200);
    return () => {
      live = false;
      clearTimeout(timer);
    };
  }, [filter, reload]);
  function update(p: Partial<Filter>) {
    setFilter((f) => ({ ...f, ...p, page: p.page ?? 0, selected: p.selected ?? null }));
  }
  async function action(a: Action) {
    setBusy(true);
    try {
      await codingApi.action(a);
      setReload((n) => n + 1);
      setError('');
    } catch (e) {
      setError(codingError(e));
    } finally {
      setBusy(false);
    }
  }
  function trash(kind: string, v: { id: string; revision: number; deleted_at: string | null }) {
    void action({
      type: 'trash',
      request_id: crypto.randomUUID(),
      id: v.id,
      kind,
      revision: v.revision,
      restore: !!v.deleted_at,
    });
  }
  function back() {
    setEditor(null);
    setPending(readDraft());
    setReload((n) => n + 1);
  }
  const selected = data?.problems.find((p) => p.id === filter.selected),
    link = (value: string) => {
      const href = safeLink(value);
      return href ? (
        <a href={href} target="_blank" rel="noreferrer">
          打开平台链接
        </a>
      ) : null;
    };
  return (
    <div className="coding-page">
      <header className="journal-page-heading">
        <p className="eyebrow">CODING / 编程练习</p>
        <h1>{t.title}</h1>
        <p>{t.hint}</p>
      </header>
      <div className="journal-toolbar">
        <Button disabled={!!editor || !data} onClick={() => setEditor(fresh('problem'))}>
          {t.newProblem}
        </Button>
        <Button
          variant="secondary"
          disabled={!!editor || !data}
          onClick={() => setEditor(fresh('contest'))}
        >
          {t.newContest}
        </Button>
        <span className="muted">仅本机 · 手动录入</span>
      </div>
      {error && <p role="alert">{error}</p>}
      {!editor && pending && (
        <section className="ui-card">
          <p>{t.pending}</p>
          <Button variant="secondary" onClick={() => setEditor(pending)}>
            {t.recover}
          </Button>
        </section>
      )}
      {editor && data ? (
        <Editor
          key={editor.id}
          initial={editor}
          data={data}
          onBack={back}
          onSaved={() => {
            setPending(null);
            back();
          }}
        />
      ) : (
        <>
          <div className="journal-toolbar" role="group" aria-label="Coding 视图">
            {[
              ['problems', t.problems],
              ['contests', t.contests],
              ['analytics', t.analytics],
            ].map(([key, name]) => (
              <Button
                key={key}
                variant={tab === key ? 'default' : 'secondary'}
                aria-pressed={tab === key}
                onClick={() => setTab(key)}
              >
                {name}
              </Button>
            ))}
            <label className="roadmap-field">
              <span>统计所在周</span>
              <input
                type="date"
                value={filter.week}
                onChange={(e) => update({ week: e.target.value })}
              />
            </label>
          </div>
          {data && tab === 'analytics' && <Stats value={data.analytics} />}
          {tab === 'problems' && (
            <>
              <section className="ui-card">
                <div className="roadmap-form-grid">
                  <label className="roadmap-field">
                    <span>搜索名称或题号</span>
                    <input
                      value={filter.query}
                      onChange={(e) => update({ query: e.target.value })}
                    />
                  </label>
                  <label className="roadmap-field">
                    <span>平台筛选</span>
                    <select
                      value={filter.platform}
                      onChange={(e) => update({ platform: e.target.value })}
                    >
                      <option value="">全部平台</option>
                      {platforms.map((p) => (
                        <option key={p}>{p}</option>
                      ))}
                    </select>
                  </label>
                  <label className="roadmap-field">
                    <span>解题状态</span>
                    <select
                      value={filter.status}
                      onChange={(e) => update({ status: e.target.value })}
                    >
                      <option value="">全部状态</option>
                      <option value="unattempted">未尝试</option>
                      <option value="unsolved">未解出</option>
                      <option value="solved">已解出</option>
                    </select>
                  </label>
                  <label className="roadmap-field">
                    <span>主题筛选</span>
                    <input value={filter.tag} onChange={(e) => update({ tag: e.target.value })} />
                  </label>
                  <label className="roadmap-field">
                    <span>题目范围</span>
                    <select
                      value={String(filter.trash)}
                      onChange={(e) => update({ trash: e.target.value === 'true' })}
                    >
                      <option value="false">活动题目</option>
                      <option value="true">回收站</option>
                    </select>
                  </label>
                </div>
              </section>
              <div className="journal-list">
                {data?.problems.map((p) => (
                  <article key={p.id} className="ui-card coding-problem">
                    <p className="eyebrow">
                      {p.platform} · {p.external_id}
                    </p>
                    <h2>{p.title}</h2>
                    <p>
                      {p.solved ? '已解出' : p.attempt_count ? '未解出' : '未尝试'} ·{' '}
                      {p.attempt_count} 次尝试 ·{' '}
                      {t.difficulty[p.difficulty as keyof typeof t.difficulty] ?? p.difficulty}
                    </p>
                    <p>{p.tags.join(' · ') || '未标注主题'}</p>
                    <p>
                      首次尝试：
                      {p.first_attempt
                        ? new Date(p.first_attempt).toLocaleString('zh-CN')
                        : '暂无记录'}
                      <br />
                      首次通过：
                      {p.solved_at ? new Date(p.solved_at).toLocaleString('zh-CN') : '尚未通过'}
                      <br />
                      语言：{p.language ?? '未记录'}
                    </p>
                    <p>
                      手工 {minutes(p.manual_seconds)} · 专注 {minutes(p.focus_seconds)}
                    </p>
                    {link(p.source_url)}
                    <p className="preserve-lines">{p.notes}</p>
                    <div className="journal-toolbar">
                      {!p.deleted_at && (
                        <>
                          <Button
                            variant="secondary"
                            onClick={() =>
                              setEditor({
                                type: 'problem',
                                id: p.id,
                                revision: p.revision,
                                input: p,
                              })
                            }
                          >
                            {t.edit}
                          </Button>
                          <Button onClick={() => update({ selected: p.id, page: filter.page })}>
                            {t.attempts}
                          </Button>
                        </>
                      )}
                      <Button
                        variant="secondary"
                        disabled={busy}
                        onClick={() => trash('problem', p)}
                      >
                        {p.deleted_at ? t.restore : t.trash}
                      </Button>
                    </div>
                  </article>
                ))}
              </div>
              {data && !data.problems.length && <p className="ui-card">{t.empty}</p>}
              {data && (
                <div className="journal-toolbar">
                  <Button
                    variant="secondary"
                    disabled={!filter.page}
                    onClick={() => update({ page: filter.page - 1 })}
                  >
                    上一页
                  </Button>
                  <span>
                    {filter.page + 1} / {Math.max(1, Math.ceil(data.total / 30))} · {data.total} 题
                  </span>
                  <Button
                    variant="secondary"
                    disabled={(filter.page + 1) * 30 >= data.total}
                    onClick={() => update({ page: filter.page + 1 })}
                  >
                    下一页
                  </Button>
                </div>
              )}
              {selected && data && (
                <section className="ui-card">
                  <h2>
                    {t.attempts} · {selected.title}
                  </h2>
                  <Button onClick={() => setEditor(fresh('attempt', selected.id))}>
                    {t.attempt}
                  </Button>
                  {data.attempts.map((a) => (
                    <article className="coding-attempt" key={a.id}>
                      <h3>
                        {a.attempted_on || '旧记录无尝试日期'} · {a.language} ·{' '}
                        {t.outcome[a.outcome as keyof typeof t.outcome] ?? a.outcome}
                      </h3>
                      <p>
                        手工 {minutes(a.manual_seconds)} · 专注 {minutes(a.focus_seconds)}
                        {a.deleted_at ? ' · 已移入回收站' : ''}
                      </p>
                      <p className="preserve-lines">{a.notes}</p>
                      <div className="journal-toolbar">
                        {!a.deleted_at && (
                          <Button
                            variant="secondary"
                            onClick={() =>
                              setEditor({
                                type: 'attempt',
                                id: a.id,
                                revision: a.revision,
                                input: a,
                              })
                            }
                          >
                            {t.edit}
                          </Button>
                        )}
                        <Button
                          variant="secondary"
                          disabled={busy}
                          onClick={() => trash('attempt', a)}
                        >
                          {a.deleted_at ? t.restore : t.trash}
                        </Button>
                      </div>
                    </article>
                  ))}
                  {!data.attempts.length && <p>还没有尝试记录。</p>}
                  <p className="muted">展示最近 200 条尝试，周统计包含全部有效尝试。</p>
                </section>
              )}
            </>
          )}
          {tab === 'contests' && data && (
            <div className="journal-list">
              {data.contests.map((c) => (
                <article key={c.id} className="ui-card">
                  <h2>{c.title}</h2>
                  <p>
                    {c.platform} · {c.event_date} · {c.participated ? '已参加' : '未参加'}
                    {c.deleted_at ? ' · 回收站' : ''}
                  </p>
                  {link(c.source_url)}
                  <p className="preserve-lines">{c.notes}</p>
                  <div className="journal-toolbar">
                    {!c.deleted_at && (
                      <Button
                        variant="secondary"
                        onClick={() =>
                          setEditor({ type: 'contest', id: c.id, revision: c.revision, input: c })
                        }
                      >
                        {t.edit}
                      </Button>
                    )}
                    <Button variant="secondary" disabled={busy} onClick={() => trash('contest', c)}>
                      {c.deleted_at ? t.restore : t.trash}
                    </Button>
                  </div>
                </article>
              ))}
              {!data.contests.length && <p className="ui-card">还没有竞赛记录。</p>}
            </div>
          )}
        </>
      )}
    </div>
  );
}
