import { locale } from '../../lib/locale';
import { useRef, useState } from 'react';
import { Card, CardHeader, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Input, Textarea } from '../../components/ui/input';
import { SessionControls } from '../../features/focus/SessionControls';
import { TaskForm } from '../../features/today/TaskForm';
import { useToday } from '../../features/today/useToday';
import { duration, localDate, type Task } from '../../features/today/model';
import { todayText as t } from '../../features/today/text';
import { TodayWidgets } from '../../kernel/modules/Extensions';
export function TodayPage() {
  const { data, browser, loading, busy, error, act, refresh } = useToday();
  const [note, setNote] = useState(''),
    [noteError, setNoteError] = useState(''),
    [capacity, setCapacity] = useState('');
  const noteId = useRef(''),
    startId = useRef<{ task: string; id: string } | null>(null);
  const tasks = data?.tasks ?? [],
    session = data?.session,
    disabled = browser || !data;
  const pending = tasks.filter(
    (task) => task.lifecycle === 'planned' || task.lifecycle === 'active',
  );
  const current = session ? tasks.find((task) => task.id === session.task_id) : pending[0];
  const next = pending.filter((task) => task.id !== current?.id),
    other = tasks.filter((task) => !pending.includes(task));
  async function start(task: Task) {
    if (startId.current?.task !== task.id)
      startId.current = { task: task.id, id: crypto.randomUUID() };
    if (await act({ type: 'start', id: startId.current.id, task_id: task.id }))
      startId.current = null;
  }
  function row(task: Task) {
    return (
      <li key={task.id}>
        <div className="task-row-title">
          <strong>{task.title}</strong>
          <span className="secondary-text">
            {task.estimated_minutes} {t.minuteUnit} · {t[task.lifecycle]}
          </span>
        </div>
        <div className="task-actions">
          <TaskForm
            task={task}
            disabled={disabled || session?.task_id === task.id}
            busy={busy}
            act={act}
          />
          {task.lifecycle === 'planned' || task.lifecycle === 'active' ? (
            <>
              <Button
                variant="secondary"
                disabled={disabled || busy || !!session || !!data?.system_blocked}
                onClick={() => void start(task)}
              >
                {t.start}
              </Button>
              <Button
                variant="ghost"
                disabled={disabled || busy || session?.task_id === task.id}
                onClick={() =>
                  void act({
                    type: 'task_state',
                    id: task.id,
                    revision: task.revision,
                    state: 'done',
                  })
                }
              >
                {t.complete}
              </Button>
              <Button
                variant="ghost"
                disabled={disabled || busy || session?.task_id === task.id}
                onClick={() =>
                  void act({
                    type: 'task_state',
                    id: task.id,
                    revision: task.revision,
                    state: 'skipped',
                  })
                }
              >
                {t.skip}
              </Button>
              <Button
                variant="ghost"
                disabled={disabled || busy || session?.task_id === task.id}
                onClick={() =>
                  void act({
                    type: 'task_state',
                    id: task.id,
                    revision: task.revision,
                    state: 'cancelled',
                  })
                }
              >
                {t.cancelTask}
              </Button>
              <Button
                variant="ghost"
                disabled={disabled || busy || session?.task_id === task.id}
                onClick={() =>
                  void act({
                    type: 'task_state',
                    id: task.id,
                    revision: task.revision,
                    state: 'moved',
                  })
                }
              >
                {t.move}
              </Button>
            </>
          ) : (
            <Button
              variant="secondary"
              disabled={disabled || busy}
              onClick={() =>
                void act({
                  type: 'task_state',
                  id: task.id,
                  revision: task.revision,
                  state: 'planned',
                })
              }
            >
              {t.reopen}
            </Button>
          )}
          <Button
            variant="ghost"
            disabled={disabled || busy || session?.task_id === task.id}
            onClick={() =>
              void act({ type: 'trash', id: task.id, revision: task.revision, deleted: true })
            }
          >
            {t.remove}
          </Button>
        </div>
      </li>
    );
  }
  return (
    <>
      <div className="page-heading">
        <div>
          <p className="eyebrow">TODAY / {data?.date ?? localDate()}</p>
          <h1>{t.title}</h1>
          <p className="intro">{t.subtitle}</p>
        </div>
        <TaskForm disabled={disabled} busy={busy} act={act} />
      </div>
      {loading && <p role="status">{t.load}</p>}
      {browser && <p className="preview-notice">{t.browser}</p>}
      {error && (
        <div role="alert" className="field-error">
          {error}{' '}
          <Button variant="secondary" disabled={busy} onClick={() => void refresh()}>
            {t.refresh}
          </Button>
        </div>
      )}
      <TodayWidgets slot="today.after-summary" />
      <div className="today-grid">
        <div className="primary-column">
          <Card className="now-card">
            <p className="eyebrow">NOW / {t.now}</p>
            <p className="journey-label">{data?.journey ?? t.noJourney}</p>
            <h2 className="now-title">{current?.title ?? t.empty}</h2>
            {current?.body && <p className="user-body secondary-text">{current.body}</p>}
            {session ? (
              <SessionControls
                session={session}
                busy={busy}
                blocked={!!data?.system_blocked}
                act={act}
              />
            ) : (
              current && (
                <div className="now-bottom">
                  <span>
                    {current.estimated_minutes} {t.minuteUnit}
                  </span>
                  <Button
                    disabled={busy || disabled || !!data?.system_blocked}
                    onClick={() => void start(current)}
                  >
                    {t.start}
                  </Button>
                </div>
              )
            )}
            {current && !session && (
              <div className="task-actions">
                <TaskForm task={current} disabled={disabled} busy={busy} act={act} />
                {(['skipped', 'cancelled', 'moved'] as const).map((state) => (
                  <Button
                    key={state}
                    variant="ghost"
                    disabled={busy || disabled}
                    onClick={() =>
                      void act({
                        type: 'task_state',
                        id: current.id,
                        revision: current.revision,
                        state,
                      })
                    }
                  >
                    {state === 'skipped' ? t.skip : state === 'cancelled' ? t.cancelTask : t.move}
                  </Button>
                ))}
                <Button
                  variant="ghost"
                  disabled={busy || disabled}
                  onClick={() =>
                    void act({
                      type: 'task_state',
                      id: current.id,
                      revision: current.revision,
                      state: 'done',
                    })
                  }
                >
                  {t.complete}
                </Button>
                <Button
                  variant="ghost"
                  disabled={busy || disabled}
                  onClick={() =>
                    void act({
                      type: 'trash',
                      id: current.id,
                      revision: current.revision,
                      deleted: true,
                    })
                  }
                >
                  {t.remove}
                </Button>
              </div>
            )}
          </Card>
          <Card>
            <CardHeader>
              <CardTitle>NEXT / {t.next}</CardTitle>
              <span>{next.length}</span>
            </CardHeader>
            {next.length ? (
              <ul className="live-task-list">{next.map(row)}</ul>
            ) : (
              <p className="secondary-text">{t.empty}</p>
            )}
          </Card>
          <Card>
            <CardHeader>
              <CardTitle>REMEMBER / {t.remember}</CardTitle>
            </CardHeader>
            <form
              className="today-note-form"
              onSubmit={async (e) => {
                e.preventDefault();
                if (!note.trim()) {
                  setNoteError(t.noteInvalid);
                  return;
                }
                setNoteError('');
                if (!noteId.current) noteId.current = crypto.randomUUID();
                if (await act({ type: 'capture', id: noteId.current, body: note })) {
                  setNote('');
                  noteId.current = '';
                }
              }}
            >
              <label htmlFor="quick-note" className="field-label">
                {t.noteLabel}
              </label>
              <Textarea
                id="quick-note"
                value={note}
                onChange={(e) => setNote(e.target.value)}
                placeholder={t.notePlaceholder}
                maxLength={100000}
                rows={3}
                disabled={disabled || busy}
              />
              {noteError && (
                <p role="alert" className="field-error">
                  {noteError}
                </p>
              )}
              <Button type="submit" disabled={disabled || busy}>
                {t.noteSave}
              </Button>
            </form>
            {data?.notes.length ? (
              data.notes.map((item) => (
                <p className="user-body note-preview" key={item.id}>
                  {item.body}
                </p>
              ))
            ) : (
              <p className="secondary-text">{t.noNotes}</p>
            )}
          </Card>
          {other.length > 0 && (
            <Card>
              <CardTitle>{t.other}</CardTitle>
              <ul className="live-task-list">{other.map(row)}</ul>
            </Card>
          )}
          <Card>
            <details>
              <summary>
                {t.trash} ({data?.trash.length ?? 0})
              </summary>
              {data?.trash.length ? (
                <ul className="live-task-list">
                  {data.trash.map((task) => (
                    <li key={task.id}>
                      <strong>{task.title}</strong>
                      <Button
                        variant="secondary"
                        disabled={busy || disabled}
                        onClick={() =>
                          void act({
                            type: 'trash',
                            id: task.id,
                            revision: task.revision,
                            deleted: false,
                          })
                        }
                      >
                        {t.restore}
                      </Button>
                    </li>
                  ))}
                </ul>
              ) : (
                <p className="secondary-text">{t.noTrash}</p>
              )}
            </details>
          </Card>
        </div>
        <div className="secondary-column">
          <Card>
            <CardHeader>
              <CardTitle>{t.capacity}</CardTitle>
            </CardHeader>
            <p className="capacity-value">
              {data?.capacity_minutes ?? 210} <span>{t.minuteUnit}</span>
            </p>
            <p className="secondary-text">{t.capacityHint}</p>
            <p>
              {t.plannedMinutes}：{data?.planned_minutes ?? 0}
            </p>
            <p>
              {t.actual}：{duration(data?.actual_seconds ?? 0)}
            </p>
            {data && data.planned_minutes > data.capacity_minutes && (
              <p className="preview-notice">{t.overCapacity}</p>
            )}
            <form
              className="capacity-form"
              onSubmit={(e) => {
                e.preventDefault();
                void act({ type: 'capacity', minutes: Number(capacity) });
              }}
            >
              <label className="field-label" htmlFor="daily-capacity">
                {t.capacity}
              </label>
              <Input
                type="number"
                id="daily-capacity"
                min={0}
                max={1440}
                required
                value={capacity}
                placeholder={String(data?.capacity_minutes ?? 210)}
                onChange={(e) => setCapacity(e.target.value)}
                disabled={disabled || busy}
              />
              <Button
                type="submit"
                variant="secondary"
                disabled={disabled || busy || capacity === ''}
              >
                {t.capacitySave}
              </Button>
            </form>
          </Card>
          <TodayWidgets slot="today.sidebar" />
          <Card>
            <CardHeader>
              <CardTitle>{t.timeline}</CardTitle>
            </CardHeader>
            {data?.timeline.length ? (
              data.timeline.map((item) => (
                <div className="timeline-entry" key={item.id}>
                  <span>
                    {new Date(item.started_at).toLocaleTimeString(locale, {
                      hour: '2-digit',
                      minute: '2-digit',
                    })}
                  </span>
                  <div>
                    <strong>{item.title}</strong>
                    <small>{duration(item.actual_seconds)}</small>
                  </div>
                </div>
              ))
            ) : (
              <p className="secondary-text">{t.noTimeline}</p>
            )}
          </Card>
        </div>
      </div>
    </>
  );
}
