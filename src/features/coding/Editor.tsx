import { useState, useId, cloneElement, isValidElement, type ReactNode } from 'react';
import { Button } from '../../components/ui/button';
import { codingApi } from './api';
import {
  platforms,
  difficulties,
  outcomes,
  draftKey,
  normalize,
  type Draft,
  type Snapshot,
  type Choice,
} from './model';
import { codingText as t, codingError } from './text';
function Field({ label, children }: { label: string; children: ReactNode }) {
  const id = useId();
  return (
    <div className="roadmap-field">
      <label htmlFor={id}>{label}</label>
      {isValidElement<{ id?: string }>(children) ? cloneElement(children, { id }) : children}
    </div>
  );
}
function Choices({
  label,
  value,
  choices,
  onChange,
}: {
  label: string;
  value: string | null;
  choices: Choice[];
  onChange: (value: string | null) => void;
}) {
  return (
    <Field label={label}>
      <select value={value ?? ''} onChange={(e) => onChange(e.target.value || null)}>
        <option value="">不关联</option>
        {choices.map((c) => (
          <option key={c.id} value={c.id}>
            {c.title}
          </option>
        ))}
      </select>
    </Field>
  );
}
function localTime(iso: string) {
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return '';
  const shifted = new Date(date.getTime() - date.getTimezoneOffset() * 60000);
  return shifted.toISOString().slice(0, 16);
}
export function Editor({
  initial,
  data,
  onBack,
  onSaved,
}: {
  initial: Draft;
  data: Snapshot;
  onBack: () => void;
  onSaved: (id: string) => void;
}) {
  const [draft, setDraft] = useState(() => normalize(initial)),
    [tags, setTags] = useState(initial.type === 'problem' ? initial.input.tags.join(', ') : ''),
    [busy, setBusy] = useState(false),
    [error, setError] = useState(''),
    [request, setRequest] = useState(initial.request_id ?? crypto.randomUUID());
  function change(input: Record<string, unknown>) {
    const nextRequest = crypto.randomUUID();
    const next = {
      ...draft,
      request_id: nextRequest,
      input: { ...draft.input, ...input },
    } as Draft;
    setDraft(next);
    setRequest(nextRequest);
    try {
      localStorage.setItem(draftKey(next), JSON.stringify(next));
    } catch {
      setError('本地待提交稿未写入，请保留此窗口并完成保存。');
    }
  }
  async function save() {
    setBusy(true);
    setError('');
    try {
      await codingApi.action({ ...draft, request_id: request });
      localStorage.removeItem(draftKey(draft));
      onSaved(draft.id);
    } catch (e) {
      setError(codingError(e));
    } finally {
      setBusy(false);
    }
  }
  const p = draft.type === 'problem' ? draft.input : null,
    c = draft.type === 'contest' ? draft.input : null,
    a = draft.type === 'attempt' ? draft.input : null;
  return (
    <section className="ui-card coding-editor">
      <h2>{p ? '题目资料' : c ? '竞赛资料' : t.attempt}</h2>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          void save();
        }}
      >
        <fieldset disabled={busy} className="coding-fieldset">
          <div className="roadmap-form-grid">
            {(p || c) && (
              <>
                <Field label="名称">
                  <input
                    required
                    maxLength={1000}
                    value={(p ?? c)!.title}
                    onChange={(e) => change({ title: e.target.value })}
                  />
                </Field>
                <Field label="平台">
                  <select
                    value={(p ?? c)!.platform}
                    onChange={(e) => change({ platform: e.target.value })}
                  >
                    {platforms.map((s) => (
                      <option key={s}>{s}</option>
                    ))}
                  </select>
                </Field>
                <Field label="平台链接">
                  <input
                    type="url"
                    maxLength={2000}
                    placeholder="https://"
                    value={(p ?? c)!.source_url}
                    onChange={(e) => change({ source_url: e.target.value })}
                  />
                </Field>
              </>
            )}
            {p && (
              <>
                <Field label="平台题号">
                  <input
                    required
                    maxLength={100}
                    value={p.external_id}
                    onChange={(e) => change({ external_id: e.target.value })}
                  />
                </Field>
                <Field label="难度">
                  <select
                    value={p.difficulty}
                    onChange={(e) => change({ difficulty: e.target.value })}
                  >
                    {difficulties.map((s) => (
                      <option key={s} value={s}>
                        {t.difficulty[s]}
                      </option>
                    ))}
                  </select>
                </Field>
                <Field label="算法标签（逗号分隔）">
                  <input
                    value={tags}
                    maxLength={3000}
                    placeholder="DP, Graph, Binary Search"
                    onChange={(e) => {
                      setTags(e.target.value);
                      change({
                        tags: [
                          ...new Set(
                            e.target.value
                              .split(/[,，]/)
                              .map((s) => s.trim())
                              .filter(Boolean),
                          ),
                        ],
                      });
                    }}
                  />
                </Field>
                <Choices
                  label="关联长期目标"
                  value={p.journey_id}
                  choices={data.journeys}
                  onChange={(v) => change({ journey_id: v })}
                />
                <Choices
                  label="关联任务"
                  value={p.task_id}
                  choices={data.tasks}
                  onChange={(v) => change({ task_id: v })}
                />
                <Choices
                  label="关联竞赛"
                  value={p.competition_id}
                  choices={data.contests
                    .filter((c) => !c.deleted_at)
                    .map((c) => ({ id: c.id, title: c.title, task_id: null }))}
                  onChange={(v) => change({ competition_id: v })}
                />
              </>
            )}
            {c && (
              <>
                <Field label="竞赛日期">
                  <input
                    type="date"
                    required
                    value={c.event_date}
                    onChange={(e) => change({ event_date: e.target.value })}
                  />
                </Field>
                <Field label="参加状态">
                  <select
                    value={String(c.participated)}
                    onChange={(e) => change({ participated: e.target.value === 'true' })}
                  >
                    <option value="false">未参加</option>
                    <option value="true">已参加</option>
                  </select>
                </Field>
              </>
            )}
            {a && (
              <>
                <Field label="尝试时间">
                  <input
                    type="datetime-local"
                    required
                    value={localTime(a.started_at)}
                    onChange={(e) => {
                      const d = new Date(e.target.value);
                      if (!Number.isNaN(d.getTime()))
                        change({
                          started_at: d.toISOString(),
                          attempted_on: e.target.value.slice(0, 10),
                        });
                    }}
                  />
                </Field>
                <Field label="语言">
                  <input
                    required
                    maxLength={100}
                    placeholder="C++ / Python / Rust"
                    value={a.language}
                    onChange={(e) => change({ language: e.target.value })}
                  />
                </Field>
                <Field label="结果">
                  <select value={a.outcome} onChange={(e) => change({ outcome: e.target.value })}>
                    {outcomes.map((s) => (
                      <option key={s} value={s}>
                        {t.outcome[s]}
                      </option>
                    ))}
                  </select>
                </Field>
                <Field label="额外手工用时（分钟）">
                  <input
                    type="number"
                    min={0}
                    max={10080}
                    step="0.1"
                    value={a.manual_seconds / 60}
                    onChange={(e) =>
                      change({ manual_seconds: Math.round(Number(e.target.value) * 60) })
                    }
                  />
                </Field>
                <Choices
                  label="关联已完成专注"
                  value={a.focus_session_id}
                  choices={data.focus}
                  onChange={(v) => change({ focus_session_id: v })}
                />
                <p className="muted">
                  已完成专注从真实账本读取，不可重复关联。额外手工用时不包含这段专注。
                </p>
              </>
            )}
          </div>
          <Field label="笔记">
            <textarea
              value={draft.input.notes}
              maxLength={100000}
              rows={5}
              onChange={(e) => change({ notes: e.target.value })}
            />
          </Field>
          {error && <p role="alert">{error}</p>}
          <div className="journal-toolbar">
            <Button type="submit" disabled={busy}>
              {busy ? '正在保存…' : t.save}
            </Button>
            <Button variant="secondary" disabled={busy} onClick={onBack}>
              {t.back}
            </Button>
          </div>
        </fieldset>
      </form>
    </section>
  );
}
