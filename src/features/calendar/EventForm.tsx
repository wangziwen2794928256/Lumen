import { useRef, useState } from 'react';
import { Dialog } from '../../components/ui/dialog';
import { Button } from '../../components/ui/button';
import { Input, Select } from '../../components/ui/input';
import type { Task, TodayAction } from '../today/model';
import {
  eventSchema,
  eventTypes,
  localDateTime,
  parseLocalDateTime,
  type CalendarEvent,
} from './model';
import { calendarText as t } from './text';
export function EventForm({
  event,
  day,
  tasks,
  disabled,
  busy,
  act,
}: {
  event?: CalendarEvent;
  day: string;
  tasks: Task[];
  disabled: boolean;
  busy: boolean;
  act: (action: TodayAction) => Promise<boolean>;
}) {
  const [open, setOpen] = useState(false),
    [error, setError] = useState('');
  const initial = () => ({
    title: event?.title ?? '',
    type: event?.event_type ?? 'personal',
    allDay: event ? event.event_date !== null : false,
    date: event?.event_date ?? day,
    start: event?.starts_at ? localDateTime(event.starts_at) : day + 'T09:00',
    end: event?.ends_at ? localDateTime(event.ends_at) : day + 'T10:00',
    task: event?.task_id ?? '',
  });
  const [draft, setDraft] = useState(initial);
  const id = useRef('');
  const updateOpen = (value: boolean) => {
    if (busy) return;
    if (value) {
      setDraft(initial());
      setError('');
      id.current = crypto.randomUUID();
    }
    setOpen(value);
  };
  return (
    <Dialog
      open={open}
      onOpenChange={updateOpen}
      title={event ? t.edit : t.add}
      description={t.formHint}
      trigger={
        <Button variant={event ? 'secondary' : 'default'} disabled={disabled || busy}>
          {event ? t.edit : t.add}
        </Button>
      }
    >
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          let start: null | string = null,
            end: null | string = null;
          try {
            if (!draft.allDay) {
              start = parseLocalDateTime(draft.start);
              end = parseLocalDateTime(draft.end);
            }
          } catch {
            setError(t.invalid);
            return;
          }
          const parsed = eventSchema.safeParse({
            title: draft.title,
            event_type: draft.type,
            event_date: draft.allDay ? draft.date : null,
            starts_at: start,
            ends_at: end,
            task_id: draft.task || null,
          });
          if (!parsed.success) {
            setError(t.invalid);
            return;
          }
          setError('');
          const ok = await act(
            event
              ? {
                  type: 'calendar_edit',
                  id: event.id,
                  revision: event.revision,
                  input: parsed.data,
                }
              : { type: 'calendar_create', id: id.current, input: parsed.data },
          );
          if (ok) setOpen(false);
          else setError(t.error);
        }}
      >
        <label htmlFor="event-title" className="field-label">
          {t.titleLabel}
        </label>
        <Input
          id="event-title"
          required
          maxLength={250}
          value={draft.title}
          disabled={busy}
          onChange={(e) => setDraft({ ...draft, title: e.target.value })}
        />
        <label htmlFor="event-type" className="field-label">
          {t.type}
        </label>
        <Select
          id="event-type"
          value={draft.type}
          disabled={busy}
          onChange={(e) => setDraft({ ...draft, type: e.target.value as typeof draft.type })}
        >
          {eventTypes.map((type) => (
            <option value={type} key={type}>
              {t.types[type]}
            </option>
          ))}
        </Select>
        <label className="field-label checkbox-label">
          <input
            type="checkbox"
            checked={draft.allDay}
            disabled={busy}
            onChange={(e) => setDraft({ ...draft, allDay: e.target.checked })}
          />
          {t.allDay}
        </label>
        {draft.allDay ? (
          <>
            <label className="field-label" htmlFor="event-date">
              {t.day}
            </label>
            <Input
              id="event-date"
              type="date"
              required
              value={draft.date}
              disabled={busy}
              onChange={(e) => setDraft({ ...draft, date: e.target.value })}
            />
          </>
        ) : (
          <div className="task-form-grid">
            {(['start', 'end'] as const).map((field) => (
              <div key={field}>
                <label className="field-label" htmlFor={'event-' + field}>
                  {t[field]}
                </label>
                <Input
                  id={'event-' + field}
                  type="datetime-local"
                  required
                  value={draft[field]}
                  disabled={busy}
                  onChange={(e) => setDraft({ ...draft, [field]: e.target.value })}
                />
              </div>
            ))}
          </div>
        )}
        <label className="field-label" htmlFor="event-task">
          {t.task}
        </label>
        <Select
          id="event-task"
          value={draft.task}
          disabled={busy}
          onChange={(e) => setDraft({ ...draft, task: e.target.value })}
        >
          <option value="">{t.none}</option>
          {event?.task_id && !tasks.some((task) => task.id === event.task_id) && (
            <option value={event.task_id}>{t.linked}</option>
          )}
          {tasks.map((task) => (
            <option key={task.id} value={task.id}>
              {task.title}
            </option>
          ))}
        </Select>
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
