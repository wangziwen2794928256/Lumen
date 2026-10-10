import { useRef, useState } from 'react';
import { Dialog } from '../../components/ui/dialog';
import { Button } from '../../components/ui/button';
import { Input, Select, Textarea } from '../../components/ui/input';
import { localDate, taskInputSchema, type Task, type TaskInput, type TodayAction } from './model';
import { todayText as t } from './text';
export function TaskForm({
  task,
  disabled,
  busy,
  act,
}: {
  task?: Task;
  disabled: boolean;
  busy: boolean;
  act: (action: TodayAction) => Promise<boolean>;
}) {
  const [open, setOpen] = useState(false),
    [error, setError] = useState('');
  const [draft, setDraft] = useState<TaskInput>({
    title: '',
    body: '',
    due_date: localDate(),
    estimated_minutes: 25,
    priority: 1,
  });
  const id = useRef('');
  function changeOpen(value: boolean) {
    if (busy) return;
    if (value) {
      setDraft(
        task
          ? { ...task }
          : { title: '', body: '', due_date: localDate(), estimated_minutes: 25, priority: 1 },
      );
      id.current = crypto.randomUUID();
      setError('');
    }
    setOpen(value);
  }
  return (
    <Dialog
      open={open}
      onOpenChange={changeOpen}
      trigger={
        <Button disabled={disabled || busy} variant={task ? 'secondary' : 'default'}>
          {task ? t.edit : t.add}
        </Button>
      }
      title={task ? t.edit : t.add}
      description={t.formHint}
    >
      <form
        onSubmit={async (event) => {
          event.preventDefault();
          const result = taskInputSchema.safeParse(draft);
          if (!result.success) {
            setError(t.invalid);
            return;
          }
          setError('');
          const ok = await act(
            task
              ? { type: 'edit', id: task.id, revision: task.revision, input: result.data }
              : { type: 'create', id: id.current, input: result.data },
          );
          if (ok) setOpen(false);
          else setError(t.error);
        }}
      >
        <label className="field-label" htmlFor="task-title">
          {t.titleLabel}
        </label>
        <Input
          id="task-title"
          value={draft.title}
          onChange={(e) => setDraft({ ...draft, title: e.target.value })}
          maxLength={250}
          required
          disabled={busy}
        />
        <label className="field-label" htmlFor="task-body">
          {t.bodyLabel}
        </label>
        <Textarea
          id="task-body"
          value={draft.body}
          onChange={(e) => setDraft({ ...draft, body: e.target.value })}
          maxLength={100000}
          rows={2}
          disabled={busy}
        />
        <label className="field-label" htmlFor="task-date">
          {t.dateLabel}
        </label>
        <Input
          type="date"
          id="task-date"
          value={draft.due_date ?? ''}
          onChange={(e) => setDraft({ ...draft, due_date: e.target.value || null })}
          disabled={busy}
        />
        <div className="task-form-grid">
          <div>
            <label className="field-label" htmlFor="task-estimate">
              {t.estimate}
            </label>
            <Input
              type="number"
              id="task-estimate"
              min={1}
              max={1440}
              value={draft.estimated_minutes}
              onChange={(e) => setDraft({ ...draft, estimated_minutes: Number(e.target.value) })}
              disabled={busy}
            />
          </div>
          <div>
            <label className="field-label" htmlFor="task-priority">
              {t.priority}
            </label>
            <Select
              id="task-priority"
              value={draft.priority}
              onChange={(e) => setDraft({ ...draft, priority: Number(e.target.value) })}
              disabled={busy}
            >
              {t.priorityLabels.map((label, index) => (
                <option key={index} value={index}>
                  {label}
                </option>
              ))}
            </Select>
          </div>
        </div>
        {error && (
          <p role="alert" className="field-error">
            {error}
          </p>
        )}
        <div className="dialog-actions">
          <Button type="submit" disabled={busy}>
            {busy ? t.saving : t.save}
          </Button>
        </div>
      </form>
    </Dialog>
  );
}
