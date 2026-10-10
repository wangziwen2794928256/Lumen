import { useRef, useState } from 'react';
import { Card } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { Select } from '../../components/ui/input';
import { TaskForm } from '../../features/today/TaskForm';
import { useToday } from '../../features/today/useToday';
import { todayText as t } from '../../features/today/text';
import { focusText as f } from '../../features/focus/text';
import { SessionControls } from '../../features/focus/SessionControls';
export function FocusPage() {
  const { data, browser, loading, busy, error, act, refresh } = useToday();
  const [selected, setSelected] = useState('');
  const request = useRef<{ task: string; id: string } | null>(null);
  const tasks = data?.tasks.filter((task) => ['planned', 'active'].includes(task.lifecycle)) ?? [];
  const task = tasks.find((task) => task.id === selected) ?? tasks[0];
  const current = data?.tasks.find((task) => task.id === data.session?.task_id);
  return (
    <>
      <div className="page-heading">
        <div>
          <p className="eyebrow">FOCUS</p>
          <h1>{f.title}</h1>
          <p className="intro">{f.subtitle}</p>
        </div>
        <TaskForm disabled={!data || browser} busy={busy} act={act} />
      </div>
      {loading && <p role="status">{t.load}</p>}
      {browser && <p className="preview-notice">{t.browser}</p>}
      {error && (
        <p role="alert" className="field-error">
          {error}
          <Button variant="secondary" onClick={() => void refresh()}>
            {t.refresh}
          </Button>
        </p>
      )}
      <Card className="focus-card">
        <p className="eyebrow">{t.local}</p>
        {data?.session ? (
          <>
            <h2>{current?.title ?? f.choose}</h2>
            <SessionControls
              session={data.session}
              busy={busy}
              blocked={!!data.system_blocked}
              details
              act={act}
            />
          </>
        ) : (
          <>
            <h2>{f.choose}</h2>
            {tasks.length ? (
              <>
                <label className="field-label" htmlFor="focus-task">
                  {f.choose}
                </label>
                <Select
                  id="focus-task"
                  value={task?.id ?? ''}
                  disabled={busy}
                  onChange={(event) => {
                    setSelected(event.target.value);
                    request.current = null;
                  }}
                >
                  {tasks.map((task) => (
                    <option key={task.id} value={task.id}>
                      {task.title} · {task.estimated_minutes} {t.minuteUnit}
                    </option>
                  ))}
                </Select>
                <div className="task-actions">
                  <Button
                    disabled={busy || !data || !!data.system_blocked}
                    onClick={async () => {
                      if (!task) return;
                      if (request.current?.task !== task.id)
                        request.current = { task: task.id, id: crypto.randomUUID() };
                      if (await act({ type: 'start', id: request.current.id, task_id: task.id }))
                        request.current = null;
                    }}
                  >
                    {t.start}
                  </Button>
                </div>
              </>
            ) : (
              <p className="secondary-text">{f.empty}</p>
            )}
          </>
        )}
        <p className="secondary-text">{f.hint}</p>
      </Card>
    </>
  );
}
